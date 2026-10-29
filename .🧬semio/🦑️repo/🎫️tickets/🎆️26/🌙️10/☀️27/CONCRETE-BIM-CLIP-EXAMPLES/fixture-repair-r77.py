"""🧪 Independently prove the physical fixture candidate before any on-disk model mutation."""
import importlib,json,time,math
from copy import deepcopy
from pathlib import Path
import numpy as np
from shapely.geometry import Polygon,Point,LineString,box
from shapely.ops import unary_union
import jsonschema
root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path(__file__).parent;out=ticket/'🗑️generated';case=json.loads((ticket/'fixture-repair-r77.json').read_text());api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');started=time.monotonic();events=[]
def event(stage,**values):
    events.append({'stage':stage,'seconds':time.monotonic()-started,**values});(out/'fixture-repair-r77-progress.json').write_text(json.dumps(events))
def find(model,tag):return next(e for e in model['elements'] if e['tag']==tag)
def rectangle(bounds):
    x0,y0,x1,y1=bounds;return [[x0,y0],[x1,y0],[x1,y1],[x0,y1]]
def candidate(model):
    result=deepcopy(model)
    for tag in case['laundry']['tags']:
        for part in find(result,tag)['geometry']['parts']:part['outline']=[[x,y+case['laundry']['translation'][1]] for x,y in part['outline']]
    basin=find(result,case['guest']['basin']);old=basin['geometry']['parts'];x0,y0,x1,y1=case['guest']['new_bounds'];t=case['guest']['rim'];bounds=[[x0,y0,x1,y1],[x0,y0,x1,y0+t],[x0,y1-t,x1,y1],[x0,y0+t,x0+t,y1-t],[x1-t,y0+t,x1,y1-t]]
    basin['geometry']['parts']=[{**part,'outline':rectangle(bound)} for part,bound in zip(old,bounds)]
    tap=find(result,case['guest']['tap']);tap['geometry']['parts']=[{**part,'outline':rectangle(case['guest'][key])} for part,key in zip(tap['geometry']['parts'],('tap_stem_bounds','tap_spout_bounds'))]
    opening=find(result,case['guest']['opening'])
    for key in ('start','end'):opening['attributes'][key]['value']+=case['guest']['station_delta']
    return result

def shape(part):return Polygon(part['outline'],part.get('holes',[]))
def intersect(a,b):return shape(a).intersection(shape(b)).area*max(0,min(a['z1'],b['z1'])-max(a['z0'],b['z0']))
def parts(model,tag):return api.solid_geometry(tag,model)['parts']
def volume(items):
    levels=sorted({p[k] for p in items for k in ('z0','z1')})
    return sum(unary_union([shape(p) for p in items if p['z0']<(lo+hi)/2<p['z1']]).area*(hi-lo) for lo,hi in zip(levels,levels[1:]))
original=api.load_model();changed=candidate(original)
old_laundry=parts(original,'FIX-LAUNDRY');column32=api.column_geometry(original,find(original,'32'))['parts'];old_overlap=sum(intersect(a,b) for a in old_laundry for b in column32)
assert math.isclose(old_overlap,case['laundry']['old_column_overlap'],abs_tol=1e-10)
clear=box(*case['guest']['clear_bounds']);old_guest=unary_union([shape(p) for p in parts(original,case['guest']['basin'])]);assert old_guest.intersection(clear).area==0
event('RED source defects reproduced',laundry_column_volume=old_overlap,guest_outside_area=old_guest.difference(clear).area)
schema=json.loads((root/'🧱️model/🧬️schema/🔣️.schema.json').read_text());jsonschema.Draft202012Validator(schema).validate(changed);event('candidate schema PASS')
columns=[p for e in changed['elements'] if e['class']=='IfcColumn' for p in api.column_geometry(changed,e)['parts']];event('native columns ready',parts=len(columns))
walls={}
for level in ('ground','first'):
    walls[level]=[p for e in changed['elements'] if e['class']=='IfcWall' and e['storey']==level for p in api._wall_geometry(changed,e)['parts']]
event('native walls ready',ground=len(walls['ground']),first=len(walls['first']))
rooms={s['tag']:Polygon(s['outline']) for s in changed['spaces']};targets=[*case['laundry']['tags'],case['guest']['basin'],case['guest']['tap']];physical={tag:parts(changed,tag) for tag in targets};inventory={}
for tag in targets:
    obj=find(changed,tag);items=physical[tag];room=rooms[case['laundry']['room'] if tag in case['laundry']['tags'] else case['guest']['room']]
    assert all(room.covers(shape(p)) for p in items),tag
    overlap_column=sum(intersect(p,c) for p in items for c in columns);overlap_wall=sum(intersect(p,w) for p in items for w in walls[obj['storey']]);assert overlap_column<case['tolerances']['volume'],(tag,'column',overlap_column);assert overlap_wall<case['tolerances']['volume'],(tag,'wall',overlap_wall)
    neighbours=[p for e in changed['elements'] if e['storey']==obj['storey'] and e['class']=='IfcFurnishingElement' and e['tag']!=tag for p in parts(changed,e['tag'])]
    overlap_furniture=sum(intersect(p,q) for p in items for q in neighbours);assert overlap_furniture<case['tolerances']['volume'],(tag,'furniture',overlap_furniture)
    native_volume=volume(items);numeric_volume=0.
    levels=sorted({p[k] for p in items for k in ('z0','z1')})
    for low,high in zip(levels,levels[1:]):
        union=unary_union([shape(p) for p in items if p['z0']<(low+high)/2<p['z1']])
        polygons=list(union.geoms) if hasattr(union,'geoms') else [union]
        for polygon in polygons:
            for sign,ring in [(1,polygon.exterior),*[(-1,hole) for hole in polygon.interiors]]:
                xy=np.asarray(ring.coords)[:-1];numeric_volume+=sign*abs(np.dot(xy[:,0],np.roll(xy[:,1],1))-np.dot(xy[:,1],np.roll(xy[:,0],1)))/2*(high-low)
    assert math.isclose(native_volume,numeric_volume,abs_tol=1e-9),(tag,native_volume,numeric_volume)
    assert math.isclose(native_volume,api.quantities(tag,changed)['volume'],abs_tol=1e-8)
    inventory[tag]={'parts':len(items),'volume':native_volume,'column_overlap':overlap_column,'wall_overlap':overlap_wall,'furniture_overlap':overlap_furniture};event('fixture material PASS',tag=tag)
