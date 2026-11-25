#!/bin/zsh
# 🧮️ Measures the candidate drawings of the leftovers (`left-candidates.py` writes them into $GEN/left) with the measure of the suite (derived.ts) and the print check of every band and its range ends (prints.ts), and the language probe of `storage-crate-stack` on the catalog document: left.sh   (run it through q.sh with 1000 MB; output in $GEN/left/candidates.txt and langgeo-<time>.txt)
source "${0:h}/env.sh"
cd "$W" && python3 left-candidates.py > "$GEN/left/candidates-py.txt" 2>&1
cd "$T"
files=("$GEN"/left/steel-column-*.json "$GEN"/left/glulam-beam-*.json)
{ timeout 600 "$B" "$W/derived.ts" $files steel-column glulam-beam; timeout 600 "$B" "$W/prints.ts" $files steel-column glulam-beam; } > "$GEN/left/candidates.txt" 2>&1
timeout 600 "$B" "$W/langgeo.ts" storage-crate-stack --items 1 > "$GEN/left/langgeo-$(date +%H%M).txt" 2>&1
tail -1 "$GEN"/left/langgeo-*.txt
