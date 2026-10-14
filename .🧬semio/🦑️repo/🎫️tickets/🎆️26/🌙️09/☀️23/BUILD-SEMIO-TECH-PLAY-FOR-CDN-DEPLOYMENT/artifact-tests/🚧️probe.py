#!/usr/bin/env python3
"""🚧️ Records, per artifact manifest, which crate of its lib-test closure the issued launcher's Source boundary refuses (no compile is awaited).

usage: 🚧️probe.py <manifests.txt> <probe.jsonl> [--jobs N] [--budget S] [--limit S] [--retry]
A manifest whose workspace cannot resolve offline (a moved path dependency) is recorded `unresolvable` without launching; --retry re-probes undecided, error and unresolvable rows.
Each manifest gets an offline lockfile refresh, then `🚦️cargo-slot.sh -- cargo test --lib --no-run` under ⏱️bounded.sh with a short limit.
Row status: refused (crate named), admitted (the launcher started compiling or finished), undecided (still planning at the limit), nolib, error.
"""
import argparse, concurrent.futures as cf, json, os, re, subprocess, time

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
p = argparse.ArgumentParser()
p.add_argument("manifests"); p.add_argument("out")
p.add_argument("--jobs", type=int, default=3); p.add_argument("--budget", type=int, default=520); p.add_argument("--limit", type=int, default=150); p.add_argument("--retry", action="store_true")
a = p.parse_args()
start = time.time()
env = {k: v for k, v in os.environ.items() if k not in ("CARGO_TARGET_DIR", "CARGO_BUILD_BUILD_DIR")}
env["CARGO_TERM_COLOR"] = "never"
done = set()
if os.path.exists(a.out):
    for l in open(a.out):
        try: j = json.loads(l)
        except Exception: continue
        if a.retry and j["status"] in ("undecided", "error", "unresolvable"): done.discard(j["manifest"])
        else: done.add(j["manifest"])
todo = [m.strip() for m in open(a.manifests) if m.strip() and m.strip() not in done]

def probe(m):
    if time.time() - start > a.budget - a.limit: return None
    text = open(f"{REPO}/{m}").read()
    package = re.search(r'^name\s*=\s*"([^"]+)"', text, re.M).group(1)
    features = ["--features", "component-app-assembly"] if re.search(r"^component-app-assembly\s*=", text, re.M) else []
    t = time.time()
    u = subprocess.run(["cargo", "update", "--workspace", "--offline", "--manifest-path", m], cwd=REPO, env=dict(env, CARGO_HOME=f"{REPO}/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/home"), capture_output=True, text=True)
    if u.returncode != 0:
        first = next((l for l in u.stderr.splitlines() if l.startswith("error")), u.stderr.strip()[-300:])
        return {"manifest": m, "package": package, "status": "unresolvable", "detail": re.sub(r"\s*\.\.\..*", "", first)[:300], "secs": round(time.time() - t)}
    r = subprocess.run(["bash", f"{TICKET}/wave0-feature-drift/⏱️bounded.sh", str(a.limit), "--", "bash", f"{TICKET}/🚦️cargo-slot.sh", "at-probe", "--", "cargo", "test", "--manifest-path", m, "--lib", "--no-run", *features], cwd=REPO, env=env, capture_output=True, text=True)
    out = r.stdout + r.stderr
    refused = re.search(r"^error: Rust crate Source refused before compiler: (\S+)", out, re.M)
    if refused: status, detail = "refused", refused.group(1)
    elif "no library targets found" in out: status, detail = "nolib", ""
    elif "Compiling " in out or r.returncode == 0: status, detail = "admitted", f"rc={r.returncode}"
    elif r.returncode == 124: status, detail = "undecided", "launcher still planning at the limit"
    else:
        tail = [l for l in out.splitlines() if l.startswith("error")]
        status, detail = "error", (tail[0] if tail else out.strip().splitlines()[-1] if out.strip() else "")[:300]
    return {"manifest": m, "package": package, "status": status, "detail": detail, "secs": round(time.time() - t)}

with open(a.out, "a") as f, cf.ThreadPoolExecutor(a.jobs) as ex:
    for row in ex.map(probe, todo):
        if row is None: continue
        f.write(json.dumps(row, ensure_ascii=False) + "\n"); f.flush()
        print(f"{row['status']:8} {row['secs']:4}s {row['package']} {row['detail']}", flush=True)
