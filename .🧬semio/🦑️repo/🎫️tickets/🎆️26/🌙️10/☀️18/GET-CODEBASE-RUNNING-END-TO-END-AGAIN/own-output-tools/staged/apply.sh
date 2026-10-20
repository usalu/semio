#!/bin/sh
# usage: apply.sh [--root DIR] [--yes] [--undo] [--ignore-running]   dry run by default; --yes writes (refuses while a dependency sync or graph construction runs)
exec bun "$(cd "$(dirname "$0")" && pwd)/apply.ts" "$@"
