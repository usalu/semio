#!/bin/sh
# usage: rebuild.sh   rebuilds the scratch overlay (the library plus every path in <overlay>.copies) and replays the recorded edits
T=$(cd "$(dirname "$0")" && pwd)
OV=${LD_OV:-/private/tmp/claude-501/ld}
cp "$OV.edits.jsonl" "$OV.edits.bak" 2>/dev/null
rm -f "$OV.edits.jsonl"
python3 "$T/overlay.py" "$OV" >/dev/null
[ -s "$OV.closures" ] && python3 "$T/overlay.py" --closure "$OV" $(cat "$OV.closures")
cp "$OV.edits.bak" "$OV.edits.jsonl" 2>/dev/null
python3 -c "
import sys;sys.path.insert(0,'$T');import ed;ed.replay()"
