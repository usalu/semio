#!/usr/bin/env python3
"""🧪️ Runs the listed test files in one `bun test` of a root and writes the raw output: python3 -I run_tests.py <root> <list> <out> [timeout s] (kills only its own bun at 6 GB RSS or the timeout; run it through mirror.py run <mirror> --seal-allow <C>/🗑️generated/cp -- ...)."""
import os, subprocess, sys, time
root, listing, out = sys.argv[1:4]
timeout = int(sys.argv[4]) if len(sys.argv) > 4 else 1500
files = ["./" + l.strip() for l in open(listing, encoding="utf8") if l.strip()]
env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=os.path.join(os.path.dirname(out), "art"))
os.makedirs(env["SEMIO_TEST_ARTIFACT_DIR"], exist_ok=True)
with open(out, "w", encoding="utf8") as sink:
    child = subprocess.Popen(["bun", "test", *files], cwd=root, env=env, stdout=sink, stderr=subprocess.STDOUT)
    start, peak = time.time(), 0
    while child.poll() is None:
        time.sleep(1)
        try:
            rss = subprocess.run(["ps", "-o", "rss=", "-p", str(child.pid)], capture_output=True, text=True).stdout.strip()
            peak = max(peak, int(rss) // 1024 if rss else 0)
        except OSError:
            pass
        if peak > 6000 or time.time() - start > timeout:
            child.terminate(); time.sleep(2)
            if child.poll() is None: child.kill()
            sink.write("\n[DEBUG] killed by the runner: peak %d MB, %d s\n" % (peak, time.time() - start))
            break
    sink.write("\n[DEBUG] exit %s peak rss %d MB wall %d s\n" % (child.returncode, peak, time.time() - start))
