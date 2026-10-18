"""📦 C6STORE owning crate of each tag: nearest ancestor with `📦️packages/🦀️rust/Cargo.toml`, its lib path, whether the file is the root and the path of the mount chain. usage: crates.py [tag...]"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
PATH = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
def owner(rel):
    d = os.path.dirname(rel)
    best = None
    while d:
        for sub in ("📦️packages/🦀️rust",):
            m = os.path.join(REPO, d, sub, "Cargo.toml")
            if os.path.isfile(m):
                return d, os.path.join(d, sub)
        d = os.path.dirname(d)
    return None, None
def libroot(mdir):
    t = open(os.path.join(REPO, mdir, "Cargo.toml"), encoding="utf-8").read()
    name = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M).group(1)
    m = re.search(r'^\[lib\][^\[]*?path\s*=\s*"([^"]+)"', t, re.M | re.S)
    p = os.path.normpath(os.path.join(mdir, m.group(1))) if m else os.path.join(mdir, "src/lib.rs")
    tests = re.findall(r'^\[\[test\]\][^\[]*?path\s*=\s*"([^"]+)"', t, re.M | re.S)
    return name, p, tests
def chain(root, target):
    out = []
    def walk(f, trail):
        p = os.path.join(REPO, f)
        if not os.path.isfile(p):
            return None
        if f == target:
            return trail + [f]
        txt = open(p, encoding="utf-8").read()
        for m in PATH.finditer(txt):
            nxt = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            r = walk(nxt, trail + [f])
            if r:
                return r
        return None
    return walk(root, [])
if __name__ == "__main__":
    for tag in (sys.argv[1:] or ft):
        rel = ft[tag]
        d, mdir = owner(rel)
        name, root, tests = libroot(mdir)
        ch = chain(root, rel)
        print(tag, name, "root" if root == rel else "mounted", "| root:", root[-60:], "| depth", len(ch) if ch else None, "| tests", tests[:2])
