"""🔎 RI3 owner index: maps every renderer crate to the files it mounts (via `#[path]`), indexes top-level item definitions per crate and answers `where is NAME defined` and `which files mention MOD::`."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/../ri2-tools")
from common import *
from rs import blank

MOUNT = re.compile(r'((?:#\[[^\]\n]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_]\w*)\s*(;|\{)')
INC = re.compile(r'include!\(\s*"([^"]+)"\s*\)')
DEF = re.compile(r'(?m)^[ \t]*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+|async\s+|const\s+)*(fn|struct|enum|const|static|type|trait|union)\s+([A-Za-z_]\w*)')
MAC = re.compile(r'(?m)^[ \t]*macro_rules!\s+([A-Za-z_]\w*)')


def files_of(root_file, seen=None):
    seen = seen if seen is not None else {}
    if root_file in seen or not os.path.isfile(root_file):
        return seen
    seen[root_file] = True
    text = read(root_file)
    clean = blank(text)
    here = os.path.dirname(root_file)
    for m in MOUNT.finditer(clean):
        attrs = text[m.start(1):m.end(1)]
        pm = re.search(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', attrs)
        if pm and m.group(3) == ";":
            files_of(os.path.normpath(os.path.join(here, pm.group(1))), seen)
        elif m.group(3) == ";" and not pm:
            for cand in (os.path.join(here, m.group(2) + ".rs"), os.path.join(here, m.group(2), "mod.rs")):
                if os.path.isfile(cand):
                    files_of(cand, seen)
                    break
    for m in INC.finditer(text):
        p = os.path.normpath(os.path.join(here, m.group(1)))
        if p.endswith(".rs"):
            files_of(p, seen)
    return seen


def build():
    tbl = crate_table(T)
    idx = {}
    crate_files = {}
    for suffix, folder in tbl.items():
        root = os.path.join(T, folder, "🦀️.rs")
        if not os.path.isfile(root):
            continue
        fs = list(files_of(root))
        crate_files[suffix] = fs
        for f in fs:
            try:
                clean = blank(read(f))
            except Exception:
                continue
            for m in DEF.finditer(clean):
                idx.setdefault(m.group(2), []).append((suffix, f))
            for m in MAC.finditer(clean):
                idx.setdefault(m.group(1), []).append((suffix, f))
    return tbl, crate_files, idx


if __name__ == "__main__":
    tbl, cf, idx = build()
    json.dump(dict(crate_files=cf), open(GEN + "/ri3-crate-files.json", "w"), ensure_ascii=False)
    for name in sys.argv[1:]:
        for suffix, f in idx.get(name, []):
            print(name, suffix, f.replace(REPO + "/", "")[-110:])
