#!/bin/bash
cd /c/git/semio
S="🎓️teaching/🏛️architecture/🧍️staffage"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
[ "$1" = "build" ] && { repo tools exec bun -- "$T/🧰️sc/build.mjs" "$S/🗂️catalog" 2>&1 | tail -3; shift; }
repo tools exec bun -- "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset solar-thermal-collector "$@" 2>&1 | tail -60
