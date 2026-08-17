from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
art2d = next(p for p in (root/'🗿️artifacts').iterdir() if p.name.endswith('2d'))
# Puzzle2dEdge struct
for f in art2d.rglob('*.rs'):
    t=f.read_text(errors='ignore')
    if 'struct Puzzle2dEdge' in t:
        i=t.find('struct Puzzle2dEdge')
        print(t[i:i+2000])
        break
# duplicate / create edge in app
app2d = next(p for p in next(root.glob('*apps*')).iterdir() if p.name.endswith('2d'))
for f in app2d.rglob('*.rs'):
    t=f.read_text(errors='ignore')
    if 'json!({\n        "id"' in t or '"handles": []' in t or 'add_edge' in t or '"edges"' in t and 'push' in t:
        if 'anchor' not in t and ('node' in f.name or 'selection' in str(f) or 'component.rs' in f.name):
            print('maybe', f)
