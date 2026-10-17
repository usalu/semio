"""🔍 Resolve include_str!/include_bytes! sites of Rust files (with env!(CARGO_MANIFEST_DIR) concat) and report missing targets. Read-only. usage: incl.py <manifest_dir> <rs files...>"""
import os, re, sys
mdir = os.path.abspath(sys.argv[1])
pat = re.compile(r'include_(?:str|bytes)!\(\s*(?:concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*"([^"]*)"\s*\)|"([^"]*)")\s*\)')
for f in sys.argv[2:]:
    f = os.path.abspath(f)
    s = open(f, encoding="utf8").read()
    for m in pat.finditer(s):
        if m.group(1) is not None:
            tgt = os.path.normpath(mdir + m.group(1))
        else:
            tgt = os.path.normpath(os.path.join(os.path.dirname(f), m.group(2)))
        line = s.count("\n", 0, m.start()) + 1
        print(("OK     " if os.path.exists(tgt) else "MISSING"), os.path.relpath(f), line, os.path.relpath(tgt))
