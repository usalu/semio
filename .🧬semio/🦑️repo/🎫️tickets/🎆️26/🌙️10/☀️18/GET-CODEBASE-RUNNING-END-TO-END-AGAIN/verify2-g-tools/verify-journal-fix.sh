#!/bin/sh
# Laws of `journal-fix` (graph-free single-file tests, run AFTER apply): journal service suite incl. the new burst / vanished-workspace laws, and the plan-reuse decision cycle which must start no journal service.
R=$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)
cd "$R" || exit 1
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
status=0
before=$(ps -axo pid,command | grep -c "[r]epo-watch.*reuse-")
out=$(bun test --timeout 120000 "./$B/🏡️resident/📜️journal/🧪️tests/🟦️.ts" 2>&1); echo "$out" | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)"; echo "$out" | grep -qE "^ *0 fail" || status=1
out=$(bun test --timeout 300000 -t "decision cycle" "./$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts" 2>&1); echo "$out" | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)"; echo "$out" | grep -qE "^ *0 fail" || status=1
sleep 2
after=$(ps -axo pid,command | grep -c "[r]epo-watch.*reuse-")
echo "reuse-* journal watchers before=$before after=$after"; [ "$after" -le "$before" ] || status=1
exit $status
