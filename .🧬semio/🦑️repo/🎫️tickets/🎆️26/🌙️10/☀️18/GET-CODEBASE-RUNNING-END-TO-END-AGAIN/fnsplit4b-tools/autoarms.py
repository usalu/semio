#!/usr/bin/env python3
"""🤖 Derives an inline arms spec for rsfn.py from the arms of one match. usage: autoarms.py <file> <fn> <matchLine> <ret> <prefix> <out.json> [--min N] [--nth N] [--method] [--type name=T ...] [--argexpr name=expr] [--skip line,...] [--only line,...] [--search DIR] [--allow-no-returns]
Names are prefix_snake(variant); struct-variant bindings are typed from the enum definition found by grepping DIR (default: the file's directory tree)."""
import json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn, emojipool

def snake(s):
    return re.sub(r"(?<!^)([A-Z])", lambda m: "_" + m.group(1), s).lower()


def enum_variant_fields(search, variant):
    d = search
    for _ in range(9):
        try:
            return enum_variant_fields_in(d, variant)
        except SystemExit:
            d = os.path.dirname(d)
    raise SystemExit("enum variant %s not found above %s" % (variant, search))

def enum_variant_fields_in(search, variant):
    for root, _, files in os.walk(search):
        for fn in files:
            if not fn.endswith(".rs"): continue
            t = open(os.path.join(root, fn), encoding="utf-8").read()
            for m in re.finditer(r"\benum\s+\w+[^{]*\{", t):
                i = m.end(); d = 1
                while i < len(t) and d:
                    d += (t[i] == "{") - (t[i] == "}"); i += 1
                body = t[m.end():i - 1]
                vm = re.search(r"(?m)^\s*%s\s*\{([^}]*)\}" % re.escape(variant), body)
                if vm: return vm.group(1)
    raise SystemExit("enum variant %s not found under %s" % (variant, search))

def main():
    a = sys.argv[1:]
    file, fname, mline, ret, prefix, out = a[:6]
    opt = a[6:]
    def flag(n, default=None):
        return opt[opt.index(n) + 1] if n in opt else default
    minl = int(flag("--min", "15")); nth = int(flag("--nth", "0"))
    skip = {int(x) for x in flag("--skip", "").split(",") if x}
    only = {int(x) for x in flag("--only", "").split(",") if x}
    types = {}
    argexpr = {}
    for i, x in enumerate(opt):
        if x == "--type": k, v = opt[i + 1].split("=", 1); types[k] = v
        if x == "--argexpr": k, v = opt[i + 1].split("=", 1); argexpr[k] = v
    search = flag("--search", os.path.dirname(file))
    text = open(file, encoding="utf-8").read()
    toks = rsfn.lex(text)
    k, op, cl = rsfn.find_fn(toks, fname, nth)
    mq = next(q for q in range(op, cl) if toks[q][1] == "match" and toks[q][2] == int(mline))
    mopen = next(q for q in range(mq, cl) if toks[q][1] == "{")
    b = 0
    for q in range(mopen, len(toks)):
        if toks[q][1] == "{": b += 1
        elif toks[q][1] == "}":
            b -= 1
            if b == 0:
                mclose = q
                break
    arms = rsfn.match_arms(toks, mopen, mclose)
    chosen = []
    for ar in arms:
        ln = toks[ar["ps"]][2]
        n = toks[ar["be"]][2] - ln + 1
        if only and ln not in only: continue
        if not only and (n < minl or ln in skip): continue
        pat = text[toks[ar["ps"]][3]:toks[ar["pe"]][3]].strip()
        chosen.append((ln, pat, n))
    taken = set(re.findall(r"\bfn\s+(\w+)", text))
    rows = []
    for ln, pat, n in chosen:
        alts = [p.strip() for p in re.split(r"\|(?![^{(]*[})])", pat)]
        heads = []
        binds = {}
        for alt in alts:
            m = re.match(r"^(?:[\w:]*::)?(\w+)\s*(\{[^}]*\})?\s*(\([^)]*\))?\s*(?:if .*)?$", alt, re.S)
            if not m:
                heads.append(re.sub(r"\W+", "_", alt).strip("_").lower() or "arm"); continue
            heads.append(snake(m.group(1)))
            if m.group(2):
                fields = [f.strip() for f in m.group(2).strip("{} ").split(",") if f.strip() and f.strip() != ".."]
                if fields:
                    defn = enum_variant_fields(search, m.group(1))
                    for f in fields:
                        name = f.split(":")[0].strip()
                        tm = re.search(r"\b%s\s*:\s*([^,}]+)" % re.escape(name), defn)
                        if not tm: raise SystemExit("no type for bind %s of %s" % (name, m.group(1)))
                        binds[name] = tm.group(1).strip()
            elif m.group(3):
                raise SystemExit("tuple pattern needs manual types: line %d %s" % (ln, alt))
        base = heads[0] if len(heads) == 1 else heads[0] + "_or_" + heads[1] if len(heads) == 2 else heads[0] + "_and_more"
        name = "%s_%s" % (prefix, base)
        i = 2
        while name in taken:
            name = "%s_%s_%d" % (prefix, base, i); i += 1
        taken.add(name)
        label = " or ".join("`%s`" % h.replace("_", " ") for h in heads[:2])
        rows.append((ln, name, "Runs the %s %s of `%s`." % (label, flag("--noun", "arm"), fname), {"binds": binds}))
    import armspec
    method = {"self_param": flag("--self", "&mut self"), "call": "self.{name}({args})"} if "--method" in opt else None
    extra = {"argexpr": argexpr} if argexpr else {}
    if flag("--generics"): extra["generics"] = flag("--generics")
    if "--wrap-ok" in opt: extra["wrap"] = ["Ok({", "})"]
    if flag("--call-suffix"): extra["call_suffix"] = flag("--call-suffix")
    if flag("--fall-tail"): extra["fall_tail"] = flag("--fall-tail")
    if "--flow-option" in opt: extra["flow_option"] = True
    if flag("--flow-wrap"): extra["flow_wrap"] = flag("--flow-wrap")
    if flag("--call-wrap"): extra["call_wrap"] = flag("--call-wrap")
    if "--ensure-semicolon" in opt: extra["ensure_semicolon"] = True
    pre = [{"regex": x.split("=>", 1)[0], "line": x.split("=>", 1)[1]} for i, x in enumerate(opt) if i and opt[i - 1] == "--prelude"]
    if pre: extra["prelude_ifs"] = pre
    armspec.build(file, fname, int(mline), rows, ret, method=method, types=types, nth=nth, out=out, **extra)
    print("arms:", ", ".join("%d:%s(%d)" % (r[0], r[1], c[2]) for r, c in zip(rows, chosen)))

main()
