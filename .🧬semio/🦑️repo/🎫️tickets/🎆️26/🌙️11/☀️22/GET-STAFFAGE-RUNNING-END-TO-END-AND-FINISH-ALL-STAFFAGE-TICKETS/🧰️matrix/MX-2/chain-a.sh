#!/bin/zsh
# 🔗️ The type check of the staffage package and vitest suites in ONE queue grant of MEM (default 2500) MB, each with its peak memory: chain-a.sh <tag> [tsc] [tsc-react] <suite>[=<name filter>] … writes 🗑️generated/matrix/MX-2/vt/<tag>-tsc.log and <tag>-<n>.log and prints one line per step.
self="${0:A}"; export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN/vt"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=${MEM:-2500} -- "$self" --inside "$@"; fi
shift; tag="$1"; shift
cd "$S/📦️packages/🟦️typescript" || exit 2
peak() { grep -E 'maximum resident set size' "$1" | awk '{printf "%d MB", $1/1048576}'; }
n=0
for item in "$@"; do
  if [[ "$item" == "tsc" ]]; then
    log="$GEN/vt/$tag-tsc.log"
    /usr/bin/time -l timeout 3000 "$B" x tsc --noEmit -p tsconfig.json > "$log" 2>&1
    echo "tsc exit $? errors $(grep -c 'error TS' "$log") peak $(peak "$log") $(date +%H:%M)"; grep 'error TS' "$log" | head -20 | cut -c1-300
    continue
  fi
  if [[ "$item" == "tsc-react" ]]; then
    log="$GEN/vt/$tag-tsc-react.log"
    (cd "$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" && /usr/bin/time -l timeout 3000 "$B" x tsc --noEmit -p tsconfig.json) > "$log" 2>&1
    echo "tsc-react exit $? errors $(grep -c 'error TS' "$log") peak $(peak "$log") $(date +%H:%M)"; grep 'error TS' "$log" | head -20 | cut -c1-300
    continue
  fi
  n=$((n + 1)); suite="${item%%=*}"; log="$GEN/vt/$tag-$n.log"
  if [[ "$item" == *=* ]]; then /usr/bin/time -l timeout "${VT_TIMEOUT:-3000}" "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "${item#*=}" > "$log" 2>&1
  else /usr/bin/time -l timeout "${VT_TIMEOUT:-3000}" "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" > "$log" 2>&1; fi
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests|Duration) ' "$log" | tr -s ' ' | tr '\n' ';') peak $(peak "$log") $(date +%H:%M)"
  grep -E '✗|×|FAIL|AssertionError|Error:' "$log" | head -${VT_LINES:-12} | cut -c1-300
done
[[ -z "$LANE" ]] && { export B R S T U W M GEN LANE=a; source "$W/jobs.sh" a; }
