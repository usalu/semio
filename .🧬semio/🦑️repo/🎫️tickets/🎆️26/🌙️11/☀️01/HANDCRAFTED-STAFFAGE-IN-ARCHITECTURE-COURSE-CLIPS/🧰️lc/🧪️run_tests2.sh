#!/usr/bin/env bash
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
E="🎓️teaching.🏛️architecture.⚡️energy"
L="$T/🗑️generated/lc/tests2"
cd /c/git/semio
run() { name="$1"; shift; bash "$T/m9_tests.sh" "$@" > "$L/$name.log" 2>&1; echo "$name exit $?" >> "$L/done.log"; }
run labels "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️labels"
run physics "$E.🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️"
run volume "$E.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️"
run scenes "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️scenes"
run integration_summer "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️integration.SceneIntegrationTest.test_summer_occupant_follows_house_scale"
echo finished >> "$L/done.log"
