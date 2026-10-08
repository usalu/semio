#!/usr/bin/env python3
"""Fixture-only: for N retained sessions, how many does a freshly attached view list? (Sessions frame size vs the 8 KiB local socket buffer)."""
import importlib.util, json, os, re, socket, struct, subprocess, sys, threading, time
spec = importlib.util.spec_from_file_location("audit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "drive-runtime-audit.py")); audit = importlib.util.module_from_spec(spec); spec.loader.exec_module(audit)
FIX = audit.FIX
for n in [int(x) for x in sys.argv[1:]]:
    audit.fixture_prepare()
    subprocess.run([audit.binary(), "daemon", "start", "--root", FIX], cwd=FIX, capture_output=True, timeout=20); time.sleep(0.4)
    path = audit.fixture_socket()
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM); s.connect(path); s.settimeout(0.2)
    stop = [False]
    def drain():
        while not stop[0]:
            try:
                if not s.recv(1 << 20): break
            except socket.timeout: pass
            except OSError: break
    threading.Thread(target=drain, daemon=True).start()
    for i in range(n):
        p = json.dumps({"type": "spawn", "session_id": "task-%d-%019d" % (10000 + i, i), "command": {"cmd": "echo", "args": ["session %d" % i], "cwd": FIX, "env": [["NX_NATIVE_COMMAND_RUNNER", "false"], ["NX_TUI", "false"]], "cols": 100, "rows": 30}}).encode()
        s.sendall(struct.pack("<IB", len(p) + 1, 1) + p); time.sleep(0.03)
    time.sleep(1.0); stop[0] = True; s.close()
    view = audit.fixture_view("sweep-%d" % n, 160, 48); view.pump(2.5)
    shown = len(re.findall(r"\[exited", view.vt.text()))
    # the Overview shows at most (rows - 8); count tab strip independent: ask the status CLI too
    status = subprocess.run([audit.binary(), "daemon", "status", "--root", FIX], cwd=FIX, capture_output=True, text=True, timeout=15).stdout.strip()
    ev = sum(1 for _ in open(FIX + "/.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/events.jsonl"))
    print("N=%3d retained (events.jsonl lines %d): Overview rows visible in view: %d (screen has room for ~38); footer: %s; `daemon status`: %s" % (n, ev, shown, audit.footer(view).strip()[:40], status[:60]))
    view.finish(); audit.fixture_stop()
