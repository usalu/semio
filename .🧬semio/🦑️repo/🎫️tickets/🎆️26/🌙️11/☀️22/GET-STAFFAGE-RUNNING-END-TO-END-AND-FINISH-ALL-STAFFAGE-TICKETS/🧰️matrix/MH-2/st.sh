#!/bin/zsh
# ⏳️ Waits up to <seconds> (default 170) for a new line in the job log or the matrix summary of batch MH-2, then prints both, the peaks and the place of the batch's requests in the memory queue: st.sh [seconds]
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"; setopt null_glob
n0=$(cat "$GEN/jobs/log.txt" "$GEN/proof/matrix-summary.txt" 2>/dev/null | wc -l)
while (( SECONDS < ${1:-170} )) && (( $(cat "$GEN/jobs/log.txt" "$GEN/proof/matrix-summary.txt" 2>/dev/null | wc -l) == n0 )); do sleep 5; done
echo "--- jobs"; tail -6 "$GEN/jobs/log.txt" 2>/dev/null; echo "--- waiting: $(ls "$GEN/jobs"/*.job 2>/dev/null | xargs -n1 basename | tr '\n' ' ')"
echo "--- matrix"; cut -c1-220 "$GEN/proof/matrix-summary.txt" 2>/dev/null | tail -8; for f in "$GEN"/jobs/*.peak "$GEN"/proof/*.peak; do echo "${f:t}: $(tail -1 $f)"; done | tail -4
cd "$R" && repo capacity status --json 2>/dev/null | python3 -c "
import json,sys,subprocess
d=json.load(sys.stdin); now=d['host']['atMs']
mine=set(subprocess.run(['pgrep','-f','MH-2/'],capture_output=True,text=True).stdout.split())
kids=set()
for p in mine: kids|=set(subprocess.run(['pgrep','-P',p],capture_output=True,text=True).stdout.split())
mine|=kids
ws=sorted(d['waiters'],key=lambda w:w['arrivedAtMs'])
print('headroom',d['evaluation']['headroomMegabytes'],'running',[(g['reservedMegabytes'],g['observedMegabytes'],'MINE' if str(g['pid']) in mine else '') for g in d['grants']],'waiting',len(ws),'my places',[(i+1,w['request']['fixedMegabytes']) for i,w in enumerate(ws) if str(w['pid']) in mine])"
date +%H:%M
