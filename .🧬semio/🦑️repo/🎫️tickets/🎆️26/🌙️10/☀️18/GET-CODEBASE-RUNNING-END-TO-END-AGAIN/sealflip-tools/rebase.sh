#!/bin/sh
# 🔁️ [sealflip] Re-cuts batch `sealflip` on the CURRENT live files (after B1 + D2 landed, or whenever the gate reports drift): copies the 11 files from live into the mirror, keeps them as base, replays the anchored edits (edit.py refuses on a moved anchor), rebuilds files/ + manifest.json + stage.py, then verify.sh. usage: sh rebase.sh <mirror-root>
R="${1:?mirror root}"; H="$(cd "$(dirname "$0")" && pwd)"; L=/Users/ueli/Documents/semio
REQ="✏️s/🧑‍💻dev/🧩️catalog/📣️publication"; [ -d "$R/$REQ" ] || { echo "the mirror lacks $REQ (the capture of the sealed entry lstats ✏️s): create it with: python3 -I C/mirror-tools/mirror.py create <name> --only library process launcher \"$REQ\""; exit 2; }
python3 -I - "$H" "$L" "$R" <<'PY' || exit 2
import os, shutil, sys
h, live, mirror = sys.argv[1:4]; g = {"__file__": os.path.join(h, "batch.py"), "__name__": "batchlib"}; exec(open(os.path.join(h, "batch.py")).read().split("def main")[0], g)
for rel in g["FILES"]: os.makedirs(os.path.dirname(os.path.join(mirror, rel)), exist_ok=True); shutil.copy2(os.path.join(live, rel), os.path.join(mirror, rel))
print("synced", len(g["FILES"]), "files from live")
PY
python3 -I "$H/batch.py" base "$R" && python3 -I "$H/edit.py" "$R" && python3 -I "$H/batch.py" make "$R" && sh "$H/staged/sealflip/verify.sh" "$R"
