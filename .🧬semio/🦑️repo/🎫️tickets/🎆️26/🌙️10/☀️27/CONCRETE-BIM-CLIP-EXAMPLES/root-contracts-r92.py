"""🧪 Exercise strict authored inventory and discipline contracts with bounded native reports."""
import hashlib,importlib,json,sys,time,unittest
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent
paths=[root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'/p for p in ('🧱️model/🔣️.json','🧱️model/🐍️.py','🧱️model/🧮️analysis/🐍️.py','🧪️tests/🐍️.py','🧪️cases/🔣️.json')]
def signatures():
    """🔏 Bind the actual source, producers and assertions for each checkpoint."""
    return {str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
stage=sys.argv[1];before=signatures();started=time.monotonic();module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
classes=['BimModelStructureContract','BimGridContract','BimIdsContract','BimPartialModelContract'] if stage=='inventory' else ['BimConsistencyContract'] if stage=='consistency' else ['BimGeometryContract']
suite=unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromTestCase(getattr(module,name)) for name in classes)
stream=(ticket/'🗑️generated'/('root-contracts-'+stage+'-r92.log')).open('w',encoding='utf8')
try:result=unittest.TextTestRunner(stream=stream,verbosity=2).run(suite)
finally:stream.close()
after=signatures();report={'stage':stage,'source_before':before,'source_after':after,'stable':before==after,'seconds':time.monotonic()-started,'tests':result.testsRun,'failures':[{'id':t.id(),'trace':s} for t,s in result.failures],'errors':[{'id':t.id(),'trace':s} for t,s in result.errors],'pass':result.wasSuccessful()}
(ticket/'🗑️generated'/('root-contracts-'+stage+'-r92.json')).write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k not in ('source_before','source_after','failures','errors')}),flush=True)
for kind in ('failures','errors'):
    for row in report[kind]:print('[DEBUG] '+row['id']+'\n'+row['trace'],flush=True)
raise SystemExit(not result.wasSuccessful())
