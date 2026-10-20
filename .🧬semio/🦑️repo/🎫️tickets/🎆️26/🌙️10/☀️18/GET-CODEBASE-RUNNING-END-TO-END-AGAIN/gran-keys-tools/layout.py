# 🗺️ Staged flat name -> live destination map and shared path constants for the gran-keys set.
import os
ROOT = os.environ.get("GK_ROOT") or "/Users/ueli/Documents/semio"
L = os.environ.get("GK_LIBRARY") or ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
R = B + "/📥️inputs/🦀️resolution"
NEW = {
    "resolution.ts": R + "/🟦️.ts",
    "schema.json": R + "/🧬️schema/🔣️.json",
    "policy.json": R + "/🔣️.json",
    "receipt.mjs": R + "/💾️receipt/🟨️.mjs",
    "receipt.d.mts": R + "/💾️receipt/🟨️.d.mts",
    "test.ts": R + "/🧪️tests/🟦️.ts",
    "fixture.json": R + "/🧪️tests/🔣️.json",
    "fixture-schema.json": R + "/🧪️tests/🧬️schema/🔣️.json",
    "oracle.py": R + "/🧪️tests/🐍️oracle/🐍️.py",
}
