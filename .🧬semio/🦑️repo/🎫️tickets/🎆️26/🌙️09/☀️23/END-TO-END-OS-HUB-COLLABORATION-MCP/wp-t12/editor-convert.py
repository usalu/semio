"""🔁️ Reads the 15 editor vocabularies' committed owner vectors (`🧫️fixtures/🔁️mutations.json` /
`🔁️mutation-contracts.json`) into the per-scenario authoring spec `wp-t12/editor-vectors.json` (before, mutation,
hand-committed after) — one applied vector per kind, plus a no-op vector (the applied after re-applied) where the leaf
declares `no-op`. Vocabularies without owner vectors keep their hand-authored entries in the spec untouched."""
import json, os
root = "/Users/ueli/Documents/semio/"
survey = json.load(open(root + ".tmp-ticket/wp-t12/generated/editor-survey.json", encoding="utf-8"))
spec_path = root + ".tmp-ticket/wp-t12/editor-vectors.json"
spec = json.load(open(spec_path, encoding="utf-8")) if os.path.exists(spec_path) else {}
for i, v in enumerate(survey):
    if not v["vectors"]: continue
    data = json.load(open(root + v["owner"] + "/" + v["vectors"][0], encoding="utf-8"))
    cases = data["cases"] if isinstance(data, dict) else data
    base = data.get("base") if isinstance(data, dict) else None
    leaves = {l["kind"]: l for l in v["leaves"]}
    scenarios, seen = [], set()
    for case in cases:
        kind = case["kind"]
        if kind in seen or case.get("valid") is False: continue
        seen.add(kind)
        before = case.get("base", base)
        after = case.get("after", case.get("expected"))
        scenarios.append({"kind": kind, "status": "applied", "before": before, "mutation": case["mutation"], "after": after, "source": v["vectors"][0]})
        if "no-op" in leaves[kind]["outcomes"]:
            scenarios.append({"kind": kind, "status": "no-op", "before": after, "mutation": case["mutation"], "after": after, "source": v["vectors"][0]})
    missing = sorted(set(leaves) - seen)
    spec[str(i)] = {"owner": v["owner"], "scenarios": scenarios, "missingKinds": missing}
    print(i, v["aggregate"], len(scenarios), "missing:", missing)
json.dump(spec, open(spec_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
