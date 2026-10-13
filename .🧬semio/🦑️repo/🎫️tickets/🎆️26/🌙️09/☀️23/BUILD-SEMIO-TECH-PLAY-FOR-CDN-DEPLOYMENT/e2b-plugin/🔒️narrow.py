#!/usr/bin/env python3
"""🔒️ Wave C4: shrinks `pub(crate)` of `semio-framework-plugin` to the accesses rustc proves.

`--strip` removes every `pub(crate)` of the crate's own files except the component root (its module declarations are
what the package root's `pub use component::*` exposes as `crate::<module>`) and except `macro_rules!` bodies (their
items are expanded in the invoking module). `🔁️fix-loop.py --apply` then re-widens exactly the items, methods and fields
rustc names in E0603/E0616/E0624/E0451, configuration by configuration, until every configuration compiles.
`--count` prints the crate's `pub(crate)` total; `--classify` splits the survivors into intra-facet (stay
`pub(crate)` in the facet crate) and cross-facet (become `pub` API of their facet crate at the cut). It compiles nothing.
Usage: narrow.py --strip | --count | --classify
"""
import importlib.util, os, re, sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("move", os.path.join(HERE, "🚚️move.py"))
mv = importlib.util.module_from_spec(spec); spec.loader.exec_module(mv)
ri, PLUGIN = mv.ri, mv.PLUGIN
ROOT = os.path.join(PLUGIN, "🦀️.rs")
PAT = re.compile(r"\bpub\(crate\)\s+")


def files():
    return [f for f in mv.crate_files() if f != ROOT and "/📦️packages/" not in f]


def macro_spans(cl):
    out = []
    for m in re.finditer(r"macro_rules!\s*\w+\s*\{", cl):
        ob = m.end() - 1
        out.append((ob, ri.skip_group(cl, ob)))
    return out


def strip():
    total = 0
    for f in files():
        raw = open(f, encoding="utf-8").read()
        cl = ri.clean(raw)
        holes = macro_spans(cl)
        hits = [m for m in PAT.finditer(cl) if not any(a <= m.start() < b for a, b in holes)]
        if not hits: continue
        for m in reversed(hits): raw = raw[:m.start()] + raw[m.end():]
        open(f, "w", encoding="utf-8").write(raw)
        total += len(hits)
    print(f"[narrow] stripped {total} pub(crate)")


def count():
    n = sum(len(PAT.findall(ri.clean(open(f, encoding="utf-8").read()))) for f in files() + [ROOT])
    print(f"[narrow] {n} pub(crate)")


def classify():
    spec2 = importlib.util.spec_from_file_location("item_graph", os.path.join(HERE, "🕸️item-graph.py"))
    ig = importlib.util.module_from_spec(spec2); spec2.loader.exec_module(ig)
    items, owners, out, inc = ig.build()
    c = Counter()
    for i, r in enumerate(items):
        if not r["name"] or r["name"].startswith("impl ") or not re.match(r"\s*pub\(crate\)", r["text"]): continue
        users = {items[j]["facet"] for j in inc.get(i, {})} - {r["facet"]}
        c["cross-facet" if users else "intra-facet"] += 1
    print(f"[narrow] top-level pub(crate) items: {dict(c)}")


if __name__ == "__main__":
    {"--strip": strip, "--count": count, "--classify": classify}[sys.argv[1]]()
