#!/usr/bin/env python3
"""🔬️ Exact owner-level reference graph of one crate from rustc's `-Zunpretty=hir-tree` dump.

Input is the raw dump (`cargo rustc --lib --profile check -- -Zunpretty=hir-tree`); `Mod`/`ExternCrate` path segments are skipped.
Every HIR owner (item, impl, assoc item) becomes a node; every resolved path inside it (`res: Def(.., DefId(0:..))`,
`SelfTyAlias { alias_to }`) becomes an edge to the local definition. Spans give the owner's file and line range (`xspans`: macro-expansion spans, used for derive-generated owners).

Impl owners also carry `self` (self type) and `trait` (implemented trait, `extern::` when foreign).\n\nUsage: hir-owner-graph.py HIR_TREE_DUMP OUT_JSON
"""
import json, re, sys
from collections import defaultdict

HEAD = re.compile(r"^DefId\(0:\d+ ~ [A-Za-z0-9_]+\[[0-9a-f]+\](.*)\) => OwnerNodes")
BARE = re.compile(r"^\s+DefId\((\d+):\d+ ~ [A-Za-z0-9_]+\[[0-9a-f]+\](.*?)\),?$")
ALIAS = re.compile(r"alias_to: DefId\((\d+):\d+ ~ [A-Za-z0-9_]+\[[0-9a-f]+\](.*?)\)")
SPAN = re.compile(r"^\s+span: (.+?\.rs):(\d+):\d+: (\d+):\d+ \(#(\d+)\)")


def main():
    src, out = sys.argv[1], sys.argv[2]
    owners = {}
    cur = None
    want = False
    for line in open(src, encoding="utf-8", errors="replace"):
        if line.startswith("DefId("):
            m = HEAD.match(line)
            if m:
                p = m.group(1).lstrip(":")
                cur = owners.setdefault(p, {"refs": defaultdict(int), "spans": defaultdict(lambda: [10**9, 0]), "xspans": defaultdict(lambda: [10**9, 0])})
            want = False
            slot = None
            continue
        if cur is None:
            continue
        if line.startswith("                        of_trait: Some("):
            slot = "trait"
        elif line.startswith("                        self_ty: Ty {"):
            slot = "self"
        if "res: Def(" in line:
            want = "kind"
            continue
        if want == "kind":
            want = False if line.strip().rstrip(",") in ("Mod", "ExternCrate") else True
            if want is False:
                continue
        if want:
            m = BARE.match(line)
            if m:
                want = False
                target = m.group(2).lstrip(":")
                if slot and slot not in cur:
                    cur[slot] = target if m.group(1) == "0" else "extern::" + target
                    slot = None
                if m.group(1) == "0":
                    cur["refs"][target] += 1
                continue
        if "alias_to: DefId(" in line:
            m = ALIAS.search(line)
            if m and m.group(1) == "0":
                cur["refs"][m.group(2).lstrip(":")] += 1
            continue
        m = SPAN.match(line)
        if m:
            f = m.group(1).replace("/./", "/")
            r = cur["spans" if m.group(4) == "0" else "xspans"][f]
            r[0] = min(r[0], int(m.group(2))); r[1] = max(r[1], int(m.group(3)))
    res = {}
    for p, o in owners.items():
        res[p] = {"refs": dict(o["refs"]), "spans": dict(o["spans"]), "xspans": dict(o["xspans"])}
        for k in ("self", "trait"):
            if k in o: res[p][k] = o[k]
    json.dump(res, open(out, "w"), ensure_ascii=False)
    print(f"{len(res)} owners, {sum(len(o['refs']) for o in res.values())} distinct owner->def edges")


if __name__ == "__main__":
    main()
