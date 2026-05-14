#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖Header
# Zero-touch macOS/Linux bootstrap: Neo4j Desktop, uv, Neo4j env vars for MCP, bun install, workspace:setup.
# Native setup is rooted here and does not depend on devcontainer scripts.
# #endregion 🔖Header
set -euo pipefail

#region 🔖Config
NEO4J_DESKTOP_INSTALLER_VERSION="${NEO4J_DESKTOP_INSTALLER_VERSION:-1.6.3}"
NEO4J_VERSION="${NEO4J_VERSION:-5.26.26}"
APOC_VERSION="${APOC_VERSION:-5.26.4}"
SKIP_NEO4J_DESKTOP="${SKIP_NEO4J_DESKTOP:-0}"
SKIP_REPO_BOOTSTRAP="${SKIP_REPO_BOOTSTRAP:-0}"
SEMIO_SESSION_START="${SEMIO_SESSION_START:-0}"
#endregion 🔖Config

#region 🔖Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR"
#endregion 🔖Paths

#region 🔖Logging
log() {
  printf '%s\n' "[semio] $*"
}
#endregion 🔖Logging

#region 🔖Neo4jEnv
append_neo4j_env_block() {
  local f="$1"
  [ -n "$f" ] || return 0
  mkdir -p "$(dirname "$f")"
  touch "$f"
  if grep -qF "#region 🔌Neo4j" "$f" 2>/dev/null; then
    return 0
  fi
  {
    printf '\n'
    printf '%s\n' "#region 🔌Neo4j"
    printf '%s\n' "export NEO4J_URI=bolt://localhost:7687"
    printf '%s\n' "export NEO4J_USERNAME=neo4j"
    printf '%s\n' "export NEO4J_PASSWORD=password"
    printf '%s\n' "export NEO4J_TELEMETRY=false"
    printf '%s\n' "#endregion 🔌Neo4j"
  } >>"$f"
  log "Appended Neo4j MCP env block to $f"
}

configure_neo4j_shell_env() {
  case "$(uname -s)" in
  Darwin)
    append_neo4j_env_block "${HOME}/.zshrc"
    append_neo4j_env_block "${HOME}/.bashrc"
    ;;
  Linux)
    append_neo4j_env_block "${HOME}/.bashrc"
    append_neo4j_env_block "${HOME}/.profile"
    ;;
  *)
    log "Unsupported OS for shell env; set NEO4J_* manually."
    ;;
  esac
  export NEO4J_URI="bolt://localhost:7687"
  export NEO4J_USERNAME="neo4j"
  export NEO4J_PASSWORD="password"
  export NEO4J_TELEMETRY="false"
}
#endregion 🔖Neo4jEnv

