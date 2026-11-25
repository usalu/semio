#!/bin/zsh
# ⚓️ The anchor and whole-figure findings of a trial copy: anchors.sh <file.json> runs the real gate of MR over the poses only (under the lock) and prints the hard lines that are not per-bone synchrony.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
(cd "$T" && "$W/q.sh" timeout 1500 "$B" "$M/gate.ts" "$1" --poses) | grep -A6 -E " HARD (anchor|sync-(height|forward|lateral)|dimension|evaluate)" | cut -c1-250
