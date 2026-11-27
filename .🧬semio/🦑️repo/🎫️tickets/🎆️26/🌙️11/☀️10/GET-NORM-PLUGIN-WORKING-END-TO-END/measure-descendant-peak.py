#!/usr/bin/env python3
"""📏️ Runs a command and samples resident set and physical footprint of every descendant captured-loader process (`bun --eval` of the closure loader) about twice a second; prints one JSON line with the peaks and the time series."""
import ctypes, ctypes.util, json, re, struct, subprocess, sys, time

libproc = ctypes.CDLL(ctypes.util.find_library("proc"))

def footprint(pid):
    buffer = ctypes.create_string_buffer(512)
    if libproc.proc_pid_rusage(pid, 4, buffer) != 0:
        return 0
    return struct.unpack_from("<Q", buffer.raw, 72)[0] // 1048576

def table():
    out = subprocess.run(["ps", "-axww", "-o", "pid=,ppid=,rss=,args="], capture_output=True, text=True).stdout
    rows = {}
    for line in out.splitlines():
        parts = line.split(None, 3)
        if len(parts) >= 3:
            rows[int(parts[0])] = (int(parts[1]), int(parts[2]) // 1024, parts[3] if len(parts) > 3 else "")
    return rows

root = subprocess.Popen(sys.argv[1:])
begin, series, peaks, labels = time.time(), [], {}, {}
while root.poll() is None:
    rows = table()
    mine = {root.pid}
    grew = True
    while grew:
        grew = False
        for pid, (ppid, _, _) in rows.items():
            if ppid in mine and pid not in mine:
                mine.add(pid); grew = True
    for pid in mine:
        if pid in rows and "--eval" in rows[pid][2] and "closureRefusalTextV1" in rows[pid][2]:
            found = re.search(r"-- [a-f0-9]{64} (.{0,60})", rows[pid][2])
            labels[pid] = found.group(1) if found else ""
            rss, foot = rows[pid][1], footprint(pid)
            series.append([round(time.time() - begin, 1), pid, rss, foot])
            peaks[pid] = [max(peaks.get(pid, [0, 0])[0], rss), max(peaks.get(pid, [0, 0])[1], foot)]
    time.sleep(0.3)
print(json.dumps({"exit": root.returncode, "peaks": {str(k): v for k, v in peaks.items()}, "labels": {str(k): v for k, v in labels.items()}, "series": series}))
