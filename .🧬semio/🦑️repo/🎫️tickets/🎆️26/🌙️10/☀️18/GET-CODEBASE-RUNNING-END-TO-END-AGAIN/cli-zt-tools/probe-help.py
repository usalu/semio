"""Runs `--help` of every registered command on an overlay (no node_modules) one process at a time; usage: probe-help.py <overlay> <out.json> [--root-script-only|--ops-only]"""
import json, os, subprocess, sys, time
overlay, out, *flags = sys.argv[1:]
cli = "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts"
catalog = json.load(open(os.path.join(overlay, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json"), encoding="utf-8"))
ops = [row["name"] for row in catalog["operations"]]
registry = os.path.join(overlay, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🔣️.json")
verbs = [row["id"] for row in json.load(open(registry, encoding="utf-8"))["commands"]] if os.path.exists(registry) else [name for name in json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "entries.json"))) if not name.startswith("_") and name not in ("repo", "policy")]
env = {key: value for key, value in os.environ.items() if not key.startswith(("SEMIO_SCRIPT", "SEMIO_BUILD_INV"))}
env["SEMIO_LANGUAGE"] = "en"
jobs = [] if "--root-script-only" in flags else [("repo-top", [cli, "repo", "--help"])] + [(f"repo {name}", [cli, "repo", name, "--help"]) for name in ops]
if "--ops-only" not in flags:
    jobs += [("root-top", ["./📜️script.ts", "--help"]), ("root-top-de", ["./📜️script.ts", "--help", "--lang=de"])] + [(f"root {name}", ["./📜️script.ts", name, "--help"]) for name in verbs] + [("root verify-de", ["./📜️script.ts", "verify", "--help", "--lang=de"])]
rows = []
for label, argv in jobs:
    start = time.time()
    try:
        done = subprocess.run(["perl", "-e", "alarm 90; exec @ARGV", "--", "bun", *argv], cwd=overlay, env=env, capture_output=True, text=True, timeout=100)
        status, stdout, stderr = done.returncode, done.stdout, done.stderr
    except subprocess.TimeoutExpired:
        status, stdout, stderr = -9, "", "timeout"
    rows.append({"label": label, "status": status, "seconds": round(time.time() - start, 2), "stdout": stdout[:300], "stderr": next((line for line in stderr.splitlines() if line.startswith(("error:", "Error", "ENOENT")) or "Cannot find" in line), (stderr.strip().splitlines() or [""])[0])[:300]})
    print(f"{status:4} {rows[-1]['seconds']:6}s {label:22} {(rows[-1]['stderr'] or (stdout.splitlines() or [''])[0])[:120]}", flush=True)
json.dump(rows, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({"ok": sum(1 for r in rows if r["status"] == 0), "failed": [r["label"] for r in rows if r["status"] != 0]}, ensure_ascii=False))
