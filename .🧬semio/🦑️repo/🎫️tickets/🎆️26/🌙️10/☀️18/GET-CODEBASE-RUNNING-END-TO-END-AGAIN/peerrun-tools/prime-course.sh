#!/bin/sh
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
C=$(dirname "$HERE")
TAG=${1:-pr}
run() { python3 -I "$C/gate-g-tools/product.py" "$TAG-$1" "$2"; echo "[$(date +%H:%M:%S)] $TAG-$1 exit $?"; }
run course '@teaching/architecture-course:build'
run hot1 '@teaching/architecture-course:build'
run hot2 '@teaching/architecture-course:build'
echo "PRIME-DONE"
