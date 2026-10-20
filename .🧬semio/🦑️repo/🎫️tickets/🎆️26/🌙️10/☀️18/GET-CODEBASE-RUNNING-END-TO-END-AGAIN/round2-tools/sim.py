#!/usr/bin/env python3
"""Simulates the round-2 apply series on a private overlay (never writes into the checkout).

usage: sim.py [--order a,b,c] [--keep] [--tag NAME] [--seed-extra FILE...]
  Seeds /private/tmp/i2/ov-<tag> with byte copies (no links, no clones) of every live file a set reads, copies each set's tool directory to /private/tmp/i2/tools-<tag>/<set> with the live root literal replaced by the overlay,
  runs the sets in order against the overlay in their write mode, and records per set the exit code, the output tail and the files it changed (sha256 inventory diff).
  Result: C/🗑️generated/round2-apply/sim-<tag>/result.json (+ one log per set)."""
import glob, hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
TICKETS = os.path.abspath(os.path.join(C, "..", "..", "..", ".."))
P = glob.glob(TICKETS + "/*/*/*/ARCHITECTURE-COURSE-ON-BAZEL")[0]
GEN = C + "/🗑️generated/round2-apply"
BUN = shutil.which("bun") or "bun"
PY = sys.executable
args = sys.argv[1:]
opt = lambda name, default=None: args[args.index(name) + 1] if name in args else default
TAG = opt("--tag", "a")
OV = f"/private/tmp/i2/ov-{TAG}"
TOOLS = f"/private/tmp/i2/tools-{TAG}"
OUT = f"{GEN}/sim-{TAG}"

DIRS = {
    "orphan-child": C + "/orphan-child-tools", "own-output": C + "/own-output-tools/staged", "testfix-defect": C + "/testfix-defect-tools",
    "hub-bzl": C + "/hub-bzl-tools", "demo-bzl": C + "/demo-bzl-tools", "docs-build": C + "/docs-build-tools", "play-bzl": C + "/play-bzl-tools",
    "bzl-rbe": C + "/round2-tools/staged/bzl-rbe", "parity": C + "/parity-tools", "testfix-law": C + "/testfix-law-tools", "mem-gov": C + "/mem-gov-tools",
    "sync-incr": C + "/sync-incr-tools", "cache-law": C + "/round2-tools/staged/cache-law", "bzl-incr": C + "/round2-tools/staged/bzl-incr", "pg-chain": C + "/pg-chain-tools", "boot-sweep": C + "/boot-sweep-tools", "bzl-default": C + "/bzl-default-tools", "bzl-default-manifest": C + "/bzl-default-tools", "bzl-default-flip": C + "/round2-tools/staged/bzl-default-flip", "input-precision": C + "/input-precision-tools",
}
NOSED = {"input-precision": {"gen-rules.py"}, "parity": {"overlay.py"}}
SKIP_COPY = {"specs", "node_modules", "backup", "__pycache__"}


def shim(name):
    source, target = DIRS[name], f"{TOOLS}/{name}"
    shutil.rmtree(target, ignore_errors=True)
    shutil.copytree(source, target, ignore=lambda d, names: [n for n in names if n in SKIP_COPY or n.endswith(".pyc")])
    for base, _, files in os.walk(target):
        for file in files:
            if not file.endswith((".py", ".ts", ".sh", ".cjs", ".mjs")) or file in NOSED.get(name, ()):
                continue
            path = os.path.join(base, file)
            text = open(path, encoding="utf8").read()
            new = text.replace(LIVE, OV)
            if name == "orphan-child" and file == "apply.py":
                new = new.replace("if live:", "if False:")
            if name == "boot-sweep" and file == "apply.py":
                new = new.replace("if live and write:", "if False:")
            if name == "pg-chain" and file == "apply.py":
                new = new.replace('raise SystemExit("repository root not found")', 'return "%s"' % OV)
            if name == "demo-bzl" and file == "apply.py":
                new = new.replace("LIVE = _root()", f'LIVE = "{OV}"')
            if name == "sync-incr" and file == "apply.py":
                new = new.replace("running = busy() if guard else []", "running = []")
            if new != text:
                open(path, "w", encoding="utf8").write(new)
    return target


def commands(name):
    t = f"{TOOLS}/{name}"
    extra = [a.split("=", 1)[1] for a in args if a.startswith(f"--arg-{name}=")]
    return extra_args(name, t) + extra


