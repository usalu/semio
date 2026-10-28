#!/bin/sh
# 🧪 [nxdeadline] one single-file bun test run from <root> inside mirror <name>: run1.sh <name> <root> <repo-relative test file> <out-log> [bun test args]
n="$1"; R="$2"; f="$3"; log="$4"; shift 4
C="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$(dirname "$log")"
SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:-$R/.cache/test-sandbox/nd}"; export SEMIO_TEST_ARTIFACT_DIR; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
python3 -I "$C/mirror-tools/mirror.py" run "$n" --peers-ok --seal-allow ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/nxdeadline" -- sh -c 'cd "$0" && exec nice -n 10 bun test --timeout 600000 "./$1" "$@"' "$R" "$f" "$@" > "$log" 2>&1
echo "exit $?"; printf '%s: %s\n' "$f" "$(/usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' "$log" | tr -s ' \n' ' ')"
