"""🧪 Actual chapter material drawing callbacks checked against independent GEOS regions."""
import ast
import importlib
import math
from pathlib import Path
from types import SimpleNamespace
import numpy as np
from shapely.geometry import Polygon,Point,box
from shapely.ops import unary_union
from shapely.affinity import affine_transform

ROOT=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
MODEL=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
DRAFT_GEOMETRY=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.📏️geometry')
M=MODEL.load_model()
G=MODEL.building_geometry(M)
records=[]

class Mark:
    """🖊️ Recording-only group; it never renders or measures a native frame."""
    def __init__(self,*args,**kwargs):
        self.items=list(args)
    def add(self,*args):
        self.items.extend(args)
        return self

def polygon(ring,color,*args,**kwargs):
    """📐 Retain every material polygon before any renderer consumes it."""
    shape=Polygon(np.asarray(ring)[:,:2])
    assert shape.is_valid and shape.area>0,(color,ring)
    records.append((color,shape))
    return Mark()

def compile_methods(folder,names,environment,stop=None):
    """🧩 Compile unchanged actual callback syntax, optionally stopping at a named drawing phase."""
    path=ROOT/folder/'🐍️.py'
    tree=ast.parse(path.read_text())
    cls=next(n for n in tree.body if isinstance(n,ast.ClassDef))
    methods=[n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name in names]
    if stop:
        method=next(n for n in methods if n.name==stop[0])
        index=next(i for i,n in enumerate(method.body) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id==stop[1] for t in n.targets))
        method.body=method.body[:index]+[ast.Return(value=ast.Call(func=ast.Name(id="locals",ctx=ast.Load()),args=[],keywords=[]))]
    space={'np':np,'math':math,'_model':MODEL,**environment}
    exec(compile(ast.fix_missing_locations(ast.Module(body=methods,type_ignores=[])),str(path),'exec'),space)
    return type('Callback',(),{name:space[name] for name in names})()

noop=lambda *args,**kwargs:Mark()
draft=SimpleNamespace(polygon=polygon,polyline=noop,segments=noop,dashes=lambda p,*args:[p],W_HAIR=1)
pair=['2','1']
columns={p['tag']:p for p in G['pilotis']}
s=columns[pair[1]]['x']-columns[pair[0]]['x']
width=M['car']['width']['value']
radii=[max(p['profile']['diameter']['value']/2 for p in columns[tag]['parts'] if p['z0']<1.65 and p['z1']>0) for tag in pair]
proxy=SimpleNamespace(part_band_footprints=MODEL.part_band_footprints,part_outline_at=MODEL.part_outline_at,clearance_limits=lambda *args,**kwargs:{})
info=compile_methods('📊️information',['_bay'],{'_model':proxy,'_draft':draft,'_engine':SimpleNamespace(VGroup=Mark),'BAY_K':.5,'BAY_X':3.65,'BAY_Y':.85,'MUTED':'muted','CONCRETE':'concrete','INK':'ink','DATA':'data'},('_bay','span'))
tracker=lambda value:SimpleNamespace(get_value=lambda:value)
info.st=SimpleNamespace(s=tracker(s),d=tracker(sum(radii)),w=tracker(width),rv={})
info._m,info._g,info._entry=M,G,pair
info._c={'car_length':M['car']['length']['value']}
info._say=noop
info._lab=lambda tag:tag
info._bay()
actual=unary_union([shape for color,shape in records if color=='concrete'])
expected=[]
for tag,x in zip(pair,(3.65-s*.5/2,3.65+s*.5/2)):
    p=columns[tag]
    for part in p['parts']:
        if part['z0']<1.65 and part['z1']>0:
            shape=Polygon(part['outline'],part.get('holes',[]))
            expected.append(affine_transform(shape,[.5,0,0,.5,x-.5*p['x'],.85-.5*p['y']]))
