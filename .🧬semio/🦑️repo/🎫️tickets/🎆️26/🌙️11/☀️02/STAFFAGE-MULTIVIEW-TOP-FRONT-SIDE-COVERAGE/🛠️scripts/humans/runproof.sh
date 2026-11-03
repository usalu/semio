#!/bin/sh
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE"
slug=$1; scale=$2; shift 2
cd /c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage && bun 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$slug" --scale "$scale" "$@" --views --projection third --out "$T/🗑️generated/humans/proofs/$slug" >/dev/null 2>&1 || echo "FAIL $slug $scale $*"
