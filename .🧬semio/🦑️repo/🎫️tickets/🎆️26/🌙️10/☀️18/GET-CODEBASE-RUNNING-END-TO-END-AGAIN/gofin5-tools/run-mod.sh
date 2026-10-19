#!/usr/bin/env bash
# usage: run-mod.sh <slug> <module dir relative to the repo root> [go test args...] -- runs one Go module in the copy with the canonical overlay
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/gofin5
REAL=/Users/ueli/Documents/semio
C="$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/gofin5-logs"; COPY="$S/repo-copy"
slug="$1"; rel="$2"; shift 2
mkdir -p "$G"
[ -n "$NOSYNC" ] || bash "$C/gofin5-tools/sync.sh" || exit 4
mod="$COPY/$rel"; cd "$mod" || exit 2
export GOWORK="$COPY/go.work"; unset GOFLAGS
python3 "$C/gocanon/plan.py" "$mod" "$G/$slug-overlay.json" > "$G/$slug-packages.txt" 2> "$G/$slug-plan.err" || { cat "$G/$slug-plan.err"; exit 3; }
pk=($(cat "$G/$slug-packages.txt"))
go test -overlay="$G/$slug-overlay.json" -count=1 -v -timeout "${TMO:-25m}" "$@" "${pk[@]}" > "$G/$slug-test.log" 2>&1
echo "exit=$?" >> "$G/$slug-test.log"
/usr/bin/grep -E '^\s*--- (PASS|FAIL|SKIP):' "$G/$slug-test.log" | sed -E 's/^\s*--- (PASS|FAIL|SKIP): ([^ ]+).*/\2 \1/' | sort > "$G/$slug-results.txt"
echo "$slug: pass=$(/usr/bin/grep -c -- '--- PASS' "$G/$slug-test.log") fail=$(/usr/bin/grep -c -- '--- FAIL' "$G/$slug-test.log") skip=$(/usr/bin/grep -c -- '--- SKIP' "$G/$slug-test.log") $(tail -1 "$G/$slug-test.log")"
