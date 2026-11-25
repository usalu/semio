#!/bin/zsh
# 🧪️ The six real suites of the proof list with one name filter over several assets, one after the other inside ONE grant of 2500 MB (a wait in the queue per suite and asset is saved): vt-all.sh <id>…   Every result line goes to $GEN/proof/vt-all.out, the whole output to $GEN/vt/all-<suite>.log
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN/vt"
export IDS="${(j:|:)@}"
cd "$R" && SEMIO_LANGUAGE=en repo capacity run --memory=2500 -- timeout 10800 zsh -c '
cd "$S/📦️packages/🟦️typescript"
echo "## $(date "+%d %H:%M") $IDS"
run() { log="$GEN/vt/all-$1.log"; timeout 2400 "$B" x vitest run --reporter=verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$1/🟦️.ts" -t "$2" > "$log" 2>&1; echo "$1 exit $? $(grep -E "^ *(Test Files|Tests) " "$log" | tr -s " " | tr "\n" ";")"; grep -E "^ *(FAIL|×|✗) " "$log" | sort -u | head -20; }
run "🧊️catalog-silhouette" "($IDS)( |$)"
run "🧊️three-views" "($IDS)( |$)"
run "🌡️catalog-physics" "($IDS)( |$)"
run "🫁️breathing-catalog" "($IDS):"
run "✅️asset-validation" "(accepts the sample figure|validates the manifest|$IDS)"
run "⚡️asset-power" "($IDS):"
' >> "$GEN/proof/vt-all.out" 2>&1
