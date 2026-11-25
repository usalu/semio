"""🧭 Drive the configured semio server over stdio and fail on any invalid exchange.

Usage: python3 -I semio-mcp-journey.py <repo-root> <folder> [ready-seconds]
Set SEMIO_OS_MCP_BIN to select a prebuilt native server for the launcher, or
SEMIO_JOURNEY_NATIVE_BIN to test that executable directly.
"""
import json
import os
import subprocess
import sys
import threading
import time


class JourneyError(Exception):
    pass


def result_of(frame):
    if not isinstance(frame, dict) or frame.get("jsonrpc") != "2.0":
        raise JourneyError(f"invalid JSON-RPC response: {frame!r}")
    if "error" in frame:
        raise JourneyError(f"JSON-RPC error: {frame['error']!r}")
    if not isinstance(frame.get("result"), dict):
        raise JourneyError(f"missing or invalid result: {frame!r}")
    return frame["result"]


def tool_result(frame):
    result = result_of(frame)
    if result.get("isError", False) is not False:
        raise JourneyError(f"tool error: {result!r}")
    if not isinstance(result.get("structuredContent"), dict) or not result["structuredContent"]:
        raise JourneyError(f"missing or invalid structuredContent: {result!r}")
    return result["structuredContent"]


def main():
    root, folder = sys.argv[1], sys.argv[2]
    ready_seconds = float(sys.argv[3]) if len(sys.argv) > 3 else 1200
    with open(f"{root}/.mcp.json", encoding="utf8") as source:
        config = json.load(source)["mcpServers"]["semio"]
    args = list(config["args"])
    args[args.index("--folder") + 1] = folder
    command = config["command"]
    env = os.environ.copy()
    if env.get("SEMIO_JOURNEY_NATIVE_BIN"):
        command = env["SEMIO_JOURNEY_NATIVE_BIN"]
        catalog = args[args.index("--catalog") + 1]
        env["SEMIO_PLUGIN_CATALOG"] = os.path.join(root, catalog)
        args = ["stdio", *args[args.index("--folder"):]]
        policy_dir = os.path.dirname(args[args.index("--operation-policy") + 1])
        args.extend(["--host-driver-policy", os.path.join(policy_dir, "🏛️host-driver-policy.json")])
    started = time.monotonic()
    frames, notes, faults = {}, [], []
    lock = threading.Condition()
    with open(f"{folder}.stderr", "w", encoding="utf8") as stderr:
        process = subprocess.Popen([command, *args, "--no-bridge", "--auto-approve", "all"], cwd=root, env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=stderr, text=True, bufsize=1)

        def pump():
            try:
                for line in process.stdout:
                    try:
                        frame = json.loads(line)
                        if not isinstance(frame, dict) or frame.get("jsonrpc") != "2.0":
                            raise ValueError(f"invalid frame: {frame!r}")
                        with lock:
                            if "id" in frame and "method" not in frame:
                                if frame["id"] in frames:
                                    raise ValueError(f"duplicate response id: {frame['id']!r}")
                                frames[frame["id"]] = frame
                            elif isinstance(frame.get("method"), str) and "id" not in frame:
                                notes.append(frame["method"])
                            else:
                                raise ValueError(f"unexpected frame: {frame!r}")
                            lock.notify_all()
                    except (ValueError, TypeError) as error:
                        with lock:
                            faults.append(f"invalid stdout {line[:160]!r}: {error}")
                            lock.notify_all()
            finally:
                with lock:
                    faults.append("server stdout closed")
                    lock.notify_all()

        reader = threading.Thread(target=pump, daemon=True)
        reader.start()
        next_id = 0

        def send(message):
            try:
                process.stdin.write(json.dumps(message) + "\n")
                process.stdin.flush()
            except (BrokenPipeError, OSError) as error:
                raise JourneyError(f"server input failed: {error}") from error

        def request(method, params, timeout):
            nonlocal next_id
            next_id += 1
            ident = next_id
            send({"jsonrpc": "2.0", "id": ident, "method": method, "params": params})
            with lock:
                lock.wait_for(lambda: ident in frames or faults, timeout)
                if faults:
                    raise JourneyError(f"{method}: {faults[0]}")
                if ident not in frames:
                    raise JourneyError(f"{method}: response timed out after {timeout}s")
                return frames.pop(ident)

        def step(name, operation):
            try:
                value = operation()
                print(f"PASS t+{time.monotonic() - started:6.1f}s {name} {str(value)[:200]}", flush=True)
                return value
            except (JourneyError, KeyError, TypeError, ValueError) as error:
                print(f"FAIL t+{time.monotonic() - started:6.1f}s {name}: {error}", flush=True)
                raise JourneyError(name) from error

        def listing(method, key, timeout):
            result = result_of(request(method, {}, timeout))
            rows = result.get(key)
            if not isinstance(rows, list) or any(not isinstance(row, dict) for row in rows):
                raise JourneyError(f"invalid {key}: {rows!r}")
            return rows

        def call(name, arguments):
            return tool_result(request("tools/call", {"name": name, "arguments": arguments}, 300))

        success = False
        try:
            def initialize():
                result = result_of(request("initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "semio-mcp-journey", "version": "1"}}, 30))
                info = result.get("serverInfo")
                if not isinstance(info, dict) or not isinstance(info.get("name"), str) or not isinstance(info.get("version"), str) or not isinstance(result.get("protocolVersion"), str) or not isinstance(result.get("capabilities"), dict):
                    raise JourneyError(f"invalid initialize result: {result!r}")
                return info

            info = step("initialize within the client connect timeout", initialize)
            send({"jsonrpc": "2.0", "method": "notifications/initialized"})
            if info["version"] == "warming":
                step("warm discovery answers at once", lambda: listing("tools/list", "tools", 30))
                def ready():
                    deadline = time.monotonic() + ready_seconds
                    while time.monotonic() < deadline:
                        with lock:
                            lock.wait_for(lambda: "notifications/tools/list_changed" in notes or faults, min(1, max(0, deadline - time.monotonic())))
                            if faults:
                                raise JourneyError(faults[0])
                            if "notifications/tools/list_changed" in notes:
                                return "announced"
                        with open(f"{folder}.stderr", encoding="utf8", errors="replace") as log:
                            if "[semio-os-mcp] unavailable:" in log.read():
                                raise JourneyError("native staging reported unavailable; see stderr")
                    raise JourneyError(f"no catalog notification within {ready_seconds}s")
                step("native catalog announced", ready)

            def gateway_tools():
                rows = listing("tools/list", "tools", 120)
                if any(not isinstance(row.get("name"), str) for row in rows):
                    raise JourneyError("tool without a string name")
                names = {row["name"] for row in rows}
                missing = {"capabilities_search", "artifact_create", "action_prepare", "action_invoke", "history_undo", "artifact_snapshot"} - names
                if missing:
                    raise JourneyError(f"missing tools: {sorted(missing)}")
                return f"{len(rows)} tools"

            step("tools/list returns the gateway tools", gateway_tools)
            step("prompts/list", lambda: listing("prompts/list", "prompts", 60))
            step("resources/list", lambda: listing("resources/list", "resources", 60))
            step("context_resolve", lambda: call("context_resolve", {"locale": "en"}))
            step("capabilities_search", lambda: call("capabilities_search", {"query": "edit the document"}))
            artifact = f"journey-{int(time.time())}"
            step("artifact_create", lambda: call("artifact_create", {"artifactId": artifact, "kind": "os.agent.probe/v1", "initial": {"note": "created by the semio MCP journey"}}))
            step("artifact_snapshot", lambda: call("artifact_snapshot", {"artifactId": artifact}))
            step("artifact_validate", lambda: call("artifact_validate", {"artifactId": artifact}))
            def undo_invalid():
                result = result_of(request("tools/call", {"name": "history_undo", "arguments": {"artifactId": artifact}}, 300))
                content = result.get("structuredContent")
                if result.get("isError") is not True or not isinstance(content, dict) or content.get("code") != "INPUT_INVALID" or "undoToken" not in json.dumps(content):
                    raise JourneyError(f"expected INPUT_INVALID naming undoToken: {result!r}")
                return content
            step("history_undo rejects missing undoToken", undo_invalid)
            process.stdin.close()
            def exit_cleanly():
                try:
                    code = process.wait(30)
                except subprocess.TimeoutExpired as error:
                    raise JourneyError("still running after 30s") from error
                reader.join(2)
                unexpected = [fault for fault in faults if fault != "server stdout closed"]
                if reader.is_alive() or unexpected or code != 0:
                    raise JourneyError(f"status={code}, reader_alive={reader.is_alive()}, faults={unexpected!r}")
                return f"status={code}"
            step("server exits on client EOF", exit_cleanly)
            success = True
        except JourneyError:
            pass
        finally:
            if process.poll() is None:
                process.kill()
            process.wait()
            if not process.stdin.closed:
                process.stdin.close()
        return 0 if success else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError, IndexError) as error:
        print(f"FAIL setup: {error}", file=sys.stderr, flush=True)
        sys.exit(1)
