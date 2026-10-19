#!/usr/bin/env bash
# usage: vet-mod.sh <slug> <module dir relative to the repo root> -- go vet of one module with the canonical overlay (copy already synced)
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
go vet -overlay="$G/$slug-overlay.json" "$@" "${pk[@]}" 2>&1 | head -${HEADN:-60}
