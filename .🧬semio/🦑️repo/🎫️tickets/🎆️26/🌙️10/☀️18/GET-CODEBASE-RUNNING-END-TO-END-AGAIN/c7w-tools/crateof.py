"""📦 c7w crate lookup: crateof.py <repo-relative file>... prints the nearest owning crate (name, dir, lib root, features) found by walking up to a dir with 📦️packages/🦀️rust/Cargo.toml, and whether the lib root's #[path] closure reaches the file."""
import os, re, sys, json
REPO = '/Users/ueli/Documents/semio'
PK = '📦️packages/🦀️rust/Cargo.toml'
PATHRX = re.compile(r'#\[path\s*=\s*"([^"]+)"\]|\binclude!\(\s*"([^"]+)"\s*\)|include_str!\(\s*"([^"]+)"\s*\)')
def closure(root):
    seen, todo = set(), [root]
    while todo:
        c = todo.pop()
        if c in seen or not os.path.isfile(c) or not c.endswith('.rs'):
            continue
        seen.add(c)
        for m in PATHRX.finditer(open(c, encoding='utf-8', errors='ignore').read()):
            rel = m.group(1) or m.group(2)
            if rel:
                todo.append(os.path.normpath(os.path.join(os.path.dirname(c), rel)))
    return seen
def crate_of(rel):
    d = os.path.dirname(rel)
    while d:
        tp = os.path.join(REPO, d, PK)
        if os.path.isfile(tp):
            t = open(tp, encoding='utf-8').read()
            name = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M).group(1)
            lib = re.search(r'^\[lib\][^\[]*?path\s*=\s*"([^"]+)"', t, re.M | re.S)
            libroot = os.path.normpath(os.path.join(REPO, d, os.path.dirname(PK), lib.group(1) if lib else 'src/lib.rs'))
            feats = re.findall(r'^([A-Za-z0-9_-]+)\s*=\s*\[', t.split('[features]')[1].split('\n[')[0], re.M) if '[features]' in t else []
            reach = os.path.join(REPO, rel) in closure(libroot)
            return dict(name=name, dir=d, lib=os.path.relpath(libroot, REPO), features=feats, reaches=reach)
        d = os.path.dirname(d)
    return None
if __name__ == '__main__':
    for rel in sys.argv[1:]:
        print(rel[-60:], json.dumps(crate_of(rel), ensure_ascii=False))
