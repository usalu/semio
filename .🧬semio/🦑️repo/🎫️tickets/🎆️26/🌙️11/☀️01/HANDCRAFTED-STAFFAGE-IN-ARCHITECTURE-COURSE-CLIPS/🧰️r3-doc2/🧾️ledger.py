import datetime
import json
import os
import sys

CATALOG = sys.argv[1]
CAST = sys.argv[2]
OUT = sys.argv[3]

CATEGORIES = ["humans", "animals", "vegetation", "furniture", "appliances", "objects", "vehicles", "machinery", "components"]
SCALES = ["50", "100", "200", "500"]
VIEWS = ["elevation", "front", "plan"]
VIEW_LABEL = {"plan": "top/plan", "front": "front", "elevation": "side/elev"}


def load(path):
    with open(path, encoding="utf-8") as fh:
        return json.load(fh)


def cell(s):
    return str(s).replace("|", "/").replace("\n", " ")


def find_category_folders(root):
    found = {}
    for cat_dir in sorted(os.listdir(root)):
        full = os.path.join(root, cat_dir)
        if not os.path.isdir(full):
            continue
        for asset_dir in sorted(os.listdir(full)):
            f = os.path.join(full, asset_dir, "🔣️.json")
            if os.path.isfile(f):
                found.setdefault(cat_dir, []).append(f)
    return found


casts = load(CAST)["casts"]
cats = find_category_folders(CATALOG)
ORDER = {c: i for i, c in enumerate(CATEGORIES)}
rows = []
gaps = {"view": [], "scale-missing": [], "scale-unavailable": [], "thermal": [], "shape-empty": [], "schema": [], "material-ref": []}
per_scale_unavail = {s: [] for s in SCALES}
cast_by_asset = {}
for cid, c in casts.items():
    cast_by_asset.setdefault(c.get("asset"), []).append(cid)
catalog_ids = set()
file_count = 0
latest_mtime = 0

for cat_dir in sorted(cats):
    for f in cats[cat_dir]:
        file_count += 1
        latest_mtime = max(latest_mtime, os.path.getmtime(f))
        doc = load(f)
        a = doc["asset"]
        aid = a["id"]
        catalog_ids.add(aid)
        catname = a["category"]
        ref = f"{catname}/{aid}"
        if not os.path.basename(os.path.dirname(os.path.dirname(f))).endswith(catname):
            gaps["schema"].append(f"{ref}: folder {os.path.basename(os.path.dirname(os.path.dirname(f)))} does not match category")
        for k in ["$schema", "version", "asset"]:
            if k not in doc:
                gaps["schema"].append(f"{ref}: missing document key {k}")
        reps = a.get("representations", {})
        view_cells = []
        scale_cells = []
        views_authored = {v: 0 for v in VIEWS}
        n_auth = 0
        for s in SCALES:
            r = reps.get(s)
            if r is None:
                scale_cells.append(f"{s} MISSING")
                gaps["scale-missing"].append(f"{ref}: scale {s} absent")
                continue
            if r["status"] == "unavailable":
                scale_cells.append(f"{s} U({r['cause']})")
                per_scale_unavail[s].append(ref)
                gaps["scale-unavailable"].append(f"{ref}: scale {s} unavailable ({r['cause']}; limiting {r.get('limitingFeature', {}).get('en', '-')}, {r.get('limitingSizeM')} m)")
                continue
            n_auth += 1
            scale_cells.append(f"{s} A")
            for v in VIEWS:
                shapes = r["geometry"].get(v, [])
                if shapes:
                    views_authored[v] += 1
                else:
                    gaps["view"].append(f"{ref}: scale {s} lacks {VIEW_LABEL[v]}")
        view_cells = [f"{VIEW_LABEL[v]} {views_authored[v]}/{n_auth}" for v in VIEWS]
        if n_auth == 0:
            gaps["view"].append(f"{ref}: no authored scale")

        th = a.get("thermal", {})
        regions = th.get("regions", [])
        emitters = th.get("emitters", [])
        passengers = th.get("passengers", [])
        mats = a.get("materials", [])
        mat_ids = {m["id"] for m in mats}
        unresolved = [r["id"] for r in regions if r.get("material") not in mat_ids]
        if unresolved:
            gaps["material-ref"].append(f"{ref}: regions with unresolved material {unresolved}")
        if not regions or not mats:
            gaps["thermal"].append(f"{ref}: regions={len(regions)} materials={len(mats)}")
        emit_cells = [f"{e['id']} ({e['kind']}, {e.get('source', '-')})" for e in emitters]
        thermal_cell = f"R{len(regions)} M{len(mats)} E{len(emitters)} P{len(passengers)}"
        if unresolved:
            thermal_cell += f" unresolved:{len(unresolved)}"
        poses = [p["id"] for p in a.get("poses", [])]
        clips = [c["id"] for c in a.get("clips", [])]
        mass = a["dimensions"]["massKg"]
        cast_ids = sorted(cast_by_asset.get(aid, []))
        if not cast_ids:
            cast_ids_cell = "-"
        else:
            cast_ids_cell = ", ".join(cast_ids)
        rows.append({
            "cat": catname, "id": aid, "en": a["title"]["en"], "de": a["title"]["de"],
            "views": "; ".join(view_cells), "thermal": thermal_cell,
            "mats": ", ".join(m["id"] for m in mats) if mats else "-",
            "scales": ", ".join(scale_cells), "poses": ", ".join(poses) or "-",
            "clips": ", ".join(clips) or "-", "emit": "; ".join(emit_cells) or "-",
            "mass": mass, "casts": cast_ids_cell, "completion": a.get("completion", "-"),
            "n_auth": n_auth,
            "gap": ", ".join(f"U@1:{s}" for s in SCALES if reps.get(s, {}).get("status") == "unavailable") or "-",
        })

