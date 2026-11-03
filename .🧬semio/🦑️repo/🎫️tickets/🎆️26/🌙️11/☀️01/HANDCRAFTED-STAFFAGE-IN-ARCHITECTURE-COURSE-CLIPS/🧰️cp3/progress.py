import sys, glob, av
tot = 0.0
files = sorted(glob.glob(sys.argv[1] + "/p/*/uncached_*.mp4"))
for f in files:
    try:
        with av.open(f) as c:
            tot += float(c.duration) / 1e6 if c.duration else 0
    except Exception as e:
        pass
print(len(files), round(tot, 1))
