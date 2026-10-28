#!/bin/sh
# usage: run.sh <label> <scratch inside|outside> <steps> <only> [jobs]   (from anywhere; sealed mirror run, mirror name gofix)
set -eu
S="$(cd "$(dirname "$0")" && pwd)"
C="$(dirname "$S")"
LIVE="${C%%/.🧬semio/*}"
REL="${C#"$LIVE"/}"
G="$C/🗑️generated/gofix"
export GOFIX_GOROOT="$LIVE/.cache/b/t/m/native/go/1.26.9/darwin-arm64-f9bb7c0a02506c5d"
mkdir -p "$G/out" "$G/gocache" "$G/gomod"
exec python3 -I "$C/mirror-tools/mirror.py" run gofix --peers-ok --seal-allow "$REL/🗑️generated/gofix" -- \
  python3 -I "$S/run.py" --mirror "${TMPDIR%/}/sm/gofix" --plan "$G/plan" --out "$G/out" --cache "$G/gocache" \
  --label "$1" --scratch "$2" --steps "$3" --only "$4" --ids "${6:-}" --jobs "${5:-2}" --run "${GOFIX_RUN:-}"
