#!/bin/sh
# Runs the catalog-wide staffage tests scoped by test name to the HW-11 assets and their bases (vitest -t); one log per test file in $GEN/tests.
. "$(dirname "$0")/env.sh"
mkdir -p "$GEN/tests"
cd "$R"
F="${FILTER:-waving-person|photographer-with-camera|person-taking-selfie|standing-adult|standing-phone-user|tour-guide}"
for t in ${TESTS:-✅️asset-validation ⚡️asset-power 🫁️breathing-catalog 🌡️catalog-physics 🔭️multiview-coverage 🧊️catalog-silhouette 🧊️volumes-connected 🧊️volumes-thermal 🧊️volumes-rules 🧊️three-views 🧊️member-solid 🪤️volume-traps 🧿️exposed-normals 🧬️schema-conformance 🧍️two-foot-stance 🦶️foot-plant 🕳️valid-range-coverage 🪞️representation-copied 📏️print-suitability 🧍️front-projection 🧍️plan-projection 💨️transport-assets 🏃️inertial-consistency 🦶️centre-of-pressure 🧊️model-bounds 🧊️model-silhouette 🧊️volumes-flood}; do
  timeout 3000 $B node_modules/vitest/vitest.mjs run --configLoader native --config "🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts" "$t" -t "$F" > "$GEN/tests/$t.log" 2>&1
  echo "$t: $(grep -E '^ +Tests ' "$GEN/tests/$t.log" | tail -1)"
done
