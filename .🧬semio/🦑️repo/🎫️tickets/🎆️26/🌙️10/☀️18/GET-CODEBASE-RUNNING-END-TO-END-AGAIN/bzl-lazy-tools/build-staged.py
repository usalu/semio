#!/usr/bin/env python3
# Builds the staged copies of the live files from the CURRENT live text with anchored replacements, records the base hashes, and prepares runnable copies (specifiers re-pointed) under run/.
import hashlib, json, os, re, sys
ROOT = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
B = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BZ = B + "/🔧️tools/🏗️bazel"
LIVE = {
    "script.ts": BZ + "/📜️script.ts",
    "schema.json": BZ + "/🧬️schema/🔣️.json",
    "selection.ts": B + "/🧭️selection/🟦️.ts",
    "neutral.ts": B + "/📜️script.ts",
    "catalog.json": B + "/⚙️operations/🔣️catalog.json",
    "actions-fixture.json": BZ + "/🧪️tests/📥️actions/🔣️.json",
    "actions-schema.json": BZ + "/🧪️tests/📥️actions/🧬️schema.json",
    "actions-test.ts": BZ + "/🧪️tests/📥️actions/🟦️.ts",
    "arguments-test.ts": BZ + "/🧪️tests/📤️arguments/🟦️.ts",
    "selection-test.ts": B + "/🧭️selection/🧪️tests/🟦️.ts",
}
sys.path.insert(0, HERE)
import patches  # noqa: E402

def sha(text): return hashlib.sha256(text.encode()).hexdigest()

def main():
    bases, out = {}, {}
    for key, path in LIVE.items():
        text = open(path, encoding="utf-8").read()
        bases[key] = sha(text)
        out[key] = patches.apply(key, text)
    os.makedirs(HERE + "/staged", exist_ok=True)
    for key, text in out.items():
        open(HERE + "/staged/" + key, "w", encoding="utf-8").write(text)
    json.dump(bases, open(HERE + "/staged/bases.json", "w"), indent=1)
    print(json.dumps({k: [bases[k][:12], len(out[k])] for k in out}))

if __name__ == "__main__": main()
