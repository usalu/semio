"""🧨️ Churn generator for the live proof: while a real build runs it (1) replaces input files by byte-identical copies with preserved mtime (new inode and ctime, as an editor saving through a rename does) and (2) creates and removes a gitignored probe file in input directories (roster churn). Never changes a byte of any input. Stops at the stop file or the time limit and removes every probe it made.
usage: toucher.py <log.json> <stop-file> <max-seconds> <interval-ms> --files f1 f2 ... --dirs d1 d2 ... [--glob <pattern> <max-files> <max-bytes>]"""
import glob, json, os, signal, sys, time

log, stop, limit, interval = sys.argv[1], sys.argv[2], float(sys.argv[3]), int(sys.argv[4]) / 1000
rest = sys.argv[5:]
stops = [rest.index(flag) for flag in ("--files", "--dirs", "--glob") if flag in rest] + [len(rest)]


def section(flag):
    if flag not in rest:
        return []
    start = rest.index(flag) + 1
    return rest[start:min(index for index in stops if index >= start)]


files, dirs, globbed = section("--files"), section("--dirs"), section("--glob")
if globbed:
    pattern, limit_files, limit_bytes = globbed[0], int(globbed[1]), int(globbed[2])
    files += [path for path in sorted(glob.glob(pattern, recursive=True)) if os.path.isfile(path) and os.path.getsize(path) <= limit_bytes][:limit_files]
stats_files = len(files)
probe = "churn-c-probe.tmp"
stats = {"files": stats_files, "cycles": 0, "replaced": 0, "probes": 0, "errors": 0, "started": time.time()}
done = False


def finish(*_):
    global done
    done = True


signal.signal(signal.SIGTERM, finish)
signal.signal(signal.SIGINT, finish)


def replace(path):
    st = os.stat(path)
    with open(path, "rb") as handle:
        data = handle.read()
    tmp = path + ".churn-c.tmp"
    with open(tmp, "wb") as handle:
        handle.write(data)
    os.chmod(tmp, st.st_mode & 0o7777)
    os.utime(tmp, ns=(st.st_atime_ns, st.st_mtime_ns))
    os.replace(tmp, path)
    stats["replaced"] += 1


def cycle():
    for path in files:
        try:
            replace(path)
        except OSError:
            stats["errors"] += 1
    for directory in dirs:
        target = os.path.join(directory, probe)
        try:
            with open(target, "w") as handle:
                handle.write("x")
            stats["probes"] += 1
            time.sleep(0.005)
            os.remove(target)
        except OSError:
            stats["errors"] += 1


try:
    while not done and not os.path.exists(stop) and time.time() - stats["started"] < limit:
        cycle()
        stats["cycles"] += 1
        time.sleep(interval)
finally:
    for directory in dirs:
        try:
            os.remove(os.path.join(directory, probe))
        except OSError:
            pass
    for path in files:
        try:
            os.remove(path + ".churn-c.tmp")
        except OSError:
            pass
    stats["ended"] = time.time()
    with open(log, "w") as handle:
        json.dump(stats, handle)
