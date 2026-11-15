#!/usr/bin/env bash
set -o pipefail
cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
T="$(cd "$(dirname "$0")" && pwd)"
tag="${1:-describe}"
mkdir -p "$T/🗑️generated"
repo run workspace:components -- describe norm 2>&1 | tee "$T/🗑️generated/$tag.log"
echo "EXIT ${PIPESTATUS[0]}" >> "$T/🗑️generated/$tag.log"
