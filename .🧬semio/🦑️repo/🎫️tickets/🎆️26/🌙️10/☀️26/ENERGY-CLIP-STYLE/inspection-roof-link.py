"""🏠 Probe the current roof event/link API before attributing a cached native failure."""
import importlib
import json
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
base = model.load_model()
event = model.demo_event("roof_soil", base)
changed = model.apply_event(event, base)
resolved = model.resolve_link(changed, "LK-ROOF")
assert "LK-ROOF" in {link["id"] for link in base["links"]}
link = next(link for link in resolved["links"] if link["id"] == "LK-ROOF")
assert link["source_revision"] == resolved["revision"]
result = {"event": event, "base_revision": base["revision"], "changed_revision": changed["revision"],
          "resolved_link": link}
(TICKET / "inspection-roof-link.md").write_text(
    "# Inspection Current Roof Link\n\nFresh-process current roof event and LK-ROOF resolution passed. "
    "The earlier cached failure did not reproduce against current fresh inputs; no compatibility alias or model replacement was added.\n\n```json\n"
    + json.dumps(result, ensure_ascii=False, indent=2) + "\n```\n", encoding="utf-8")
print("[DEBUG] Current LK-ROOF resolved", link["source_revision"], flush=True)
