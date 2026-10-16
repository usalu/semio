"""Own-closure census of semio-framework-os-pack (lib and test closure: files, bytes, active lines), baseline snapshot versus live tree. Mount walk over #[path], implicit `mod x;` and include!/include_str! of rust files."""
import os, re, sys
ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
LIB = "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🦀️.rs"
MOUNT = re.compile(r"((?:#\[[^\]\n]*\]\s*)*)(?:pub(?:\([^)\n]*\))?\s+)?mod\s+([A-Za-z_]\w*)\s*;")
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INC = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')
def strip(t):
    t = re.sub(r"/\*[\s\S]*?\*/", lambda m: " " * len(m.group(0)), t)
    return re.sub(r"(^|[^:\"])//[^\n]*", lambda m: m.group(1), t)
def closure(base, tests):
    seen, order, q = set(), [], [os.path.join(base, LIB)]
    while q:
        f = q.pop()
        if f in seen or not os.path.exists(f): continue
        seen.add(f); order.append(f)
        t = strip(open(f, encoding="utf8").read()); here = os.path.dirname(f)
        for m in MOUNT.finditer(t):
            attrs = m.group(1)
            if not tests and re.search(r"cfg\(\s*test\s*\)", attrs): continue
            pm = PATH.search(attrs)
            if pm: q.append(os.path.normpath(os.path.join(here, pm.group(1))))
            else:
                stem = os.path.basename(f)[:-3]; b = here if stem == "mod" else os.path.join(here, stem)
                for c in (os.path.join(b, m.group(2) + ".rs"), os.path.join(b, m.group(2), "mod.rs"), os.path.join(here, m.group(2) + ".rs")):
                    if os.path.exists(c): q.append(c); break
        for m in INC.finditer(t): q.append(os.path.normpath(os.path.join(here, m.group(1))))
    return order
def stats(files):
    b = a = 0
    for p in files:
        t = open(p, encoding="utf8").read(); b += len(t.encode()); a += sum(1 for l in t.split("\n") if l.strip() and not l.strip().startswith("//"))
    return len(files), b, a
for tests in (False, True):
    sb = stats(closure(F + "/🗑️generated/orc-work/snap", tests)); sn = stats(closure(ROOT, tests))
    print(("test" if tests else "lib "), "closure files/bytes/active lines:", sb, "->", sn)
