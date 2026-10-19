#!/usr/bin/env python3
"""🧭 Resumable one-process-per-file bun test census with a per-file cap and a cargo guard."""
import json, os, re, signal, subprocess, sys, time

ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/census2"
ART = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/ta2"
ENVWIRE_FALLBACK = ""


def envelope():
    return subprocess.run(["bun", C + "/census2-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout


def cargo_pgids():
    out = subprocess.run(["ps", "-axo", "pgid=,comm="], capture_output=True, text=True).stdout
    res = {}
    for line in out.splitlines():
        parts = line.strip().split(None, 1)
        if len(parts) != 2:
            continue
        name = os.path.basename(parts[1])
        if name in ("cargo", "rustc"):
            res.setdefault(int(parts[0]), []).append(name)
    return res


def run_one(f, env, cap, log_path):
    with open(log_path, "wb") as log:
        proc = subprocess.Popen(["bun", "test", "--timeout", "120000", "./" + f], cwd=ROOT, env=env,
                                stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
    t0 = time.time()
    reason = None
    last_ps = 0.0
    while True:
        rc = proc.poll()
        if rc is not None:
            break
        now = time.time()
        if now - last_ps >= 2.0:
            last_ps = now
            if proc.pid in cargo_pgids():
                reason = "CARGO-SPAWNED"
                os.killpg(proc.pid, signal.SIGKILL)
                proc.wait()
                rc = -998
                break
        if now - t0 > cap:
            reason = "TIMEOUT"
            os.killpg(proc.pid, signal.SIGKILL)
            proc.wait()
            rc = -999
            break
        time.sleep(0.5)
    return rc, reason, round(time.time() - t0, 1)


def count(k, txt):
    m = re.findall(r"^\s*(\d+) " + k + r"\b", txt, re.M)
    return int(m[-1]) if m else 0


def main():
    lst, out, budget = sys.argv[1], sys.argv[2], float(sys.argv[3])
    cap = float(sys.argv[4]) if len(sys.argv) > 4 else 200
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
    wire = envelope()
    env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=ART, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT,
               SEMIO_SCRIPT_PROCESS_INVOCATION=wire, SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype")
    t0 = time.time()
    n = 0
    for f in files:
        if f in done:
            continue
        if time.time() - t0 > budget:
            break
        lp = G + "/logs/" + f"{n:05d}_" + re.sub(r"[^A-Za-z0-9]+", "_", f)[-100:] + ".log"
        rc, reason, secs = run_one(f, env, cap, lp)
        txt = open(lp, encoding="utf-8", errors="replace").read()
        rec = dict(file=f, rc=rc, reason=reason, pass_=count("pass", txt), fail=count("fail", txt),
                   skip=count("skip", txt), todo=count("todo", txt), secs=secs, log=lp)
        if rc == 0 and not reason:
            os.remove(lp)
            rec["log"] = None
        open(out, "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n")
        n += 1
    remaining = len([f for f in files if f not in done]) - n
    print(f"ran {n}, remaining {remaining}, elapsed {time.time()-t0:.0f}s")


main()
