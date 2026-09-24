"""Aligns the outcome-mismatched wfc and sequence mutations under the bridge's projection (applied/info/warning → applied,
error/fatal → rejected), additively and only on direct leaf evidence: a descriptor gains `error`/`fatal` when the leaf's own
code returns `MutationOutcome::error`/`::fatal`; a manifest row loses `rejected` only when neither the descriptor nor the
leaf code (with no delegated guard) can reject, and gains it when the descriptor declares a rejecting class."""
import json, os, re, pathlib
root = "/Users/ueli/Documents/semio/"
targets = {
    "s.wfc.wfc2d": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any",
    "s.wfc.grid2d": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any",
    "s.wfc.grid3d": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any",
    "s.sequence.sequence": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts",
    "s.fem.2d": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets",
    "s.fem.3d": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets",
}
PROJECT = {"applied": "applied", "info": "applied", "warning": "applied", "error": "rejected", "fatal": "rejected"}
def code_of(leaf):
    return "".join(open(os.path.join(dp, f), encoding="utf-8").read() for dp, _, fs in os.walk(leaf) if "🧪️tests" not in dp for f in fs if f.endswith(".rs"))
for artifact, owner in targets.items():
    leaves = {}
    for dirpath, _, files in os.walk(root + owner):
        if "🔣️.json" in files and os.path.basename(os.path.dirname(dirpath)) == "🧬️mutations":
            try: d = json.load(open(os.path.join(dirpath, "🔣️.json")))
            except ValueError: continue
            if "semanticKind" in d: leaves.setdefault(d["semanticKind"], []).append(dirpath)
    for dirpath, _, files in os.walk(root + owner):
        if not (dirpath.endswith("🔮️oracles") and "🔣️.json" in files): continue
        mpath = pathlib.Path(dirpath, "🔣️.json"); text = mpath.read_text(encoding="utf-8"); changed = False
        for m in json.loads(text).get("mutationManifests", []):
            if m["artifact"] != artifact: continue
            for row in m["mutations"]:
                for leaf in leaves.get(row["id"], []):
                    dpath = pathlib.Path(leaf, "🔣️.json"); dtext = dpath.read_text(encoding="utf-8"); classes = json.loads(dtext)["outcomeClasses"]
                    if sorted(row["outcomes"]) == sorted({PROJECT[c] for c in classes}): continue
                    code = code_of(leaf)
                    added = [c for c in ("error", "fatal") if c not in classes and re.search(rf"MutationOutcome::{c}\s*\(", code)]
                    if added:
                        block = re.search(r'"outcomeClasses":\s*\[[^\]]*\]', dtext); indent = re.search(r'\n(\s*)"outcomeClasses"', dtext).group(1)
                        classes = classes + added
                        dtext = dtext[:block.start()] + '"outcomeClasses": [\n' + ",\n".join(f'{indent}  "{c}"' for c in classes) + f"\n{indent}]" + dtext[block.end():]
                        json.loads(dtext); dpath.write_text(dtext, encoding="utf-8")
                        print(f"descriptor {artifact} {row['id']}: + {added}")
                    runtime = sorted({PROJECT[c] for c in classes})
                    if sorted(row["outcomes"]) == runtime: continue
                    if set(row["outcomes"]) - {"applied", "rejected"}:
                        print(f"UNDECIDED {artifact} {row['id']}: manifest declares {row['outcomes']}, which the bridge projection cannot report"); continue
                    delegated = re.search(r"guards::|return\s+rejection|validate\w*\(", code)
                    if "rejected" in row["outcomes"] and "rejected" not in runtime and delegated:
                        print(f"UNDECIDED {artifact} {row['id']}: delegated guard"); continue
                    span = re.search(r'"id":\s*' + re.escape(json.dumps(row["id"])) + r'[^{}]*?"outcomes":\s*\[[^\]]*\]', text)
                    text = text[:span.start()] + re.sub(r'"outcomes":\s*\[[^\]]*\]', '"outcomes": ' + json.dumps(runtime), span.group(0)) + text[span.end():]
                    changed = True
                    print(f"manifest {artifact} {row['id']}: {row['outcomes']} -> {runtime}")
        if changed: json.loads(text); mpath.write_text(text, encoding="utf-8")
