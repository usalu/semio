"""🔒️ Methods and fields with a restricted visibility in production crates that the test files of the bins name (candidates for E0624/E0616/E0451 once a test is compiled outside its defining crate).
Name-keyed and conservative: a name is reported when no public method/field of that name exists anywhere in production code. usage: restrict_scan.py"""
import collections, re
from lib import *
import dupscan as D

BIN_SUFFIX = ("shell-tests-", "frame-tests-", "-tests")
crates = crate_table()
roots = {}
for s, d in crates.items():
    roots[s] = T + "/" + d + "/🦀️.rs"
test_files, prod_files = {}, {}
for s, root in roots.items():
    is_bin = any(s.startswith(b) or s.endswith(b) for b in BIN_SUFFIX) and s != "test-support"
    for f in D.reach(root):
        if not f.endswith(".rs"):
            continue
        if f == root and is_bin:
            continue
        istest = "/🧪️tests/" in f or "test-hosts" in f or is_bin
        (test_files if istest else prod_files).setdefault(f, s)
for f in list(prod_files):
    if f in test_files:
        del prod_files[f]
IMPL = re.compile(r"^(?:unsafe\s+)?impl\b")
FN = re.compile(r"^\s*(pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+|extern\s+\"[^\"]*\"\s+)*fn\s+([A-Za-z_]\w*)")
STRUCT = re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?struct\s+([A-Za-z_]\w*)[^;{]*\{")
FIELD = re.compile(r"^\s*(pub(?:\([^)]*\))?\s+)?([A-Za-z_]\w*)\s*:")
meth_r, meth_p, fld_r, fld_p = collections.defaultdict(list), set(), collections.defaultdict(list), set()
for f, c in prod_files.items():
    text = blank(read(f))
    lines = text.split("\n")
    depth, head = 0, None
    for l in lines:
        st = l.strip()
        if depth == 0:
            head = None
            if IMPL.match(st) and " for " not in st.split("{")[0]:
                head = "impl"
            else:
                m = STRUCT.match(st)
                if m:
                    head = "struct:" + m.group(1)
        elif depth == 1 and head:
            if head == "impl":
                m = FN.match(l)
                if m:
                    (meth_p.add(m.group(2)) if m.group(1) and m.group(1).strip() == "pub" else meth_r[m.group(2)].append((c, os.path.basename(os.path.dirname(f)), (m.group(1) or "").strip() or "private")))
            else:
                m = FIELD.match(l)
                if m:
                    (fld_p.add(m.group(2)) if m.group(1) and m.group(1).strip() == "pub" else fld_r[m.group(2)].append((c, head[7:], (m.group(1) or "").strip() or "private")))
        depth += l.count("{") - l.count("}")
test_meth, test_fld = collections.defaultdict(set), collections.defaultdict(set)
test_defs = set()
for f, c in test_files.items():
    text = blank(read(f))
    for m in re.finditer(r"\bfn\s+([A-Za-z_]\w*)", text):
        test_defs.add(m.group(1))
    for m in re.finditer(r"\.\s*([A-Za-z_]\w*)\s*(?:::<[^()]*>)?\(", text):
        test_meth[m.group(1)].add((c, os.path.basename(os.path.dirname(f))))
    for m in re.finditer(r"\.\s*([A-Za-z_]\w*)\b(?!\s*(?:::<[^()]*>)?\()", text):
        test_fld[m.group(1)].add((c, os.path.basename(os.path.dirname(f))))
    for m in re.finditer(r"\b[A-Z]\w*\s*\{([^{}]*)\}", text):
        for k in re.finditer(r"(?:^|,)\s*([a-z_]\w*)\s*:", m.group(1)):
            test_fld[k.group(1)].add((c, os.path.basename(os.path.dirname(f))))
print("test files", len(test_files), "production files", len(prod_files))
print("== methods")
n = 0
for name in sorted(meth_r):
    if name in meth_p or name in test_defs or name not in test_meth:
        continue
    defs = {c for c, d, v in meth_r[name]}
    users = {(c, t) for c, t in test_meth[name] if c not in defs}
    if not users:
        continue
    n += 1
    print(f"  {name}: defined {sorted({(c, v) for c, d, v in meth_r[name]})[:3]} used by {sorted(users)[:4]}")
print(n)
print("== fields")
n = 0
for name in sorted(fld_r):
    if name in fld_p or name not in test_fld:
        continue
    defs = {c for c, d, v in fld_r[name]}
    users = {(c, t) for c, t in test_fld[name] if c not in defs}
    if not users:
        continue
    n += 1
    print(f"  {name}: defined {sorted({(c, d, v) for c, d, v in fld_r[name]})[:3]} used by {sorted(users)[:4]}")
print(n)
