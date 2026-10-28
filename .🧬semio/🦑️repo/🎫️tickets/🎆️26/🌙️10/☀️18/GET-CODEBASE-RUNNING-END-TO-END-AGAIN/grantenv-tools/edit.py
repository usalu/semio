#!/usr/bin/env python3
"""🎫️ grantenv-tools: the host-capacity grant and plan variables are authority (an admission token handed to nested work, never a planner input), so the world witness records no read of them. Usage: python3 -I edit.py <root> apply|revert (revert restores base/schema.json)."""
import os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
S = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🧬️schema/🔣️.json"
PAIRS = [
 ("\"authorityEnvironment\": [\"SEMIO_SCRIPT_PROCESS_INVOCATION\"],", "\"authorityEnvironment\": [\"SEMIO_SCRIPT_PROCESS_INVOCATION\", \"SEMIO_HOST_CAPACITY_GRANT\", \"SEMIO_HOST_CAPACITY_PLAN\"],"),
 ("\"SEMIO_BUILD_REUSE_GRAPH\", \"SEMIO_HOST_CAPACITY_GRANT\", \"SEMIO_HOST_CAPACITY_PLAN\"]", "\"SEMIO_BUILD_REUSE_GRAPH\"]"),
]
root, mode = sys.argv[1], sys.argv[2]
path = os.path.join(root, S)
if mode == "revert":
    shutil.copy2(os.path.join(HERE, "base", "schema.json"), path); print("restored"); sys.exit(0)
text = open(path, encoding="utf8").read()
for old, new in PAIRS:
    if text.count(old) != 1: sys.exit("DRIFT " + old[:60])
    text = text.replace(old, new)
open(path, "w", encoding="utf8").write(text); print("applied")
