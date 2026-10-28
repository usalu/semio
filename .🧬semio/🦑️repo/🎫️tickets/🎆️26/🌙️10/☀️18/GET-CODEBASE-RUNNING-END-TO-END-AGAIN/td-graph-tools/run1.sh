#!/bin/sh
# 🧪 [td-graph] one single-file bun test run: usage run1.sh <root> <repo-relative test file> <out-log> [bun test args]; cwd = <root> unless TG_CWD=dir (then the test's directory); BUN overrides the runtime; prints the pass/fail summary and failing names; full output in <out-log>
R="$1"; f="$2"; log="$3"; shift 3
C="$(cd "$(dirname "$0")/.." && pwd)"
: "${SEMIO_TEST_ARTIFACT_DIR:=$C/🗑️generated/td-graph/art}"; export SEMIO_TEST_ARTIFACT_DIR
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR" "$(dirname "$log")"
if [ "$TG_CWD" = dir ]; then cd "$R/$(dirname "$f")" || exit 2; t="./$(basename "$f")"; else cd "$R" || exit 2; t="./$f"; fi
nice -n 10 "${BUN:-bun}" test --timeout 170000 "$t" "$@" > "$log" 2>&1
printf '%s: %s\n' "$f" "$(/usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' "$log" | tr -s ' \n' ' ')"
/usr/bin/grep -E '^\(fail\)' "$log" | cut -c1-260
