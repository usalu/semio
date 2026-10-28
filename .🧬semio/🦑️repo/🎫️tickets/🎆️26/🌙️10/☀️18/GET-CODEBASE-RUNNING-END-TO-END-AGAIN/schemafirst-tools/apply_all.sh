#!/bin/sh
# 🏭️ Applies (or reverts, in reverse order) every schemafirst batch to a root: sh apply_all.sh <root> [apply|revert]
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=$1
verb=${2:-apply}
list=$(cat "$here/order.txt")
[ "$verb" = revert ] && list=$(sed '1!G;h;$!d' "$here/order.txt")
for b in $list; do python3 -I "$here/staged/$b/stage.py" "$verb" --root "$root" | tail -1; done
