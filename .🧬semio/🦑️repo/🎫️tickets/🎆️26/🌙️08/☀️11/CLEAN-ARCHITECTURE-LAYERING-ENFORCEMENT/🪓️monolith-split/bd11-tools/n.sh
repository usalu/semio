#!/bin/zsh
# usage: n.sh <path-substring> : needles for the one failing test file whose path contains the substring
B="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/bd11-tools"
f=$(/usr/bin/grep -aF -- "$1" $B/failing.list | head -1); [ -z "$f" ] && { echo "no match"; exit 1; }
[ $(/usr/bin/grep -aFc -- "$1" $B/failing.list) -gt 1 ] && { echo "ambiguous:"; /usr/bin/grep -aF -- "$1" $B/failing.list; exit 1; }
echo "FILE $f"
export SEMIO_TEST_ARTIFACT_DIR="$B/../🗑️generated/bd11-artifacts" SEMIO_REPO_ROOT=/Users/ueli/Documents/semio
python3 $B/needles.py "/Users/ueli/Documents/semio/$f" 2>&1 | cut -c1-${2:-260}
