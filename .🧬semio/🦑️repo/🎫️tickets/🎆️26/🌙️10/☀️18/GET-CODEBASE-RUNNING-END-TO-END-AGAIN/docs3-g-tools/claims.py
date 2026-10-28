"""📓️ docs3-g claim check: every SEMIO_ name and every named symbol, key or message fragment that a staged doc batch mentions must exist in the live tree or in a staged code batch.

usage: python3 -I claims.py <extra staged search dir>...   (streams `rg -o | sort -u`, never holds a tree in memory)
Prints one row per needle: where it was found (live, staged) or MISSING; exit 1 when anything is missing."""
import json
import os
import re
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ROOT = C
while not os.path.isfile(os.path.join(ROOT, "AGENTS.md")):
    ROOT = os.path.dirname(ROOT)
index = json.load(open(os.path.join(HERE, "staged", "index.json"), encoding="utf8"))
needles = set()
per_batch = {}
for row in index["batches"]:
    spec = json.load(open(os.path.join(HERE, "staged", row["name"], "edits.json"), encoding="utf8"))
    for edit in spec["edits"]:
        text = edit.get("block") or edit.get("new") or ""
        found = set(re.findall(r"SEMIO_[A-Z0-9_]+", text))
        per_batch.setdefault(row["name"], set()).update(found)
        needles.update(found)
EXTRA = [
    "fleet_status", "RelocateScratch", "renameOverBusyFileSyncV1", "freeSpaceSyncV1", "hostProbes", "keepNewest", "stageExecutableDigestV1", "jobserver-empty", "pollBackoffBasisPoints",
    "dependencies.vouch", "root-gone", "directory-gone", "tracked-unknown", "live-writer", "symlink-root", "not-directory", "not-closed", "platformCauses", "quietFollowUpMilliseconds",
    "node-native-core", "compiler-helpers", "pressureUnmeasured", "dedupe.freeUnknown", "log:unflushed", "max-profiles", "daemon-startup", "semio-repo-watch", "xcode-select",
    "GetDiskFreeSpaceExW", "ucrtbase", "libSystem", "UnauthorizedAccessException", "mustRun", "Refusing to wait", "Lock order violated", "cannot vouch", "cannot vouch for every change",
    "reads the operand as a filter", "was stopped at its ceiling", "cannot be reused later", "could not be prepared", "cannot be reached by this process", "SKIPPED, not passed",
    "tests touched the real repository root", "forecast quiet because the earlier run", "purge-artifacts", "envelopes", "quietAvoidsSoftStops", "stopWaitMilliseconds",
    "maximumResidentBytes", "decodedRowsRetainMilliseconds", "hold-installed", "pressure-warn", "untrusted", "overflow", "tag.py", "Deadlock detected", "Self-deadlock", "LeaseOrderErrorV1",
    "semio-build-scheduler", "is gone: ending process", "left {count} process", "Preparing {step}", "cannot be built on",
]
for word in EXTRA:
    needles.add(word.replace(" ", " "))
with tempfile.NamedTemporaryFile("w", delete=False, suffix=".txt", encoding="utf8") as handle:
    handle.write("\n".join(sorted(needles, key=lambda word: (-len(word), word))) + "\n")
    needle_file = handle.name


def search(paths, extra):
    command = ["rg", "-F", "-o", "-N", "--no-filename", "--no-messages", "-g", "!*.log", "-g", "!*.jsonl", "-g", "!node_modules", "-g", "!🗑️generated", "-f", needle_file] + extra + paths
    process = subprocess.Popen(command + [], stdout=subprocess.PIPE, text=True, encoding="utf8")
    seen = set()
    for line in process.stdout:
        seen.add(line.rstrip("\n"))
        if len(seen) > 100000:
            break
    process.wait()
    return seen


live = search([os.path.join(ROOT, "🧰️framework"), os.path.join(ROOT, ".github"), os.path.join(ROOT, ".devcontainer")], [])
staged_dirs = [os.path.join(C, name) for name in sorted(os.listdir(C)) if name.endswith("-g-tools") and name != "docs3-g-tools"] + sys.argv[1:]
staged = search(staged_dirs, ["--no-ignore", "--hidden"])
missing = 0
for needle in sorted(needles):
    where = ("live" if needle in live else "") + (" staged" if needle in staged else "")
    if not where:
        missing += 1
    print("%-52s %s" % (needle, where.strip() or "MISSING"))
os.unlink(needle_file)
print("needles %d, missing %d" % (len(needles), missing))
sys.exit(1 if missing else 0)
