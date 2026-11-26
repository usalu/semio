"""🧾 Hash the physical artifacts referenced by a retained linker invocation."""
import hashlib, json, os
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
g=Path(__file__).resolve().parent/'🗑️generated/bridge-current/b4'
record=json.loads((g/'link.json').read_text())
units=[Path(x) for x in record['args'] if x.endswith('.rlib')]
files=set(units)
for library in units:
    if library.with_suffix('.rmeta').is_file(): files.add(library.with_suffix('.rmeta'))
files.update(g/r['path'] for r in record['objects'])
files.update([g/'proc.core.wasm',g/'proc.fold1.wasm',g/'proc.fold2.wasm',g/'semio_hub_procedural_component.wasm'])
def read(p):
    with p.open('rb') as f:
        stamp=os.fstat(f.fileno())
        digest=hashlib.sha256()
        for block in iter(lambda:f.read(4<<20),b''): digest.update(block)
        if (stamp.st_size,stamp.st_mtime_ns)!=(p.stat().st_size,p.stat().st_mtime_ns):raise ValueError(str(p))
    return str(p),digest.hexdigest()
with (g/'linked-product-checksums.ndjson').open('w') as f, ThreadPoolExecutor(max_workers=4) as pool:
    for result in pool.map(read,sorted(files,key=lambda p:(p.is_symlink(),-p.stat().st_size))):
        print(json.dumps(result,ensure_ascii=False),file=f,flush=True)
print('[DEBUG]',len(files),'verified physical linker artifacts',flush=True)
