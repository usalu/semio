from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
art2d = next(p for p in (root/'🗿️artifacts').iterdir() if p.name.endswith('2d'))
# find Puzzle2dSnapshot in schema or component
for f in art2d.rglob('*.rs'):
    t=f.read_text(errors='ignore')
    if 'struct Puzzle2dSnapshot' in t:
        i=t.find('struct Puzzle2dSnapshot')
        print('FILE', f)
        print(t[i-300:i+1500])
# find all places that push nodes onto fixtures without anchor
app2d = next(p for p in next(root.glob('*apps*')).iterdir() if p.name.endswith('2d'))
comp=(app2d/'🦀️component.rs').read_text()
# node json! patterns
import re
for m in re.finditer(r'"id": id', comp):
    print('\n--- around id ---')
    print(comp[m.start()-200:m.start()+500])
