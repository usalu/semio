#!/bin/bash
# usage: matrix.sh <name> <task-id>...   generates the closure workspace of the selection and measures it (analysis cold/warm, execution cold/warm/clean/server loss).
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$C/bzl-lazy-tools"; G="$C/🗑️generated/bzl-lazy"; W=/private/tmp/bzlazy
export LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8
name=$1; shift
cd "$ROOT" || exit 2
rm -rf "$W/ws-$name"
BZL_LAZY_WS="$W/ws-$name" BZL_LAZY_SELECTION="run $1" bun "$T/run.ts" "$T/gen.ts" "$@" 2>&1 | command grep '^{"tasks"' > "$G/gen-$name.json" || { echo "gen failed $name"; exit 1; }
label=$(python3 -c "import json;d=json.load(open('$G/gen-$name.json'));print('//:'+list(d['labels'].values())[0])")
python3 "$T/measure.py" "$name" "$W/ws-$name" "$label" "${MODE:-all}" > "$G/m-$name.json"
python3 - <<PY
import json
g=json.load(open("$G/gen-$name.json"));m=json.load(open("$G/m-$name.json"))
print(json.dumps({"name":"$name","tasks":g["tasks"],"cached":g["cached"],"implMs":g["implementationMs"],"genMs":g["generateMs"],"anaCold":m["analysis_cold"]["analysisMs"],"anaWarm":m["analysis_warm"]["analysisMs"],"cold":m["exec_cold"]["summary"] if "exec_cold" in m else None}))
PY