#region 🔖CodexMcp
sync_codex_mcp_config() {
  local template="$REPO_ROOT/.codex/config.toml"
  [ -f "$template" ] || return 0
  local target="$HOME/.codex/config.toml"
  mkdir -p "$(dirname "$target")"
  local tmp
  tmp="$(mktemp)"
  if [ -f "$target" ]; then
    awk '
      /^\[mcp_servers\./ { skip=1; next }
      /^\[/ { skip=0 }
      !skip { print }
    ' "$target" >"$tmp"
  fi
  sed "s|cwd = \"\\.\"|cwd = \"$REPO_ROOT\"|g" "$template" >>"$tmp"
  mv "$tmp" "$target"
}
#endregion 🔖CodexMcp

#region 🔖Neo4jRuntime
is_neo4j_reachable() {
  if command -v nc >/dev/null 2>&1; then
    nc -z 127.0.0.1 7687 >/dev/null 2>&1
    return $?
  fi
  timeout 1 bash -c "echo >/dev/tcp/127.0.0.1/7687" >/dev/null 2>&1
}

run_cypher() {
  local database="$1"
  local cypher="$2"
  local bundled_cypher_shell="$REPO_ROOT/.repo/cache/neo4j/neo4j-community-$NEO4J_VERSION/bin/cypher-shell"
  if command -v cypher-shell >/dev/null 2>&1; then
    cypher-shell -a bolt://localhost:7687 -u neo4j -p password -d "$database" --format plain "$cypher" >/dev/null 2>&1
    return $?
  fi
  if [ -x "$bundled_cypher_shell" ]; then
    "$bundled_cypher_shell" -a bolt://localhost:7687 -u neo4j -p password -d "$database" --format plain "$cypher" >/dev/null 2>&1
    return $?
  fi
  return 1
}

run_cypher_expect() {
  local database="$1"
  local cypher="$2"
  local pattern="$3"
  local bundled_cypher_shell="$REPO_ROOT/.repo/cache/neo4j/neo4j-community-$NEO4J_VERSION/bin/cypher-shell"
  local output
  if command -v cypher-shell >/dev/null 2>&1; then
    output="$(cypher-shell -a bolt://localhost:7687 -u neo4j -p password -d "$database" --format plain "$cypher" 2>/dev/null)" || return 1
    printf '%s\n' "$output" | grep -Eq "$pattern"
    return $?
  fi
  if [ -x "$bundled_cypher_shell" ]; then
    output="$("$bundled_cypher_shell" -a bolt://localhost:7687 -u neo4j -p password -d "$database" --format plain "$cypher" 2>/dev/null)" || return 1
    printf '%s\n' "$output" | grep -Eq "$pattern"
    return $?
  fi
  return 1
}

neo4j_schema_cypher_uri() {
  local technology="$1"
  printf 'file://%s/.repo/\\uD83D\\uDEC2/%s.cypher\n' "$REPO_ROOT" "$technology"
}

detect_neo4j_desktop_dbms_home() {
  ps -eo args 2>/dev/null | sed -nE 's/.*--home-dir="?([^" ]*\.Neo4jDesktop2?\/Data\/dbmss\/[^" ]+)"?.*/\1/p' | head -1
}

set_text_setting() {
  local file="$1"
  local key="$2"
  local value="$3"
  mkdir -p "$(dirname "$file")"
  touch "$file"
  if grep -Eq "^[#[:space:]]*${key}=" "$file"; then
    sed -i.bak -E "s|^[#[:space:]]*${key}=.*|${key}=${value}|" "$file"
  else
    printf '%s=%s\n' "$key" "$value" >>"$file"
  fi
}

install_neo4j_desktop_apoc() {
  local dbms_home
  dbms_home="$(detect_neo4j_desktop_dbms_home)"
  if [ -z "$dbms_home" ]; then
    log "APOC auto-install skipped because the reachable DBMS is not a running Neo4j Desktop local DBMS."
    return 1
  fi

  mkdir -p "$dbms_home/plugins"
  local core_jar
  core_jar="$(find "$dbms_home/labs" -maxdepth 1 -type f -name 'apoc-*-core.jar' 2>/dev/null | sort -r | head -1)"
  if [ -n "$core_jar" ]; then
    cp -f "$core_jar" "$dbms_home/plugins/$(basename "$core_jar")"
    local apoc_version
    apoc_version="$(basename "$core_jar" | sed -nE 's/apoc-(.+)-core\.jar/\1/p')"
    if [ -n "$apoc_version" ] && [ ! -f "$dbms_home/plugins/apoc-extended-$apoc_version.jar" ]; then
      curl -fSL --retry 3 --retry-delay 2 -o "$dbms_home/plugins/apoc-extended-$apoc_version.jar" "https://repo.maven.apache.org/maven2/org/neo4j/procedure/apoc-extended/$apoc_version/apoc-extended-$apoc_version.jar"
    fi
  fi

  set_text_setting "$dbms_home/conf/neo4j.conf" "dbms.security.procedures.allowlist" "apoc.*"
  set_text_setting "$dbms_home/conf/neo4j.conf" "dbms.security.procedures.unrestricted" "apoc.*"
  set_text_setting "$dbms_home/conf/neo4j.conf" "server.directories.import" "$REPO_ROOT"
  set_text_setting "$dbms_home/conf/apoc.conf" "apoc.export.file.enabled" "true"
  set_text_setting "$dbms_home/conf/apoc.conf" "apoc.import.file.enabled" "true"
  set_text_setting "$dbms_home/conf/apoc.conf" "apoc.import.file.use_neo4j_config" "false"

  if [ -x "$dbms_home/bin/neo4j" ]; then
    log "Restarting Neo4j Desktop local semio DBMS to load APOC..."
    pkill -f "$dbms_home" >/dev/null 2>&1 || true
    for _ in $(seq 1 30); do
      is_neo4j_reachable || break
      sleep 1
    done
    nohup "$dbms_home/bin/neo4j" console >"$dbms_home/logs/semio-native-console.log" 2>&1 &
    for _ in $(seq 1 45); do
      is_neo4j_reachable && break
      sleep 2
    done
  else
    log "APOC installed into the semio DBMS. Restart it in Neo4j Desktop to load the plugin."
  fi
}

java_major_version() {
  if ! command -v java >/dev/null 2>&1; then
    printf '0\n'
    return 0
  fi
  java -version 2>&1 | sed -n 's/.*version "\([0-9][0-9]*\).*/\1/p' | head -1
}

ensure_java_runtime() {
  local major
  major="$(java_major_version)"
  if [ -n "$major" ] && [ "$major" -ge 21 ]; then
    return 0
  fi
  if [ "$SEMIO_SESSION_START" = "1" ]; then
    log "Java 21+ is required for Neo4j. Run setup.mac.sh or setup.linux.sh to install it."
    return 1
  fi
  case "$(uname -s)" in
  Darwin)
    if command -v brew >/dev/null 2>&1; then
      brew install openjdk@21 || true
      for b in /opt/homebrew/opt/openjdk@21/bin /usr/local/opt/openjdk@21/bin; do
        [ -d "$b" ] && export PATH="$b:$PATH"
      done
    fi
    ;;
  Linux)
    if command -v apt-get >/dev/null 2>&1; then
      if command -v sudo >/dev/null 2>&1 && [ "$(id -u)" -ne 0 ]; then
        sudo apt-get update -qq
        sudo apt-get install -y --no-install-recommends openjdk-21-jre-headless || sudo apt-get install -y --no-install-recommends openjdk-17-jre-headless
      elif [ "$(id -u)" -eq 0 ]; then
        apt-get update -qq
        apt-get install -y --no-install-recommends openjdk-21-jre-headless || apt-get install -y --no-install-recommends openjdk-17-jre-headless
      fi
    fi
    ;;
  esac
}

