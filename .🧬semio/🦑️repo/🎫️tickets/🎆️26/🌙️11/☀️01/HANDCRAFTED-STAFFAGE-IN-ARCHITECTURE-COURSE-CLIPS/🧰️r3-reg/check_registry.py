"""🧭 Read-only registry checker for the handcrafted staffage catalog, environment presets and the cast manifest.

Run: python -I check_registry.py  (stdlib only). Writes 🗑️generated/r3-reg/check.json next to the ticket and exits 1 on any failure.
"""
import json
import os
import sys
from collections import Counter

sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
TICKET = os.path.dirname(HERE)
ROOT = next(
    os.path.join(p, "🎓️teaching", "🏛️architecture")
    for p in (os.path.abspath(os.path.join(TICKET, *[".."] * i)) for i in range(12))
    if os.path.isdir(os.path.join(p, "🎓️teaching", "🏛️architecture"))
)
S = os.path.join(ROOT, "🧍️staffage")
CATALOG = os.path.join(S, "🗂️catalog")
INDEX = os.path.join(CATALOG, "🔣️.json")
ENV_INDEX = os.path.join(CATALOG, "🌤️environments", "🔣️.json")
CASTS = os.path.join(ROOT, "⚡️energy", "🏗️authoring", "🎨️visuals", "🧍️staffage", "🔣️.json")
OUT = os.path.join(TICKET, "🗑️generated", "r3-reg", "check.json")
VIEWS = {"dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments"}
ENV_KEYS = {"airTempC", "meanRadiantTempC", "relativeHumidity", "windMS", "windDirectionDeg", "solarWM2", "diffuseFraction", "pressurePa"}
UNREGISTERED_CAST_FIELDS = ("timeline", "transient")


def load(path, pairs=None):
    with open(path, encoding="utf-8") as f:
        return json.load(f, object_pairs_hook=pairs) if pairs else json.load(f)


def dup_keys(path):
    found = []

    def hook(pairs):
        found.extend(k for k, n in Counter(k for k, _ in pairs).items() if n > 1)
        return dict(pairs)

    load(path, hook)
    return found


def lang_ok(t):
    return isinstance(t, dict) and all(isinstance(t.get(k), str) and t[k].strip() for k in ("en", "de"))


def dup_groups(items):
    groups = {}
    for k, v in items:
        groups.setdefault(json.dumps(v, sort_keys=True, ensure_ascii=False), []).append(k)
    return [g for g in groups.values() if len(g) > 1]


index = load(INDEX)
entries = index["entries"]
cat_ids = {c["id"]: c for c in index["categories"]}
problems = {k: [] for k in (
    "index_dup_ids", "folder_no_index", "index_no_folder", "folder_no_manifest", "manifest_id_mismatch",
    "category_mismatch", "missing_en_de_index", "missing_en_de_manifest", "pose_or_clip_missing_en_de",
    "index_poses_not_in_manifest", "index_clips_not_in_manifest", "manifest_poses_clips_not_in_index",
    "env_dup_ids", "env_missing_en_de", "env_keys_incomplete", "cast_bad", "cast_raw_duplicate_keys",
    "cast_duplicate_content", "cast_unregistered_fields")}

ids_index = [e["id"] for e in entries]
problems["index_dup_ids"] = [k for k, n in Counter(ids_index).items() if n > 1]

folders = {}
for cat in sorted(os.listdir(CATALOG)):
    p = os.path.join(CATALOG, cat)
    if os.path.isdir(p):
        for a in sorted(os.listdir(p)):
            if os.path.isdir(os.path.join(p, a)):
                folders[a] = (cat, os.path.join(p, a))

asset_docs = {}
for a, (cat, ap) in folders.items():
    mp = os.path.join(ap, "🔣️.json")
    if not os.path.isfile(mp):
        problems["folder_no_manifest"].append(a)
        continue
    doc = load(mp)
    asset_docs[a] = doc["asset"]
    if doc["asset"]["id"] != a:
        problems["manifest_id_mismatch"].append((a, doc["asset"]["id"]))
    if doc["asset"].get("category") not in cat_ids or cat_ids[doc["asset"]["category"]]["folder"] != cat:
        problems["category_mismatch"].append((a, cat, doc["asset"].get("category")))
    if not lang_ok(doc["asset"].get("title")):
        problems["missing_en_de_manifest"].append(a)
    for p in doc["asset"].get("poses", []) + doc["asset"].get("clips", []):
        if not lang_ok(p.get("title")):
            problems["pose_or_clip_missing_en_de"].append((a, p.get("id")))

