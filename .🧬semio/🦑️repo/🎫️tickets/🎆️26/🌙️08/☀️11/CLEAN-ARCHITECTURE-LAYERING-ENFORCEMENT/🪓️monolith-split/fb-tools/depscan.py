"""🔎 Crate-head scan for the FB carve: walks a crate's mounted source closure (mod/#[path]/include!) and lists the external crate heads it names."""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/pos")
from posstub import mask

ROOT = "/Users/ueli/Documents/semio"
PATH_RX = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
MOD_RX = re.compile(r"^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;", re.M)
STD = {"std", "core", "alloc", "crate", "self", "super", "Self", "proc_macro", "test"}


def closure(root_file):
    seen, stack, files = set(), [root_file], []
    while stack:
        f = stack.pop()
        if f in seen or not os.path.exists(f):
            continue
        seen.add(f)
        files.append(f)
        t = open(f, encoding="utf-8").read()
        m = mask(t)
        d = os.path.dirname(f)
        for line_match in re.finditer(r'((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', t):
            attrs, name = line_match.group(1), line_match.group(2)
            pm = PATH_RX.search(attrs)
            if pm:
                stack.append(os.path.normpath(os.path.join(d, pm.group(1))))
            else:
                for cand in (os.path.join(d, name + ".rs"), os.path.join(d, name, "mod.rs")):
                    if os.path.exists(cand):
                        stack.append(cand)
    return files


def heads(files):
    out = {}
    local = set()
    for f in files:
        t = open(f, encoding="utf-8").read()
        m = mask(t)
        for mm in re.finditer(r"\bmod\s+(\w+)", m):
            local.add(mm.group(1))
        for mm in re.finditer(r"\bextern\s+crate\s+(\w+)\s+as\s+(\w+)", m):
            local.add(mm.group(2))
            out.setdefault(mm.group(1), set()).add("extern-crate")
        for mm in re.finditer(r"\buse\s+(?:\w+::)*\w+\s+as\s+(\w+)\s*;", m):
            local.add(mm.group(1))
        for mm in re.finditer(r"(?<![\w.])(?<!::)([a-z][a-z0-9_]*)::", m):
            out.setdefault(mm.group(1), set()).add(os.path.basename(os.path.dirname(f)))
        for mm in re.finditer(r"#\[\s*([a-z][a-z0-9_]*)::", m):
            out.setdefault(mm.group(1), set()).add(os.path.basename(os.path.dirname(f)))
    return out, local


if __name__ == "__main__":
    lib = sys.argv[1]
    files = closure(lib)
    h, local = heads(files)
    print(len(files), "files")
    for k in sorted(h):
        if k in STD or k in local:
            continue
        print(k)
