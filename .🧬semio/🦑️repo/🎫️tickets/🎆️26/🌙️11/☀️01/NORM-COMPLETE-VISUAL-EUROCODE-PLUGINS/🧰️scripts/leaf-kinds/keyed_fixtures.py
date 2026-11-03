"""Committed `✅apply` vectors (before, mutation, after, outcome, diff) of `index_selector` kinds, and null-padding of existing diff vectors. `spec` maps a patch stem (`MembersBars`) to ([camel field names], [(camel lane, child stem)])."""
import copy, glob, json, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import index_selector as I


def row_at(doc, links, picks):
    cur, rows = doc, []
    for l, i in zip(links, picks):
        cur = cur[camel(l.coll)][i]
        rows.append(cur)
    return rows


def picks_for(doc, links):
    cur, out = doc, []
    for l in links:
        out.append(0)
        cur = cur[camel(l.coll)][0]
    return out


def sel_value(l, row, i):
    return i if I.is_index(l.sel) else row[l.key]


def new_value(leaf, old, defs, doc):
    if leaf.ty == "f64":
        return round(old * 2.0, 9) if old else 1.5
    if leaf.ty in ("u32", "u8"):
        return old + 1
    if leaf.ty == "bool":
        return not old
    if leaf.ty.startswith("enum:"):
        vals = defs[leaf.rust_ty]["enum"]
        return vals[(vals.index(old) + 1) % len(vals)]
    return (old + "-alt") if old else "alt"


def empty_rows():
    return {"removed": [], "inserted": [], "moved": [], "modified": []}


def pad(stem, values, spec):
    fields, lanes = spec[stem]
    out = {f: values.get(f) for f in fields}
    for lane, child in lanes:
        out[lane] = values.get(lane)
    return out


def stem_of(st, links):
    return st.stem(links)


def modified(st, links, picks_rows, spec, inner_values):
    """rows lane of the LAST link with one modified entry; wrapped into the parents' modifications (lanes mode) down to the diff root."""
    keys = [r[l.key] for l, r in zip(links, picks_rows)]
    rows = empty_rows()
    rows["modified"] = [{"id": keys[-1], "patch": pad(stem_of(st, links), inner_values, spec)}]
    for i in range(len(links) - 2, -1, -1):
        sub = links[: i + 1]
        r = empty_rows()
        r["modified"] = [{"id": keys[i], "patch": pad(stem_of(st, sub), {camel(links[i + 1].coll): rows}, spec)}]
        rows = r
    return rows


def lane_edit(st, parent_links, parent_rows, last_link, spec, edit):
    """`edit` fills the rows lane of `last_link` ({inserted: [...]} or {removed: [...]}) under the parents."""
    rows = empty_rows()
    rows.update(edit)
    for i in range(len(parent_links) - 1, -1, -1):
        sub = parent_links[: i + 1]
        lane = camel((parent_links + [last_link])[i + 1].coll)
        r = empty_rows()
        r["modified"] = [{"id": parent_rows[i][parent_links[i].key], "patch": pad(stem_of(st, sub), {lane: rows}, spec)}]
        rows = r
    return rows


def mutation_json(art, k, fields):
    if art.external:
        return {k.record: fields}
    return dict([("mutation", k.record[:1].lower() + k.record[1:])] + list(fields.items()))


