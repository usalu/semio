#!/usr/bin/env bash
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
E="🎓️teaching.🏛️architecture.⚡️energy"
L="$T/🗑️generated/lc/tests3"
mkdir -p "$L"
cd /c/git/semio
run() { name="$1"; shift; bash "$T/m9_tests.sh" "$@" > "$L/$name.log" 2>&1; echo "$name exit $?" >> "$L/done.log"; }
run staffage_only "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only"
run labels "$E.🏗️authoring.🎨️visuals.🧪️tests.🐍️labels"
run cooling "$E.❄️cooling.🎬️clip.🧪️tests.🐍️"
run cooling_ig "$E.❄️cooling.🎬️clip.👥️internal-gains.🧪️tests.🐍️"
echo finished >> "$L/done.log"
