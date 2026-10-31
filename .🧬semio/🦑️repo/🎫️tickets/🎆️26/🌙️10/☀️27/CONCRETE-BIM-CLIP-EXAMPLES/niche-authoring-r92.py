"""🚿 Author original wall-basin niches through existing first-party geometry interfaces."""
import importlib,importlib.util,json,math,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();ticket=Path(__file__).parent;spec=importlib.util.spec_from_file_location('tap_authoring',ticket/'tap-parts-r92.py');factory=importlib.util.module_from_spec(spec);spec.loader.exec_module(factory)
case=json.loads((root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🚿️niches.json').read_text(encoding='utf8'));assets={}
def ellipse(cx,cy,rx,ry,n=48):
    """⭕ Sample explicitly inferred ceramic construction from a source-qualified fixture outline."""
    return [[round(cx+rx*math.cos(2*math.pi*i/n),12),round(cy+ry*math.sin(2*math.pi*i/n),12)]for i in range(n)]
for row in case['basins']:
    cx,cy=row['axis'];rx,ry=[v/2 for v in row['size']];parts=[];outer=ellipse(cx,cy,rx,ry);hole=ellipse(cx,cy,.025,.025)
    def ring(outer,inner,z0,z1,role,top_outer=None,top_inner=None):
        """🧱 Keep each annular band closed and its central cavity physically empty."""
        for i in range(len(outer)):
            j=(i+1)%len(outer);parts.append({'outline':[outer[i],outer[j],inner[j],inner[i]],'z0':z0,'z1':z1,'material':'MAT-SANITARY-CERAMIC','role':role,**({'top_outline':[top_outer[i],top_outer[j],top_inner[j],top_inner[i]]}if top_outer else {})})
    ring(ellipse(cx,cy,rx*.7,ry*.7),hole,.69,.71,'basin_floor');ring(ellipse(cx,cy,rx*.7,ry*.7),ellipse(cx,cy,rx*.62,ry*.62),.71,case['rim'],'basin_shell',outer,ellipse(cx,cy,rx*.9,ry*.9));ring(ellipse(cx,cy,.032,.032),hole,.67,.69,'drain_neck')
    bx,by=row['back'];limit=.13 if by>0 else (.12 if by<0 else .17)
    clip=([[cx-rx-.02,cy+limit],[cx+rx+.02,cy+limit],[cx+rx+.02,cy+ry+.02],[cx-rx-.02,cy+ry+.02]]if by>0 else [[cx-rx-.02,cy-ry-.02],[cx+rx+.02,cy-ry-.02],[cx+rx+.02,cy-limit],[cx-rx-.02,cy-limit]]if by<0 else [[cx+limit,cy-ry-.02],[cx+rx+.02,cy-ry-.02],[cx+rx+.02,cy+ry+.02],[cx+limit,cy+ry+.02]])
    cap=a._analysis._clip(outer,clip,digits=None);holes=[ellipse(x,y,case['diameter']/2,case['diameter']/2,case['segments'])for x,y in row['mounts']];deck=a.profile_mesh([cap,*holes])
    for triangle in deck['triangles']:parts.append({'outline':[deck['vertices'][i]for i in triangle],'z0':case['rim'],'z1':case['rim']+.02,'material':'MAT-SANITARY-CERAMIC','role':'tap_deck'})
    def box(x0,y0,x1,y1,z0,z1):
        """🔩 Add inferred supporting metal without interpenetrating bowl or cabinet."""
        parts.append({'outline':[[x0,y0],[x1,y0],[x1,y1],[x0,y1]],'z0':z0,'z1':z1,'material':'MAT-METAL','role':'basin_bracket'})
    if row['tag']=='FIX-SERVANT-BASIN-1':
        for x in [cx-.13,cx+.13]:box(x-.01,cy-.09,x+.01,16.01,.67,.69)
    elif row['tag']=='FIX-SERVANT-BASIN-2':
        for x in [cx-.13,cx+.13]:box(x-.01,15.69,x+.01,15.71,.49,.67);box(x-.01,15.69,x+.01,cy+.06,.67,.69)
    else:
        for y in [cy-.11,cy+.11]:box(15.89,y-.01,15.915,y+.01,.49,.67);box(cx-.06,y-.01,15.915,y+.01,.67,.69)
    taps=[]
    for x,y in row['mounts']:
        outlet=[x-.16*bx,y-.16*by,case['joint']];taps+=factory.tap_parts(a,m,[x,y,case['rim']],[x,y,case['joint']],outlet,case['diameter'],case['wall_thickness'],case['segments'])
    assets[row['tag']]=parts;assets[row['tap']]=taps
(ticket/'🗑️generated'/'niche-assets-r92.json').write_text(json.dumps(assets,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:len(v)for k,v in assets.items()}),flush=True)
