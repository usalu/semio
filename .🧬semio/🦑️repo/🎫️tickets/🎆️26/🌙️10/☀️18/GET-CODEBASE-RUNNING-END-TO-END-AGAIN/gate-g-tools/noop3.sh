#!/bin/sh
# 🔁 Three consecutive course:build runs for the hot no-op measurement: `noop3.sh <tag>`.
C=$(cd "$(dirname "$0")/.." && pwd)
for i in 1 2 3; do python3 "$C/gate-g-tools/product.py" "$1-course-noop$i" '@teaching/architecture-course:build'; done