def build(art, F, kinds, defs, base, spec, top_keys, new_row):
    st = art.style
    for k in kinds:
        links = k.ent.links
        after = copy.deepcopy(base)
        diff = copy.deepcopy(top_keys) if isinstance(top_keys, dict) else {key: None for key in top_keys}
        if isinstance(k, I.TopKind):
            l = k.leaf
            val = new_value(l, base[camel(l.field)], defs, base)
            after[camel(l.field)] = val
            fields = {l.newc: val}
            diff[camel(l.field)] = val
            write(art, F, k, base, after, fields, diff)
            continue
        if getattr(k, "effect", False):
            l = k.leaf
            row = copy.deepcopy(base["effects"][0])
            val = new_value(l, row[camel(l.field)], defs, base)
            row[camel(l.field)] = val
            after["effects"][0] = row
            diff["effects"] = [{"op": "Replace", "index": 0, "id": "", "value": row}]
            write(art, F, k, base, after, {"index": 0, l.newc: val}, diff)
            continue
        if isinstance(k, I.SetKind):
            l = k.leaf
            picks = picks_for(base, links)
            rows = row_at(base, links, picks)
            tgt = rows[-1]
            cur = tgt
            for part in l.path:
                cur = cur[camel(part)]
            val = new_value(l, cur, defs, base)
            arows = row_at(after, links, picks)[-1]
            for part in l.path[:-1]:
                arows = arows[camel(part)]
            arows[camel(l.path[-1])] = val
            fields = {camel(s): sel_value(lk, r, i) for s, lk, r, i in zip(k.ent.sels, links, rows, picks)}
            fields[l.newc] = val
            top = modified(st, links, rows, spec, {camel(l.field): val})
        elif isinstance(k, I.InsertKind):
            parents = links[:-1]
            picks = picks_for(base, parents)
            prow = row_at(base, parents, picks)
            holder = prow[-1] if prow else base
            coll = holder[camel(links[-1].coll)]
            row = new_row(k, coll, base)
            at = min(1, len(coll))
            ah = row_at(after, parents, picks)[-1] if parents else after
            ah[camel(links[-1].coll)].insert(at, copy.deepcopy(row))
            fields = {camel(lk.sel): sel_value(lk, r, i) for lk, r, i in zip(parents, prow, picks)}
            fields["index"] = at
            fields[k.item_c] = row
            top = lane_edit(st, parents, prow, links[-1], spec, {"inserted": [{"index": at, "row": row}]})
        else:
            parents = links[:-1]
            picks = picks_for(base, parents)
            prow = row_at(base, parents, picks)
            holder = prow[-1] if prow else base
            coll = holder[camel(links[-1].coll)]
            at = 0
            ah = row_at(after, parents, picks)[-1] if parents else after
            gone = ah[camel(links[-1].coll)].pop(at)
            fields = {camel(lk.sel): sel_value(lk, r, i) for lk, r, i in zip(parents, prow, picks)}
            fields["index"] = at
            top = lane_edit(st, parents, prow, links[-1], spec, {"removed": [{"id": gone[links[-1].key], "index": at}]})
        diff[camel(links[0].coll)] = top
        write(art, F, k, base, after, fields, diff)


def write(art, F, k, base, after, fields, diff):
    d = art.fix + "/" + k.dirname + "/✅apply"
    F.write(d + "/🔣️.before.json", dumps(base))
    F.write(d + "/🔣️.mutation.json", dumps(mutation_json(art, k, fields)))
    F.write(d + "/🔣️.after.json", dumps(after))
    F.write(d + "/🔣️.outcome.json", dumps({"status": "applied"}))
    F.write(d + "/🔣️.diff.json", dumps(diff))


def pad_rows(rows, stem, spec, pfx):
    rows = copy.deepcopy(rows)
    rows["modified"] = [{"id": m["id"], "patch": pad_patch(m["patch"], stem, spec, pfx)} for m in rows["modified"]]
    return rows


def pad_patch(patch, stem, spec, pfx):
    fields, lanes = spec[stem]
    out = {f: patch.get(f) for f in fields}
    for lane, child in lanes:
        v = patch.get(lane)
        out[lane] = pad_rows(v, child, spec, pfx) if v else None
    return out


def pad_existing(F, art, spec, pfx):
    for p in glob.glob(art.fix + "/*/*/🔣️.diff.json"):
        d = json.loads(F.read(p))
        out = {}
        for key, v in d.items():
            stem = pascal(key)
            out[key] = pad_rows(v, stem, spec, pfx) if isinstance(v, dict) and stem in spec and v.get("modified") else v
        F.write(p, dumps(out))
