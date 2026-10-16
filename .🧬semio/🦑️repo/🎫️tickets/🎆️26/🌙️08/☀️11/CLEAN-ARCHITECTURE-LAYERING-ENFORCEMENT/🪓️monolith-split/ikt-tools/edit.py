import sys, hashlib
def edit(path, pairs):
    t = open(path, encoding='utf8').read()
    for old, new in pairs:
        n = t.count(old)
        assert n == 1, f"anchor count {n} in {path}: {old[:90]!r}"
        t = t.replace(old, new)
    open(path, 'w', encoding='utf8').write(t)