problems["folder_no_index"] = [a for a in folders if a not in ids_index]
problems["index_no_folder"] = [i for i in ids_index if i not in folders]

for e in entries:
    if not lang_ok(e.get("title")):
        problems["missing_en_de_index"].append(e["id"])
    if e["id"] in asset_docs:
        d = asset_docs[e["id"]]
        mp = {p["id"] for p in d.get("poses", [])}
        mc = {c["id"] for c in d.get("clips", [])}
        problems["index_poses_not_in_manifest"] += [(e["id"], p) for p in e.get("poses", []) if p not in mp]
        problems["index_clips_not_in_manifest"] += [(e["id"], c) for c in e.get("clips", []) if c not in mc]
        ip, ic = set(e.get("poses", [])), set(e.get("clips", []))
        problems["manifest_poses_clips_not_in_index"] += [(e["id"], x) for x in sorted((mp - ip) | (mc - ic))]

env_doc = load(ENV_INDEX)
presets = env_doc["presets"]
problems["env_dup_ids"] = [k for k, n in Counter(p["id"] for p in presets).items() if n > 1]
env_ids = {p["id"] for p in presets}
for p in presets:
    if not lang_ok(p.get("title")) or not lang_ok(p.get("description")):
        problems["env_missing_en_de"].append(p["id"])
    if set(p.get("environment", {})) != ENV_KEYS:
        problems["env_keys_incomplete"].append((p["id"], sorted(ENV_KEYS ^ set(p.get("environment", {})))))
env_physical_groups = [
    [p["id"] for p in presets if json.dumps(p["environment"], sort_keys=True) == json.dumps(q["environment"], sort_keys=True)]
    for q in presets
]
env_physical_groups = sorted({tuple(g) for g in env_physical_groups if len(g) > 1})

cast_doc = load(CASTS)
casts = cast_doc["casts"]
problems["cast_raw_duplicate_keys"] = dup_keys(CASTS)
problems["cast_duplicate_content"] = dup_groups(casts.items())
cast_stats = {"casts_total": len(casts), "shared_total": sum(k.startswith("shared-") for k in casts),
              "casts_without_environment": [k for k, c in casts.items() if "environment" not in c]}
for k, c in casts.items():
    errs = []
    a = c.get("asset")
    if a not in asset_docs:
        errs.append("asset-missing:" + str(a))
    else:
        d = asset_docs[a]
        if "pose" in c and c["pose"] not in {p["id"] for p in d.get("poses", [])}:
            errs.append("pose-missing:" + str(c["pose"]))
        if "clip" in c and c["clip"] not in {x["id"] for x in d.get("clips", [])}:
            errs.append("clip-missing:" + str(c["clip"]))
        if "pose" not in c and "clip" not in c:
            errs.append("no-pose-or-clip")
    if c.get("view") not in VIEWS:
        errs.append("view-invalid:" + str(c.get("view")))
    if "environment" in c and c["environment"] not in env_ids:
        errs.append("environment-missing:" + str(c["environment"]))
    if "environment" not in c:
        errs.append("no-environment")
    unreg = [f for f in UNREGISTERED_CAST_FIELDS if f in c]
    if unreg:
        problems["cast_unregistered_fields"].append((k, unreg))
    if errs:
        problems["cast_bad"].append((k, errs))

failures = {k: v for k, v in problems.items() if v}
out = {
    "index_entries": len(entries), "index_categories": len(index["categories"]),
    "index_collections": len(index["collections"]), "asset_folders": len(folders),
    "asset_manifests": len(asset_docs), "env_presets": len(presets),
    "env_physical_duplicate_groups": env_physical_groups,
    **cast_stats, "problems": problems, "failures": sorted(failures),
}
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w", encoding="utf-8", newline="\n") as f:
    json.dump(out, f, ensure_ascii=False, indent=1)
print(json.dumps({k: v for k, v in out.items() if k != "problems"}, ensure_ascii=False, indent=1))
print("PROBLEMS:", json.dumps(failures, ensure_ascii=False, indent=1) if failures else "none")
print("RESULT:", "FAIL" if failures else "PASS", "->", OUT)
sys.exit(1 if failures else 0)
