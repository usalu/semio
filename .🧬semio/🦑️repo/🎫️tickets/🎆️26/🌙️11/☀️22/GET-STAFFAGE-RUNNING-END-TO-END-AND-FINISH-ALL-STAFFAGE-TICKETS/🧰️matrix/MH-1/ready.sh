#!/bin/zsh
# ✅️ Says whether the trial copy of an asset may be written: the trial gate ended without a hard finding, the silhouette guard of the trial is OK and the validator issues and sweep findings are not more than before; exit 0 when so: ready.sh <id>
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
a="$1"; n() { grep -E "$2" "$GEN/measure/$1-$a.log" | grep -oE '[0-9]+ (issues|findings)' | cut -d' ' -f1 }
[[ "$(tail -1 "$GEN/gate/try-$a.log")" == "hard findings: 0" ]] || { echo "$a not ready: $(tail -1 "$GEN/gate/try-$a.log")"; exit 1 }
grep -q "silhouette: OK" "$GEN/measure/try-$a.log" || { echo "$a not ready: silhouette"; exit 1 }
[[ "$(n try validate:)" -le "$(n before validate:)" && "$(n try sweep:)" -le "$(n before sweep:)" ]] || { echo "$a not ready: validate or sweep"; exit 1 }
echo "$a ready"
