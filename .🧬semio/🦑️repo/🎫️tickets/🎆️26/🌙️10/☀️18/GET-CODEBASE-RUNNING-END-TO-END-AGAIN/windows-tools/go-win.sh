#!/usr/bin/env bash
# go-win.sh <root> <workdir>: flatten the launcher package + its test cases into one module, then vet, build and compile the tests for every Windows and POSIX target
set -euo pipefail
launcher="$1/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"
module="$launcher/📦️packages/🐹️go"
work="$2"
rm -rf "$work"; mkdir -p "$work/cmd"
printf 'module github.com/usalu/semio/repo/launcher\n\ngo 1.25\n' >"$work/go.mod"
index=0
for source in "$module/🐹️.go" "$module"/*/🐹️.go; do
  [ "$(basename "$(dirname "$source")")" = "📦️main" ] && continue
  index=$((index + 1)); cp "$source" "$work/s$index.go"
done
for source in "$launcher"/🧪️tests/*/🐹️.go; do
  index=$((index + 1)); cp "$source" "$work/t${index}_test.go"
done
sed 's#^import launcher .*#import launcher "github.com/usalu/semio/repo/launcher"#' "$module/📦️main/🐹️.go" >"$work/cmd/main.go"
cd "$work"
for target in windows/amd64 windows/arm64 linux/amd64 darwin/arm64; do
  os=${target%/*}; arch=${target#*/}
  echo "== $target vet (sources + tests)"; GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 GOOS=$os GOARCH=$arch go vet . ./cmd 2>&1 | head -40 || true
  echo "== $target build"; GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 GOOS=$os GOARCH=$arch go build -trimpath -o "$work/repo-$os-$arch" ./cmd 2>&1 | head -20 || true
  echo "== $target test binary"; GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 GOOS=$os GOARCH=$arch go test -c -o "$work/test-$os-$arch" . 2>&1 | head -20 || true
done
ls -la "$work" | grep -E 'repo-|test-'
