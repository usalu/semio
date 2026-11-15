#!/usr/bin/env bash
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
tag=$1; pkg=$2; shift 2
cd "$R/✏️s/🔌️plugins/📕️norm" || exit 1
repo cargo --store=shared --configuration=prototype -- test -p "$pkg" --lib --no-fail-fast -- "$@" > "$T/🗑️generated/normtest-a-$tag.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/normtest-a-$tag.log"
