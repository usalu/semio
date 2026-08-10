from pathlib import Path
p = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
puzzle = next(x for x in p.iterdir() if x.name.endswith("puzzle"))
# find add_node in node module and add_node_to_fixture full
for f in puzzle.rglob('*.rs'):
    if 'target' in f.parts: continue
    t=f.read_text(errors='ignore')
    if 'pub fn add_node(' in t or 'fn add_node(' in t and 'ctx' in t:
        if 'add_node' in t and ('CommandCtx' in t or 'ctx:' in t):
            # print functions
            for key in ['pub fn add_node(', 'fn add_node(', 'pub fn add_node_to_fixture', 'fn new_node_id']:
                i=t.find(key)
                if i>=0:
                    print('\n====', f.name, key)
                    print(t[i:i+1200])
