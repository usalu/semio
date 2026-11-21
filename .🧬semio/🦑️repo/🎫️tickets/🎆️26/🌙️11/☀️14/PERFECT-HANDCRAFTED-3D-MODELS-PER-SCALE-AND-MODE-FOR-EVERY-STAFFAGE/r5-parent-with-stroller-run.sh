#!/bin/sh
# 🔁 Round 5: rebuilds the volumes source, the 2D of every scale and the candidate of humans/parent-with-stroller, validates it and probes print suitability.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r5-parent-with-stroller-forms.py >/dev/null && "$BUN" r3-stroller-build.ts && "$BUN" r3-stroller-draw2d.ts >/dev/null && python3 r3-stroller-roots.py >/dev/null && python3 r3-stroller-candidate.py >/dev/null && "$BUN" r3-validate.ts --file "🗑️generated/r3-parent-with-stroller/full/🔣️.json" | tail -6
