#!/bin/sh
H=$(cd "$(dirname "$0")" && pwd)
# usage: verify.sh <root>  (root = a tree holding the batch applied; default: walk up to the directory holding 🧰️framework)
R=${1:-$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)}
cd -P "$R" || exit 1
status=0
out=$(bun test --timeout 300000 "./🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🚪️gate/🟦️.ts" 2>&1); echo "dules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🚪️gate/🟦️.ts: $(echo "$out" | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr -s " \n" " ")"; echo "$out" | /usr/bin/grep -E "^\(fail\)" | cut -c1-200; echo "$out" | /usr/bin/grep -qE "^ *0 fail" || status=1
if [ "$(uname)" = Darwin ]; then
  out=$(sandbox-exec -p '(version 1)(allow default)' bun test --timeout 300000 "./🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🚪️gate/🟦️.ts" 2>&1); echo "sandboxed (ps refused): $(echo "$out" | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr -s " \n" " ")"; echo "$out" | /usr/bin/grep -E "^\(fail\)" | cut -c1-200
  echo "$out" | /usr/bin/grep -qE "^ *0 fail" || status=1
fi
exit $status
