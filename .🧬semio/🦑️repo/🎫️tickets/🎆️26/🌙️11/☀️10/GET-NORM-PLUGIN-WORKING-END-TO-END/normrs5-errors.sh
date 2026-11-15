#!/usr/bin/env bash
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
grep -E '^error' -A6 "$1" | grep -vE '^\s*\|\s*$' | sed -E 's#'"$R"'/##; s#📦️packages/🦀️rust/\.\./\.\./##' | cut -c1-260 | head -${2:-60}
