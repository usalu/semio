#!/usr/bin/env python3
"""⏳️ Waits for a quiet window and recreates the derived graph image at its declared size, once: no repo command that started before the declared size was raised is alive (such a process would birth the old size again), the image envelope differs from the declaration, and the exclusive image lease is obtainable without waiting. Polls, never queues, never kills anything; exits 0 when the image has the declared size. Usage: recreate-when-quiet.py [--ignore-pid=<pid>]... [--poll=<seconds>] [--limit=<seconds>] [--dry-run]"""
import json, os, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "cache-gc-tools"))
import recreate  # noqa: E402
from paths import J, ROOT, TARGET  # noqa: E402

POLICY = J(ROOT, TARGET["budget-policy"])


def declared():
    with open(POLICY, encoding="utf-8") as handle:
        return json.load(handle)["partitions"]["graph"]["capacityBytes"]


def seconds(etime):
    days, _, rest = etime.partition("-") if "-" in etime else ("0", "", etime)
    parts = [int(value) for value in rest.split(":")]
    while len(parts) < 3:
        parts.insert(0, 0)
    return int(days) * 86400 + parts[0] * 3600 + parts[1] * 60 + parts[2]


def stale(cutoff, ignored):
    out = subprocess.run(["ps", "-A", "-o", "pid=,etime=,command="], capture_output=True, text=True).stdout
    now, rows, mine = time.time(), [], str(os.getpid())
    for line in out.splitlines():
        pid, etime, command = (line.split(None, 2) + ["", ""])[:3]
        if pid == mine or pid in ignored or ("script.ts repo" not in command and "script.ts\" repo" not in command):
            continue
        if now - seconds(etime) < cutoff:
            rows.append("%s %s" % (pid, command[:90]))
    return rows


def main(argv):
    ignored = {value.split("=", 1)[1] for value in argv if value.startswith("--ignore-pid=")}
    poll = int(next((value.split("=", 1)[1] for value in argv if value.startswith("--poll=")), "5"))
    limit = int(next((value.split("=", 1)[1] for value in argv if value.startswith("--limit=")), "7200"))
    dry = "--dry-run" in argv
    directory = J(ROOT, recreate.DERIVED["graph"])
    started = time.time()
    while time.time() - started < limit:
        want, have = declared(), recreate.envelope(directory)
        if have == want:
            print("%s the graph image has its declared size (%s bytes)" % (time.strftime("%H:%M:%S"), have), flush=True)
            return 0
        cutoff = os.stat(POLICY).st_mtime
        old = stale(cutoff, ignored)
        if old:
            print("%s waiting: %d repo command(s) predate the declaration: %s" % (time.strftime("%H:%M:%S"), len(old), "; ".join(old[:3])), flush=True)
        elif dry:
            print("dry-run: quiet; would recreate (envelope %s -> %s)" % (have, want))
            return 0
        else:
            code = recreate.run("graph", True)
            print("%s recreate exit %s" % (time.strftime("%H:%M:%S"), code), flush=True)
        time.sleep(poll)
    print("limit reached without a quiet window", flush=True)
    return 4


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
