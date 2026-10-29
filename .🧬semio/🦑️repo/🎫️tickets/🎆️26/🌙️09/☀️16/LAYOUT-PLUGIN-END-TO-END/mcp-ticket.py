import argparse
import json
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument("operation", choices=("reopen", "close"))
parser.add_argument("--input", type=Path)
args = parser.parse_args()
workspace = Path(__file__).resolve().parents[7]
process = subprocess.Popen(["repo", "mcp"], cwd=workspace, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
sequence = 0

def request(method, parameters):
    """📡 Sends one repository MCP request and retains its matching response."""
    global sequence
    sequence += 1
    process.stdin.write(json.dumps({"jsonrpc": "2.0", "id": sequence, "method": method, "params": parameters}) + "\n")
    process.stdin.flush()
    while True:
        line = process.stdout.readline()
        if not line:
            raise RuntimeError("Repository MCP closed before its response")
        response = json.loads(line)
        if response.get("id") == sequence:
            return response

try:
    request("initialize", {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "codex-layout", "version": "1"}})
    process.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
    process.stdin.flush()
    if args.operation == "reopen":
        request("resources/read", {"uri": "repo://goals"})
        parameters = {"path": "26/09/16/LAYOUT-PLUGIN-END-TO-END", "prompt": "Get layout plugin working end to end.", "client": "codex", "llm": "codex"}
    else:
        if args.input is None:
            parser.error("close requires --input with the final reviewable ticket payload")
        parameters = json.loads(args.input.read_text(encoding="utf-8"))
    result = request("tools/call", {"name": "ticket_" + args.operation, "arguments": parameters})
    print(json.dumps(result, ensure_ascii=False))
    if "error" in result or result.get("result", {}).get("isError"):
        raise SystemExit(1)
finally:
    process.terminate()
    process.wait(timeout=10)
