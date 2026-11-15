"""📐 Read-only audit of staffage asset changes against HEAD: thermal areaM2 classified by ruling C16, other thermal, representation and rig changes."""
import json
import math
import pathlib
import subprocess

TICKET = pathlib.Path(__file__).resolve().parent
GEN = TICKET / "🗑️generated" / "areaaudit"
FORBIDDEN_KINDS = {"foliage", "fur", "feather", "hair"}
FACTOR = 1.5
TOLERANCE = 1e-9
ROOT = pathlib.Path(
    subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=TICKET, capture_output=True, text=True, check=True).stdout.strip()
)
CLASS_NAMES = {
    "a": "FORBIDDEN (leaf model or foliage/fur/feather/hair kind)",
    "b": "TOO LARGE (ratio outside [1/1.5, 1.5])",
    "c": "WITHIN FACTOR 1.5",
}


def git_lines(*args):
    out = subprocess.run(["git", "-c", "core.quotepath=off", *args], cwd=ROOT, capture_output=True, check=True).stdout
    return out.decode("utf-8").splitlines()


def is_asset(path):
    parts = path.split("/")
    return len(parts) >= 5 and parts[-5].endswith("staffage") and parts[-4].endswith("catalog") and parts[-1].endswith(".json")


def asset_paths():
    names = set(git_lines("ls-files")) | set(git_lines("ls-files", "--others", "--exclude-standard"))
    return sorted(p for p in names if is_asset(p))


def load_head(path):
    res = subprocess.run(["git", "-c", "core.quotepath=off", "show", "HEAD:" + path], cwd=ROOT, capture_output=True)
    return json.loads(res.stdout.decode("utf-8")) if res.returncode == 0 else None


def load_tree(path):
    f = ROOT / path
    return json.loads(f.read_text(encoding="utf-8")) if f.is_file() else None


def short(value):
    text = json.dumps(value, ensure_ascii=False)
    return text if len(text) <= 80 else text[:77] + "..."


def keyed(lst):
    return all(isinstance(x, dict) and "id" in x for x in lst)


def diff(a, b, path, out):
    if isinstance(a, list) and isinstance(b, list) and keyed(a) and keyed(b):
        a = {x["id"]: x for x in a}
        b = {x["id"]: x for x in b}
        sep = lambda k: f"{path}[{k}]"
    elif isinstance(a, dict) and isinstance(b, dict):
        sep = lambda k: f"{path}.{k}" if path else str(k)
    elif isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            out.append({"path": path, "change": "length", "old": len(a), "new": len(b)})
        for i, (x, y) in enumerate(zip(a, b)):
            diff(x, y, f"{path}[{i}]", out)
        return
    else:
        if a != b:
            out.append({"path": path, "change": "changed", "old": short(a), "new": short(b)})
        return
    for k in sorted(set(a) | set(b), key=str):
        p = sep(k)
        if k not in a:
            out.append({"path": p, "change": "added", "new": short(b[k])})
        elif k not in b:
            out.append({"path": p, "change": "removed", "old": short(a[k])})
        else:
            diff(a[k], b[k], p, out)


def region_map(thermal):
    return {r["id"]: r for r in thermal.get("regions", [])}


def without(d, *keys):
    return {k: v for k, v in d.items() if k not in keys}


def area_record(old_r, new_r, rid):
    ov, nv = old_r.get("areaM2"), new_r.get("areaM2")
    kind, model = new_r.get("kind"), new_r.get("model")
    numeric = isinstance(ov, (int, float)) and isinstance(nv, (int, float))
    ratio = nv / ov if numeric and ov > 0 and nv > 0 else None
    if model == "leaf" or kind in FORBIDDEN_KINDS:
        cls = "a"
    elif ratio is None or not (1 / FACTOR - TOLERANCE <= ratio <= FACTOR + TOLERANCE):
        cls = "b"
    else:
        cls = "c"
    return {
        "region": rid,
        "kind": kind,
        "model": model,
        "old_kind": old_r.get("kind"),
        "old_model": old_r.get("model"),
        "old": ov,
        "new": nv,
        "ratio": round(ratio, 6) if ratio is not None else None,
        "log_ratio": round(math.log(ratio), 6) if ratio is not None else None,
        "class": cls,
    }


def thermal_changes(old_t, new_t):
    areas, other = [], []
    diff(without(old_t, "regions"), without(new_t, "regions"), "thermal", other)
    old_r, new_r = region_map(old_t), region_map(new_t)
    for rid in sorted(set(old_r) | set(new_r)):
        base = f"thermal.regions[{rid}]"
        if rid not in old_r:
            other.append({"path": base, "change": "added"})
        elif rid not in new_r:
            other.append({"path": base, "change": "removed"})
        else:
            if old_r[rid].get("areaM2") != new_r[rid].get("areaM2"):
                areas.append(area_record(old_r[rid], new_r[rid], rid))
            diff(without(old_r[rid], "areaM2"), without(new_r[rid], "areaM2"), base, other)
    return areas, other


