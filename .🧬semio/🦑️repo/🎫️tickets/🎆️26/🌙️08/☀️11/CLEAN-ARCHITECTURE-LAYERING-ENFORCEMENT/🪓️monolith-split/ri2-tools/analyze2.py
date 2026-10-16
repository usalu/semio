#!/usr/bin/env python3
# 🔬️ Item-level direction graph of the Shell staging trees: every top-level item (free item, type, impl block) with the free items and types it names, and every edge that points up the crate rank order.
import json, sys, collections, re
from model import *
import items as itm

def impl_type(head):
    h = re.sub(r"^impl\s*(<[^>]*(?:<[^>]*>[^>]*)*>)?\s*", "", head.strip())
    m = re.match(r"(?:[A-Za-z_][A-Za-z0-9_:<>, &']*\s+for\s+)?&?(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)", h)
    if " for " in head:
        tail = head.split(" for ", 1)[1]
        m2 = re.match(r"\s*&?(?:mut\s+)?(?:'?\w+\s+)?([A-Za-z_][A-Za-z0-9_]*)", tail)
        return m2.group(1) if m2 else None
    return m.group(1) if m else None

def current_mounts():
    import json
    ms = [dict(m) for m in original_mounts() if m["kind"] in ("staging", "chunk")]
    st = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/state.json")) if os.path.exists(os.path.dirname(os.path.abspath(__file__)) + "/state.json") else {"pulled": {}}
    have = {m["file"] for m in ms}
    for suffix, lst in st["pulled"].items():
        for pm in lst:
            if pm["file"] not in have:
                ms.append(dict(name=pm["name"], file=pm["file"], cfg=[], vis="", reexport=pm.get("reexport", []), kind="staging", crate=suffix, slug=pm["slug"]))
    return ms


def load():
    ms = current_mounts()
    its = []
    for m in ms:
        text = read(m["file"])
        lines = text.split("\n")
        for it in itm.table(text):
            body = "\n".join(lines[it["start"] - 1:it["end"]])
            head = lines[it["head"] - 1]
            rec = dict(mod=m["name"], crate=m["crate"], slug=m["slug"], kind=it["kind"], name=it["name"], vis=it["vis"], file=m["file"], start=it["start"], end=it["end"], body=body, mk=m["kind"])
            if it["kind"] == "impl":
                rec["impl_type"] = impl_type(head)
                rec["trait"] = " for " in head
            its.append(rec)
    return ms, its

def graph(its):
    defs = collections.defaultdict(list)
    for i, it in enumerate(its):
        if it["kind"] not in ("impl",):
            defs[it["name"]].append(i)
    edges = collections.defaultdict(set)
    for i, it in enumerate(its):
        toks = tokens(it["body"])
        own = it["name"]
        for t in toks:
            if t in defs:
                for j in defs[t]:
                    if j != i:
                        edges[i].add(j)
    return defs, edges

if __name__ == "__main__":
    ms, its = load()
    defs, edges = graph(its)
    print("items", len(its), "edges", sum(len(v) for v in edges.values()))
    up = []
    for i, ds in edges.items():
        ci = its[i]["crate"]
        if ci is None: continue
        for j in ds:
            cj = its[j]["crate"]
            if cj is None: continue
            if RANK[cj] > RANK[ci]:
                up.append((i, j))
    print("upward edges", len(up))
    byp = collections.Counter((its[i]["crate"], its[j]["crate"]) for i, j in up)
    for (a, b), n in sorted(byp.items(), key=lambda kv: (RANK[kv[0][0]], RANK[kv[0][1]])):
        print(f"  {a} -> {b}: {n}")
    json.dump([[its[i]["crate"], its[i]["slug"], its[i]["name"], its[i]["kind"], its[j]["crate"], its[j]["slug"], its[j]["name"], its[j]["kind"]] for i, j in up], open(GEN + "/ri2-upward1.json", "w"), ensure_ascii=False, indent=0)
