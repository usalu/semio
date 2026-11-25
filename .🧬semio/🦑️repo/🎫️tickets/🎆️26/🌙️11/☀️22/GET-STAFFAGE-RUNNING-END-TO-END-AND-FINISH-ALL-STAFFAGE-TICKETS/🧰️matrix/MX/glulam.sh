#!/bin/sh
# 🪵️ Removes the sub-print bearing steps of `glulam-beam` through the chain of batch MC (its rewritten ops writer `🧰️matrix/MC/make/glulam-beam.py`, `write.sh` applies twice and expects the second run to change nothing) and proves the asset with the proof list of the multiview brief, one queue grant per step: validateAsset, prints at every key and range end, check CLI, authors' gate, derived-view findings with the proposed allowlist entry, the fresh matrix, the six suites of the brief and the multiview cases that name it. Output: 🗑️generated/matrix/MX/glulam-*.txt and 🗑️generated/matrix/MC (tool output of MC's scripts).
R=/Users/ueli/Documents/semio
U="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
T="$S/🧪️tests/🔭️multiview-coverage"
W="$U/🧰️matrix/MC"
G="$U/🗑️generated/matrix/MX"
B=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$R" || exit 2
repo capacity run --memory=1000 -- timeout 900 /bin/zsh "$W/write.sh" glulam-beam > "$G/glulam-write.txt" 2>&1
repo capacity run --memory=800 -- timeout 600 "$B" "$W/validate.ts" glulam-beam > "$G/glulam-validate.txt" 2>&1
ENDS=$(python3 -c "
import json,glob
a=json.load(open(glob.glob('$S/🗂️catalog/*/glulam-beam/🔣️.json')[0]))['asset']
ds=set()
for k,r in a['representations'].items():
    if r.get('status')=='authored': ds|={int(k),r['validRange']['minDenominator'],r['validRange']['maxDenominator']}
print(','.join(str(d) for d in sorted(ds)))")
repo capacity run --memory=800 -- timeout 600 "$B" "$W/print.ts" glulam-beam "$ENDS" --no-ops > "$G/glulam-print.txt" 2>&1
repo capacity run --memory=1000 -- timeout 900 /bin/zsh "$W/check.sh" glulam-beam > "$G/glulam-check.txt" 2>&1
repo capacity run --memory=800 -- timeout 900 "$B" "$T/🔎️scan.ts" --language en glulam-beam > "$G/glulam-scan.txt" 2>&1
repo capacity run --memory=800 -- timeout 600 "$B" "$U/🧰️matrix/MX/derived-with.ts" glulam-beam steel-column > "$G/glulam-derived.txt" 2>&1
repo capacity run --memory=1200 -- timeout 3000 "$B" "$T/📊️matrix.ts" --language en --out "$G/matrix-glulam" --jobs 1 --fresh --assets glulam-beam > "$G/glulam-matrix.txt" 2>&1
repo capacity run --memory=2500 -- timeout 3400 /bin/zsh "$W/beam.sh" mx > "$G/glulam-suites.txt" 2>&1
echo done
