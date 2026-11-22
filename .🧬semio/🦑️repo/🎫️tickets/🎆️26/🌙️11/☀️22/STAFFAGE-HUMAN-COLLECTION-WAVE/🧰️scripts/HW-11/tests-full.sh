#!/bin/sh
# Runs the catalog-wide staffage tests that are not named per asset over the whole catalog; the HW-11 assets are then looked up in the logs ($GEN/tests-full).
. "$(dirname "$0")/env.sh"
mkdir -p "$GEN/tests-full"
cd "$R"
for t in ${TESTS:-✅️asset-validation 🔭️multiview-coverage 🧊️volumes-connected 🧍️two-foot-stance 📏️print-suitability 🧬️schema-conformance 🕳️valid-range-coverage 🪞️representation-copied 🧊️member-solid 🪤️volume-traps 🧿️exposed-normals 🦶️foot-plant 🦶️centre-of-pressure 🏃️inertial-consistency 💨️transport-assets 🧍️front-projection 🧊️volumes-thermal 🧊️model-bounds}; do
  timeout 3000 $B node_modules/vitest/vitest.mjs run --configLoader native --config "🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts" "$t" > "$GEN/tests-full/$t.log" 2>&1
  echo "$t: $(grep -E '^ +Tests ' "$GEN/tests-full/$t.log" | tail -1) | HW-11 mentions: $(grep -cE 'waving-person|photographer-with-camera|person-taking-selfie' "$GEN/tests-full/$t.log")"
done
