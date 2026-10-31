#!/bin/bash
# usage: probe.sh asset  -> generated/NW-H3/probe-<asset>.txt
T=/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE
cd /home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage
repo tools exec bun -- $T/🧰️scripts/core/suit-probe.ts --asset $1 --limit ${2:-60} > $T/🗑️generated/NW-H3/probe-$1.txt 2>&1
