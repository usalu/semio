"""🕳️ Author source-defined cellar cement finish while retaining structural world geometry."""
import importlib,json,copy,sys
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely import constrained_delaunay_triangles
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();ticket=Path(__file__).parent;probe=copy.deepcopy(m);datum=next(s for s in probe['storeys'] if s['tag']=='cellar');datum['elevation'].update(value=-2.4,basis='sourced',source='SAB2024-p99; CSIC2022 Annex6 finished cellar leader')
base_tags=[]
for e in probe['elements']:
 if e['geometry'].get('base_level')=='cellar':
  base_tags.append(e['tag'])
  if e['class']=='IfcFooting':
   for p in e['geometry']['parts']:p['z0']=round(p['z0']-.02,12);p['z1']=round(p['z1']-.02,12)
  else:e['geometry']['base_level']=-2.42
sl=a._find(probe,'SL-CELLAR');sl['material']='MAT-LIME-CONCRETE';sl['geometry']['top_offset'].update(value=-.02,basis='sourced',source='SAB2024-p66-p99 structural−2.42 finished−2.40')
for r in sl['geometry']['regions']:r['top_offset']['value']=-.02;r['material']='MAT-LIME-CONCRETE'
boiler=a._find(probe,'BL-1')
for point in boiler['geometry']['points']:point[2]=round(point[2]+.02,9)
stair=a._find(probe,'ST-CG');new=json.loads((ticket/'🗑️generated/cellar-stair-authored-r92.json').read_text())['geometry'];stair['geometry']['step_parts']=new['step_parts'];stair['geometry']['structural_parts']=new['structural_parts']
for tag in ['OP-CELLAR-STAIR-ACCESS','OP-HOIST-CELLAR','OP-COAL-CELLAR']:
 e=a._find(probe,tag);e['attributes']['sill']['value']=.02
 if tag=='OP-CELLAR-STAIR-ACCESS':e['attributes']['head']['value']=2.12
slab=a._slab_geometry(probe,sl);assert all(abs(p['z1']+2.42)<1e-8 for p in slab['full_parts']);assert abs(a.stair_geometry('ST-CG',probe)['top'])<1e-8
net=unary_union([Polygon(r) for p in slab['full_parts'] for r in a.part_regions(p,p['z1'])]);cuts=[]
for e in probe['elements']:
 if e['storey']!='cellar' or e['tag'] in ('SL-CELLAR','BL-1','ST-CG'):continue
 if e['class']=='IfcWall':parts=a._wall_geometry(probe,e)['parts']
 elif e['class']=='IfcColumn':parts=a.column_geometry(probe,e)['parts']
 elif e['class'] in ('IfcFooting','IfcBuildingElementProxy'):parts=a._solid_geometry(probe,e)['parts']
 else:continue
 for p in parts:
  if p['z0']<-2.41<p['z1']:
   if e['class']=='IfcColumn':cuts.append(Polygon(a.part_footprint(p)))
   else:cuts.extend(Polygon(r) for r in a.part_regions(p,-2.41))
net=net.difference(unary_union(cuts));parts=[{'outline':[list(xy) for xy in p.exterior.coords[:-1]],'z0':-.02,'z1':0.,'material':'MAT-CEMENT-SAND','role':'wearing'} for p in constrained_delaunay_triangles(net).geoms if p.area>1e-10]
e={'tag':'FIN-CELLAR-CEMENT','guid':a.global_id('FIN-CELLAR-CEMENT'),'class':'IfcCovering','predefined_type':'FLOORING','name':{'en':'Original cellar cement-sand wearing layer','de':'Ursprüngliche Zement-Sand-Nutzschicht im Keller'},'storey':'cellar','geometry':{'kind':'flooring','base_level':'cellar','parts':parts},'metadata':{'author':'Atelier (architecture)','revision':1,'domains':['architecture']},'material':'MAT-CEMENT-SAND'}
g=a._solid_geometry(probe,e);assert all(abs(p['z0']+2.42)<1e-8 and abs(p['z1']+2.4)<1e-8 for p in g['parts']);assert abs(sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in g['parts'])-net.area*.02)<1e-8
assert max(p['z1'] for p in a.stair_geometry('ST-CG',probe)['steps'])==0.
result={'native_preflight':True,'base_tags':base_tags,'slab':sl['geometry'],'boiler_points':boiler['geometry']['points'],'stair_parts':{'step_parts':stair['geometry']['step_parts'],'structural_parts':stair['geometry']['structural_parts']},'floor':e,'net_area':net.area,'limits':['Cellar finish/material/direct datums sourced; boiler lift is inferred contact correction.','Structural walls/columns/footings retain actual−2.42 world base; no blanket foundation shift.','ST-CG inferred13rises regenerated from2.40 finished-to-finished height;30mm finish reserve/60mm waist preserved.']};(ticket/'🗑️generated/cellar-floor-authoring-r92.json').write_text(json.dumps(result),encoding='utf8');print('[DEBUG] '+json.dumps({'cellar_native_preflight':True,'net_area':net.area,'floor_parts':len(parts),'preserved_structural_bases':base_tags}),flush=True)
