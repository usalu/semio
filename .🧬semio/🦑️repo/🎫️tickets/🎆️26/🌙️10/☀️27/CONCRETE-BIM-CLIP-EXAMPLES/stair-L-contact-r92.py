"""🪜 Locate the source-registered L support against the actual native stair underside."""
import importlib,json,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
from shapely.geometry import Polygon,LineString
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();g=api._stair_geometry(m,api.element('ST-FR',m));outline=json.loads((Path(__file__).parent/'structural-service-cases-r92.json').read_text())['missing_stair_L']['outline_candidate_m'];shape=Polygon(outline);rows=[]
for p in outline+[[6.22,10.18],[6.37,10.18],[6.38,10.30],[6.347144829,10.210220372]]:
    hits=[]
    for part in g['structural_parts']:
        for ring in api.part_sections(part,p[0],0):
            cut=Polygon(ring).intersection(LineString([[p[1],3.14],[p[1],6.6]]))
            if not cut.is_empty:hits.append(cut.bounds[1])
    rows.append({'xy':p,'underside':min(hits) if hits else None,'hits':hits})
result={'outline':outline,'rows':rows};(Path(__file__).parent/'🗑️generated/stair-L-contact-r92.json').write_text(json.dumps(result,indent=2),encoding='utf8');print(json.dumps(result),flush=True)
