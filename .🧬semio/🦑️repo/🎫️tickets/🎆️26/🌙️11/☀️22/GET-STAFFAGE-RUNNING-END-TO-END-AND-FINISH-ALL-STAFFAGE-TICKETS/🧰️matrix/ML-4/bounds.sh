#!/bin/zsh
# 🧊️ The suite 🧊️model-bounds to its end in one queue grant, verbose, with `/usr/bin/time -l`, into 🗑️generated/matrix/ML-4/bounds-<label>.log: bounds.sh <label> <MB> [<name filter>]
me="${0:A}"; source "${0:A:h}/env.sh"; label="$1"; mb="$2"; filter="$3"; mkdir -p "$GEN"; log="$GEN/bounds-$label.log"
if [[ "$4" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en repo capacity run --memory="$mb" -- timeout 7200 "$me" "$label" "$mb" "$filter" --inside > "$log" 2>&1; echo "bounds $label exit $? $(date +%H:%M)" >> "$log"; exit; fi
cd "$P" && /usr/bin/time -l timeout 7000 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🧊️model-bounds/🟦️.ts" ${filter:+-t} ${filter}
