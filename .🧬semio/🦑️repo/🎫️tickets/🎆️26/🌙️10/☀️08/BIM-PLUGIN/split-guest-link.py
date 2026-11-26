"""🧩 Release front-end and code-generation memory before composing the BIM guest."""
from pathlib import Path
import json, os, subprocess, sys
ticket=Path(__file__).resolve().parent
g=ticket/'🗑️generated'
r=json.loads(next((g/'compiler-retry').glob('w-component-*.invocation.json')).read_text())
out=g/'split-guest'
out.mkdir(exist_ok=True)
args=list(r['arguments'])
args[args.index('--out-dir')+1]=str(out)
args=[a.replace('--strip-debug','--strip-all') for a in args]
args+=['-Copt-level=s','-Ccodegen-units=32','-Zno-link']
env=dict(os.environ,**r['cargo'])
rust=r['compiler']
if '--link-only' not in sys.argv:
    with (out/'codegen.log').open('wb') as log:
        subprocess.run([rust,*args],env=env,cwd=r['cwd'],stdout=log,stderr=log,check=True)
rlink=next(out.glob('*.rlink'))
with (out/'link.log').open('wb') as log:
    subprocess.run([rust,'--crate-name',r['crate'] if 'crate' in r else 'semio_hub_bim_component','--crate-type','cdylib','--target',r['target'] if 'target' in r else 'wasm32-wasip2','-Zlink-only',str(rlink),'-Clink-arg=--threads=1','-Clink-arg=--strip-all','-Clink-arg=--max-memory=536870912','-Clink-arg=-zstack-size=8388608','-Clink-arg=-O0','-Clink-arg=--no-merge-data-segments'],env=env,cwd=r['cwd'],stdout=log,stderr=log,check=True)
assert (out/'semio_hub_bim_component.wasm').is_file()
