#!/bin/zsh
# 🚦️ Runs the gate of the MR fixer (`gate.ts --twins`, every pose, clip instant and transition) for one asset id or one trial document and prints only the hard findings with their bones and the total; the whole listing goes to $GEN/gate/<name>.log: g.sh <id|file.json> [<log name>]   (GATE_ARGS="--poses --scales 50" narrows the run)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
arg="$1"; name="${2:-${${arg:t}%.json}}"; mkdir -p "$GEN/gate"
(cd "$T" && timeout ${GATE_TIMEOUT:-2400} "$B" "$M/gate.ts" "$arg" --twins ${=GATE_ARGS}) > "$GEN/gate/$name.log" 2>&1
echo "[DEBUG] gate exit $? $name"
awk '/ HARD /{p=1; print; next} /^    /{if(p) print; next} {p=0} /^hard findings/{print}' "$GEN/gate/$name.log"
