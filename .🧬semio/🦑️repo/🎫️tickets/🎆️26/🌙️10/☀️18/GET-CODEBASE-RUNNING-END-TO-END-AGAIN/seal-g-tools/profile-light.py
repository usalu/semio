#!/usr/bin/env python3
"""⏱️ Where a sealed light command spends its time (run INSIDE a mirror): replays the trampoline's spawn (loader via --eval, snapshot through fd 3, ready on fd 4) and times spawn -> ready -> exit.
usage: mm run <n> -- python3 -I profile-light.py <pinned-bun> [runs]"""
import glob, json, os, subprocess, sys, threading, time
PB, RUNS = sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 5
store = glob.glob(".🧬semio/🦑️repo/⚡️cache/implementation/*/current.json")[0]
pointer = json.load(open(store, encoding="utf-8")); d = os.path.dirname(store)
loader = open("%s/%s.loader.js" % (d, pointer["loader"]), encoding="utf-8").read(); snap = "%s/%s.json" % (d, pointer["digest"])
for n in range(RUNS):
    r3, w3 = os.pipe(); r4, w4 = os.pipe()
    t0 = time.perf_counter()
    p = subprocess.Popen([PB, "--eval", loader, "--", pointer["digest"], "--semio-role=door", "repo", "--help"], stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=dict(os.environ, BUN_RUNTIME_TRANSPILER_CACHE_PATH="0"), pass_fds=(r3, w4), preexec_fn=lambda: (os.dup2(r3, 3), os.dup2(w4, 4)))
    os.close(r3); os.close(w4)
    def feed():
        with open(snap, "rb") as f, os.fdopen(w3, "wb") as out:
            try:
                while True:
                    block = f.read(1 << 20)
                    if not block: break
                    out.write(block)
            except BrokenPipeError:
                pass
    threading.Thread(target=feed, daemon=True).start()
    os.read(r4, 1); ready = time.perf_counter() - t0
    p.wait(); done = time.perf_counter() - t0
    print("run %d  spawn->ready %.2f s  spawn->exit %.2f s" % (n + 1, ready, done), flush=True)
