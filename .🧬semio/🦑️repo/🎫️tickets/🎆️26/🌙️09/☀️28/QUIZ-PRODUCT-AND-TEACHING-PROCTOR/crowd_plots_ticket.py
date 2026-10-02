"""Reopen or close the ticket on disk for the crowd-after-submission revision (repo MCP down): `reopen` or `close <summary file>`.

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
        " Reopened 2026-10-02 (crowd after submission): by default the results of the others show only after a quiz is"
        " submitted; the learner can explicitly ask to see the distribution before; everything about the others is plotted"
        " (answer distributions per item, score distributions)."
    )
    data["note"] += " Reopened manually on 2026-10-02 (crowd after submission; repo and semio MCP servers failed to connect), see 📓️crowd-plots-report.md."
elif verb == "close":
    addition = json.loads(pathlib.Path(sys.argv[2]).read_text(encoding="utf-8"))
    data["status"] = "closed"
    data["summary"] += " " + addition["summary"]
    data["note"] += " Closed manually on 2026-10-02 after the crowd-after-submission revision (repo MCP still down)."
    listed = {file for files in data["files"].values() for file in files}
    for kind, files in addition["files"].items():
        data["files"][kind] += [file for file in files if file not in listed]
else:
    raise SystemExit(f"unknown verb: {verb}")

path.write_bytes((json.dumps(data, ensure_ascii=False, indent=2) + "\n").replace("\n", "\r\n").encode("utf-8"))
sys.stdout.buffer.write(f"{verb}: status {data['status']}, {sum(len(files) for files in data['files'].values())} files\n".encode("utf-8"))
