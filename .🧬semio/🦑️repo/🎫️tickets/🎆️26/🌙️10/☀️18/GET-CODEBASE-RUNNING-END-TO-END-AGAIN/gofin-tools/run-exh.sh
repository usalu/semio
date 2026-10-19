#!/usr/bin/env bash
# usage: run-exh.sh <moduleRelDir> <slug> [extra go test args...]  -- runs inside the throwaway repo copy only
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/gofin-logs"
COPY="$S/repo-copy"
rel="$1"; slug="$2"; shift 2
mod="$COPY/$rel"
cd "$mod" || exit 2
export GOWORK="$COPY/go.work"
unset GOFLAGS
python3 "$C/gocanon/plan.py" "$mod" "$G/$slug-overlay.json" > "$G/$slug-packages.txt" 2> "$G/$slug-plan.err" || { cat "$G/$slug-plan.err"; exit 3; }
pk=($(cat "$G/$slug-packages.txt"))
go test -overlay="$G/$slug-overlay.json" -tags exhaustive -count=1 -v -timeout 25m "$@" "${pk[@]}" > "$G/$slug-exh.log" 2>&1
echo "exit=$?" >> "$G/$slug-exh.log"
/usr/bin/grep -E '^\s*--- (PASS|FAIL|SKIP)|^(ok|FAIL)\s|^exit=' "$G/$slug-exh.log" | sed -E 's/ \([0-9.]+s\)//' | sort | uniq -c | sort -rn | head -80