ensure_native_neo4j_tools() {
  local cache_root="$REPO_ROOT/.repo/cache/neo4j"
  local runtime_root="$cache_root/neo4j-community-$NEO4J_VERSION"
  mkdir -p "$cache_root"
  if [ ! -d "$runtime_root" ]; then
    local archive="$cache_root/neo4j-community-$NEO4J_VERSION-unix.tar.gz"
    log "Downloading Neo4j Community $NEO4J_VERSION tools for cypher-shell..."
    curl -fSL --retry 3 --retry-delay 2 -o "$archive" "https://dist.neo4j.org/neo4j-community-$NEO4J_VERSION-unix.tar.gz"
    tar -xzf "$archive" -C "$cache_root"
  fi

  mkdir -p "$runtime_root/plugins"
  if [ ! -f "$runtime_root/plugins/apoc-core-$APOC_VERSION-core.jar" ]; then
    curl -fSL --retry 3 --retry-delay 2 -o "$runtime_root/plugins/apoc-core-$APOC_VERSION-core.jar" "https://repo.maven.apache.org/maven2/org/neo4j/procedure/apoc-core/$APOC_VERSION/apoc-core-$APOC_VERSION-core.jar"
  fi
  if [ ! -f "$runtime_root/plugins/apoc-$APOC_VERSION-extended.jar" ]; then
    curl -fSL --retry 3 --retry-delay 2 -o "$runtime_root/plugins/apoc-$APOC_VERSION-extended.jar" "https://github.com/neo4j-contrib/neo4j-apoc-procedures/releases/download/$APOC_VERSION/apoc-$APOC_VERSION-extended.jar"
  fi

  local conf="$runtime_root/conf/neo4j.conf"
  mkdir -p "$runtime_root/data" "$runtime_root/logs"
  set_conf_value "$conf" "server.default_listen_address" "127.0.0.1"
  set_conf_value "$conf" "server.bolt.listen_address" ":7687"
  set_conf_value "$conf" "server.http.listen_address" ":7474"
  set_conf_value "$conf" "dbms.usage_report.enabled" "false"
  set_conf_value "$conf" "server.directories.data" "$runtime_root/data"
  set_conf_value "$conf" "server.directories.logs" "$runtime_root/logs"
  set_conf_value "$conf" "server.directories.import" "$REPO_ROOT"
  set_conf_value "$conf" "dbms.security.procedures.allowlist" "apoc.*"
  set_conf_value "$conf" "dbms.security.procedures.unrestricted" "apoc.*"
  {
    printf '%s\n' "apoc.export.file.enabled=true"
    printf '%s\n' "apoc.import.file.enabled=true"
    printf '%s\n' "apoc.import.file.use_neo4j_config=false"
  } >"$runtime_root/conf/apoc.conf"
  printf '%s\n' "$runtime_root"
}

