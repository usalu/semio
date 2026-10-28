#!/usr/bin/env bash
set -euo pipefail
module="$1/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go"
work="$2"
rm -rf "$work"; mkdir -p "$work/cmd"
printf 'module github.com/usalu/semio/repo/launcher\n\ngo 1.25\n' >"$work/go.mod"
index=0
for source in "$module/🐹️.go" "$module"/*/🐹️.go; do
  [ "$(basename "$(dirname "$source")")" = "📦️main" ] && continue
  index=$((index + 1)); cp "$source" "$work/s$index.go"
done
cp "$module/📦️main/🐹️.go" "$work/cmd/main.go"
cd "$work"
for target in linux/amd64 linux/arm64 darwin/arm64 windows/amd64; do
  os=${target%/*}; arch=${target#*/}
  echo "== $target vet"; GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 GOOS=$os GOARCH=$arch nice go vet ./... 2>&1 | head -40
  echo "== $target build"; GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 GOOS=$os GOARCH=$arch nice go build -trimpath -ldflags "-s -w" -o "$work/repo-$os-$arch" ./cmd 2>&1 | head -40
done
ls -la "$work"/repo-*
