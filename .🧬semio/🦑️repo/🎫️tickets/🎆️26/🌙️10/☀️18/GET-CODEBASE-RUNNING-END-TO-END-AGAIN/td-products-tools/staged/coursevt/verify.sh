#!/bin/sh
# 🎓 usage: verify.sh <mirror-root>   (mirror with the batch applied; mirror holds 🧰️framework + 🎓️teaching)
M="$1"; T="$(cd "$(dirname "$0")/../.." && pwd)"
sh "$T/course-vitest.sh" "$M" 2>&1 | grep -v '^\[' | tail -8
