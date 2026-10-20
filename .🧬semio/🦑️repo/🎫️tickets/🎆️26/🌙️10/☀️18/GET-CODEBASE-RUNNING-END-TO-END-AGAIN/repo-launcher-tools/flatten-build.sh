#!/usr/bin/env bash
# Builds the launcher the way the install script does: member directories flattened into one package. Usage: flatten-build.sh <package-dir> <work-dir> <out-binary> [GOOS GOARCH]
set -euo pipefail
module="$1"; work="$2"; out="$3"
rm -rf "$work"; mkdir -p "$work/launcher" "$work/cmd"
printf 'module github.com/usalu/semio/repo/launcher\n\ngo 1.25\n' > "$work/go.mod"
i=0
for f in "$module/🐹️.go" "$module"/*/🐹️.go; do
  [ "$(basename "$(dirname "$f")")" = "📦️main" ] && continue
  i=$((i+1)); cp "$f" "$work/s$i.go"
done
cp "$module/📦️main/🐹️.go" "$work/cmd/main.go"
cd "$work"
GOWORK=off CGO_ENABLED=0 ${5:+GOOS=$4 GOARCH=$5} go build -trimpath -o "$out" ./cmd
