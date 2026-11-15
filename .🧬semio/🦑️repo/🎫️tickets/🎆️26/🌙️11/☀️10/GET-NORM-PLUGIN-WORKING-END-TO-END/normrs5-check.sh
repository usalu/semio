#!/usr/bin/env bash
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
cd "$R/✏️s/🔌️plugins/📕️norm" || exit 1
repo cargo --store=shared --configuration=prototype -- check --all-targets --keep-going > "$T/🗑️generated/normrs5-check9.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/normrs5-check9.log"
