import os, re, sys
py, ts = sys.argv[1], sys.argv[2]
self_pat = re.compile(r'^export function parse(\w+)\(value: unknown, at = "\$"\): \1 \{\n  return parse\1\(value', re.M)
fn_pat = re.compile(r'^export function parse(\w+)\(.*?^\}\n', re.M | re.S)
def blocks(text):
    return {m.group(1): m.group(0) for m in fn_pat.finditer(text)}, fn_pat.sub("", text)
differing = fixed = 0
other = []
for dp, dn, fn in os.walk(py):
    for f in fn:
        a = os.path.join(dp, f)
        b = os.path.join(ts, os.path.relpath(a, py))
        sa, sb = open(a, encoding="utf-8").read(), open(b, encoding="utf-8").read()
        if sa == sb:
            continue
        differing += 1
        ba, ra = blocks(sa); bb, rb = blocks(sb)
        selfrec = {m.group(1) for m in self_pat.finditer(sa)}
        ok = set(ba) == set(bb) and all(ba[n] == bb[n] for n in ba if n not in selfrec) and all(not self_pat.search(bb[n]) for n in selfrec if n in bb)
        if ok and ra.replace("\n\n", "\n") == rb.replace("\n\n", "\n"):
            fixed += len(selfrec)
        else:
            other.append((os.path.relpath(a, py)[:120], sorted(selfrec)[:3], sorted(set(ba) ^ set(bb))[:3]))
print(f"differing files {differing}; self-recursive parsers replaced {fixed}; files whose difference is NOT explained by the self-call fix: {len(other)}")
for row in other[:10]: print(row)
