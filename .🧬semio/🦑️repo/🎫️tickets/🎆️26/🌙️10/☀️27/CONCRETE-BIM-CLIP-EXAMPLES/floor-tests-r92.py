"""🪵 Run native floor acceptance contracts."""
import importlib,sys,traceback,time,json
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
m=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪵️floors');out=[]
for name in sorted(n for n in dir(m) if n.startswith('test_') and (len(sys.argv)<2 or sys.argv[1] in n)):
 t=time.perf_counter()
 try:getattr(m,name)();row={'test':name,'passed':True}
 except Exception:row={'test':name,'passed':False,'traceback':traceback.format_exc()}
 row['seconds']=time.perf_counter()-t;out.append(row);print('[DEBUG] '+json.dumps(row),flush=True)
(Path(__file__).parent/'🗑️generated/floor-tests-r92.json').write_text(json.dumps(out),encoding='utf8')
raise SystemExit(not all(r['passed'] for r in out))
                                                                                                                                                                     