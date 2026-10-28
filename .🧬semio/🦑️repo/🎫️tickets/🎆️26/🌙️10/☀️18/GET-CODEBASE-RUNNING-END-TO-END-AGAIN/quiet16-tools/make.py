"""🧰️ quiet16: builds the flat staged batch `staged/<batch>/` (manifest {rel:{file,base,new}}, files/, base/, stage.py from the xplat5 mode-keeping template, verify.sh) from a mirror root that holds the batch applied. The base of a file is what the gate will see when it lands this batch: the newest of the batches it is stacked on that names the file (since G46 agents16's `allrun`/`farmfree`/`behind` and builderlock `multiwriter` are live, so `hold` sits on live and `lastrun` on `hold`) else the live file, else nothing (a new file).
usage: python3 -I make.py <batch> <mirror root> [--swap base|new]   (--swap writes the base or the new bytes of every batch file into the mirror, to compare base and batch on one mirror)"""
import hashlib, json, os, shutil, stat, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
AGENTS16 = [os.path.join(C, "agents16-tools", "staged", name) for name in ("allrun", "farmfree", "behind")]
MULTIWRITER = [os.path.join(C, "audit9-tools", "staged", "multiwriter")]
STACKS = {"hold": [], "lastrun": [os.path.join(HERE, "staged", "hold")]}
FILES = {
    "hold": [
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🚏️marks/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧬️schema/🔣️.json",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🧷️births/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️control/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🧷️births/🥒️.feature",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🚦️width/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🚦️width/🥒️.feature",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🚦️width/🐍️oracle/🐍️.py",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🚦️width/🧫️fixtures/🔣️.json",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🚪️gate/🟦️.ts",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/👥️agents/🟦️.ts",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/👥️agents/🥒️.feature",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🤫️quiet/🟦️.ts",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🤫️quiet/🥒️.feature",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🔼️upgrade-before-custody/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🤫️quiet/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🤫️quiet/🥒️.feature",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🤫️quiet/🐍️oracle/🐍️.py",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🤫️quiet/🧫️fixtures/🔣️.json",
    ],
    "lastrun": [
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🌦️forecast/🟦️.ts",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪜️phases/🧫️fixtures/🔣️.json",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🧫️fixtures/🔣️.json",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🐍️oracle/🐍️.py",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/📼️recorded.json",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/🔣️.json",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🟦️.ts",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🧫️fixtures/🔣️.json",
        "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🟦️.ts",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🤫️quiet/🧫️fixtures/🔣️.json",
    ],
}


DOCS = {
    "lastrun": "🔁️ Staged batch `lastrun` (lane quiet16, stacked on `hold`): the door forecasts a run by its class's own last outcome: quiet when the newest run of the class executed nothing (however long ago), or when the newest run executed work and ended within the follow-up window; the decayed quiet share and the minimum of quiet runs are gone from the forecast, the policy, the schema, the fixtures and the Python ports, the oracles' fixtures are regenerated. Guide bullets: docs_lastrun.py (anchored, hot file).",
    "hold": "🤫️ Staged batch `hold` (lane quiet16): a quiet forecast holds until work really runs. A nested invocation that inherits a quiet grant runs at once with the plan the holder carried and never grows it; the planner and a graph rebuild ask for the growth from any process of the run (phase marks off included); every Bazel action birth waits for the growth and born actions run only as wide as the grant's jobs; the holder grows a quiet grant whose tree outgrows the quiet reservation without pausing the run; a run whose peak exceeded the quiet size of its class is learned as executed. Guide bullets: docs_hold.py (anchored, hot file).",
}


def sha(data):
    return None if data is None else hashlib.sha256(data).hexdigest()


def read(path):
    return open(path, "rb").read() if os.path.exists(path) else None


def base_of(rel, batch="hold"):
    data = read(os.path.join(LIVE, rel))
    for stage in STACKS[batch]:
        row = json.load(open(os.path.join(stage, "manifest.json"), encoding="utf8")).get(rel)
        if row is not None:
            data = open(os.path.join(stage, "files", row["file"]), "rb").read()
    return data


def write(target, data):
    if data is None:
        if os.path.exists(target): os.remove(target)
        return
    os.makedirs(os.path.dirname(target), exist_ok=True)
    mode = stat.S_IMODE(os.stat(target).st_mode) if os.path.exists(target) else 0o644
    temporary = target + ".quiet16.tmp"
    open(temporary, "wb").write(data); os.chmod(temporary, mode); os.replace(temporary, target)


def main():
    batch, root = sys.argv[1], os.path.abspath(sys.argv[2])
    out = os.path.join(HERE, "staged", batch)
    if "--swap" in sys.argv:
        which = sys.argv[sys.argv.index("--swap") + 1]
        manifest = json.load(open(os.path.join(out, "manifest.json"), encoding="utf8"))
        for rel, row in manifest.items():
            source = os.path.join(out, "base" if which == "base" else "files", row["file"])
            write(os.path.join(root, rel), read(source) if os.path.exists(source) else None)
        print("swapped", which, len(manifest)); return
    for name in ("files", "base"):
        shutil.rmtree(os.path.join(out, name), ignore_errors=True); os.makedirs(os.path.join(out, name))
    manifest = {}
    for index, rel in enumerate(FILES[batch]):
        new, base = read(os.path.join(root, rel)), base_of(rel, batch)
        if new is None: sys.exit("missing in mirror: " + rel)
        file = "%02d%s" % (index, os.path.splitext(rel)[1])
        open(os.path.join(out, "files", file), "wb").write(new)
        if base is not None: open(os.path.join(out, "base", file), "wb").write(base)
        manifest[rel] = {"file": file, "base": sha(base), "new": sha(new)}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read()
    head, rest = template.split('"""', 2)[1], template.split('"""', 2)[2]
    summary = DOCS[batch]
    usage = head[head.index("usage:"):]
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write('#!/usr/bin/env python3\n"""' + summary + "\n" + usage + '"""' + rest)
    print("built", batch, len(manifest), "files")


main()
