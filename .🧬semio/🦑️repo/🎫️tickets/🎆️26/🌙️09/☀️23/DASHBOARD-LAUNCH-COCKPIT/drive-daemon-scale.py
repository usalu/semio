#!/usr/bin/env python3
"""Fixture-only: fills the daemon with 127 finished sessions through the protocol, checks the eviction order at the 128-session cap, then attaches a real view and measures how long the restore takes."""
import importlib.util, json, os, socket, struct, subprocess, sys, threading, time
spec = importlib.util.spec_from_file_location("audit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "drive-runtime-audit.py")); audit = importlib.util.module_from_spec(spec); spec.loader.exec_module(audit)
FIX = audit.FIX
audit.fixture_prepare()
subprocess.run([audit.binary(), "daemon", "start", "--root", FIX], cwd=FIX, capture_output=True, timeout=20); time.sleep(0.5)
path = audit.fixture_socket()
s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM); s.connect(path); s.settimeout(0.2)
stop = False
def drain():
    while not stop:
        try:
            if not s.recv(1 << 20): break
        except socket.timeout: pass
        except OSError: break
threading.Thread(target=drain, daemon=True).start()
def send(msg):
    p = json.dumps(msg).encode(); s.sendall(struct.pack("<IB", len(p) + 1, 1) + p)
def spawn(sid, text):
    send({"type": "spawn", "session_id": sid, "command": {"cmd": "echo", "args": [text], "cwd": FIX, "env": [], "cols": 100, "rows": 30}})
spawn("task-99999-1783675878398726000", "OLDEST-BY-TIME-BUT-HIGHEST-KEY"); time.sleep(0.1)
for i in range(127):
    spawn("task-10000-%019d" % i, "session %d" % i); time.sleep(0.03)
time.sleep(1.5)
ids = lambda: [x[0] for x in (audit.daemon_sessions() or [])]
before = audit.daemon_sessions(); print("sessions held:", len(before or []))
spawn("task-50000-1783675999999999999", "NEWEST"); time.sleep(0.5)
after = audit.daemon_sessions() or []
names = {x[0] for x in after}
print("after the 129th spawn: held", len(after), "| oldest-by-time session still present:", any("OLDEST" in str(x) or x[0].endswith("398726000") for x in after), "| evicted ids:", sorted({x[0] for x in (before or [])} - names))
stop = True; s.close()
view = audit.fixture_view("scale-reattach", 160, 48)
t0 = view.now(); view.pump(0.2); view.snapshot("00")
view.pump(30, quiet=1.5)
settle = max((t for t, n in view.timeline), default=0)
print("view restore: output stopped %.1fs after launch; bytes %d; footer %s; rss %d MB" % (settle, len(view.raw), audit.footer(view).strip()[-60:], audit.rss_kb(view.pid) // 1024))
view.snapshot("01-settled")
start = view.now(); res = audit.measure_key(view, audit.DOWN, quiet=0.1, limit=10); print("one Down key after restore: first byte %.0f ms, settled %.0f ms" % (res[0] * 1000, res[1] * 1000))
n = 0
for _ in range(140):
    view.send(audit.TAB); view.pump(0.04); n += 1
print("after 140 Tab presses the view is alive:", view.alive())
view.finish(); audit.fixture_stop()
