#!/usr/bin/env bash
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
E="🎓️teaching.🏛️architecture.⚡️energy"
L="$T/🗑️generated/lc/tests"
mkdir -p "$L"
cd /c/git/semio
run() {
  name="$1"; shift
  bash "$T/m9_tests.sh" "$@" > "$L/$name.log" 2>&1
  echo "$name exit $?" >> "$L/done.log"
}
run staffage_only "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only"
run visuals "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️"
run scenes "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️scenes"
run cooling_ig "$E.❄️cooling.🎬️clip.👥️internal-gains.🧪️tests.🐍️"
run cooling "$E.❄️cooling.🎬️clip.🧪️tests.🐍️"
run vent_arch "$E.🌬️ventilation.🎬️clip.🧪️architecture.🐍️"
run volume "$E.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️"
run physics "$E.🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️"
run authoring_clip "$E.🏗️authoring.🎬️clip.🧪️tests.🐍️"
run integration "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️integration"
echo finished >> "$L/done.log"
