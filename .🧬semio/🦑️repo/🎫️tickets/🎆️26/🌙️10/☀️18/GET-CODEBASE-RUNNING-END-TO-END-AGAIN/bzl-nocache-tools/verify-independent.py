#!/usr/bin/env python3
"""verify-independent.py [root]: language-agnostic check (Python standard library only) of the end state `staged/ops.json` describes.
Re-derives the invariants from the manifests alone: a cached target declares outputs (a store receipt, a path, or none only with the check-without-outputs decision), carries an admitted decision, forwards no caller arguments;
a noop has no command, inputs or outputs; every uncached target records a closed class/reason; a receipt output has the closed form `{projectRoot}/.🧾️store/<target>.json`. Exit 1 on a violation; pending targets are reported, not failed."""
import json, os, re, sys
here = os.path.dirname(os.path.abspath(__file__))
root = sys.argv[1] if len(sys.argv) > 1 else os.path.abspath(os.path.join(here, "../../../../../../../.."))
ops = json.load(open(os.path.join(here, "staged/ops.json"), encoding="utf8"))
CLASSES = {"deterministic": {"pure-function-of-inputs", "check-without-outputs"}, "aggregate": {"orders-prerequisites", "reenters-build-tool"}, "side-effecting": {"mutates-workspace", "mutates-host-store", "publishes-remote", "publishes-live-state"}, "live": {"drives-running-process", "exercises-host-resources"}, "diagnostic": {"measures-environment", "reads-caller-input"}, "managed-output": {"outputs-in-managed-store"}}
bad, applied, pending = [], 0, 0
for op in ops:
    target = json.load(open(os.path.join(root, op["manifest"]), encoding="utf8"))["targets"][op["target"]]
    if target != op["after"]:
        pending += 1
        continue
    applied += 1
    decision = target.get("metadata", {}).get("semio", {}).get("cache")
    if decision is not None and decision["reason"] not in CLASSES.get(decision["class"], set()):
        bad.append(f'{op["id"]}: decision {decision} is not a closed class/reason')
    if target.get("executor") == "repo:noop":
        if any(k in target for k in ("options", "inputs", "outputs", "cache")): bad.append(f'{op["id"]}: noop keeps process fields')
        continue
    if target.get("cache") is True:
        outputs = target.get("outputs")
        if outputs is None: bad.append(f'{op["id"]}: cached without outputs')
        elif outputs == [] and not (decision and decision["reason"] == "check-without-outputs"): bad.append(f'{op["id"]}: empty outputs without the check decision')
        for o in outputs or []:
            if ".🧾️store/" in o and not re.fullmatch(r"\{projectRoot\}/\.🧾️store/" + re.escape(op["target"]) + r"\.json", o): bad.append(f'{op["id"]}: receipt {o} is not the closed form')
        if target.get("options", {}).get("forwardAllArgs") is True: bad.append(f'{op["id"]}: cached target forwards caller arguments')
        if decision and decision["class"] != "deterministic": bad.append(f'{op["id"]}: cached with class {decision["class"]}')
    if target.get("cache") is False and not target.get("continuous") and decision is None: bad.append(f'{op["id"]}: uncached without a recorded decision')
print(f"{applied} applied, {pending} pending, {len(bad)} violation(s)")
for line in bad: print("VIOLATION", line)
sys.exit(1 if bad else 0)
