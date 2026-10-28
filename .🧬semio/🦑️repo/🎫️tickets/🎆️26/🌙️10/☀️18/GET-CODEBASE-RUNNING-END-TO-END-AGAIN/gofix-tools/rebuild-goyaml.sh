#!/bin/sh
# 🧾 Rebases batch `goyaml` onto the live build-owner registry (a hot root file): sh rebuild-goyaml.sh [mirror-root]   (default $TMPDIR/sm/gofix)
set -eu
S=$(cd "$(dirname "$0")" && pwd)
C=$(dirname "$S")
LIVE=${C%%/.🧬semio/*}
R=${1:-${TMPDIR%/}/sm/gofix}
Y="🧰️framework/🛍️products/🦑️repo/🔨️modules/🧾️yaml/📦️packages/🐹️go"
cp "$LIVE/📋️build-owners.json" "$R/📋️build-owners.json"
(cd "$R" && GOFIX_WRITE=1 bun "$S/reconcile.ts" "$R" "$Y" >/dev/null)
python3 -I "$S/make-batch.py" goyaml '🧾 Staged batch `goyaml` (lane gofix): the Go module of the YAML codec gets the project every other Go module has (`@semio-tech/repo-yaml-go:test` runs its 9 cases on the official route) and its row in the build-owner registry.' "$R" "$LIVE" "$Y/📋️project.json" "$Y/📜️script.ts" '📋️build-owners.json'
