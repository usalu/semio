"""⚡️ Verify affected slab and complete-geometry cache isolation contracts."""
import hashlib,importlib,json,sys,time,unittest
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;p=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py';before=hashlib.sha256(p.read_bytes()).hexdigest();suite=unittest.TestSuite();start=time.monotonic()
for name in ('⚡️slab-members','⚡️slabs','⚡️geometry-cache'):suite.addTests(unittest.defaultTestLoader.loadTestsFromName('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.'+name))
r=unittest.TextTestRunner(verbosity=2).run(suite);report={'tests':r.testsRun,'success':r.wasSuccessful(),'producer_before':before,'producer_after':hashlib.sha256(p.read_bytes()).hexdigest(),'seconds':time.monotonic()-start};(ticket/'🗑️generated/slab-cache-cohort-r92.json').write_text(json.dumps(report,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(report));raise SystemExit(not r.wasSuccessful())
