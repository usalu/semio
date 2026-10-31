"""🧱 Retain rejected derived wall cells before correcting native construction composition."""
import importlib,json,sys,hashlib,time,traceback
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
source=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';apipath=source.with_name('🐍️.py');out={'before':{'source':hashlib.sha256(source.read_bytes()).hexdigest(),'api':hashlib.sha256(apipath.read_bytes()).hexdigest()},'walls':[]};api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();start=time.monotonic()
import numpy as np
from scipy.spatial import ConvexHull
for tag in ('W-G-GARAGE','W-G-GARAGE-LOWER','W-PARENT-BATH','W-TERRACE-RAMP'):
    print('[DEBUG] Wall operand capture '+tag,flush=True);g=api._wall_geometry(m,api._find(m,tag));row={'tag':tag,'parts':len(g['parts']),'rejected':[], 'reference_parts':g.get('interface_reference_parts'), 'interface_owners':api._find(m,tag)['geometry'].get('interface_owners')}
    for index,part in enumerate(g['parts']):
        if not any(key in part for key in ('mesh','section','top_outline')):continue
        vertices=np.asarray(api.part_vertices(part));origin=vertices.min(axis=0);spans=vertices.max(axis=0)-origin
        try:
            if np.any(spans<=0):raise ValueError('Zero world extent')
            hull=ConvexHull((vertices-origin)/spans)
            api._polyhedron_part(api.part_vertices(part),api.part_faces(part))
            if api._polyhedron_volume(part)<=0:raise ValueError('Nonpositive occupied cell')
        except Exception as error:row['rejected'].append({'index':index,'part':part,'native_volume':api._polyhedron_volume(part),'error':str(error).splitlines()[0]})
    out['walls'].append(row)
out.update(seconds=time.monotonic()-start,after={'source':hashlib.sha256(source.read_bytes()).hexdigest(),'api':hashlib.sha256(apipath.read_bytes()).hexdigest()});(Path(__file__).parent/'🗑️generated'/('wall-rejected-native-'+(sys.argv[1] if len(sys.argv)>1 else 'capture')+'-r92.json')).write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps({**{k:v for k,v in out.items()if k!='walls'},'walls':[{**r,'rejected':[{'index':p['index'],'volume':p['native_volume'],'error':p['error']}for p in r['rejected']]}for r in out['walls']]}))
