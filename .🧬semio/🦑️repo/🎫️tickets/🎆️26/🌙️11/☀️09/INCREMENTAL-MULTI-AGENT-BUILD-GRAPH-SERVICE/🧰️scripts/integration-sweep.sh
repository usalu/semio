#!/usr/bin/env bash
# Runs every build-library test file alone in light mode; writes "<pass> <fail> <exit> <file>" lines to $1 and one log per file next to it.
#
# Test artifact directory policy (so a sweep needs no workaround):
#   scratch     the kind the suite table declares for the file (`integration-scratch.py`): `checkout` files get the checkout-local base,
#               `sandbox` files a base below the system temporary directory (tests that look for a workspace root or a Git work tree
#               above them need that);
#   short       base .🧬semio/🦑️repo/⚡️cache/tests/ia plus an 8 hex job directory (far below the 256 character path limit);
#   checkout    below the checkout, so Git, Cargo and the checkout-scoped tests see a repository;
#   one kind    exactly one deterministic kind per file, never a retry in the other kind (a failure is a failure);
#   exempt      the base is recreated and its mtime refreshed before every job, so the evidence reclaim (which removes unused or missing
#               evidence below the cache root) never finds it unused or absent while a job runs; a job directory is removed after a pass.
# Environment: JOBS (at most 4, the host budget), LIST (file list instead of discovery), RESUME (keep earlier results), SEMIO_TEST_ARTIFACT_BASE.
if [ -z "$SEMIO_SWEEP_COPY" ]; then copy="$(mktemp)"; cp "$0" "$copy"; SEMIO_SWEEP_COPY=1 SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)" exec bash "$copy" "$@"; fi
cd "$(git rev-parse --show-toplevel)"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
OUT="$1"; [ -n "$RESUME" ] || : > "$OUT"; sed -i '/^DONE$/d' "$OUT"
export ARTIFACT_BASE="${SEMIO_TEST_ARTIFACT_BASE:-$PWD/.🧬semio/🦑️repo/⚡️cache/tests/ia}"; mkdir -p "$(dirname "$OUT")/sweep-logs"
export SCRIPT_DIR OUT LOGS="$(dirname "$OUT")/sweep-logs"
jobs=${JOBS:-4}; [ "$jobs" -gt 4 ] && jobs=4
attempt(){ f="$1"; base="$2"; mkdir -p "$base"; touch "$base"
  export SEMIO_TEST_ARTIFACT_DIR="$base/$(printf '%s' "$f" | md5sum | cut -c1-8)"; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
  r=$(timeout 180 repo tools exec --light bun -- test "./$f" 2>&1); code=$?
  [ "$code" = 0 ] && rm -rf "$SEMIO_TEST_ARTIFACT_DIR"; }
one(){ f="$1"; base="$ARTIFACT_BASE"; [ "$(python3 "$SCRIPT_DIR/integration-scratch.py" "$f")" = sandbox ] && base="${SEMIO_SWEEP_SANDBOX:-${TMPDIR:-/tmp}/semio-ia}"
  attempt "$f" "$base"
  printf '%s\n' "$r" > "$LOGS/$(printf '%s' "$f" | md5sum | cut -c1-12).log"
  p=$(grep -E '^ *[0-9]+ pass' <<<"$r" | awk '{print $1}'); fl=$(grep -E '^ *[0-9]+ fail' <<<"$r" | awk '{print $1}')
  echo "${p:-?} ${fl:-?} $code $f" >> "$OUT"; }
export -f attempt
export -f one
{ if [ -n "$LIST" ]; then cat "$LIST"; else find "$B" -path '*/node_modules' -prune -o -type f -name '🟦️.ts' -path '*🧪️tests*' -print; fi; } | grep -v "🧲️holder\|📥️worker\|🪆️kernel-role\|♻️plan-reuse/🧪️tests/.*fixtures\|📊️bench\|🌊️burst\|🎭️stub\|📜️child" | sort | grep -vxFf <(awk '{ $1=$2=$3=""; sub(/^ +/,""); print }' "$OUT") | xargs -P "$jobs" -I{} bash -c 'one "$1"' _ {}
echo DONE >> "$OUT"
