#!/bin/bash
# Applies the prepared 🏃️process patches (pass 1030 actinfra) to the real checkout once act12 has an exit line.
# Usage: apply-process-patches.sh   (run from anywhere); stops at the first patch that does not apply cleanly.
set -e
T="$(cd "$(dirname "$0")" && pwd)"; cd /Users/ueli/Documents/semio
grep -q '^exit' "$T/../../🗑️generated/1030/act12.status" || { echo "act12 has not exited"; exit 1; }
for p in "$T"/process-patches/*.patch; do patch -p1 --no-backup-if-mismatch < "$p"; done
