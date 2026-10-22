#!/bin/sh
# usage: go-run.sh <overlay checkout> [go test args]: layers the overlay's Go module closure into a scratch root, plans the canonical Go overlay and runs the tests of the repo MCP module
set -e
OV="$1"; shift
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/repo-mcp-build-tools"
ROOT=/private/tmp/agent-session/go-root
cd "$OV"
nice -n 10 python3 "$C/overlay-prepare.py" --repo "$OV" --root "$ROOT" --fresh >/dev/null 2>&1
nice -n 10 bun "$C/overlay-plan.ts" "$ROOT" /private/tmp/agent-session/go-overlay.json >/dev/null
[ -d "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧫️fixtures" ] || { mkdir -p "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli" && cp -R "$OV/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧫️fixtures" "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/"; }
cd "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp"
export GOTOOLCHAIN=local GOCACHE=/private/tmp/agent-session/gocache
nice -n 10 go test -short -count=1 -overlay /private/tmp/agent-session/go-overlay.json "$@" .
