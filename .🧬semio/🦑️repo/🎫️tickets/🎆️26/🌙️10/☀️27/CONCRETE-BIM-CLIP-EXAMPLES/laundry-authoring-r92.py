"""🧺 Author the source-qualified longitudinal formed-concrete laundry pair."""
import importlib,importlib.util,json,math,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();ticket=Path(__file__).parent;spec=importlib.util.spec_from_file_location('tap_authoring',ticket/'tap-parts-r92.py');factory=importlib.util.module_from_spec(spec);spec.loader.exec_module(factory);case=json.loads((root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧺️laundry.json').read_text(encoding='utf8'));parts=[];assets={}
def rect(x0,y0,x1,y1):
    """▭ Keep exact contact boundaries for a formed concrete component."""
    return [[x0,y0],[x1,y0],[x1,y1],[x0,y1]]
def circle(x,y,r,n=32):
    """⭕ Preserve an inferred real through-bore with finite material thickness."""
    return [[x+r*math.cos(2*math.pi*i/n),y+r*math.sin(2*math.pi*i/n)]for i in range(n)]
def band(outline,z0,z1,role,holes=None):
    """🧱 Triangulate explicit holes while preserving native closed disjoint concrete."""
    if holes:
        mesh=a.profile_mesh([outline,*holes]);polygons=[[mesh['vertices'][i]for i in tri]for tri in mesh['triangles']]
    else:polygons=[outline]
    parts.extend({'outline':p,'z0':z0,'z1':z1,'material':'MAT-RENDERED','role':role}for p in polygons)
for row in case['bowls']:
    x0,y0,x1,y1=row['bounds'];z0,z1=case['floor'];wall=case['wall'];back=case['back_deck'];mx,my,_=row['mount'];dx,dy,_=row['drain'];band(rect(x0,y0,x1,y1),z0,z1,'trough_floor',[circle(dx,dy,.025,48)]);band(rect(x0,y0,x0+back,y1),z1,case['rim'],'trough_rear_deck',[circle(mx,my,.0125)]);band(rect(x0+back,y0,x1-wall,y0+wall),z1,case['rim'],'trough_wall');band(rect(x0+back,y1-wall,x1-wall,y1),z1,case['rim'],'trough_wall');band(rect(x1-wall,y0,x1,y1),z1,case['rim'],'trough_wall')
    for x in [x0+.04,x1-.14]:
        for y in [y0+.04,y1-.14]:band(rect(x,y,x+.10,y+.10),0,z0,'trough_leg')
    assets[row['tap']]=factory.tap_parts(a,m,[mx,my,.93],[mx,my,row['outlet'][2]],row['outlet'])
assets[case['tag']]=parts;(ticket/'🗑️generated'/'laundry-assets-r92.json').write_text(json.dumps(assets,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:len(v)for k,v in assets.items()}),flush=True)
