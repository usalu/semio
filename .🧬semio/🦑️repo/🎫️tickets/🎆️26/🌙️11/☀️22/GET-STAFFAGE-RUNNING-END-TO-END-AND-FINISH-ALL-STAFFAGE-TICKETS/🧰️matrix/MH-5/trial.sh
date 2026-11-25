#!/bin/zsh
# 🧷️ A trial of extra ops on top of the kept ones: trial.sh <id> <scales> '<extra ops json>' applies ops/<id>.json plus the extra ops to a copy ($GEN/try/<id>.trial.json), prints the fit lines of the extra ops, the model gate, the silhouette guard of the backup and the copy at the scales (sil.ts) and the anchor and whole-figure findings of the real gate over the poses (anchors.sh).
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
a="$1"; scales="$2"; out="$GEN/try/$a.trial.json"; ops="$GEN/try/$a.trial.ops.json"; mkdir -p "$GEN/try"
python3 -c 'import json,sys,os; p=sys.argv[1]; base=json.load(open(p)) if os.path.exists(p) else []; extra=json.loads(sys.argv[2]); json.dump(base+extra,open(sys.argv[3],"w")); print(len(base),"kept ops +",len(extra))' "$W/ops/$a.json" "$3" "$ops"
n=$(python3 -c 'import json,sys; print(len(json.loads(sys.argv[1])))' "$3")
(cd "$T" && "$W/b.sh" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$ops" --out "$out" | grep -v "^written" | tail -$((n + 1)) | cut -c1-240; "$W/b.sh" "$W/mgate.ts" "$out" | grep -E "HARD" | cut -c1-200)
(cd "$S/📦️packages/🟦️typescript" && "$W/q.sh" timeout 1500 "$B" "$W/sil.ts" --scales "$scales" "$GEN/backup/$a.json" "$out")
"$W/anchors.sh" "$out"
