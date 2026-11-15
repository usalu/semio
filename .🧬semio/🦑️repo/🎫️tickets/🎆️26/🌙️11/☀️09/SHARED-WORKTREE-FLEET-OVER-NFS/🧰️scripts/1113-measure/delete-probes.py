import os, glob, time, json
S = "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🧰️scripts/1113-measure"
GEN = "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🗑️generated"
PROBES = S + "/probes"
SEG = "/Users/Shared/semio-store/v1/origins/g1-server/segments"
def newest():
    return max(glob.glob(SEG + "/*"), key=lambda f: (os.path.getmtime(f), f))
targets = [(f"1113-measure/probes/probe-{n}.txt", os.path.join(PROBES, f"probe-{n}.txt")) for n in range(1, 11)]
targets += [(f"🗑️generated/probe-{n}.txt", os.path.join(GEN, f"probe-{n}.txt")) for n in range(1, 11)]
seg = newest(); off = os.path.getsize(seg)
t0 = time.time(); deleted = []
for needle, p in targets:
    if os.path.exists(p):
        os.remove(p); deleted.append(needle)
print("deleted_files", len(deleted))
res = {}
pending = {n: t0 for n, _ in targets}
deadline = t0 + 120
while pending and time.time() < deadline:
    cur = newest()
    if cur != seg:
        seg, off = cur, 0
    with open(seg, "rb") as f:
        f.seek(off); data = f.read(); off = off + len(data)
    text = data.decode("utf-8", "replace")
    now = time.time()
    for needle in list(pending):
        if needle in text:
            res[needle] = round((now - t0) * 1000, 1)
            del pending[needle]
    time.sleep(0.05)
for needle, _ in targets:
    print(needle, "delete_published_ms=" + str(res.get(needle, "TIMEOUT")))
try:
    os.rmdir(PROBES); print("rmdir probes ok")
except OSError as e:
    print("rmdir probes:", e)
try:
    os.rmdir(GEN); print("rmdir generated ok")
except OSError as e:
    print("rmdir generated:", e)
