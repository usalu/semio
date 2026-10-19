#!/usr/bin/env python3
import json, os, re, subprocess, sys, time
from concurrent.futures import ThreadPoolExecutor

ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/tst3c-out"
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
ART = S + "/ta3c"
HOIST = S + "/hoist/tsconfig.json"


def run(f, env, hoist, tag):
    import tempfile, shutil
    base = env["SEMIO_TEST_ARTIFACT_DIR"]
    env = dict(env, SEMIO_TEST_ARTIFACT_DIR=tempfile.mkdtemp(dir=base, prefix="r"))
    extra = ["--tsconfig-override", HOIST] if hoist else []
    s = time.time()
    p = subprocess.run(["perl", "-e", "alarm 300; exec @ARGV", "--", "bun", "test"] + extra + ["--timeout", "120000", "./" + f], cwd=ROOT, env=env, capture_output=True)
    txt = (p.stdout or b"").decode("utf-8", "replace") + (p.stderr or b"").decode("utf-8", "replace")
    rc = p.returncode
    def num(k):
        m = re.findall(r"^\s*(\d+) " + k + r"\b", txt, re.M)
        return int(m[-1]) if m else 0
    rec = dict(file=f, rc=rc, pass_=num("pass"), fail=num("fail"), skip=num("skip"), err=num("error"), secs=round(time.time() - s, 1))
    if rc != 0:
        lp = G + "/logs-" + tag + "/" + re.sub(r"[^A-Za-z0-9]+", "_", f)[-120:] + ".log"
        os.makedirs(os.path.dirname(lp), exist_ok=True)
        open(lp, "w", encoding="utf-8").write(txt[-60000:])
        rec["log"] = lp
    shutil.rmtree(env["SEMIO_TEST_ARTIFACT_DIR"], ignore_errors=True)
    return rec


def main():
    lst, out, tag = sys.argv[1], sys.argv[2], sys.argv[3]
    hoist = os.environ.get("TST3C_HOIST") == "1"
    files = [l.strip() for l in open(lst, encoding="utf-8") if l.strip()]
    done = set()
    if os.path.exists(out):
        for l in open(out, encoding="utf-8"):
            try: done.add(json.loads(l)["file"])
            except Exception: pass
    art = os.environ.get("TST3C_ART") or ART
    os.makedirs(art, exist_ok=True)
    wire = subprocess.run(["bun", C + "/tst3c-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout
    env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=art, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, SEMIO_SCRIPT_PROCESS_INVOCATION=wire, SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype")
    todo = [f for f in files if f not in done]
    t0 = time.time()
    budget = float(sys.argv[4]) if len(sys.argv) > 4 else 1e9
    def w(f):
        if time.time() - t0 > budget: return
        for attempt in range(40):
            try:
                r = run(f, env, hoist, tag)
                open(out, "a", encoding="utf-8").write(json.dumps(r, ensure_ascii=False) + "\n")
                return
            except OSError:
                time.sleep(15)
    with ThreadPoolExecutor(2) as ex:
        list(ex.map(w, todo))
    print("ran", len(todo))

main()