set_conf_value() {
  local conf="$1"
  local key="$2"
  local value="$3"
  if grep -Eq "^[#[:space:]]*${key}=" "$conf"; then
    sed -i.bak -E "s|^[#[:space:]]*${key}=.*|${key}=${value}|" "$conf"
  else
    printf '%s=%s\n' "$key" "$value" >>"$conf"
  fi
}

ensure_native_neo4j() {
  local runtime_root
  if is_neo4j_reachable; then
    log "Neo4j is reachable at bolt://localhost:7687."
  else
    log "Neo4j is not reachable. Create and start a native Neo4j Desktop local DBMS named semio on Bolt port 7687, password password, then run this setup again."
    return 0
  fi

  runtime_root="$(ensure_native_neo4j_tools)"
  if ! run_cypher_expect neo4j "SHOW PROCEDURES YIELD name WHERE name IN ['apoc.cypher.runFile', 'apoc.export.cypher.query'] RETURN count(name) AS count;" "\\b2\\b"; then
    install_neo4j_desktop_apoc || true
    if ! run_cypher_expect neo4j "SHOW PROCEDURES YIELD name WHERE name IN ['apoc.cypher.runFile', 'apoc.export.cypher.query'] RETURN count(name) AS count;" "\\b2\\b"; then
      log "Neo4j is reachable, but APOC is not ready. In Neo4j Desktop, install/enable APOC for the local semio DBMS and restart it."
      return 0
    fi
  fi

  for technology in semio elements coda reuse; do
    if run_cypher system "CREATE DATABASE ${technology} IF NOT EXISTS;"; then
      log "Neo4j database ready: ${technology}."
    else
      log "Neo4j database ${technology} was not created; using the reachable default database for this setup."
    fi
    local schema_file="$REPO_ROOT/.repo/🛂/${technology}.cypher"
    if [ -f "$schema_file" ] && grep -Eqv '^[[:space:]]*(//|:|$)' "$schema_file"; then
      local schema_uri
      schema_uri="$(neo4j_schema_cypher_uri "$technology")"
      run_cypher neo4j "CALL apoc.cypher.runFile('$schema_uri') YIELD row RETURN count(row) AS rows;" || true
      log "Neo4j schema imported: ${technology}."
    fi
  done
}
#endregion 🔖Neo4jRuntime

#region 🔖EnsureUv
ensure_uv() {
  if command -v uv >/dev/null 2>&1; then
    return 0
  fi
  log "Installing uv…"
  curl -LsSf https://astral.sh/uv/install.sh | sh
  export PATH="${HOME}/.local/bin:${PATH}"
}
#endregion 🔖EnsureUv

#region 🔖EnsureBun
ensure_bun() {
  if command -v bun >/dev/null 2>&1; then
    return 0
  fi
  log "Installing Bun…"
  export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"
  curl -fsSL https://bun.sh/install | bash
  export PATH="${BUN_INSTALL}/bin:${PATH}"
}
#endregion 🔖EnsureBun

#region 🔖InstallNeo4jDesktop
install_linux_fuse_deps() {
  if [ "${SKIP_LINUX_APT:-0}" = "1" ]; then
    return 0
  fi
  if ! command -v apt-get >/dev/null 2>&1; then
    return 0
  fi
  if command -v sudo >/dev/null 2>&1 && [ "$(id -u)" -ne 0 ]; then
    log "Installing fuse dependencies for Neo4j Desktop AppImage (apt)…"
    sudo apt-get update -qq
    sudo apt-get install -y --no-install-recommends wget curl ca-certificates libfuse2 fuse3 libglib2.0-0 \
      libgtk-3-0 libnotify4 libnss3 libxss1 libxtst6 xdg-utils \
      || log "Optional apt packages skipped (non-fatal)."
  elif [ "$(id -u)" -eq 0 ]; then
    apt-get update -qq
    apt-get install -y --no-install-recommends wget curl ca-certificates libfuse2 fuse3 \
      || true
  fi
}

