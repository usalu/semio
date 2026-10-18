"""🧪 c6fw reader assertion emulator: for each reader file of a split root, evaluates the direct `includes` / `toContain` assertions on a variable (or inline read) bound to the root against (a) the original text, (b) the new root only, (c) the new module tree. usage: readers4.py [tag..]"""
import os, re, subprocess, sys, json
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from treelib import tree_text, R
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
tags = sys.argv[1:] or list(ft)
targets = {os.path.normpath(os.path.join(R, ft[t])): t for t in tags}
S = r'"((?:[^"\\\n]|\\.)*)"'
def unq(s):
    return bytes(s, "utf-8").decode("unicode_escape").encode("latin-1").decode("utf-8") if "\\" in s and False else s.replace('\\"', '"').replace("\\\\", "\\").replace("\\n", "\n")
out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-co", "--exclude-standard", "--", "*.ts", "*.tsx", "*.mjs", ":!.🧬semio"], cwd=R, capture_output=True, text=True).stdout.split("\n")
ORIG = {t: open(f"{HERE}/orig/{t}.rs", encoding="utf-8").read() for t in tags}
NEWROOT = {t: open(os.path.join(R, ft[t]), encoding="utf-8").read() for t in tags}
TREE = {t: tree_text(os.path.join(R, ft[t])) for t in tags}
READ = re.compile(r'(?:(\w+)\s*=\s*)?(?:readFileSync\(\s*new URL\(' + S + r'\s*,\s*import\.meta\.url\)\s*,\s*"utf8"\s*\)|await Bun\.file\(new URL\(' + S + r'\s*,\s*import\.meta\.url\)\)\.text\(\))')
total = 0
for rel in out:
    if not rel or "node_modules" in rel:
        continue
    p = os.path.join(R, rel)
    if not os.path.isfile(p):
        continue
    t = open(p, encoding="utf-8", errors="ignore").read()
    if "🦀️.rs" not in t:
        continue
    d = os.path.dirname(p)
    binds = {}
    for m in READ.finditer(t):
        var, rel1, rel2 = m.group(1), m.group(2), m.group(3)
        r = rel1 or rel2
        tgt = os.path.normpath(os.path.join(d, r))
        if tgt in targets:
            if var:
                binds.setdefault(var, []).append(targets[tgt])
    rows = []
    for var, tg in binds.items():
        tag = tg[0]
        for m in re.finditer(r'expect\(\s*' + re.escape(var) + r'\.includes\(' + S + r'\)\s*\)\.toBe\((true|false)\)', t):
            rows.append((var, tag, unq(m.group(1)), m.group(2) == "true", "includes"))
        for m in re.finditer(r'expect\(\s*' + re.escape(var) + r'\s*\)\.(not\.)?toContain\(' + S + r'\)', t):
            rows.append((var, tag, unq(m.group(2)), m.group(1) is None, "toContain"))
    if not rows:
        continue
    bad_root = [r for r in rows if (r[2] in NEWROOT[r[1]]) != r[3]]
    bad_tree = [r for r in rows if (r[2] in TREE[r[1]]) != r[3]]
    bad_orig = [r for r in rows if (r[2] in ORIG[r[1]]) != r[3]]
    print(f"{rel[-96:]}: assertions {len(rows)}; fail-on-orig {len(bad_orig)}; fail-on-new-root {len(bad_root)}; fail-on-tree {len(bad_tree)}")
    for r in bad_tree:
        if r in bad_orig:
            continue
        print("      TREE-FAIL", r[0], r[1], r[4], r[3], repr(r[2][:80]))
    for r in bad_root:
        if r not in bad_tree and r not in bad_orig:
            print("      root-fail(tree ok)", r[0], r[1], r[3], repr(r[2][:60]))
