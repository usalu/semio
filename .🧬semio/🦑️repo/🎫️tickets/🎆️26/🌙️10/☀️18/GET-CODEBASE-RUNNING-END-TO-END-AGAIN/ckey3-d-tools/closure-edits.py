"""Lists the files of the live narrow capture edited since a time: python3 closure-edits.py <implementation.executor-command.json> "<YYYY-MM-DD HH:MM | <N>m = last N minutes>" (cwd = repo root)."""
import json, os, sys, time
snap = sys.argv[1]; since = time.time() - float(sys.argv[2][:-1]) * 60 if sys.argv[2].endswith("m") else time.mktime(time.strptime(sys.argv[2], "%Y-%m-%d %H:%M"))
files = [f["path"] for f in json.load(open(snap))["files"]]
rows = sorted((os.path.getmtime(p), p) for p in files if os.path.exists(p) and os.path.getmtime(p) >= since)
print(len(files), "files,", len(rows), "edited since", sys.argv[2])
for m, p in rows: print(time.strftime("%H:%M:%S", time.localtime(m)), p)
