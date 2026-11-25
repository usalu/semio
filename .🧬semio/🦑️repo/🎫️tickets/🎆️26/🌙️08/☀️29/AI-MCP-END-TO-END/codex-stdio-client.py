"""🔭 Queries a standalone Codex app-server's MCP inventory without opening a chat."""
import json
import queue
import subprocess
import sys
import threading
from pathlib import Path

repo = Path.cwd()
ticket = Path(__file__).resolve().parent
entry = json.loads((repo/".mcp.json").read_text())["mcpServers"]["semio"]
config = ["-c", "mcp_servers.repo.enabled=false"]
if len(sys.argv) > 1:
    args = entry["args"]
    words = ["stdio", *args[args.index("--folder"):], "--no-bridge", "--host-driver-policy", str(Path(args[-1]).with_name("🏛️host-driver-policy.json"))]
    config += ["-c", "mcp_servers.semio.command="+json.dumps(str(Path(sys.argv[1]).resolve())), "-c", "mcp_servers.semio.args="+json.dumps(words, ensure_ascii=False), "-c", "mcp_servers.semio.env.SEMIO_PLUGIN_CATALOG="+json.dumps(str(repo / args[args.index("--catalog")+1]), ensure_ascii=False)]
config += ["-c", "mcp_servers.semio.cwd="+json.dumps(str(repo))]
inbox = queue.Queue()
with (ticket/"🗑️generated/codex-stdio-client.stderr").open("w") as err:
    child = subprocess.Popen(["codex","app-server",*config],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=err,text=True)
    def read():
        for line in child.stdout:
            inbox.put(json.loads(line))
        inbox.put(None)
    threading.Thread(target=read,daemon=True).start()
    def send(message):
        child.stdin.write(json.dumps(message)+"\n")
        child.stdin.flush()
    def request(ident, method, params):
        send({"id":ident, "method":method, "params":params})
        while True:
            row=inbox.get(timeout=20)
            if row is None: raise RuntimeError("Codex exited")
            if row.get("id")==ident:
                if "error" in row: raise RuntimeError(row["error"])
                return row["result"]
    try:
        request(1,"initialize",{"clientInfo":{"name":"semio-stdio-check","version":"1"},"capabilities":{"experimentalApi":True}})
        send({"method":"initialized"})
        status=request(2,"mcpServerStatus/list",{"serverName":"semio","detail":"full"})
        for server in status["data"]:
            if server["name"]=="semio":
                names=set(server["tools"])
                print("Codex discovered",len(names),"semio tools",flush=True)
                assert "context_resolve" in names
                break
        else: raise RuntimeError("no semio server")
    finally:
        child.stdin.close()
        try: child.wait(timeout=3)
        except subprocess.TimeoutExpired:
            child.terminate()
            child.wait(timeout=10)
