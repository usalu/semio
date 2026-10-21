"""Real light calls through the MCP against the real repository root: leases_status and capacity_status (no --tool, no discovery). Usage: python3 real-call.py <repo-binary>"""
import json, os, subprocess, sys, time
env = dict(os.environ, SEMIO_REPO_MCP_CLIENT="client", SEMIO_LANGUAGE="en")
proc = subprocess.Popen(["nice", "-n", "10", sys.argv[1], "mcp"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, env=env, cwd=os.getcwd(), text=True, bufsize=1)
n = 0
def rpc(method, params=None):
    global n
    n += 1
    proc.stdin.write(json.dumps({"jsonrpc": "2.0", "id": n, "method": method, "params": params or {}}) + "\n"); proc.stdin.flush()
    notes = 0
    while True:
        message = json.loads(proc.stdout.readline())
        if message.get("id") == n:
            return message, notes
        notes += 1
rpc("initialize", {"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "real-call", "version": "1"}})
proc.stdin.write('{"jsonrpc":"2.0","method":"notifications/initialized"}\n'); proc.stdin.flush()
for tool in sys.argv[2:]:
    started = time.time()
    reply, notes = rpc("tools/call", {"name": tool, "arguments": {"agent": "repo-mcp-build"}, "_meta": {"progressToken": tool}})
    structured = reply["result"].get("structuredContent", {})
    data = json.dumps(structured.get("data"))[:240] if structured.get("data") is not None else structured.get("stderr", "")[-300:]
    print(tool, "ok=%s exit=%s %.1fs progress=%d command=%s agent=%s" % (structured.get("ok"), structured.get("exitCode"), time.time() - started, notes, structured.get("command"), structured.get("agent", {}).get("id")))
    print("   ", data)
proc.stdin.close()
