"""👁️ Copies the memo sidecar header a graph planner child leaves in its temporary directory (`semio-graph-plan-*/memo.json`) before the parent removes it: `sidecar_watch.py <out-dir> [<seconds>]`; stops after the seconds or on a `STOP` file in the out-dir."""
import glob, os, shutil, sys, tempfile, time

out = sys.argv[1]
limit = float(sys.argv[2]) if len(sys.argv) > 2 else 3600
os.makedirs(out, exist_ok=True)
seen = {}
end = time.time() + limit
base = tempfile.gettempdir()
while time.time() < end and not os.path.exists(os.path.join(out, "STOP")):
    for header in glob.glob(os.path.join(base, "semio-graph-plan-*", "memo.json")):
        try:
            stat = os.stat(header)
        except OSError:
            continue
        key = (header, stat.st_mtime_ns)
        if key in seen:
            continue
        seen[key] = True
        name = "[DEBUG] sidecar-%d-%s.json" % (int(time.time()), os.path.basename(os.path.dirname(header)))
        try:
            shutil.copyfile(header, os.path.join(out, name))
        except OSError:
            pass
    time.sleep(0.03)
