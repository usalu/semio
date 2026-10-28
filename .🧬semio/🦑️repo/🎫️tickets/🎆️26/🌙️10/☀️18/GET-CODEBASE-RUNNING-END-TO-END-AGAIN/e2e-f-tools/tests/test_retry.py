"""🔁 Law of retry.sh: a finished wave of specs (green or red) is a verdict and is never run again; a run that ended before any wave finished is run again.
usage: python3 -I test_retry.py <e2e-f-tools dir>   (a throw-away ticket with a stub driver is built around copies of the scripts)
Oracle: the two real log shapes of the gate (wave lines `[e2e] <stack>: <shard> passed|failed (<code>)`, 10-27 run e2e-1027-1d1) against POSIX `grep -E` run by the test itself on the same pattern file."""
import os, shutil, subprocess, sys, tempfile

TOOLS = os.path.abspath(sys.argv[1])
GEN = "\U0001F5D1️generated"
FAILED_WAVE = "[e2e] dev-shared: shared failed (1)\nerror: end-to-end gate failed after 2614 s: specs failed\n"
PASSED_WAVE = "[e2e] dev-exclusive: exclusive passed\n"
NO_WAVE = "[e2e] dev: proctor ready at http://127.0.0.1:1 (development)\nerror: port probe failed (1)\n"

def trial(logs, codes, script="retry.sh"):
    ticket = tempfile.mkdtemp(prefix="retry-law-")
    try:
        for sub in ("e2e-f-tools", "e2e-orch-tools", GEN): os.makedirs(os.path.join(ticket, sub))
        for name in (script, "specs-finished.pattern"):
            if os.path.exists(os.path.join(TOOLS, name)): shutil.copy(os.path.join(TOOLS, name), os.path.join(ticket, "e2e-f-tools", name))
        stub = os.path.join(ticket, "e2e-orch-tools", "run.sh")
        rows = "\n".join("%d) printf '%%s' '%s' > \"%s/e2e-course-$id-course-test-e2e.log\"; exit %d;;" % (n + 1, logs[n].replace("'", "'\\''"), os.path.join(ticket, GEN), codes[n]) for n in range(len(codes)))
        with open(stub, "w") as f:
            f.write('#!/bin/sh\nwhile [ $# -gt 0 ]; do [ "$1" = --run-id ] && id=$2; shift; done\nn=$(( $(ls "%s" | wc -l | tr -d " ") + 1 ))\ncase $n in\n%s\n*) exit 99;;\nesac\n' % (os.path.join(ticket, GEN), rows))
        try:
            done = subprocess.run(["sh", os.path.join(ticket, "e2e-f-tools", script), "p", "dev"], env={**os.environ, "RETRY_PAUSE": "0"}, capture_output=True, text=True, timeout=15)
        except subprocess.TimeoutExpired:
            return -1, len(os.listdir(os.path.join(ticket, GEN)))
        return done.returncode, len(os.listdir(os.path.join(ticket, GEN)))
    finally:
        shutil.rmtree(ticket, ignore_errors=True)

def grep_matches(text):
    pattern = os.path.join(TOOLS, "specs-finished.pattern")
    return subprocess.run(["grep", "-a", "-q", "-E", "-f", pattern], input=text.encode(), capture_output=True).returncode == 0

failures = []
def law(name, ok):
    print(("ok   " if ok else "FAIL ") + name)
    if not ok: failures.append(name)

law("oracle: pattern accepts a failed wave", grep_matches(FAILED_WAVE))
law("oracle: pattern accepts a passed wave", grep_matches(PASSED_WAVE))
law("oracle: pattern rejects a run that never reached a wave", not grep_matches(NO_WAVE))
rc, attempts = trial([FAILED_WAVE], [1])
law("a failed wave ends after one attempt with its exit code (rc=%d attempts=%d)" % (rc, attempts), rc == 1 and attempts == 1)
rc, attempts = trial([NO_WAVE, NO_WAVE, PASSED_WAVE], [1, 1, 0])
law("a run without a wave is run again until green (rc=%d attempts=%d)" % (rc, attempts), rc == 0 and attempts == 3)
sys.exit(1 if failures else 0)
