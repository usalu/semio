#!/bin/zsh
# 🧪️ Chain B of lane CL-B, one queue request after the other: the whole suite `🔭️multiview-coverage` (3600 MB) and the whole suite `🧊️three-views` (2500 MB) on the engine as it is. Lines into `$GEN/log/runs.txt`, `$GEN/log/chain-b.done` marks the end.
source "${0:A:h}/env.sh"
for pair in "3600 multiview-coverage 🔭️multiview-coverage" "2500 three-views 🧊️three-views"; do
  set -- ${=pair}
  zsh "$W/q.sh" "$1" 5400 "$2" "$S/📦️packages/🟦️typescript" "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$3/🟦️.ts"
done
date > "$GEN/log/chain-b.done"
