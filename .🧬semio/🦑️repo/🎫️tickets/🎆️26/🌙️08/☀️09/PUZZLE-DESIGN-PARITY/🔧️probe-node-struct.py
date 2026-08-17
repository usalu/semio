from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
c = root/'🗿️artifacts/◻2d/🦀️component.rs'
t = c.read_text()
i = t.find('pub struct Puzzle2dNode')
print(t[i:i+1200])
# also check serde on enum
i = t.find('enum Puzzle2dNodeAnchor')
print('\nENUM\n', t[i-200:i+250])
# mutation apply path
for f in root.rglob('*.rs'):
    if 'target' in f.parts: continue
    tt=f.read_text(errors='ignore')
    if 'Puzzle2dNode' in tt and ('from_value' in tt or 'from_str' in tt) and ('mutation' in tt.lower() or 'diff' in str(f).lower() or 'op' in str(f)):
        print('cand', f)
# search setNode in artifacts 2d
for f in (root/'🗿️artifacts/◻2d').rglob('*.rs'):
    tt=f.read_text(errors='ignore')
    if 'setNode' in tt or 'SetNode' in tt or 'set_node' in tt:
        print('SET', f)
        for key in ['setNode', 'SetNode', 'set_node', 'anchor']:
            print(' ', key, tt.count(key))
