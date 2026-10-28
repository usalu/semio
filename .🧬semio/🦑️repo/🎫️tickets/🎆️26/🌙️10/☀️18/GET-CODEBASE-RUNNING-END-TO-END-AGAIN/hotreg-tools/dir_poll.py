"""🔬 Polls the entries of one directory every 30 ms and logs each identity change (dev, ino, mode, size, mtime, ctime) with a time stamp: `dir_poll.py <dir> <out> <seconds>`; stops on a STOP file beside <out>."""
import os, sys, time

directory, out, limit = sys.argv[1], sys.argv[2], float(sys.argv[3])
stop = out + ".STOP"
state = {}
end = time.time() + limit
with open(out, "w", encoding="utf8") as log:
    while time.time() < end and not os.path.exists(stop):
        now = time.time()
        try:
            names = os.listdir(directory)
        except OSError:
            names = []
        seen = set()
        for name in names:
            seen.add(name)
            try:
                st = os.lstat(os.path.join(directory, name))
            except OSError:
                continue
            ident = (st.st_dev, st.st_ino, st.st_mode, st.st_size, st.st_mtime_ns, st.st_ctime_ns)
            if state.get(name) != ident:
                if name in state:
                    log.write("%.3f CHANGE %s %s\n" % (now, name, [n for n, a, b in zip(["dev", "ino", "mode", "size", "mtime", "ctime"], state[name], ident) if a != b]))
                else:
                    log.write("%.3f NEW %s\n" % (now, name)) if state else None
                state[name] = ident
        for name in list(state):
            if name not in seen:
                log.write("%.3f GONE %s\n" % (now, name))
                del state[name]
        log.flush()
        time.sleep(0.03)
