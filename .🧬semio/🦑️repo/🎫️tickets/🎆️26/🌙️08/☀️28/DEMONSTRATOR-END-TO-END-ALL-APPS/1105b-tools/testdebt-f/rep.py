import sys
def apply(path, edits):
    s = open(path, encoding='utf-8').read()
    for old, new, n in edits:
        c = s.count(old)
        if c != n:
            sys.exit(f"FAIL count {c} != {n} for: {old[:100]!r}")
        s = s.replace(old, new)
    open(path, 'w', encoding='utf-8').write(s)
