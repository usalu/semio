#!/usr/bin/env python3
"""Runs the test groups of every staged set against one tree and writes a JSON record (counts and failing test names per group).

usage: check-tests.py <root> <out.json> [--groups a,b] [--jobs 2]

Only explicit files are passed to `bun test` (never a directory or a bare filter); at most `--jobs` groups run at once."""
import json, os, re, subprocess, sys, time
from concurrent.futures import ThreadPoolExecutor

BUILD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BZ = f"{BUILD}/🔧️tools/🏗️bazel"
BS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel"
INF = f"{BUILD}/🕸️graph/📥️inference/🧪️tests"
C = "🎓️teaching/🏛️architecture/📚️course"
P = "🎓️teaching/🛂️proctor"

GROUPS = {
    "adapter-mode-keys-paths": [f"{BZ}/🧪️tests/{n}/🟦️.ts" for n in ("📥️mode", "📥️keys", "📥️paths", "📥️platform")],
    "adapter-core": [f"{BZ}/🧪️tests/{n}/🟦️.ts" for n in ("📥️workspace", "📥️sources", "📥️actions", "📥️inventory", "📥️fingerprint", "📥️delegation", "🔏️implementation", "📥️coverage", "📥️outputs", "📥️closure")],
    "adapter-modules": [f"{BZ}/🏃️execution/🧪️tests/🟦️.ts", f"{BZ}/🧭️commands/🧪️tests/🟦️.ts", f"{BZ}/🎯️affected/🧪️tests/🟦️.ts", f"{BZ}/💾️storage/🧪️tests/🟦️.ts", f"{BS}/🩺️readiness/🧪️tests/🟦️.ts", f"{BUILD}/🧭️invocation/🧪️tests/🟦️.ts"],
    "adapter-aggregate": [f"{BZ}/🧪️tests/🟦️.ts"],
    "inference-declared": [f"{INF}/{n}/🟦️.ts" for n in ("📐️declared-inputs", "🔗️workspace-closure", "🔌️providers", "🧮️covered-inputs")],
    "inference-course-law": [f"{INF}/🎓️course-inputs/🟦️.ts"],
    "course-continuous": [f"{C}/🧪️tests/🟢️continuous/🟦️.ts", f"{C}/🧪️tests/🟢️continuous/🧾️manifests/🟦️.ts", f"{C}/🧪️tests/🎛️dashboard/🟦️.ts", f"{C}/🧪️tests/🎚️mode/🟦️.ts"],
    "course-deploy": [f"{C}/🧪️tests/🧪️deploy/🟦️.ts"],
    "course-proctor": [f"{C}/🧪️tests/🧾️proctor-product/🟦️.ts", f"{C}/🧪️tests/🏗️proctor-build/🟦️.ts"],
}


def run(root, name, files):
    present = [f for f in files if os.path.isfile(os.path.join(root, f))]
    started = time.time()
    if not present:
        return name, {"files": 0, "absent": len(files), "seconds": 0, "pass": 0, "fail": 0, "skip": 0, "failing": [], "exit": None}
    environment = {**os.environ}
    environment.pop("SEMIO_TICKET_DIR", None)
    process = subprocess.run(["bun", "test", *[f"./{f}" for f in present]], cwd=root, capture_output=True, text=True, env=environment)
    text = process.stdout + process.stderr
    count = lambda word: sum(int(m.group(1)) for m in re.finditer(rf"^\s*(\d+) {word}$", text, re.M))
    failing = sorted(set(re.sub(r" \[[0-9.]+m?s\]$", "", m.group(1)) for m in re.finditer(r"^\(fail\) (.*)$", text, re.M)))
    return name, {"files": len(present), "absent": len(files) - len(present), "seconds": round(time.time() - started), "pass": count("pass"), "fail": count("fail"), "skip": count("skip"), "failing": failing, "exit": process.returncode, "tail": text[-1500:] if process.returncode and not failing else ""}


def main():
    root, out = os.path.abspath(sys.argv[1]), sys.argv[2]
    wanted = sys.argv[sys.argv.index("--groups") + 1].split(",") if "--groups" in sys.argv else list(GROUPS)
    jobs = int(sys.argv[sys.argv.index("--jobs") + 1]) if "--jobs" in sys.argv else 2
    with ThreadPoolExecutor(max_workers=jobs) as pool:
        results = dict(pool.map(lambda name: run(root, name, GROUPS[name]), wanted))
    json.dump({"root": root, "groups": results}, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    for name, row in results.items():
        print(f"{name:26} files={row['files']:2} pass={row['pass']:4} fail={row['fail']:3} skip={row['skip']:2} {row['seconds']:4}s")


main()
