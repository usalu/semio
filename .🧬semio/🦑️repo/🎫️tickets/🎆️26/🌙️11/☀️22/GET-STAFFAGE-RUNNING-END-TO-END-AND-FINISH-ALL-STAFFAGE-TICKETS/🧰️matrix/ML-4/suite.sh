#!/bin/zsh
# 🧪️ Whole suites to their end in one queue grant, one after the other, verbose, each with `/usr/bin/time -l`, into 🗑️generated/matrix/ML-4/suite-<label>-<n>.log: suite.sh <label> <MB> <suite folder>…
me="${0:A}"; source "${0:A:h}/env.sh"; label="$1"; mb="$2"; shift 2; mkdir -p "$GEN"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory="$mb" -- timeout 14000 "$me" "$label" "$mb" --inside "$@"; fi
shift; n=0
for suite in "$@"; do
  n=$((n + 1)); log="$GEN/suite-$label-$n.log"
  (cd "$P" && /usr/bin/time -l timeout 6000 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts") > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';') peak $(( $(grep 'maximum resident' "$log" | awk '{print $1}') / 1048576 )) MB $(date +%H:%M)"
done
