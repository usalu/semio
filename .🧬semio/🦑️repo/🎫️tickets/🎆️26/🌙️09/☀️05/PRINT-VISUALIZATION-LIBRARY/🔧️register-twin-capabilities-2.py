"""🔮️ Declares the twin capabilities of the new twin cases on the oracles that adjudicate them."""
import io, json, os

os.chdir(os.path.join(os.environ["SEMIO_ROOT"], "🧰️framework/🛍️products/📓️print/🔮️oracle"))
doc = json.load(io.open("🔣️.json", encoding="utf-8"))
extra = {
    "d3-array": ["network-placement-layouts-twin"],
    "dagre": ["network-layered-layout-twin"],
}
for entry in doc["oracles"]:
    for capability in extra.get(entry["id"], []):
        if capability not in entry["capabilities"]:
            entry["capabilities"].append(capability)
io.open("tmp.json", "w", encoding="utf-8", newline="\n").write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
os.replace("tmp.json", "🔣️.json")
print("ok")
