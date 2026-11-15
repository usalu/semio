#!/usr/bin/env python3 -I
"""📊 Read-only audit of staffage 3D volumes: one CSV row per asset plus ranked lists as JSON and Markdown fragments."""

import csv
import glob
import json
import os
import collections

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
TICKET = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
GEN = os.path.join(TICKET, "🗑️generated", "audit-volume")
SCALES = [50, 100, 200, 500]
ORGANIC = {"animals", "humans", "vegetation"}
KINDS = ["box", "ellipsoid", "capsule", "revolve", "extrude", "loft"]


def visible(solid, scale):
    md = solid.get("maxDenominator")
    return md is None or scale <= md


def analyse(path):
    doc = json.load(open(path, encoding="utf-8"))
    a = doc["asset"]
    cat = a["category"]
    slug = a["id"]
    bones = [b["id"] for b in a["rig"]["bones"]]
    solids = a["volume"]["solids"]
    reps = a["representations"]
    therm = a["thermal"]["regions"]
    therm_ids = [r["id"] for r in therm]

    kinds = collections.Counter(s["kind"] for s in solids)
    total_exp = 0
    box_exp = 0
    for s in solids:
        n = s["repeat"]["count"] if "repeat" in s else 1
        total_exp += n
        if s["kind"] == "box":
            box_exp += n

    md_counts = {v: sum(1 for s in solids if s.get("maxDenominator") == v) for v in (50, 100, 200)}
    vis_s = {v: sum(1 for s in solids if visible(s, v)) for v in SCALES}
    vis_e = {v: sum((s["repeat"]["count"] if "repeat" in s else 1) for s in solids if visible(s, v)) for v in SCALES}
    scale_diff = sum(1 for s in solids if s.get("maxDenominator") in (50, 100, 200))

    materials = {s["material"] for s in solids}
    roles = {s["role"] for s in solids}
    tones = {round(s["tone"], 4) for s in solids}

    covered_bones = set()
    for s in solids:
        covered_bones.add(s["bone"])
        for w in s.get("weights", []):
            covered_bones.add(w["bone"])
    uncovered_bones = [b for b in bones if b not in covered_bones]

    sol_region_refs = collections.defaultdict(int)
    solids_with_region = 0
    solids_undefined_region = 0
    for s in solids:
        rs = []
        if "region" in s:
            rs.append(s["region"])
        rs.extend(s.get("regions", []))
        if rs:
            solids_with_region += 1
        for r in rs:
            sol_region_refs[r] += 1
            if r not in therm_ids:
                solids_undefined_region += 1
    solids_without_region = len(solids) - solids_with_region
    regions_no_solid = [r for r in therm_ids if sol_region_refs.get(r, 0) == 0]

    members50 = 0
    members_all = 0
    member_bones = set()
    status = {str(sc): reps[str(sc)].get("status") for sc in SCALES}
    unavailable = [f"1:{sc} {reps[str(sc)].get('limitingFeature', '')}".strip() for sc in SCALES if status[str(sc)] != "authored"]
    for sc in SCALES:
        if status[str(sc)] != "authored":
            continue
        for part in ("elevation", "plan", "front"):
            for shp in reps[str(sc)]["geometry"][part]:
                if shp.get("kind") == "member":
                    members_all += 1
                    if sc == 50:
                        members50 += 1
                        member_bones.add(shp.get("bone"))
                        member_bones.add(shp.get("from", {}).get("bone") if isinstance(shp.get("from"), dict) else None)
                        member_bones.add(shp.get("to", {}).get("bone") if isinstance(shp.get("to"), dict) else None)
    member_bones.discard(None)

    g50 = reps["50"]["geometry"]
    elev50 = len(g50["elevation"])
    plan50 = len(g50["plan"])
    front50 = len(g50["front"])
    shapes50 = elev50 + plan50 + front50

    decisions = a["volume"]["decisions"]
    dec_chars = sum(len(d.get("en", "")) for d in decisions)

    ratio = total_exp / elev50 if elev50 else 0.0
    box_share = kinds.get("box", 0) / len(solids)
    box_share_exp = box_exp / total_exp if total_exp else 0.0
    organic = 1 if cat in ORGANIC else 0
    zero_diff = 1 if scale_diff == 0 else 0
    crude = round(0.4 * box_share_exp + 0.3 * (1 - min(1.0, ratio)) + 0.2 * organic + 0.1 * zero_diff, 4)

    return {
        "category": cat,
        "slug": slug,
        "bones": len(bones),
        "solids": len(solids),
        "solids_expanded": total_exp,
        "kind_box": kinds.get("box", 0),
        "kind_ellipsoid": kinds.get("ellipsoid", 0),
        "kind_capsule": kinds.get("capsule", 0),
        "kind_revolve": kinds.get("revolve", 0),
        "kind_extrude": kinds.get("extrude", 0),
        "kind_loft": kinds.get("loft", 0),
        "box_share_solids": round(box_share, 4),
        "box_share_expanded": round(box_share_exp, 4),
        "md_50": md_counts[50],
        "md_100": md_counts[100],
        "md_200": md_counts[200],
        "md_500_or_all": sum(1 for s in solids if s.get("maxDenominator") in (None, 500)),
        "vis50_solids": vis_s[50],
        "vis100_solids": vis_s[100],
        "vis200_solids": vis_s[200],
        "vis500_solids": vis_s[500],
        "vis50_expanded": vis_e[50],
        "vis100_expanded": vis_e[100],
        "vis200_expanded": vis_e[200],
        "vis500_expanded": vis_e[500],
        "scale_differentiated_solids": scale_diff,
        "distinct_materials": len(materials),
        "distinct_roles": len(roles),
        "distinct_tones": len(tones),
        "thermal_regions": len(therm_ids),
        "thermal_regions_without_solid": len(regions_no_solid),
        "thermal_regions_without_solid_ids": ";".join(regions_no_solid),
        "solids_with_region": solids_with_region,
        "solids_without_region": solids_without_region,
        "solids_undefined_region": solids_undefined_region,
        "bones_without_solid": len(uncovered_bones),
        "bones_without_solid_ids": ";".join(uncovered_bones),
        "member_shapes_50": members50,
        "member_shapes_all_scales": members_all,
        "member_bones_50": len(member_bones),
        "shapes50_elevation": elev50,
        "shapes50_plan": plan50,
        "shapes50_front": front50,
        "ratio_expanded_solids_per_elev50": round(ratio, 3),
        "status_50": status["50"],
        "status_100": status["100"],
        "status_200": status["200"],
        "status_500": status["500"],
        "unavailable_scales": "; ".join(unavailable),
        "poses": len(a.get("poses", [])),
        "clips": len(a.get("clips", [])),
        "decisions_count": len(decisions),
        "decisions_en_chars": dec_chars,
        "organic": organic,
        "zero_scale_differentiation": zero_diff,
        "crude_score": crude,
        "_path": os.path.relpath(path, CATALOG),
    }


