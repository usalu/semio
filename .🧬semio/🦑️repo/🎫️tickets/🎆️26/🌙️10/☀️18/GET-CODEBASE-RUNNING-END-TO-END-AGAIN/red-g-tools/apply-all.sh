#!/bin/sh
# usage: apply-all.sh <root> [batch...]  applies the red-g batches (default order) to a root (mirror)
S="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/red-g-tools/stage.py"
root="$1"; shift; [ $# -eq 0 ] && set -- tests tests2 runtimelaw
for b in "$@"; do python3 -I "$S" apply "$b" --root "$root" || exit 1; done
