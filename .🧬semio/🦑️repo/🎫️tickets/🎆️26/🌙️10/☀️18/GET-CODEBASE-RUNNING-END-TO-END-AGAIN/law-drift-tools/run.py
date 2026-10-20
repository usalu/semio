#!/usr/bin/env python3
import json, os, re, subprocess, sys, time, tempfile, shutil
from concurrent.futures import ThreadPoolExecutor
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/law-drift-run"
ART = os.environ.get("LD_ART") or C + "/🗑️generated/l"
ANSI = re.compile(r"\x1b\[[0-9;]*m")

def main():
    lst, out, budget = sys.argv[1], sys.argv[2], float(sys.argv[3])
    cap = sys.argv[4] if len(sys.argv) > 4 else "300"
    files = [l.rstrip("\n") for l in open(lst, encoding="utf-8") if l.strip()]
    done = set()
    if os.path.exists(out):
        for l in open(out, encoding="utf-8"):
            try: done.add(json.loads(l)["file"])
            except Exception: pass
    os.makedirs(G + "/logs", exist_ok=True)
    ART_ = os.environ.get("TST7_ART") or ART
    os.makedirs(ART_, exist_ok=True)
    env = dict(os.environ, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, REPO_ROOT=ROOT, SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype")
    t0 = time.time()
    todo = [f for f in files if f not in done]
    def run(f):
        if time.time() - t0 > budget: return None
        wire = json.loads(subprocess.run(["bun", C + "/law-drift-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout)
        e = dict(env, **wire, SEMIO_TEST_ARTIFACT_DIR=tempfile.mkdtemp(dir=ART_, prefix=""))
        s = time.time()
        try:
            p = subprocess.run(["perl", "-e", "alarm " + cap + "; exec @ARGV", "--", "bun", "test", "--timeout", "120000", "./" + f], cwd=ROOT, env=e, capture_output=True)
            txt = ANSI.sub("", (p.stdout or b"").decode("utf-8", "replace") + (p.stderr or b"").decode("utf-8", "replace"))
            rc = p.returncode
        except Exception as ex:
            txt = "RUNNER " + repr(ex); rc = -1
        shutil.rmtree(e["SEMIO_TEST_ARTIFACT_DIR"], ignore_errors=True)
        def num(k):
            m = re.findall(r"^\s*(\d+) " + k + r"\b", txt, re.M)
            return int(m[-1]) if m else 0
        rec = dict(file=f, rc=rc, pass_=num("pass"), fail=num("fail"), skip=num("skip"), err=num("error"), secs=round(time.time() - s, 1))
        if rc != 0:
            lp = G + "/logs/" + re.sub(r"[^A-Za-z0-9]+", "_", f)[-120:] + ".log"
            open(lp, "w", encoding="utf-8").write(txt[-60000:])
            rec["log"] = lp
            rec["fails"] = [l.strip()[:200] for l in txt.splitlines() if l.startswith("(fail)")][:12]
            rec["errs"] = [l.strip()[:300] for l in txt.splitlines() if l.lstrip().startswith(("error:", "Error:", "TypeError", "ReferenceError", "SyntaxError"))][:6]
            rec["alarm"] = rc in (142, -14) or "Alarm clock" in txt
        open(out, "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n")
        return rec
    with ThreadPoolExecutor(2) as ex:
        res = list(ex.map(run, todo))
    n = len([r for r in res if r])
    print(f"ran {n}, remaining {len(todo)-n}, elapsed {time.time()-t0:.0f}s")
main()
