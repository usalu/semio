#!/bin/sh
# Applies the staged infer-f batch after the 02:45 freeze: refuses when a file changed since staging (BASE.tsv holds the sha256 of the tree content the batch was made against), otherwise copies staged/files over the tree. Run from anywhere; announce in result-infer-f.md first.
set -e
cd /Users/ueli/Documents/semio
S='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/infer-f-tools/staged'
while IFS="$(printf '\t')" read -r hash f; do
  if [ -e "$f" ]; then now=$(shasum -a 256 "$f" | cut -d' ' -f1); else now=absent; fi
  [ "$now" = "$hash" ] || { echo "CHANGED SINCE STAGING (merge by hand): $f"; exit 1; }
done < "$S/BASE.tsv"
while IFS= read -r f; do mkdir -p "$(dirname "$f")"; cp "$S/files/$f" "$f"; done < "$S/FILES.txt"
python3 "$S/readme-patch.py"
echo applied
