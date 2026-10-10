import concurrent.futures
import os
import re
import subprocess
import sys
import time

ROOT = sys.argv[1]
MODE = sys.argv[2] if len(sys.argv) > 2 else "check"
TESTS = os.path.join(ROOT, "\U0001F9EA️tests")
PYTHON = sys.argv[3]
ONLY = sys.argv[4:]


def targets():
    for name in sorted(os.listdir(TESTS)):
        script = os.path.join(TESTS, name, "\U0001F40D️.py")
        if not os.path.isfile(script):
            continue
        text = open(script, encoding="utf-8").read()
        found = re.findall(r"python \S*\U0001F40D️\.py %s <path to ([^>]+)>" % MODE, text) or re.findall(r"python \S*\U0001F40D️\.py check <path to ([^>]+)>", text)
        yield name, script, [os.path.join(ROOT, path.strip().strip("`")) for path in found[:1]]


def run(item):
    name, script, paths = item
    if not paths:
        return name, "skip", 0, "no documented fixtures path"
    started = time.time()
    try:
        done = subprocess.run([PYTHON, "-X", "utf8", script, MODE, paths[0]], capture_output=True, timeout=600, env={**os.environ, "PYTHONUTF8": "1"}, encoding="utf-8", errors="replace")
        tail = "\n".join(done.stdout.strip().splitlines()[-6:] + done.stderr.strip().splitlines()[-3:])
        return name, "ok" if done.returncode == 0 else "FAIL(%d)" % done.returncode, time.time() - started, tail
    except subprocess.TimeoutExpired:
        return name, "TIMEOUT", time.time() - started, ""


with concurrent.futures.ThreadPoolExecutor(4) as pool:
    for name, status, seconds, tail in pool.map(run, list(targets())):
        print("== %s %s %.0fs" % (name, status, seconds))
        if status != "ok":
            print(tail[:900])
