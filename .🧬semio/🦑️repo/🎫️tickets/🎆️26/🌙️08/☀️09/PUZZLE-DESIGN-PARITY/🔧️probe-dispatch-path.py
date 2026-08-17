from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
# search for fixture sync, pump, rebase in 2d and 3d apps and plugin framework
needles = ['scene.fixture', 'reload_fixture', 'sync_fixture', 'from_snapshot', 'to_fixture', 'pump', 'ingest', 'rebase', 'hydrate']
for label, path in [
    ('2d', next((root/'🎛️apps').glob('*2d*'))/'🦀️component.rs'),
    ('3d', next((root/'🎛️apps').glob('*3d*'))/'🦀️component.rs'),
]:
    t=path.read_text()
    print('====', label, '====')
    for n in needles:
        print(n, t.count(n))

# framework plugin app dispatch
fw = Path('/Users/ueli/Documents/semio/🧰️framework')
hits=[]
for f in fw.rglob('*.rs'):
    if 'target' in f.parts: continue
    try: tt=f.read_text(errors='ignore')
    except: continue
    if 'attach_backbone' in tt and 'fn dispatch' in tt:
        hits.append(f)
    if 'fn attach_backbone' in tt:
        hits.append(f)
print('hits', len(set(map(str,hits))))
for f in sorted(set(map(str,hits)))[:20]:
    print(f)
