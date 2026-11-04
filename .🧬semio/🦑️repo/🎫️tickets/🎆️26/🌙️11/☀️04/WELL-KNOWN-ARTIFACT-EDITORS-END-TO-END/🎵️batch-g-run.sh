#!/usr/bin/env bash
set -u
ROOT=/home/deck/semio
A="$ROOT/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END/🗑️generated"
run() { local dir="$1" name="$2"; shift 2; (cd "$A/$dir" && repo cargo --store=shared --configuration=prototype -- test "$@" --lib) > "$T/batch-g-$name.log" 2>&1; echo "$name exit=$?" >> "$T/batch-g-summary.log"; }
runroot() { local name="$1"; shift; (cd "$ROOT" && repo cargo --store=shared --configuration=prototype -- test "$@" --lib) > "$T/batch-g-$name.log" 2>&1; echo "$name exit=$?" >> "$T/batch-g-summary.log"; }
: > "$T/batch-g-summary.log"
runroot kit -p semio-framework-plugin-window-kits -p semio-framework-ui
run 🎵️mp3 mp3 --workspace
run 🔊️wav wav --workspace
run 🎥️mp4 mp4 --workspace
run 📼️avi avi --workspace