for group in ('laundry','guest'):
    record=case[group];tag=record.get('basin','FIX-LAUNDRY');items=find(changed,tag)['geometry']['parts']
    free=record.get('cavity_samples',[record.get('cavity_sample')]);occupied=record.get('occupied_samples',[record.get('occupied_sample')])
    for x,y,z in free:assert not unary_union([shape(p) for p in items if p['z0']<z<p['z1']]).intersects(Point(x,y).buffer(.04))
    for x,y,z in occupied:assert unary_union([shape(p) for p in items if p['z0']<z<p['z1']]).covers(Point(x,y).buffer(.02))
assert all(clear.covers(shape(p)) for p in physical[case['guest']['basin']]);slab=find(changed,case['guest']['slab']);native_clear=next(v for v in slab['geometry']['voids'] if v['tag']==case['guest']['void']);assert Polygon(native_clear['outline']).equals(clear)
opening=api.opening_geometry(case['guest']['opening'],changed);filling=api.filling_geometry(case['guest']['door'],changed);assert math.isclose(filling['clear_width'],case['guest']['clear_width'],abs_tol=1e-10)
for group,storey in (('laundry','ground'),('guest','first')):
    record=case[group];base=0 if storey=='ground' else 3.17;passage=LineString(record['route']).buffer(record['body_width']/2,cap_style=2);route_parts=walls[storey]+[p for e in changed['elements'] if e['storey']==storey and e['class']=='IfcFurnishingElement' for p in parts(changed,e['tag'])]+columns
    door_tag='D-LAUNDRY' if group=='laundry' else case['guest']['door'];fixed=api.filling_geometry(door_tag,changed)['parts'];route_parts += [p for p in fixed if p['role']=='frame']
    assert sum(passage.intersection(shape(p)).area for p in route_parts if p['z0']<base+record['body_height'] and p['z1']>base)<case['tolerances']['area'],group
    event('body route PASS',group=group)
standing=box(*case['guest']['standing_bounds']);assert rooms[case['guest']['room']].covers(standing)
assert all(standing.intersection(shape(p)).area<case['tolerances']['area'] for p in walls['first']+columns+[p for e in changed['elements'] if e['storey']=='first' and e['class']=='IfcFurnishingElement' for p in parts(changed,e['tag'])] if p['z0']<4.97 and p['z1']>3.17)
for group in ('laundry','guest'):
    record=case[group]
    for tag in (record['tags'][1:] if group=='laundry' else [record['tap']]):
        items=find(changed,tag)['geometry']['parts'];stem,spout=items;bounds=shape(spout).bounds
        point=Point((bounds[0]+bounds[2])/2,bounds[1] if group=='laundry' else bounds[3])
        bowl=find(changed,'FIX-LAUNDRY' if group=='laundry' else record['basin'])['geometry']['parts']
        z=.86 if group=='laundry' else .82
        assert not unary_union([shape(p) for p in bowl if p['z0']<z<p['z1']]).intersects(point.buffer(.02)),tag
for group,storey in (('laundry','ground'),('guest','first')):
    record=case[group];base=0 if storey=='ground' else 3.17;standing=box(*record['standing_bounds'])
    assert rooms[record['room']].covers(standing)
    obstacles=walls[storey]+columns+[p for e in changed['elements'] if e['storey']==storey and e['class']=='IfcFurnishingElement' for p in parts(changed,e['tag'])]
    assert all(standing.intersection(shape(p)).area<case['tolerances']['area'] for p in obstacles if p['z0']<base+record['body_height'] and p['z1']>base),group
event('tap discharge and standing bodies PASS')
assert original==api.load_model();event('candidate physical trial PASS')
record={'source_unchanged':True,'inventory':inventory,'old_laundry_column_overlap':old_overlap,'guest_clear_bounds':list(clear.bounds),'door_clear_width':filling['clear_width'],'door_rough':{k:find(changed,case['guest']['opening'])['attributes'][k]['value'] for k in ('start','end')},'seconds':time.monotonic()-started,'events':events}
(out/'fixture-repair-r77-trial.json').write_text(json.dumps(record));print(record)
