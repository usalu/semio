#!/usr/bin/env bash
# Runs the launcher cases the way the canonical planner maps them: member directories and test cases flattened into one package. Usage: flatten-test.sh <module-dir> <work-dir> <overlay-root> [go test args]
set -euo pipefail
module="$1"; work="$2"; root="$3"; shift 3
rm -rf "$work"; mkdir -p "$work"
printf 'module github.com/usalu/semio/repo/launcher\n\ngo 1.25\n' > "$work/go.mod"
i=0
for f in "$module/📦️packages/🐹️go/🐹️.go" "$module"/📦️packages/🐹️go/*/🐹️.go; do
  [ "$(basename "$(dirname "$f")")" = "📦️main" ] && continue
  i=$((i+1)); cp "$f" "$work/s$i.go"
done
for f in "$module"/🧪️tests/*/🐹️.go; do i=$((i+1)); cp "$f" "$work/s${i}_test.go"; done
cd "$work"
SEMIO_WORKSPACE_ROOT="$root" GOWORK=off CGO_ENABLED=0 go test -count=1 "$@" ./...
