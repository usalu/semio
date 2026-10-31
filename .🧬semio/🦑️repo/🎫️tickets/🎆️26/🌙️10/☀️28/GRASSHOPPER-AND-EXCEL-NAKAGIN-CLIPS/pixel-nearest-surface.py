"""🔎 Independently attribute changed raster pixels to exact exported physical source triangles."""
import argparse
from collections import Counter
import importlib.util
import hashlib
import json
from pathlib import Path
import sys
import numpy as np

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
ORACLE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🚶️circulation/🐍️.py'
spec=importlib.util.spec_from_file_location('pixel_independent_face_oracle',ORACLE)
O=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=O
spec.loader.exec_module(O)

def convex_fan(polygon,normal=None):
    """📐 Fan only actual planar convex material verified by every normalized edge half-plane."""
    points=np.asarray(polygon,float)
    local=points-points[0]
    if normal is None:
        crosses=np.cross(local[:,None,:],local[None,:,:]).reshape(-1,3)
        lengths=np.linalg.norm(crosses,axis=1)
        if not len(lengths) or float(lengths.max())==0:
            return None
        normal=crosses[int(np.argmax(lengths))]/float(lengths.max())
        epsilon=64*np.spacing(float(np.abs(points).max()) or 1.0)
        if float(np.abs(local@normal).max())>epsilon:
            return None
    else:
        normal=np.asarray(normal,float)
    projected=np.delete(local,int(np.argmax(np.abs(normal))),axis=1)
    spans=np.ptp(projected,axis=0)
    if np.any(spans==0):
        return None
    normalized=projected/spans
    edges=np.roll(normalized,-1,axis=0)-normalized
    delta=normalized[None,:,:]-normalized[:,None,:]
    side=edges[:,None,0]*delta[:,:,1]-edges[:,None,1]*delta[:,:,0]
    if not ((side>=0).all() or (side<=0).all()):
        return None
    return tuple(points[[0,index,index+1]] for index in range(1,len(points)-1) if np.any(np.cross(points[index]-points[0],points[index+1]-points[0])!=0))


def source_facets(polygon):
    """🧊 Resolve only genuinely warped original quads by their declared shortest 3D mesh diagonal."""
    points=np.asarray(polygon,float)
    if len(points)==4:
        vectors=points[1:]-points[0]
        crosses=np.cross(vectors[:,None,:],vectors[None,:,:]).reshape(-1,3)
        lengths=np.linalg.norm(crosses,axis=1)
        if float(lengths.max())>0:
            normal=crosses[int(np.argmax(lengths))]/float(lengths.max())
            epsilon=64*np.spacing(max(float(np.abs(points).max()),0.0) or 1.0)
            if float(np.abs((points-points[0])@normal).max())>epsilon:
                diagonal02=float(np.dot(points[2]-points[0],points[2]-points[0]))
                diagonal13=float(np.dot(points[3]-points[1],points[3]-points[1]))
                ids=((0,1,2),(0,2,3)) if diagonal02<=diagonal13 else ((0,1,3),(1,2,3))
                return tuple(points[list(indices)] for indices in ids),True
    if len(points)==3:
        if np.any(np.cross(points[1]-points[0],points[2]-points[0])!=0):
            return (points,),False
        raise ValueError('Original material triangle must have positive area')
    try:
        triangles=O.face_triangles(tuple(map(tuple,points)),tuple(range(len(points))))
    except ValueError:
        triangles=convex_fan(points)
        if triangles is None:
            raise
    actual=triangles or convex_fan(points)
    if actual is None or len(actual)==0:
        raise ValueError('Original material boundary must not disappear during triangulation')
    return actual,False


