"""🎭 Read-only check of the course cast manifest against the staffage catalog: asset, pose or clip, scale band, viewpoint, mirror and the D19 mirror allowlist."""

import json
import sys
from collections import Counter
from pathlib import Path

casts_path, catalog_dir, guard_path, output = (Path(a) for a in sys.argv[1:5])
VIEWPOINTS = ("elevation", "front", "plan")
DEFAULT_SCALE = 50

casts = json.loads(casts_path.read_text(encoding="utf-8"))["casts"]
guard = json.loads(guard_path.read_text(encoding="utf-8"))
allowed_mirror = guard["cast_rules"]["mirrored_assets"]

manifest = json.loads((catalog_dir / "🔣️.json").read_text(encoding="utf-8"))
folder_of = {c["id"]: c["folder"] for c in manifest["categories"]}
rows = {e["id"]: e for e in manifest["entries"]}


def asset_file(slug: str) -> Path | None:
    row = rows.get(slug)
    if row is None:
        return None
    folder = catalog_dir / folder_of[row["category"]] / slug
    if not folder.is_dir():
        return None
    files = [p for p in folder.iterdir() if p.suffix == ".json"]
    return files[0] if len(files) == 1 else None


def asymmetry(asset: dict) -> list[str]:
    reasons = []
    for bone in asset["rig"]["bones"]:
        if bone.get("z", 0) != 0:
            reasons.append(f"bone {bone['id']} z={bone['z']}")
    for anchor in asset["anchors"]:
        if anchor.get("z", 0) != 0:
            reasons.append(f"anchor {anchor['id']} z={anchor['z']}")
    for denominator, band in asset["representations"].items():
        if band.get("status") != "authored":
            continue
        for view in ("front", "plan"):
            for shape in band.get("geometry", {}).get(view, []) or []:
                solid = shape.get("solid") if isinstance(shape, dict) else None
                if solid and solid.get("kind") == "extruded":
                    z0, z1 = solid["z0"], solid["z1"]
                    if not (z0 < 0 < z1) and (z0, z1) != (0, 0):
                        reasons.append(f"{denominator} {view} shape {shape.get('id')} one-sided z0={z0} z1={z1}")
    return sorted(set(reasons))


results = []
failures = Counter()
failing_ids: dict[str, list[str]] = {}
asset_cache: dict[str, dict | None] = {}

for cast_id, cast in casts.items():
    problems = []
    slug = cast["asset"]
    if slug not in asset_cache:
        path = asset_file(slug)
        asset_cache[slug] = json.loads(path.read_text(encoding="utf-8"))["asset"] if path else None
    asset = asset_cache[slug]
    scale = cast.get("scale", DEFAULT_SCALE)
    if asset is None:
        problems.append("asset-missing")
    else:
        poses = {p["id"]: p for p in asset["poses"]}
        named = {pid for pid, p in poses.items() if not p.get("key")}
        if ("pose" in cast) == ("clip" in cast):
            problems.append("pose-xor-clip")
        if "pose" in cast:
            if cast["pose"] not in poses:
                problems.append(f"pose-missing:{cast['pose']}")
            elif cast["pose"] not in named:
                problems.append(f"pose-is-key-pose:{cast['pose']}")
        if "clip" in cast and cast["clip"] not in {c["id"] for c in asset["clips"]}:
            problems.append(f"clip-missing:{cast['clip']}")
        band = asset["representations"].get(str(scale))
        if band is None:
            problems.append(f"scale-no-band:{scale}")
        elif band["status"] != "authored":
            problems.append(f"scale-unavailable:{scale}")
        viewpoint = cast.get("viewpoint")
        if viewpoint not in VIEWPOINTS:
            problems.append(f"viewpoint-invalid:{viewpoint}")
        if cast.get("mirror") is True:
            if viewpoint != "elevation":
                problems.append(f"mirror-not-elevation:{viewpoint}")
            asym = asymmetry(asset)
            if asym and not str(allowed_mirror.get(slug, "")).strip():
                problems.append("mirror-asymmetric-without-d19-reason")
            if not str(allowed_mirror.get(slug, "")).strip():
                problems.append("mirror-asset-not-in-allowlist")
    for problem in problems:
        failures[problem.split(":")[0]] += 1
    if problems:
        failing_ids[cast_id] = problems
    results.append({"id": cast_id, "asset": slug, "pose": cast.get("pose"), "clip": cast.get("clip"), "scale": scale, "viewpoint": cast.get("viewpoint"), "mirror": cast.get("mirror", False), "problems": problems})

summary = {"casts": len(casts), "failingCasts": len(failing_ids), "passingCasts": len(casts) - len(failing_ids), "failureCounts": dict(failures.most_common())}
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps({"generatedBy": "haiku-registry", "summary": summary, "failingCasts": failing_ids, "casts": results}, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(json.dumps(summary, ensure_ascii=False))
for cast_id, problems in failing_ids.items():
    print(f"{cast_id}: {'; '.join(problems)}")
