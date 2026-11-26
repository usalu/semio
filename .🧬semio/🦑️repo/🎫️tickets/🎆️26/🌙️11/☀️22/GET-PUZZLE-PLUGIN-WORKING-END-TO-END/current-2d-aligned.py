from pathlib import Path
import os
ticket=Path(__file__).resolve().parent
source=(ticket/'current-2d-general.py').read_text()
start=source.index('script = script.replace("dependencies')
end=source.index("if os.environ.get('PUZZLE_REFRESH",start)
addition='''
import re
seed=next(cache.glob('semio-s-artifact-puzzle-2d-unit-tests/19d18fac48622475/fingerprint/lib-*.json'))
fresh=json.loads(seed.read_text())['deps']
pending=fresh.copy()
resolved={}
while pending:
    package,n,_,v=pending.pop(0)
    if n in resolved:continue
    file=artifacts.get(v.to_bytes(8,'little').hex())
    if file is None:continue
    resolved[n]=file
    info=file.resolve().parents[1]/'fingerprint'/f'lib-{n}.json'
    if info.exists():pending.extend(json.loads(info.read_text())['deps'])
for package,n,_,v in dependencies:
    key=v.to_bytes(8,'little').hex()
    if n in resolved:artifacts[key]=resolved[n]
    elif n.startswith('semio_') or key not in artifacts:
        candidates=[p for ext in ('rlib','so') for p in libs.glob(f'lib{n}-*.{ext}') if p.exists()]
        if candidates:artifacts[key]=max(candidates,key=lambda p:p.stat().st_mtime_ns)
'''
source=source[:start]+'script=script.replace("dependencies = meta[\'deps\']", "dependencies = meta[\'deps\']"+'+repr(addition)+')\n'+source[end:]
exec(compile(source,str(ticket/'current-2d-general.py'),'exec'))
