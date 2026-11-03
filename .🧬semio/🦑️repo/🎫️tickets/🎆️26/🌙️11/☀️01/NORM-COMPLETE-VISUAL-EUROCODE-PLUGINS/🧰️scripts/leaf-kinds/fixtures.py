"""Committed mutation vectors (before, mutation, after, outcome, diff) of generated kinds, and diff padding of existing vectors."""
import copy, glob, json, os
from common import *


def rows_at(doc, chain, ids):
    cur = doc
    for i, (coll, _, sel) in enumerate(chain):
        cur = next(r for r in cur[camel(coll)] if r["id"] == ids[i])
    return cur


def pick_ids(doc, chain):
    ids, cur = [], doc
    for coll, _, sel in chain:
        row = cur[camel(coll)][0]
        ids.append(row["id"])
        cur = row
    return ids


def new_value(doc, leaf, old, defs):
    if leaf.ty == "f64":
        return round(old * 2.0, 9) if old else 1.5
    if leaf.ty == "u32":
        return old + 1
    if leaf.ty == "bool":
        return not old
    if leaf.ty.startswith("enum:"):
        vals = defs[leaf.rust_ty]["enum"]
        return vals[(vals.index(old) + 1) % len(vals)]
    if leaf.field.endswith("_id"):
        coll = {"member_id": "members", "material_id": "materials", "section_id": "sections"}.get(leaf.field)
        if coll:
            alt = [r["id"] for r in doc.get(camel(coll), []) if r["id"] != old]
            if alt:
                return alt[0]
    return old + "-alt" if old else "alt"


def sparse_diff(leaf, ids, value, spec):
    """Full-null patch lanes with the one leaf set, nested down the chain."""
    ent = leaf.ent
    empty = lambda: {"removed": [], "inserted": [], "moved": [], "modified": []}
    inner = {camel(leaf.field): value}
    for depth in range(len(ent.chain) - 1, -1, -1):
        name = "".join(pascal(c[0]) for c in ent.chain[: depth + 1])
        patch = pad_patch(inner, spec[name], spec)
        rows = empty()
        rows["modified"] = [{"id": ids[depth], "patch": patch}]
        if depth:
            inner = {camel(ent.chain[depth][0]): rows}
        else:
            return rows


def pad_patch(patch, s, spec):
    out = {}
    for l in s.fields:
        out[camel(l.field)] = patch.get(camel(l.field))
    for n, r in s.lanes:
        lane = patch.get(camel(n))
        child = spec[r[len(spec_prefix(spec)): -len("Rows")]]
        out[camel(n)] = pad_rows(lane, child, spec) if lane else None
    return out


def spec_prefix(spec):
    return next(iter(spec.values())).P


def pad_rows(rows, s, spec):
    rows = copy.deepcopy(rows)
    rows["modified"] = [{"id": m["id"], "patch": pad_patch(m["patch"], s, spec)} for m in rows["modified"]]
    return rows


def pad_diff(diff, spec, top):
    out = {}
    for k, v in diff.items():
        name = pascal(k)
        out[k] = pad_rows(v, spec[name], spec) if isinstance(v, dict) and name in spec and v.get("modified") else v
    return out


def pad_existing(F, art, spec):
    for p in glob.glob(art.fix + "/*/*/🔣️.diff.json"):
        d = json.loads(F.read(p))
        F.write(p, dumps(pad_diff(d, spec, None)))


def build(art, F, leaves, defs, base_doc, spec, top_keys):
    for l in leaves:
        ent = l.ent
        ids = pick_ids(base_doc, ent.chain)
        row = rows_at(base_doc, ent.chain, ids)
        cur = row
        for part in l.path:
            cur = cur[camel(part)]
        val = new_value(base_doc, l, cur, defs)
        after = copy.deepcopy(base_doc)
        ar = rows_at(after, ent.chain, ids)
        for part in l.path[:-1]:
            ar = ar[camel(part)]
        ar[camel(l.path[-1])] = val
        mut = {"mutation": l.record[:1].lower() + l.record[1:]}
        for sel, i in zip(ent.sels, ids):
            mut[camel(sel)] = i
        mut[l.newc] = val
        sd = sparse_diff(l, ids, val, spec)
        diff = {k: None for k in top_keys}
        diff[camel(ent.chain[0][0])] = sd
        base = art.fix + "/" + l.dirname + "/✅apply"
        F.write(base + "/🔣️.before.json", dumps(base_doc))
        F.write(base + "/🔣️.mutation.json", dumps(mut))
        F.write(base + "/🔣️.after.json", dumps(after))
        F.write(base + "/🔣️.outcome.json", dumps({"status": "applied"}))
        F.write(base + "/🔣️.diff.json", dumps(diff))
