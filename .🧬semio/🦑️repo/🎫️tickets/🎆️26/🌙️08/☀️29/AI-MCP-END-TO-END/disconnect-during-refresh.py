"""🔌 Checks a clean MCP disconnect while the configured launcher provisions dependencies."""
from collections import deque
import json
import queue
import subprocess
import threading

entry = json.load(open(".mcp.json", encoding="utf-8"))["mcpServers"]["semio"]
process = subprocess.Popen([entry["command"], *entry["args"]], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
events = queue.Queue()
diagnostics = deque(maxlen=22)
def pump(stream, kind):
    for line in stream:
        if kind == "progress":
            diagnostics.append(line.rstrip())
        events.put((kind, line))
    events.put((kind, None))

for stream, kind in [(process.stdout, "rpc"), (process.stderr, "progress")]:
    threading.Thread(target=pump, args=(stream, kind), daemon=True).start()
try:
    process.stdin.write(json.dumps({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "disconnect-test", "version": "1"}}}) + "\n")
    process.stdin.flush()
    initialized = refreshing = False
    while not (initialized and refreshing):
        kind, line = events.get(timeout=30)
        if line is None:
            raise RuntimeError("unexpected EOF before disconnect")
        if kind == "rpc":
            result = json.loads(line)
            initialized |= result.get("result", {}).get("serverInfo", {}).get("name") == "semio"
        else:
            refreshing |= "repo dependencies ensure" in line
    process.stdin.close()
    status = process.wait(timeout=30)
    print(f"initialized={initialized} refreshing={refreshing} disconnect exit={status}")
    if status:
        print("\n".join(diagnostics))
        raise SystemExit(status)
finally:
    if process.poll() is None:
        process.kill()
        process.wait()