expected=unary_union(expected)
assert actual.symmetric_difference(expected).area<1e-9
assert not actual.covers(Point(3.65-s*.5/2,.85))
assert actual.covers(Point(3.65+s*.5/2,.85))
print('PASS actual entry drawing: GEOS material union, empty100mm bore, solid neighbour, no degenerate polygons')

records.clear()
wall=next(w for w in G['walls'] if w['tag']=='W-S')
op=wall['openings'][0]
fill=next(f for f in G['fillings'] if f['tag']==op['fill_tag'])
aux=[b for b in G['beams'] if MODEL.element(b['tag'],M).get('geometry',{}).get('section_sweep',{}).get('wall')=='W-S']
rep=compile_methods('👁️representations',['_wall_local_ring','_clip_station','_cut_plan'],{'_drafting':draft,'_engine':SimpleNamespace(VGroup=Mark),'TAG':'W-S','P':DRAFT_GEOMETRY.P,'GLASS':'glass','ARCH':'arch','DATA':'data','W_THIN':1,'INK':'ink'})
rep.m=SimpleNamespace(base=M,geo=G,wall=wall,window=op['fill_tag'])
rep.hatch=noop
origin=np.asarray(wall['start'])
axis=(np.asarray(wall['end'])-origin)/wall['length']
normal=np.array([-axis[1],axis[0]])
x_from=wall['length']-3
for h in (.5,.72,.82,1.465,2.1):
    records.clear()
    rep._cut_plan(h,0,0,1,x_from)
    z=wall['base']+h
    expected=[]
    for item in [wall,fill,*aux]:
        for part in item['parts']:
            if part['z0']<=z<=part['z1']:
                assert not {'section','top_outline','mesh'}&set(part)
                shape=Polygon(part['outline'],part.get('holes',[]))
                local=affine_transform(shape,[axis[0],axis[1],normal[0],normal[1],-float(np.dot(origin,axis)),-float(np.dot(origin,normal))+wall['thickness']/2])
                cropped=local.intersection(box(x_from,-100,100,100))
                if cropped.area>0:
                    expected.append(affine_transform(cropped,[1,0,0,1,-x_from,0]))
    expected=unary_union(expected)
    actual=unary_union([shape for _,shape in records])
    assert actual.symmetric_difference(expected).area<1e-8,(h,actual.area,expected.area)
print('PASS five actual facade cut callbacks: GEOS occupied wall/filling/L/U/shelf union and native crop, no collapsed polygons')

import copy
Mark.move_to=lambda self,*args,**kwargs:self
proxy=SimpleNamespace(**{name:getattr(MODEL,name) for name in ('load_model','partial_model','building_geometry','role','clash_check')},translate_structure=lambda *args:{},tributary=lambda *args:{})
lesson=compile_methods('🧩️models',['_prepare'],{'_model':proxy,'copy':copy,'DISCIPLINES':('architecture','structure','services'),'VGroup':Mark})
lesson._prepare()
records.clear()
check=compile_methods('🧩️models',['_check'],{'VGroup':Mark,'Circle':Mark,'polygon':polygon,'segments':noop,'P':DRAFT_GEOMETRY.P,'WARN':'warn','BACKGROUND':'background','STRUCT':'structure','SERV':'services'},('_check','pipe_radius'))
check.__dict__.update(vars(lesson))
check.plan=Mark()
check._ground_plan=lambda **kwargs:SimpleNamespace(to_screen=lambda x,y:np.array([x,y,0.]))
check.remove=check.add=noop
check.text=check.dimension=noop
check.quantity=lambda value,unit:str(value)
cut=check._check()
pipe_material=unary_union([shape for color,shape in records if color=='services'])
column_material=unary_union([shape for color,shape in records if color=='structure'])
assert not pipe_material.covers(Point(cut['at'](cut['pipe']['points'][0][:2])[:2]))
assert column_material.covers(Point(cut['at']([cut['piloti']['x'],cut['piloti']['y']])[:2]))
for point,material in ((lesson.clash['a_point'],pipe_material),(lesson.clash['b_point'],column_material)):
    assert material.boundary.distance(Point(cut['at'](point[:2])[:2]))<1e-7
