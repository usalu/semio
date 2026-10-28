#!/bin/sh
H=$(cd "$(dirname "$0")" && pwd)
# usage: verify.sh <root>  (root = a tree holding the batch applied; default: walk up to the directory holding 🧰️framework)
R=${1:-$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)}
cd -P "$R" || exit 1
status=0
# the two files scan every framework source through `git ls-files`: run inside a git work tree (a mirror needs `git init`); the Bun default of 5 s per test is what failed.
out=$(bun test --timeout 300000 "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/⛓️cpu-last/🟦️.ts" 2>&1); echo "🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/⛓️cpu-last/🟦️.ts: $(echo "$out" | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr -s " \n" " ")"; echo "$out" | /usr/bin/grep -E "^\(fail\)" | cut -c1-200; echo "$out" | /usr/bin/grep -qE "^ *0 fail" || status=1
out=$(bun test --timeout 300000 "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🔒️exclusive-writer/🟦️.ts" 2>&1); echo "ement/🏗️compiler/📏️capacity/🧪️tests/🔒️exclusive-writer/🟦️.ts: $(echo "$out" | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr -s " \n" " ")"; echo "$out" | /usr/bin/grep -E "^\(fail\)" | cut -c1-200; echo "$out" | /usr/bin/grep -qE "^ *0 fail" || status=1
exit $status
