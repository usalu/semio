#!/usr/bin/env python3
"""🚆️ L1 train driver over `w3-trains.json`.
  list [<train>]      sets in landing order
  dry <id>            the set's dry run on the live tree (capture generated/<train>/<id>-dry.txt)
  apply <id>          [pre] → dry run (stop on rc≠0 or a reported problem) → write inside `l1-land.py write` (byte backup + record)
  revert <id>         `l1-land.py revert <id>` (restores only files still equal to what the write produced)
  crates <id…>        union of Cargo packages / TS packages owning the recorded files of the given sets"""
import glob
import json
import re
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent
T = str(ROOT / ".tmp-ticket")
HUB = str(ROOT / ".🧬semio/🌐hub")
NATIVE = (f"zsh {T}/📜️fleet-mutex.sh native l1 -- env CARGO_INCREMENTAL=0 NX_DAEMON=false "
          f"CARGO_BUILD_BUILD_DIR={ROOT}/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR={T}/wp-l1/target nice -n 5")
PROBLEM = re.compile(r"(?:^|\s)[1-9]\d* (?:problem|conflict)|problems=[1-9]|conflicts=[1-9]|\bPROBLEM\b|\bCONFLICT\b|Traceback", re.I)
LOGS = ROOT / ".🧬semio/🌐hub/s14-l1-logs"
SETS = {entry["id"]: entry for entry in json.loads((HERE / "w3-trains.json").read_text(encoding="utf-8"))["sets"]}


def expand(command):
    maps = " ".join(f"--map '{path}'" for path in sorted(glob.glob(f"{HUB}/s14-t14-logs/s14b-f9-map-*.tsv")))
    command = command.replace("{MAPS}", maps).replace("{NATIVE}", NATIVE)
    return re.sub(r"(?<![\w/.])T/", T + "/", command)


def run(command, capture):
    capture.parent.mkdir(parents=True, exist_ok=True)
    started = time.time()
    argv = ["zsh", "-c", command] if isinstance(command, str) else command
    with capture.open("w", encoding="utf-8") as handle:
        handle.write(f"$ {command}\n")
        handle.flush()
        rc = subprocess.run(argv, cwd=ROOT, stdout=handle, stderr=subprocess.STDOUT).returncode
    text = capture.read_text(encoding="utf-8", errors="replace")
    tail = " | ".join(line.strip()[:160] for line in text.strip().splitlines()[-3:])
    print(f"[l1-train] {capture.name} rc={rc} {time.time() - started:.0f}s :: {tail}", flush=True)
    return rc, text


def dry(entry):
    if not entry["dry"]:
        print(f"[l1-train] {entry['id']}: no dry run (generator)")
        return 0
    rc, text = run(expand(entry["dry"]), LOGS / entry["train"] / f"{entry['id']}-dry.txt")
    problems = [line for line in text.splitlines() if PROBLEM.search(line) and not re.search(r"\b0 (?:problem|conflict)", line)]
    if problems:
        print(f"[l1-train] {entry['id']}: dry run reports problems:\n  " + "\n  ".join(problems[:12]))
    return rc if rc else (9 if problems else 0)


def apply(entry):
    if entry.get("pre"):
        rc, _ = run(expand(entry["pre"]), LOGS / entry["train"] / f"{entry['id']}-pre.txt")
        if rc:
            return rc
    rc = dry(entry)
    if rc:
        print(f"[l1-train] {entry['id']}: SKIPPED (dry run rc={rc})")
        return rc
    scopes = sum((["--scope", scope] for scope in entry.get("scope", [])), [])
    land = ["python3", f"{HERE}/l1-land.py", "write", entry["id"], *scopes, "--", "zsh", "-c", expand(entry["write"])]
    rc, _ = run(land, LOGS / entry["train"] / f"{entry['id']}-write.txt")
    return rc


if __name__ == "__main__":
    verb, ids = sys.argv[1], sys.argv[2:]
    if verb == "list":
        for entry in SETS.values():
            if not ids or entry["train"] in ids:
                print(f"{entry['train']}  {entry['id']:<24} {entry['owner']:<14} {entry['revert']}")
    elif verb == "dry":
        sys.exit(max(dry(SETS[name]) for name in ids))
    elif verb == "apply":
        sys.exit(apply(SETS[ids[0]]))
    elif verb == "revert":
        sys.exit(subprocess.run(["python3", str(HERE / "l1-land.py"), "revert", ids[0], *ids[1:]]).returncode)
    elif verb == "crates":
        for name in ids:
            subprocess.run(["python3", str(HERE / "l1-land.py"), "crates", name])
