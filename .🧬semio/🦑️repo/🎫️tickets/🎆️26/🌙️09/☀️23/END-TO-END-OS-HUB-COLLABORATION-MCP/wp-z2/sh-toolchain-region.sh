#region 🔖️NativeToolchain
run_privileged() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  elif command -v sudo >/dev/null 2>&1; then
    sudo "$@"
  else
    log "Root privileges are required for: $*"
    return 1
  fi
}

linux_package_manager() {
  local manager
  for manager in apt-get dnf pacman zypper; do
    if command -v "$manager" >/dev/null 2>&1; then
      printf '%s' "$manager"
      return 0
    fi
  done
  return 1
}

linux_toolchain_packages() {
  case "$1" in
  apt-get) printf '%s\n' build-essential cmake ninja-build pkg-config uuid-dev curl ca-certificates git unzip xz-utils libicu-dev procps ;;
  dnf) printf '%s\n' gcc gcc-c++ make cmake ninja-build pkgconf-pkg-config libuuid-devel curl ca-certificates git unzip xz tar libicu procps-ng findutils ;;
  pacman) printf '%s\n' base-devel cmake ninja pkgconf util-linux-libs curl ca-certificates git unzip xz icu procps-ng ;;
  zypper) printf '%s\n' gcc gcc-c++ make cmake ninja pkg-config libuuid-devel curl ca-certificates git unzip xz tar libicu-devel procps ;;
  esac
}

install_linux_packages() {
  local manager="$1"
  shift
  case "$manager" in
  apt-get) run_privileged env DEBIAN_FRONTEND=noninteractive apt-get update -qq && run_privileged env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "$@" ;;
  dnf) run_privileged dnf install -y "$@" ;;
  pacman) run_privileged pacman -Sy --needed --noconfirm "$@" ;;
  zypper) run_privileged zypper --non-interactive install --no-recommends "$@" ;;
  *) return 1 ;;
  esac
}

report_missing_tools() {
  local missing=() tool
  for tool in "$@"; do
    command -v "$tool" >/dev/null 2>&1 || missing+=("$tool")
  done
  if [ "${#missing[@]}" -gt 0 ]; then
    log "Native toolchain incomplete, missing: ${missing[*]}. Install them with your system package manager and rerun setup."
  fi
}

ensure_native_toolchain() {
  case "$(uname -s)" in
  Darwin)
    xcode-select -p >/dev/null 2>&1 || log "Xcode Command Line Tools are missing; run xcode-select --install and rerun setup."
    if command -v brew >/dev/null 2>&1; then
      brew install cmake ninja pkg-config || log "Homebrew could not install cmake, ninja and pkg-config."
    else
      log "Homebrew not available; install CMake, Ninja and pkg-config before running native C++ builds."
    fi
    export CMAKE_PRESET="${CMAKE_PRESET:-macos}"
    ;;
  Linux)
    local manager packages=() package
    if manager="$(linux_package_manager)"; then
      while IFS= read -r package; do packages+=("$package"); done < <(linux_toolchain_packages "$manager")
      log "Installing the native toolchain with $manager: ${packages[*]}"
      install_linux_packages "$manager" "${packages[@]}" || log "The native toolchain install through $manager failed (package index, lock or privileges); setup continues and reports what is missing."
    else
      log "No supported package manager (apt-get, dnf, pacman, zypper) found; install a C/C++ compiler, make, CMake, Ninja, pkg-config, libuuid, ICU, curl, git and unzip."
    fi
    export CMAKE_PRESET="${CMAKE_PRESET:-linux}"
    ;;
  esac
  report_missing_tools cc c++ make cmake ninja pkg-config curl git unzip
}

verify_sha256() {
  local file="$1" expected="$2" actual
  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$file" | awk '{print $1}')"
  else
    actual="$(shasum -a 256 "$file" | awk '{print $1}')"
  fi
  [ "$actual" = "$expected" ] || {
    log "Checksum mismatch for $file (expected $expected, got $actual)."
    return 1
  }
}

ensure_rust_toolchain() {
  export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}" RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
  export PATH="$CARGO_HOME/bin:$PATH"
  if ! command -v rustup >/dev/null 2>&1; then
    log "Installing rustup…"
    curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | RUSTUP_INIT_SKIP_PATH_CHECK=yes sh -s -- -y --profile minimal --default-toolchain none || return 1
  fi
  log "Installing the rust-toolchain.toml toolchain…"
  (cd "$REPO_ROOT" && rustup toolchain install) || return 1
}

go_minor_version() {
  go version 2>/dev/null | sed -nE 's/^go version go1\.([0-9]+).*/\1/p'
}

ensure_go() {
  export PATH="$LOCAL_BIN:$PATH"
  local minor
  minor="$(go_minor_version)"
  if [ -n "$minor" ] && [ "$minor" -ge 21 ]; then
    return 0
  fi
  local os arch version archive root download
  case "$(uname -s)" in
  Darwin) os=darwin ;;
  Linux) os=linux ;;
  *) return 1 ;;
  esac
  case "$(uname -m)" in
  x86_64 | amd64) arch=amd64 ;;
  arm64 | aarch64) arch=arm64 ;;
  *) return 1 ;;
  esac
  version="$(curl -fsSL --retry 3 'https://go.dev/VERSION?m=text' | sed -n 1p)" || return 1
  archive="$version.$os-$arch.tar.gz"
  root="${XDG_DATA_HOME:-$HOME/.local/share}/semio/go/$version"
  if [ ! -x "$root/go/bin/go" ]; then
    log "Installing Go $version into $root…"
    download="$(mktemp -d)"
    curl -fsSL --retry 3 -o "$download/$archive" "https://dl.google.com/go/$archive" || return 1
    verify_sha256 "$download/$archive" "$(curl -fsSL --retry 3 "https://dl.google.com/go/$archive.sha256")" || return 1
    mkdir -p "$root" && tar -xzf "$download/$archive" -C "$root" || return 1
    rm -rf "$download"
  fi
  mkdir -p "$LOCAL_BIN"
  ln -sf "$root/go/bin/go" "$LOCAL_BIN/go"
  ln -sf "$root/go/bin/gofmt" "$LOCAL_BIN/gofmt"
}

ensure_dotnet() {
  export DOTNET_ROOT="${DOTNET_ROOT:-$HOME/.dotnet}" DOTNET_CLI_TELEMETRY_OPTOUT=1
  export PATH="$LOCAL_BIN:$PATH"
  if command -v dotnet >/dev/null 2>&1 && dotnet --list-runtimes 2>/dev/null | grep -q "^Microsoft.NETCore.App ${DOTNET_CHANNEL}\."; then
    return 0
  fi
  log "Installing .NET SDK $DOTNET_CHANNEL into $DOTNET_ROOT…"
  local installer
  installer="$(mktemp)"
  curl -fsSL --retry 3 -o "$installer" https://dot.net/v1/dotnet-install.sh || return 1
  bash "$installer" --channel "$DOTNET_CHANNEL" --install-dir "$DOTNET_ROOT" --no-path || return 1
  rm -f "$installer"
  mkdir -p "$LOCAL_BIN"
  ln -sf "$DOTNET_ROOT/dotnet" "$LOCAL_BIN/dotnet"
}
#endregion 🔖️NativeToolchain
