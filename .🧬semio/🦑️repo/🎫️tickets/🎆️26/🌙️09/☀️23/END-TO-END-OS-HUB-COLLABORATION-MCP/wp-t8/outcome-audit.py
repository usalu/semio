"""Audits each outcome-mismatched mutation: what the leaf descriptor declares, whether the leaf's code can reject, and what the manifest declares."""
import json, os, re, sys
root = "/Users/ueli/Documents/semio/"
targets = {
    "s.wfc.wfc2d": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any",
    "s.wfc.grid2d": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any",
    "s.wfc.grid3d": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any",
    "s.fem.2d": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets",
    "s.fem.3d": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets",
    "s.sequence.sequence": "✏️s/🔌️plugins/" + next(d for d in os.listdir(root + "✏️s/🔌️plugins") if d.endswith("sequence")) + "/🗿️artifacts",
}
REJECT = re.compile(r"MutationOutcome::(?:error|fatal)\b|\.(?:error|fatal)\(|Outcome::rejected|reject\(")
for artifact, owner in targets.items():
    leaves = {}
    for dirpath, _, files in os.walk(root + owner):
        if "🔣️.json" in files and os.path.basename(os.path.dirname(dirpath)) == "🧬️mutations":
            try: d = json.load(open(os.path.join(dirpath, "🔣️.json")))
            except ValueError: continue
            if "semanticKind" in d: leaves.setdefault(d["semanticKind"], []).append((dirpath, d.get("outcomeClasses")))
    manifests = []
    for dirpath, _, files in os.walk(root + owner):
        if dirpath.endswith("🔮️oracles") and "🔣️.json" in files:
            for m in json.load(open(os.path.join(dirpath, "🔣️.json"))).get("mutationManifests", []):
                if m["artifact"] == artifact: manifests.append((dirpath, m))
    for path, m in manifests:
        for row in m["mutations"]:
            for leaf, classes in leaves.get(row["id"], []):
                code = "".join(open(os.path.join(dp, f), encoding="utf-8").read() for dp, _, fs in os.walk(leaf) if "🧪️tests" not in dp for f in fs if f.endswith(".rs"))
                can_reject = bool(REJECT.search(code))
                runtime = sorted({{"applied": "applied", "warning": "applied", "info": "no-op", "error": "rejected", "fatal": "rejected"}[c] for c in classes or []})
                if sorted(row["outcomes"]) != runtime:
                    print(f"{artifact}@{m['subset']} {row['id']}: manifest={row['outcomes']} descriptor={classes} codeRejects={can_reject}")
