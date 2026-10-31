"""🧬 Validate the complete current physical source with independent Draft202012 JSON Schema."""
import hashlib,importlib.util,json,sys,time
from pathlib import Path
from jsonschema import Draft202012Validator
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());ticket=Path(__file__).parent;model=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model';paths=[model/'🔣️.json',model/'🧬️schema/🔣️.schema.json',Path(__file__),ticket/'fixture-fits-author-r95.py']+[p for p in model.rglob('*.py')if '🧪️tests'not in str(p)]+[model/'🧪️tests'/name for name in ('🔧️fixture-fits.py','🔧️fixture-fits.json','🔧️fixture-fits.schema.json')]
def hashes():
    """🔐 Bind the actual whole source and independent validation grammar."""
    return {str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();source=json.loads(paths[0].read_text(encoding='utf8'));spec=importlib.util.spec_from_file_location('fixture_candidate',ticket/'fixture-fits-author-r95.py');author=importlib.util.module_from_spec(spec);sys.path.insert(0,str(root));spec.loader.exec_module(author);source=author.apply(source);schema=json.loads(paths[1].read_text(encoding='utf8'));Draft202012Validator.check_schema(schema);start=time.monotonic();errors=[{'path':list(e.absolute_path),'schema_path':list(e.absolute_schema_path),'message':e.message[:1000]}for e in Draft202012Validator(schema).iter_errors(source)];after=hashes();out={'pass':not errors and before==after,'stable':before==after,'before':before,'after':after,'products':len(source['elements']),'errors':errors,'seconds':time.monotonic()-start};(ticket/'🗑️generated/fixture-candidate-schema-r95.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k not in('before','after','errors')}));print('[DEBUG] schema_errors='+str(len(errors)));raise SystemExit(not out['pass'])