install_neo4j_desktop_linux_appimage() {
  local arch
  arch="$(uname -m)"
  if [ "$arch" != "x86_64" ]; then
    log "Neo4j Desktop Linux AppImage is published for x86_64 only; install manually from https://neo4j.com/deployment-center/?desktop-gdb (arch=${arch})."
    return 0
  fi
  install_linux_fuse_deps
  local ver="$NEO4J_DESKTOP_INSTALLER_VERSION"
  local url="https://dist.neo4j.org/neo4j-desktop/linux/neo4j-desktop-${ver}-x86_64.AppImage"
  local dest_dir="${HOME}/.local/share/semio/neo4j-desktop"
  local dest="${dest_dir}/neo4j-desktop-${ver}-x86_64.AppImage"
  mkdir -p "$dest_dir"
  if [ -f "$dest" ]; then
    log "Neo4j Desktop AppImage already present: $dest"
  else
    log "Downloading Neo4j Desktop ${ver} AppImage…"
    curl -fSL --retry 3 --retry-delay 2 -o "$dest.partial" "$url"
    mv "$dest.partial" "$dest"
  fi
  chmod a+x "$dest"
  local link="${HOME}/.local/bin/neo4j-desktop"
  mkdir -p "${HOME}/.local/bin"
  ln -sf "$dest" "$link"
  log "Neo4j Desktop ready: $link"
}

install_neo4j_desktop_macos() {
  if ! command -v brew >/dev/null 2>&1; then
    log "Installing Homebrew (non-interactive)…"
    NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" || true
  fi
  for b in /opt/homebrew/bin/brew /usr/local/bin/brew; do
    if [ -x "$b" ]; then
      eval "$("$b" shellenv)"
      break
    fi
  done
  if ! command -v brew >/dev/null 2>&1; then
    log "Homebrew not available; install Neo4j Desktop from https://neo4j.com/deployment-center/?desktop-gdb"
    return 0
  fi
  log "Installing Neo4j Desktop (Homebrew cask neo4j-desktop)…"
  if brew list --cask neo4j-desktop >/dev/null 2>&1; then
    brew upgrade --cask neo4j-desktop || true
  else
    brew install --cask neo4j-desktop
  fi
}

install_neo4j_desktop() {
  if [ "$SKIP_NEO4J_DESKTOP" = "1" ]; then
    log "Skipping Neo4j Desktop install (SKIP_NEO4J_DESKTOP=1)."
    return 0
  fi
  case "$(uname -s)" in
  Darwin) install_neo4j_desktop_macos ;;
  Linux) install_neo4j_desktop_linux_appimage ;;
  *) log "Skipping Neo4j Desktop auto-install on this OS." ;;
  esac
}
#endregion 🔖InstallNeo4jDesktop

#region 🔖GitSafe
configure_git_safe_directories() {
  git config --global --add safe.directory "$REPO_ROOT" || true
  if [ -f "$REPO_ROOT/.gitmodules" ]; then
    while IFS= read -r path; do
      [ -n "$path" ] || continue
      git config --global --add safe.directory "$REPO_ROOT/$path" || true
    done < <(git config -f "$REPO_ROOT/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
  fi
}
#endregion 🔖GitSafe

#region 🔖RepoBootstrap
repo_bootstrap() {
  if [ "$SKIP_REPO_BOOTSTRAP" = "1" ]; then
    log "Skipping repo bootstrap (SKIP_REPO_BOOTSTRAP=1)."
    return 0
  fi
  cd "$REPO_ROOT"
  export NX_WORKSPACE_DATA_DIRECTORY="${REPO_ROOT}/.nx/workspace-data-terminal"
  mkdir -p "$NX_WORKSPACE_DATA_DIRECTORY"
  configure_git_safe_directories
  ensure_bun
  log "bun install…"
  bun install
  log "workspace:setup…"
  bun nx run workspace:setup
}
#endregion 🔖RepoBootstrap

#region 🔖Main
cd "$REPO_ROOT"
configure_neo4j_shell_env
sync_codex_mcp_config
if [ "$SEMIO_SESSION_START" = "1" ]; then
  ensure_native_neo4j
  log "Native IDE session setup complete."
  exit 0
fi
install_neo4j_desktop
ensure_uv
ensure_native_neo4j
repo_bootstrap
log "Native (Unix) bootstrap complete. Open a new shell to load NEO4J_* from your profile, or run: export NEO4J_URI=bolt://localhost:7687 …"
#endregion 🔖Main
