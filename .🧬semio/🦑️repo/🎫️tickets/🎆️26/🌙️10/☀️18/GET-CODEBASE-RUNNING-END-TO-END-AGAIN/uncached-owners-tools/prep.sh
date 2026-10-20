#!/usr/bin/env bash
# usage: prep.sh   regenerates the inputs of the census and of the decision pipeline under C/🗑️generated/uncached-owners (read-only on the repo): tracked manifest list, static graph (a parametrised copy of bzl-coverage-tools/graph.ts), staged coverage classifier, target inventory (scan.ts), handler analysis (analyze.ts, profile-records.ts)
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"; C="$(cd "$here/.." && pwd)"; root="$(git -C "$here" rev-parse --show-toplevel)"; G="$C/🗑️generated/uncached-owners"
mkdir -p "$G/coverage-ov/🧬️schema"
git -C "$root" ls-files '*project.json' | command grep -v '^\.🧬semio/' > "$G/pj-list.txt"
sed -e 's|const TICKET = join(import.meta.dir, "..");|const TICKET = join(import.meta.dir, "../..");|' -e 's|"🗑️generated", "bzl-coverage"|"🗑️generated", "uncached-owners"|' "$C/bzl-coverage-tools/graph.ts" > "$G/graph-run.ts"
bun "$G/graph-run.ts"
cp "$C/bzl-coverage-tools/staged/coverage.ts" "$G/coverage-ov/coverage.ts"
cp "$C/bzl-coverage-tools/staged/coverage-schema.json" "$G/coverage-ov/🧬️schema/🎯️coverage.json"
bun "$here/scan.ts" > /dev/null
bun "$here/mk-targets.ts"
bun "$here/analyze.ts" > /dev/null
bun "$here/profile-records.ts" > /dev/null
echo "prepared: $G"
