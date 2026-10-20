"""Black-box MCP client harness for a built `repo` executable (stdlib only; macOS, Linux).
Usage: python3 probe.py <repo-binary> <workdir> [--repo-root REAL_ROOT]
It copies the catalogue and the policy into <workdir> (a throwaway repository root: it gets a go.mod so the root resolves there), starts `repo mcp` in it and checks: start latency, initialize/instructions, tool list, task capability, an invalid call, a failing child (no build script in the throwaway root), progress, cancellation and the survival of the connection afterwards."""
import json, os, shutil, subprocess, sys, time

binary, work = sys.argv[1], os.path.abspath(sys.argv[2])
real = os.getcwd()
for index, argument in enumerate(sys.argv):
    if argument == "--repo-root":
        real = sys.argv[index + 1]
modules = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
files = [modules + "/📚️library/🏗️build/⚙️operations/🔣️catalog.json", modules + "/📚️library/🏗️build/🔣️policy.json", modules + "/🔌️mcp/🖼️assets/🏗️build-surface.json"]
shutil.rmtree(work, ignore_errors=True)
for relative in files:
    destination = os.path.join(work, relative)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    staged = os.path.join(os.path.dirname(os.path.abspath(__file__)), "staged", relative)
    shutil.copy2(staged if os.path.isfile(staged) else os.path.join(real, relative), destination)
open(os.path.join(work, "go.mod"), "w").write("module probe\n")
env = dict(os.environ, SEMIO_REPO_MCP_CLIENT="client", SEMIO_LANGUAGE="en")
t0 = time.time()
proc = subprocess.Popen(["nice", "-n", "10", binary, "mcp"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env, cwd=work, text=True, bufsize=1)
identifier = 0
def send(message):
    proc.stdin.write(json.dumps(message) + "\n")
    proc.stdin.flush()
def request(method, params=None, wait=60):
    global identifier
    identifier += 1
    mine = identifier
    send({"jsonrpc": "2.0", "id": mine, "method": method, "params": params or {}})
    notes = []
    while True:
        line = proc.stdout.readline()
        if not line:
            raise SystemExit("server closed: " + proc.stderr.read())
        message = json.loads(line)
        if message.get("id") == mine:
            return message, notes
        notes.append(message)
checks = []
def check(name, condition, detail=""):
    checks.append((name, bool(condition), detail))
    print(("PASS " if condition else "FAIL ") + name + (" :: " + str(detail) if detail and not condition else ""))

reply, _ = request("initialize", {"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "probe", "version": "1"}})
start = time.time() - t0
send({"jsonrpc": "2.0", "method": "notifications/initialized"})
result = reply["result"]
check("initialize answers fast", start < 1.0, f"{start:.3f}s")
check("instructions are tool-neutral", "default" in result["instructions"] and "remains the default" not in result["instructions"] and "Deutsch:" in result["instructions"])
check("tasks capability", "tasks" in result["capabilities"] and "list" in result["capabilities"]["tasks"])
t1 = time.time()
reply, _ = request("tools/list")
tools = {tool["name"]: tool for tool in reply["result"]["tools"]}
check("tools/list fast", time.time() - t1 < 0.5, f"{time.time() - t1:.3f}s")
check("tool count", len(tools) == 36, len(tools))
for name in ["run", "test", "run_many", "affected", "show_projects", "show_project", "graph", "leases_status", "capacity_status", "cache_status", "incremental_run", "concurrency_run", "ticket_open"]:
    check("tool " + name, name in tools)
run = tools.get("run", {})
check("annotations and execution", run.get("annotations", {}).get("readOnlyHint") is False and run.get("execution", {}).get("taskSupport") == "optional")
check("bilingual description", "EN: " in run.get("description", "") and "DE: " in run.get("description", ""))
reply, _ = request("tools/call", {"name": "run", "arguments": {"target": "nocolon"}})
check("invalid call is a readable tool error", reply["result"].get("isError") is True and "target" in reply["result"]["content"][0]["text"], reply)
t2 = time.time()
reply, notes = request("tools/call", {"name": "graph", "arguments": {"agent": "probe-agent", "language": "en"}, "_meta": {"progressToken": "p"}})
structured = reply["result"].get("structuredContent", {})
check("failing child gives a structured error result", reply["result"].get("isError") is True and structured.get("exitCode") not in (0, None) and structured.get("agent", {}).get("id") == "probe-agent", reply)
check("exact command reported", structured.get("command") == ["repo", "graph", "--json", "--graph-policy=current"], structured.get("command"))
check("progress notifications arrived", any(note.get("method") == "notifications/progress" for note in notes), len(notes))
print("   child failure took %.2fs; stderr tail: %s" % (time.time() - t2, structured.get("stderr", "")[-160:].replace("\n", " | ")))
reply, _ = request("tools/call", {"name": "show_projects", "arguments": {"agent": "probe-agent"}}, wait=60)
reply2, _ = request("ping")
check("connection survives failing children", "error" not in reply2)
reply, _ = request("resources/read", {"uri": "repo://build-surface"})
check("surface resource", "watch" in reply["result"]["contents"][0]["text"])
proc.stdin.close()
code = proc.wait(timeout=10)
err = proc.stderr.read()
check("clean exit at end of input", code == 0, code)
check("readiness line on stderr", "[repo-mcp] ready" in err, err[-200:])
print("stderr:", err.strip().splitlines()[0] if err.strip() else "")
sys.exit(0 if all(ok for _, ok, _ in checks) else 1)
