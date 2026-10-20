#!/usr/bin/env python3
"""⛓️ Splits the method chain `Head(...).a(...).b(...)....z(...)` that ends a Rust function into segment functions `fn part_N(builder: B) -> B { builder.a(...)... }` placed after it; the function threads the builder through them and keeps the last calls.
usage: chain2fns.py <spec.json> [APPLY]   spec: {file, function, nth?, max_lines, builder_type, keep_tail: N (calls kept in the root, default 1), types: {local: type}, noun}"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn, emojipool
spec = json.load(open(sys.argv[1], encoding="utf-8")); APPLY = len(sys.argv) > 2
file = spec["file"]
text = open(file, encoding="utf-8").read()
toks = rsfn.lex(text)
k, op, cl = rsfn.find_fn(toks, spec["function"], spec.get("nth", 0))
sts = rsfn.statements(toks, op, cl)
cs, ce = sts[-1]
assert toks[ce][1] != ";", "the chain must be the tail expression"
dots, d = [], 0
for q in range(cs, ce + 1):
    t = toks[q][1]
    if t in "([{": d += 1
    elif t in ")]}": d -= 1
    elif d == 0 and t == "." and toks[q][0] == "p" and toks[q - 1][1] == ")": dots.append(q)
off = [toks[q][3] for q in dots]
head = text[toks[cs][3]:off[0]]
calls = [text[off[i]:(off[i + 1] if i + 1 < len(off) else toks[ce][4])] for i in range(len(off))]
keep_tail = spec.get("keep_tail", 1)
movable = calls[:len(calls) - keep_tail]
tail = calls[len(calls) - keep_tail:]
lc = lambda s: s.count("\n") + 1
segs, cur, size = [], [], 0
for c in movable:
    if cur and size + lc(c) > spec["max_lines"]:
        segs.append(cur); cur = []; size = 0
    cur.append(c); size += lc(c)
if cur: segs.append(cur)
B = spec["builder_type"]
em = emojipool.pick(text, len(segs))
fn = spec["function"]
locals_ = spec.get("types", {})
helpers, root_calls, names = [], [], []
for i, sg in enumerate(segs):
    body = "".join(sg)
    used = [n for n in locals_ if re.search(r"(?<![\w.])%s\b" % re.escape(n), body)]
    pl = ["builder: %s" % B] + ["%s: %s" % (n, locals_[n]) for n in used]
    name = "%s_part_%d" % (fn, i + 1)
    names.append(name)
    body_lines = body.split("\n")
    helpers.append("/// %s The %s slice %d of %d of `%s`: %s.\nfn %s(%s) -> %s {\n    builder%s\n}" % (em[i], spec.get("noun", "manifest"), i + 1, len(segs), fn, spec.get("what", "its builder calls"), name, ", ".join(pl), B, body))
    root_calls.append("    let builder = %s(%s);" % (name, ", ".join(["builder"] + used)))
ind = re.match(r"\s*", text[text.rfind("\n", 0, toks[cs][3]) + 1:]).group(0)
new_tail = ind + "let builder = " + head.strip() + ";\n" + "\n".join(root_calls) + "\n" + ind + "builder" + "".join(tail)
new_text = text[:toks[cs][3] - len(ind)] + new_tail + text[toks[ce][4]:]
end = toks[cl][4] + (len(new_text) - len(text))
new_text = new_text[:end] + "\n\n" + "\n\n".join(helpers) + new_text[end:]
print("calls", len(calls), "segments", [sum(lc(c) for c in sg) for sg in segs], "tail calls", len(tail), "emoji", em)
if APPLY:
    open(file, "w", encoding="utf-8").write(new_text)
    print("APPLIED")
