#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖️Header
# Builds and installs the `repo` launcher on macOS, Linux and the devcontainer: the one small native program that provisions the pinned JavaScript runtime
# itself and starts the repository implementation, so that neither `bun` nor a Go toolchain has to be installed by hand.
# Stage 0 of a bare machine: the pinned Go toolchain is found (PATH, the repository tools partition, the user cache) or fetched from the pins in
# `🔩️native/🥾️bootstrap/🧬️schema/🔣️.json` (https only, SHA-256 checked before it is unpacked, one atomic rename, progress, Ctrl+C leaves nothing); it needs only curl, tar and a SHA-256 tool.
# Usage: bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh" [--launcher-only] [destination-directory]
# The default destination is ${XDG_DATA_HOME:-$HOME/.local/share}/semio/launcher/bin and is never put on PATH: the launcher is only the bootstrap. The PATH `repo` is the native command, owned by `repo run repo-mcp:install`;
# unless --launcher-only is given (CI, the full bootstrap, which chains the same install later), this installer ends by running exactly that with the new launcher, so one command takes a bare machine to a working `repo` and `repo mcp`. Run it again after pulling to rebuild.
# Environment: SEMIO_GO_ROOT (user cache of the toolchain, default ${XDG_DATA_HOME:-$HOME/.local/share}/semio/go), SEMIO_GO_MIRROR (https or loopback http directory that holds the pinned archive).
# Der Launcher baut sich aus dem Checkout; das gepinnte Go holt sich das Skript bei Bedarf selbst (nur https, SHA-256 vor dem Entpacken). Zielverzeichnis optional, Standard siehe oben; der Launcher kommt nie in den PATH, das `repo` im PATH ist der native Befehl (`repo run repo-mcp:install`), den das Skript am Ende selbst installiert, außer mit --launcher-only.
# #endregion 🔖️Header
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
module="$here/../📦️packages/🐹️go"
root="$(cd "$here/../../../../../.." && pwd)"
pins="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json"
destination=""
launcher_only=0
go_home="${SEMIO_GO_ROOT:-${XDG_DATA_HOME:-$HOME/.local/share}/semio/go}"

say() {
  printf '[semio] %s\n[semio] %s\n' "$1" "$2" >&2
}

fail() {
  say "$1" "$2"
  exit 1
}

for argument in "$@"; do
  case "$argument" in
  --launcher-only) launcher_only=1 ;;
  -*) fail "Unknown option $argument; usage: 🐚️.sh [--launcher-only] [destination-directory]" "Unbekannte Option $argument; Aufruf: 🐚️.sh [--launcher-only] [Zielverzeichnis]" ;;
  *) destination="$argument" ;;
  esac
done
destination="${destination:-${XDG_DATA_HOME:-$HOME/.local/share}/semio/launcher/bin}"

pin_row() {
  { grep -F "{\"tool\":\"go\",\"version\":\"" "$pins" | grep -F "\"platform\":\"$1\",\"arch\":\"$2\"," | sed -n '1p'; } || true
}

pin_field() {
  printf '%s' "$1" | sed -n "s/.*\"$2\":\"\([^\"]*\)\".*/\1/p"
}

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  else
    fail "A SHA-256 tool (sha256sum or shasum) is required to verify the Go toolchain." "Ein SHA-256-Werkzeug (sha256sum oder shasum) ist zum Prüfen der Go-Toolchain nötig."
  fi
}

host_platform() {
  case "$(uname -s)" in
  Darwin) printf darwin ;;
  Linux) printf linux ;;
  *) return 1 ;;
  esac
}

host_arch() {
  case "$(uname -m)" in
  x86_64 | amd64) printf x64 ;;
  arm64 | aarch64) printf arm64 ;;
  *) return 1 ;;
  esac
}

is_pinned_go() {
  [ -x "$1" ] && [ "$("$1" version 2>/dev/null | awk '{print $3}')" = "go$2" ]
}

work=""
partial=""
lock=""
child=""
launcher_ready=0
cleanup() {
  [ -z "$work" ] || rm -rf "$work"
  [ -z "$partial" ] || rm -rf "$partial"
  [ -z "$lock" ] || rm -rf "$lock"
}
cancelled() {
  if [ -n "$child" ]; then
    kill -TERM "$child" 2>/dev/null || true
    wait "$child" 2>/dev/null || true
  fi
  if [ "$launcher_ready" -eq 1 ]; then
    say "Cancelled; the launcher is installed, the native repo command is not. Finish with: $destination/repo run repo-mcp:install" "Abgebrochen; der Launcher ist installiert, der native repo-Befehl nicht. Zum Abschluss: $destination/repo run repo-mcp:install"
  else
    say "Cancelled; nothing was installed." "Abgebrochen; nichts wurde installiert."
  fi
  exit 130
}
trap cleanup EXIT
trap cancelled INT TERM

