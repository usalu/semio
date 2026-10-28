"""[obs-g] stage.py <build|check|apply|revert> [--mirror M] [--root R]: the staged batch `fleet` of `repo fleet status` and the MCP tool `fleet_status`.
Full files (the module `🚢️fleet-status`, three Bazel status leaves, twelve catalogue/door/law/fixture files) are listed in `staged/fleet/manifest.json`; their bytes are flat in `staged/fleet/files/f<NNN>` because the real paths exceed the path limit inside the ticket. README.md and 📓️build-guide.md of the repo product are shared prose and are changed by the anchored, idempotent scripts apply_readme.py and apply_guide.py instead.
build: copies the files from the workbench (mirror) and records base (sha256 of the real file now, null for a new file) and new sha256. check: one state per file (pending, applied, base-changed). apply: all-or-nothing, refuses on base-changed; originals go to `staged/fleet/backup/`. revert: restores the originals of every file that still equals its new bytes and removes the new ones."""
import hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged", "fleet")
MANIFEST = os.path.join(STAGED, "manifest.json")
PROSE = ["README.md", "📓️build-guide.md"]
PRODUCT = "🧰️framework/🛍️products/🦑️repo/"
LIB = PRODUCT + "🔨️modules/📚️library/"
BUILD = LIB + "🏗️build/"
EXTRA = [
    BUILD + "🔧️tools/🏗️bazel/💾️storage/🟦️.ts",
    BUILD + "🔧️tools/🏗️bazel/🧵️sessions/🔌️port/🟦️.ts",
    BUILD + "🔧️tools/🏗️bazel/🧵️sessions/📖️status/🟦️.ts",
    BUILD + "⚙️operations/🔣️catalog.json",
    BUILD + "⚙️operations/🧬️schema/🔣️.json",
    BUILD + "⚙️operations/🧪️tests/🛗️requires/🔣️.json",
    BUILD + "⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json",
    BUILD + "⚙️operations/🧪️tests/🧫️fixtures/🧭️scoping/🔣️.json",
    BUILD + "🚪️door/🔣️.json",
    BUILD + "🚪️door/🟦️.ts",
    BUILD + "🚪️door/🧪️tests/🚦️startup/🔣️.json",
    BUILD + "🗂️suites/🔣️.json",
    LIB + "⚡️caching/📋️project.json",
    PRODUCT + "🔨️modules/🔌️mcp/🖼️assets/🏗️build-surface.json",
    PRODUCT + "🔨️modules/🔌️mcp/🧪️tests/🧫️fixtures/🏗️build-surface.json",
]


def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.isfile(path) else None


def arg(name, default):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else default


def replace(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".fleet-tmp"
    with open(temporary, "wb") as handle:
        handle.write(data)
    os.replace(temporary, path)


def listing(mirror):
    module = os.path.join(mirror, BUILD, "🚢️fleet-status")
    found = []
    for base, _, names in os.walk(module):
        for name in names:
            found.append(os.path.relpath(os.path.join(base, name), mirror))
    return sorted(found) + EXTRA


def build(mirror, root):
    rows = {}
    for index, rel in enumerate(listing(mirror)):
        data = open(os.path.join(mirror, rel), "rb").read()
        name = "f%03d" % index
        replace(os.path.join(STAGED, "files", name), data)
        rows[rel] = {"file": name, "base": digest(os.path.join(root, rel)), "new": hashlib.sha256(data).hexdigest()}
    json.dump(rows, open(MANIFEST, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("built", len(rows), "files,", sum(1 for row in rows.values() if row["base"] is None), "new")


def states(root, rows):
    out = {}
    for rel, row in rows.items():
        now = digest(os.path.join(root, rel))
        out[rel] = "applied" if now == row["new"] else "pending" if now == row["base"] else "base-changed"
    return out


def main():
    mode, root, mirror = sys.argv[1], arg("--root", "/Users/ueli/Documents/semio"), arg("--mirror", "")
    if mode == "build":
        return build(mirror, root)
    rows = json.load(open(MANIFEST, encoding="utf-8"))
    state = states(root, rows)
    if mode == "check":
        for rel, value in state.items():
            print(value, rel)
        print(len(rows), "files:", {value: list(state.values()).count(value) for value in set(state.values())})
        return
    backup = os.path.join(STAGED, "backup")
    if mode == "apply":
        bad = [rel for rel, value in state.items() if value == "base-changed"]
        if bad:
            sys.exit("refused, changed since staging (re-run build from a fresh workbench, or merge by hand):\n" + "\n".join(bad))
        for rel, row in rows.items():
            if state[rel] == "applied":
                continue
            target = os.path.join(root, rel)
            if row["base"] is not None:
                os.makedirs(os.path.dirname(os.path.join(backup, row["file"])), exist_ok=True)
                shutil.copyfile(target, os.path.join(backup, row["file"]))
            replace(target, open(os.path.join(STAGED, "files", row["file"]), "rb").read())
        for script in ("apply_readme.py", "apply_guide.py"):
            result = subprocess.run([sys.executable, "-I", os.path.join(HERE, script), root], capture_output=True, text=True)
            print(script, (result.stdout + result.stderr).strip())
        return print("applied", len(rows), "files")
    if mode == "revert":
        for rel, row in rows.items():
            target = os.path.join(root, rel)
            if state[rel] != "applied":
                print("skipped", state[rel], rel)
            elif row["base"] is None:
                os.remove(target)
            else:
                replace(target, open(os.path.join(backup, row["file"]), "rb").read())
        return print("reverted; README.md and the guide keep their lines (remove them by hand)")
    sys.exit("usage: stage.py <build|check|apply|revert> [--mirror M] [--root R]")


main()
