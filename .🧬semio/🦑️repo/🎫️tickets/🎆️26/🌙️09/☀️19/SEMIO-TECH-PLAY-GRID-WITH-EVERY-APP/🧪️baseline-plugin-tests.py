#!/usr/bin/env python3
"""🧪️ Per-crate native test baseline for every crate under ✏️s/🔌️plugins: `python3 🧪️baseline-plugin-tests.py <metadata.json> <outDir> [plugin-substring…]`."""
import json, os, re, subprocess, sys, time
metadata, out = sys.argv[1], sys.argv[2]
only = sys.argv[3:]
root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/"
os.makedirs(out, exist_ok=True)
packages = [p for p in json.load(open(metadata))["packages"] if p["manifest_path"].startswith(root)]
if only: packages = [p for p in packages if any(o in p["manifest_path"] for o in only)]
env = {**os.environ, "CARGO_INCREMENTAL": "0", "RUST_BACKTRACE": "0"}
summary_path = os.path.join(out, "summary.tsv")
done = set()
if os.path.exists(summary_path):
    done = {line.split("\t")[0] for line in open(summary_path, encoding="utf-8")}
for p in packages:
    name = p["name"]
    if name in done: continue
    plugin = p["manifest_path"][len(root):].split("/")[0]
    features = ["--features", "component-app-assembly"] if "component-app-assembly" in p.get("features", {}) else []
    started = time.time()
    proc = subprocess.run(["cargo", "test", "-p", name, "--lib", "--tests", "--no-fail-fast", *features, "--", "--test-threads=4"], cwd="/Users/ueli/Documents/semio", env=env, capture_output=True, text=True)
    text = proc.stdout + proc.stderr
    open(os.path.join(out, f"{name}.txt"), "w", encoding="utf-8").write(text)
    passed = sum(int(x) for x in re.findall(r"test result: \w+\. (\d+) passed", text))
    failed = sum(int(x) for x in re.findall(r"; (\d+) failed", text))
    compile_error = "error: could not compile" in text or ("error[" in text and "test result" not in text)
    aborted = "process didn't exit successfully" in text and "signal" in text
    status = "compile-error" if compile_error and passed == 0 else ("aborted" if aborted else ("ok" if proc.returncode == 0 else "fail"))
    with open(summary_path, "a", encoding="utf-8") as f:
        f.write(f"{name}\t{plugin}\t{status}\t{passed}\t{failed}\t{int(time.time()-started)}s\n")
    print(name, status, passed, failed, flush=True)
