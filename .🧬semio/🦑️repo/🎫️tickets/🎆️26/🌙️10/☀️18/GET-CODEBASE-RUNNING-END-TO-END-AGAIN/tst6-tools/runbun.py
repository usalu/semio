#!/usr/bin/env python3
import json, os, re, shutil, subprocess, sys, tempfile, time
from concurrent.futures import ThreadPoolExecutor

ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/tst6-out"
ART = ROOT + "/.cache/tst6"


def main():
    lst, out, budget = sys.argv[1], sys.argv[2], float(sys.argv[3])
    cap = sys.argv[4] if len(sys.argv) > 4 else "300"
    workers = 2
    files = [l.strip() for l in open(lst, encoding="utf-8") if l.strip()]
    done = set()
    if os.path.exists(out):
        for l in open(out, encoding="utf-8"):
            try:
                done.add(json.loads(l)["file"])
            except Exception:
                pass
    os.makedirs(G + "/logs", exist_ok=True)
    os.makedirs(ART, exist_ok=True)
    wire = json.loads(subprocess.run(["bun", C + "/tst6-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True, env=dict(os.environ, SEMIO_REPO_ROOT=ROOT)).stdout)
    base = dict(os.environ, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, REPO_ROOT=ROOT, SEMIO_WORKSPACE_ROOT=ROOT, SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype", SEMIO_BUILD_TOOL="nx", SEMIO_SCRIPT_PROCESS_INVOCATION=wire["invocation"], SEMIO_SCRIPT_PROVIDERS=wire["providers"])
    base.pop("NODE_PATH", None)
    t0 = time.time()
    todo = [f for f in files if f not in done]

    def run(f):
        if time.time() - t0 > budget:
            return None
        art = tempfile.mkdtemp(dir=ART, prefix="r")
        env = dict(base, SEMIO_TEST_ARTIFACT_DIR=art)
        s = time.time()
        p = subprocess.run(["perl", "-e", "alarm " + cap + "; exec @ARGV", "--", "bun", "test", "--timeout", "120000", "./" + f], cwd=ROOT, env=env, capture_output=True)
        txt = (p.stdout or b"").decode("utf-8", "replace") + (p.stderr or b"").decode("utf-8", "replace")
        rc = p.returncode
        d = time.time() - s
        def num(k):
            m = re.findall(r"^\s*(\d+) " + k + r"\b", txt, re.M)
            return int(m[-1]) if m else 0
        rec = dict(file=f, rc=rc, pass_=num("pass"), fail=num("fail"), skip=num("skip"), err=num("error"), secs=round(d, 1))
        if rc != 0:
            lp = G + "/logs/" + re.sub(r"[^A-Za-z0-9]+", "_", f)[-120:] + ".log"
            open(lp, "w", encoding="utf-8").write(txt[-60000:])
            rec["log"] = lp
        shutil.rmtree(art, ignore_errors=True)
        open(out, "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n")
        return rec

    with ThreadPoolExecutor(workers) as ex:
        res = list(ex.map(run, todo))
    n = len([r for r in res if r])
    print(f"ran {n}, remaining {len(todo)-n}, elapsed {time.time()-t0:.0f}s")


main()
