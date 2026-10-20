#!/usr/bin/env python3
"""Runs explicit suite files one at a time (or LIVE=1 against the real checkout, 2 concurrent) and records pass/fail.
usage: run.py LISTFILE OUTJSONL [CAP_SECONDS]; env LIVE=1 for the real repo, CONC for concurrency."""
import json, os, re, subprocess, sys, tempfile, shutil, time
from concurrent.futures import ThreadPoolExecutor
LIVE = os.environ.get("LIVE") == "1"
ROOT = "/Users/ueli/Documents/semio" if LIVE else "/private/tmp/testfix-law"
LOGS = "/private/tmp/testfix-law/.logs" + ("-live" if LIVE else "")
ANSI = re.compile(r"\x1b\[[0-9;]*m")
def main():
    lst, out = sys.argv[1], sys.argv[2]
    cap = sys.argv[3] if len(sys.argv) > 3 else "300"
    conc = int(os.environ.get("CONC", "1"))
    files = [l.rstrip("\n") for l in open(lst, encoding="utf-8") if l.strip()]
    os.makedirs(LOGS, exist_ok=True)
    art = "/private/tmp/testfix-law/.art"
    os.makedirs(art, exist_ok=True)
    envwire = ROOT + "/.tfl-envwire.ts"
    if not LIVE: shutil.copyfile(os.path.dirname(os.path.abspath(__file__)) + "/envwire.ts", envwire)
    base = dict(os.environ, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, REPO_ROOT=ROOT, SEMIO_BUILD_MODE="prototype")
    def run(f):
        wire = {}
        if not LIVE:
            w = subprocess.run(["bun", envwire], cwd=ROOT, capture_output=True, text=True)
            try: wire = json.loads(w.stdout)
            except Exception: wire = {}
        e = dict(base, **wire, SEMIO_TEST_ARTIFACT_DIR=tempfile.mkdtemp(dir=art, prefix=""))
        s = time.time()
        try:
            p = subprocess.run(["perl", "-e", "alarm " + cap + "; exec @ARGV", "--", "bun", "test", "--timeout", "120000", "./" + f], cwd=ROOT, env=e, capture_output=True)
            txt = ANSI.sub("", (p.stdout or b"").decode("utf-8", "replace") + (p.stderr or b"").decode("utf-8", "replace")); rc = p.returncode
        except Exception as ex:
            txt = "RUNNER " + repr(ex); rc = -1
        shutil.rmtree(e["SEMIO_TEST_ARTIFACT_DIR"], ignore_errors=True)
        def num(k):
            m = re.findall(r"^\s*(\d+) " + k + r"\b", txt, re.M)
            return int(m[-1]) if m else 0
        rec = dict(file=f, rc=rc, pass_=num("pass"), fail=num("fail"), secs=round(time.time() - s, 1))
        lp = LOGS + "/" + re.sub(r"[^A-Za-z0-9]+", "_", f)[-120:] + ".log"
        open(lp, "w", encoding="utf-8").write(txt[-80000:]); rec["log"] = lp
        rec["fails"] = [l.strip()[:200] for l in txt.splitlines() if l.startswith("(fail)")][:12]
        open(out, "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n")
        print(("PASS " if rc == 0 else "FAIL ") + f[-100:], rec["pass_"], rec["fail"], rec["secs"], flush=True)
    with ThreadPoolExecutor(conc) as ex: list(ex.map(run, files))
main()
