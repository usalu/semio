"""Reopen or close the ticket on disk for the overview camera revision (repo MCP down): `reopen` or `close <summary file>`.

The summary file is JSON: `{ "summary": "...", "files": { "created": [...], "updated": [...], "removed": [...] } }`.
"""
import json
import pathlib
import sys

ticket = pathlib.Path(__file__).resolve().parent
path = ticket / "🎫️ticket.json"
data = json.loads(path.read_text(encoding="utf-8"))
verb = sys.argv[1]

if verb == "reopen":
    data["status"] = "open"
    data["description"] += (
        " Reopened 2026-10-02 (overview exactly like play): two layers as on semio-tech play and the mit-bestand"
        " demonstrator — fixed cards on a grid in front, the screen-sized pages behind the glass; the background moves"
        " under the glass while the mouse is between the cards, and hovering a card brings its page to the screen, crisp."
    )
    data["note"] += " Reopened manually on 2026-10-02 (overview exactly like play; repo and semio MCP servers still not connected), see 📓️overview-camera-report.md."
elif verb == "close":
    addition = json.loads(pathlib.Path(sys.argv[2]).read_text(encoding="utf-8"))
    data["status"] = "closed"
    data["summary"] += " " + addition["summary"]
    data["note"] += " Closed manually on 2026-10-02 after the overview-like-play revision (repo MCP still down)."
    listed = {file for files in data["files"].values() for file in files}
    for kind, files in addition["files"].items():
        data["files"][kind] += [file for file in files if file not in listed]
else:
    raise SystemExit(f"unknown verb: {verb}")

path.write_bytes((json.dumps(data, ensure_ascii=False, indent=2) + "\n").replace("\n", "\r\n").encode("utf-8"))
sys.stdout.buffer.write(f"{verb}: status {data['status']}, {sum(len(files) for files in data['files'].values())} files\n".encode("utf-8"))
