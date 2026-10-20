#!/bin/sh
# usage: check.sh [--staged]   (default: the installed test; --staged: the staged copy, before apply.py)
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
cd "$root"
if [ "${1:-}" = "--staged" ]; then file="$here/files/📥️keys/🟦️.ts"; else file="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️keys/🟦️.ts"; fi
exec bun test "./$(python3 -c 'import os,sys;print(os.path.relpath(sys.argv[1],sys.argv[2]))' "$file" "$root")"
