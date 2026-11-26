#!/bin/zsh
# 🧪️ Vitest suites one after the other in one queue grant, verbose, with `/usr/bin/time -l`, into 🗑️generated/matrix/MX-3/vt-<label>-<n>.log: vt.sh <label> <MB> <suite folder>[=<name filter>]…
me="${0:A}"; source "${0:A:h}/env.sh"; mkdir -p "$GEN"
label="$1"; mb="$2"; shift 2
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory="$mb" -- "$me" "$label" "$mb" --inside "$@"; fi
shift; n=0
for item in "$@"; do
  n=$((n + 1)); suite="${item%%=*}"; log="$GEN/vt-$label-$n.log"
  filter=(); [[ "$item" == *=* ]] && filter=(-t "${item#*=}")
  (cd "$P" && /usr/bin/time -l timeout 3000 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" $filter) > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';') peak $(( $(grep 'maximum resident' "$log" | awk '{print $1}') / 1048576 )) MB $(date +%H:%M)"
done
