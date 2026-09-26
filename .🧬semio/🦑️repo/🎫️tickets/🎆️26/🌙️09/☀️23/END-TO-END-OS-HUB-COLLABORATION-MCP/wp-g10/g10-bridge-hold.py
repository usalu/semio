"""🧷️ G10 bridge live proof: hold one stdio `semio-os-mcp` (any build) open with its bridge offered into a
private rendezvous dir, then optionally fill its `/bridge` listener with finished HTTP requests.

usage: python3 g10-bridge-hold.py <binary> <bridge-dir> <status-file> [fill-count]

Writes one JSON line per event to <status-file>: the listener url, the pid, and — when a fill count is
given — how many requests were answered, how many timed out, the CLOSED sockets the gateway holds, and
whether one more request after the fill is still answered. Holds stdin open until SIGTERM, then closes it (EOF) and records whether the offer file is gone.
"""
import json, os, re, signal, socket, subprocess, sys, time

binary, bridge_dir, status_path = sys.argv[1], sys.argv[2], sys.argv[3]
fill = int(sys.argv[4]) if len(sys.argv) > 4 else 0
status = open(status_path, "a", buffering=1)


def emit(**row):
    row["at"] = time.strftime("%H:%M:%S")
    status.write(json.dumps(row) + "\n")


env = dict(os.environ, S_AGENT_BRIDGE_DIR=bridge_dir)
err_path = status_path + ".stderr"
proc = subprocess.Popen([binary, "stdio", "--scopes", "workspace.read"], stdin=subprocess.PIPE, stdout=subprocess.DEVNULL, stderr=open(err_path, "a"), env=env, start_new_session=True)
url = None
deadline = time.time() + 60
while time.time() < deadline and url is None:
    time.sleep(0.25)
    found = re.search(r"bridge listening on ws://([0-9.]+):(\d+)/bridge", open(err_path).read())
    if found:
        url = (found.group(1), int(found.group(2)))
emit(event="started", pid=proc.pid, listener=f"{url[0]}:{url[1]}" if url else None)
if url is None:
    proc.terminate()
    sys.exit(1)


def one_request(timeout):
    started = time.time()
    try:
        with socket.create_connection(url, timeout=timeout) as conn:
            conn.settimeout(timeout)
            conn.sendall(b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
            head = conn.recv(256)
            return ("answered", head.split(b"\r\n", 1)[0].decode(errors="replace"), round((time.time() - started) * 1000))
    except socket.timeout:
        return ("timeout", None, round((time.time() - started) * 1000))
    except OSError as error:
        return ("error", str(error), round((time.time() - started) * 1000))


if fill:
    outcomes = {}
    for _ in range(fill):
        kind, line, _ = one_request(3)
        outcomes[kind] = outcomes.get(kind, 0) + 1
    time.sleep(1)
    closed = subprocess.run(["lsof", "-a", "-p", str(proc.pid), "-iTCP"], capture_output=True, text=True).stdout.count("CLOSED")
    kind, line, ms = one_request(5)
    emit(event="filled", fill=fill, outcomes=outcomes, closedSocketsHeld=closed, afterFill={"outcome": kind, "statusLine": line, "ms": ms})

def stop(*_):
    proc.stdin.close()
    try:
        proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        proc.terminate()
    emit(event="stopped", code=proc.returncode, offerLeft=os.path.exists(os.path.join(bridge_dir, "offers", f"{proc.pid}.json")))
    sys.exit(0)


signal.signal(signal.SIGTERM, stop)
while proc.poll() is None:
    time.sleep(1)
emit(event="exited", code=proc.returncode)
