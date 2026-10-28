"""🔬 Lists the planner outputs of a Bazel session slot whose identity differs from its plan receipt: `outputs_drift.py <slot-dir> [<root>]`."""
import json, os, sys, glob

ROOT = sys.argv[2] if len(sys.argv) > 2 else os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *([".."] * 8)))
slot = sys.argv[1]
schema = json.load(open(glob.glob(os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧬️schema/🔣️.json"))[0]))
policy = schema["definitions"]["Policy"]["const"]
receipt = json.load(open(os.path.join(slot, "plan.receipt.json")))
rows = receipt["outputs"]
for row in rows:
    name = row["name"]
    try:
        st = os.lstat(os.path.join(slot, name))
    except OSError:
        print("MISSING", name)
        continue
    now = ":".join(map(str, [st.st_dev, st.st_ino, st.st_mode, st.st_size, st.st_mtime_ns, st.st_ctime_ns]))
    if now != row["identity"]:
        old = row["identity"].split(":")
        new = now.split(":")
        parts = [n for n, a, b in zip(["dev", "ino", "mode", "size", "mtime", "ctime"], old, new) if a != b]
        print("DRIFT", name, parts)
print("checked", len(rows), "recordedAtMs", receipt["recordedAtMs"])