def shape_count(a, b):
    index = lambda lst: {(s.get("id", i) if isinstance(s, dict) else i): s for i, s in enumerate(lst)}
    A, B = index(a), index(b)
    return sum(1 for k in set(A) | set(B) if A.get(k) != B.get(k))


def repr_changes(old, new):
    out = {}
    for scale in sorted(set(old) | set(new), key=str):
        o, n = old.get(scale, {}), new.get(scale, {})
        og, ng = o.get("geometry", {}), n.get("geometry", {})
        viewpoints = {}
        for vp in sorted(set(og) | set(ng), key=str):
            count = shape_count(og.get(vp, []), ng.get(vp, []))
            if count:
                viewpoints[vp] = count
        keys = sorted(k for k in set(o) | set(n) if k != "geometry" and o.get(k) != n.get(k))
        scale_change = None if scale in old and scale in new else ("added" if scale not in old else "removed")
        if viewpoints or keys or scale_change:
            out[scale] = {"scale_change": scale_change, "viewpoints": viewpoints, "keys": keys}
    return out


def top_level_counts(old_doc, new_doc):
    changes = []
    strip = lambda doc: {**doc, "asset": without(doc["asset"], "thermal", "representations", "rig")}
    diff(strip(old_doc), strip(new_doc), "", changes)
    counts = {}
    for c in changes:
        key = ".".join(c["path"].split(".")[:2]) if c["path"] else "(root)"
        counts[key] = counts.get(key, 0) + 1
    return counts


def build_record(path, old_doc, new_doc):
    parts = path.split("/")
    cat, slug = parts[-3], parts[-2]
    old_a, new_a = old_doc["asset"], new_doc["asset"]
    old_t, new_t = old_a.get("thermal", {}), new_a.get("thermal", {})
    areas, th_other = thermal_changes(old_t, new_t)
    for a in areas:
        a.update(category=cat, slug=slug, path=path)
    rig = []
    diff(old_a.get("rig"), new_a.get("rig"), "rig", rig)
    return {
        "path": path,
        "category": cat,
        "slug": slug,
        "asset_id": new_a.get("id"),
        "regions_total": len(new_t.get("regions", [])),
        "areas": areas,
        "thermal_other": th_other,
        "representations": repr_changes(old_a.get("representations", {}), new_a.get("representations", {})),
        "rig": rig,
        "top_level": top_level_counts(old_doc, new_doc),
    }


def fmt(v):
    if v is None:
        return "n/a"
    return f"{v:.4g}" if isinstance(v, float) else str(v)


def ratio_cell(r):
    return "n/a (zero old or new)" if r["ratio"] is None else f"{r['ratio']:.4f}"


def area_table(rows):
    lines = ["| # | Category | Slug | Region | Kind | Model | Old m2 | New m2 | Ratio new/old |", "|---|---|---|---|---|---|---|---|---|"]
    for i, r in enumerate(rows, 1):
        lines.append(f"| {i} | {r['category']} | {r['slug']} | {r['region']} | {r['kind']} | {r['model']} | {fmt(r['old'])} | {fmt(r['new'])} | {ratio_cell(r)} |")
    return lines if rows else lines + ["| - | none | | | | | | | |"]


def sort_key(r):
    return -abs(r["log_ratio"]) if r["log_ratio"] is not None else -math.inf


def totals(records):
    table = {}
    for rec in records:
        t = table.setdefault(rec["category"], {"assets": 0, "regions": 0, "a": 0, "b": 0, "c": 0, "thermal_other": 0, "shapes": 0, "rig": 0, "top_level": 0})
        t["assets"] += 1
        t["regions"] += rec["regions_total"]
        for a in rec["areas"]:
            t[a["class"]] += 1
        t["thermal_other"] += len(rec["thermal_other"])
        t["shapes"] += sum(sum(s["viewpoints"].values()) for s in rec["representations"].values())
        t["rig"] += len(rec["rig"])
        t["top_level"] += sum(rec["top_level"].values())
    return dict(sorted(table.items()))


