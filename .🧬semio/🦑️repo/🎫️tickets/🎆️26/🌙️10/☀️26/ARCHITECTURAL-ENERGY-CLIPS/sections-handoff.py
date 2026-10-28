"""🤝 Record section completion and retain the shared ticket for already active work."""

from pathlib import Path
import json
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding="utf-8")
TICKET = Path(__file__).resolve().parent
WORKSPACE = next(parent for parent in TICKET.parents if (parent / "🧰️framework").is_dir())
BINARY = (WORKSPACE / ".🧬semio/🦑️repo/native-repo-path").read_text(encoding="utf-8").strip()
FILES = sorted({Path(path).resolve().relative_to(WORKSPACE).as_posix() for path in
                re.findall(r"^- `([^`]+)`", (TICKET / "files.md").read_text(encoding="utf-8"), re.MULTILINE)})
SUMMARY = (TICKET / "closing-summary.md").read_text(encoding="utf-8").removeprefix("Verified Complete\n").strip()
PROCESS = subprocess.Popen([BINARY, "mcp"], cwd=WORKSPACE, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=subprocess.DEVNULL, text=True, encoding="utf-8")


def call(identifier, method, params):
    """📨 Use sequential initialized repository MCP requests without changing another worker's files."""
    PROCESS.stdin.write(json.dumps({"jsonrpc": "2.0", "id": identifier, "method": method, "params": params}) + "\n")
    PROCESS.stdin.flush()
    while True:
        line = PROCESS.stdout.readline()
        if not line:
            raise RuntimeError("Repository MCP ended before the handoff completed")
        row = json.loads(line)
        if row.get("id") == identifier:
            if "error" in row or row.get("result", {}).get("isError"):
                raise RuntimeError(row)
            return row["result"]


try:
    call(1, "initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                           "clientInfo": {"name": "codex", "version": "1"}})
    PROCESS.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
    PROCESS.stdin.flush()
    if json.loads((TICKET / "🎫️ticket.json").read_text(encoding="utf-8"))["status"] != "open":
        call(2, "tools/call", {"name": "ticket_reopen", "arguments": {
            "path": "26/10/26/ARCHITECTURAL-ENERGY-CLIPS", "client": "codex", "llm": "codex",
            "prompt": "Record the already verified uniform section correction after another concurrent owner completed its ticket interaction. No new source work is assigned.",
            "no_management": True}})
    closed = call(3, "tools/call", {"name": "ticket_close", "arguments": {
        "path": "26/10/26/ARCHITECTURAL-ENERGY-CLIPS", "title": "Architectural Energy Clips",
        "summary": SUMMARY, "files": FILES, "no_management": True}})
    reopened = call(4, "tools/call", {"name": "ticket_reopen", "arguments": {
        "path": "26/10/26/ARCHITECTURAL-ENERGY-CLIPS", "client": "codex", "llm": "codex",
        "prompt": "Uniform section correction is complete and verified; see sections.md. Keep this shared ticket open for the already active power-first and silhouette verification work. Its generated outputs were recreated after section-output cleanup and remain preserved for their owner. No new task is being assigned; final shared-output cleanup belongs to the existing worker when it finishes.",
        "no_management": True}})
    print(json.dumps({"recorded_completion": closed.get("structuredContent", {}).get("status"),
                      "shared_ticket": reopened.get("structuredContent", {}).get("status"),
                      "files": len(FILES)}, ensure_ascii=False))
finally:
    PROCESS.terminate()
