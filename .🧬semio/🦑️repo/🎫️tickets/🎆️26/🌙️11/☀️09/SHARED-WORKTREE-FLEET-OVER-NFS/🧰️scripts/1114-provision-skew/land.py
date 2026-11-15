#!/usr/bin/env python3
"""Landing of package P10 (provision skew) under 1113-landing-protocol.md: three-way merge of the files built in the scratch tree onto the live module (git merge-file, base = the live file at the start of the work), new files only when absent. Usage: land.py <scratch-tree> <base-module-copy> [--dry]. Run with the landing lock held."""
import hashlib, os, shutil, subprocess, sys, tempfile

LIVE = "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
MODIFIED = [
    "🛡️resilience/🟦️.ts",
    "🔁️replica/🟦️.ts",
    "🔁️replica/⌨️command/🟦️.ts",
    "🗺️topology/🥾️provision/🐚️.sh",
    "🗺️topology/🥾️provision/🔵️.ps1",
    "🗺️topology/🧪️tests/🟦️.ts",
    "README.md",
]
NEW = [
    "🗺️topology/🥾️provision/🧬️schema/🔣️.json",
    "🗺️topology/🥾️provision/🟦️.ts",
    "🗺️topology/🥾️provision/🧪️tests/🐍️.py",
    "🗺️topology/🥾️provision/🧪️tests/🟦️.ts",
]


def sha(path):
    if not os.path.exists(path):
        return "-"
    return hashlib.sha256(open(path, "rb").read()).hexdigest()[:16]


def put(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    handle, temporary = tempfile.mkstemp(dir=os.path.dirname(path), prefix=".land-")
    with os.fdopen(handle, "wb") as out:
        out.write(data)
    os.chmod(temporary, os.stat(path).st_mode if os.path.exists(path) else 0o644)
    os.replace(temporary, path)


def main():
    scratch, base, dry = sys.argv[1], sys.argv[2], "--dry" in sys.argv
    plan = []
    for name in NEW:
        live, mine = os.path.join(LIVE, name), os.path.join(scratch, name)
        if os.path.exists(live) and open(live, "rb").read() != open(mine, "rb").read():
            sys.exit("new file exists with other content: " + name)
        plan.append((name, "new", None, open(mine, "rb").read()))
    for name in MODIFIED:
        live, old, mine = os.path.join(LIVE, name), os.path.join(base, name), os.path.join(scratch, name)
        current, original, wanted = open(live, "rb").read(), open(old, "rb").read(), open(mine, "rb").read()
        if current == wanted:
            plan.append((name, "same", current, wanted)); continue
        if current == original:
            plan.append((name, "copy", current, wanted)); continue
        with tempfile.TemporaryDirectory() as folder:
            a, b, c = (os.path.join(folder, x) for x in "abc")
            open(a, "wb").write(current); open(b, "wb").write(original); open(c, "wb").write(wanted)
            result = subprocess.run(["git", "merge-file", a, b, c])
            if result.returncode != 0:
                sys.exit("merge conflicts (%d) in %s: resolve by hand" % (result.returncode, name))
            plan.append((name, "merge", current, open(a, "rb").read()))
    for name, kind, before, after in plan:
        live = os.path.join(LIVE, name)
        print("%-5s %s before %s after %s" % (kind, name, sha(live), hashlib.sha256(after).hexdigest()[:16]))
        if not dry and kind != "same":
            put(live, after)


main()
