"""🧩 Execute the actual reviewed domain-ownership regression against current source."""
import hashlib,importlib,json,sys,time,traceback
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;model=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model';paths=[model/'🔣️.json',model/'🧬️schema/🔣️.schema.json',model.parent/'🧪️tests/🐍️.py',Path(__file__),*[model/'🧪️tests'/('📚️references'+ext)for ext in('.py','.json','.schema.json')]]
paths += [p for p in model.rglob('*.py')if '🧪️tests'not in str(p)]
def hashes():
    """🔐 Bind the actual source, neutral input and test body."""
    return {str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();start=time.monotonic();error=None
try:importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📚️references').test_qualified_citation_grammar_matches_language_neutral_and_independent_schema_controls()
except Exception:error=traceback.format_exc();print('[DEBUG] '+error,flush=True)
after=hashes();out={'pass':error is None and before==after,'error':error,'tests':1,'before':before,'after':after,'stable':before==after,'seconds':time.monotonic()-start};(ticket/'🗑️generated'/('source-reference-'+sys.argv[1]+'-r95.json')).write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k not in('before','after','error')}));raise SystemExit(not out['pass'])
