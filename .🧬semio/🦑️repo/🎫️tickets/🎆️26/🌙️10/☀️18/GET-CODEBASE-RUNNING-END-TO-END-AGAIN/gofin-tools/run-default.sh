#!/usr/bin/env bash
# usage: run-default.sh <moduleRelDir> <slug> [go test args...] -- default-tag run inside the throwaway repo copy
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/gofin-logs"; COPY="$S/repo-copy"
rel="$1"; slug="$2"; shift 2
mod="$COPY/$rel"; cd "$mod" || exit 2
export GOWORK="$COPY/go.work"; unset GOFLAGS
python3 "$C/gocanon/plan.py" "$mod" "$G/$slug-overlay.json" > "$G/$slug-packages.txt" 2> "$G/$slug-plan.err" || { cat "$G/$slug-plan.err"; exit 3; }
pk=($(cat "$G/$slug-packages.txt"))
go test -overlay="$G/$slug-overlay.json" -count=1 -v -timeout 20m "$@" "${pk[@]}" > "$G/$slug-def.log" 2>&1
echo "exit=$?" >> "$G/$slug-def.log"
echo "$slug: pass=$(/usr/bin/grep -c -- '--- PASS' "$G/$slug-def.log") fail=$(/usr/bin/grep -c -- '--- FAIL' "$G/$slug-def.log") skip=$(/usr/bin/grep -c -- '--- SKIP' "$G/$slug-def.log") $(tail -1 "$G/$slug-def.log")"
