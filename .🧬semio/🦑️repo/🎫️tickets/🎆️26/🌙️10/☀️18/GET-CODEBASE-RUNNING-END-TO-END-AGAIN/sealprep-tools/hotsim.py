#!/usr/bin/env python3
"""🔥️ [sealprep] Graph-free hot-path simulation on a mirror: one agent runs a heavy graph-free command in sealed mode (the switch set explicitly, so the run does not depend on the flip) (`repo cache status`, door -> executor) again and again while a peer edits two closure files of the build library; per run it records the implementation digest the executor ran (profile mark `source.served`), the digest a live-mode capture would carry (identity.ts, the loader's own identity function), whether the run promoted, and the wall time. The graph memo key and the Bazel plan receipt both contain that digest, so "digest moved" == "memo miss + replan" for the next graph run.
usage: hotsim.py <mirror-root> <tag> [--default: set NO switch at all, the policy default decides (the flip proof)] [--var NAME] [--promote-step: promote before run-1 and between run-4 and run-5]   (writes C/🗑️generated/sealflip/hotsim-<tag>.jsonl, restores the edited files byte-exact at the end)"""
import json, os, subprocess, sys, time, glob
root = os.path.realpath(sys.argv[1]); tag = sys.argv[2]
var = sys.argv[sys.argv.index("--var") + 1] if "--var" in sys.argv else "SEMIO_REPO_IMPLEMENTATION"
here = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(here)
out = os.path.join(C, "🗑️generated", "sealprep", "hotsim-%s.jsonl" % tag)
pb = sorted(glob.glob("/Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun"))[0]
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
door = os.path.join(root, L, "🚪️door/📜️script.ts")
edits = [os.path.join(root, L, "⏱️profile/🟨️.mjs"), os.path.join(root, L, "🏛️sealed/🚪️entry/🟦️.ts")]
saved = {p: open(p, "rb").read() for p in edits}
env0 = {k: v for k, v in os.environ.items() if not k.startswith(("SEMIO_REPO_IMPLEMENTATION", "SEMIO_REPO_SEAL"))}
def live_digest():
    r = subprocess.run([pb, os.path.join(here, "identity.ts"), root], capture_output=True, text=True, timeout=600)
    return r.stdout.strip().splitlines()[-1][:12] if r.returncode == 0 and r.stdout.strip() else "?" + (r.stderr.strip().splitlines() or [""])[-1][:120]
def heavy(label, extra=None):
    env = dict(env0, SEMIO_REPO_PROFILE="1", **({} if "--default" in sys.argv else {var: "sealed"}), **(extra or {}))
    t = time.time(); r = subprocess.run([pb, door, "repo", "cache", "status"], cwd=root, env=env, capture_output=True, text=True, timeout=900)
    wall = round(time.time() - t, 2); text = r.stdout + r.stderr
    served = next((l.split("\t")[-1] for l in text.splitlines() if "\tsource.served\t" in l), "?")
    notes = [l[:160] for l in text.splitlines() if l.startswith("[implementation]") and not l.startswith("[implementation] Versiegelter") and "Arbeitsbaum" not in l]
    if r.returncode: notes = notes[:2] + [l[:200] for l in text.splitlines() if l.strip() and not l.startswith(("[script]", "#", "[implementation] Promoting", "[implementation] Der Arbeitsbaum wird zu"))][:2]
    return {"step": label, "exit": r.returncode, "wall": wall, "served": served.replace("sealed:", "")[:12], "live": live_digest(), "promoted": any("Promoted the sealed" in l for l in notes), "notes": notes[:3]}
def edit(i):
    with open(edits[i], "ab") as h: h.write(b"\n")
    return {"step": "peer-edit-%d" % (i + 1), "file": os.path.relpath(edits[i], root)}
def promote():
    t = time.time(); r = subprocess.run([pb, door, "repo"], cwd=root, env=dict(env0, **{var: "promote"}), capture_output=True, text=True, timeout=900)
    if r.returncode: print("promote failed: " + " | ".join(l[:200] for l in (r.stdout + r.stderr).splitlines() if l.startswith("[implementation]") and "Promoting" not in l and "wird zu" not in l)[:400], file=sys.stderr)
    return {"step": "explicit-promote", "exit": r.returncode, "wall": round(time.time() - t, 2), "notes": [l[:160] for l in (r.stdout + r.stderr).splitlines() if l.startswith("[implementation]")][:2]}
plan = [lambda: heavy("run-1"), lambda: edit(0), lambda: heavy("run-2"), lambda: heavy("run-3"), lambda: edit(1), lambda: heavy("run-4")]
if "--promote-step" in sys.argv: plan = [promote] + plan + [promote, lambda: heavy("run-5")]
try:
    with open(out, "w") as sink:
        for step in plan:
            row = step(); sink.write(json.dumps(row, ensure_ascii=False) + "\n"); sink.flush(); print(json.dumps(row, ensure_ascii=False)[:300], flush=True)
finally:
    for p, b in saved.items():
        with open(p, "wb") as h: h.write(b)
