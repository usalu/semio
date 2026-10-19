#!/usr/bin/env bash
# usage: all.sh <tag> [go test args] -- runs every Go module (except client cli/mcp handled too) sequentially
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; shift
M="🧰️framework/🛍️products/🦑️repo/🔨️modules"
bash "$C/gofin6-tools/sync.sh" || exit 4
export NOSYNC=1
while read -r d; do
  case "$d" in */🚀️bin|*/📦️main) continue;; esac
  slug="$tag-$(echo "$d" | /usr/bin/sed -E 's|.*/🔨️modules/||; s|/📦️packages/🐹️go||; s|/|_|g')"
  bash "$C/gofin6-tools/run-mod.sh" "$slug" "${d#./}" "$@"
done < <(sed -n 's/^\t\(\.\/.*\)$/\1/p' /Users/ueli/Documents/semio/go.work)
