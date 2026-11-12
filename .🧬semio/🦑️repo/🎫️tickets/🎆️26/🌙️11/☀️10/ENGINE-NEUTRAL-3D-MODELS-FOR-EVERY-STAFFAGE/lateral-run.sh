#!/bin/sh
# usage: lateral-run.sh <slug> : restores the pre-task file from 🗑️generated/lateral/before, computes the plan, applies it, prints validator findings and the before/after metrics
set -e
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE"
G="$T/🗑️generated/lateral"
B=/private/tmp/claude-501/sg/bin/bun
slug=$1
cd "$T"
cp "$G/before/$slug.json" "$R/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/$slug/🔣️.json"
$B lateral-convert.ts "$slug" --before "$G/before/$slug.json" > "$G/convert-$slug.txt" 2>&1
grep "^1:" "$G/convert-$slug.txt" || true
python3 lateral-apply.py "$slug"
