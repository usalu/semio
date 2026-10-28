#!/bin/sh
# 🧪 [isolate] runs one test file inside mirror <name> (root $SEMIO_MIRROR_HOME/<name>, default /private/tmp/m) in mode plain|env (ISOLATE_SHORT_ART=1: plain artifacts below <mirror>/.cache/test-sandbox/isa for path-hungry graph suites), full log to 🗑️generated/isolate/log/<tag>.log, prints the summary, failing names and the first error lines. usage: run.sh <name> <mode> <rel> <tag> [bun test args]
name="$1"; mode="$2"; f="$3"; tag="$4"; shift 4
C="$(cd "$(dirname "$0")/.." && pwd)"; R="${SEMIO_MIRROR_HOME:-/private/tmp/m}/$name"; G="$C/🗑️generated/isolate"; mkdir -p "$G/log" "$G/art"
T="$R/.red-g-run"; mkdir -p "$T"
sed "s#/Users/ueli/Documents/semio#$R#g" "$C/ver-f-tools/proc-run.ts" > "$T/proc-run.ts"
log="$G/log/$tag.log"; cd "$R" || exit 1
case "$mode" in
 env) VF_WALL=600 nice -n 5 bun "$T/proc-run.ts" file "$R/$f" --timeout 300000 "$@" > "$log" 2>&1;;
 *) A="$G/art/$tag"; [ -n "$ISOLATE_SHORT_ART" ] && A="$R/.cache/test-sandbox/isa/$tag"; rm -rf "$A"; mkdir -p "$A"; SEMIO_TEST_ARTIFACT_DIR="$A" nice -n 5 bun test "./$f" --timeout 300000 "$@" > "$log" 2>&1;;
esac
printf '%s %s %s: %s\n' "$name" "$mode" "$tag" "$(/usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' "$log" | tr -s ' \n' ' ')"
/usr/bin/grep -E '^\(fail\)' "$log" | cut -c1-200
/usr/bin/grep -E '^(error|Error|  error):' "$log" | sort | uniq -c | sort -rn | head -6 | cut -c1-400
