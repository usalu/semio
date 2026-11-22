"""🎼 Verify the native model vectors in single-worker groups with fresh memory grants."""
import os,sys,re,json,subprocess,shutil
from pathlib import Path
p=Path(__file__).resolve().parent;s=next(Path('.').glob('🎓*/*architecture/*staffage'));m=next(s.glob('*tests/*model-digests/*.ts'));f=json.loads(next(s.glob('*fixtures/*model-digests/*.json')).read_bytes())
base=[shutil.which('node'),'node_modules/vitest/vitest.mjs','run','--config',str(p/'vs-vitest.config.ts'),'--maxWorkers=1','--no-file-parallelism','-t']
groups=['covers every pose|pose .*GLB']+sorted({re.escape('.'.join(k.split('.')[:3])+'.') for k in f['fbx']})
for i,expr in enumerate(groups):
 log=p/'🗑️generated'/('moment-digest-vitest-'+str(i)+'.log')
 with log.open('w') as stream:r=subprocess.run(base+[expr],env={**os.environ,'VS_TEST':str(m)},stdout=stream,stderr=stream)
 if r.returncode:raise RuntimeError(str((expr,r.returncode,log.read_text()[-1200:])))
 print(expr,[l for l in log.read_text().splitlines() if 'Tests ' in l][-1].strip(),flush=True)