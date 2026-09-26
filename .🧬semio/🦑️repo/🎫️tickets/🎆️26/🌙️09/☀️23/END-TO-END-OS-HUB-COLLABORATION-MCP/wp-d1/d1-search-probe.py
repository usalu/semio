"""🔎️ D1: asks a real `semio-os-mcp` stdio gateway `capabilities_search` over a chosen descriptor root (`SEMIO_REPO_ROOT`), so the
before/after score spread is the gateway's own BM25, not a re-implementation. Usage: d1-search-probe.py <binary> <root> <query> <out.json>"""
import json, os, shutil, subprocess, sys, tempfile, time

binary, root, query, out = sys.argv[1:5]
space = tempfile.mkdtemp(prefix="s12-d1-search-space-", dir="/Users/ueli/Documents/semio/.🧬semio/🌐hub")
env = dict(os.environ, SEMIO_REPO_ROOT=root)
process = subprocess.Popen([binary, "stdio", "--folder", space, "--no-bridge", "--scopes", "workspace.read"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, env=env, text=True)

def send(message):
    process.stdin.write(json.dumps(message) + "\n")
    process.stdin.flush()

def receive(identifier, deadline=120):
    started = time.time()
    while time.time() - started < deadline:
        line = process.stdout.readline()
        if not line:
            break
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        if message.get("id") == identifier:
            return message
    raise SystemExit(f"no response for id {identifier}")

send({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "d1-search-probe", "version": "1"}}})
receive(1)
send({"jsonrpc": "2.0", "method": "notifications/initialized"})
send({"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "capabilities_search", "arguments": {"query": query, "limit": 100}}})
answer = receive(2)
process.terminate()
process.wait(timeout=10)
shutil.rmtree(space, ignore_errors=True)
structured = answer["result"].get("structuredContent") or {}
json.dump(structured, open(out, "w"), ensure_ascii=False, indent=1)
results = structured.get("results", [])
scores = [round(hit["score"], 4) for hit in results]
top15 = results[:15]
print(f"root={root} query={query!r} total={structured.get('total')} returned={len(results)} distinct_scores={len(set(scores))}/{len(scores)} empty_descriptions_top15={sum(1 for hit in top15 if not hit.get('description'))}/15")
for hit in top15:
    print(f"  {hit['score']:.4f}  {hit['capabilityId']}  {'(empty)' if not hit.get('description') else hit['description'][:70]}")
