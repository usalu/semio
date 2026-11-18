#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
B=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
I=$B/🕸️graph/📥️inference
N=$I/📦️project-nodes
m=$((10#$(date -u +%M)))
if ! { (( m <= 4 )) || (( m >= 30 && m <= 34 )); }; then echo "outside the landing window"; exit 2; fi
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R101 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
python3 "$D/land2.py" || { python3 "$D/restore2.py"; rm -r "$T/landing.lock"; exit 3; }
"$D/runall.sh" after2 "$N/🌐️owner-pool/🧪️tests/🎚️lanes/🟦️.ts" "$B/👁️observation/🧪️tests/🔎️lookup-holds/🟦️.ts" "$B/👁️observation/🧪️tests/🏁️rows-at-end/🟦️.ts" "$N/🌐️owner-pool/🧪️tests/🟦️.ts" "$N/🌐️owner-pool/🧪️tests/🔁️no-record/🟦️.ts" "$I/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$I/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts" "$N/🧪️tests/📥️take/🟦️.ts" "$N/📥️take/🧪️tests/🟦️.ts" > /dev/null 2>&1
cat /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R101/after2.txt | cut -c1-30,1-1 >/dev/null
awk -F'|' '{print $2}' /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R101/after2.txt | sed 's#.*/##' | head -0
cat /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R101/after2.txt | sed 's#^[^|]*|##' | cut -c1-260
date -u
