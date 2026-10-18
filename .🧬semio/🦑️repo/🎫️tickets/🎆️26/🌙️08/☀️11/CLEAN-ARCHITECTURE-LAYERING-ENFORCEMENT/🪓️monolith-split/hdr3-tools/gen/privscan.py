import re, sys, os
# privscan.py <test-file> <lib-closure-list-file> : non-pub definitions of the lib closure referenced by the test file
t = open(sys.argv[1]).read()
files = [l.split("\t")[-1].strip() for l in open(sys.argv[2]) if l.count("\t") == 4]
pat = re.compile(r"(?m)^(\s*)(pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?(fn|struct|enum|static|trait|type|union|const)\s+(\w+)")
rows = {}
for f in files:
    s = open("/Users/ueli/Documents/semio/" + f).read()
    for m in pat.finditer(s):
        vis = (m.group(2) or "").strip()
        if vis == "pub": continue
        name = m.group(4)
        if len(name) < 4: continue
        if re.search(r"\b%s\b" % re.escape(name), t):
            rows.setdefault(name, set()).add((vis or "priv", m.group(3), os.path.basename(os.path.dirname(f))))
for n in sorted(rows):
    print(n, sorted(rows[n]))
