"""🔬️ Drives a fresh semio MCP server over stdio and runs JSON-RPC calls given as JSON lines.

Usage: python3 🐍️probe.py calls.jsonl [timeout_seconds]
Each line of calls.jsonl is {"name": "<tool>", "arguments": {...}} or {"method": "<rpc>", "params": {...}}.
"""
import json, os, subprocess, sys, threading, time, queue

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
ARGS = json.load(open(os.path.join(ROOT, ".mcp.json")))["mcpServers"]["semio"]


def main():
    calls = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
    timeout = float(sys.argv[2]) if len(sys.argv) > 2 else 600
    proc = subprocess.Popen([ARGS["command"], *ARGS["args"]], cwd=ROOT, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=open(os.environ.get("PROBE_STDERR") or os.path.join(os.path.dirname(__file__), "🗑️generated", os.environ.get("PROBE_STDERR", "probe-stderr.txt")), "w"), text=True, bufsize=1)
    lines = queue.Queue()
    threading.Thread(target=lambda: [lines.put(l) for l in proc.stdout], daemon=True).start()

    def send(obj):
        proc.stdin.write(json.dumps(obj) + "\n")
        proc.stdin.flush()

    def wait(id_, deadline):
        while time.time() < deadline:
            try:
                msg = json.loads(lines.get(timeout=1))
            except queue.Empty:
                continue
            if msg.get("id") == id_:
                return msg
        return {"error": "timeout"}

    deadline = time.time() + timeout
    send({"jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "probe", "version": "0"}}})
    wait(0, deadline)
    send({"jsonrpc": "2.0", "method": "notifications/initialized"})
    for i, call in enumerate(calls, 1):
        if "sleep" in call:
            time.sleep(call["sleep"])
            continue
        if "method" in call:
            send({"jsonrpc": "2.0", "id": i, "method": call["method"], "params": call.get("params", {})})
        else:
            send({"jsonrpc": "2.0", "id": i, "method": "tools/call", "params": {"name": call["name"], "arguments": call.get("arguments", {})}})
        started = time.time()
        reply = wait(i, deadline)
        print(json.dumps({"call": call, "seconds": round(time.time() - started, 2), "reply": reply}), flush=True)
    proc.terminate()


if __name__ == "__main__":
    main()
