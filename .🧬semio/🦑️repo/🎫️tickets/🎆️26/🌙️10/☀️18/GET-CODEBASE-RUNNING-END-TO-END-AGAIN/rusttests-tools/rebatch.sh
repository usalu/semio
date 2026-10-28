#!/bin/sh
# 🔁 [rusttests] regenerates staged/envelope against the CURRENT live files (replays the anchored edits of edit*.py; stops loudly when an anchor moved), then checks it against live. Run it when `stage.py check` reports drifted. usage: sh rebatch.sh
set -e; T="$(cd "$(dirname "$0")" && pwd)"; C="$(dirname "$T")"; L="$(cd "$C/../../../../../../.." && pwd)"; G="$C/🗑️generated/rt"
sh "$T/rebuild.sh"
python3 -I "$T/make_batch.py" envelope "$G/S0" "$G/S1" "$T/doc-envelope.txt"
python3 -I "$T/staged/envelope/stage.py" check --root "$L"