wait_for_lock() {
  local attempts=0 owner
  mkdir -p "$(dirname "$1")"
  until mkdir "$1" 2>/dev/null; do
    owner="$(cat "$1/pid" 2>/dev/null || true)"
    if [ -n "$owner" ] && ! kill -0 "$owner" 2>/dev/null; then
      rm -rf "$1"
      continue
    fi
    attempts=$((attempts + 1))
    [ "$attempts" -le 900 ] || fail "Another process has been installing the Go toolchain for 15 minutes: $1" "Ein anderer Prozess installiert seit 15 Minuten die Go-Toolchain: $1"
    [ "$attempts" -ne 1 ] || say "Waiting for another process that installs the Go toolchain…" "Warte auf einen anderen Prozess, der die Go-Toolchain installiert…"
    sleep 1
  done
  printf '%s' "$$" >"$1/pid"
  lock="$1"
}

download_go() {
  local row="$1" version="$2" target="$3" url sha base archive actual tty=0 protocols="=https"
  url="$(pin_field "$row" url)"
  sha="$(pin_field "$row" sha256)"
  archive="${url##*/}"
  if [ -n "${SEMIO_GO_MIRROR:-}" ]; then
    case "$SEMIO_GO_MIRROR" in
    https://*) base="${SEMIO_GO_MIRROR%/}" ;;
    http://127.0.0.1[:/]* | http://localhost[:/]* | http://\[::1\][:/]*) base="${SEMIO_GO_MIRROR%/}" protocols="=https,http" ;;
    *) fail "SEMIO_GO_MIRROR must be an https URL or a loopback http URL." "SEMIO_GO_MIRROR muss eine https-URL oder eine Loopback-http-URL sein." ;;
    esac
    url="$base/$archive"
  fi
  command -v curl >/dev/null 2>&1 || fail "curl is required to download the Go toolchain." "curl ist zum Herunterladen der Go-Toolchain nötig."
  command -v tar >/dev/null 2>&1 || fail "tar is required to unpack the Go toolchain." "tar ist zum Entpacken der Go-Toolchain nötig."
  mkdir -p "$(dirname "$target")"
  wait_for_lock "$target.lock"
  if is_pinned_go "$target/bin/go" "$version"; then
    return 0
  fi
  partial="$(mktemp -d "$(dirname "$target")/.partial.XXXXXX")"
  say "Downloading Go $version (pinned, SHA-256 checked): $url" "Lade Go $version herunter (gepinnt, SHA-256 geprüft): $url"
  [ -t 2 ] && tty=1
  if [ "$tty" -eq 1 ]; then
    curl --proto "$protocols" --proto-redir "=https" --tlsv1.2 -fL --progress-bar --retry 3 --connect-timeout 30 --max-time 1800 --max-filesize 400000000 -o "$partial/$archive" "$url" || fail "The Go download failed: $url" "Der Go-Download ist fehlgeschlagen: $url"
  else
    curl --proto "$protocols" --proto-redir "=https" --tlsv1.2 -fsSL --retry 3 --connect-timeout 30 --max-time 1800 --max-filesize 400000000 -o "$partial/$archive" "$url" || fail "The Go download failed: $url" "Der Go-Download ist fehlgeschlagen: $url"
  fi
  actual="$(sha256_of "$partial/$archive")"
  [ "$actual" = "$sha" ] || fail "Checksum mismatch for $archive (pinned $sha, received $actual); nothing was unpacked." "Prüfsumme von $archive stimmt nicht (gepinnt $sha, erhalten $actual); nichts wurde entpackt."
  say "Unpacking Go $version…" "Entpacke Go $version…"
  mkdir "$partial/unpacked"
  tar -xzf "$partial/$archive" -C "$partial/unpacked" || fail "The Go archive could not be unpacked." "Das Go-Archiv konnte nicht entpackt werden."
  is_pinned_go "$partial/unpacked/go/bin/go" "$version" || fail "The unpacked toolchain does not report Go $version." "Die entpackte Toolchain meldet nicht Go $version."
  rm -rf "$target"
  mv "$partial/unpacked/go" "$target"
  say "Installed Go $version: $target" "Go $version installiert: $target"
}

