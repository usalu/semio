"""🗄️ Execute the isolated pantry boundary RED and exact candidate GREEN through the actual registered contract."""
import hashlib,importlib,importlib.util,json,sys,time,traceback
from copy import deepcopy
from pathlib import Path
sys.dont_write_bytecode=True
R=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(R));P=Path(__file__).parent;M=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model';A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔧️fixture-fits');paths=[M/'🔣️.json',Path(__file__),P/'fixture-fits-author-r95.py']+[M/'🧪️tests'/n for n in('🔧️fixture-fits.py','🔧️fixture-fits.json','🔧️fixture-fits.schema.json')]+[p for p in M.rglob('*.py')if '🧪️tests'not in str(p)]
def hashes():
    """🔐 Bind all actual producer, source and exact contract bytes."""
    return{str(p.relative_to(R)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();original=A.api.load_model;source=original();rows=[];spec=importlib.util.spec_from_file_location('pantry_author',P/'fixture-fits-author-r95.py');author=importlib.util.module_from_spec(spec);spec.loader.exec_module(author)
for mode in('original','candidate'):
    start=time.monotonic();value=source if mode=='original'else author.apply(source);A.api.load_model=lambda:deepcopy(value)
    try:A.test_pantry_storage_rear_plane_clears_the_complete_source_wall_envelope();row={'mode':mode,'passed':True,'error':None}
    except AssertionError:row={'mode':mode,'passed':False,'error':traceback.format_exc()}
    finally:A.api.load_model=original
    row['seconds']=time.monotonic()-start;rows.append(row)
after=hashes();out={'pass':not rows[0]['passed']and rows[1]['passed']and before==after,'stable':before==after,'rows':rows,'before':before,'after':after,'scope':'Only the exact pantry source boundary contract with independent GEOS; no full wall-body or whole-building claim.'};(P/'🗑️generated/fixture-storage-red-green-r95.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k not in('before','after')},ensure_ascii=True),flush=True);raise SystemExit(not out['pass'])
