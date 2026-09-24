import os, json, sys, shutil
root = "/Users/ueli/Documents/semio"
cases = json.load(open(sys.argv[1]))
log = []
for c in cases:
    case = os.path.join(root, c["case"]); name = os.path.basename(case)
    src = os.path.join(case, "🧫️fixtures"); dst = os.path.join(root, c["owner"], "🧫️fixtures", name)
    if not os.path.isdir(src): log.append(f"SKIP no-src {c['case']}"); continue
    if os.path.exists(dst): log.append(f"SKIP dst-exists {dst}"); continue
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.move(src, dst)
    for f in os.listdir(case):
        p = os.path.join(case, f)
        if not os.path.isfile(p): continue
        t = open(p, encoding="utf-8").read()
        n = t.replace("local://", f"shared://{name}/")
        if n != t: open(p, "w", encoding="utf-8").write(n); log.append(f"rewrote {t.count('local://')} {c['case']}/{f}")
    log.append(f"moved {c['case']}/🧫️fixtures -> {os.path.relpath(dst, root)}")
print("\n".join(log))
