#!/bin/sh
# usage: gate.sh <crate dir relative to repo root>...  canonical source-boundary closure (prod + test cfg) per crate
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
for d in "$@"; do bun "$PWD/$C/rgplug-tools/gate.ts" "$d"; done
