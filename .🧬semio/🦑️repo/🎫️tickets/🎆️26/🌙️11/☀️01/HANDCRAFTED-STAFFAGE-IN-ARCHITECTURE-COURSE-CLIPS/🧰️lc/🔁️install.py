import os, sys, time
src, dst = sys.argv[1], sys.argv[2]
data = open(src, "rb").read()
if os.path.getsize(dst) != 0:
    sys.exit("destination not empty: refusing")
tmp = dst + ".lc-tmp"
with open(tmp, "wb") as h:
    h.write(data)
for _ in range(40):
    try:
        os.replace(tmp, dst); break
    except OSError:
        time.sleep(0.5)
print(os.path.getsize(dst))
