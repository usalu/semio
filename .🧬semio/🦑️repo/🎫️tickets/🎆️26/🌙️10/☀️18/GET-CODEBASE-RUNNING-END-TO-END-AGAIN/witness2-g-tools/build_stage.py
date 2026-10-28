#!/usr/bin/env python3
"""📦️ usage: build_stage.py <mirror root> -> flattens the files the witness2-g batch changes (mirror) against their live base into staged/files + staged/base (f<NNN> + manifest.json each); paths longer than 250 UTF-16 units are refused."""
import json, os, shutil, sys
LIVE = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
N = "🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/"
FILES = [N + p for p in ["🟦️.ts", "🧬️schema/🔣️.json", "🧭️client/🟦️.ts", "🗻️mount/🟦️.ts", "🗻️mount/🧬️schema/🔣️.json", "🗻️mount/🧪️tests/🟦️.ts", "🗻️mount/🧪️tests/🔣️.json", "🗻️mount/🧪️tests/🐍️.py", "🪪️vouch/🟦️.ts", "🪪️vouch/🧬️schema/🔣️.json", "🪪️vouch/🧪️tests/🟦️.ts"]] + [L + "🏗️build/" + p for p in [
    "🏡️resident/📜️journal/🟦️.ts", "🏡️resident/📜️journal/🧪️tests/🟦️.ts", "🏡️resident/📜️journal/🪪️vouch/🟦️.ts", "🏡️resident/📜️journal/🪪️vouch/🧪️tests/🟦️.ts",
    "🔧️tools/🏗️bazel/📜️script.ts", "🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts", "🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts",
    "🔭️witness/🟨️.d.mts", "🔭️witness/🟨️.mjs", "🔭️witness/🧪️tests/🟦️.ts", "🔭️witness/🧬️schema/🔣️.json"]]
def units(text): return len(text.encode("utf-16-le")) // 2
def main(mirror):
    stage = HERE + "/staged"
    shutil.rmtree(stage, ignore_errors=True)
    for sub in ("files", "base"): os.makedirs(stage + "/" + sub)
    new, base = {}, {}
    for number, rel in enumerate(FILES, 1):
        name = "f%03d" % number
        if units(rel) > 250: raise SystemExit("path too long: %d %s" % (units(rel), rel))
        shutil.copyfile(mirror + "/" + rel, stage + "/files/" + name); new[rel] = {"file": name}
        if os.path.exists(LIVE + "/" + rel): shutil.copyfile(LIVE + "/" + rel, stage + "/base/" + name); base[rel] = {"file": name}
    for sub, manifest in (("files", new), ("base", base)):
        with open(stage + "/" + sub + "/manifest.json", "w", encoding="utf8") as handle: json.dump(manifest, handle, ensure_ascii=False, indent=1)
    print("staged", len(new), "files,", len(base), "with a base; longest path", max(units(rel) for rel in FILES), "UTF-16 units")
main(sys.argv[1])
