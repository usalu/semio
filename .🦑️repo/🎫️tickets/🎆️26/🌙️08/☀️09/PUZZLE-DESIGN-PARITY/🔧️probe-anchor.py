from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
arts = root/'🗿️artifacts'
for d in sorted(arts.iterdir()):
    if not d.is_dir(): continue
    c = d/'🦀️component.rs'
    if not c.exists(): continue
    t = c.read_text()
    print('==', d.name)
    for needle in ['enum Puzzle2dNodeAnchor', 'enum Puzzle3dObjectAnchor', 'enum Puzzle5dPartAnchor', 'anchor:', 'Fixed', '#[serde', 'default_anchor', 'fn default']:
        pass
    # extract anchor enum and node struct
    for key in ['NodeAnchor', 'ObjectAnchor', 'PartAnchor', 'pub struct Puzzle2dNode', 'pub struct Puzzle3dObject', 'pub anchor']:
        i = t.find(key)
        if i >= 0 and ('Anchor' in key or key.startswith('pub struct') or key=='pub anchor'):
            print(t[i:i+500])
            print('---')
# how setNode mutation is built / diff
for f in (root/'🗿️artifacts').rglob('*diff*'):
    print('diff file', f)
for f in (root/'🗿️artifacts').rglob('*.rs'):
    tt=f.read_text(errors='ignore')
    if 'set_node' in tt or 'SetNode' in tt or 'setNode' in tt:
        if 'mutation' in str(f).lower() or 'diff' in str(f).lower() or 'op' in f.name.lower() or 'spr' in str(f).lower():
            print('hit', f)
