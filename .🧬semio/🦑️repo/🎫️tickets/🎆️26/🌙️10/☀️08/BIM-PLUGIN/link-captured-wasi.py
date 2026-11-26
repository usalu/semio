"""🧱 Link a WASI fixture against one consistent captured Cargo unit and its dependencies."""
from pathlib import Path
from bim_link import large_link
import json, sys, subprocess
receipt,source,output,*bindings=sys.argv[1:]
request=json.loads(Path(receipt).read_text())
original=request['arguments']
out=Path(original[original.index('--out-dir')+1])
binary=Path(output).resolve()
args=[request['compiler'],'--crate-type','bin','--edition=2024','--target','wasm32-wasip2',str(Path(source).resolve()),'-o',str(binary),'-Zno-parallel-backend','-Zthreads=1','-Clink-arg=--threads=1','-Clink-arg=--strip-all','-Clink-arg=--max-memory=536870912','-Clink-arg=-zstack-size=8388608','-L','dependency='+str(out)]
for i,arg in enumerate(original[:-1]):
    if arg=='-L':args+=['-L',original[i+1]]
for b in bindings:
    alias,crate=b.split('=')
    if crate=='self':
        archive=next(out.glob('lib'+request['crate']+'-*.rlib'))
    else:
        archive=Path(next(original[i+1].split('=',1)[1] for i,arg in enumerate(original[:-1]) if arg=='--extern' and original[i+1].startswith(crate+'=') and original[i+1].endswith(('.rlib','.rmeta'))))
    for ext in ['.rlib','.rmeta']:args+=['--extern',alias+'='+str(archive.with_suffix(ext))]
with large_link():subprocess.run(args,cwd=request['cwd'],check=True)
