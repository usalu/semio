#!/usr/bin/env python3
"""🧬 Replaces N identical depth-1 blocks of a Rust test (differing only in `let name = LITERAL;` header lines) by calls of one parametrized helper.
usage: dedupe-blocks.py <spec.json> [APPLY]  spec: {file, function, nth?, first_block_line, last_block_end?, params:[{name, type}], helper, doc_emoji_doc, ctx:[{name, type}], call_ctx:[expr...], generics?: "<T>", where?: "..."}
Each block must start with `{` at depth 1 and contain `let <param> = <literal>;` lines for every param; the literals become call arguments (strings stay quoted)."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn, emojipool
spec = json.load(open(sys.argv[1], encoding="utf-8")); APPLY = len(sys.argv) > 2
file = spec["file"]
text = open(file, encoding="utf-8").read()
toks, k, op, cl, params, rows = rsfn.analyse(text, spec["function"], spec.get("nth", 0))
blocks = [r for r in rows if r["head"] == "{" and r["a"] >= spec["first_block_line"]]
lines = text.split("\n")
pn = [p["name"] for p in spec["params"]]
vals, norm = [], []
for r in blocks:
    body = lines[r["a"]:r["b"] - 1]
    vs = {}
    keep = []
    for l in body:
        m = re.match(r"^\s*let (%s) = (.+);\s*$" % "|".join(pn), l)
        if m and m.group(1) not in vs:
            vs[m.group(1)] = m.group(2); continue
        keep.append(l)
    assert set(vs) == set(pn), (r["a"], vs)
    t = "\n".join(keep)
    for n in sorted(pn, key=lambda n: -len(vs[n])):
        lit = vs[n].strip('"')
        t = t.replace(lit, "\u0001" + n.upper() + "\u0001") if lit and not lit.isdigit() else t
    vals.append(vs); norm.append(t)
base = norm[0]
bad = [i for i, n in enumerate(norm) if n != base]
print("blocks", len(blocks), "identical-after-normalization", len(blocks) - len(bad), "differing", bad)
if bad:
    import difflib
    i = bad[0]
    print("\n".join(list(difflib.unified_diff(base.split("\n"), norm[i].split("\n"), lineterm=""))[:20]))
    sys.exit(1)
em = emojipool.pick(text, 1)[0]
body0 = lines[blocks[0]["a"]:blocks[0]["b"] - 1]
body0 = [l for l in body0 if not re.match(r"^\s*let (%s) = .+;\s*$" % "|".join(pn), l)]
for n in pn:
    lit = re.search(r"let %s = (.+);" % n, "\n".join(lines[blocks[0]["a"]:blocks[0]["b"] - 1])).group(1).strip('"')
    body0 = [l.replace('"%s"' % lit, n) if False else l for l in body0]
dirlit = {n: re.search(r"let %s = (.+);" % n, "\n".join(lines[blocks[0]["a"]:blocks[0]["b"] - 1])).group(1) for n in pn}
body0 = [re.sub(r"\(%s\)" % re.escape(dirlit[spec.get("path_param", pn[0])]), "(%s)" % spec.get("path_param", pn[0]), l) for l in body0]
body0 = [l[4:] if l.startswith("    ") else l for l in body0]
sig_params = ", ".join("%s: %s" % (c["name"], c["type"]) for c in spec["ctx"] + spec["params"])
gen = spec.get("generics", "")
helper = ["/// %s %s" % (em, spec["doc"]), "fn %s%s(%s)%s {" % (spec["helper"], gen, sig_params, (" " + spec["where"]) if spec.get("where") else "")] + ["    " + l if l.strip() else l for l in body0] + ["}"]
calls = []
for vs in vals:
    calls.append("    %s(%s);" % (spec["helper"], ", ".join(spec["call_ctx"] + [vs[n] for n in pn])))
fn_start = toks[k][2]
first = blocks[0]["a"]; last = blocks[-1]["b"]
new_lines = lines[:first - 1] + calls + lines[last:]
# helper goes before the test function (after its attributes)
ins = fn_start - 1
while ins - 1 >= 0 and lines[ins - 1].lstrip().startswith(("#[", "///")): ins -= 1
shift_ins = ins
out = new_lines[:shift_ins] + helper + [""] + new_lines[shift_ins:]
print("helper lines", len(helper), "calls", len(calls), "emoji", em)
if APPLY:
    open(file, "w", encoding="utf-8").write("\n".join(out))
    print("APPLIED")
