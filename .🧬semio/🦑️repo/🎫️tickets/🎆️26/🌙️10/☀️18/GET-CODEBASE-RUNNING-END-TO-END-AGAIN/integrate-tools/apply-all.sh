#!/usr/bin/env bash
# 🚦 usage: apply-all.sh [--yes] [--group A|B|C|all] [--undo A|B|C] [--root DIR] [--force-while-running]   (dry run by default; refuses on drift; see apply-all.ts)
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec bun "$here/apply-all.ts" "$@"
