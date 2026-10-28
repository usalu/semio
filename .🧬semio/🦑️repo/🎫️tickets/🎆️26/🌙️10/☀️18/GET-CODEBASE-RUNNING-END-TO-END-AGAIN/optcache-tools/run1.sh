#!/bin/sh
# 🧪 [optcache] one bun test run inside a mirror root: usage run1.sh <root> <repo-relative test file> <out-log> [bun test args]
R="$1"; f="$2"; log="$3"; shift 3
mkdir -p "$(dirname "$log")"
: "${SEMIO_TEST_ARTIFACT_DIR:=$R/.cache/test-sandbox/oc}"; export SEMIO_TEST_ARTIFACT_DIR; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
cd "$R" || exit 2
nice -n 10 "${BUN:-bun}" test --timeout 170000 "./$f" "$@" > "$log" 2>&1
printf '%s: %s\n' "$f" "$(/usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' "$log" | tr -s ' \n' ' ')"
/usr/bin/grep -E '^\((fail|pass)\)' "$log" | cut -c1-260
