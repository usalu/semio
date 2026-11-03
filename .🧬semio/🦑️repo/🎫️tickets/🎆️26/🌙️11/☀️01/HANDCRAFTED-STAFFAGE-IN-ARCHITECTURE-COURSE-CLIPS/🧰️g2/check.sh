#!/bin/bash
# usage: check.sh asset-id [extra check args]   builds the asset into the catalog and runs the catalog check on it
cd /c/git/semio
S="🎓️teaching/🏛️architecture/🧍️staffage"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
id="$1"; shift
repo tools exec bun -- "$T/🧰️g2/build.mjs" "$S/🗂️catalog" "$id" 2>&1 | tail -3
repo tools exec bun -- "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$id" "$@" 2>&1 | tail -40
