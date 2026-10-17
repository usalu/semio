"""🕸️ rii: files reachable from a crate lib root through #[path] and include! (read only), then the head check of each. usage: reach.py <manifest-dir>..."""
import os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]|include!\(\s*"([^"]+)"\s*\)')
def lib_root(mdir):
    t = open(mdir + "/Cargo.toml", encoding="utf8").read()
    m = re.search(r'(?ms)^\[lib\].*?^path\s*=\s*"([^"]+)"', t)
    return os.path.normpath(os.path.join(mdir, m.group(1)))
def reach(root):
    seen, todo = [], [root]
    while todo:
        f = todo.pop()
        if f in seen or not os.path.isfile(f):
            continue
        seen.append(f)
        t = open(f, encoding="utf8").read()
        for m in PATH.finditer(t):
            rel = m.group(1) or m.group(2)
            if rel == ".":
                continue
            todo.append(os.path.normpath(os.path.join(os.path.dirname(f), rel)))
    return seen
tot = 0
for mdir in sys.argv[1:]:
    files = reach(lib_root(mdir))
    out = subprocess.run([sys.executable, HERE + "/heads.py", mdir] + files, capture_output=True, text=True).stdout
    print("==", os.path.basename(os.path.dirname(os.path.dirname(mdir))), len(files), "files")
    print(out.strip())
