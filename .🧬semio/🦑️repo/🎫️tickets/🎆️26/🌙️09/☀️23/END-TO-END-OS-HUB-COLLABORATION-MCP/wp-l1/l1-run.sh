#!/bin/zsh
# 🚆️ L1: apply sets in order (dry run → write inside l1-land). A set whose dry run fails is SKIPPED (nothing written); a set whose
# write returns rc≠0 is REVERTED at once (l1-land revert). Continues with the next set either way. Summary lines "[l1-run] …".
# usage: zsh l1-run.sh <capture> <set…>
setopt no_bg_nice
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-l1
out="${1:A}"; shift
{
  echo "[l1-run] START $(date '+%F %T') sets: $*"
  for s in "$@"; do
    if [ -f "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-l1-backup/$s/manifest.json" ]; then echo "[l1-run] ALREADY-LANDED $s (record exists) — untouched"; continue; fi
    python3 $W/l1-train.py apply "$s"; rc=$?
    if [ -f "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-l1-backup/$s/manifest.json" ]; then
      if [ $rc -eq 0 ]; then
        echo "[l1-run] LANDED $s $(date '+%T') $(python3 -c "import json;d=json.load(open('/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-l1-backup/$s/manifest.json'));print(len(d['files']),'files')")"
      else
        echo "[l1-run] WRITE-FAILED $s rc=$rc → revert"; python3 $W/l1-land.py revert "$s"; echo "[l1-run] REVERTED $s $(date '+%T')"
      fi
    else
      echo "[l1-run] SKIPPED $s rc=$rc (dry run) $(date '+%T')"
    fi
  done
  echo "[l1-run] END $(date '+%F %T')"
} >> "$out" 2>&1