def extra_args(name, t):
    return {
        "orphan-child": [PY, t + "/apply.py", "--root", OV],
        "own-output": [BUN, t + "/apply.ts", "--root", OV, "--yes", "--ignore-running", "--backup", f"/private/tmp/i2/backup-{TAG}/own-output"],
        "testfix-defect": [PY, t + "/apply.py", "--write"],
        "cache-law": [PY, t + "/apply.py", "--root", OV, "--write"],
        "bzl-incr": [PY, t + "/apply.py", "--write", "--root", OV],
        "pg-chain": [PY, t + "/apply.py", "apply", "--rebase", "--write", "--root", OV],
        "boot-sweep": [PY, t + "/apply.py", "--write", "--root", OV],
        "bzl-nocache": [BUN, C + "/round2-tools/staged/bzl-nocache/apply.ts", "--write"],
        "bzl-default": [PY, t + "/apply.py", "--root", OV, "--group", "all", "--write", "--ignore-running"],
        "bzl-default-manifest": [PY, t + "/apply.py", "--root", OV, "--group", "manifest", "--write", "--ignore-running"],
        "bzl-default-flip": [PY, t + "/apply.py", "--root", OV, "--group", "flip", "--flip-gate-passed", "--write", "--ignore-running"],
        "hub-bzl": [PY, t + "/apply.py", "--write"],
        "demo-bzl": [PY, t + "/apply.py", "--write", "--root", OV],
        "docs-build": [PY, t + "/apply.py", "--write", "--root", OV],
        "play-bzl": [PY, t + "/apply.py", "--write"],
        "bzl-rbe": [PY, t + "/apply.py"],
        "parity": [PY, t + "/apply.py", "--write"],
        "testfix-law": [PY, t + "/apply.py", "--write"],
        "mem-gov": [PY, t + "/apply.py", "--write", "--ignore-runs"],
        "sync-incr": [PY, t + "/apply.py", "apply", "--write", "--root", OV],
        "input-precision": [PY, t + "/apply.py", "--apply"],
        "x-followup": [PY, P + "/x-ingress-tools/followup/apply.py", "--live", "--root", OV],
        "x-graph": [PY, P + "/x-graph-tools/apply.py", "--live", "--root", OV],
        "course-A": [PY, C + "/course-bzl-tools/staged/apply-course.py", "--live", "--root", OV, "--only", "x-dev,x-keys,course-bzl"],
        "course-B": [PY, C + "/course-bzl-tools/staged/apply-course.py", "--live", "--root", OV, "--only", "x-inputs,x-site,x-release,x-paths,x-proctor,reconcile,x-graph", "--with-graph"],
        "course-all": [PY, C + "/course-bzl-tools/staged/apply-course.py", "--live", "--root", OV, "--with-graph"],
        "x-series": [PY, P + "/x-integrate-tools/apply-all.py", "--live", "--root", OV, "--with-cleanup"],
    }[name]


def inventory():
    rows = {}
    for base, _, files in os.walk(OV):
        for file in files:
            path = os.path.join(base, file)
            if os.path.islink(path):
                continue
            rows[os.path.relpath(path, OV)] = hashlib.sha256(open(path, "rb").read()).hexdigest()
    return rows


def seed(extra):
    seeds = json.load(open(GEN + "/seeds.json", encoding="utf8"))
    wanted = set()
    for key, rows in seeds.items():
        if key != "x-series-new":
            wanted.update(rows)
    wanted.update(extra)
    wanted.add("AGENTS.md")
    shutil.rmtree(OV, ignore_errors=True)
    copied = 0
    for rel in sorted(wanted):
        source = os.path.join(LIVE, rel)
        if os.path.isfile(source):
            target = os.path.join(OV, rel)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(source, "rb") as reader, open(target, "wb") as writer:
                writer.write(reader.read())
            shutil.copymode(source, target)
            copied += 1
    return copied


def main():
    order = opt("--order", "orphan-child,own-output,testfix-defect,x-followup,x-series,x-graph,hub-bzl,bzl-rbe,demo-bzl,play-bzl,docs-build").split(",")
    extra = []
    if "--seed-extra" in args:
        extra = [l.strip() for l in open(opt("--seed-extra"), encoding="utf8") if l.strip()]
    os.makedirs(OUT, exist_ok=True)
    os.makedirs("/private/tmp/i2", exist_ok=True)
    print("seeded", seed(extra), "files into", OV)
    result = []
    for name in order:
        if name in DIRS:
            shim(name)
        before = inventory()
        if name == "parity":
            prep = subprocess.run([PY, f"{TOOLS}/parity/build-staged.py"], capture_output=True, text=True, cwd=OV)
            open(f"{OUT}/{name}.prep.log", "w", encoding="utf8").write(prep.stdout + "\n--- stderr ---\n" + prep.stderr)
            if prep.returncode:
                print("FAIL parity prepare:", (prep.stdout + prep.stderr)[-400:])
        run = subprocess.run(commands(name), capture_output=True, text=True, cwd=OV, env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1", "SEMIO_ROOT": OV})
        after = inventory()
        changed = sorted(rel for rel in after if before.get(rel) != after[rel])
        row = {"set": name, "rc": run.returncode, "changed": changed, "new": [r for r in changed if r not in before]}
        open(f"{OUT}/{name}.log", "w", encoding="utf8").write(run.stdout + "\n--- stderr ---\n" + run.stderr)
        result.append(row)
        print(f"{'ok ' if run.returncode == 0 else 'FAIL'} rc={run.returncode} {name:15} changed {len(changed):3} (new {len(row['new'])})" + ("" if run.returncode == 0 else "\n    " + (run.stdout + run.stderr).strip()[-400:].replace("\n", "\n    ")))
        if run.returncode != 0 and "--continue" not in args:
            break
    json.dump({"order": order, "result": result}, open(f"{OUT}/result.json", "w"), ensure_ascii=False, indent=1)
    if "--keep" not in args:
        shutil.rmtree(TOOLS, ignore_errors=True)


main()
