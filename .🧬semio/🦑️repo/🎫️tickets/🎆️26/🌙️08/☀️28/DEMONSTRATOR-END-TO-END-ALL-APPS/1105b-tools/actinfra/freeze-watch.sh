#!/bin/bash
# 🧊️ Read-only: classifies every working-tree edit newer than a seal stamp by what it costs a running activation.
# Usage: freeze-watch.sh [stamp-file]   default stamp: 🗑️generated/1105b/freeze (else sync-done-1107)
# Classes: A receipt (manifest/lock/patch: dependency receipt stale = ~45 min verify + restart; needs lock+sync)
#          B closure (framework + os plugin runtime: stales ~1000 graph projects and every component action)
#          C repo (repo product implementation: loaded by the running command)
#          D leaf (one plugin/extension/hub crate: one project refresh ~100 s + its own component action)
# Exit 0 = no A/B/C edit (leaf edits only or none), 1 = A, B or C edit present. No repo run, no writes.
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$HERE"
while [ ! -f "$ROOT/AGENTS.md" ] && [ "$ROOT" != "/" ]; do ROOT="$(dirname "$ROOT")"; done
[ -f "$ROOT/AGENTS.md" ] || { echo "FAIL repo root not found"; exit 2; }
G="$(cd "$HERE/../.." && pwd)/🗑️generated/1105b"
cd "$ROOT" || exit 2
stamp="${1:-$G/freeze}"; [ -e "$stamp" ] || stamp="$G/sync-done-1107"
[ -e "$stamp" ] || { echo "FAIL no stamp file ($stamp)"; exit 2; }
echo "stamp $(basename "$stamp") $(date -r "$(stat -f %m "$stamp")" '+%m-%d %H:%M')"
{ git ls-files -m -z -- ':!.🧬semio' ':!🎓️teaching'; git ls-files -o --exclude-standard -z -- '*.rs' '*.ts' '*.tsx' '*.mjs' '*.wit' '*.json' '*.toml' '*.patch' '*.lock' ':!.🧬semio' ':!🎓️teaching'; } | sort -zu | while IFS= read -r -d '' f; do
  [ -f "$f" ] && [ "$f" -nt "$stamp" ] || continue
  case "$f" in
    */Cargo.toml|*/Cargo.lock|*/package.json|*/bun.lock|*/bun.lockb|*.patch|*/rust-toolchain.toml) c=A;;
    🧰️framework/🛍️products/🦑️repo/*) c=C;;
    🧰️framework/*) c=B;;
    *) c=D;;
  esac
  printf '%s\t%s\n' "$c" "$f"
done > "${TMPDIR:-/tmp}/freeze-watch.$$"
describe() {
  case "$1" in
    A) echo "receipt: lock+sync, about 45 min and a restart";;
    B) echo "closure: stales about 1000 projects and all component actions";;
    C) echo "repo implementation";;
    D) echo "leaf: one project refresh";;
  esac
}
status=0
for c in A B C D; do
  n=$(/usr/bin/grep -c "^$c	" "${TMPDIR:-/tmp}/freeze-watch.$$")
  label=$(describe "$c")
  echo "$c $n $label"
  /usr/bin/grep "^$c	" "${TMPDIR:-/tmp}/freeze-watch.$$" | head -5 | cut -f2 | sed 's/^/    /'
  [ "$c" != D ] && [ "$n" -gt 0 ] && status=1
done
rm -f "${TMPDIR:-/tmp}/freeze-watch.$$"
exit $status
