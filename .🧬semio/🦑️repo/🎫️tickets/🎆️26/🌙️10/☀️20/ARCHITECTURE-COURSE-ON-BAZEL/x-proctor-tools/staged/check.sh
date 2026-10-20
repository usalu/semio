#!/usr/bin/env bash
# Verifies the staged x-proctor change on a scratch copy of the three files it edits; never touches the checkout.
# Usage: bash check.sh [--live]   (--live: also assert the checkout itself is in the target state, i.e. apply.py --check passes)
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/x-proctor-check.XXXXXX")"
trap 'rm -rf "$SCRATCH"' EXIT
FILES=(
  "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json"
  "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json"
  "🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts"
)
for file in "${FILES[@]}"; do mkdir -p "$SCRATCH/$(dirname "$file")"; cp "$ROOT/$file" "$SCRATCH/$file"; done
ln -s "$ROOT/🧰️framework" "$SCRATCH/🧰️framework"

echo "== first apply"; python3 "$HERE/apply.py" --root "$SCRATCH"
cp -R "$SCRATCH/🎓️teaching" "$SCRATCH/first"
echo "== second apply (must change nothing)"; python3 "$HERE/apply.py" --root "$SCRATCH" | tee "$SCRATCH/second.txt"
grep -q "nothing (already applied)" "$SCRATCH/second.txt"
for file in "${FILES[@]}"; do cmp "$SCRATCH/$file" "$SCRATCH/first/${file#🎓️teaching/}"; done
echo "== --check on the applied copy"; python3 "$HERE/apply.py" --root "$SCRATCH" --check
echo "== the manifests parse"; for file in "${FILES[@]:0:2}"; do python3 -c "import json,sys; json.load(open(sys.argv[1],encoding='utf8'))" "$SCRATCH/$file"; done
echo "== the manifest laws against the applied copy"
(cd "$ROOT" && PROCTOR_MANIFEST="$SCRATCH/${FILES[0]}" bun test "./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧾️proctor-product/🟦️.ts")
if [ "${1:-}" = "--live" ]; then echo "== live checkout"; python3 "$HERE/apply.py" --check; fi
echo "check.sh: OK"
