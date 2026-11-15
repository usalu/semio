#!/usr/bin/env bash
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
tag=$1; shift
cd "$R/✏️s/🔌️plugins/📕️norm" || exit 1
repo cargo --store=shared --configuration=prototype -- test "$@" --no-fail-fast > "$T/🗑️generated/normtest-b-$tag.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/normtest-b-$tag.log"
