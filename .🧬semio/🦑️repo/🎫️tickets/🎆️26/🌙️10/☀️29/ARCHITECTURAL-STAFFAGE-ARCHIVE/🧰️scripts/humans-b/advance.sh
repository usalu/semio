#!/bin/bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
cd /home/deck/semio; repo tools exec bun -- "$T/🧰️scripts/humans-b-advance.ts" "$T/🧰️scripts/humans-b/out/$1.json" 2>&1 | grep -v '^\['
