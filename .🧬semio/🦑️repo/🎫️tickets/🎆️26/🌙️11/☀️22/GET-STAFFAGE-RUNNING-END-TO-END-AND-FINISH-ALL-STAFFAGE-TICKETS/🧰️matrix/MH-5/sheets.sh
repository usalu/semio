#!/bin/zsh
# 🖼️ The sheets to look at for one person, under the lock: sheets.sh <id> <pose|-> <scale>… renders the product sheet of each scale from the backup (before) and from the catalog file (after), in the default pose (`-`) or the named pose, into $GEN/sheets (look.sh of MR).
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
a="$1"; pose="$2"; shift 2
[[ "$pose" == "-" ]] && unset POSE || export POSE="$pose"
"$W/q.sh" env BATCH=MH-5 TAG=before "$M/look.sh" "$a" "$@"
"$W/q.sh" env BATCH=MH-5 TAG=after "$M/look.sh" "$a" "$@"
