"""🧱 Builds an inline phase spec for rsfn.py plan/apply. usage (import): build(file, function, rows, types=None, modes=None, keep=None, nth=0, out=None, **extra)
rows: [(name, doc, from, to)] or [(name, doc, from, to, {extra phase keys})]; every phase gets a unique doc emoji and lands as a free function after the function."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import emojipool

def build(file, function, rows, types=None, modes=None, keep=None, nth=0, out=None, **extra):
    text = open(file, encoding="utf-8").read()
    em = emojipool.pick(text, len(rows))
    phases = []
    for r, e in zip(rows, em):
        name, doc, a, b = r[:4]
        opt = r[4] if len(r) > 4 else {}
        p = {"name": name, "doc": "%s %s" % (e, doc), "from": a, "to": b}
        p.update(opt)
        phases.append(p)
    if keep == "auto":
        import rsfn
        text_ = open(file, encoding="utf-8").read()
        blk = extra.get("block_line")
        if blk:
            toks_, k_, op_, cl_, params_, rows_ = rsfn.analyse_block(text_, function, nth, blk, extra.get("context", {}))
        else:
            toks_, k_, op_, cl_, params_, rows_ = rsfn.analyse(text_, function, nth)
        covered = set()
        for ph in phases:
            for r in rows_:
                if ph["from"] <= r["a"] <= ph["to"]: covered.add(r["a"])
        keep = [r["a"] for r in rows_ if r["a"] not in covered]
    spec = {"file": file, "function": function, "nth": nth, "types": types or {}, "modes": modes or {}, "keep": keep or [], "phases": phases}
    spec.update(extra)
    if out: json.dump(spec, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    return spec