def main():
    paths = sorted(glob.glob(os.path.join(CATALOG, "*", "*", "🔣️.json")))
    rows = [analyse(p) for p in paths]
    fields = [k for k in rows[0] if not k.startswith("_")] + ["_path"]
    out_csv = os.path.join(TICKET, "audit-volumes.csv")
    with open(out_csv, "w", encoding="utf-8", newline="") as f:
        w = csv.DictWriter(f, fieldnames=fields)
        w.writeheader()
        for r in rows:
            w.writerow({k: r[k] for k in fields})

    crude = sorted(rows, key=lambda r: (-r["crude_score"], r["ratio_expanded_solids_per_elev50"]))[:30]
    zero = [r for r in rows if r["zero_scale_differentiation"]]
    thermal = sorted(rows, key=lambda r: (-r["thermal_regions_without_solid"], -r["thermal_regions"]))
    thermal = [r for r in thermal if r["thermal_regions_without_solid"] > 0 or r["solids_undefined_region"] > 0]
    bones_gap = sorted([r for r in rows if r["bones_without_solid"] > 0], key=lambda r: -r["bones_without_solid"])
    unav = [r for r in rows if r["unavailable_scales"]]
    summary = {
        "assets_with_unavailable_2d_scale": len(unav),
        "assets": len(rows),
        "solids": sum(r["solids"] for r in rows),
        "solids_expanded": sum(r["solids_expanded"] for r in rows),
        "kinds": {k: sum(r["kind_" + k] for r in rows) for k in KINDS},
        "zero_scale_diff_assets": len(zero),
        "assets_with_thermal_gap": len(thermal),
        "thermal_regions_total": sum(r["thermal_regions"] for r in rows),
        "thermal_regions_without_solid_total": sum(r["thermal_regions_without_solid"] for r in rows),
        "solids_without_region_total": sum(r["solids_without_region"] for r in rows),
        "solids_undefined_region_total": sum(r["solids_undefined_region"] for r in rows),
        "bones_total": sum(r["bones"] for r in rows),
        "bones_without_solid_total": sum(r["bones_without_solid"] for r in rows),
        "assets_with_bones_without_solid": len(bones_gap),
        "member_shapes_50_total": sum(r["member_shapes_50"] for r in rows),
        "assets_with_members": sum(1 for r in rows if r["member_shapes_all_scales"] > 0),
        "poses_total": sum(r["poses"] for r in rows),
        "clips_total": sum(r["clips"] for r in rows),
        "decisions_total": sum(r["decisions_count"] for r in rows),
        "maxD_counts": {v: sum(r["md_" + str(v)] for r in rows) for v in (50, 100, 200)},
    }
    os.makedirs(GEN, exist_ok=True)
    def slim(r):
        return {k: r[k] for k in ("category", "slug", "solids", "solids_expanded", "box_share_expanded", "shapes50_elevation",
                                  "ratio_expanded_solids_per_elev50", "zero_scale_differentiation", "scale_differentiated_solids",
                                  "bones", "bones_without_solid", "thermal_regions", "thermal_regions_without_solid",
                                  "solids_without_region", "solids_undefined_region", "member_shapes_50", "crude_score",
                                  "distinct_materials", "distinct_roles", "distinct_tones", "poses", "clips", "decisions_count",
                                  "decisions_en_chars", "thermal_regions_without_solid_ids", "bones_without_solid_ids",
                                  "status_200", "status_500", "unavailable_scales")}
    json.dump({"summary": summary, "crude30": [slim(r) for r in crude], "zero": [slim(r) for r in zero],
               "thermal": [slim(r) for r in thermal], "unavailable": [slim(r) for r in unav], "bones_gap": [slim(r) for r in bones_gap]},
              open(os.path.join(GEN, "rankings.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps(summary, ensure_ascii=False, indent=1))


if __name__ == "__main__":
    main()
