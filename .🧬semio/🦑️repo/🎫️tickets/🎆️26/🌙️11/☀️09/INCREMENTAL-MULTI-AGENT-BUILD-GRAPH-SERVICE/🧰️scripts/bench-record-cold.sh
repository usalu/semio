#!/usr/bin/env bash
# 📝 Waits for the queued cold-plan baseline report and appends its numbers to the bench audit.
T="$(cd "$(dirname "$0")/.." && pwd)"; D="$T/🗑️generated/bench/baseline"
while ! ls "$D"/report-*.json >/dev/null 2>&1; do sleep 30; done
f=$(ls -t "$D"/report-*.json | head -1)
python3 - "$f" "$T/🔍️audits/bench.md" <<'PY'
import json,sys
r=json.load(open(sys.argv[1]));s=r["results"][0]["samples"]
out="\n## Cold-plan baseline (queued dry-run through the gate)\n\n"
if not s: out+="Status: %s, no sample.\n"%r["results"][0]["status"]
for x in s:
    out+="exit %s, wall %d ms, phases %s, closures %s, peak repo RSS %s MB, swap %s MB, min available %s MB, OOM kills %s\n"%(x["exit"],x["wallMs"],x["phaseMs"],x["closures"],x["peakRssMb"],x["swapMb"],x["minAvailableMb"],x["oomKills"])
open(sys.argv[2],"a").write(out)
PY
