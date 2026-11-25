#!/bin/sh
# 🪞️ Builds a private mirror of the staffage sources in a scratch folder (outside the worktree) and applies patches of this folder to it, so a cure is measured on a copy and nothing in S changes.
# The patches are applied with `git apply` in the scratch folder, which is no repository (the host has no `patch`).
# usage: mirror.sh <scratch-dir> <name> [patch ...]   -> prints the staffage root of the mirror
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
R=$(cd "$HERE/../../../../../../../../.." && pwd)
REL="🎓️teaching/🏛️architecture/🧍️staffage"
S="$R/$REL"
OUT="$1/$2"
shift 2
rm -rf "$OUT"
M="$OUT/$REL"
mkdir -p "$M"
for d in 🔨️modules 🧬️schema 🧪️tests 🧫️fixtures 📦️packages 🔮️oracles; do cp -R "$S/$d" "$M/$d"; done
cp "$S/README.md" "$M/README.md"
ln -s "$S/🗂️catalog" "$M/🗂️catalog"
ln -s "$R/🧰️framework" "$OUT/🧰️framework"
ln -s "$R/🎓️teaching/node_modules" "$OUT/🎓️teaching/node_modules"
for f in package.json bun.lock; do [ -e "$R/🎓️teaching/$f" ] && ln -s "$R/🎓️teaching/$f" "$OUT/🎓️teaching/$f"; done
for f in package.json tsconfig.json bunfig.toml; do [ -e "$R/$f" ] && ln -s "$R/$f" "$OUT/$f"; done
[ -e "$R/node_modules" ] && ln -s "$R/node_modules" "$OUT/node_modules"
for p in "$@"; do (cd "$OUT" && git apply -p1 "$HERE/$p" >&2); done
printf '%s\n' "$M"
