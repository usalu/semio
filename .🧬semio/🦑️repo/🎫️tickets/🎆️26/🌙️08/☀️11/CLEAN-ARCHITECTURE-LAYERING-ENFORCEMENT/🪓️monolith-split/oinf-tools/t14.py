from settle import *
tree, mods = load_mods()
nodes = build(mods)
root = [n for n in nodes if n.mod == '🌍️world/🦀️.rs']
byparent = collections.defaultdict(list)
for n in root:
    key = n.parent if n.parent else '-'
    byparent[key].append(n)
for k, v in sorted(byparent.items(), key=lambda kv: -sum(n.lines for n in kv[1])):
    if k == '-':
        print('FREE:', ', '.join(f"{n.name}({n.kind[:2]}{n.lines})" for n in v))
    else:
        print(f"{k}: {sum(n.lines for n in v)} lines, {len(v)} members")
