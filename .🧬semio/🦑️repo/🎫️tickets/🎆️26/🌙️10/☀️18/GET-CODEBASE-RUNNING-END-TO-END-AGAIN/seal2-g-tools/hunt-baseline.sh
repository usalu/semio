#!/bin/sh
# 🎲️ Promotes the baseline mirror then hunts the flake on it. usage: sh hunt-baseline.sh <mirror> <tag>
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 2
python3 -I $C/mirror-tools/mirror.py run $1 --peers-ok -- env SEMIO_REPO_IMPLEMENTATION=promote bun repo --help > "${TMPDIR:-/tmp}/s2g-flake/$2-promote.txt" 2>&1
exec sh $C/seal2-g-tools/flake-all.sh $1 $2
