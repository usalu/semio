#!/usr/bin/env python3
"""🧪️ Compiles the lib test target of each listed artifact manifest (through 🚦️cargo-slot.sh onto the issued `bun repo cargo --store=shared` launcher; logs and results under ⚡️cache/at) and records errors and the test binary.

usage: 🧪️sweep.py <manifests.txt> <results.jsonl> [--lane NAME] [--only SUBSTR] [--force] [--retry] [--budget SECONDS]
Before each compile the artifact workspace lockfile absorbs moved path members offline (`cargo update --workspace --offline`, no compiler), because the launcher fetches `--locked`.
--retry recompiles manifests whose latest row is fail or blocked; --test runs 🧪️per-test.py on each crate right after it compiles (one crate's test binary at a time).
A cargo pass still running at the budget is interrupted (bounded.sh, exit 124), left unrecorded, and resumes from the build dir on the next call.
Each result row: {manifest, package, features, status: ok|fail|blocked|nolib, errors, dep_errors, codes, binary, secs}; blocked = a dependency outside the crate failed (absolute path).
"""
import argparse, collections, json, os, re, subprocess, time

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
CACHE = f"{REPO}/.🧬semio/🦑️repo/⚡️cache/at"
p = argparse.ArgumentParser()
p.add_argument("manifests"); p.add_argument("results")
p.add_argument("--lane", default="a"); p.add_argument("--budget", type=int, default=560); p.add_argument("--only", default=""); p.add_argument("--force", action="store_true"); p.add_argument("--retry", action="store_true"); p.add_argument("--test", action="store_true")
a = p.parse_args()
env = {k: v for k, v in os.environ.items() if k not in ("CARGO_TARGET_DIR", "CARGO_BUILD_BUILD_DIR")}
env["CARGO_TERM_COLOR"] = "never"
done = set()
if os.path.exists(a.results) and not a.force:
    for l in open(a.results):
        try: j = json.loads(l)
        except Exception: continue
        if a.retry and j["status"] in ("fail", "blocked"): done.discard(j["manifest"])
        else: done.add(j["manifest"])
start = time.time()

def run_tests(only=None):
    """▶️ Runs 🧪️per-test.py inside what is left of the budget: one package, or every compiled package with unrecorded tests."""
    left = int(a.budget - (time.time() - start))
    if left < 30: return
    cmd = ["python3", f"{TICKET}/artifact-tests/🧪️per-test.py", a.results, f"{CACHE}/tests", "--timeout", "120", "--budget", str(left)]
    if only: cmd += ["--only", only, "--exact-package"]
    subprocess.run(cmd, cwd=REPO)

if a.test and os.path.exists(a.results): run_tests()
manifests = [m.strip() for m in open(a.manifests) if m.strip() and a.only in m]
os.makedirs(f"{CACHE}/logs", exist_ok=True)
for m in manifests:
    if m in done: continue
    text = open(f"{REPO}/{m}").read()
    package = re.search(r'^name\s*=\s*"([^"]+)"', text, re.M).group(1)
    features = re.search(r"^component-app-assembly\s*=", text, re.M) is not None
    left = int(a.budget - (time.time() - start))
    if left < 30: print("budget", flush=True); break
    cmd = ["bash", f"{TICKET}/wave0-feature-drift/⏱️bounded.sh", str(left), "--", "bash", f"{TICKET}/🚦️cargo-slot.sh", f"at-{a.lane}", "--", "cargo", "test", "--manifest-path", m, "--lib", "--no-run", "--message-format", "json-diagnostic-short"]
    if features: cmd += ["--features", "component-app-assembly"]
    t = time.time()
    subprocess.run(["cargo", "update", "--workspace", "--offline", "--manifest-path", m], cwd=REPO, env=dict(env, CARGO_HOME=f"{REPO}/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/home"), capture_output=True)
    r = subprocess.run(cmd, cwd=REPO, env=env, capture_output=True, text=True)
    if r.returncode == 124: print(f"deadline {package}", flush=True); break
    errors, dep_errors, codes, binary, rendered = 0, 0, collections.Counter(), None, []
    for line in r.stdout.splitlines():
        try: j = json.loads(line)
        except Exception: continue
        if j.get("reason") == "compiler-message" and j["message"]["level"] == "error" and not j["message"]["message"].startswith("aborting due to"):
            errors += 1; dep_errors += j["message"]["rendered"].startswith("/"); codes[(j["message"].get("code") or {}).get("code") or "other"] += 1; rendered.append(j["message"]["rendered"])
        if j.get("reason") == "compiler-artifact" and j.get("executable") and j.get("profile", {}).get("test"):
            binary = j["executable"]
    nolib = "no library targets found" in r.stderr
    status = "nolib" if nolib else ("ok" if r.returncode == 0 and binary else ("blocked" if dep_errors else "fail"))
    with open(f"{CACHE}/logs/{package}.log", "w") as log: log.write("".join(rendered) + "\n--- stderr ---\n" + r.stderr[-20000:])
    row = {"manifest": m, "package": package, "features": features, "status": status, "errors": errors, "dep_errors": dep_errors, "codes": dict(codes), "binary": binary, "secs": round(time.time() - t)}
    with open(a.results, "a") as out: out.write(json.dumps(row, ensure_ascii=False) + "\n")
    print(f"{status:5} {errors:4} {row['secs']:5}s {package}", flush=True)
    if a.test and status == "ok": run_tests(package)
