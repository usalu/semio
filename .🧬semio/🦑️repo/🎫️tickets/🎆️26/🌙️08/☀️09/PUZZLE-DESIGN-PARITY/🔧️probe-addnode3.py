from pathlib import Path
p = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
puzzle = next(x for x in p.iterdir() if x.name.endswith("puzzle"))
hits=[]
for f in puzzle.rglob('*.rs'):
    if 'target' in f.parts: continue
    try:
        t=f.read_text(errors='ignore')
    except Exception as e:
        continue
    if 'fn add_node' in t:
        hits.append((str(f), t.count('fn add_node'), 'add_node_to_fixture' in t, 'anchor' in t))
print('hits', len(hits))
for h in hits:
    print(h)

# print from 2d component around add_node_to_fixture and node module
apps = next(x for x in puzzle.iterdir() if "apps" in x.name)
twod = next(x for x in apps.iterdir() if "2d" in x.name)
for f in sorted(twod.rglob('*')):
    print('file', f.relative_to(twod))
