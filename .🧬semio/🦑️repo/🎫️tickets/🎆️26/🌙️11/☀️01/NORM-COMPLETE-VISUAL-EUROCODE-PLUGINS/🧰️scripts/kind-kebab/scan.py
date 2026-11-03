"""🧷 Lists mutation leaves whose SEMANTICS.kind differs from the dsl::Mutations derive's to_kebab(variant) (compile-time assert)."""
import re, glob, os, sys

def to_kebab(name):
    out = ""; ch = list(name)
    for i, c in enumerate(ch):
        if c in "_-":
            if out and not out.endswith("-"): out += "-"
            continue
        if c.isupper():
            p = ch[i-1] if i else None; n = ch[i+1] if i + 1 < len(ch) else None
            b = p is not None and (p.islower() or p.isdigit()) or (p is not None and p.isupper() and n is not None and n.islower())
            if b and not out.endswith("-"): out += "-"
            out += c.lower()
        else: out += c
    return out

base = sys.argv[1] if len(sys.argv) > 1 else "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
for art in sorted(os.listdir(base)):
    roots = glob.glob(f"{base}/{art}/**/🧬️schema/🧬️mutations", recursive=True)
    if not roots: continue
    for f in glob.glob(f"{roots[0]}/**/🦀️.rs", recursive=True):
        t = open(f, encoding="utf-8").read()
        for m in re.finditer(r'for (\w+)[^{]*\{\s*const SEMANTICS[^}]*kind: "([^"]+)"', t):
            if m.group(2) != to_kebab(m.group(1)):
                print(f"{art}\t{m.group(1)}\t{m.group(2)}\t{to_kebab(m.group(1))}")
