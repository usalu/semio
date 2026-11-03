#!/usr/bin/env bash
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
mkdir -p "$T/🗑️generated/g6"
bash "$T/m9_tests.sh" 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only >"$T/🗑️generated/g6/run.txt" 2>&1 </dev/null
grep -aE "^AssertionError|^Ran |^OK|^FAILED" "$T/🗑️generated/g6/run.txt" | cut -c1-200
