import os, sys, time
mount = sys.argv[1]; data = os.path.join(mount, "data"); stop = sys.argv[2]
while not os.path.exists(stop):
    s = os.statvfs(mount)
    try: claims = len(os.listdir(os.path.join(data, "claims")))
    except OSError: claims = -1
    try: entries = len(os.listdir(data))
    except OSError: entries = -1
    print(time.strftime("%H:%M:%S"), "used%", round(100 * (1 - s.f_bavail / s.f_blocks), 1), "free", s.f_bavail * s.f_frsize, "entries", entries, "claims", claims, flush=True)
    time.sleep(2)
