#!/bin/sh
# usage: mrun.sh <mirror-name> <mode: env|plain|chain> <repo-relative test file> [bun test args]  -> runs the file INSIDE the mirror (sealed), summary + failing names
name="$1"; mode="$2"; f="$3"; shift 3
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
R="${TMPDIR%/}/sm/$name"; [ -d "$R" ] || R="$(python3 -I "$C/mirror-tools/mirror.py" list | awk -v n="$name" '$1==n{print $NF}')"
T="$R/.red-g-run"; mkdir -p "$T"
sed "s#/Users/ueli/Documents/semio#$R#g" "$C/ver-f-tools/proc-run.ts" > "$T/proc-run.ts"
sed "s#/Users/ueli/Documents/semio#$R#g; s#\"\.\./\.\./\.\./\.\./\.\./\.\./\.\./\.\./🧰️#\"$R/🧰️#g" "$C/chain-g-tools/wire.ts" > "$T/wire.ts"
cd "$R" || exit 1
case "$mode" in
 env) out=$(VF_WALL=300 bun "$T/proc-run.ts" file "$R/$f" "$@" 2>&1);;
 chain) ART=$T/art; mkdir -p "$ART"; wire=$(bun "$T/wire.ts" 2>/dev/null); out=$(SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" bun test "./$f" "$@" 2>&1);;
 *) out=$(bun test "./$f" "$@" 2>&1);;
esac
printf '%s %s: %s\n' "$name" "$mode" "$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' | tr -s ' \n' ' ')"
echo "$out" | /usr/bin/grep -E '^\(fail\)' | cut -c1-240
echo "$out" | /usr/bin/grep -E '^error:' | head -3 | cut -c1-300
