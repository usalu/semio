#!/bin/sh
# Loads the CLI of a tree through its help paths with the pinned runtime (no capacity, no graph, no cache mount): a ReferenceError or a missing import in any command module shows here. usage: smoke-ov.sh <root>
ROOT="$1"
BUN=$(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
S="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts"
cd "$ROOT" || exit 2
bad=0
for args in "--help" "run --help --tool=nx" "run --help --tool=bazel" "leases --help" "cache status --help" "cache verify --help" "tools --help" "tools list --help" "show --help" "preflight --help" "units --help" "bench --help" "service --help" "remote-cache --help" "remote-execution --help" "capacity --help" "doctor --help" "dependencies --help" "incremental --help" "concurrency --help" "graph --help" "exec --help"; do
  out=$(SEMIO_REPOSITORY_ROOT="$ROOT" nice -n 10 "$BUN" "$S" repo $args 2>&1); rc=$?
  if [ $rc -ne 0 ] || printf '%s' "$out" | /usr/bin/grep -q "ReferenceError\|SyntaxError\|Cannot find module\|is not defined"; then
    bad=$((bad+1)); echo "SMOKE FAIL rc=$rc: repo $args"; printf '%s\n' "$out" | /usr/bin/grep -m3 "Error\|Cannot find" | cut -c1-260
  else echo "ok: repo $args"; fi
done
echo "smoke: $bad failure(s)"
exit $bad