rows.sort(key=lambda r: (ORDER[r["cat"]], r["id"]))
unknown_casts = [f"{cid}: asset {c.get('asset')}" for cid, c in casts.items() if c.get("asset") not in catalog_ids]
gaps_casts_none = sorted({r["cat"] + "/" + r["id"] for r in rows if r["casts"] == "-"})

stamp = datetime.datetime.now().astimezone().strftime("%Y-%m-%d %H:%M %z")
mtime = datetime.datetime.fromtimestamp(latest_mtime).astimezone().strftime("%Y-%m-%d %H:%M %z")

out = []
w = out.append
w("# 🧾️ Round 3 Asset Ledger (provenance registry)")
w("")
w(f"- Snapshot date: **2026-11-03**, generated **{stamp}** from the catalog as it stood then. Other workers are adding assets at the same time; this is a snapshot, not a live view. Newest asset JSON mtime at generation: {mtime}.")
w(f"- Scope: {file_count} asset documents in `🗂️catalog/<category>/<id>/🔣️.json` across nine categories. The root manifest, `🌤️environments` and `🖨️print-profiles` are not assets and are excluded.")
w("- Schema: `🧬️schema/🔣️.json` (`Asset`, `Representation`, `AuthoredRepresentation`, `UnavailableRepresentation`, `Emitter`, `ThermalModel`, `Dimensions`). Field names are used as declared there.")
w("- Cast map: `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json`, matched by its `asset` field.")
w("- Generator: `🧰️r3-doc2/🧾️ledger.py`, run with `python -I` against the catalog. Raw tool output: `🗑️generated/r3-doc2/`.")
w("")
w("## Legend")
w("")
w("| Token | Meaning |")
w("|---|---|")
w("| `top/plan`, `front`, `side/elev` `k/n` | `k` authored scales, out of `n` authored, whose `geometry.plan`, `geometry.front` or `geometry.elevation` has at least one shape. Complete means `k` equals `n`. |")
w("| Scale `50 A` | Authored representation (`status: authored`). |")
w("| Scale `200 U(cause)` | Unavailable representation (`status: unavailable`). An explicit decision, not a missing key. Flagged as a gap for review. |")
w("| Scale `MISSING` | Key absent from `representations`. A schema violation. |")
w("| `R` / `M` / `E` / `P` | Thermal regions / asset materials / thermal emitters / passengers. |")
w("| Emitter `(kind, source)` | `ThermalModel.emitters[]` entry: its `kind` and its `source` (`ParameterSource`). |")
w("| Transport | The asset schema has no transport source field. Transport appears only in the manifest `analytics` enum and in prose, so the column is `-` for every asset. |")
w("| `-` | Nothing declared. |")
w("")
w("## Gap summary")
w("")
w(f"- Assets: {len(rows)}. Authored at all four scales: {sum(1 for r in rows if r['n_auth'] == 4)}. Assets with at least one unavailable scale: {sum(1 for r in rows if r['n_auth'] < 4)}.")
for s in SCALES:
    w(f"- Unavailable at scale 1:{s}: {len(per_scale_unavail[s])}")
w(f"- Missing or incomplete view (an authored scale lacking elevation, front or plan): {len(gaps['view'])} occurrences in {len({g.split(':')[0] for g in gaps['view']})} assets.")
w(f"- Missing scale key: {len(gaps['scale-missing'])}. Schema violations: {len(gaps['schema'])}.")
w(f"- Thermal capability gap (no regions or no materials): {len(gaps['thermal'])}.")
w(f"- Regions whose `material` does not resolve to an asset material: {len(gaps['material-ref'])}.")
w(f"- Assets with no cast in the cast manifest: {len(gaps_casts_none)}.")
w(f"- Cast entries whose `asset` is not in the catalog: {len(unknown_casts)}.")
w("")
w("### Gap details")
w("")
for label, key in [("Missing view", "view"), ("Missing scale key", "scale-missing"), ("Unavailable scale", "scale-unavailable"), ("Thermal capability", "thermal"), ("Unresolved material", "material-ref"), ("Schema", "schema")]:
    w(f"**{label}**")
    if gaps[key]:
        for g in gaps[key]:
            w(f"- {g}")
    else:
        w("- none")
    w("")
w("**Assets with no cast**")
w("- " + (", ".join(gaps_casts_none) if gaps_casts_none else "none"))
w("")
w("**Cast entries with unknown asset**")
w("- " + ("; ".join(unknown_casts) if unknown_casts else "none"))
w("")
w("## Ledger")
cur = None
for r in rows:
    if r["cat"] != cur:
        cur = r["cat"]
        w("")
        w(f"### {cur}")
        w("")
        w("| category/id | en name | de name | views (top/plan, front, side/elev) | thermal | materials | scales | gap | poses | clips | emitters | transport | massKg | cast ids |")
        w("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    w(f"| {cell(r['cat'])}/{cell(r['id'])} | {cell(r['en'])} | {cell(r['de'])} | {cell(r['views'])} | {cell(r['thermal'])} | {cell(r['mats'])} | {cell(r['scales'])} | {cell(r['gap'])} | {cell(r['poses'])} | {cell(r['clips'])} | {cell(r['emit'])} | - | {r['mass']} | {cell(r['casts'])} |")
w("")

with open(OUT, "w", encoding="utf-8", newline="\n") as fh:
    fh.write("\n".join(out))
print("rows", len(rows), "files", file_count, "casts", len(casts), "unknown_casts", len(unknown_casts), "nocast", len(gaps_casts_none))
