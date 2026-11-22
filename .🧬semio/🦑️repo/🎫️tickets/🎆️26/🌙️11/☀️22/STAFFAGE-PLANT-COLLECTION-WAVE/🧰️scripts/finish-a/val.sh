#!/bin/sh
# 🩺 Finisher A inner loop: whole-asset validation of one candidate document with the given r3-validate flags (for example `--volumes --no-print`, `--scale 500 --rules cross-scale --no-print`), lines cut to 400 characters.
# usage: val.sh <asset document json> [r3-validate flags...]
HERE="$(cd "$(dirname "$0")" && pwd)"
P="$HERE/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
FILE="$1"; shift
cd "$P" && "$BUN" r3-validate.ts --file "$FILE" "$@" 2>&1 | cut -c1-400
