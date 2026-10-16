#!/usr/bin/env python3
# 🧱️ Units of the Shell item graph: a type with its impls (or a free item) as one movable node; edges are the names a unit mentions; ranks are crate ranks (the monolith residue sits above every crate).
import collections, re, sys
from analyze2 import *

TOP = 10**6
TYPE_KINDS = {"struct", "enum", "union", "type", "trait"}


def build():
    ms, its = load()
    for i, it in enumerate(its):
        it["id"] = i
    unit_of = {}
    units = []
    def new_unit(members):
        u = dict(id=len(units), members=members)
        units.append(u)
        for m in members:
            unit_of[m] = u["id"]
        return u
    # type-like and free items first
    byname = collections.defaultdict(list)
    for i, it in enumerate(its):
        if it["kind"] == "impl":
            continue
        if it["kind"] == "mod" and (it["name"].endswith("tests") or it["name"].endswith("_tests") or it["name"] == "tests"):
            continue
        u = new_unit([i])
        byname[it["name"]].append(u["id"])
    # impls attach to the unit of their type (same crate/slug preferred)
    loose = []
    for i, it in enumerate(its):
        if it["kind"] != "impl":
            continue
        t = it.get("impl_type")
        cands = [u for u in byname.get(t, []) if its[units[u]["members"][0]]["kind"] in TYPE_KINDS]
        pick = None
        for u in cands:
            f = its[units[u]["members"][0]]
            if f["crate"] == it["crate"] and f["slug"] == it["slug"]:
                pick = u
                break
        if pick is None:
            for u in cands:
                f = its[units[u]["members"][0]]
                if f["crate"] == it["crate"]:
                    pick = u
                    break
        if pick is None and len(cands) >= 1:
            pick = cands[0]
        if pick is None:
            loose.append(i)
            new_unit([i])
        else:
            units[pick]["members"].append(i)
            unit_of[i] = pick
    defs = collections.defaultdict(set)
    for u in units:
        for m in u["members"]:
            if its[m]["kind"] != "impl":
                defs[its[m]["name"]].add(u["id"])
    edges = collections.defaultdict(set)
    for u in units:
        toks = set()
        for m in u["members"]:
            toks |= tokens(its[m]["body"])
        for t in toks:
            for v in defs.get(t, ()):
                if v != u["id"]:
                    edges[u["id"]].add(v)
    for u in units:
        cs = {its[m]["crate"] for m in u["members"]}
        u["crates"] = cs
        u["crate"] = next(iter(cs)) if len(cs) == 1 else None
        u["mk"] = its[u["members"][0]]["mk"]
    return ms, its, units, edges, defs, loose


def rank_of(c):
    return TOP if c is None else RANK[c]


if __name__ == "__main__":
    ms, its, units, edges, defs, loose = build()
    print("units", len(units), "loose impls", len(loose))
    for i in loose:
        print("  loose", its[i]["crate"], its[i]["slug"], its[i]["name"], its[i]["impl_type"])
    mixed = [u for u in units if u["crate"] is None and u["mk"] == "staging"]
    print("units split over crates:", len(mixed))
    for u in mixed[:30]:
        print("  ", [(its[m]["crate"], its[m]["name"][:30]) for m in u["members"]])