ensure_go() {
  local platform arch row version sha partition
  [ -f "$pins" ] || fail "The pinned toolchain table is missing: $pins" "Die Tabelle der gepinnten Toolchains fehlt: $pins"
  platform="$(host_platform)" || fail "Unsupported operating system $(uname -s); on Windows use the PowerShell installer." "Nicht unterstütztes Betriebssystem $(uname -s); unter Windows den PowerShell-Installer verwenden."
  arch="$(host_arch)" || fail "Unsupported architecture $(uname -m)." "Nicht unterstützte Architektur $(uname -m)."
  row="$(pin_row "$platform" "$arch")"
  [ -n "$row" ] || fail "No pinned Go for $platform/$arch in $pins." "Kein gepinntes Go für $platform/$arch in $pins."
  version="$(pin_field "$row" version)"
  sha="$(pin_field "$row" sha256)"
  if command -v go >/dev/null 2>&1 && is_pinned_go "$(command -v go)" "$version"; then
    GO="$(command -v go)"
    return 0
  fi
  partition="$root/.cache/b/t/m/native/go/$version/$platform-$arch-${sha:0:16}/bin/go"
  if is_pinned_go "$partition" "$version"; then
    GO="$partition"
    return 0
  fi
  GO="$go_home/v$version/bin/go"
  is_pinned_go "$GO" "$version" || download_go "$row" "$version" "$go_home/v$version"
  is_pinned_go "$GO" "$version" || fail "The Go toolchain $version is not usable at $GO." "Die Go-Toolchain $version ist unter $GO nicht benutzbar."
}

GO=""
ensure_go

work="$(mktemp -d)"
mkdir -p "$work/cmd" "$destination"
printf 'module github.com/usalu/semio/repo/launcher\n\ngo 1.25\n' >"$work/go.mod"
index=0
for source in "$module/🐹️.go" "$module"/*/🐹️.go; do
  [ "$(basename "$(dirname "$source")")" = "📦️main" ] && continue
  index=$((index + 1))
  cp "$source" "$work/s$index.go"
done
cp "$module/📦️main/🐹️.go" "$work/cmd/main.go"

(cd "$work" && GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 "$GO" build -trimpath -ldflags "-s -w" -o "$work/repo" ./cmd) || fail "Building the launcher failed." "Das Bauen des Launchers ist fehlgeschlagen."
rm -f "$destination/repo.new"
cp "$work/repo" "$destination/repo.new"
chmod 755 "$destination/repo.new"
mv -f "$destination/repo.new" "$destination/repo"
launcher_ready=1
printf '[semio] Installed the repo launcher: %s/repo\n[semio] Repo-Launcher installiert: %s/repo\n' "$destination" "$destination" >&2

physical() {
  local path="$1" target
  while [ -L "$path" ]; do
    target="$(readlink "$path")"
    case "$target" in
    /*) path="$target" ;;
    *) path="$(dirname "$path")/$target" ;;
    esac
  done
  printf '%s/%s' "$(cd "$(dirname "$path")" && pwd -P)" "$(basename "$path")"
}

native_executable() {
  local path
  path="$(sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' "$HOME/.local/share/semio/repo/installed.json" 2>/dev/null | sed -n '1p')" || true
  if [ -n "$path" ] && [ -x "$path" ]; then physical "$path"; fi
}

chain_native() {
  local status=0
  say "Installing the native repo command: repo run repo-mcp:install" "Installiere den nativen repo-Befehl: repo run repo-mcp:install"
  (cd "$root" && SEMIO_WORKSPACE_ROOT="$root" exec "$destination/repo" run repo-mcp:install) <&0 &
  child=$!
  wait "$child" || status=$?
  child=""
  [ "$status" -eq 0 ] || fail "The native installation failed with exit code $status; the launcher is installed, retry: $destination/repo run repo-mcp:install" "Die native Installation ist mit Exit-Code $status fehlgeschlagen; der Launcher ist installiert, erneut versuchen: $destination/repo run repo-mcp:install"
}

report_shadows() {
  local native entry found
  native="$(native_executable)"
  IFS=: read -r -a path_entries <<<"$PATH"
  for entry in ${path_entries[@]+"${path_entries[@]}"}; do
    [ -n "$entry" ] && [ -x "$entry/repo" ] && [ ! -d "$entry/repo" ] || continue
    found="$(physical "$entry/repo")"
    [ "$found" = "$native" ] && continue
    printf '[semio] Another repo on PATH is not the native repo command: %s/repo. The native command owns the PATH repo, install it: repo run repo-mcp:install\n[semio] Ein anderes repo im PATH ist nicht der native repo-Befehl: %s/repo. Der native Befehl besitzt das repo im PATH, installieren: repo run repo-mcp:install\n' "$entry" "$entry" >&2
  done
}

if [ "$launcher_only" -eq 1 ]; then
  say "Launcher only; no repo command was put on PATH. Install the native one: $destination/repo run repo-mcp:install" "Nur der Launcher; kein repo-Befehl wurde in den PATH gelegt. Den nativen installieren: $destination/repo run repo-mcp:install"
else
  chain_native
  say "The native repo command is installed. Open a new shell to use it." "Der native repo-Befehl ist installiert. Eine neue Shell öffnen, um ihn zu nutzen."
fi
report_shadows
