#!/usr/bin/env python3
"""🧩 Splits a Rust function whose body is a single `vec![ a, b, ... ]` into slice functions of about N lines; the root extends one vector with every slice in order.
usage: vec2chunks.py <spec.json> [APPLY]   spec: {file, function, nth?, max_lines, noun}"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn, emojipool

spec = json.load(open(sys.argv[1], encoding="utf-8"))
APPLY = len(sys.argv) > 2 and sys.argv[2] == "APPLY"
file = spec["file"]
text = open(file, encoding="utf-8").read()
toks = rsfn.lex(text)
k, op, cl = rsfn.find_fn(toks, spec["function"], spec.get("nth", 0))
sts = rsfn.statements(toks, op, cl)
assert len(sts) == 1 and toks[sts[0][0]][1] == "vec", "the body must be one vec! literal"
vq = sts[0][0]
bo = vq + 2
assert toks[vq + 1][1] == "!" and toks[bo][1] == "["
d = 0
for q in range(bo, cl):
    if toks[q][1] in "([{": d += 1
    elif toks[q][1] in ")]}":
        d -= 1
        if d == 0:
            bc = q
            break
commas = []
d = 0
for q in range(bo + 1, bc):
    t = toks[q][1]
    if t in "([{": d += 1
    elif t in ")]}": d -= 1
    elif d == 0 and t == "," and toks[q][0] == "p": commas.append(q)
bounds = [bo] + commas
elems = []
for i, c in enumerate(bounds):
    end = commas[i] if i < len(commas) else bc
    s_off = toks[c][4]
    e_off = toks[end][3] if i < len(commas) else toks[bc][3]
    seg = text[s_off:toks[end][4] if i < len(commas) else toks[bc][3]]
    if seg.strip().rstrip(",").strip():
        elems.append(seg.strip("\n").rstrip().rstrip(",") + ",")
lines_of = lambda s: s.count("\n") + 1
chunks, cur, size = [], [], 0
for e in elems:
    n = lines_of(e)
    if cur and size + n > spec["max_lines"]:
        chunks.append(cur); cur = []; size = 0
    cur.append(e); size += n
if cur: chunks.append(cur)
sigtext = text[toks[k][3]:toks[op][3]]
ret = re.search(r"->\s*(.*?)\s*$", sigtext, re.S).group(1)
vis = "pub " if re.search(r"(?m)^\s*pub\s", text[max(0, toks[k][3] - 40):toks[k][3] + 3]) else ""
em = emojipool.pick(text, len(chunks))
fn = spec["function"]
helpers = []
names = []
for i, ch in enumerate(chunks):
    name = "%s_part_%d" % (fn, i + 1)
    names.append(name)
    body = "\n".join(ch)
    helpers.append("/// %s The %s slice %d of %d: %s.\nfn %s() -> %s {\n    vec![\n%s\n    ]\n}" % (em[i], spec.get("noun", "probe"), i + 1, len(chunks), spec.get("what", "its representative values"), name, ret, body))
root_body = "{\n    let mut all = %s();\n%s    all\n}" % (names[0], "".join("    all.extend(%s());\n" % n for n in names[1:]))
fn_end = toks[cl][4]
new_text = text[:toks[op][3]] + root_body.lstrip("") + "\n\n" + "\n\n".join(helpers) + text[fn_end:]
print("elements", len(elems), "chunks", [lines_of("\n".join(c)) for c in chunks], "emoji", em)
if APPLY:
    open(file, "w", encoding="utf-8").write(new_text)
    print("APPLIED")
