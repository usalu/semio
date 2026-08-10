from pathlib import Path
import re
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
for dim in ['*2d*', '*3d*']:
    apps=next((root/'🎛️apps').glob(dim.replace('*','') if False else dim[1:] if False else dim))
    
for dim_glob, label in [('*2d*','2d'), ('*3d*','3d')]:
    appdir=next(p for p in (next(root.glob('*apps*'))).iterdir() if p.name.endswith(label))
    t=(appdir/'🦀️component.rs').read_text()
    print('\n########', label)
    for key in ['hydrate', 'to_fixture', 'fn apply_action', 'fn run_command', 'PlaySnapshot', 'before_command', 'after_inbound', 'fn mutate', 'document_mut', 'with_fixture']:
        for m in re.finditer(key, t):
            ctx=t[max(0,m.start()-80):m.start()+200]
            if any(x in ctx for x in ['fn ', 'hydrate', 'to_fixture', 'snapshot']):
                print('---', key, 'at', m.start())
                print(t[max(0,m.start()-100):m.start()+500])
                break

# plugin component dispatch_typed / handle
plugin=Path('/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs')
tt=plugin.read_text()
for key in ['fn dispatch_typed', 'fn attach_backbone', 'pump', 'inbound', 'hydrate', 'before_mutate']:
    i=tt.find(key)
    print('\nPLUGIN', key, i)
    if i>=0:
        print(tt[i:i+1500])