assert abs(np.linalg.norm(np.asarray(lesson.clash['a_point'])[:2]-np.asarray(lesson.clash['b_point'])[:2])-lesson.clash['distance'])<1e-7
print('PASS actual hypothetical clash preparation/drawing: coherent hosted reroute, hollow pipe, solid source19, actual material-boundary dimension witnesses')

records.clear()
entrance_draft=SimpleNamespace(View=lambda *args,**kwargs:lambda x,y,z:(x,y,0),polygon=polygon)
entrance=compile_methods('📊️information',['_wall_detail'],{'_draft':entrance_draft,'_engine':SimpleNamespace(VGroup=Mark),'_panels':SimpleNamespace(frame_box=noop),'PX':0,'PW':6,'MUTED':'muted','INK':'ink','GLASS':'glass','LABEL':12,'SMALL':10})
entrance._m,entrance._g=M,G
entrance._say=noop
entrance.tr=lambda en,de:en
entrance._wall_detail()
z=G['storeys']['ground']+1.2
parts=[*G['curtain_wall']['parts'],*MODEL.filling_geometry('D-ENTRY',M)['parts']]
expected=[]
for part in parts:
    if part['z0']<=z<=part['z1']:
        assert not {'section','top_outline','mesh'}&set(part)
        expected.append(Polygon(part['outline'],part.get('holes',[])))
actual=unary_union([shape for _,shape in records])
assert actual.symmetric_difference(unary_union(expected)).area<1e-8
assert any(part.get('role') in ('panel','batten') for part in MODEL.filling_geometry('D-ENTRY',M)['parts'])
print('PASS actual native entrance material callback: GEOS surround and timber leaves, record paths, no artificial inter-path gap')

records.clear()
column_draft=SimpleNamespace(polygon=polygon,segments=noop,hatch=noop,circle_points=noop,polyline=noop,W_HAIR=1,W_EDGE=1)
column_scene=compile_methods('📊️information',['_column'],{'_draft':column_draft,'_engine':SimpleNamespace(VGroup=Mark,Circle=Mark,Arrow=Mark),'BACKGROUND':'background','INK':'ink','CONCRETE':'floor','STRUCT':'structure','WARN':'warn','DATA':'data','MUTED':'muted','LABEL':12,'SMALL':10,'TONES':{'sourced':'source','inferred':'inferred','assumed':'assumed'}})
column_scene._m,column_scene._g=M,G
column_scene._say=column_scene.dimension=noop
column_scene.quantity=lambda value,unit:str(value)
column_scene._word=lambda value:value
column_scene._column()
from shapely.geometry import LineString
selected=next(p for p in G['pilotis'] if p['tag']==MODEL.role('column_example',M))
floor=next(p for p in G['slabs'] if p['tag']==MODEL.role('floor',M))
expected=[]
for part in floor['parts']:
    line=Polygon(part['outline'],part.get('holes',[])).intersection(LineString([[-100,selected['y']],[100,selected['y']]]))
    for segment in getattr(line,'geoms',[line]):
        if segment.geom_type!='LineString' or segment.length<1e-12:
            continue
        left,_,right,_=segment.bounds
        shape=box(left,part['z0'],right,part['z1']).intersection(box(selected['x']-.62/.7,-100,selected['x']+.62/.7,100))
        if shape.area>0:
            expected.append(affine_transform(shape,[.7,0,0,.7,1.55-.7*selected['x'],-1.95-.7*selected['base']]))
actual=unary_union([shape for color,shape in records if color=='floor'])
assert actual.symmetric_difference(unary_union(expected)).area<1e-9
print('PASS actual selected-column floor section: independent GEOS native void/region cuts and local crop, source diameter basis')
