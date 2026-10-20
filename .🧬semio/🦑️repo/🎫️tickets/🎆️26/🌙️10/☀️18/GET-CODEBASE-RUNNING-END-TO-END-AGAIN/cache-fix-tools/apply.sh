#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export SEMIO_ROOT="${SEMIO_ROOT:-$(git -C "$here" rev-parse --show-toplevel)}"
bun "$here/apply-library.ts" "$@"
bun "$here/apply-manifests.ts" "$@"
