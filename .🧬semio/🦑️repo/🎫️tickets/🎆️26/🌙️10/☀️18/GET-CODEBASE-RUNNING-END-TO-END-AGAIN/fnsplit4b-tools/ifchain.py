#!/usr/bin/env python3
"""🌿 Extracts branches of an if / else-if / else chain of a Rust function into helper functions placed after it; each branch body becomes `helper(args)?;`-style calls.
usage: ifchain.py <spec.json> [APPLY]
spec: {file, function, nth?, if_line, ret ("Result<(), String>"), tail ("Ok(())"), call ("{name}({args})?;"), types:{name:type}, modes:{name:"mut"|"ref"|"val"}, branches:[{pos, name, doc}], method?: {self_param, call_prefix}}"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn, emojipool
spec = json.load(open(sys.argv[1], encoding="utf-8")); APPLY = len(sys.argv) > 2
file = spec["file"]
text = open(file, encoding="utf-8").read()
toks = rsfn.lex(text)
k, op, cl = rsfn.find_fn(toks, spec["function"], spec.get("nth", 0))
iq = next(q for q in range(op, cl) if toks[q][1] == "if" and toks[q][2] == spec["if_line"])
def block_end(q):
    d = 0
    for i in range(q, len(toks)):
        if toks[i][1] == "{": d += 1
        elif toks[i][1] == "}":
            d -= 1
            if d == 0: return i
def block_open(q):
    d = 0
    for i in range(q, len(toks)):
        t = toks[i][1]
        if t in "([": d += 1
        elif t in ")]": d -= 1
        elif d == 0 and t == "{": return i
branches = []
q = iq
while True:
    bo = block_open(q + 1); be = block_end(bo)
    branches.append((bo, be))
    if be + 1 < len(toks) and toks[be + 1][1] == "else":
        if toks[be + 2][1] == "if": q = be + 2; continue
        bo2 = be + 2; be2 = block_end(bo2); branches.append((bo2, be2)); break
    break
print("chain branches:", [(toks[a][2], toks[b][2]) for a, b in branches])
types = spec["types"]; modes = spec.get("modes", {})
em = emojipool.pick(text, len(spec["branches"]))
edits, helpers = [], []
for br, e in zip(spec["branches"], em):
    bo, be = branches[br["pos"]]
    inner_s, inner_e = toks[bo][4], toks[be][3]
    used = rsfn.used_in(toks, bo + 1, be - 1, set(types))
    names = [n for n in types if n in used]
    pl, args = [], []
    for n in names:
        ty = types[n]; mo = modes.get(n, "ref")
        if ty.startswith("&"): pl.append("%s: %s" % (n, ty)); args.append(n)
        elif mo == "mut": pl.append("%s: &mut %s" % (n, ty)); args.append("&mut " + n)
        elif mo == "ref": pl.append("%s: &%s" % (n, ty)); args.append("&" + n)
        else: pl.append("%s: %s" % (n, ty)); args.append(n)
    body = text[inner_s:inner_e]
    for n in names:
        if modes.get(n, "ref") == "ref" and not types[n].startswith("&"):
            body = re.sub(r"(?<![&\w])&%s\b(?!\s*[\.\[\(!]|::)" % re.escape(n), n, body)
    lines = body.split("\n")
    ind = len(re.match(r"\s*", lines[1] if len(lines) > 1 else "").group(0)) if len(lines) > 1 else 0
    lines = [l[ind - 4:] if l.startswith(" " * (ind - 4)) and ind >= 4 else l for l in lines]
    body = "\n".join(lines).strip("\n")
    sig = "fn %s(%s) -> %s {" % (br["name"], ", ".join(pl), spec["ret"])
    if len(sig) > 150: sig = "fn %s(\n    %s,\n) -> %s {" % (br["name"], ",\n    ".join(pl), spec["ret"])
    helpers.append("/// %s %s\n%s\n%s\n    %s\n}" % (e, br["doc"], sig, body, spec.get("tail", "Ok(())")))
    call = spec.get("call", "{name}({args})?;").format(name=br["name"], args=", ".join(args))
    base = len(re.match(r"\s*", text[text.rfind("\n", 0, toks[bo][3]) + 1:]).group(0))
    edits.append((inner_s, inner_e, "\n" + " " * (base + 4) + call + "\n" + " " * base))
    print("branch", br["name"], "lines", toks[be][2] - toks[bo][2] + 1, "ins", names)
if APPLY:
    new = text
    for s_, e_, rep in sorted(edits, reverse=True): new = new[:s_] + rep + new[e_:]
    shift = sum(len(r) - (e_ - s_) for s_, e_, r in edits)
    end = toks[cl][4] + shift
    new = new[:end] + "\n\n" + "\n\n".join(helpers) + new[end:]
    open(file, "w", encoding="utf-8").write(new)
    print("APPLIED")
