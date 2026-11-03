"""🔎 Scan crates for `crate::<name>` references to modules not mounted/re-exported in the same crate."""
import os, re, sys, tomllib

ROOT = sys.argv[1] if len(sys.argv) > 1 and __name__ == "__main__" else "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
READ = lambda path: open(path, encoding="utf-8").read()
EXISTS = os.path.exists
MOD = re.compile(r'(?:#\[path\s*=\s*"([^"]+)"\]\s*)?(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*(;|\{)')
CRATE = re.compile(r'\bcrate::(?:\{([^}]*)\}|(\w+))')
DEF = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:fn|struct|enum|trait|type|const|static|union|macro_rules!)\s+(\w+)', re.M)
USE = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);', re.M)
EXT = re.compile(r'extern\s+crate\s+\S+\s+as\s+(\w+)')

def strip(s):
    s = re.sub(r'//[^\n]*', '', s)
    s = re.sub(r'/\*.*?\*/', '', s, flags=re.S)
    return s

def idents_of_use(body):
    out = set()
    body = re.sub(r'\s+', ' ', body)
    for m in re.finditer(r'(\w+)\s+as\s+(\w+)', body):
        out.add(m.group(2))
    for m in re.finditer(r'[\w:]*?(\w+)\s*(?=[,}]|$)', body):
        out.add(m.group(1))
    return out

def resolve(base_file, path, name, inline_dir=None):
    d = os.path.dirname(base_file)
    if path:
        return os.path.normpath(os.path.join(d, path))
    stem = os.path.splitext(os.path.basename(base_file))[0]
    cands = [os.path.join(d, name + ".rs"), os.path.join(d, name, "mod.rs")]
    if stem not in ("lib", "mod", "🦀️"):
        cands = [os.path.join(d, stem, name + ".rs")] + cands
    return next((c for c in cands if EXISTS(c)), None)

def walk(f, seen, files):
    if f in seen or not f or not EXISTS(f):
        return
    seen.add(f)
    src = strip(READ(f))
    files[f] = src
    for m in MOD.finditer(src):
        if m.group(3) == ";":
            walk(resolve(f, m.group(1), m.group(2)), seen, files)

PKG = {}
GLOB = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?use\s+(\w+)(?:::\w+)*::\*\s*;', re.M)

def index():
    for dp, dn, fn in os.walk(ROOT):
        dn[:] = [d for d in dn if d not in ("target", "node_modules", ".git")]
        if "Cargo.toml" in fn:
            try:
                t = tomllib.load(open(os.path.join(dp, "Cargo.toml"), "rb"))
                lp = t.get("lib", {}).get("path")
                if lp:
                    PKG[t["package"]["name"].replace("-", "_")] = os.path.normpath(os.path.join(dp, lp))
            except Exception:
                pass

def known_of(root, depth=0, only=None):
    files = {}
    walk(root, set(), files)
    rs = files[root]
    known = set(EXT.findall(rs)) | set(DEF.findall(rs))
    mods = {}
    for m in MOD.finditer(rs):
        known.add(m.group(2))
        if m.group(3) == ";":
            mods[m.group(2)] = resolve(root, m.group(1), m.group(2))
    for m in USE.finditer(rs):
        known |= idents_of_use(m.group(1))
    opaque = False
    for m in GLOB.finditer(rs):
        first = m.group(1)
        tgt = PKG.get(first)
        if first in mods and mods[first] and depth < 6:
            k, o, _ = known_of(mods[first], depth + 1)
            known |= k
            opaque |= o
        elif tgt and depth < 6 and EXISTS(tgt):
            k, o, _ = known_of(tgt, depth + 1)
            known |= k
            opaque |= o
        elif first not in ("super", "self", "crate"):
            opaque = True
    return known, opaque, files

def scan_crate(toml):
    t = tomllib.loads(READ(toml))
    lp = t.get("lib", {}).get("path")
    if not lp:
        return None
    root = os.path.normpath(os.path.join(os.path.dirname(toml), lp))
    if not EXISTS(root):
        return None
    known, opaque, files = known_of(root)
    missing = {}
    for f, src in files.items():
        for m in CRATE.finditer(src):
            names = [m.group(2)] if m.group(2) else [re.split(r'::|\s', x.strip())[0] for x in m.group(1).split(",") if x.strip()]
            for n in names:
                if n and n not in known and n != "self":
                    missing.setdefault(n, set()).add(os.path.relpath(f, ROOT))
    return t["package"]["name"], root, missing, opaque

def main():
    index()
    bad = 0
    for dp, dn, fn in os.walk(ROOT):
        dn[:] = [d for d in dn if d not in ("target", "node_modules", ".git")]
        if "Cargo.toml" in fn:
            try:
                r = scan_crate(os.path.join(dp, "Cargo.toml"))
            except Exception as e:
                print(f"ERR {dp}: {e}")
                continue
            if r and r[2]:
                bad += 1
                print(f"CRATE {r[0]}{' (opaque-glob)' if r[3] else ''}\n  root {os.path.relpath(r[1], ROOT)}")
                for n, fs in sorted(r[2].items()):
                    print(f"  missing crate::{n}  <- {sorted(fs)[0]}" + (f" (+{len(fs)-1})" if len(fs) > 1 else ""))
    print(f"findings: {bad} crates")

if __name__ == "__main__":
    main()
