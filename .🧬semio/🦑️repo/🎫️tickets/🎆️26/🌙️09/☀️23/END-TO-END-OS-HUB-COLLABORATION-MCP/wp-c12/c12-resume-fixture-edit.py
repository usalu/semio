"""🧫️ C12 14c: the resume fixture's two cases the live lease parser refuses — a write grant needs the editor role (grant follows role),
another component needs the closed browser actor's source digest to follow it. Idempotent; `--dry-run` prints the plan only."""
import json, sys
path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json"
data = json.load(open(path, encoding="utf-8"))
cases = data["cases"]
b = "b" * 64
changes = []
for row in cases:
    if row["case"] == "write access granted since":
        row["case"] = "the editor role granted since"
        row["patch"] = {"surface": {"surfaceId": "s.gis.gismap@1/*#editor", "appId": "s.gis.gismap@1/*#editor", "role": "editor"}, "grant": {"write": True}}
        changes.append("editor role case")
    if row["case"] == "another component" and "browserActor" not in row["patch"]:
        row["patch"]["browserActor"] = {"sourceComponentSha256": b}
        changes.append("another component follows the browser actor")
print(changes or "nothing to do")
if changes and "--dry-run" not in sys.argv:
    open(path, "w", encoding="utf-8").write(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