def source_carriers(polygon):
    """🧬 Recover carrier planes only from original canonical physical geometry and winding."""
    points=np.asarray(polygon,float)
    facets,warped=source_facets(points) if len(points)==4 else ((),False)
    candidates=facets if warped else (points,)
    result=[]
    for boundary in candidates:
        local=boundary-boundary[0]
        crosses=np.cross(local[:,None,:],local[None,:,:]).reshape(-1,3)
        lengths=np.linalg.norm(crosses,axis=1)
        if not len(lengths) or float(lengths.max())==0:
            raise ValueError('An original carrier requires positive material area')
        normal=crosses[int(np.argmax(lengths))]/float(lengths.max())
        winding=np.cross(local,np.roll(local,-1,axis=0)).sum(axis=0)
        if float(normal@winding)<0:
            normal=-normal
        epsilon=64*np.spacing(float(np.abs(boundary).max()) or 1.0)
        if float(np.abs(local@normal).max())>epsilon:
            raise ValueError('Original carrier conditioning requires an explicit diagnostic')
        result.append((boundary[0],normal,boundary,epsilon))
    return tuple(result)


class SurfaceScene:
    """📐 Fit the actual world-to-pixel projection and retain each physical face boundary separately."""
    def __init__(self,items,towards,tolerance=1e-7):
        self.items={row['index']:row for row in items}
        if len(self.items)!=len(items): raise ValueError('Duplicate exported source index')
        world,pixels,polygons=[],[],[]
        for row in items:
            vertices=np.asarray(row['world_vertices'],float)
            cursor=0
            for polygon_index,projected in enumerate(row['pixel_polygons']):
                projected=np.asarray(projected,float)
                polygon=vertices[cursor:cursor+len(projected)]
                if len(polygon)!=len(projected): raise ValueError('Source polygon correspondence is incomplete')
                cursor+=len(projected)
                world.extend(polygon)
                pixels.extend(projected)
                polygons.append((row,polygon,polygon_index))
            if cursor!=len(vertices): raise ValueError('Unmapped world source vertices')
        self.original_polygons={(row['index'],index):polygon for row,polygon,index in polygons}
        self.carrier_cache={}
        world,pixels=np.asarray(world),np.asarray(pixels)
        homogeneous=np.column_stack((world,np.ones(len(world))))
        fit,_,rank,_=np.linalg.lstsq(homogeneous,pixels,rcond=None)
        self.fit_residual=float(np.max(np.abs(homogeneous@fit-pixels)))
        if rank<4 or self.fit_residual>tolerance: raise ValueError('Export does not define one exact orthographic camera')
        self.matrix,self.offset=fit[:3].T,fit[3]
        _,_,vectors=np.linalg.svd(self.matrix)
        self.towards=vectors[-1]
        if self.towards@np.asarray(towards)<0: self.towards=-self.towards
        if abs(float(self.towards@np.asarray(towards)/np.linalg.norm(towards)))<1-1e-7: raise ValueError('Camera direction disagrees with independently recovered nullspace')
        self.inverse=np.linalg.pinv(self.matrix)
        self.depth=float(np.max(world@self.towards))+max(float(np.ptp(world,axis=0).max()),1)
        triangles,owners=[],[]
        self.drawn=set()
        self.warped_polygons=[]
        for row,polygon,polygon_index in polygons:
            if row['opacity']<=0: continue
            faces,warped=source_facets(polygon)
            if warped:
                self.warped_polygons.append((row['index'],polygon_index))
            retained=[]
            for triangle in faces:
                triangle=np.asarray(triangle,float)
                normal=np.cross(triangle[1]-triangle[0],triangle[2]-triangle[0])
                if float(normal@self.towards)>0:
                    retained.append(triangle)
            faces=retained
            if faces: self.drawn.add(row['index'])
            triangles.extend(faces)
            owners.extend([row['index']]*len(faces))
        self.triangles=np.asarray(triangles,float).reshape(-1,3,3)
        self.owners=np.asarray(owners,int)
        self.a=self.triangles[:,0]
        self.e1=self.triangles[:,1]-self.a
        self.e2=self.triangles[:,2]-self.a
        self.direction=-self.towards
        self.h=np.cross(self.direction,self.e2)
        det=np.einsum('ij,ij->i',self.e1,self.h)
        self.valid=np.isfinite(det)&(det!=0)
        self.inv=np.divide(1,det,out=np.zeros_like(det),where=self.valid)

    def intersections(self,points):
        """🧮 Intersect parallel pixel rays with actual triangles using vectorized Möller–Trumbore equations."""
        origins=(np.asarray(points)-self.offset)@self.inverse.T+self.depth*self.towards
        offsets=origins[:,None,:]-self.a
        u=np.einsum('sni,ni->sn',offsets,self.h)*self.inv
        q=np.cross(offsets,self.e1)
        v=np.einsum('sni,i->sn',q,self.direction)*self.inv
        distance=np.einsum('sni,ni->sn',q,self.e2)*self.inv
        valid=self.valid&(u>=-1e-9)&(v>=-1e-9)&(u+v<=1+1e-9)&(distance>=0)
        return np.where(valid,self.depth-distance,np.nan)

    def contributors(self,hits,retained=False,depth_tolerance=1e-8):
        """🪟 Trace physical layers until opaque material, preserving coplanar attribution ties."""
        depths={}
        for triangle in np.flatnonzero(np.isfinite(hits)):
            owner=int(self.owners[triangle])
            if retained and self.items[owner]['removed']: continue
            depths[owner]=max(depths.get(owner,-np.inf),float(hits[triangle]))
        pending=sorted(depths,key=lambda owner:depths[owner],reverse=True)
        layers=[]
        while pending:
            near=depths[pending[0]]
            group=[owner for owner in pending if near-depths[owner]<=depth_tolerance]
            layers.append({'depth':near,'ids':sorted(group)})
            pending=[owner for owner in pending if owner not in group]
            if any(self.items[owner]['opacity']>=1-1e-12 for owner in group): break
        return {'layers':layers,'visible_ids':[owner for layer in layers for owner in layer['ids']],'hit_ids':sorted(depths),'nearest_depth':layers[0]['depth'] if layers else None,'nearest_ids':layers[0]['ids'] if layers else []}

    def painter(self,hits,mode):
        """🎨 Attribute ideal point compositing separately from the physical nearest-surface order."""
        ids={int(self.owners[i]) for i in np.flatnonzero(np.isfinite(hits))}
        ids=[i for i in ids if not self.items[i]['removed'] or mode=='full']
        key='new_rank' if mode=='culled' else 'full_rank'
        ordered=sorted(ids,key=lambda i:self.items[i][key] if self.items[i][key] is not None else -1,reverse=True)
        result=[]
        for owner in ordered:
            result.append(owner)
            if self.items[owner]['opacity']>=1-1e-12: break
        return result


