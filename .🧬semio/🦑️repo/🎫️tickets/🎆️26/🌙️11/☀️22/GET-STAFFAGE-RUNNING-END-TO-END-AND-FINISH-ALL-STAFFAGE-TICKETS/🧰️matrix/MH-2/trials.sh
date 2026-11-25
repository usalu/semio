#!/bin/zsh
# 🧫️ Several trials under ONE grant: trials.sh <round> <name>… runs the prepared $GEN/try/<name>.inner.sh (written by `PREPARE=1 trial.sh <id> <name>`) one after the other, each into $GEN/try/<name>.out ended by TRIALDONE, and appends `ROUND <round> done` to $GEN/try/rounds.txt.
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
round="$1"; shift; master="$GEN/try/round-$round.sh"; : > "$master"
for n in "$@"; do print -r -- "/bin/zsh \"$GEN/try/$n.inner.sh\" > \"$GEN/try/$n.out\" 2>&1; echo \"TRIALDONE exit \$?\" >> \"$GEN/try/$n.out\"" >> "$master"; done
"$W/q.sh" ${MB:-2500} ${TIMEOUT:-7200} "$T" /bin/zsh "$master"; echo "ROUND $round done exit $? $(date +%H:%M)" >> "$GEN/try/rounds.txt"
