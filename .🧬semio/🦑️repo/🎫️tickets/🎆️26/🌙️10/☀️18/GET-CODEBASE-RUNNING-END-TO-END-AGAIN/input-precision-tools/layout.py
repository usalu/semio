# 🗺️ Staged flat name -> live destination map shared by prep-run.py and apply.py.
import os
ROOT = "/Users/ueli/Documents/semio"
L = os.environ.get("IP_LIBRARY") or ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
P = B + "/📥️inputs/🎯️precision"
NEW = {
    "precision.ts": P + "/🟦️.ts",
    "schema.json": P + "/🧬️schema/🔣️.json",
    "rules.json": P + "/🔣️.json",
    "test.ts": P + "/🧪️tests/🟦️.ts",
    "fixture.json": P + "/🧪️tests/🔣️.json",
    "fixture-schema.json": P + "/🧪️tests/🧬️schema/🔣️.json",
    "oracle.py": P + "/🧪️tests/🐍️oracle/🐍️.py",
}
