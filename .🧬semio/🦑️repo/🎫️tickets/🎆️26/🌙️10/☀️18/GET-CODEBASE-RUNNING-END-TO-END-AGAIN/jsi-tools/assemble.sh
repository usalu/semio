#!/bin/zsh
# usage: assemble.sh <stage-dir>  (splits the original monolith into <stage-dir> and adds the hand-written root and revision-graph kernel)
set -e
T="$(cd "$(dirname "$0")" && pwd)"
STAGE="$1"
node "$T/split.cjs" "$T/spec.cjs" "$STAGE" | tail -3
cp "$T/manual/root.mjs" "$STAGE/🟨️.mjs"
mkdir -p "$STAGE/🪢️revision-graph"
cp "$T/manual/revision-graph.mjs" "$STAGE/🪢️revision-graph/🟨️.mjs"
cp "$T/manual/revision-graph.d.mts" "$STAGE/🪢️revision-graph/🟨️.d.mts"
mkdir -p "$STAGE/🗂️components/🧬️schema"
cp "$T/manual/components.schema.json" "$STAGE/🗂️components/🧬️schema/🔣️.json"
