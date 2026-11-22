#!/usr/bin/env bash
# 🔁️ One PW-04 round: rebuild the given trees (default all three) and run the full validation of each; logs to 🗑️generated/PW-04/<id>.validate.txt.
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
T3="$HERE/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CAT="$HERE/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation"
OUT="$HERE/../../🗑️generated/PW-04"
IDS=("$@")
[ ${#IDS[@]} -eq 0 ] && IDS=(ginkgo honey-locust sweetgum)
mkdir -p "$OUT"
[ -z "${SKIP_BUILD:-}" ] && cd "$HERE" && repo tools exec bun -- build.ts "${IDS[@]}" 2>&1 | grep -v '^\['
for id in "${IDS[@]}"; do
  (cd "$T3" && repo tools exec bun -- r3-validate.ts --file "$CAT/$id/🔣️.json" --volumes ${EXTRA:-} > "$OUT/$id.validate.txt" 2>&1)
  echo "== $id"; grep -v '^\[' "$OUT/$id.validate.txt" | head -40
done
