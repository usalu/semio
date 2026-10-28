#!/bin/sh
# 🏭️ Rebuilds every schemafirst batch from the files of a root (default: the live root) and prints each batch's `requires`: sh rebuild_all.sh [root]
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
export SF_SRC=$root
for b in $(cat "$here/order.txt"); do python3 "$here/batches/$b.py" || exit 1; done
