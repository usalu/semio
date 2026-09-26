"""🔎️ G10: asks a headless `semio-os-mcp stdio --folder` for "set adjacency kind" and reports whether architect's
`setAdjacencyKind` appears exactly once (audit P1-6: a duplicate capability id degraded `capabilities_search`).
usage: python3 g10-adjacency-probe.py <binary> <empty-folder>"""
import json, subprocess, sys
binary, folder = sys.argv[1], sys.argv[2]
proc = subprocess.Popen([binary, "stdio", "--folder", folder, "--no-bridge", "--scopes", "workspace.read"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
def call(i, method, params):
    proc.stdin.write(json.dumps({"jsonrpc": "2.0", "id": i, "method": method, "params": params}) + "\n"); proc.stdin.flush()
    while True:
        line = proc.stdout.readline()
        if not line: raise SystemExit("gateway closed")
        msg = json.loads(line)
        if msg.get("id") == i: return msg
call(1, "initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "g10-adjacency", "version": "1"}})
proc.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n"); proc.stdin.flush()
answer = call(2, "tools/call", {"name": "capabilities_search", "arguments": {"query": "set adjacency kind", "limit": 50}})
content = answer["result"].get("structuredContent") or {}
hits = [hit.get("capabilityId") or hit.get("id") for hit in content.get("results", content.get("hits", []))]
print(json.dumps({"hits": len(hits), "adjacency": [h for h in hits if h and "djacency" in h], "duplicates": sorted({h for h in hits if hits.count(h) > 1}), "isError": answer["result"].get("isError")}))
proc.stdin.close(); proc.wait(timeout=30)
