import re, sys, os
# uses.py <test-file> <mod-file>... : which definitions of each module file the test file references
t = open(sys.argv[1]).read()
for mf in sys.argv[2:]:
    s = open(mf).read()
    defs = set(re.findall(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:fn|struct|enum|const|static|trait|type|union)\s+(\w+)", s))
    hit = sorted(n for n in defs if len(n) > 3 and re.search(r"\b%s\b" % re.escape(n), t))
    print(os.path.basename(os.path.dirname(mf)), len(defs), len(hit), hit[:40])
