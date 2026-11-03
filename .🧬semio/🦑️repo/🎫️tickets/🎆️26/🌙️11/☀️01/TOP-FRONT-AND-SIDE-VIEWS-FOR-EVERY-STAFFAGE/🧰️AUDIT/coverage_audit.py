"""🔎 Coverage audit of the staffage catalog: elevation, front and plan for every authored scale.

Read-only over the catalog. Schema-driven: scales, views, decision fields, categories and the ground role come from
🧬️schema/🔣️.json (semio.staffage/v2). Assets: 🗂️catalog/<category>/<id>/🔣️.json, manifest: 🗂️catalog/🔣️.json.
Writes coverage.json into 🗑️generated/AUDIT and the report audit-catalog-coverage.md into the ticket root.
Run as a file: python <ticket>/🧰️AUDIT/coverage_audit.py
"""

import json
import re
from collections import Counter, defaultdict
from pathlib import Path

TICKET = Path(__file__).resolve().parent.parent
LANGS = ("en", "de")
SCHEMA_ID = "semio.staffage/v2"
GAP_KINDS = ("json", "document", "category", "representation", "status", "geometry", "decisions", "viewpoints", "manifest")


def ascii_key(name):
    return re.sub(r"[^a-z0-9-]", "", name.lower())


def child_dir(parent, key):
    for child in sorted(parent.iterdir()):
        if child.is_dir() and ascii_key(child.name) == key:
            return child
    raise SystemExit(f"no directory '{key}' under {parent}")


def repo_root():
    for folder in [TICKET, *TICKET.parents]:
        if (folder / ".git").exists():
            return folder
    raise SystemExit("repo root with .git not found")


def read_json(path):
    try:
        return json.loads(path.read_text(encoding="utf-8")), None
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        return None, f"{type(error).__name__}: {error}"


def find_key(node, key, path=()):
    if isinstance(node, dict):
        for name, value in node.items():
            if name == key:
                yield path + (name,)
            yield from find_key(value, key, path + (name,))
    elif isinstance(node, list):
        for index, value in enumerate(node):
            yield from find_key(value, key, path + (str(index),))


def load_schema(staffage):
    for path in child_dir(staffage, "schema").glob("*.json"):
        data, error = read_json(path)
        if error is None and isinstance(data, dict) and data.get("$id") == SCHEMA_ID:
            return data
    raise SystemExit("schema semio.staffage/v2 not found")


def view_state(geometry, view, ground):
    if view not in geometry:
        return "missing"
    shapes = geometry[view]
    if not isinstance(shapes, list) or not shapes:
        return "empty"
    if all(isinstance(shape, dict) and shape.get("role") == ground for shape in shapes):
        return "ground-only"
    return "ok"


def audit_asset(category, folder, doc, manifest_entry, cfg, add, rows):
    asset = doc.get("asset") if isinstance(doc, dict) else None
    if not isinstance(asset, dict):
        add("document", category, folder, "-", "asset object missing")
        return
    if asset.get("id") != folder:
        add("document", category, folder, "-", f"asset id {asset.get('id')!r} differs from folder")
    if asset.get("category") != category:
        add("category", category, folder, "-", f"asset category {asset.get('category')!r} differs from folder")
    if manifest_entry is None:
        add("manifest", category, folder, "-", "manifest entry missing")
    reps = asset.get("representations") if isinstance(asset.get("representations"), dict) else {}
    for scale in cfg["scales"]:
        rep = reps.get(scale)
        status = rep.get("status") if isinstance(rep, dict) else None
        row = {"category": category, "asset": folder, "scale": scale, "status": status,
               **{view: "n/a" for view in cfg["views"]}, "decisions": "n/a", "complete": False}
        rows.append(row)
        if not isinstance(rep, dict):
            add("representation", category, folder, scale, "representation missing")
        elif status == "authored":
            geometry = rep.get("geometry") if isinstance(rep.get("geometry"), dict) else {}
            for view in cfg["views"]:
                row[view] = view_state(geometry, view, cfg["ground"])
                if row[view] != "ok":
                    add("geometry", category, folder, scale, f"{view} geometry {row[view]}")
            texts = rep.get("decisions") if isinstance(rep.get("decisions"), dict) else {}
            missing = 0
            for field in cfg["decisions"]:
                for lang in LANGS:
                    text = texts.get(field)
                    value = text.get(lang) if isinstance(text, dict) else None
                    if not isinstance(value, str) or not value.strip():
                        missing += 1
                        add("decisions", category, folder, scale, f"decisions.{field}.{lang} missing")
            row["decisions"] = "ok" if missing == 0 else "gap"
            row["complete"] = all(row[view] == "ok" for view in cfg["views"]) and missing == 0
        elif status != "unavailable":
            add("status", category, folder, scale, f"status {status!r} is neither authored nor unavailable")
        if manifest_entry is not None:
            listed = (manifest_entry.get("representations") or {}).get(scale)
            if listed != status:
                add("manifest", category, folder, scale, f"manifest lists {listed!r}, asset has {status!r}")


