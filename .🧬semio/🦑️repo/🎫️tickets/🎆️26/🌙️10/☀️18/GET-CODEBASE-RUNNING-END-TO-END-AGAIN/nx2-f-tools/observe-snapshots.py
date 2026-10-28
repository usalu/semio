"""observe-snapshots.py <out> <seconds>: every 1 s logs each graph snapshot file (mtime within 10 min) whose (ino, mtime, ctime, nlink) changed since the last sample; one line per change with wall time."""
import os, sys, time
root = "/Users/ueli/Documents/semio/.cache/b/g/m/snapshots"
out, limit = sys.argv[1], float(sys.argv[2])
seen = {}
end = time.time() + limit
with open(out, "a", buffering=1) as fh:
    while time.time() < end:
        now = time.time()
        for e in os.scandir(root):
            if not e.name.endswith(".json"): continue
            try: s = e.stat(follow_symlinks=False)
            except OSError: continue
            if s.st_mtime < now - 600: continue
            key = (s.st_ino, s.st_mtime_ns, s.st_ctime_ns, s.st_nlink)
            if seen.get(e.name) != key:
                fh.write(f"{time.strftime('%H:%M:%S')} {e.name[:12]} ino={s.st_ino} nlink={s.st_nlink} mtime={time.strftime('%H:%M:%S', time.localtime(s.st_mtime))} ctime={time.strftime('%H:%M:%S', time.localtime(s.st_ctime))} prev={seen.get(e.name) is not None}\n")
                seen[e.name] = key
        time.sleep(1)
