#!/bin/sh
# 🦀 Laws of batch `consumption`: sh verify.sh <mirror root>  (official envelope of a library build grant without artifacts, as its laws require, via td-misc-tools/envrun.sh; base in a mirror/fresh clone: ENOENT + cascade, after: 24/0; needs the repository's Cargo manifests in the mirror, see td-misc-tools/extra-only.txt)
set -e
here=$(cd "$(dirname "$0")" && pwd); root=$(cd "${1:?root}" && pwd -P)
python3 -I "$here/stage.py" check --root "$root"
case "$root" in */sm/*) m=$(basename "$root");; *) echo "verify.sh runs the envelope inside a mirror: pass a mirror root"; exit 2;; esac
NOART=1 sh "$here/../../envrun.sh" "$m" repo-graph-maintenance:test "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/📥️consumption/🟦️.ts" | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$'
