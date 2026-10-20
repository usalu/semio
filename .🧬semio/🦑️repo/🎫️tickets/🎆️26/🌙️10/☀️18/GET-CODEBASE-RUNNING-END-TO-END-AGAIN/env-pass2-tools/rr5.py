#!/usr/bin/env python3
# usage: rr.py <tag> <budgetSeconds> <listfile> idx...   (idx = 1-based line of the list file; names prefix+idx) ; <=2 concurrent
import subprocess, sys, time, threading, os
C = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag, budget, listfile = sys.argv[1], float(sys.argv[2]), sys.argv[3]
prefix = os.environ.get("PFX", "A")
files = [l.strip() for l in open(listfile, encoding="utf-8") if l.strip()]
idx = [int(x) for x in sys.argv[4:]] or list(range(1, len(files) + 1))
done = set()
sp = C + "/env-pass2-tools/summary-" + tag + ".tsv"
if os.path.exists(sp):
    done = {l.split("\t")[0] for l in open(sp, encoding="utf-8")}
t0 = time.time()
todo = [i for i in idx if "%s%03d" % (prefix, i) not in done]
lock = threading.Lock()
def worker():
    while True:
        with lock:
            if not todo or time.time() - t0 > budget:
                return
            i = todo.pop(0)
        subprocess.run([C + "/env-pass2-tools/one5.sh", files[i - 1], "%s%03d" % (prefix, i)], env=dict(os.environ, RUN_TAG=tag))
ts = [threading.Thread(target=worker) for _ in range(2)]
[t.start() for t in ts]; [t.join() for t in ts]
print("remaining", len(todo))
