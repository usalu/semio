#!/bin/sh
# 🧪 [nxdeadline] one test file in mirror <name> with the red-g chain envelope (as mrun.sh chain) keeping the FULL log: chain1.sh <name> <repo-relative test file> <log> [bun test args]
n="$1"; f="$2"; log="$3"; shift 3; C="$(cd "$(dirname "$0")/.." && pwd)"
R="${TMPDIR%/}/sm/$n"; T="$R/.red-g-run"; mkdir -p "$T/art"
sed "s#/Users/ueli/Documents/semio#$R#g; s#\"\.\./\.\./\.\./\.\./\.\./\.\./\.\./\.\./🧰️#\"$R/🧰️#g" "$C/chain-g-tools/wire.ts" > "$T/wire.ts"
cd "$R" || exit 1; wire=$(bun "$T/wire.ts" 2>/dev/null)
SEMIO_TEST_ARTIFACT_DIR="$T/art" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" bun test "./$f" "$@" > "$log" 2>&1
printf '%s: %s\n' "$f" "$(/usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' "$log" | tr -s ' \n' ' ')"
