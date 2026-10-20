#!/usr/bin/env bash
# Verifies the x-inputs staging.
#   check.sh mirror <dir>   byte-copy the course closure into <dir> (outside the repo), apply, run the laws there
#   check.sh live           run the laws against the live tree (after apply.py ran on it)
# Never hard-links or clones repository sources.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
tests="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests"
mode="${1:-}"
case "$mode" in
  mirror)
    root="${2:?mirror directory required}"
    python3 "$here/../mkmirror-copy.py" "$root" ${3:+--sync}
    python3 "$here/apply.py" --root "$root"
    python3 "$here/apply.py" --root "$root" | grep -v '^unchanged' && { echo "apply.py is not idempotent"; exit 1; } || true
    ;;
  live) root="/Users/ueli/Documents/semio" ;;
  *) echo "usage: check.sh mirror <dir> | live" >&2; exit 2 ;;
esac
cd "$root"
bun test "./$tests/📐️declared-inputs/🟦️.ts" "./$tests/🔗️workspace-closure/🟦️.ts"
bun test "./$tests/🎓️course-inputs/🟦️.ts"
bun test "./$tests/🔌️providers/🟦️.ts" "./$tests/🧮️covered-inputs/🟦️.ts"
