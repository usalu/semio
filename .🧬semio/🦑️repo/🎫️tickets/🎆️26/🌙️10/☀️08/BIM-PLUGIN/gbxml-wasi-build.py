from pathlib import Path
import json, re, subprocess
T=Path(__file__).resolve().parent; g=T/'🗑️generated'
r=json.loads(max((g/'compiler-retry').glob('w-svg-*.success.json'),key=lambda p:p.stat().st_mtime).read_text()); a=r['arguments']; rust=r['compiler']
dirs=[Path(a[i+1][11:]) for i,x in enumerate(a[:-1]) if x=='-L' and a[i+1].startswith('dependency=')]
infer=Path(next(a[i+1].split('=',1)[1] for i,x in enumerate(a[:-1]) if x=='--extern' and a[i+1].startswith('semio_s_artifact_bim_model_inferences=') and a[i+1].endswith('.rmeta')))
m=subprocess.check_output([rust,'-Zls=root',str(infer)],text=True)
hash=re.search(r'^\d+ semio_s_artifact_bim_model_snapshot-(\w+) hash ',m,re.M)[1]
snapshot=next(p for d in dirs for p in [d/('libsemio_s_artifact_bim_model_snapshot-'+hash+'.rlib')] if p.is_file())
archive=g/'refresh-wasi-energy-final/libsemio_s_artifact_bim_model_gbxml-energyfix.rlib'
out=g/'gbxml-wasi-final'; out.mkdir(exist_ok=True)
args=[rust,'--crate-name','gbxml_fixture','--crate-type','bin','--edition=2024','--target','wasm32-wasip2',str(T/'gbxml-fixture-wasi.rs'),'-o',str(out/'gbxml.wasm'),'-Clink-arg=--threads=1','-Clink-arg=--strip-all','-Clink-arg=--max-memory=536870912','-Clink-arg=-zstack-size=8388608']
for d in [archive.parent,archive.parent/'coherent',*dirs]: args+=['-L','dependency='+str(d)]
for alias,p in [('bim_gbxml',archive),('bim_snapshot',snapshot)]:
 for ext in ('.rlib','.rmeta'): args+=['--extern',alias+'='+str(p.with_suffix(ext))]
subprocess.run(args,cwd=r['cwd'],check=True)
