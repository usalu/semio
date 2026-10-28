#!/bin/sh
# 🔁️ quietfc-g: rebuilds the working overlay from the live tree and applies the whole staged change to it (schema, code edits, new files). Usage: sh rebuild.sh
echo "SUPERSEDED (quietfc2, 10-27): the anchor route no longer carries the full-request bound; use stage.py check|apply|revert (flat batch) and verify.sh with QFC_ROOT=<mirror>" >&2; exit 2
set -e
H=$(cd "$(dirname "$0")" && pwd)
OV=/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/quietfc-g-ov
python3 "$H/overlay.py" build ov >/dev/null
python3 "$H/edit_schema.py" "$OV" >/dev/null
python3 "$H/edit_code.py" "$OV" >/dev/null
python3 "$H/install_new.py" "$OV"
R="$OV/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests"
(cd "$R/🔼️upgrade/🐍️oracle" && python3 ./🐍️.py >/dev/null)
(cd "$R/🌤️followup/🐍️oracle" && python3 ./🐍️.py >/dev/null)