class FragmentPaint:
    """🎨 Intersect actual exported paint pieces independently of original physical nearest surfaces."""
    def __init__(self,scene,rows):
        self.scene,self.rows=scene,tuple(rows)
        identities=set()
        triangles,owners,carrier_origins,carrier_normals=[],[],[],[]
        groups,geometries={},{}
        self.geometry_rows=[]
        for index,row in enumerate(self.rows):
            identity=(row['stage'],row['rank'],row['piece'])
            if identity in identities or row['stage'] not in ('full','stable','culled'):
                raise ValueError('Paint stage/rank/piece must identify one unique actual piece')
            identities.add(identity)
            if row['source'] not in scene.items or not 0<=row['polygon']<len(scene.items[row['source']]['pixel_polygons']):
                raise ValueError('Paint piece must identify an original physical source polygon')
            polygon=np.asarray(row['world_vertices'],float)
            if polygon.ndim!=2 or polygon.shape[1]!=3 or len(polygon)<3 or not np.isfinite(polygon).all():
                raise ValueError('Paint pieces require finite actual three-dimensional polygons')
            declared=np.asarray(row['normal'],float)
            if declared.shape!=(3,) or not np.isfinite(declared).all() or not np.any(declared!=0):
                raise ValueError('Paint normal must be a finite nonzero geometric vector')
            length=float(np.linalg.norm(declared))
            if not np.isfinite(length) or length==0:
                raise ValueError('Paint normal must admit finite normalization')
            declared=declared/length
            carrier_key=(row['source'],row['polygon'])
            if carrier_key not in scene.carrier_cache:
                scene.carrier_cache[carrier_key]=source_carriers(scene.original_polygons[carrier_key])
            carrier=None
            for carrier_index,(origin,normal,boundary,epsilon) in enumerate(scene.carrier_cache[carrier_key]):
                piece_epsilon=64*np.spacing(max(float(np.abs(boundary).max()),float(np.abs(polygon).max())) or 1.0)
                if float(normal@declared)>0 and float(np.abs((boundary-origin)@declared).max())<=epsilon and float(np.abs((polygon-origin)@normal).max())<=piece_epsilon:
                    carrier=(origin,normal,carrier_index)
                    break
            if carrier is None:
                raise ValueError('Paint piece must match a validated original canonical carrier')
            origin,normal,carrier_index=carrier
            opacity=scene.items[row['source']]['opacity']
            group=(row['stage'],row['rank'])
            if group in groups and groups[group]!=opacity:
                raise ValueError('One compound paint rank must have one actual opacity')
            groups[group]=opacity
            key=(row['source'],row['polygon'],carrier_index,polygon.tobytes())
            if key not in geometries:
                geometry=len(self.geometry_rows)
                geometries[key]=geometry
                self.geometry_rows.append([])
                actual=convex_fan(polygon,normal)
                if actual is None:
                    actual,_=source_facets(polygon)
                if len(actual)==0:
                    raise ValueError('Positive paint material must not disappear')
                triangles.extend(actual)
                owners.extend([geometry]*len(actual))
                carrier_origins.extend([origin]*len(actual))
                carrier_normals.extend([normal]*len(actual))
            self.geometry_rows[geometries[key]].append(index)
        self.triangles=np.asarray(triangles,float).reshape(-1,3,3)
        self.owners=np.asarray(owners,int)
        self.carrier_origins=np.asarray(carrier_origins,float).reshape(-1,3)
        self.carrier_normals=np.asarray(carrier_normals,float).reshape(-1,3)
        self.a=self.triangles[:,0]
        self.e1=self.triangles[:,1]-self.a
        self.e2=self.triangles[:,2]-self.a
        self.direction=-scene.towards
        self.h=np.cross(self.direction,self.e2)
        determinant=np.einsum('ij,ij->i',self.e1,self.h)
        self.valid=np.isfinite(determinant)&(determinant!=0)
        self.inv=np.divide(1,determinant,out=np.zeros_like(determinant),where=self.valid)
        self.depth=max(scene.depth,float(np.max(self.triangles@scene.towards,initial=scene.depth))+1)

    def intersections(self,points):
        """🔬 Trace actual clipped paint geometry without changing the independent physical scene."""
        origins=(np.asarray(points)-self.scene.offset)@self.scene.inverse.T+self.depth*self.scene.towards
        offsets=origins[:,None,:]-self.a
        u=np.einsum('sni,ni->sn',offsets,self.h)*self.inv
        q=np.cross(offsets,self.e1)
        v=np.einsum('sni,i->sn',q,self.direction)*self.inv
        base=(np.asarray(points)-self.scene.offset)@self.scene.inverse.T
        denominator=self.carrier_normals@self.scene.towards
        numerator=np.einsum('ni,ni->n',self.carrier_origins,self.carrier_normals)[None,:]-base@self.carrier_normals.T
        depth=np.divide(numerator,denominator[None,:],out=np.full_like(numerator,np.nan),where=denominator[None,:]!=0)
        valid=self.valid&(u>=-1e-9)&(v>=-1e-9)&(u+v<=1+1e-9)&(depth<=self.depth)
        return np.where(valid,depth,np.nan)

    def contributors(self,hits,stage):
        """🪟 Union pieces once per actual paint rank, preserving transparent layers and source aliases."""
        if stage not in ('full','stable','culled'):
            raise ValueError('Explicit paint stage required')
        by_rank={}
        indices=set()
        for triangle in np.flatnonzero(np.isfinite(hits)):
            indices.update(self.geometry_rows[int(self.owners[triangle])])
        for index in sorted(indices):
            row=self.rows[index]
            if row['stage']==stage and self.scene.items[row['source']]['opacity']>0:
                by_rank.setdefault(row['rank'],set()).add(index)
        layers=[]
        for rank in sorted(by_rank,reverse=True):
            indices=sorted(by_rank[rank])
            sources=sorted({self.rows[index]['source'] for index in indices})
            opacity=self.scene.items[sources[0]]['opacity']
            layers.append({'rank':rank,'sources':sources,'rows':indices,'pieces':[self.rows[index]['piece'] for index in indices],'opacity':opacity})
            if opacity>=1-1e-12:
                break
        return {'layers':layers,'visible_ids':[source for layer in layers for source in layer['sources']]}


