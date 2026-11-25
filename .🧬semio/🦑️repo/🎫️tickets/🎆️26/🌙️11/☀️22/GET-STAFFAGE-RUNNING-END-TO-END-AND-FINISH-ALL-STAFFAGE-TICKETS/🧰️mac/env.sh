#!/usr/bin/env bash
# 🍎️ Shared paths for the Mac lane scripts: pinned bun of this machine, repo root, staffage root.
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
S="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
OUT="$HERE/../🗑️generated/mac"
