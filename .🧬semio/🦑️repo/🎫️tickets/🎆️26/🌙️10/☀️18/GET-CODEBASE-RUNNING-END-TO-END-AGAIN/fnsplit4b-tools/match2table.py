#!/usr/bin/env python3
"""🗂️ Rewrites a Rust `fn f(key: &str) -> T { match key { "lit" [| "lit"]* => EXPR, ..., other => FALLBACK } }` into a `const TABLE: &[(&str, T)]` plus a lookup function.
usage: match2table.py <spec.json> [APPLY]   spec: {file, function, nth?, table, emoji, doc, key?: "str"|"slice" (auto), copy_derive?: "StructName"}
The table keeps every arm expression verbatim (dedented by four spaces); the lookup keeps the fallback arm with its binding re-bound to the key."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn

spec = json.load(open(sys.argv[1], encoding="utf-8"))
APPLY = len(sys.argv) > 2 and sys.argv[2] == "APPLY"
file = spec["file"]
text = open(file, encoding="utf-8").read()
toks = rsfn.lex(text)
k, op, cl = rsfn.find_fn(toks, spec["function"], spec.get("nth", 0))
sts = rsfn.statements(toks, op, cl)
assert len(sts) == 1 and toks[sts[0][0]][1] == "match", "the body must be exactly one match"
mq = sts[0][0]
mopen = next(q for q in range(mq, cl) if toks[q][1] == "{")
d = 0
for q in range(mopen, len(toks)):
    if toks[q][1] == "{": d += 1
    elif toks[q][1] == "}":
        d -= 1
        if d == 0:
            mclose = q
            break
scrut = text[toks[mq + 1][3]:toks[mopen][3]].strip()
arms = rsfn.match_arms(toks, mopen, mclose)
fallback = arms[-1]
fb_pat = text[toks[fallback["ps"]][3]:toks[fallback["pe"]][3]].strip()
assert re.fullmatch(r"\w+|_", fb_pat), "last arm must be a catch-all binding: " + fb_pat
entries = []
multi = False
for a in arms[:-1]:
    ptoks = [toks[q] for q in range(a["ps"], a["pe"])]
    pats = [t[1] for t in ptoks if t[0] == "str"]
    assert all(t[0] == "str" or t[1] == "|" for t in ptoks), "non-literal pattern at line %d" % ptoks[0][2]
    if len(pats) > 1: multi = True
    body = text[toks[a["bs"]][3]:toks[a["be"]][4]]
    entries.append((pats, body))
sig_start = toks[k][3]
sig = text[sig_start:toks[op][3]]
ret = re.search(r"->\s*(.*?)\s*$", sig, re.S).group(1)
vis = re.match(r"((?:pub(?:\([^)]*\))?\s+)?)", text[sig_start - 12:sig_start].split("\n")[-1]).group(1)
lines = text.split("\n")
def dedent(s, n=4):
    return "\n".join(l[n:] if l.startswith(" " * n) else l for l in s.split("\n"))
ktype = "&[&str]" if multi else "&str"
start_line = toks[k][2]
attrs = []
al = start_line - 2
while al >= 0 and lines[al].lstrip().startswith("#["):
    attrs.insert(0, lines[al]); al -= 1
out = ["/// %s %s" % (spec["emoji"], spec["doc"])] + attrs + ["const %s: &[(%s, %s)] = &[" % (spec["table"], ktype, ret.replace("&'static ", "&"))]
for pats, body in entries:
    key = ("&[%s]" % ", ".join(pats)) if multi else pats[0]
    b = dedent(body)
    out.append("    (%s, %s)," % (key, b.lstrip() if "\n" not in b else b.lstrip()))
out.append("];")
fb_expr = text[toks[fallback["bs"]][3]:toks[fallback["be"]][4]]
find = "|(keys, _)| keys.contains(&%s)" % scrut if multi else "|(key, _)| *key == %s" % scrut
rebind = ("let %s = %s;\n            " % (fb_pat, scrut)) if fb_pat != "_" else ""
new_body = "{\n    match %s.iter().find(%s) {\n        Some((_, value)) => *value,\n        None => {\n            %s%s\n        }\n    }\n}" % (spec["table"], find, rebind, fb_expr.rstrip(","))
start_line = toks[k][2]
fn_start_off = text.rfind("\n", 0, sig_start) + 1
doc_start = fn_start_off
ln = start_line - 1
while ln - 1 >= 0 and lines[ln - 1].lstrip().startswith(("///", "#[")): ln -= 1
head_off = sum(len(l) + 1 for l in lines[:ln])
new_text = text[:head_off] + "\n".join(out) + "\n\n" + text[head_off:toks[op][3]] + new_body.lstrip("") + text[toks[cl][4]:]
if spec.get("copy_derive"):
    name = spec["copy_derive"]
    m = re.search(r"(?m)^(#\[derive\(([^)]*)\)\]\s*\n)?(pub(\([^)]*\))?\s+)?struct %s\b" % re.escape(name), new_text)
    assert m, "struct not found"
    if m.group(1):
        inner = m.group(2)
        add = [x for x in ("Clone", "Copy") if x not in inner]
        if add:
            new_text = new_text[:m.start(2)] + inner.rstrip() + ", " + ", ".join(add) + new_text[m.end(2):]
    else:
        new_text = new_text[:m.start()] + "#[derive(Clone, Copy)]\n" + new_text[m.start():]
used = {m.replace("\ufe0f", "") for m in re.findall(r"(?m)^\s*(?:///|//!)\s*(\S+)", text)}
if spec["emoji"].replace("\ufe0f", "") in used: print("WARNING: doc emoji", spec["emoji"], "already used in the file")
print("arms", len(entries), "multi", multi, "table lines", len("\n".join(out).split("\n")), "fn lines", len(new_body.split("\n")))
if APPLY:
    open(file, "w", encoding="utf-8").write(new_text)
    print("APPLIED")
else:
    sys.stdout.write("\n".join(new_text.split("\n")[ln - 1: ln + 6]) + "\n...\n")
    tail_ln = new_text.count("\n", 0, new_text.index("Some((_, value))")) 
    sys.stdout.write("\n".join(new_text.split("\n")[tail_ln - 3: tail_ln + 8]) + "\n")
