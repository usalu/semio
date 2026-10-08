#!/usr/bin/env python3
"""Read-only probe of the workspace daemon: connects to the Unix socket, prints the greeting and the session table for a short window. Sends only `list`."""
import json, os, socket, struct, sys, time, glob, hashlib
sock_path = sys.argv[1]
seconds = float(sys.argv[2]) if len(sys.argv) > 2 else 2.0
s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM); s.settimeout(2.0); s.connect(sock_path)
def frames(buf):
    out = []
    while len(buf) >= 4:
        n = struct.unpack("<I", buf[:4])[0]
        if len(buf) < 4 + n: break
        out.append((buf[4], bytes(buf[5:4+n]))); del buf[:4+n]
    return out
buf = bytearray(); t0 = time.time(); got = []
def send(msg):
    p = json.dumps(msg).encode(); s.sendall(struct.pack("<IB", len(p)+1, 1) + p)
s.settimeout(0.2)
sent = False; counts = {}
while time.time() - t0 < seconds:
    try:
        d = s.recv(65536)
        if not d: print("EOF from daemon after", round(time.time()-t0,2)); break
        buf += d
    except socket.timeout: pass
    for kind, payload in frames(buf):
        if kind == 1:
            m = json.loads(payload); counts[m["type"]] = counts.get(m["type"], 0) + 1; got.append((round(time.time()-t0,2), m))
        else:
            sid_len = struct.unpack("<H", payload[:2])[0]; sid = payload[2:2+sid_len].decode(); counts["output:"+sid] = counts.get("output:"+sid, 0) + len(payload) - 2 - sid_len
    if not sent: send({"type": "list"}); sent = True
for t, m in got:
    if m["type"] == "attached": print(t, "attached daemon_pid", m["daemon_pid"])
    elif m["type"] == "sessions":
        print(t, "sessions:", len(m["sessions"]))
        for x in m["sessions"]:
            c = x["command"]
            print("  ", x["session_id"], x["status"], "pid", x["pid"], "code", x["code"], c["cols"], "x", c["rows"], "|", c["cmd"], " ".join(c["args"])[:100], "| env", len(c["env"]))
    else: print(t, m["type"], json.dumps(m)[:200])
print("counts", counts)
