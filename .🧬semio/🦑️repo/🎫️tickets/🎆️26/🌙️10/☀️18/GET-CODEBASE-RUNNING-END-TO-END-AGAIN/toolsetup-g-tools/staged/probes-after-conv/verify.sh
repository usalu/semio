#!/bin/sh
# Laws of the `probes` batch (graph-free, single-file tests, run AFTER apply): the witness laws incl. the new host-probe law, the plan-reuse oracle that plans with the real planner functions.
# Real-run proof (one official course:build twice): the first prints no "[bazel] this plan cannot be reused later: ... sysctl/hdiutil/mount/ps" and the second prints "[bazel] plan reused".
R=$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)
cd "$R" || exit 1
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
status=0
for file in "$B/🔭️witness/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts"; do
  out=$(bun test --timeout 60000 "./$file" 2>&1); echo "$out" | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)"
  echo "$out" | grep -qE "^ *0 fail" || status=1
done
exit $status
