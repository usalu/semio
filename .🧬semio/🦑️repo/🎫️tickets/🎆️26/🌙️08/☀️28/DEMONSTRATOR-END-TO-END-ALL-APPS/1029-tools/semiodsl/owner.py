import os, re, sys, pathlib

root = pathlib.Path(sys.argv[1])
crates = []
for cargo in root.rglob('Cargo.toml'):
    text = cargo.read_text(encoding='utf-8')
    m = re.search(r'^\[lib\][^\[]*?path\s*=\s*"([^"]+)"', text, re.M | re.S)
    n = re.search(r'^name\s*=\s*"([^"]+)"', text, re.M)
    if not m or not n:
        continue
    lib = (cargo.parent / m.group(1)).resolve()
    crates.append((lib.parent, n.group(1), cargo))
for arg in sys.argv[2:]:
    f = pathlib.Path(arg).resolve()
    best = None
    for d, name, cargo in crates:
        if d in f.parents or d == f.parent:
            if best is None or len(str(d)) > len(str(best[0])):
                best = (d, name, cargo)
    print(arg.split('subsets/')[-1], '->', best[1] if best else None, best[2] if best else '')
