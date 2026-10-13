#!/usr/bin/env python3
"""🕸️ Item-level facet graph of `semio-framework-plugin` for planning the E2b back-edge waves.

Reuses the module walk and the facet assignment of `🧭️facet-imports.py`. Every production module is split into its
top-level items (E1's `🧱️rust_items.py`); an `impl` joins the item that defines its self type (else its trait) in the
same crate. An edge is a reference by identifier to a name defined exactly once at module level. It compiles nothing,
and it only plans: `🧭️facet-imports.py` (import paths) plus rustc stay the verdict.

Usage:
  item-graph.py --back                       item edges against the facet order
  item-graph.py --refs NAME                  outgoing and incoming edges of one item
  item-graph.py --down FACET NAME[,NAME…]    closure that must move down with NAME to sit in FACET
  item-graph.py --up FACET NAME[,NAME…]      items below FACET that use NAME (they block moving NAME up)
"""
import importlib.util, os, re, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("facet_imports", os.path.join(HERE, "🧭️facet-imports.py"))
fi = importlib.util.module_from_spec(spec); spec.loader.exec_module(fi)
ri, ORDER = fi.ri, fi.ORDER
IDENT = re.compile(r"\b[A-Za-z_][A-Za-z0-9_]*\b")


def build():
    fi.walk(f"{fi.PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{fi.PLUGIN}/📦️packages/🦀️rust")
    top = {}
    for m in fi.modules:
        s = m.path.split("::")
        if len(s) == 3 and s[2] not in top: top[s[2]] = m
    fac = {k: fi.facet_of(v) for k, v in top.items()}
    items, impls = [], []
    for m in fi.modules:
        if m.test: continue
        s = m.path.split("::")
        if len(s) < 3 or s[2] == "app": continue
        key = s[2]
        raw, cl = fi.load(m.file)
        a, b = m.rng
        a += 1 if cl[a:a + 1] == "{" else 0
        holes = sorted((c.rng for c in fi.modules if c.file == m.file and c is not m and a <= c.rng[0] < b))
        its, _ = ri.items(raw, cl, a, b)
        for it in its:
            if any(x <= it["head"] <= y for x, y in holes): continue
            if it["kind"] in ("use", "mod", "extern_crate", "macro_call"): continue
            head = raw[max(0, it["head"] - 300):it["head"]]
            if re.search(r"#\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*$", head): continue
            text = cl[it["head"]:it["end"]]
            rec = {"mod": key, "facet": fac.get(key, "other"), "kind": it["kind"], "name": it.get("name"), "text": text,
                   "file": os.path.relpath(m.file, fi.PLUGIN), "line": ri.line_of(raw, it["head"]), "lines": text.count("\n") + 1}
            if it["kind"] == "impl": rec["self"], rec["trait"] = it.get("self"), it.get("trait"); impls.append(rec)
            else: items.append(rec)
    owners = defaultdict(list)
    for i, r in enumerate(items):
        if r["name"]: owners[r["name"]].append(i)
    for r in impls:
        o = owners.get(r["self"]) or owners.get(r["trait"])
        if o and len(o) == 1 and items[o[0]]["mod"] == r["mod"]:
            items[o[0]]["text"] += "\n" + r["text"]; items[o[0]]["lines"] += r["lines"]
        else:
            r["name"] = f"impl {r['trait'] + ' for ' if r['trait'] else ''}{r['self']}@{r['mod']}:{r['line']}"
            items.append(r)
    owners = defaultdict(list)
    for i, r in enumerate(items):
        if r["name"] and not r["name"].startswith("impl "): owners[r["name"]].append(i)
    modtext = defaultdict(str)
    for m in fi.modules:
        s_ = m.path.split("::")
        if not m.test and len(s_) >= 3: modtext[s_[2]] += fi.load(m.file)[1][m.rng[0]:m.rng[1]] + "\n"
    memo = {}

    def imported(src, dst, t):
        k = (src, dst, t)
        if k not in memo: memo[k] = re.search(rf"\b{dst}::(?:\w+::)*(?:\{{[^;]*?\b{t}\b|{t}\b)", modtext[src]) is not None
        return memo[k]
    out = defaultdict(lambda: defaultdict(set))
    for i, r in enumerate(items):
        txt = r["text"]
        toks = {t for t in IDENT.findall(txt) if t[0].isupper()}
        toks |= set(re.findall(r"(?<![.\w])([a-z_][a-z0-9_]*)\s*(?:\(|!|::<)", txt)) | set(re.findall(r"::([a-z_][a-z0-9_]*)\b", txt))
        toks |= set(re.findall(r"(?<![.\w])([a-z_][a-z0-9_]*)\s*(?=[,)\]}])", txt))
        for t in toks:
            o = owners.get(t)
            if not o or len(o) != 1 or o[0] == i: continue
            m = items[o[0]]["mod"]
            if m == r["mod"] or imported(r["mod"], m, t): out[i][o[0]].add(t)
        if r["kind"] == "impl":
            for t in (r.get("self"), r.get("trait")):
                o = owners.get(t)
                if o and len(o) == 1: out[i][o[0]].add(t)
    inc = defaultdict(lambda: defaultdict(set))
    for a, tos in out.items():
        for b, n in tos.items(): inc[b][a] |= n
    return items, owners, out, inc


def rank(f): return ORDER.index(f) if f in ORDER else -1


def label(r): return f"{r['facet']}:{r['mod']}::{r['name']} [{r['lines']}] ({r['file']}:{r['line']})"


def main():
    a = sys.argv[1:]
    items, owners, out, inc = build()
    idx = lambda n: owners[n][0] if n in owners and len(owners[n]) == 1 else sys.exit(f"ambiguous or unknown: {n}")
    if "--back" in a:
        rows = defaultdict(set)
        for i, tos in out.items():
            for j, n in tos.items():
                if rank(items[j]["facet"]) > rank(items[i]["facet"]) >= 0: rows[(i, j)] |= n
        for (i, j) in sorted(rows, key=lambda x: (rank(items[x[0]]["facet"]), items[x[0]]["mod"], items[x[0]]["name"] or "")):
            print(f"{label(items[i])}\n      -> {label(items[j])}")
        print(f"\n{len(rows)} item edges, {len({j for _, j in rows})} distinct targets")
    if "--refs" in a:
        i = idx(a[a.index("--refs") + 1]); print(label(items[i]))
        for j in sorted(out[i], key=lambda j: -rank(items[j]["facet"])): print("   ->", label(items[j]))
        for j in sorted(inc[i], key=lambda j: rank(items[j]["facet"])): print("   <-", label(items[j]))
    for flag, forward in (("--down", True), ("--up", False)):
        if flag not in a: continue
        f = a[a.index(flag) + 1]; seeds = [idx(n) for n in a[a.index(flag) + 2].split(",")]
        seen, stack = set(seeds), list(seeds)
        while stack:
            i = stack.pop()
            for j in (out[i] if forward else inc[i]):
                if j in seen: continue
                if (forward and rank(items[j]["facet"]) > rank(f)) or (not forward and 0 <= rank(items[j]["facet"]) < rank(f)):
                    seen.add(j); stack.append(j)
        for i in sorted(seen, key=lambda i: (rank(items[i]["facet"]), items[i]["mod"])): print(label(items[i]))
        print(f"\n{len(seen)} items, {sum(items[i]['lines'] for i in seen)} lines")


if __name__ == "__main__":
    main()
