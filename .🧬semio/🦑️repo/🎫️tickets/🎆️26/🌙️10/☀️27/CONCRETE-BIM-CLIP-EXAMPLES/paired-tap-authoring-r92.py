"""🏺 Author source-registered pedestal ellipses and paired mounted hollow taps."""
import importlib,importlib.util,json,math,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();ticket=Path(__file__).parent;spec=importlib.util.spec_from_file_location('tap_authoring',ticket/'tap-parts-r92.py');factory=importlib.util.module_from_spec(spec);spec.loader.exec_module(factory)
case=json.loads((root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🚰taps.json').read_text(encoding='utf8'));assets={}
def ellipse(cx,cy,rx,ry,n=48):
    """⭕ Preserve a sampled original oval and its inferred construction rings."""
    return [[round(cx+rx*math.cos(2*math.pi*i/n),12),round(cy+ry*math.sin(2*math.pi*i/n),12)]for i in range(n)]
for row in case['rows']:
    cx,cy=row['axis'];rx,ry=[v/2 for v in row['size']];parts=[]
    def ring(outer,inner,z0,z1,role,top_outer=None,top_inner=None):
        """🧱 Author one conserved annular material band without filling its cavity."""
        for i in range(len(outer)):
            j=(i+1)%len(outer);parts.append({'outline':[outer[i],outer[j],inner[j],inner[i]],'z0':z0,'z1':z1,'material':'MAT-SANITARY-CERAMIC','role':role,**({'top_outline':[top_outer[i],top_outer[j],top_inner[j],top_inner[i]]}if top_outer else {})})
    hole=ellipse(cx,cy,.025,.025);outer=ellipse(cx,cy,rx,ry)
    ring(ellipse(cx,cy,rx*.7,ry*.7),hole,.69,.71,'basin_floor');ring(ellipse(cx,cy,rx*.7,ry*.7),ellipse(cx,cy,rx*.62,ry*.62),.71,row['rim'],'basin_shell',outer,ellipse(cx,cy,rx*.9,ry*.9));ring(ellipse(cx,cy,.13,.16),hole,0,.04,'basin_foot');ring(ellipse(cx,cy,.085,.11),hole,.04,.69,'basin_pedestal')
    cap=a._analysis._clip(outer,[[cx-rx-.02,cy-ry-.02],[cx-.16,cy-ry-.02],[cx-.16,cy+ry+.02],[cx-rx-.02,cy+ry+.02]],digits=None);mounts=[[cx+case['stem_offset_x'],cy+off]for off in case['stem_offsets_y']];holes=[ellipse(x,y,case['diameter']/2,case['diameter']/2,case['segments'])for x,y in mounts];deck=a.profile_mesh([cap,*holes])
    for triangle in deck['triangles']:parts.append({'outline':[deck['vertices'][i]for i in triangle],'z0':row['rim'],'z1':row['rim']+.02,'material':'MAT-SANITARY-CERAMIC','role':'tap_deck'})
    taps=[]
    for x,y in mounts:taps+=factory.tap_parts(a,m,[x,y,row['rim']],[x,y,case['joint_z']],[cx-.06,y,case['joint_z']],case['diameter'],case['wall'],case['segments'])
    assets[row['basin']]=parts+(taps if row['owned'] else [])
    if not row['owned']:assets[row['tap']]=taps
(ticket/'🗑️generated'/'paired-tap-assets-r92.json').write_text(json.dumps(assets,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:len(v)for k,v in assets.items()}),flush=True)