def main():
    root = repo_root()
    staffage = child_dir(child_dir(child_dir(root, "teaching"), "architecture"), "staffage")
    catalog = child_dir(staffage, "catalog")
    defs = load_schema(staffage)["$defs"]
    cfg = {
        "categories": set(defs["CategoryId"]["enum"]),
        "scales": defs["Asset"]["properties"]["representations"]["required"],
        "views": defs["AuthoredRepresentation"]["properties"]["geometry"]["required"],
        "decisions": defs["RepresentationDecisions"]["required"],
        "ground": "ground",
    }
    assert cfg["ground"] in defs["ShapeRole"]["enum"]

    gaps = []

    def add(kind, category, asset, scale, item):
        gaps.append({"kind": kind, "category": category, "asset": asset, "scale": scale, "item": item})

    docs = []
    for path in sorted(catalog.rglob("*.json")):
        parts = path.relative_to(catalog).parts
        doc, error = read_json(path)
        docs.append((parts, path, doc, error))

    manifest = {}
    for parts, _, doc, error in docs:
        if len(parts) == 1 and error is None and isinstance(doc, dict) and isinstance(doc.get("entries"), list):
            manifest = {entry.get("id"): entry for entry in doc["entries"] if isinstance(entry, dict)}

    rows = []
    counts = Counter()
    asset_totals = Counter()
    for parts, path, doc, error in docs:
        label = "/".join(parts)
        if error is not None:
            add("json", ascii_key(parts[0]) if len(parts) > 1 else "-", label, "-", f"JSON parse error {error}")
            counts["parse_errors"] += 1
            continue
        counts["json_files"] += 1
        for hit in find_key(doc, "viewpoints"):
            add("viewpoints", "-", label, "-", f"leftover key {'.'.join(hit)}")
        is_asset = len(parts) == 3 and ascii_key(parts[0]) in cfg["categories"]
        if not is_asset:
            counts["documents"] += 1
            continue
        category = ascii_key(parts[0])
        counts["assets"] += 1
        asset_totals[category] += 1
        audit_asset(category, parts[1], doc, manifest.get(parts[1]), cfg, add, rows)

    for row in rows:
        if row["status"] == "authored":
            counts["authored"] += 1
            counts["complete"] += row["complete"]
        elif row["status"] == "unavailable":
            counts["unavailable"] += 1
    gap_by_kind = Counter(gap["kind"] for gap in gaps)
    gap_by_category = Counter(gap["category"] for gap in gaps)

    out = child_dir(TICKET, "generated") / "AUDIT"
    out.mkdir(parents=True, exist_ok=True)
    (out / "coverage.json").write_text(
        json.dumps({"counts": counts, "rows": rows, "gaps": gaps}, ensure_ascii=False, indent=2), encoding="utf-8")

    per_category = defaultdict(Counter)
    for row in rows:
        stats = per_category[row["category"]]
        stats["representations"] += 1
        stats[row["status"] or "missing"] += 1
        stats["complete"] += row["complete"]
        for view in cfg["views"]:
            stats[f"{view}_ok"] += row[view] == "ok"
    for category in sorted(cfg["categories"]):
        stats = per_category[category]
        stats["assets"] = asset_totals[category]
        stats["parse_errors"] = sum(1 for gap in gaps if gap["kind"] == "json" and gap["category"] == category)
        stats["gaps"] = gap_by_category.get(category, 0)

    lines = [
        "# Staffage catalog coverage audit",
        "",
        "Script: `🧰️AUDIT/coverage_audit.py`. Schema: `🧬️schema/🔣️.json` (semio.staffage/v2). "
        "Data: `🗂️catalog/<category>/<id>/🔣️.json`. Raw rows: `🗑️generated/AUDIT/coverage.json` (deleted after review).",
        "",
        "## Totals",
        "",
        f"- Catalog JSON files read: {counts['json_files']} (asset documents {counts['assets']}, other documents {counts['documents']}), parse errors {counts['parse_errors']}.",
        f"- Scale representations: {len(rows)} (authored {counts['authored']}, unavailable {counts['unavailable']}).",
        f"- Authored representations with elevation, front, plan and both-language decisions complete: {counts['complete']} of {counts['authored']}.",
        f"- Gaps: {len(gaps)}.",
        "",
        "Gaps by kind: " + ", ".join(f"{kind} {gap_by_kind.get(kind, 0)}" for kind in GAP_KINDS) + ".",
        "",
        "## Per category",
        "",
        "| category | assets | reps | authored | unavailable | elevation ok | front ok | plan ok | decisions complete | complete | gaps |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for category in sorted(cfg["categories"]):
        stats = per_category[category]
        decisions_ok = sum(1 for row in rows if row["category"] == category and row["decisions"] == "ok")
        lines.append(
            f"| {category} | {stats['assets']} | {stats['representations']} | {stats['authored']} | {stats['unavailable']} | "
            f"{stats['elevation_ok']} | {stats['front_ok']} | {stats['plan_ok']} | {decisions_ok} | {stats['complete']} | {stats['gaps']} |")
    lines += ["", "## Gap list", "", "| category | asset | scale | kind | missing item |", "|---|---|---:|---|---|"]
    for gap in sorted(gaps, key=lambda g: (g["category"], g["asset"], g["scale"], g["kind"], g["item"])):
        lines.append(f"| {gap['category']} | {gap['asset']} | {gap['scale']} | {gap['kind']} | {gap['item']} |")
    if not gaps:
        lines.append("| - | - | - | - | none |")
    (TICKET / "audit-catalog-coverage.md").write_text("\n".join(lines) + "\n", encoding="utf-8")

    print(f"assets={counts['assets']} authored={counts['authored']} unavailable={counts['unavailable']} "
          f"complete={counts['complete']} parse_errors={counts['parse_errors']} gaps={len(gaps)}")


if __name__ == "__main__":
    main()
