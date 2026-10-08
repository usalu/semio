#!/usr/bin/env python3
import json
import os
import re
import sys

ROOT = "/Users/ueli/Documents/semio"
T = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT")
GEN = os.path.join(T, "🗑️generated/launch-dependents")
LIST = os.path.join(GEN, "all-files.txt")

EXCLUDE_COMPONENTS = {"node_modules", ".git", "temp", "target", "dist"}
EXCLUDE_PREFIXES = (".cursor/plans",)
MAX_BYTES = 12 * 1024 * 1024

PATTERNS = {
    "launch.json": r"launch\.json",
    "launch.seed": r"launch\.seed",
    "claude-launch": r"\.claude/launch",
    "launchSeed": r"launchSeed|launch_seed|LAUNCH_SEED|LaunchSeed",
    "LAUNCH_OUTPUT_REL_PATH": r"LAUNCH_OUTPUT_REL_PATH|SEED_REL_PATH|LAUNCH_FILE|LAUNCH_OUTPUT",
    "mod:🚀️launch": r"🚀️launch|🚀launch|🚀️/launch",
    "devLaunchers": r"devLaunchers|projectLaunchers|dev_launchers|project_launchers|DevLaunchers|ProjectLaunchers",
    "serverReadyAction": r"serverReadyAction",
    "node-terminal": r"node-terminal",
    "presentation": r"\"presentation\"|presentation\.group|presentation\.order|presentation\.hidden",
    "group-id": r"\b\d_(?:gate|dev|build|test|run|serve|task|verify|publish|check)\b",
    "launch-wording": r"launch (?:row|entry|entries|rows|configuration|configurations|config|configs|compound|compounds)|launch-(?:row|entry|configuration|config)|launch_(?:row|entry|configuration|config)|launchConfiguration|LaunchConfiguration|launchEntry|launchRow|launch\s+and\s+debug",
    "preview_start": r"preview_start|preview-start|previewStart",
    "run-and-debug": r"Run and Debug|Run & Debug|Run And Debug|run-and-debug|runAndDebug|debug panel|Debug panel|F5\b",
    "inputs-var": r"\$\{input:",
    "compounds": r"\"compounds\"|compounds:",
    "launch-fixtures": r"launch-configurations|🚀️launch-configurations",
    "launch-generic": r"\blaunch\.|\blaunch/|/launch\b",
}

import os as _os
if _os.environ.get("PATTERNS_JSON"):
    PATTERNS = json.load(open(_os.environ["PATTERNS_JSON"], encoding="utf-8"))
OUTNAME = _os.environ.get("OUTNAME", "scan.json")
COMPILED = {k: re.compile(v) for k, v in PATTERNS.items()}


def excluded(path: str) -> bool:
    if path.startswith(EXCLUDE_PREFIXES):
        return True
    comps = path.split("/")
    if len(comps) > 3 and comps[0].startswith(".") and "semio" in comps[0] and "🎫" in comps[2]:
        return True
    return any(c in EXCLUDE_COMPONENTS for c in comps)


def main() -> int:
    only = set(sys.argv[1:])
    results = {}
    skipped = []
    with open(LIST, encoding="utf-8", errors="surrogateescape") as fh:
        files = [line.rstrip("\n") for line in fh if line.strip()]
    for rel in files:
        if excluded(rel):
            continue
        full = os.path.join(ROOT, rel)
        try:
            st = os.stat(full)
        except OSError:
            continue
        if not os.path.isfile(full):
            continue
        if st.st_size > MAX_BYTES:
            skipped.append((rel, st.st_size))
            continue
        try:
            with open(full, "rb") as bf:
                data = bf.read()
        except OSError:
            continue
        if b"\0" in data[:8192]:
            continue
        text = data.decode("utf-8", errors="replace")
        hits = {}
        lines = text.split("\n")
        for name, rx in COMPILED.items():
            if only and name not in only:
                continue
            found = []
            for idx, line in enumerate(lines, 1):
                if len(line) > 4000:
                    scan = line[:4000]
                else:
                    scan = line
                if rx.search(scan):
                    found.append(idx)
            if found:
                hits[name] = found
        if hits:
            results[rel] = hits
    with open(os.path.join(GEN, OUTNAME), "w", encoding="utf-8") as out:
        json.dump({"results": results, "skipped": skipped}, out, ensure_ascii=False, indent=1)
    print(len(results), "files with hits;", len(skipped), "skipped")
    return 0


if __name__ == "__main__":
    sys.exit(main())
