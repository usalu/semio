#!/usr/bin/env bash
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/bjv"
S="/c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage"
mkdir -p "$G"
cd "$S/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "$T/q2-vitest-alias.config.ts" transport-field transport-view respiration parcel-jet transport-assets cast-export schema-twin schema-conformance > "$G/vitest.log" 2>&1 < /dev/null
echo "vitest rc=$?" > "$G/vitest-status.txt"
bash "$T/m9_tests.sh" 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only > "$G/py-staffage-only.log" 2>&1 < /dev/null
echo "staffage_only rc=$?" >> "$G/vitest-status.txt"
bash "$T/m9_tests.sh" 🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️ > "$G/py-vent-volume.log" 2>&1 < /dev/null
echo "vent-volume rc=$?" >> "$G/vitest-status.txt"
bash "$T/m9_tests.sh" 🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🧪️architecture.🐍️ > "$G/py-vent-arch.log" 2>&1 < /dev/null
echo "vent-arch rc=$?" >> "$G/vitest-status.txt"
echo DONE >> "$G/vitest-status.txt"
