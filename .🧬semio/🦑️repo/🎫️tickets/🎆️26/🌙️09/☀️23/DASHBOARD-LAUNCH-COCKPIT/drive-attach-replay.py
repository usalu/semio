#!/usr/bin/env python3
"""Read-only: attach to a daemon socket like a view does and read the replay as fast as Python can; reports whether the daemon drops the client mid-replay."""
import json, socket, struct, sys, time
path, delay = sys.argv[1], float(sys.argv[2]) if len(sys.argv) > 2 else 0.0
for attempt in range(3):
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM); s.settimeout(0.5); s.connect(path)
    p = json.dumps({"type": "attach", "client_id": "audit-replay"}).encode(); s.sendall(struct.pack("<IB", len(p) + 1, 1) + p)
    t0 = time.time(); total = 0; frames = 0; types = []; buf = bytearray(); outcome = "timeout"
    while time.time() - t0 < 6:
        if delay: time.sleep(delay)
        try: d = s.recv(1 << 20)
        except socket.timeout: continue
        if not d: outcome = "EOF (dropped by daemon)"; break
        total += len(d); buf += d
        while len(buf) >= 4:
            n = struct.unpack("<I", buf[:4])[0]
            if len(buf) < 4 + n: break
            kind, body = buf[4], bytes(buf[5:4 + n]); del buf[:4 + n]; frames += 1
            if kind == 1: types.append(json.loads(body)["type"])
        if "replay_complete" in types: outcome = "replay_complete"; break
    print("attempt", attempt + 1, "delay", delay, "->", outcome, "after %.2fs" % (time.time() - t0), "bytes", total, "frames", frames, "control", types)
    s.close(); time.sleep(0.3)
