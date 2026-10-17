#!/usr/bin/env bash
# [DEBUG] Render PF beats at -ql with layout guard + x-extent audit; log to checks/audit_<beat>.log
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
OUT="${PF_MEDIA:-/tmp/pf_media}"
cd "$ROOT"
for BEAT in "$@"; do
  LAYOUT_CHECK=1 .venv/bin/python "$(dirname "$0")/extent_audit.py" -ql --disable_caching --media_dir "$OUT" \
    tutorial/energy/demand/1_physical_fundamentals/scene_1.py "$BEAT" > "$(dirname "$0")/audit_$BEAT.log" 2>&1
  echo "== $BEAT"; grep -E "\[LAYOUT\]|\[DEBUG\]|Traceback|Error" "$(dirname "$0")/audit_$BEAT.log" | head -40
done
