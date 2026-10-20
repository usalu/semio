#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export SEMIO_ROOT="${SEMIO_ROOT:-$(git -C "$here" rev-parse --show-toplevel)}"
exec bun "$here/apply.ts" "$@"
