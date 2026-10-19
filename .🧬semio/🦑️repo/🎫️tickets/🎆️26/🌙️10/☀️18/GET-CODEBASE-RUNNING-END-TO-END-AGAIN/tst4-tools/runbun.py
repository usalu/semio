#!/usr/bin/env python3
import json, os, re, subprocess, sys, time

ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/tst4-out"


def main():
    lst, out, budget = sys.argv[1], sys.argv[2], float(sys.argv[3])
    cap = float(sys.argv[4]) if len(sys.argv) > 4 else 900
    tmo = sys.argv[5] if len(sys.argv) > 5 else "120000"
    files = [l.strip() for l in open(lst, encoding="utf-8") if l.strip()]
    done = set()
    if os.path.exists(out):
        for l in open(out, encoding="utf-8"):
            try:
                done.add(json.loads(l)["file"])
            except Exception:
                pass
    os.makedirs(G + "/logs", exist_ok=True)
    os.makedirs(G + "/artifacts", exist_ok=True)
    ART = "/Users/ueli/Documents/semio/.cache/tst4/ta" + str(os.getpid() % 1000)
    if os.environ.get("TST2_ART") == "ticket": ART = C + "/🗑️generated/ta"
    os.makedirs(ART, exist_ok=True)
    wire = subprocess.run(["bun", C + "/tst4-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True, env=dict(os.environ, SEMIO_REPO_ROOT=ROOT)).stdout
    import json as _j
    W = _j.loads(wire)
    env = dict(os.environ, NODE_PATH="/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/hoist/node_modules", SEMIO_TEST_ARTIFACT_DIR=ART, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, SEMIO_SCRIPT_PROCESS_INVOCATION=W["invocation"],SEMIO_SCRIPT_PROVIDERS=W["providers"], SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype",SEMIO_WORKSPACE_ROOT=ROOT,SEMIO_BUILD_TOOL="nx")
    t0 = time.time()
    n = 0
    for f in files:
        if f in done:
            continue
        if time.time() - t0 > budget:
            break
        s = time.time()
        try:
            extra = ["--tsconfig-override", "/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/hoist/tsconfig.json"] if os.environ.get("TST2_HOIST") == "1" else []
            p = subprocess.run(["bun", "test"] + extra + ["--timeout", tmo, "./" + f], cwd=ROOT, env=env, capture_output=True, timeout=cap)
            txt = (p.stdout or b"").decode("utf-8", "replace") + (p.stderr or b"").decode("utf-8", "replace")
            rc = p.returncode
        except subprocess.TimeoutExpired as e:
            txt = ((e.stdout or b"").decode("utf-8", "replace") if isinstance(e.stdout, bytes) else (e.stdout or "")) + ((e.stderr or b"").decode("utf-8", "replace") if isinstance(e.stderr, bytes) else (e.stderr or "")) + "\n[runner] HARD CAP"
            rc = -999
        d = time.time() - s
        def num(k):
            m = re.findall(r"^\s*(\d+) " + k + r"\b", txt, re.M)
            return int(m[-1]) if m else 0
        rec = dict(file=f, rc=rc, pass_=num("pass"), fail=num("fail"), skip=num("skip"), err=num("error"), secs=round(d, 1))
        if rc != 0:
            lp = G + "/logs/" + re.sub(r"[^A-Za-z0-9]+", "_", f)[-120:] + ".log"
            open(lp, "w", encoding="utf-8").write(txt[-60000:])
            rec["log"] = lp
        open(out, "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n")
        n += 1
    remaining = len([f for f in files if f not in done]) - n
    print(f"ran {n}, remaining {remaining}, elapsed {time.time()-t0:.0f}s")


main()
