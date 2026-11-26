"""📏️ Sample the macOS process-tree resident size while a repository command runs."""
import ctypes
import json
import os
import signal
import subprocess
import sys
import time

lib = ctypes.CDLL('/usr/lib/libproc.dylib')
lib.proc_listpids.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p, ctypes.c_int]
lib.proc_pidinfo.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_uint64, ctypes.c_void_p, ctypes.c_int]
child = subprocess.Popen(sys.argv[2:])
for sig in (signal.SIGINT, signal.SIGTERM):
    signal.signal(sig, lambda number, _frame: child.send_signal(number))
peak = samples = 0
while child.poll() is None:
    pids = (ctypes.c_int * 65536)()
    count = max(0, lib.proc_listpids(1, 0, pids, ctypes.sizeof(pids))) // ctypes.sizeof(ctypes.c_int)
    rows = {}
    for pid in pids[:count]:
        if pid <= 0:
            continue
        info = ctypes.create_string_buffer(136)
        task = ctypes.create_string_buffer(96)
        if lib.proc_pidinfo(pid, 3, 0, info, len(info)) < 20 or lib.proc_pidinfo(pid, 4, 0, task, len(task)) < 16:
            continue
        rows[pid] = (ctypes.c_uint32.from_buffer(info, 16).value, ctypes.c_uint64.from_buffer(task, 8).value)
    tree = {child.pid}
    while True:
        expanded = tree | {pid for pid, (parent, _) in rows.items() if parent in tree}
        if expanded == tree:
            break
        tree = expanded
    peak = max(peak, sum(rows.get(pid, (0, 0))[1] for pid in tree))
    samples += 1
    time.sleep(0.25)
with open(sys.argv[1], 'w', encoding='utf-8') as report:
    json.dump({'peakTreeResidentBytes': peak, 'samples': samples, 'sampleIntervalSeconds': 0.25, 'exitCode': child.returncode}, report)
sys.exit(child.returncode)
