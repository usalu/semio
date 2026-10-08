#!/usr/bin/env python3
"""Fixture-only: speaks the daemon protocol directly (no TUI) to measure PTY->client throughput, replay size and slow-client behaviour."""
import json, os, socket, struct, subprocess, sys, time, signal
sys.path.insert(0, os.path.dirname(__file__))
import importlib.util
spec = importlib.util.spec_from_file_location("audit", os.path.join(os.path.dirname(__file__), "drive-runtime-audit.py")); audit = importlib.util.module_from_spec(spec); spec.loader.exec_module(audit)
FIX = audit.FIX
audit.fixture_prepare()
subprocess.run([audit.binary(), "daemon", "start", "--root", FIX], cwd=FIX, capture_output=True, timeout=20)
time.sleep(0.5)
path = audit.fixture_socket(); print("socket", path)

def connect():
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM); s.settimeout(0.5); s.connect(path); return s
def send(s, msg):
    p = json.dumps(msg).encode(); s.sendall(struct.pack("<IB", len(p) + 1, 1) + p)

def run(command, seconds, read_delay=0.0, label=""):
    s = connect(); buf = bytearray(); sid = "audit-%d-%d" % (os.getpid(), int(time.time() * 1000))
    send(s, {"type": "attach", "client_id": "audit"})
    send(s, {"type": "spawn", "session_id": sid, "command": {"cmd": "sh", "args": ["-c", command], "cwd": FIX, "env": [], "cols": 156, "rows": 40}})
    t0 = time.time(); total = 0; marks = []; text = b""; last = t0; dropped = False
    while time.time() - t0 < seconds:
        if read_delay: time.sleep(read_delay)
        try: data = s.recv(1 << 20)
        except socket.timeout: data = None
        except OSError as e: print(label, "recv error", e); break
        if data == b"": dropped = True; print(label, "EOF: the daemon dropped this client at %.2fs" % (time.time() - t0)); break
        if data: buf += data
        while len(buf) >= 4:
            n = struct.unpack("<I", buf[:4])[0]
            if len(buf) < 4 + n: break
            kind, body = buf[4], bytes(buf[5:4 + n]); del buf[:4 + n]
            if kind == 2:
                idl = struct.unpack("<H", body[:2])[0]; d = body[2 + idl:]; total += len(d); text += d[-64:]
                if len(marks) == 0 or time.time() - marks[-1][0] >= 1: marks.append((time.time(), total))
        if b"HEAVY-DONE" in text[-200:]: break
    el = time.time() - t0
    print(label, "received %d bytes in %.1fs = %.0f KB/s; done=%s dropped=%s; per-second:" % (total, el, total / el / 1024, b"HEAVY-DONE" in text, dropped), [(round(a - t0), b // 1024) for a, b in marks][:12])
    s.close(); return sid

run("yes line-of-heavy-output | head -200000; echo HEAVY-DONE", 90, 0.0, "fast reader (python blocking recv):")
run("yes line-of-heavy-output | head -200000; echo HEAVY-DONE", 90, 0.08, "slow reader (80 ms between reads, like the view's UI loop):")
run("i=0; while [ $i -lt 20000 ]; do echo line-$i-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx; i=$((i+1)); done; echo HEAVY-DONE", 90, 0.0, "shell loop 20000 lines (builder-like):")
# a client that stops reading entirely: does the daemon cut it?
s = connect(); send(s, {"type": "attach", "client_id": "stalled"}); time.sleep(0.3)
sid = "audit-stall-%d" % os.getpid()
send(s, {"type": "spawn", "session_id": sid, "command": {"cmd": "sh", "args": ["-c", "yes line-of-heavy-output | head -400000; echo HEAVY-DONE"], "cwd": FIX, "env": [], "cols": 156, "rows": 40}})
time.sleep(8)
try:
    s.settimeout(0.5); got = 0
    while True:
        d = s.recv(1 << 20)
        if not d: print("stalled client (did not read for 8 s): EOF after reading", got, "bytes -> daemon dropped it"); break
        got += len(d)
        if got > 40_000_000: print("stalled client survived; read", got); break
except socket.timeout: print("stalled client: still connected after 8 s, buffered", got, "bytes")
s.close()
audit.fixture_stop()
