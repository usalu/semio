#!/bin/sh
# 🐹️ verify.sh <root>: every Go launcher case (the language contract reads the shared vectors), vet for windows/linux/darwin, and the MCP module (embeds the launcher) still compiles. Needs the pinned Go: LP_GO=<path to go 1.26.9> (default `go`).
set -u
ROOT=${1:?usage: verify.sh <root>}
HERE=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$HERE/../../.." && pwd)
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go"
fail=0
mkdir -p "/private/tmp/m/lpgo"
bun "$C/langpick-tools/gotest.ts" "$ROOT" "$G" "/private/tmp/m/lpgo" || fail=1
for os in windows linux darwin; do GOOS=$os LP_GOVERB=vet bun "$C/langpick-tools/gotest.ts" "$ROOT" "$G" "/private/tmp/m/lpgo-$os" || { fail=1; echo "RED vet $os"; }; done
bun "$C/langpick-tools/gotest.ts" "$ROOT" "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp" "$ROOT/.tmp/lp-mcp" -run XXX_NONE || fail=1
echo "launcher verify exit $fail"
exit $fail
