import fcntl, os, shutil, sys, time
root, hours = sys.argv[1], float(sys.argv[2])
bound = time.time() - hours * 3600
def newest(path):
    m = os.stat(path).st_mtime
    for base, dirs, files in os.walk(path):
        for f in files:
            try: m = max(m, os.lstat(os.path.join(base, f)).st_mtime)
            except OSError: pass
    return m
def size(path):
    t = 0
    for base, dirs, files in os.walk(path):
        for f in files:
            try: t += os.lstat(os.path.join(base, f)).st_size
            except OSError: pass
    return t
profiles = [os.path.join(root, "debug", "build")]
for t in ("wasm32-wasip2", "wasm32-unknown-unknown"):
    tdir = os.path.join(root, t)
    if os.path.isdir(tdir):
        profiles += [os.path.join(tdir, p, "build") for p in os.listdir(tdir)]
removed, freed = 0, 0
for profile_build in profiles:
    if not os.path.isdir(profile_build): continue
    for pkg in os.listdir(profile_build):
        pdir = os.path.join(profile_build, pkg)
        if not os.path.isdir(pdir): continue
        units = [os.path.join(pdir, u) for u in os.listdir(pdir) if os.path.isdir(os.path.join(pdir, u))]
        if len(units) < 2: continue
        ages = sorted(((newest(u), u) for u in units), reverse=True)
        for m, u in ages[1:]:
            if m > bound: continue
            try:
                fd = os.open(os.path.join(u, ".lock"), os.O_RDWR | os.O_CREAT)
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except OSError:
                continue
            try:
                freed += size(u); shutil.rmtree(u, ignore_errors=True); removed += 1
            finally:
                os.close(fd)
print(time.strftime("%F %T"), root, "removed", removed, "units", round(freed / 2**30, 1), "GiB")
