#!/bin/sh
cd "$1"; out="$2"; : > "$out"
for d in 🪣️declined 🔓️release 🏘️hub 🪆️authority; do
 echo "### $d" >> "$out"
 SEMIO_TEST_ARTIFACT_DIR="$3" timeout 900 repo tools exec bun -- test "./🧪️tests/$d/🟦️.ts" 2>&1 | grep -v "capacity\|still working" >> "$out"
done
echo "### done" >> "$out"
