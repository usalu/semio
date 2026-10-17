#!/usr/bin/env python3
"""Usage: privscan.py <test.rs> <srcdir-or-file>...  -- lists identifiers used by the test that are defined in the sources with their visibility (module-level items, impl methods, struct fields)."""
import re, sys, glob, os
test = open(sys.argv[1]).read()
def strip(s):
    s = re.sub(r'//[^\n]*', '', s)
    s = re.sub(r'"(?:\\.|[^"\\])*"', '""', s)
    return s
used = set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', strip(test)))
files = []
for p in sys.argv[2:]:
    if os.path.isdir(p):
        files += [f for f in glob.glob(p + '/**/*.rs', recursive=True) if '🧪️tests' not in f]
    else:
        files.append(p)
item = re.compile(r'^(?P<ind>[ \t]*)(?P<vis>pub(?:\([^)]*\))?\s+)?(?:unsafe\s+|const\s+|async\s+)*(?P<kind>fn|struct|enum|const|static|type|trait|union)\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)', re.M)
for f in sorted(files):
    s = open(f).read()
    rel = os.path.relpath(f, os.path.dirname(sys.argv[2]) if os.path.isfile(sys.argv[2]) else sys.argv[2])
    for m in item.finditer(s):
        n = m.group('name')
        if n in used:
            line = s.count('\n', 0, m.start()) + 1
            print(f"{n}\t{m.group('kind')}\t{(m.group('vis') or 'private').strip()}\tind={len(m.group('ind'))}\t{rel}:{line}")
