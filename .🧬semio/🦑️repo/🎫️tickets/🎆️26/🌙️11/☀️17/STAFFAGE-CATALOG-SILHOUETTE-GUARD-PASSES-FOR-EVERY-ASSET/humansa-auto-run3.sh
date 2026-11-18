#!/bin/bash
. "$(dirname "$0")/humansa-env.sh"
slug="$1"; sc="$2"; views="$3"; base="$4"; out="$5"
cp "$base" "$out"
OPS="$base" "$B" "$N/humansa-auto.ts" "$slug" "$out.new" "$sc" "$views" > "$out.log" 2>&1
python3 - <<PY
import json
a=json.load(open("$out")); b=json.load(open("$out.new")); json.dump(a+b,open("$out","w"),indent=1)
PY
