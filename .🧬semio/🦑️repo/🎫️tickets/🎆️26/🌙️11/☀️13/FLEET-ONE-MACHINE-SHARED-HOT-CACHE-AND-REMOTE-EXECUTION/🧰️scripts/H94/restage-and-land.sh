#!/bin/bash
# Restage the sealed tool snapshots (lane H94) after the seal and land them through `repo land`.
# Scratch tests: python3 farm.py <scratch>/farm; python3 tree2.py <scratch>/tree2 <scratch>/farm (plain copies, NEVER hard links); sync.sh; bun test <module>/🧪️tests/🟦️.ts
# The stage lives in the scratch folder (long taxonomy paths): S=<scratch>/stage-H94, checkout the two live files, copy the module and the patched files into $S/tree.
set -e
S="${1:?scratch stage folder}"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules"
repo land --lane H94 --title "Sealed Tool Snapshots" checkout "$S" "$B/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts" "$B/🤝️shared-worktree/🔁️replica/🧷️sidecars/🟦️.ts"
# apply sidecar.patch / sidecars.patch to $S/tree/<file> (patch -p0 on a copy), copy 🧰️tool-snapshot into $S/tree/$B/📚️library/🏗️build/🏛️sealed/
repo land check "$S"
repo land --wave "Wave 1127" --rebase "$S"
