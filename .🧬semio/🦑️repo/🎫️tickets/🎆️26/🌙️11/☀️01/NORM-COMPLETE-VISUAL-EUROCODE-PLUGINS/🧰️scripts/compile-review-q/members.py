import os, re, sys, time
ROOT = "/home/deck/semio/✏️s"
MIN = int(sys.argv[1]); FILTERS = sys.argv[2:]
now = time.time()
OLD = 60*60*24*3
known = set(); recent = {}
pat = re.compile(r'\.([a-z_][a-z0-9_]*)\b')
defpat = re.compile(r'\bfn\s+([a-z_]\w*)|\b([a-z_]\w*)\s*:|\blet\s+(?:mut\s+)?([a-z_]\w*)|\b([a-z_]\w*)\s*=>')
def scan(d0, recent_only):
    for d, ds, fs in os.walk(d0):
        ds[:] = [x for x in ds if x not in ("target", "node_modules", ".git")]
        for f in fs:
            if not f.endswith(".rs"): continue
            p = os.path.join(d, f)
            try: st = os.stat(p)
            except OSError: continue
            t = open(p, errors="ignore").read()
            t = re.sub(r'"(?:\\.|[^"\\])*"', '""', re.sub(r'//[^\n]*', '', t))
            for m in defpat.finditer(t):
                for g in m.groups():
                    if g: known.add(g)
            if now - st.st_mtime > MIN*60:
                known.update(pat.findall(t))
            elif any(x in p for x in FILTERS):
                for m in pat.finditer(t):
                    recent.setdefault(m.group(1), []).append((p, t.count("\n", 0, m.start())+1))
scan(ROOT, False)
scan("/home/deck/semio/🧰️framework", False)
for n, locs in sorted(recent.items()):
    if n in known or n.isdigit(): continue
    print(n, len(locs), locs[0][0].replace(ROOT, "")[-110:], locs[0][1])