def sample_grid(x,y,count):
    """🔬 Include the pixel centre and a regular grid of strictly interior subpixel locations."""
    offsets=(np.arange(count)+.5)/count
    return np.unique(np.vstack((np.array([[x+.5,y+.5]]),np.array([(x+a,y+b) for a in offsets for b in offsets]))),axis=0)


def analyze(scene,pixels,grid=5,progress=None,cancelled=None):
    """🧭 Report sampled visible losses, hidden underpaint and independent painter-order disagreements."""
    result=[]
    for index,(x,y) in enumerate(pixels):
        if cancelled and cancelled(): raise InterruptedError('Independent pixel audit cancelled')
        if progress and (index%25==0 or index==len(pixels)-1): progress(index,len(pixels))
        points=sample_grid(x,y,grid)
        hits=scene.intersections(points)
        samples=[]
        for point,hit in zip(points,hits):
            before,after=scene.contributors(hit),scene.contributors(hit,True)
            removed=[]
            coplanar=[]
            for layer in before['layers']:
                erased=[owner for owner in layer['ids'] if scene.items[owner]['removed']]
                if any(not scene.items[owner]['removed'] and scene.items[owner]['opacity']>=1-1e-12 for owner in layer['ids']): coplanar.extend(erased)
                else: removed.extend(erased)
            hidden=[owner for owner in before['hit_ids'] if scene.items[owner]['removed'] and owner not in before['visible_ids']]
            same_depth=before['nearest_depth']==after['nearest_depth'] or (before['nearest_depth'] is not None and after['nearest_depth'] is not None and abs(before['nearest_depth']-after['nearest_depth'])<=1e-8)
            samples.append({'point':point.tolist(),'before':before,'after':after,'removed_visible':removed,'coplanar_removed':coplanar,'removed_hidden':hidden,'nearest_depth_unchanged':same_depth,'painter_full':scene.painter(hit,'full'),'painter_stable':scene.painter(hit,'stable'),'painter_culled':scene.painter(hit,'culled')})
        near_ids={owner for sample in samples for owner in sample['before']['nearest_ids']}
        visible=sum(bool(sample['removed_visible']) for sample in samples)
        hidden=sum(bool(sample['removed_hidden']) for sample in samples)
        unchanged=all(sample['nearest_depth_unchanged'] for sample in samples)
        ties=sum(bool(sample['coplanar_removed']) for sample in samples)
        label='sampled-visible-material-removed' if visible else 'coplanar-source-attribution-tie' if ties else 'opaque-join-underpaint-candidate' if hidden and unchanged and len(near_ids)>1 else 'sampled-hidden-underpaint' if hidden and unchanged else 'retained-material-or-order-only'
        result.append({'pixel':[int(x),int(y)],'classification':label,'sample_count':len(samples),'visible_removed_samples':visible,'hidden_removed_samples':hidden,'coplanar_tie_samples':ties,'nearest_depth_unchanged':unchanged,'nearest_source_ids':sorted(near_ids),'painter_reorder_samples':sum(sample['painter_stable']!=sample['painter_culled'] for sample in samples),'samples':samples})
    return result


