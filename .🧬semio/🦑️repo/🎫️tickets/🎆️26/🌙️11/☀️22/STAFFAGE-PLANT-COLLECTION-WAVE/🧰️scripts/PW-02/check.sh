#!/bin/sh
# Build one PW-02 tree and run the fast validation (schema + validateAsset, optional extra r3-validate args), one tool process at a time.
# usage: check.sh <id> [r3-validate args...]
set -u
ID="$1"; shift
HERE="$(cd "$(dirname "$0")" && pwd)"
P="$HERE/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
C="$HERE/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/$ID/🔣️.json"
G="$HERE/../../🗑️generated/PW-02"
mkdir -p "$G"
cd "$HERE" && repo tools exec bun -- build.ts "$ID" 2>&1 | grep -v "^\[capacity\]\|still working\|waits for" > "$G/build-$ID.log"
cd "$P" && repo tools exec bun -- r3-validate.ts --file "$C" "$@" 2>&1 | grep -v "^\[capacity\]\|still working\|waits for" > "$G/val-$ID.txt"
tail -1 "$G/val-$ID.txt"
