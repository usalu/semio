"""🔌 agents3: proves the REAL-run branch of `run.py` (gating, restore check, verdict.json, exit code) without a build: `run_harness` is replaced by a copy of the dry run's positive result, the output goes to `🗑️generated/agents3/wiring-real/` (never the final `verdict.json`).
`python3 -I wiring_real.py` (needs one `run.sh --dry-run` first)."""
import importlib.util, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("agents3_run", os.path.join(HERE, "run.py"))
run = importlib.util.module_from_spec(spec)
spec.loader.exec_module(run)
SOURCE = os.path.join(run.GEN, "dry", "drypos", "run")
run.GEN = os.path.join(run.GEN, "wiring-real")
shutil.rmtree(run.GEN, ignore_errors=True)
os.makedirs(run.GEN)


def replay(args, tag, outdir, env_extra):
    shutil.copytree(SOURCE, outdir)
    return 0


run.run_harness = replay
args = type("Args", (), {"tag": "wiring", "wait_quiet": False, "prime": False})()
code = run.real_run(args)
verdict = json.load(open(os.path.join(run.GEN, "verdict.json"), encoding="utf8"))
checks = [("exit code 0 for a PASS verdict", code == 0), ("verdict.json mode real", verdict["mode"] == "real"), ("every key present", {"overall", "agents", "criteria", "host", "starvation", "swapStorm", "languages", "restoredExact", "editTargetsDirty"} <= set(verdict)), ("three agents recorded", [a["agent"] for a in verdict["agents"]] == ["A", "B", "C"]), ("live targets clean", verdict["editTargetsDirty"] == [])]
for name, ok in checks:
    print("%-40s %s" % (name, "ok" if ok else "FAIL"))
print("REAL-BRANCH WIRING", "OK" if all(ok for _, ok in checks) else "FAILED", "->", os.path.join(run.GEN, "verdict.json"))
sys.exit(0 if all(ok for _, ok in checks) else 1)