def boundary_witness(scene,owner,point):
    """📏 Measure exact distance to exported polygon edges and extrapolated source-plane depth."""
    item=scene.items[owner]
    point=np.asarray(point,float)
    closest=np.inf
    for polygon in item['pixel_polygons']:
        a=np.asarray(polygon,float)
        delta=np.roll(a,-1,axis=0)-a
        length=np.einsum('ij,ij->i',delta,delta)
        along=np.divide(np.einsum('ij,ij->i',point-a,delta),length,out=np.zeros_like(length),where=length>0)
        nearest=a+np.clip(along,0,1)[:,None]*delta
        closest=min(closest,float(np.linalg.norm(nearest-point,axis=1).min()))
    base=(point-scene.offset)@scene.inverse.T
    normal=np.asarray(item['normal'],float)
    origin=np.asarray(item['world_vertices'][0],float)
    depth=float(normal@(origin-base)/(normal@scene.towards))
    return {'id':owner,'key':item['key'],'role':item['role'],'removed':item['removed'],'normal':item['normal'],'projected_edge_distance_pixels':closest,'extrapolated_plane_depth':depth}


def main():
    """🗃️ Consume exact exported source data and current Cairo controls without importing the renderer."""
    from PIL import Image
    parser=argparse.ArgumentParser()
    parser.add_argument('--items',type=Path,required=True)
    parser.add_argument('--towards',type=float,nargs=3,required=True)
    parser.add_argument('--before',type=Path,required=True)
    parser.add_argument('--after',type=Path,required=True)
    parser.add_argument('--grid',type=int,default=5)
    parser.add_argument('--before-ids',type=Path)
    parser.add_argument('--after-ids',type=Path)
    parser.add_argument('--input-attestation',type=Path)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args()
    if args.grid<1: raise ValueError('Positive subpixel grid required')
    attestation=json.loads(args.input_attestation.read_text(encoding='utf8')) if args.input_attestation else None
    source_hash=hashlib.sha256(args.items.read_bytes()).hexdigest()
    for path in (path for path in (args.before,args.after,args.before_ids,args.after_ids) if path):
        if path.stat().st_mtime+1e-3<args.items.stat().st_mtime:
            if not attestation or attestation.get('source_sha256')!=source_hash or attestation.get('image_sha256',{}).get(path.name)!=hashlib.sha256(path.read_bytes()).hexdigest():
                raise ValueError('Raster predates export without matching explicit source/image attestation: '+str(path))
    before=np.asarray(Image.open(args.before).convert('RGBA'))
    after=np.asarray(Image.open(args.after).convert('RGBA'))
    if before.shape!=after.shape: raise ValueError('Raster control dimensions differ')
    y,x=np.where(np.any(before[:,:,:3]!=after[:,:,:3],axis=2))
    scene=SurfaceScene(json.loads(args.items.read_text(encoding='utf8')),args.towards)
    rows=analyze(scene,list(zip(x,y)),args.grid)
    id_arrays={name:np.asarray(Image.open(path).convert('RGB')) for name,path in (('before',args.before_ids),('after',args.after_ids)) if path}
    for row in rows:
        px,py=row['pixel']
        centre=next(sample for sample in row['samples'] if np.allclose(sample['point'],[px+.5,py+.5],atol=1e-12,rtol=0))
        for name,array in id_arrays.items():
            rgb=array[py,px].astype(int)
            source_id=int(rgb@[65536,256,1])-1
            row[name+'_cairo_source_id']=source_id
            row[name+'_cairo_matches_physical_centre']=source_id in centre[name]['nearest_ids'] if source_id>=0 else not centre[name]['nearest_ids']
    report={'source_sha256':source_hash,'input_attestation':str(args.input_attestation) if args.input_attestation else None,'items':str(args.items),'before':str(args.before),'after':str(args.after),'grid':args.grid,'pixel_count':len(rows),'changed_channels':int(np.count_nonzero(before!=after)),'projection_max_residual':scene.fit_residual,'world_to_pixel':scene.matrix.tolist(),'pixel_offset':scene.offset.tolist(),'towards':scene.towards.tolist(),'triangles':len(scene.triangles),'classifications':dict(Counter(row['classification'] for row in rows)),'pixels':rows,'limits':'Finite centre/grid sampling is attribution evidence, not exhaustive pixel coverage or an AA equivalence waiver.'}
    args.output.write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8')
    print('[DEBUG] '+json.dumps({k:report[k] for k in ('pixel_count','changed_channels','projection_max_residual','triangles','classifications')}),flush=True)

if __name__=='__main__':
    main()
