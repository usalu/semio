#!/bin/zsh
# 🧪️ One trial of an asset's ops under ONE grant of the memory queue (a grant costs minutes of waiting, so the steps run one after the other inside it): trial.sh <id> <name> copies the backup to $GEN/try/<name>.json, splits the lumped anchors of `anchors/<id>.json`, fits `ops/<id>.json` (OPS=<file> another ops file) four times, gates the copy over every subject (`gate2.ts --twins --json`; GATE_ARGS narrows, e.g. `--scales 50,100`), measures silhouette guard, validate and sweep (`../MR/measure.ts`; MEASURE_ARGS) and, with WHO="<scale>|<subject>|<bone-regex>;…", prints the shape extents of those subjects (`who.ts`). Output: $GEN/try/<name>.out, ended by TRIALDONE. PREPARE=1 only writes the copy and $GEN/try/<name>.inner.sh, so that several trials can share one grant (`trials.sh`).
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
a="$1"; n="${2:-$1}"; t="$GEN/try/$n.json"; o="$GEN/try/$n.out"; inner="$GEN/try/$n.inner.sh"; mkdir -p "$GEN/try"
cp "$GEN/backup/$a.json" "$t"
[[ -f "$W/anchors/$a.json" ]] && python3 "$W/split-anchors.py" "$t" "$W/anchors/$a.json" --write > /dev/null
{ print -r -- "cd \"$T\""
  for i in 1 2 3 4; do print -r -- "\"$B\" \"$W/fit2.ts\" \"$t\" --ops \"${OPS:-$W/ops/$a.json}\" --out \"$t\" > \"$GEN/try/$n.fit$i.log\" 2>&1; grep -E REFUSED \"$GEN/try/$n.fit$i.log\" | cut -c1-300; tail -1 \"$GEN/try/$n.fit$i.log\""; done
  print -r -- "\"$B\" \"$W/gate2.ts\" \"$t\" --twins ${GATE_ARGS} --json \"$GEN/try/$n.gate.json\" > \"$GEN/try/$n.gate.log\" 2>&1; grep -E 'HARD|^    [a-z]|^hard' \"$GEN/try/$n.gate.log\" | cut -c1-260"
  print -r -- "cd \"$S/📦️packages/🟦️typescript\"; \"$B\" \"$M/measure.ts\" \"$t\" ${MEASURE_ARGS} 2>&1 | grep -E 'bbox|silhouette:|validate:|sweep:' | cut -c1-240"
  print -r -- "cd \"$T\""
  for w in ${(s:;:)WHO}; do parts=("${(@s:|:)w}"); print -r -- "echo \"### 1:${parts[1]} ${parts[2]}\"; \"$B\" \"$W/who.ts\" \"$t\" ${parts[1]} \"${parts[2]}\" \"${parts[3]}\" 2>&1 | cut -c1-240"; done
} > "$inner"
[[ -n "$PREPARE" ]] && exit 0
"$W/q.sh" ${MB:-2500} ${TIMEOUT:-3000} "$T" /bin/zsh "$inner" > "$o" 2>&1; echo "TRIALDONE exit $?" >> "$o"
