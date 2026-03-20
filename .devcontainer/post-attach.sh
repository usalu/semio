#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖PostAttach
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VSIX_PATH="repo/vscode/repo.vsix"
EXTENSION_PUBLISHER=""
EXTENSION_NAME=""
EXTENSION_ID=""
INSTALL_LOCK_FILE="/tmp/semio-post-attach-extension-install.lock"
WSL_ERROR="Command is only available in WSL or inside a Visual Studio Code terminal."
GITKRAKEN_WORKSPACE_NAME="${SEMIO_GITKRAKEN_WORKSPACE_NAME:-semio}"
SKIP_EXTENSION_INSTALL="${SEMIO_POST_ATTACH_SKIP_EXTENSION_INSTALL:-}"

#region 🔖GitKrakenCliHelpers
install_gitkraken_desktop() {
  if command -v gitkraken >/dev/null 2>&1; then
    return 0
  fi

  case "$(uname -m)" in
    x86_64|amd64)
      local gitkraken_arch="amd64"
      ;;
    aarch64|arm64)
      local gitkraken_arch="arm64"
      ;;
    *)
      echo "⚠️  Skipping GitKraken Desktop install for unsupported architecture: $(uname -m)"
      return 0
      ;;
  esac

  echo "Installing GitKraken Desktop for Linux..."
  mkdir -p "${HOME:-/home/vscode}/.gitkraken"
  local temp_dir
  temp_dir="$(mktemp -d)"
  trap 'rm -rf "$temp_dir"' RETURN
  curl -fsSL "https://release.gitkraken.com/linux/gitkraken-${gitkraken_arch}.deb" -o "$temp_dir/gitkraken.deb"
  sudo apt-get update
  sudo apt-get install -y "$temp_dir/gitkraken.deb"
  rm -rf "$temp_dir"
  trap - RETURN

  echo "GitKraken Desktop installed."
}

install_gitkraken_cli() {
  if [ -x "${HOME:-/home/vscode}/.local/bin/gk" ] && "${HOME:-/home/vscode}/.local/bin/gk" --version >/dev/null 2>&1; then
    return 0
  fi
  if command -v gk >/dev/null 2>&1 && gk --version >/dev/null 2>&1; then
    return 0
  fi

  case "$(uname -m)" in
    x86_64|amd64)
      local gitkraken_arch="amd64"
      ;;
    aarch64|arm64)
      local gitkraken_arch="arm64"
      ;;
    i386|i686)
      local gitkraken_arch="386"
      ;;
    *)
      echo "⚠️  Skipping GitKraken CLI install for unsupported architecture: $(uname -m)"
      return 0
      ;;
  esac

  echo "Installing GitKraken CLI..."
  local release_json
  release_json="$(curl -fsSL https://api.github.com/repos/gitkraken/gk-cli/releases/latest)"
  local download_url
  download_url="$(
    printf '%s' "$release_json" | jq -r \
      --arg suffix "linux_${gitkraken_arch}.zip" \
      '.assets[] | select(.name | endswith($suffix)) | .browser_download_url' | head -n 1
  )"
  if [ -z "$download_url" ] || [ "$download_url" = "null" ]; then
    echo "⚠️  Unable to resolve GitKraken CLI download URL for architecture: ${gitkraken_arch}"
    return 0
  fi

  mkdir -p "${HOME:-/home/vscode}/.local/bin" "${HOME:-/home/vscode}/.local/share/GitKrakenCLI" "${HOME:-/home/vscode}/.local/share/gk"
  local temp_dir
  temp_dir="$(mktemp -d)"
  trap 'rm -rf "$temp_dir"' RETURN
  curl -fsSL "$download_url" -o "$temp_dir/gk.zip"
  unzip -q "$temp_dir/gk.zip" -d "$temp_dir"
  install -m 0755 "$temp_dir/gk" "${HOME:-/home/vscode}/.local/bin/gk"
  rm -rf "$temp_dir"
  trap - RETURN

  "${HOME:-/home/vscode}/.local/bin/gk" --version >/dev/null 2>&1
  echo "GitKraken CLI installed."
}
#endregion 🔖GitKrakenCliHelpers

