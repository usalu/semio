"""🔒 Close both concrete-clip tickets manually while the repo MCP server is unavailable.

See https://github.com/usalu/semio/issues/2775 and https://github.com/usalu/semio/issues/2776.
"""
import json
import subprocess
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
SUMMARIES = {
    "CONCRETE-BIM-CLIP-EXAMPLES": (
        "🏗️bim",
        "All six BIM clips (information, objects, representations, models, exchange, consistency) rebuilt on one "
        "schema-backed Villa Savoye teaching model (🧱️model JSON + schema): every IFC value is drawn on the building "
        "it describes (plans, sections, axonometry, analysis net, exchange round trips, revision lifecycle). Assumed "
        "values are starred and sourced (piloti count 25*, sources 15–25); type vs occurrence property sets, hanging "
        "garden on the first floor. Audits, verification and final verification per clip; strict layout checker; "
        "transcripts and README regenerated; 137/138 tests pass (the one error needs the missing repo binary). "
        "Rendered en/de × speaker/spectator at 1080p30 and delivered with provenance (see render-r2.md in "
        "CONCRETE-OPTIMIZATION-CLIP-EXAMPLES)."),
    "CONCRETE-OPTIMIZATION-CLIP-EXAMPLES": (
        "🎯️optimization",
        "All six optimization clips (extrema, sampling, experiments, index, pareto, algorithms) rebuilt on the "
        "seminar-room façade study: the shared model returns full heating/cooling/lighting balance terms with "
        "continuous beam-weighted shading, and every plotted value is shown simultaneously by its physical meaning "
        "in a live room section (sun, overhang, glazing, lamps, losses) with printed derivations that hold as "
        "printed. Audits, verification, final verification, strict layout checks (0 findings except 2 in "
        "algorithms), 70/70 tests, transcripts and README regenerated. All 12 BIM and optimization videos rendered "
        "en/de × speaker/spectator at 1080p30 through a ticket repo shim and delivered as H.264 with delivery.json "
        "provenance; see render-r2.md."),
}


def changed(area: str) -> list[str]:
    """📂 List working-tree changes below one architecture topic."""
    output = subprocess.run(["git", "-c", "core.quotepath=false", "status", "--porcelain", "--",
                             f"🎓️teaching/🏛️architecture/{area}"], cwd=ROOT, capture_output=True, text=True,
                            encoding="utf-8", check=True).stdout
    return [line[3:] for line in output.splitlines()]


for slug, (area, summary) in SUMMARIES.items():
    folder = TICKET.parent / slug
    path = folder / "🎫️ticket.json"
    ticket = json.loads(path.read_text(encoding="utf-8"))
    ticket["status"] = "closed"
    ticket["interactions"].append({
        "kind": "ticket.close", "date": "2026-10-28 07:30:00", "author": "", "system": "windows",
        "client": "claude-code", "checkpoint": "", "prompt": summary, "llm": "opus-4-7",
        "files": changed(area) + sorted(str(p.relative_to(ROOT)).replace("\\", "/") for p in folder.rglob("*") if p.is_file()),
    })
    ticket["note"] = "repo MCP server failed to connect on 2026-10-27 18:15 and 2026-10-28; reopened and closed manually."
    path.write_text(json.dumps(ticket, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("[DEBUG] closed", slug, len(ticket["interactions"][-1]["files"]), "files")
