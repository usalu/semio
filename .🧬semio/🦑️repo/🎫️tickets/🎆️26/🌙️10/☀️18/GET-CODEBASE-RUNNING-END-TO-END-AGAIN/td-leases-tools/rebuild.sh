#!/bin/sh
# 🔁 [td-leases] rebases a staged batch onto the CURRENT live files: copies the live bytes of every file of files-<batch>.txt into a scratch root, replays edit_<batch>.py there (anchored edits; it stops loudly when an anchor moved) and regenerates staged/<batch> (base = live now). Run it when `stage.py check` reports drifted after a peer landed on the same files. usage: sh rebuild.sh <envelope|startup>
set -e; T="$(cd "$(dirname "$0")" && pwd)"; C="$(dirname "$T")"; L="$(cd "$C/../../../../../../.." && pwd)"; b="$1"; S="$C/🗑️generated/tl/rb-$b"
rm -rf "$S"; mkdir -p "$S"
while IFS= read -r rel; do [ -f "$L/$rel" ] && { mkdir -p "$S/$(dirname "$rel")"; cp "$L/$rel" "$S/$rel"; }; done < "$T/files-$b.txt"
python3 -I "$T/edit_$b.py" "$S"
python3 -I "$T/make_batch.py" "$b" "$S" "$T/doc-$b.txt" $(cat "$T/files-$b.txt" | tr '\n' ' ')
python3 -I "$T/staged/$b/stage.py" check --root "$L"
rm -rf "$S"
