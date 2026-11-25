#!/bin/zsh
# ✅️ Says whether the trial copy of an asset may be written: the trial gate ended without a hard finding, the silhouette guard of the trial is OK and the validator issues and sweep findings are not more than before; exit 0 when so: ready.sh <id>   (PARTIAL=1 accepts a trial that has fewer hard findings than the measurement before instead of none)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
a="$1"; n() { grep -E "$2" "$GEN/measure/$1-$a.log" | grep -oE '[0-9]+ (issues|findings)' | cut -d' ' -f1 }
h() { tail -1 "$GEN/gate/$1-$a.log" | grep -oE '[0-9]+$' }
if [[ -n "$PARTIAL" ]]; then [[ -n "$(h try)" && "$(h try)" -lt "$(h before)" ]] || { echo "$a not better: $(h before) before, $(h try) in the trial"; exit 1 }
else [[ "$(tail -1 "$GEN/gate/try-$a.log")" == "hard findings: 0" ]] || { echo "$a not ready: $(tail -1 "$GEN/gate/try-$a.log")"; exit 1 }; fi
grep -q "silhouette: OK" "$GEN/measure/try-$a.log" || { echo "$a not ready: silhouette"; exit 1 }
[[ "$(n try validate:)" -le "$(n before validate:)" && "$(n try sweep:)" -le "$(n before sweep:)" ]] || { echo "$a not ready: validate or sweep"; exit 1 }
echo "$a ready"
