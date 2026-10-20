#!/usr/bin/env python3
"""✅️ Overlay verification (waits for load < 30 before each heavy step, nice -n 10, one process at a time). Logs: C/🗑️generated/ide-free/verify/.
Steps: census (bun harness + negative control), go-build, go-test (changed packages), ts-syntax (bun build --no-bundle on changed TS), json (parse all changed JSON). Summary in verify-summary.json."""
import json, os, shutil, subprocess, sys, time
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
O_ENV = os.environ.get("VERIFY_ROOT")
ROOT = os.path.abspath(os.path.join(C, *[".."] * 7)); O = O_ENV or "/private/tmp/ide-free/root"
LOG = os.path.join(C, "🗑️generated", "ide-free", os.environ.get("VERIFY_LOG", "verify")); os.makedirs(LOG, exist_ok=True)
M = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
plan = json.load(open(os.path.join(HERE, "staged", "plan.json"), encoding="utf-8"))
summary = {}

def load():
    return float(subprocess.run(["sysctl", "-n", "vm.loadavg"], capture_output=True, text=True).stdout.split()[1])

def wait():
    for _ in range(40):
        if load() < 30: return
        time.sleep(30)
    raise SystemExit("load stayed above 30 for 20 minutes")

def run(name, cmd, cwd, env=None, timeout=1500):
    wait(); t0 = time.time()
    e = dict(os.environ, **(env or {}))
    r = subprocess.run(["nice", "-n", "10"] + cmd, cwd=cwd, env=e, capture_output=True, text=True, timeout=timeout)
    open(os.path.join(LOG, name + ".log"), "w", encoding="utf-8").write(r.stdout + "\n--- stderr ---\n" + r.stderr)
    summary[name] = {"rc": r.returncode, "seconds": round(time.time() - t0, 1), "tail": (r.stdout + r.stderr)[-600:]}
    print(name, r.returncode, summary[name]["seconds"], flush=True)
    json.dump(summary, open(os.path.join(LOG, "verify-summary.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    return r.returncode

steps = sys.argv[1:] or ["json", "census", "ts-syntax", "go-build", "go-test", "go-target", "go-ide-fixture"]
changed = set()
mv = {m["src"]: m["dst"] for m in plan["moves"]}
for e in plan["edits"] + plan["writes"]: changed.add(mv.get(e["path"], e["path"]))
for m in plan["moves"]: changed.add(m["dst"])
for c in plan["creates"]: changed.add(c["path"])
changed = {p for p in changed if os.path.isfile(os.path.join(O, p))}

if "json" in steps:
    bad = []
    for p in sorted(changed):
        if p.endswith(".json") and "tsconfig" not in p:
            try: json.load(open(os.path.join(O, p), encoding="utf-8"))
            except ValueError as err: bad.append((p, str(err)[:80]))
    summary["json"] = {"checked": sum(1 for p in changed if p.endswith(".json")), "bad": bad}; print("json", len(bad), flush=True)

if "census" in steps:
    for link, target in (("node_modules", os.path.join(ROOT, "node_modules")), ("🧰️framework/🔨️modules/🧬️schema", os.path.join(ROOT, "🧰️framework/🔨️modules/🧬️schema"))):
        d = os.path.join(O, link); os.makedirs(os.path.dirname(d), exist_ok=True)
        if not os.path.lexists(d): os.symlink(os.path.realpath(target), d)
    subprocess.run([sys.executable, os.path.join(HERE, "census-harness.py"), O], check=True, capture_output=True)
    h = os.path.join(O, M, "📚️library/🧪️tests/🎛️dashboard-control/harness.test.ts")
    run("census-positive", ["bun", "test", h], O)
    # negative control: the classifier without the new fixture rows must fail on the same probes
    fx = os.path.join(O, M, "📚️library/🧪️tests/🎛️dashboard-control/🧫️fixtures/🔍️ide-census.json")
    keep = open(fx, encoding="utf-8").read(); doc = json.loads(keep)
    doc["directories"] = [d for d in doc["directories"] if d not in (".cursor", ".windsurf")]; doc["directoryPaths"] = ["x/y"]
    doc["pathCases"] = [r for r in doc["pathCases"] if ".cursor" not in r["path"] and ".windsurf" not in r["path"] and ".github/hooks" not in r["path"]]
    open(fx, "w", encoding="utf-8").write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
    rc = run("census-negative-control", ["bun", "test", h], O)
    open(fx, "w", encoding="utf-8").write(keep)
    summary["census-negative-control"]["expected_failure"] = rc != 0

if "ts-syntax" in steps:
    bad = []
    for p in sorted(changed):
        if p.endswith((".ts", ".tsx", ".mjs")):
            wait()
            r = subprocess.run(["nice", "-n", "10", "bun", "build", "--no-bundle", os.path.join(O, p), "--outfile", "/private/tmp/ide-free/syntax.out.js"], capture_output=True, text=True, cwd=O)
            if r.returncode != 0: bad.append((p, (r.stderr or r.stdout)[-200:]))
    summary["ts-syntax"] = {"bad": bad}; print("ts-syntax bad", len(bad), flush=True)

GOENV = {"GOWORK": os.path.join(O, "go.work"), "GOPROXY": "off", "GOFLAGS": "", "GOTOOLCHAIN": "local", "GOPLAN_TAXONOMY": os.path.join(O, M, "📚️library/🔣️taxonomy.json")}
GOMODS = [("tickets", "🎫️tickets/📦️packages/🐹️go", "🎫️tickets/🧪️tests", "PlanSource|ApplyTicketPlan|PostTicketPlan|StripPlan|FormatPlan|Close|Reopen|Lifecycle"),
          ("providers", "🧩️providers/📦️packages/🐹️go", "🧩️providers/🧪️tests", "."), ("hooks", "🪝️hooks/📦️packages/🐹️go", "🪝️hooks/🧪️tests", "."),
          ("workspace", "🏠️workspace/📦️packages/🐹️go", "🏠️workspace/🧪️tests", "Ide|Formatter"), ("cli-pkg", "⌨️cli/📦️packages/🐹️go", "⌨️cli/🧪️tests", "Mcp|Description|Factory"),
          ("client-cli", "💻️client/⌨️cli", None, "PlanSource|ApplyTicketPlan|PostTicketPlan|AgentClientProvider|FormatCopilot|CopilotEvent|Provider"), ("client-mcp", "💻️client/🔌️mcp", None, ".")]
def plan_for(name, rel, extra=None):
    out = os.path.join(LOG, "plan-" + name + ".json")
    r = subprocess.run([sys.executable, os.path.join(HERE, "goplan.py"), os.path.join(O, M, rel), out, "--tests", "--workspace", os.path.join(O, "go.work")] + (["--extra-tests", os.path.join(O, M, extra.split("|")[0])] + (["--only", extra.split("|")[1]] if "|" in extra else []) if extra else []), env=dict(os.environ, **GOENV), capture_output=True, text=True)
    if r.returncode != 0: raise SystemExit("goplan failed for " + name + ": " + r.stderr[-300:])
    return out
if "go-build" in steps:
    for name, rel, extra, _ in GOMODS: run("go-vet-" + name, ["go", "vet", "-overlay=" + plan_for(name, rel, extra), "."], os.path.join(O, M, rel), GOENV)
if "go-test" in steps:
    for name, rel, extra, pat in GOMODS: run("go-test-" + name, ["go", "test", "-count=1", "-timeout", "20m", "-overlay=" + plan_for(name, rel, extra), "-run", pat, "."], os.path.join(O, M, rel), GOENV, 1800)

if "go-ide-fixture" in steps:
    rel = "🏠️workspace/📦️packages/🐹️go"; plan_path = plan_for("workspace", rel, None)
    doc = json.load(open(plan_path, encoding="utf-8")); doc["Replace"][os.path.join(os.path.realpath(os.path.join(O, M, rel)), "zz_ide_free_fixture_test.go")] = os.path.join(HERE, "ide_fixture_test.go.txt")
    json.dump(doc, open(plan_path, "w", encoding="utf-8"), ensure_ascii=False)
    run("go-ide-fixture", ["go", "test", "-count=1", "-overlay=" + plan_path, "-run", "TestIdeFreeAuthorityFixture", "-v", "."], os.path.join(O, M, rel), dict(GOENV, IDE_FREE_FIXTURE=os.path.join(O, M, "🎛️dashboard/🧪️tests/🧫️fixtures/🧭️authority/🔣️.json")))

if "go-target" in steps:
    for name, rel, extra, _ in GOMODS:
        if name in ("tickets", "client-cli"):
            run("go-target-" + name, ["go", "test", "-count=1", "-v", "-overlay=" + plan_for(name, rel, extra), "-run", "RefusesCursor|KiroSpecID|TestApplyTicketPlanFromIDs|StripPlan|FormatPlan|TestPostTicketPlanComment", "."], os.path.join(O, M, rel), GOENV)
json.dump(summary, open(os.path.join(LOG, "verify-summary.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("VERIFY DONE")
