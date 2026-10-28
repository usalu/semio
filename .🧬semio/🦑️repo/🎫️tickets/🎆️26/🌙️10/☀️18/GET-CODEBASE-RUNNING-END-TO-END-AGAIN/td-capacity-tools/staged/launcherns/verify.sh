#!/bin/sh
H=$(cd "$(dirname "$0")" && pwd)
# usage: verify.sh <root>  (root = a tree holding the batch applied; default: walk up to the directory holding 🧰️framework)
R=${1:-$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)}
cd -P "$R" || exit 1
status=0
sh "$H/../../suite-in-root.sh" "$R" compiler-capacity-inheritance || status=1
exit $status
