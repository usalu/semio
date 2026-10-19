#!/usr/bin/env python3
import json, os, re, subprocess, sys, time
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/tst2-out"
ART = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/ta"

def main():
    lst, out, budget = sys.argv[1], sys.argv[2], float(sys.argv[3])
    cap = float(sys.argv[4]) if len(sys.argv) > 4 else 1500
    cfgs = [l.strip() for l in open(lst, encoding="utf-8") if l.strip()]
    done = set()
    if os.path.exists(out):
        for l in open(out, encoding="utf-8"):
            try: done.add(json.loads(l)["file"])
            except Exception: pass
    os.makedirs(G + "/logs", exist_ok=True); os.makedirs(ART, exist_ok=True)
    wire = subprocess.run(["bun", C + "/tst2-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout
    t0 = time.time(); n = 0
    for cfg in cfgs:
        if cfg in done: continue
        if time.time() - t0 > budget: break
        abscfg = os.path.join(ROOT, cfg)
        cwd = os.path.normpath(os.path.join(os.path.dirname(abscfg), "..", ".."))
        pkg = os.path.join(cwd, "📦️packages", "🟦️typescript")
        if os.path.isdir(pkg): cwd = pkg
        policy = dict(version=1, cwd=cwd, toolPath=ROOT + "/node_modules/vitest/vitest.mjs", runtime=subprocess.run(["which","bun"],capture_output=True,text=True).stdout.strip(), coverageRuntime=subprocess.run(["which","bun"],capture_output=True,text=True).stdout.strip(), cacheRoot=G + "/vtcache", coverageDirectory=G + "/vtcov", budgetMs=120000)
        env = dict(os.environ, SEMIO_VITEST_POLICY=json.dumps(policy), SEMIO_TEST_ARTIFACT_DIR=ART, SEMIO_SCRIPT_PROCESS_INVOCATION=wire, SEMIO_TEST_LEVEL="long", NO_COLOR="1", CI="1")
        s = time.time()
        if os.environ.get("TST2_HOIST") == "1":
            names = json.load(open(C + "/tst2-tools/hoist-names.json", encoding="utf-8"))
            Hh = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/hoist/node_modules"
            wd = C + "/🗑️generated/vtwrap"; os.makedirs(wd, exist_ok=True)
            wp = wd + "/" + re.sub(r"[^A-Za-z0-9]+", "_", cfg)[-90:] + ".mts"
            rows = [["^" + re.escape(n) + "(?=/|$)", Hh + "/" + n] for n in names]
            open(wp, "w", encoding="utf-8").write('import { mergeConfig } from "vitest/config";\nimport base from ' + json.dumps(abscfg) + ';\nconst rows: [string, string][] = ' + json.dumps(rows) + ';\nexport default mergeConfig(base as any, { resolve: { alias: rows.map(([find, replacement]) => ({ find: new RegExp(find), replacement })) } });\n')
            abscfg = wp
        try:
            p = subprocess.run(["node", ROOT + "/node_modules/vitest/vitest.mjs", "run", "--config", abscfg, "--testTimeout", "120000", "--hookTimeout", "120000", "--reporter=default"], cwd=cwd, env=env, capture_output=True, text=True, timeout=cap)
            txt = (p.stdout or "") + (p.stderr or ""); rc = p.returncode
        except subprocess.TimeoutExpired as e:
            txt = "[runner] HARD CAP"; rc = -999
        d = time.time() - s
        def grab(k):
            m = re.findall(r"(\d+) " + k, txt)
            return int(m[-1]) if m else 0
        tests = re.findall(r"Tests\s+(.*)", txt)
        files = re.findall(r"Test Files\s+(.*)", txt)
        rec = dict(file=cfg, rc=rc, tests=tests[-1].strip() if tests else "", files=files[-1].strip() if files else "", secs=round(d, 1))
        if rc != 0:
            lp = G + "/logs/vt_" + re.sub(r"[^A-Za-z0-9]+", "_", cfg)[-110:] + ".log"
            open(lp, "w", encoding="utf-8").write(txt[-80000:]); rec["log"] = lp
        open(out, "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n"); n += 1
    print(f"ran {n}, elapsed {time.time()-t0:.0f}s")
main()
