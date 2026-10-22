#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖️Header
# Builds and installs the `repo` launcher on macOS, Linux and the devcontainer: the one small native program that provisions the pinned JavaScript runtime
# itself and starts the repository implementation, so that no `bun` has to be installed. It needs only Go (the native bootstrap installs the pinned Go).
# Usage: bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh" [destination-directory]
# The default destination is ${XDG_DATA_HOME:-$HOME/.local/share}/semio/launcher/bin; put it on PATH. Run it again after pulling to rebuild.
# Der Launcher baut sich aus dem Checkout; Voraussetzung ist nur Go. Zielverzeichnis optional, Standard siehe oben.
# #endregion 🔖️Header
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
module="$here/../📦️packages/🐹️go"
destination="${1:-${XDG_DATA_HOME:-$HOME/.local/share}/semio/launcher/bin}"

if ! command -v go >/dev/null 2>&1; then
  printf '%s\n' "Go 1.25 or newer is required to build the launcher; the native bootstrap installs it." "Go 1.25 oder neuer ist zum Bauen des Launchers nötig; das native Bootstrap installiert es." >&2
  exit 1
fi

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/cmd" "$destination"
printf 'module github.com/usalu/semio/repo/launcher\n\ngo 1.25\n' >"$work/go.mod"
index=0
for source in "$module/🐹️.go" "$module"/*/🐹️.go; do
  [ "$(basename "$(dirname "$source")")" = "📦️main" ] && continue
  index=$((index + 1))
  cp "$source" "$work/s$index.go"
done
cp "$module/📦️main/🐹️.go" "$work/cmd/main.go"

(cd "$work" && GOWORK=off CGO_ENABLED=0 go build -trimpath -ldflags "-s -w" -o "$work/repo" ./cmd)
rm -f "$destination/repo.new"
cp "$work/repo" "$destination/repo.new"
chmod 755 "$destination/repo.new"
mv -f "$destination/repo.new" "$destination/repo"
printf '[semio] Installed the repo launcher: %s/repo\n[semio] Repo-Launcher installiert: %s/repo\n' "$destination" "$destination" >&2
case ":$PATH:" in
*":$destination:"*) ;;
*) printf '[semio] Add to PATH: %s\n[semio] Zum PATH hinzufügen: %s\n' "$destination" "$destination" >&2 ;;
esac
