"""Group plans: the 429 tracked (+2 ignored) .cursor/plans files and .codex/plans/declarative-actions.md become ticket-owned files."""
import json, os
from blib import *

g = "plans"
m = json.load(open(os.path.join(HERE, "staged", "plans-map.json"), encoding="utf-8"))
for tdir, doc in m["archiveTickets"].items():
    CREATE(g, tdir + "/🎫️ticket.json", jtext(doc), "plans-archive-ticket")
for e in m["entries"]:
    if e["kind"] == "duplicate":
        DELETE(g, e["src"], "plans-duplicate")
    else:
        MOVE(g, e["src"], e["dest"], "plans-move")
for e in m["ticketEdits"]:
    ED(g, e["path"], e["old"], e["new"], (1,), "plans-ticket-local")
ED(g, ".gitignore", "!**/🧫️fixtures/**\n", "!**/🧫️fixtures/**\n\n# 🗺️ A ticket owns its plans: *.plan.md files in a ticket folder are evidence, even when a generic rule above (for example `target*`) matches the name.\n!.🧬semio/🦑️repo/🎫️tickets/**/*.plan.md\n", (1,), "plans-gitignore")
RMDIR(g, ".cursor/plans"); RMDIR(g, ".cursor"); RMDIR(g, ".codex/plans")
