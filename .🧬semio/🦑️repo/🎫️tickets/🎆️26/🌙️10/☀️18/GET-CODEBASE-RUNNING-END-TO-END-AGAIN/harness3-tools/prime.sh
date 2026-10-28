#!/bin/sh
# 🔥 harness3: warms the three products before the acceptance scenarios (official runs, one at a time, sequential): course, print:test, dashboard, then course hot twice.
# Usage: sh prime.sh [tag]   (run it detached: python3 -I chain-tools/detach.py <log> sh harness3-tools/prime.sh)
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
C=$(dirname "$HERE")
TAG=${1:-h3}
run() { python3 -I "$C/gate-g-tools/product.py" "$TAG-$1" "$2"; echo "[$(date +%H:%M:%S)] $TAG-$1 exit $?"; }
run course '@teaching/architecture-course:build'
run print '@semio-tech/print-rs:test'
run dash '@semio-tech/repo-dashboard-rs:build'
run hot1 '@teaching/architecture-course:build'
run hot2 '@teaching/architecture-course:build'
echo "PRIME-DONE"
