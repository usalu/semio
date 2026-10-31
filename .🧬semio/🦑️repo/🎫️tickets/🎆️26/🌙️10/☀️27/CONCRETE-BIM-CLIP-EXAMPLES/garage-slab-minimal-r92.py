"""🧬 Minimize the actual source operand regression without altering any production geometry."""
import importlib,json,sys
from pathlib import Path
from shapely.geometry import Polygon
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');ticket=Path(__file__).parent;data=json.loads((ticket/'🗑️generated/garage-slab-topology-r92.json').read_text(encoding='utf8'))['operands'][0]
def invalid(pieces,voids):
    """🔎 Keep the independently observed GEOS failure as the reduction oracle."""
    return any(not Polygon(p).is_valid for p in api._surface_difference(pieces,voids))
pieces=[next(p for p in data['pieces']if invalid([p],data['voids']))];voids=data['voids'][:]
for void in voids[:]:
    candidate=[v for v in voids if v is not void]
    if invalid(pieces,candidate):voids=candidate
out={'pieces':pieces,'voids':voids,'previous_output':api._surface_difference(pieces,voids)};(ticket/'🗑️generated/garage-slab-minimal-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(out))
