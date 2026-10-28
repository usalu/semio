"""🏗️ quietfc-g staged3: builds this batch from a mirror root (`python3 -I make.py <mirror-root>`): every file that differs from the live tree among the batch paths becomes `files/fNNN` with the live file as `base/fNNN` and `manifest.json` {rel:{file,base,new}}."""
import hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
HC = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
PATHS = [
    HC + "/📏️decision/🟦️.ts", HC + "/🔼️upgrade/🟦️.ts", HC + "/🧬️schema/🔣️.json",
    HC + "/🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py", HC + "/🧪️tests/🔼️upgrade/🟦️.ts", HC + "/🧪️tests/🔼️upgrade/🧫️fixtures/🔣️.json", HC + "/🧪️tests/🌤️followup/🧫️fixtures/🔣️.json",
    LIB + "/🌡️host-capacity/🟦️.ts", LIB + "/🌡️host-capacity/🧪️tests/🌤️followup/🟦️.ts", LIB + "/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/📼️g34.json",
    LIB + "/🔧️tools/🏗️bazel/📜️script.ts", "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md",
]


def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.isfile(path) else None


rows = {}
for index, rel in enumerate(PATHS):
    source, live = os.path.join(sys.argv[1], rel), os.path.join(REPO, rel)
    name = "f%03d" % index
    shutil.copyfile(source, os.path.join(HERE, "files", name))
    if os.path.isfile(live):
        shutil.copyfile(live, os.path.join(HERE, "base", name))
    rows[rel] = {"file": name, "base": digest(live), "new": digest(source)}
json.dump(rows, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("staged", len(rows), "files")
