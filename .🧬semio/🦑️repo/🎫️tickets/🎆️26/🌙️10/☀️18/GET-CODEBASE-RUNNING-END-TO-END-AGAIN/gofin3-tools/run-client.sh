#!/usr/bin/env bash
# usage: run-client.sh <slug> [go test args...] -- syncs the client module into the throwaway repo copy, runs it with the canonical overlay
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad
REAL=/Users/ueli/Documents/semio
C="$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/gofin3-logs"; COPY="$S/repo-copy"
M="🧰️framework/🛍️products/🦑️repo/🔨️modules"
slug="$1"; shift
mkdir -p "$G"
rsync -a --delete "$REAL/$M/💻️client/" "$COPY/$M/💻️client/" || exit 4
mod="$COPY/$M/💻️client/⌨️cli"; cd "$mod" || exit 2
export GOWORK="$COPY/go.work"; unset GOFLAGS
python3 "$C/gocanon/plan.py" "$mod" "$G/$slug-overlay.json" > "$G/$slug-packages.txt" 2> "$G/$slug-plan.err" || { cat "$G/$slug-plan.err"; exit 3; }
pk=($(cat "$G/$slug-packages.txt"))
go test -overlay="$G/$slug-overlay.json" -count=1 -v -timeout 25m "$@" "${pk[@]}" > "$G/$slug-test.log" 2>&1
echo "exit=$?" >> "$G/$slug-test.log"
echo "$slug: pass=$(/usr/bin/grep -c -- '--- PASS' "$G/$slug-test.log") fail=$(/usr/bin/grep -c -- '--- FAIL' "$G/$slug-test.log") skip=$(/usr/bin/grep -c -- '--- SKIP' "$G/$slug-test.log") $(tail -1 "$G/$slug-test.log")"
