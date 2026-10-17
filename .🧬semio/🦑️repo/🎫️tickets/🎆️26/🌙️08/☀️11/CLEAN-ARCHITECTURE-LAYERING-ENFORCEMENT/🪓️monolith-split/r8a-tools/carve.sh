#!/bin/zsh
# usage: carve.sh <5d|2d> <verify|apply|check> : runs the per-package carve step (niced, logs F/🗑️generated/r8a-<step><pkg>.log/.exit)
cd /Users/ueli/Documents/semio || exit 99
FD=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
case $1 in 5d) PKG='✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust';; 2d) PKG='✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust';; esac
FLAGS=(--no-test-aggregate --mutation-families=on)
case $2 in
  verify) rm -rf /Users/ueli/.cache/r8a/v$1; zsh $FD/r8a-tools/step.sh verify$1 bun ./📜️script.ts new crates "$PKG" $FLAGS --verify --oracle --sandbox=/Users/ueli/.cache/r8a/v$1;;
  apply) zsh $FD/r8a-tools/step.sh apply$1 bun ./📜️script.ts new crates "$PKG" $FLAGS --backup=$FD/🗑️generated/r8a-backup-$1;;
  check) rm -rf /Users/ueli/.cache/r8a/ca$1; zsh $FD/r8a-tools/step.sh check$1 bun ./📜️script.ts new crates "$PKG" --check-applied --base=${3:-HEAD} --samples=500 --sandbox=/Users/ueli/.cache/r8a/ca$1;;
esac
