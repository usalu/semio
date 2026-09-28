#!/usr/bin/env python3
"""🧪️ R10 T0: proves the directories that exist only in prepared window-3 sets against the taxonomy BEFORE those sets land.

Builds a simulation root (a fresh git repository outside the checkout) holding the tracked + untracked-not-ignored files of
the subtrees the sets touch plus the taxonomy, lays each set's new files there, then runs `taxonomy-kinds.ts --root` over the
touched scopes and the SAME scopes on the live tree; only the rows the sets introduce are kept (an unresolved LIVE ancestor
of a planned directory is only reported: registering an existing subtree is its owner's decision, not this pass). The `--json`
rows are the shape `taxonomy-register.py` consumes (paths relative to the root, so they register in the live taxonomy as is).
Sets:
  --set s18|sh2|z4     S18 named-layout twin (its own script, `S18_PATCH_ROOT`), SH2 route B (payload manifest), Z4 devcontainer
                       lifecycle (copy list + retirements)
  --tree <dir>         a payload tree whose relative paths are repo paths (C12 splice `patch/tree`, AV2 `payload/files`)
  --placeholder <path> a new repo file a set's script writes itself (only its directory matters for the kinds)
The live tree is only read.
usage: python3 taxonomy-planned.py <simulation root> --json <out.json> [--taxonomy <candidate.json>] (--set …|--tree …|--placeholder …)…
"""
import json
import os
import shutil
import subprocess
import sys

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
TICKET = os.path.dirname(HERE)
TAXONOMY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
CONFIG = "🧰️framework/🛍️products/💻️os/🎚️config"
ELEMENTS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
HOST_TESTS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests"
HOME = "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home"
CONTAINERS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers"
BUILTIN = {
    "s18": ([CONFIG, f"{ELEMENTS}/🐚️Shell"], [CONFIG]),
    "sh2": ([CONFIG, f"{ELEMENTS}/🏛️ShellHost", HOST_TESTS, HOME], [CONFIG, f"{ELEMENTS}/🏛️ShellHost", HOST_TESTS, HOME]),
    "z4": ([CONTAINERS], [CONTAINERS]),
}


def options(args, name):
    return [args[index + 1] for index, arg in enumerate(args) if arg == name]


def nearest_existing(path):
    while path and not os.path.isdir(os.path.join(REPO, path)):
        path = os.path.dirname(path)
    return path


def outermost(paths):
    return sorted({path for path in paths if not any(path != other and path.startswith(f"{other}/") for other in paths)})


def apply_builtin(name, sim):
    if name == "s18":
        s18 = subprocess.run(["python3", f"{TICKET}/wp-s18/s18-named-layout-rust-twin.py"], cwd=sim, env={**os.environ, "S18_PATCH_ROOT": sim}, capture_output=True, text=True)
        print(f"[planned] S18 twin rc={s18.returncode} {s18.stdout.strip().splitlines()[-1:] if s18.stdout.strip() else ''} {s18.stderr.strip()[-400:]}")
    elif name == "sh2":
        subtrees = BUILTIN["sh2"][0]
        rows = [row for row in json.load(open(f"{TICKET}/wp-sh2/b1/payload/manifest.json", encoding="utf-8")) if any(row["path"].startswith(f"{subtree}/") for subtree in subtrees)]
        for row in rows:
            os.makedirs(os.path.dirname(os.path.join(sim, row["path"])), exist_ok=True)
            shutil.copyfile(f"{TICKET}/wp-sh2/b1/payload/{row['id']}.new", os.path.join(sim, row["path"]))
        print(f"[planned] SH2 route B: {len(rows)} files")
    elif name == "z4":
        z4 = os.path.join(TICKET, "wp-z4/devcontainer-lifecycle")
        for source, target in ((f"{z4}/module/🟦️.ts", f"{CONTAINERS}/🔁️lifecycle/🟦️.ts"), (f"{z4}/law/🟦️.ts", f"{CONTAINERS}/🧪️tests/🔁️lifecycle/🟦️.ts"), (f"{z4}/fixture/🔣️.json", f"{CONTAINERS}/🧫️fixtures/🔁️lifecycle/🔣️.json")):
            os.makedirs(os.path.dirname(os.path.join(sim, target)), exist_ok=True)
            shutil.copyfile(source, os.path.join(sim, target))
        for retired in (f"{CONTAINERS}/🧪️tests/🧩️extension-attach", f"{CONTAINERS}/🧪️tests/🔒️persistent-state", f"{CONTAINERS}/🧫️fixtures/🧩️extension-attach", f"{CONTAINERS}/🧫️fixtures/🔒️persistent-state"):
            shutil.rmtree(os.path.join(sim, retired), ignore_errors=True)
        print("[planned] Z4 lifecycle: 3 files")


