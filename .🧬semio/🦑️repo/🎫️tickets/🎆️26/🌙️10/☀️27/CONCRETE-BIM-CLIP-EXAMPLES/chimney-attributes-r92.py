"""🪔 Execute the original low-terminal contract on the current native source."""
import hashlib,importlib,json,sys,time,traceback
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
stage=sys.argv[1];out=Path(__file__).parent/'🗑️generated';source=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';before=hashlib.sha256(source.read_bytes()).hexdigest();start=time.monotonic();error=None
try:importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚬️prisms').test_all_native_chimneys_export_exact_nine_attribute_schema()
except Exception:error=traceback.format_exc();print(error,flush=True)
result={'source_before':before,'source_after':hashlib.sha256(source.read_bytes()).hexdigest(),'stage':stage,'seconds':time.monotonic()-start,'error':error,'pass':error is None};(out/('chimney-attributes-'+stage+'-r92.json')).write_text(json.dumps(result,indent=2),encoding='utf8');print(json.dumps(result),flush=True);raise SystemExit(bool(error))
