"""🔮️ States the four remaining no-oracle decisions in the substitute the platform actually reads:
a reference computed inside the one adapter is a specification vector, not a second implementation
(which the platform defines as a second adapter). Retires `solar-position`, whose case now has the
registered suncalc oracle."""
import io, json, os

os.chdir(os.path.join(os.environ["SEMIO_ROOT"], "🧰️framework/🛍️products/📓️print/🔮️oracle"))
doc = json.load(io.open("🔣️.json", encoding="utf-8"))

vectors = {"rk-field-integration", "critical-path-method", "projectile-integration", "survival-product-limit"}
kept = []
for entry in doc["noOracleDecisions"]:
    if entry["id"] == "solar-position":
        continue
    if entry["id"] in vectors:
        entry["substitutes"] = ["specification-vectors"]
    kept.append(entry)
doc["noOracleDecisions"] = kept

for entry in doc["oracles"]:
    if entry["id"] == "suncalc":
        entry["capabilities"] = ["diagram-solar-position"]

io.open("tmp.json", "w", encoding="utf-8", newline="\n").write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
os.replace("tmp.json", "🔣️.json")
print("decisions", len(kept))