#region 🔖F3DHelpers
install_f3d() {
  if command -v f3d >/dev/null 2>&1; then
    return 0
  fi

  case "$(uname -m)" in
    x86_64|amd64)
      local f3d_arch="x86_64"
      ;;
    aarch64|arm64)
      local f3d_arch="arm64"
      ;;
    *)
      echo "⚠️  Skipping F3D install for unsupported architecture: $(uname -m)"
      return 0
      ;;
  esac

  echo "Installing F3D for Linux..."
  local temp_dir
  temp_dir="$(mktemp -d)"
  trap 'rm -rf "$temp_dir"' RETURN
  
  # Try to download the latest release
  local release_json
  release_json="$(curl -fsSL https://api.github.com/repos/f3d-app/f3d/releases/latest)"
  local download_url
  download_url="$(
    printf '%s' "$release_json" | jq -r \
      --arg suffix "Linux-${f3d_arch}.deb" \
      '.assets[] | select(.name | endswith($suffix)) | .browser_download_url' | head -n 1
  )"
  
  if [ -z "$download_url" ] || [ "$download_url" = "null" ]; then
    echo "⚠️  Unable to resolve F3D download URL for architecture: ${f3d_arch}, trying fallback..."
    # Fallback to a known version
    download_url="https://github.com/f3d-app/f3d/releases/download/v3.4.1/F3D-3.4.1-Linux-${f3d_arch}.deb"
  fi

  curl -fsSL "$download_url" -o "$temp_dir/f3d.deb"
  sudo apt-get update
  sudo apt-get install -y "$temp_dir/f3d.deb"
  rm -rf "$temp_dir"
  trap - RETURN

  echo "F3D installed."
}
#endregion 🔖F3DHelpers

