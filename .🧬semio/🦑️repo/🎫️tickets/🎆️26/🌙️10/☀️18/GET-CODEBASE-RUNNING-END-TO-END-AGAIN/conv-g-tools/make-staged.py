import os, shutil, hashlib, json, build, ops
H = build.HERE; S = H + "/staged"; shutil.rmtree(S, ignore_errors=True)
paths = [p for p in dict.fromkeys(list(build.PATHS) + list(ops.NEW)) if os.path.exists(H + "/new/" + hashlib.sha256(p.encode()).hexdigest()[:10])]
touched = {build.resolve(a) for a in ops.OPS} | set(ops.NEW)
n = 0
for p in paths:
    if p not in touched: continue
    key = hashlib.sha256(p.encode()).hexdigest()[:10]; new = open(H + "/new/" + key, "rb").read()
    base = open(H + "/base/" + key, "rb").read() if p not in ops.NEW else None
    if base == new: continue
    os.makedirs(os.path.dirname(S + "/files/" + p), exist_ok=True); open(S + "/files/" + p, "wb").write(new)
    if base is not None:
        os.makedirs(os.path.dirname(S + "/base/" + p), exist_ok=True); open(S + "/base/" + p, "wb").write(base)
    n += 1
print(n, "staged files")
