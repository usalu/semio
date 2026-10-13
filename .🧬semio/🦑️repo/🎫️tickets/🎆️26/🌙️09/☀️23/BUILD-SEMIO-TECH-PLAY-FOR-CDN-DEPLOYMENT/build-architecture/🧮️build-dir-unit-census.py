"""🧮️ Per-crate census of a shared Cargo build-dir profile (new layout): unit count, compile wall from
fingerprint/invoked.timestamp to rlib mtime, rlib size and dep-info Rust lines. Read-only.
usage: python3 🧮️build-dir-unit-census.py <wasm-dev|wasm-release> <top-n>"""
import os, sys, glob, collections, re
root = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/wasm32-wasip2/" + sys.argv[1] + "/build"
linecache = {}
def lines(p):
    if p not in linecache:
        try: linecache[p] = sum(1 for _ in open(p, 'rb'))
        except Exception: linecache[p] = 0
    return linecache[p]
rows = []
for crate in os.listdir(root):
    for h in os.listdir(os.path.join(root, crate)):
        u = os.path.join(root, crate, h)
        inv = os.path.join(u, "fingerprint", "invoked.timestamp")
        out = os.path.join(u, "out")
        if not os.path.isdir(out) or not os.path.exists(inv): continue
        rl = [f for f in os.listdir(out) if f.endswith(".rlib")]
        ds = [f for f in os.listdir(out) if f.endswith(".d")]
        if not rl: continue
        r = os.path.join(out, rl[0]); t0 = os.path.getmtime(inv); t1 = os.path.getmtime(r)
        n = 0
        if ds:
            txt = open(os.path.join(out, ds[0]), encoding='utf8', errors='replace').read()
            first = txt.split("\n\n")[0]
            files = set(re.findall(r'(/[^\s:]+?\.rs)(?=[:\s])', first))
            n = sum(lines(f) for f in files)
        rows.append((crate, h, round(t1 - t0), os.path.getsize(r) // 1048576, n))
by = collections.defaultdict(list)
for r in rows: by[r[0]].append(r)
print("units", len(rows), "crates", len(by), "sum_s", sum(r[2] for r in rows))
print("crate\tunits\tmax_s\tsum_s\tmax_rlib_MB\tmax_lines")
for c, rs in sorted(by.items(), key=lambda kv: -max(r[2] for r in kv[1]))[:int(sys.argv[2])]:
    print(f"{c}\t{len(rs)}\t{max(r[2] for r in rs)}\t{sum(r[2] for r in rs)}\t{max(r[3] for r in rs)}\t{max(r[4] for r in rs)}")