def kinds(root, scopes, out):
    run = subprocess.run(["bun", f"{HERE}/taxonomy-kinds.ts", "--root", root, "--json", out, *scopes], cwd=HERE, capture_output=True, text=True)
    print("\n".join(line for line in run.stdout.splitlines() if line.startswith("[taxonomy-kinds]")), run.stderr[-2000:])
    if run.returncode != 0:
        sys.exit(run.returncode)
    return json.load(open(out, encoding="utf-8"))


def main():
    args = sys.argv[1:]
    sim = os.path.abspath(args[0])
    out = args[args.index("--json") + 1]
    candidate = args[args.index("--taxonomy") + 1] if "--taxonomy" in args else None
    sets, trees, placeholders = options(args, "--set"), options(args, "--tree"), options(args, "--placeholder")
    tree_files = [(os.path.join(tree, relative), relative) for tree in trees for relative in (os.path.relpath(os.path.join(base, name), tree) for base, _, names in os.walk(tree) for name in names)]
    planned_files = [relative for _, relative in tree_files] + placeholders
    subtrees = [path for name in sets for path in BUILTIN[name][0]] + [nearest_existing(os.path.dirname(path)) for path in planned_files]
    scopes = outermost([path for name in sets for path in BUILTIN[name][1]] + [nearest_existing(os.path.dirname(path)) for path in planned_files])
    if os.path.exists(sim):
        shutil.rmtree(sim)
    os.makedirs(sim)
    listed = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", *outermost(subtrees)], cwd=REPO, capture_output=True, check=True).stdout.decode("utf-8").split("\0")
    copied = 0
    for relative in [path for path in listed if path] + [TAXONOMY]:
        source = os.path.join(REPO, relative)
        if os.path.isfile(source):
            os.makedirs(os.path.dirname(os.path.join(sim, relative)), exist_ok=True)
            shutil.copy2(source, os.path.join(sim, relative))
            copied += 1
    if candidate:
        shutil.copy2(candidate, os.path.join(sim, TAXONOMY))
    print(f"[planned] simulation root {sim}: {copied} files; scopes {len(scopes)}")
    for name in sets:
        apply_builtin(name, sim)
    for source, relative in tree_files:
        os.makedirs(os.path.dirname(os.path.join(sim, relative)), exist_ok=True)
        shutil.copyfile(source, os.path.join(sim, relative))
    for relative in placeholders:
        os.makedirs(os.path.dirname(os.path.join(sim, relative)), exist_ok=True)
        open(os.path.join(sim, relative), "w", encoding="utf-8").close()
    print(f"[planned] trees: {len(tree_files)} files; placeholders: {len(placeholders)}")
    subprocess.run(["git", "init", "-q"], cwd=sim, check=True)
    rows = kinds(sim, scopes, out)
    live = {row["path"] for row in kinds(REPO, scopes, out.replace(".json", ".live.json"))}
    planned = [row["path"] for row in rows if row["path"] not in live]
    kept = [row for row in rows if row["path"] not in live]
    for row in rows:
        if row["path"] in live and any(path.startswith(f"{row['path']}/") for path in planned):
            print(f"[planned] live ancestor unresolved (not registered here): {row['path']}")
    json.dump(kept, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for row in kept:
        print(f"[planned] {row['path']} parent={row['parentKind']}")


main()
