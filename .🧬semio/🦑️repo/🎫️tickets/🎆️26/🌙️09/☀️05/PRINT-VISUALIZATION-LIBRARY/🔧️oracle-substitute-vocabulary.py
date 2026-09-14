"""🔤️ Puts the last two no-oracle decisions into the schema's substitute vocabulary. `render-scene`
and `viz-typeset-appearance` invented `self-consistency-rebuild` and `pdf-text-extraction-probe`;
the enum's own words for those are `metamorphic-laws` (the same source rebuilt gives the same bytes
is a metamorphic relation) and `second-parser` (a PDF text extractor reads the artifact back)."""
import io, json, os

os.chdir(os.path.join(os.environ["SEMIO_ROOT"], "🧰️framework/🛍️products/📓️print/🔮️oracle"))
doc = json.load(io.open("🔣️.json", encoding="utf-8"))
rename = {"self-consistency-rebuild": "metamorphic-laws", "pdf-text-extraction-probe": "second-parser"}
for entry in doc["noOracleDecisions"]:
    seen = []
    for value in entry["substitutes"]:
        mapped = rename.get(value, value)
        if mapped not in seen:
            seen.append(mapped)
    entry["substitutes"] = seen
io.open("tmp.json", "w", encoding="utf-8", newline="\n").write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
os.replace("tmp.json", "🔣️.json")
print("ok")
