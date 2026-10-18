import sys, os, subprocess, hashlib, json, time, hashlib
from concurrent.futures import ThreadPoolExecutor
import xml.etree.ElementTree as ET
R = "/Users/ueli/Documents/semio"
F = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G = F + "/🗑️generated"
label, listfile = sys.argv[1], sys.argv[2]
jobs = int(sys.argv[3]) if len(sys.argv) > 3 else 4
limit = int(sys.argv[4]) if len(sys.argv) > 4 else 420
files = [l for l in open(listfile, encoding="utf8").read().split("\n") if l]
out = f"{G}/tsl2-{label}"
os.makedirs(out, exist_ok=True)
def cwd_for(f):
    d = os.path.dirname(os.path.join(R, f))
    while d.startswith(R) and d != R:
        if os.path.exists(os.path.join(d, "package.json")) and os.path.exists(os.path.join(d, "node_modules")) or os.path.exists(os.path.join(d, "package.json")):
            return d
        d = os.path.dirname(d)
    return R
def key(f): return hashlib.sha1(f.encode()).hexdigest()[:12]
def one(f):
    k = key(f)
    xml = f"{out}/{k}.xml"; log = f"{out}/{k}.log"
    env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=f"{R}/.🧬semio/🦑️repo/⚡️cache/🧪️tests/tsl2-{label}-{k}", SEMIO_BUILD_MODE="prototype")
    os.makedirs(env["SEMIO_TEST_ARTIFACT_DIR"], exist_ok=True)
    env["GIT_CONFIG_COUNT"] = "1"; env["GIT_CONFIG_KEY_0"] = "core.fsmonitor"; env["GIT_CONFIG_VALUE_0"] = "false"
    if "transaction-v2" in f:
        import uuid
        u = str(uuid.uuid4()); env["SEMIO_TRANSACTION_V2_RUN_ID"] = "1-" + u
        env["SEMIO_TRANSACTION_V2_RUN_ROOT"] = os.path.join(env["SEMIO_TEST_ARTIFACT_DIR"], "🧾️runs", "🔖️" + u)
        os.makedirs(env["SEMIO_TRANSACTION_V2_RUN_ROOT"], exist_ok=True)
    if os.environ.get("TSL2_ENVELOPE"):
        env["SEMIO_SCRIPT_PROCESS_INVOCATION"] = open(G + "/tsl2-wire.txt", encoding="utf8").read().strip()
    t0 = time.time(); rc = None
    for attempt in range(2):
        if os.path.exists(xml): os.remove(xml)
        cmd = ["nice", "-n", "10", "perl", "-e", f"alarm {limit}; exec @ARGV", "--", "bun", "test", "--timeout", "120000", "--reporter=junit", f"--reporter-outfile={xml}", os.path.join(R, f)]
        p = subprocess.run(cmd, cwd=R, env=env, capture_output=True, text=True, errors="replace", start_new_session=True)
        open(log, "w", encoding="utf8").write(p.stdout + "\n--- stderr ---\n" + p.stderr)
        rc = p.returncode
        if rc in (-11, 139, 134, -6) or "Segmentation fault" in p.stderr or "panic" in p.stderr[:400].lower() and not os.path.exists(xml):
            continue
        break
    rows = []
    if os.path.exists(xml):
        try:
            for tc in ET.parse(xml).iter("testcase"):
                st = "failed" if (tc.find("failure") is not None or tc.find("error") is not None) else ("skipped" if tc.find("skipped") is not None else "passed")
                rows.append((st, tc.get("name") or ""))
        except Exception as e:
            rows.append(("xmlerror", str(e)))
    return {"file": f, "key": k, "rc": rc, "secs": round(time.time() - t0, 1), "attempt": attempt, "rows": rows}
res = []
with ThreadPoolExecutor(jobs) as ex:
    for r in ex.map(one, files):
        res.append(r)
        n = {}
        for s, _ in r["rows"]: n[s] = n.get(s, 0) + 1
        print(r["rc"], r["secs"], n, r["file"].split("🔨️modules/")[-1][:150], flush=True)
json.dump(res, open(f"{G}/tsl2-{label}.json", "w", encoding="utf8"), ensure_ascii=False)
