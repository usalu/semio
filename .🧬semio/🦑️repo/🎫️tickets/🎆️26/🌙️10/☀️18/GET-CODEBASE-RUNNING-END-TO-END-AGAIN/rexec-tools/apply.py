#!/usr/bin/env python3
"""Applies the staged hermetic-execution set to the live tree. New files are content-hash checked (identical = already applied, different = drift = refuse); the adapter and the test index are patched by anchored edits against the CURRENT live text, so concurrent unrelated edits survive. Usage: apply.py [--dry-run|--apply] [--force-while-running]."""
import hashlib, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from adapter_edits import patch

ROOT = os.environ.get("REXEC_ROOT", "/Users/ueli/Documents/semio")
HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BZ = f"{L}/🏗️build/🔧️tools/🏗️bazel"
NEW = [
    f"{L}/🏗️build/🏃️execution/🧬️schema/🔒️hermetic.json",
    f"{BZ}/🏃️execution/🟦️.ts",
    f"{BZ}/🏃️execution/🏭️runner/🟦️.ts",
    f"{BZ}/🏃️execution/🧬️schema/🔣️.json",
    f"{BZ}/🏃️execution/🧪️tests/🟦️.ts",
    f"{BZ}/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json",
    f"{BZ}/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts",
]
INDEX = f"{BZ}/🧪️tests/🟦️.ts"
INDEX_OLD = 'import "../💾️storage/🧪️tests/🟦️.ts";\n'
INDEX_NEW = INDEX_OLD + 'import "../🏃️execution/🧪️tests/🟦️.ts";\nimport "../🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts";\n'
ADAPTER = f"{BZ}/📜️script.ts"

def sha(data): return hashlib.sha256(data if isinstance(data, bytes) else data.encode()).hexdigest()
def read(path):
    with open(path, "rb") as handle: return handle.read()

def plan():
    rows, refusals = [], []
    for rel in NEW:
        staged, live = os.path.join(STAGED, rel), os.path.join(ROOT, rel)
        if not os.path.exists(staged):
            refusals.append(f"staged file missing: {rel}"); continue
        want = read(staged)
        if not os.path.exists(live): rows.append(("create", rel, want, sha(want)))
        elif read(live) == want: rows.append(("same", rel, want, sha(want)))
        else: refusals.append(f"drift: live differs from staged: {rel} (live {sha(read(live))[:12]} staged {sha(want)[:12]})")
    text = read(os.path.join(ROOT, ADAPTER)).decode()
    try: after = patch(text)
    except SystemExit as error: refusals.append(f"adapter anchor refused: {error}"); after = text
    rows.append(("same" if after == text else "patch", ADAPTER, after.encode(), sha(after)))
    index = read(os.path.join(ROOT, INDEX)).decode()
    if INDEX_NEW in index: rows.append(("same", INDEX, index.encode(), sha(index)))
    elif index.count(INDEX_OLD) != 1: refusals.append("test index anchor missing or repeated")
    else:
        updated = index.replace(INDEX_OLD, INDEX_NEW); rows.append(("patch", INDEX, updated.encode(), sha(updated)))
    return rows, refusals

def running():
    out = subprocess.run(["ps", "-eo", "pid,command"], capture_output=True, text=True).stdout
    return [line.strip()[:160] for line in out.splitlines() if ("dependencies sync" in line or "show projects" in line) and "apply.py" not in line and "grep" not in line]

args = sys.argv[1:]
rows, refusals = plan()
for action, rel, _data, digest in rows: print(f"{action:7} {digest[:12]} {rel}")
for message in refusals: print("REFUSED", message)
busy = running()
if busy and "--force-while-running" not in args: refusals.append("a dependency sync or show projects is running (any write aborts it): " + busy[0])
if refusals:
    for message in refusals[len(refusals) - 1:]: print("REFUSED", message)
    sys.exit(1)
if "--apply" not in args:
    print("dry run: nothing written"); sys.exit(0)
for action, rel, data, _digest in rows:
    if action == "same": continue
    path = os.path.join(ROOT, rel); os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".rexec.tmp"
    with open(temporary, "wb") as handle: handle.write(data)
    os.replace(temporary, path)
    print("written", rel)
