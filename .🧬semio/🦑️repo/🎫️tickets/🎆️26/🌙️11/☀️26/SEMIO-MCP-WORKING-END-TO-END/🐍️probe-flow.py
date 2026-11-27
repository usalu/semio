"""🔬️ Scripted flow over a fresh semio MCP server: calls.jsonl lines may carry `"as": "name"` to keep the reply and use `"${name.path.to.field}"` in later arguments.

Calls may carry `attempts`/`interval` plus `until` (substring the reply must contain; default: no error) to poll until a refreshed server answers.\n\nUsage: python3 🐍️probe-flow.py flow.jsonl [timeout_seconds]
"""
import json, os, re, subprocess, sys, threading, time, queue

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 7))
ARGS = json.load(open(os.path.join(ROOT, ".mcp.json")))["mcpServers"]["semio"]


def reply_data(reply):
    result = reply.get("result", {})
    return result.get("structuredContent") or result


def lookup(saved, path):
    node = saved
    try:
        for part in path.split("."):
            node = node[int(part)] if isinstance(node, list) else node[part]
    except (KeyError, IndexError, ValueError, TypeError):
        return f"<missing {path}>"
    return node


def substitute(value, saved):
    if isinstance(value, str):
        whole = re.fullmatch(r"\$\{([^}]+)\}", value)
        if whole:
            return lookup(saved, whole.group(1))
        return re.sub(r"\$\{([^}]+)\}", lambda match: str(lookup(saved, match.group(1))), value)
    if isinstance(value, list):
        return [substitute(item, saved) for item in value]
    if isinstance(value, dict):
        return {key: substitute(item, saved) for key, item in value.items()}
    return value


def main():
    calls = [json.loads(line) for line in open(sys.argv[1]) if line.strip() and not line.startswith("#")]
    timeout = float(sys.argv[2]) if len(sys.argv) > 2 else 600
    command, env = [ARGS["command"], *ARGS["args"]], dict(os.environ)
    if os.environ.get("NATIVE_BINARY"):
        flags = ARGS["args"]
        pick = lambda name: flags[flags.index(name) + 1]
        policy = os.path.join(os.path.dirname(os.path.join(ROOT, pick("--operation-policy"))), "🏛️host-driver-policy.json")
        command = [os.environ["NATIVE_BINARY"], "stdio", "--folder", pick("--folder"), "--scopes", pick("--scopes"), "--operation-policy", pick("--operation-policy"), "--host-driver-policy", policy]
        env.update(json.load(open(os.environ["NATIVE_ENV"])))
    proc = subprocess.Popen(command, cwd=ROOT, env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=open(os.environ.get("PROBE_STDERR") or os.path.join(HERE, "🗑️generated", "probe-stderr.txt"), "w"), text=True, bufsize=1)
    lines = queue.Queue()
    threading.Thread(target=lambda: [lines.put(line) for line in proc.stdout], daemon=True).start()

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
    saved = {}
    for i, call in enumerate(calls, 1):
        call = {**call, "arguments": substitute(call.get("arguments", {}), saved), "params": substitute(call.get("params", {}), saved)}
        if "method" in call:
            send({"jsonrpc": "2.0", "id": i, "method": call["method"], "params": call["params"]})
        else:
            send({"jsonrpc": "2.0", "id": i, "method": "tools/call", "params": {"name": call["name"], "arguments": call["arguments"]}})
        started = time.time()
        reply = wait(i, deadline)
        attempts = 0
        while attempts < call.get("attempts", 0) and not (call.get("until") in json.dumps(reply, ensure_ascii=False) if "until" in call else not reply.get("result", {}).get("isError") and "error" not in reply):
            attempts += 1
            time.sleep(call.get("interval", 20))
            i2 = 100000 + i * 1000 + attempts
            send({"jsonrpc": "2.0", "id": i2, "method": "tools/call", "params": {"name": call["name"], "arguments": call["arguments"]}})
            reply = wait(i2, deadline)
        if "as" in call:
            saved[call["as"]] = reply_data(reply)
        shown = {"name": call.get("name") or call.get("method"), "arguments": call["arguments"]}
        print(json.dumps({"call": shown, "seconds": round(time.time() - started, 2), "isError": reply.get("result", {}).get("isError"), "reply": reply_data(reply) if "result" in reply else reply}, ensure_ascii=False)[:3500], flush=True)
    proc.terminate()


if __name__ == "__main__":
    main()