def write_markdown(path, records, notes, tot, flat):
    by_class = {c: sorted([r for r in flat if r["class"] == c], key=sort_key) for c in "abc"}
    lines = [
        "# Area audit: thermal areaM2 and other asset changes against HEAD",
        "",
        "Generated by `areaaudit.py` (read-only; `git show HEAD:<path>` against the working tree). Ruling: C16 in `design.md`.",
        "",
        f"Assets audited: {len(records)}. Regions with changed areaM2: {len(flat)} "
        f"(a: {len(by_class['a'])}, b: {len(by_class['b'])}, c: {len(by_class['c'])}). Machine-readable: `area-audit.json`.",
        "",
        "## Totals per category",
        "",
        "| Category | Assets | Regions | areaM2 changed | (a) forbidden | (b) too large | (c) within 1.5 | Other thermal changes | Shapes changed (all scales) | Rig changes | Top-level changes |",
        "|---|---|---|---|---|---|---|---|---|---|---|",
    ]
    for cat, t in tot.items():
        changed = t["a"] + t["b"] + t["c"]
        lines.append(f"| {cat} | {t['assets']} | {t['regions']} | {changed} | {t['a']} | {t['b']} | {t['c']} | {t['thermal_other']} | {t['shapes']} | {t['rig']} | {t['top_level']} |")
    for c, title in [("a", "Table A: FORBIDDEN (C16 leaf/foliage/fur/feather/hair; areaM2 must not change)"),
                     ("b", "Table B: TOO LARGE (ratio outside [1/1.5, 1.5]; must not be applied, list in area-review.md)"),
                     ("c", "Table C: WITHIN FACTOR 1.5")]:
        lines += ["", f"## {title}", "", f"Sorted by |log ratio|, most extreme first.", ""] + area_table(by_class[c])
    lines += ["", "## Other thermal changes (excluding areaM2)", "", "| Asset | Path | Change | Old | New |", "|---|---|---|---|---|"]
    other_rows = [(r["category"] + "/" + r["slug"], o) for r in records for o in r["thermal_other"]]
    for asset, o in other_rows:
        lines.append(f"| {asset} | {o['path']} | {o['change']} | {o.get('old', '')} | {o.get('new', '')} |")
    if not other_rows:
        lines.append("| none | | | | |")
    lines += ["", "## Representation changes (shape counts per scale and viewpoint)", "", "| Asset | Scale | Viewpoint | Shapes changed | Other keys changed |", "|---|---|---|---|---|"]
    rep_rows = 0
    for r in records:
        for scale, info in r["representations"].items():
            vps = info["viewpoints"] or {"-": 0}
            for vp, cnt in vps.items():
                rep_rows += 1
                lines.append(f"| {r['category']}/{r['slug']} | {scale} | {vp} | {cnt} | {', '.join(info['keys']) + (' (scale ' + info['scale_change'] + ')' if info['scale_change'] else '')} |")
    if not rep_rows:
        lines.append("| none | | | | |")
    lines += ["", "## Rig changes", "", "| Asset | Changes | First paths |", "|---|---|---|"]
    rig_rows = [r for r in records if r["rig"]]
    for r in rig_rows:
        first = "; ".join(c["path"] for c in r["rig"][:5])
        lines.append(f"| {r['category']}/{r['slug']} | {len(r['rig'])} | {first} |")
    if not rig_rows:
        lines.append("| none | | |")
    lines += ["", "## Top-level changes outside thermal, representations and rig", "", "| Asset | Top-level key | Changes |", "|---|---|---|"]
    top_rows = [(r, k, n) for r in records for k, n in r["top_level"].items()]
    for r, k, n in top_rows:
        lines.append(f"| {r['category']}/{r['slug']} | {k} | {n} |")
    if not top_rows:
        lines.append("| none | | |")
    if notes:
        lines += ["", "## Notes", ""] + [f"- {n}" for n in notes]
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main():
    records, notes = [], []
    for p in asset_paths():
        old_doc, new_doc = load_head(p), load_tree(p)
        if new_doc is None:
            notes.append(f"{p}: missing in working tree")
            continue
        if old_doc is None:
            notes.append(f"{p}: not in HEAD (new file), not diffed")
            continue
        records.append(build_record(p, old_doc, new_doc))
    flat = sorted((a for r in records for a in r["areas"]), key=lambda a: (a["class"], sort_key(a)))
    tot = totals(records)
    GEN.mkdir(parents=True, exist_ok=True)
    (GEN / "run.log").write_text(f"assets={len(records)} areas={len(flat)} notes={len(notes)}\n" + "\n".join(notes) + "\n", encoding="utf-8")
    payload = {
        "rule": "C16",
        "factor": FACTOR,
        "classes": CLASS_NAMES,
        "assets_audited": len(records),
        "areas": flat,
        "totals": tot,
        "assets": records,
        "notes": notes,
    }
    (TICKET / "area-audit.json").write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    write_markdown(TICKET / "area-audit.md", records, notes, tot, flat)
    print(f"assets={len(records)} areas={len(flat)} a={sum(a['class']=='a' for a in flat)} b={sum(a['class']=='b' for a in flat)} c={sum(a['class']=='c' for a in flat)}")


if __name__ == "__main__":
    main()
