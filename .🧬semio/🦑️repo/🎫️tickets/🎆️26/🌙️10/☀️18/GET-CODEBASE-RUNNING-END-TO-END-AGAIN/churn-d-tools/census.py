"""Census of concurrent-edit refusal texts in run logs under 🗑️generated (last N hours). usage: census.py <hours>"""
import os, re, sys, time, collections
root = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated"
hours = float(sys.argv[1]) if len(sys.argv) > 1 else 36
limit = time.time() - hours * 3600
pat = re.compile(r"(generation changed during consumption|Stale loaded[^\n\"]{0,60}|restart required|outdated implementation|Another process edited|[Ss]ource identity changed[^\n\"]{0,40}|topology changed|roster changed|descriptor or path changed|lockfile had changes[^\n\"]{0,40}|changed during [a-z ]{3,30}|Current [a-z -]{3,40} changed[^\n\"]{0,40}|Actual [a-z ]{3,40} changed[^\n\"]{0,40}|concurrent[^\n\"]{0,40}edit)")
rows = collections.defaultdict(lambda: [0, set()])
for dirpath, dirs, files in os.walk(root, followlinks=False):
    dirs[:] = [d for d in dirs if d not in ("backup", "orig", "node_modules", "art")]
    for name in files:
        if not name.endswith((".log", ".err", ".out")):
            continue
        path = os.path.join(dirpath, name)
        try:
            st = os.stat(path)
            if st.st_mtime < limit or st.st_size > 200_000_000:
                continue
            with open(path, "r", errors="replace") as f:
                for line in f:
                    m = pat.search(line)
                    if m:
                        key = m.group(1)[:90]
                        rows[key][0] += 1
                        rows[key][1].add(os.path.relpath(path, root))
        except OSError:
            pass
for key, (count, paths) in sorted(rows.items(), key=lambda kv: -kv[1][0]):
    print(f"{count:6d}  {key}  [{len(paths)} files] e.g. {sorted(paths)[:3]}")
