"""submit a job to the daemon and wait for its output text"""
import json, os, sys, time, uuid
from io_ import *
def submit(op, asset, wait=True, timeout=3600, file=None, **kw):
    os.makedirs(JOBS, exist_ok=True)
    jid = f"{int(time.time()*1000)}-{op}-{asset}-{uuid.uuid4().hex[:4]}"
    job = {"op": op, "file": file or path(asset), **kw}
    tmp = f"{JOBS}/{jid}.tmp"
    json.dump(job, open(tmp, "w")); os.rename(tmp, f"{JOBS}/{jid}.job2.json" if os.path.exists(f"{JOBS}/ready2") else f"{JOBS}/{jid}.job.json")
    if not wait: return jid
    out = f"{JOBS}/{jid}.out.txt"; t0 = time.time()
    while not os.path.exists(out):
        if time.time() - t0 > timeout: raise TimeoutError(jid)
        time.sleep(1)
    return open(out).read()
if __name__ == "__main__":
    op, asset = sys.argv[1], sys.argv[2]
    kw = json.loads(sys.argv[3]) if len(sys.argv) > 3 else {}
    print(submit(op, asset, **kw))
