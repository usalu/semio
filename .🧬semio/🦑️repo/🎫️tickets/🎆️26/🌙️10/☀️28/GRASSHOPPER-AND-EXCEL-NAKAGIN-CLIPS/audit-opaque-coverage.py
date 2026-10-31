"""🔬 Attribute actual source-face coverage with no-AA, silhouette and normal Cairo controls."""
import argparse
from collections import Counter
import importlib
import inspect
import hashlib
import json
from pathlib import Path
import sys
import cairo
import numpy as np
from manim import Camera,VGroup,tempconfig
from PIL import Image

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
parser=argparse.ArgumentParser()
parser.add_argument('--render',action='store_true')
parser.add_argument('--source',choices=('capsule','capital'),default='capsule')
parser.add_argument('--tower',type=int,default=0)
parser.add_argument('--azimuth',type=float,default=55)
parser.add_argument('--elevation',type=float,default=24)
parser.add_argument('--modes',nargs='*')
parser.add_argument('--detail',choices=('exterior','full'),default='exterior')
parser.add_argument('--layout',choices=('Slash','Backslash','J','L','P','S'),default='Slash')
parser.add_argument('--glass-opacity',type=float,default=1)
parser.add_argument('--resolution',type=int,default=256)
parser.add_argument('--output-name',required=True)
parser.add_argument('--baseline-name')
parser.add_argument('--baseline-partitions',action='store_true')
args=parser.parse_args()
if not args.output_name.replace('-','').isalnum():
    raise ValueError('Invalid output name')
output=Path(__file__).parent/'🗑️generated'/args.output_name
output.mkdir(parents=True,exist_ok=True)
baseline=None
if args.baseline_name is not None:
    if not args.baseline_name.replace('-','').isalnum():
        raise ValueError('Invalid baseline name')
    baseline=output.parent/args.baseline_name/'coverage-items.json'
sources=list(dict.fromkeys([*map(Path,V.source_files()),Path(V._model.__file__),Path(V._model.__file__).with_name('🔣️.json'),Path(__file__),*([baseline] if baseline is not None else [])]))
start_sources={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}

def facet_normal(polygon):
    """🧭 Derive actual material winding independently of bucket-normal metadata."""
    points=np.asarray(polygon,float)
    local=points-points[0]
    normal=np.cross(local,np.roll(local,-1,axis=0)).sum(axis=0)
    length=float(np.linalg.norm(normal))
    if not length:
        raise ValueError('A canonical facet needs nonzero oriented material area')
    return normal/length


class ExactCamera(Camera):
    """🔎 Disable actual Cairo antialiasing for an exact physical face/silhouette control."""
    def get_cairo_context(self,pixel_array):
        context=super().get_cairo_context(pixel_array)
        context.set_antialias(cairo.ANTIALIAS_NONE)
        return context

if args.resolution not in (256,512,1024):
    raise ValueError('Resolution must be 256,512 or1024')
size=args.resolution
frame=6 if args.source=='capsule' else 16
if args.source=='capsule':
    parts=V._cap_parts(V._cap(V._model.capsule_type('A')),args.detail,layout=args.layout)
    centre=(0,2,1.25)
else:
    tower=V._model.towers()[args.tower]
    parts=V._capital_parts(tower.index,tower.height,tower.capital_height,tower.capital_overhang,tower.core_side)
    vertices=np.vstack([mesh.V for part,mesh in parts])
    centre=(vertices.min(axis=0)+vertices.max(axis=0))/2
