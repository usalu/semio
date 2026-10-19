"""🧹 Rewrites `&String` / `&Vec<T>` parameters of the extracted Rust phases to `&str` / `&[T]` (clippy::ptr_arg) after checking that the bodies only read through the deref-safe methods.
usage: ptrarg.py [APPLY] <file.rs>..."""
import re, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import rsfn
APPLY = sys.argv[1] == "APPLY"
files = sys.argv[2:] if APPLY else sys.argv[1:]
if files and files[0].startswith("@"):
    files = [l for l in open(files[0][1:], encoding="utf-8").read().split("\n") if l]
RISK = ("clone", "to_owned", "as_str", "as_slice", "push", "extend", "capacity", "into_boxed_slice", "into_iter", "retain", "remove", "insert", "truncate", "clear", "drain", "sort", "reserve")
total = 0
for f in files:
    text = open(f, encoding="utf-8").read()
    toks = rsfn.lex(text)
    edits = []
    k = 0
    while k < len(toks) - 1:
        if toks[k][1] == "fn" and toks[k][0] == "id" and toks[k + 1][0] == "id":
            name = toks[k + 1][1]
            nth = sum(1 for x in range(k) if toks[x][1] == "fn" and toks[x + 1][1] == name)
            try:
                kk, op, cl = rsfn.find_fn(toks, name, nth)
            except SystemExit:
                k += 1; continue
            s, e = rsfn.params_of(toks, kk, op)
            plist = text[toks[s][4]:toks[e][3]]
            for m in re.finditer(r"(\w+)\s*:\s*&(String|Vec<[^()]*?>)(?=\s*,|\s*$)", plist, re.S):
                pname, ty = m.group(1), m.group(2)
                body = [toks[q] for q in range(op, cl + 1)]
                bad = []
                clones = []
                for i, t in enumerate(body):
                    if t[0] == "id" and t[1] == pname and i + 2 < len(body) and body[i + 1][1] == "." and body[i + 2][1] in RISK and not (i > 0 and body[i - 1][1] == "."):
                        if body[i + 2][1] == "clone" and ty == "String" and i + 3 < len(body) and body[i + 3][1] == "(" and body[i + 4][1] == ")":
                            clones.append(body[i + 2])
                        else:
                            bad.append(body[i + 2][1])
                    if t[0] == "id" and t[1] == pname and i > 0 and body[i - 1][1] == "*":
                        bad.append("deref")
                if bad:
                    print("SKIP %s::%s %s %s" % (f[-50:], name, pname, bad)); continue
                for ct in clones:
                    edits.append((ct[3], ct[4], "to_owned"))
                new = "str" if ty == "String" else "[" + ty[4:-1] + "]"
                edits.append((toks[s][4] + m.start(), toks[s][4] + m.end(), "%s: &%s" % (pname, new)))
                total += 1
            k = op + 1
        else:
            k += 1
    if APPLY and edits:
        for a, b, rep in sorted(edits, reverse=True):
            text = text[:a] + rep + text[b:]
        open(f, "w", encoding="utf-8").write(text)
print("params rewritten:" if APPLY else "params convertible:", total)