#region 🔖DetectIDE
find_working_clis() {
  shopt -s nullglob
  clis=()
  add_cli_path() {
    local cli="$1"
    local existing=""
    if [ -z "$cli" ]; then
      return 0
    fi
    for existing in "${clis[@]}"; do
      if [ "$existing" = "$cli" ]; then
        return 0
      fi
    done
    clis+=("$cli")
  }
  for cli_env in ANTIGRAVITY_IPC_HOOK_CLI WINDSURF_IPC_HOOK_CLI CURSOR_IPC_HOOK_CLI VSCODE_IPC_HOOK_CLI; do
    cli_path="${!cli_env:-}"
    if [ -n "$cli_path" ]; then
      # Check for regular file AND executable (not sockets which also have -x)
      if [ -f "$cli_path" ] && [ -x "$cli_path" ]; then
        add_cli_path "$cli_path"
      fi
    fi
  done
  cli_order=()
  add_cli() {
    local cli="$1"
    local existing=""
    for existing in "${cli_order[@]}"; do
      if [ "$existing" = "$cli" ]; then
        return 0
      fi
    done
    cli_order+=("$cli")
  }
  if [ -n "${ANTIGRAVITY_IPC_HOOK_CLI:-}" ] || [ "${TERM_PROGRAM:-}" = "antigravity" ]; then
    add_cli "antigravity"
  fi
  if [ -n "${WINDSURF_IPC_HOOK_CLI:-}" ] || [ "${TERM_PROGRAM:-}" = "windsurf" ]; then
    add_cli "windsurf"
  fi
  if [ -n "${CURSOR_IPC_HOOK_CLI:-}" ] || [ "${TERM_PROGRAM:-}" = "cursor" ]; then
    add_cli "cursor"
  fi
  if [ -n "${VSCODE_IPC_HOOK_CLI:-}" ] || [ "${TERM_PROGRAM:-}" = "vscode" ]; then
    add_cli "code"
    add_cli "code-insiders"
  fi
  add_cli "antigravity"
  add_cli "windsurf"
  add_cli "cursor"
  add_cli "code-insiders"
  add_cli "code"
  for cmd in "${cli_order[@]}"; do
    for bin in $(which -a "$cmd" 2>/dev/null); do
      if "$bin" --version >/dev/null 2>&1; then
        add_cli_path "$bin"
      fi
    done
  done
  for cmd in "${cli_order[@]}"; do
    if [ "$cmd" = "antigravity" ]; then
      for bin in /home/vscode/.antigravity-server/bin/*/bin/remote-cli/antigravity; do
        if [ -x "$bin" ]; then
          if "$bin" --version >/dev/null 2>&1; then
            add_cli_path "$bin"
          fi
        fi
      done
    fi
    if [ "$cmd" = "windsurf" ]; then
      for bin in /home/vscode/.windsurf-server/bin/*/bin/remote-cli/windsurf; do
        if [ -x "$bin" ]; then
          if "$bin" --version >/dev/null 2>&1; then
            add_cli_path "$bin"
          fi
        fi
      done
    fi
    if [ "$cmd" = "cursor" ]; then
      for bin in /home/vscode/.cursor-server/bin/*/bin/remote-cli/cursor; do
        if [ -x "$bin" ]; then
          if "$bin" --version >/dev/null 2>&1; then
            add_cli_path "$bin"
          fi
        fi
      done
    fi
    if [ "$cmd" = "code" ] || [ "$cmd" = "code-insiders" ]; then
      for bin in /home/vscode/.vscode-server/bin/*/bin/remote-cli/code; do
        if [ -x "$bin" ]; then
          if "$bin" --version >/dev/null 2>&1; then
            add_cli_path "$bin"
          fi
        fi
      done
      for bin in /home/vscode/.vscode-server-insiders/bin/*/bin/remote-cli/code-insiders; do
        if [ -x "$bin" ]; then
          if "$bin" --version >/dev/null 2>&1; then
            add_cli_path "$bin"
          fi
        fi
      done
    fi
  done
  for bin in "${clis[@]}"; do
    echo "$bin"
  done
  shopt -u nullglob
  return 0
}

if [ -n "$SKIP_EXTENSION_INSTALL" ]; then
  IDE_CLIS=()
else
  mapfile -t IDE_CLIS < <(find_working_clis)
fi
#endregion 🔖DetectIDE

#region 🔖GitKrakenWorkspace
collect_gitkraken_workspace_repos() {
  local repos=()
  local path=""

  append_repo() {
    local repo_path="$1"
    local existing=""
    if [ -z "$repo_path" ] || [ ! -d "$repo_path" ]; then
      return 0
    fi
    for existing in "${repos[@]}"; do
      if [ "$existing" = "$repo_path" ]; then
        return 0
      fi
    done
    repos+=("$repo_path")
  }

  if git -C "$WORKSPACE" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    append_repo "$WORKSPACE"
  fi
  if [ -f "$WORKSPACE/.gitmodules" ]; then
    while IFS= read -r path; do
      [ -n "$path" ] || continue
      if git -C "$WORKSPACE/$path" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        append_repo "$WORKSPACE/$path"
      fi
    done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
  fi
  printf '%s\n' "${repos[@]}"
}

configure_gitkraken_workspace() {
  if ! command -v gk >/dev/null 2>&1; then
    echo "⚠️  GitKraken CLI not available, skipping GitKraken workspace bootstrap."
    return 0
  fi

  # Skip workspace configuration if authentication is needed
  if ! gk auth status >/dev/null 2>&1; then
    echo "⚠️  GitKraken CLI not authenticated, skipping workspace bootstrap."
    return 0
  fi

  mapfile -t gitkraken_repos < <(collect_gitkraken_workspace_repos)
  if [ "${#gitkraken_repos[@]}" -eq 0 ]; then
    echo "⚠️  No git repositories found for GitKraken workspace bootstrap."
    return 0
  fi

  local info_output=""
  local repo=""
  local missing_repos=()
  local repo_csv=""
  local missing_csv=""
  local created=""
  local has_workspace=""

  info_output="$(gk ws info "$GITKRAKEN_WORKSPACE_NAME" 2>/dev/null || true)"
  if [ -z "$(printf '%s' "$info_output" | tr -d '[:space:]')" ]; then
    has_workspace=""
  elif printf '%s\n' "$info_output" | grep -Fq "you do not have any workspaces"; then
    has_workspace=""
  elif printf '%s\n' "$info_output" | grep -Fq "no workspace with name"; then
    has_workspace=""
  else
    has_workspace="1"
  fi

  if [ -n "$has_workspace" ]; then
    for repo in "${gitkraken_repos[@]}"; do
      if ! printf '%s\n' "$info_output" | grep -Fq "$repo"; then
        missing_repos+=("$repo")
      fi
    done
    if [ "${#missing_repos[@]}" -gt 0 ]; then
      missing_csv="$(IFS=,; printf '%s' "${missing_repos[*]}")"
      gk ws update "$GITKRAKEN_WORKSPACE_NAME" --add-repos "$missing_csv" >/dev/null 2>&1 || true
      echo "✅ GitKraken workspace updated: $GITKRAKEN_WORKSPACE_NAME"
    else
      echo "✅ GitKraken workspace already current: $GITKRAKEN_WORKSPACE_NAME"
    fi
  else
    repo_csv="$(IFS=,; printf '%s' "${gitkraken_repos[*]}")"
    gk ws create "$GITKRAKEN_WORKSPACE_NAME" --add-repos "$repo_csv" >/dev/null 2>&1 || true
    created="1"
    echo "✅ GitKraken workspace created: $GITKRAKEN_WORKSPACE_NAME"
  fi

  if [ -n "$created" ] || [ "${#missing_repos[@]}" -gt 0 ]; then
    gk ws refresh "$GITKRAKEN_WORKSPACE_NAME" >/dev/null 2>&1 || true
  fi
  gk ws set "$GITKRAKEN_WORKSPACE_NAME" >/dev/null 2>&1 || true
  echo "✅ GitKraken default workspace set: $GITKRAKEN_WORKSPACE_NAME"
}
#endregion 🔖GitKrakenWorkspace

install_gitkraken_desktop
install_gitkraken_cli
configure_gitkraken_workspace
install_f3d

#region 🔖GitKrakenAutoStart
start_gitkraken_if_needed() {
  if ! command -v gitkraken >/dev/null 2>&1; then
    echo "⚠️  GitKraken Desktop not available, skipping auto-start."
    return 0
  fi

  # Check if GitKraken is already running
  if pgrep -f "gitkraken" >/dev/null 2>&1; then
    echo "✅ GitKraken is already running."
    return 0
  fi

  # Set up virtual display if needed
  if [ -z "$DISPLAY" ]; then
    export DISPLAY=:99
  fi

  # Start Xvfb if not running
  if ! pgrep -f "Xvfb.*:99" >/dev/null 2>&1; then
    echo "🖥️  Starting virtual display..."
    Xvfb :99 -screen 0 1920x1080x24 -ac +extension GLX +render -noreset >/dev/null 2>&1 &
    sleep 2
  fi

  # Check if we're in WSL environment
  if grep -q "Microsoft\|WSL" /proc/version 2>/dev/null; then
    echo "🚀 Starting GitKraken with WSL-compatible flags..."
    # Start GitKraken with no-sandbox and no-debug flags for WSL compatibility
    gitkraken --no-sandbox --no-debug --path "$WORKSPACE" >/dev/null 2>&1 &
    echo "✅ GitKraken started in background."
  else
    echo "🚀 Starting GitKraken..."
    gitkraken --no-debug --no-sandbox --path "$WORKSPACE" >/dev/null 2>&1 &
    echo "✅ GitKraken started in background."
  fi
}

# Only auto-start if not explicitly disabled
if [ "${SEMIO_GITKRAKEN_AUTO_START:-true}" = "true" ]; then
  start_gitkraken_if_needed
fi
#endregion 🔖GitKrakenAutoStart

#region 🔖F3DAutoStart
start_f3d_if_needed() {
  if ! command -v f3d >/dev/null 2>&1; then
    echo "⚠️  F3D not available, skipping auto-start."
    return 0
  fi

  # Check if F3D is already running
  if pgrep -f "f3d" >/dev/null 2>&1; then
    echo "✅ F3D is already running."
    return 0
  fi

  # Check if display is available
  if [ -z "$DISPLAY" ]; then
    echo "⚠️  No display available, skipping F3D auto-start. F3D requires a GUI environment."
    echo "💡 To use F3D, please run in a GUI environment or set up X11 forwarding."
    return 0
  fi

  # Check if we're in WSL environment
  if grep -q "Microsoft\|WSL" /proc/version 2>/dev/null; then
    echo "🚀 Starting F3D with WSL-compatible flags..."
    # Start F3D with no-sandbox for WSL compatibility
    f3d --no-sandbox >/dev/null 2>&1 &
    echo "✅ F3D started in background."
  else
    echo "🚀 Starting F3D..."
    f3d >/dev/null 2>&1 &
    echo "✅ F3D started in background."
  fi
}

# Only auto-start if not explicitly disabled
if [ "${SEMIO_F3D_AUTO_START:-true}" = "true" ]; then
  start_f3d_if_needed
fi
#endregion 🔖F3DAutoStart

#region 🔖McpConfig
sync_mcp_client_configs() {
  local home_dir="${HOME:-/home/vscode}"
  local repo_root="${REPO_ROOT:-/workspaces/semio}"
  local source_path="${repo_root}/.mcp.json"

  if [ ! -f "$source_path" ]; then
    echo "⚠️  MCP config source not found at $source_path, skipping client MCP bootstrap."
    return 0
  fi

  node - "$repo_root" "$home_dir" <<'EOF'
const fs = require("fs");
const path = require("path");

const [repoRoot, homeDir] = process.argv.slice(2);
const sourcePath = path.join(repoRoot, ".mcp.json");
const sourceData = JSON.parse(fs.readFileSync(sourcePath, "utf8"));
const sourceServers = sourceData.mcpServers ?? {};

function absolutizeValue(value) {
  if (typeof value !== "string" || value.length === 0) {
    return value;
  }
  if (value.startsWith("./")) {
    return path.join(repoRoot, value.slice(2));
  }
  return value;
}

function normalizeServer(server) {
  const sourceArgs = Array.isArray(server.args) ? server.args : [];
  return {
    ...server,
    command: absolutizeValue(server.command),
    args: sourceArgs.map((arg, index) => {
      if (typeof arg !== "string") {
        return arg;
      }
      const previousArg = index > 0 ? sourceArgs[index - 1] : "";
      if ((previousArg === "--directory" || previousArg === "-C") && !path.isAbsolute(arg)) {
        return path.join(repoRoot, arg);
      }
      return absolutizeValue(arg);
    }),
  };
}

function escapeTomlString(value) {
  return String(value).replace(/\\/g, "\\\\").replace(/"/g, '\\"');
}

const normalizedServers = Object.fromEntries(
  Object.entries(sourceServers).map(([name, server]) => [name, normalizeServer(server)]),
);

const windsurfDir = path.join(homeDir, ".codeium", "windsurf");
fs.mkdirSync(windsurfDir, { recursive: true });
fs.writeFileSync(
  path.join(windsurfDir, "mcp_config.json"),
  `${JSON.stringify({ mcpServers: normalizedServers }, null, 2)}\n`,
);

const codexDir = path.join(homeDir, ".codex");
fs.mkdirSync(codexDir, { recursive: true });
const codexConfigPath = path.join(codexDir, "config.toml");
const existingCodexText = fs.existsSync(codexConfigPath)
  ? fs.readFileSync(codexConfigPath, "utf8")
  : "";

function stripManagedCodexMcpBlocks(configText) {
  const lines = configText.split(/\r?\n/);
  const keptLines = [];
  let skippingManagedBlock = false;

  for (const line of lines) {
    const trimmedLine = line.trim();
    const isTableHeader = /^\[[^\]]+\]\s*$/.test(trimmedLine);

    if (
      trimmedLine === "# Codex MCP Server Configuration" ||
      trimmedLine === `# Generated from ${sourcePath} by .devcontainer/post-attach.sh`
    ) {
      continue;
    }

    if (/^\[mcp_servers\.[^\]]+\]\s*$/.test(trimmedLine)) {
      skippingManagedBlock = true;
      continue;
    }

    if (skippingManagedBlock && isTableHeader) {
      skippingManagedBlock = false;
    }

    if (!skippingManagedBlock) {
      keptLines.push(line);
    }
  }

  return keptLines.join("\n").replace(/\s+$/, "");
}

const preservedCodexText = stripManagedCodexMcpBlocks(existingCodexText);
const codexLines = [];
if (preservedCodexText.length > 0) {
  codexLines.push(preservedCodexText, "");
}
codexLines.push(
  "# Codex MCP Server Configuration",
  `# Generated from ${sourcePath} by .devcontainer/post-attach.sh`,
  "",
);
for (const [name, server] of Object.entries(normalizedServers)) {
  codexLines.push(`[mcp_servers.${name}]`);
  codexLines.push(`command = "${escapeTomlString(server.command)}"`);
  codexLines.push(`cwd = "${escapeTomlString(repoRoot)}"`);
  if (Array.isArray(server.args) && server.args.length > 0) {
    codexLines.push(
      `args = [${server.args
        .map((arg) => `"${escapeTomlString(arg)}"`)
        .join(", ")}]`,
    );
  } else {
    codexLines.push("args = []");
  }
  codexLines.push(`enabled = ${server.disabled === true ? "false" : "true"}`);
  codexLines.push("");
}
fs.writeFileSync(codexConfigPath, `${codexLines.join("\n")}\n`);
EOF

  echo "✅ Synced MCP configs for Windsurf and Codex."
}
#endregion 🔖McpConfig

#region 🔖VSCodeChatPersistence
merge_dir_contents_if_present() {
  local source_dir="$1"
  local target_dir="$2"
  if [ ! -d "$source_dir" ]; then
    return 0
  fi
  mkdir -p "$target_dir"
  cp -a -n "$source_dir"/. "$target_dir"/ 2>/dev/null || true
}

reconcile_vscode_chat_workspace_storage() {
  local storage_root="${HOME:-/home/vscode}/.vscode-server/data/User/workspaceStorage"
  local provider=""
  local provider_dir=""
  local active_workspace=""
  local target_provider_dir=""
  local source_provider_dir=""
  local active_dirs=()
  local provider_sources=()
  local provider_names=(
    "GitHub.copilot-chat"
    "openai.chatgpt"
  )

  if [ ! -d "$storage_root" ]; then
    return 0
  fi

  while IFS= read -r active_workspace; do
    [ -n "$active_workspace" ] || continue
    active_dirs+=("$active_workspace")
  done < <(find "$storage_root" -mindepth 1 -maxdepth 1 -type d -exec test -e '{}/vscode.lock' ';' -print | sort)

  if [ "${#active_dirs[@]}" -eq 0 ]; then
    return 0
  fi

  for provider in "${provider_names[@]}"; do
    provider_sources=()
    while IFS= read -r provider_dir; do
      [ -n "$provider_dir" ] || continue
      provider_sources+=("$provider_dir")
    done < <(find "$storage_root" -mindepth 2 -maxdepth 2 -type d -name "$provider" | sort)

    if [ "${#provider_sources[@]}" -lt 2 ]; then
      continue
    fi

    for active_workspace in "${active_dirs[@]}"; do
      target_provider_dir="${active_workspace}/${provider}"
      if [ ! -d "$target_provider_dir" ]; then
        for source_provider_dir in "${provider_sources[@]}"; do
          if [ "$source_provider_dir" != "$target_provider_dir" ]; then
            mkdir -p "$active_workspace"
            cp -a "$source_provider_dir" "$target_provider_dir" 2>/dev/null || true
            break
          fi
        done
      fi

      for source_provider_dir in "${provider_sources[@]}"; do
        if [ "$source_provider_dir" = "$target_provider_dir" ]; then
          continue
        fi
        merge_dir_contents_if_present "${source_provider_dir}/transcripts" "${target_provider_dir}/transcripts"
        merge_dir_contents_if_present "${source_provider_dir}/chat-session-resources" "${target_provider_dir}/chat-session-resources"
      done
    done
  done

  echo "✅ Reconciled persisted VS Code chat workspace storage."
}
#endregion 🔖VSCodeChatPersistence

#region 🔖InstallExtension
if [ "${#IDE_CLIS[@]}" -gt 0 ]; then
  exec 201>"$INSTALL_LOCK_FILE"
  if flock -w 180 201; then  # Increased timeout from 120 to 180 seconds
    needs_rebuild=""
    if [ ! -f "$VSIX_PATH" ]; then
      needs_rebuild="1"
      echo "📦 Extension VSIX not found, rebuilding..."
    else
      for src in repo/vscode/extension.ts repo/vscode/queries.ts repo/vscode/package.json repo/vscode/vite.config.ts; do
        if [ -f "$src" ] && [ "$VSIX_PATH" -ot "$src" ]; then
          needs_rebuild="1"
          echo "📦 Extension source updated, rebuilding..."
          break
        fi
      done
    fi
    if [ -n "$needs_rebuild" ]; then
      echo "🔨 Building semio VSCode extension..."
      if (cd repo/vscode && npm run package); then
        echo "✅ Extension build completed."
      else
        echo "⚠️  Extension build failed, continuing without extension install."
        flock -u 201
        exec 201>&-
        # Continue with other setup even if extension build fails
      fi
    fi
    if [ -f "repo/vscode/package.json" ]; then
      EXTENSION_PUBLISHER=$(node -p "require('./repo/vscode/package.json').publisher" 2>/dev/null || echo "")
      EXTENSION_NAME=$(node -p "require('./repo/vscode/package.json').name" 2>/dev/null || echo "")
    fi
    if [ -n "$EXTENSION_PUBLISHER" ] && [ -n "$EXTENSION_NAME" ]; then
      EXTENSION_ID="${EXTENSION_PUBLISHER}.${EXTENSION_NAME}"
    fi
    if [ -f "$VSIX_PATH" ]; then
      installed_any=""
      for ide_cli in "${IDE_CLIS[@]}"; do
        echo "📦 Trying to install extension via $ide_cli..."
        install_output="$("$ide_cli" --install-extension "$VSIX_PATH" --force 2>&1)" || true
        if echo "$install_output" | grep -Fq "$WSL_ERROR"; then
          echo "⚠️  $ide_cli not available from WSL, trying next CLI..."
          continue
        fi
        if [ -n "$EXTENSION_ID" ]; then
          list_output="$("$ide_cli" --list-extensions 2>&1)" || true
          if echo "$list_output" | grep -Fq "$WSL_ERROR"; then
            echo "⚠️  $ide_cli not available from WSL, trying next CLI..."
            continue
          fi
          if echo "$list_output" | grep -Fqx "$EXTENSION_ID"; then
            echo "✅ Extension installed via $ide_cli"
            installed_any="1"
            break  # Success, no need to try other CLIs
          fi
        else
          echo "✅ Extension installed via $ide_cli"
          installed_any="1"
          break  # Success, no need to try other CLIs
        fi
      done
      if [ -z "$installed_any" ]; then
        echo "⚠️  No IDE CLI could install the extension from this attach session."
        echo "💡 You may need to install the extension manually from the VSIX file: $VSIX_PATH"
      fi
    else
      echo "⚠️  Extension file not found at $VSIX_PATH"
    fi
    flock -u 201
  else
    echo "⚠️  Timed out waiting for extension install lock, skipping extension install for this attach."
  fi
  exec 201>&-
else
  echo "ℹ️  No IDE CLIs detected, skipping extension installation."
fi
#endregion 🔖InstallExtension

#region 🔖McpConfig
sync_mcp_client_configs
reconcile_vscode_chat_workspace_storage
#endregion 🔖McpConfig

echo "✅ Post-attach setup complete."
#endregion 🔖PostAttach