with tempconfig({'pixel_width':size,'pixel_height':size,'frame_width':frame,'frame_height':frame,'renderer':'cairo','dry_run':True}):
    view=V.make_view((args.azimuth,args.elevation),(0,0,0),1,centre)
    basis,towards=V._basis(view)
    camera={"towards":np.asarray(towards).tolist(),"basis":basis.tolist(),"centre":np.asarray(view.centre).tolist(),"point":np.asarray(view.point).tolist(),"scale":view.scale,"pixel_width":size,"pixel_height":size,"frame_width":frame,"frame_height":frame,"projection":"orthographic"}
    (output/"coverage-camera.json").write_text(json.dumps(camera,indent=2),encoding="utf-8")
    groups={}
    items=V._assembly_items(parts,base=V.CAPSULE if args.source=="capsule" else V.CAPITAL,groups=groups,lod=True,glass_opacity=args.glass_opacity)
    retained=V._opaque_visible(items,view,padding=V._raster_footprint(1,.45))
    full=V._section_order(items,view)
    new=V._section_order(retained,view)
    selected={id(item) for item in retained}
    source_indices={id(item):i for i,item in enumerate(items)}
    owner_index=lambda item:source_indices[id(V._source_item(item))]
    stable=[item for item in full if id(V._source_item(item)) in selected]
    paint=[]
    for stage,ordered in (("full",full),("stable",stable),("culled",new)):
        for rank,item in enumerate(ordered):
            for piece,polygon in enumerate(item.polygons or (item.verts,)):
                paint.append({"stage":stage,"rank":rank,"piece":piece,"source":owner_index(item),"polygon":piece if item.polygon is None else item.polygon,"world_vertices":np.asarray(polygon).tolist(),"normal":list(V._paint_normal(item,polygon))})
    (output/"coverage-paint-fragments.json").write_text(json.dumps(paint,separators=(",",":")),encoding="utf-8")
    rows=[]
    for index,item in enumerate(items):
        defaults={key:parameter.default for key,parameter in inspect.signature(item.draw).parameters.items()}
        screen=V._screen(view,item.verts)
        pixels=np.column_stack(((screen[:,0]+frame/2)*size/frame,(frame/2-screen[:,1])*size/frame))
        polygons=[]
        for polygon in item.polygons or (item.verts,):
            p=V._screen(view,polygon)
            polygons.append(np.column_stack(((p[:,0]+frame/2)*size/frame,(frame/2-p[:,1])*size/frame)).tolist())
        rows.append({'index':index,'key':defaults.get('part_key'),'role':defaults.get('role'),'material':item.material,'opacity':item.opacity,'normal':np.asarray(item.normal).tolist(),'world_vertices':item.verts.tolist(),'pixel_polygons':polygons,'pixel_bounds':[pixels.min(axis=0).tolist(),pixels.max(axis=0).tolist()],'raw_camera_facing':bool(item.normal@towards>1e-6),'facet_camera_facing':[[bool(facet_normal(facet)@towards>1e-6) for facet in V._polygon_facets(polygon)] for polygon in item.polygons or (item.verts,)],'removed':id(item) not in selected,'colour':str(defaults.get('color'))})
    if baseline is not None:
        previous=json.loads(baseline.read_text(encoding='utf-8'))
        if args.baseline_partitions:
            def material(rows):
                result=Counter()
                for row in rows:
                    offset=0
                    for polygon in row['pixel_polygons']:
                        length=len(polygon)
                        vertices=row['world_vertices'][offset:offset+length]
                        result[(row['key'],row['role'],row['material'],row['opacity'],tuple(map(tuple,vertices)))]+=1
                        offset+=length
                return result
            if material(rows)!=material(previous):
                raise AssertionError('Actual source-polygon material differs across the declared regrouping')
            print('[DEBUG] Exact original material-polygon Counter matches',args.baseline_name,flush=True)
        else:
            fields=('index','key','role','material','opacity','normal','world_vertices','pixel_polygons','removed')
            if len(rows)!=len(previous) or any(any(row[field]!=old[field] for field in fields) for row,old in zip(rows,previous)):
                raise AssertionError('Original source geometry or exact retained IDs differ from the immutable baseline')
            print('[DEBUG] Exact original geometry and retained IDs match',args.baseline_name,flush=True)
    (output/'coverage-items.json').write_text(json.dumps(rows,separators=(',',':')),encoding='utf-8')
    print('[DEBUG] Exported source items',len(items),'retained',len(retained),flush=True)
    if args.render:
        report={'source_items':len(items),'retained_items':len(retained),'modes':{}}
        for resolution,modes in ((size,('geometric','silhouette','ids','aa-fill','aa-strokes','material')),(size*4,('geometric','silhouette','material'))):
            with tempconfig({'pixel_width':resolution,'pixel_height':resolution,'frame_width':frame,'frame_height':frame}):
                for mode in modes:
                    if args.modes is not None and mode not in args.modes:
                        continue
                    images=[]
                    for label,ordered in (('full',full),('stable',stable),('culled',new)):
                        for group in groups.values():
                            group.remove(*group.submobjects)
                            group.material_underpaint.remove(*group.material_underpaint.submobjects)
                        if mode=='material':
                            shapes=V._paint_ordered(ordered,view)
                        else:
                            shapes=[]
                            for item in ordered:
                                shape=item.draw(view)
                                if mode in ('geometric','silhouette','ids','aa-fill'):
                                    shape.set_stroke(width=0)
                                if mode=='silhouette':
                                    shape.set_fill('#FFFFFF',1)
                                if mode=='ids':
                                    shape.set_fill('#'+format(owner_index(item)+1,'06x'),1)
                                shapes.append(shape)
                        camera=(ExactCamera if mode in ('geometric','silhouette','ids') else Camera)(background_color='#000000' if mode=='ids' else V.BACKGROUND)
                        camera.capture_mobjects([VGroup(*shapes)])
                        image=camera.pixel_array.copy()
                        images.append(image)
                        Image.fromarray(image).save(output/(str(resolution)+'-'+mode+'-'+label+'.png'))
                    metrics={}
                    for name,a,b in (('stable_removed',0,1),('retained_order',1,2),('total',0,2)):
                        difference=np.abs(images[a].astype(int)-images[b].astype(int))
                        metrics[name]={'channels':int(np.count_nonzero(difference)),'pixels':int((difference[:,:,:3]>0).any(axis=2).sum()),'maximum':int(difference.max())}
                    report['modes'][str(resolution)+'-'+mode]=metrics
                    (output/'coverage-aa-controls.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
                    print('[DEBUG]',resolution,mode,json.dumps(metrics),flush=True)

end_sources={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}
provenance={'start_sources':start_sources,'end_sources':end_sources,'unchanged':start_sources==end_sources}
(output/'coverage-provenance.json').write_text(json.dumps(provenance,indent=2),encoding='utf-8')
if not provenance['unchanged']:
    raise AssertionError('Actual source changed during coverage controls')
