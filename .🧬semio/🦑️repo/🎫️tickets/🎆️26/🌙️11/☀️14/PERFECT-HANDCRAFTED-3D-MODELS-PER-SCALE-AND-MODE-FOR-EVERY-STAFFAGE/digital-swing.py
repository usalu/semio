"""⛓️ Join both halves of each playground-swing link at the wire centreline."""
from pathlib import Path
from runpy import run_path
import json

ticket = Path(__file__).resolve().parent
source = run_path(str(ticket / "r3-playground-swing-s50.py"))
update = run_path(str(ticket / "digital-models.py"))["replace_scale"]
catalog = next(ticket.parents[6].glob("🎓️teaching/*/*staffage/🗂️catalog/*/playground-swing/🔣️.json"))
links = [s for s in source["chains"]() if s["id"].startswith("links-")]
for path, shipped in [(ticket / "volumes/objects/playground-swing.json", False), (catalog, True)]:
    document = json.loads(path.read_bytes())
    model = (document["asset"] if shipped else document)["volumes"]["50"]
    for name, items, prefix in [("body", links, "links-"), ("thermal", source["thermal"](links), "th-links-")]:
        parts = model[name]
        index = next(i for i, item in enumerate(parts) if item["id"].startswith(prefix))
        model[name] = [s for s in parts[:index] if not s["id"].startswith(prefix)] + items + [s for s in parts[index:] if not s["id"].startswith(prefix)]
    update(path, 50, model, shipped)
