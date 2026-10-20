#!/usr/bin/env bash
# usage: apply.sh [--dry-run] [--rebase]
#   Applies the uncached-owners set: first the library edits (the schema must admit the four new reasons before any manifest records one), then the manifest operations.
#   Order of the series: C/cache-fix-tools/apply.sh, then this script. Idempotent; every manifest is re-read right before its write.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export SEMIO_ROOT="${SEMIO_ROOT:-$(git -C "$here" rev-parse --show-toplevel)}"
schema="$SEMIO_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/💾️cache.json"
if ! grep -q '"Unclassified"' "$schema"; then echo "the cache-fix set is not applied yet: run C/cache-fix-tools/apply.sh first" >&2; exit 3; fi
bun "$here/apply-library.ts" "$@"
bun "$here/apply-manifests.ts" "$@"
