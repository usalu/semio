#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖PostAttach
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VSIX_PATH="semio-repo/vscode/semio-repo.vsix"
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
      gk ws update "$GITKRAKEN_WORKSPACE_NAME" --add-repos "$missing_csv" >/dev/null
      echo "✅ GitKraken workspace updated: $GITKRAKEN_WORKSPACE_NAME"
    else
      echo "✅ GitKraken workspace already current: $GITKRAKEN_WORKSPACE_NAME"
    fi
  else
    repo_csv="$(IFS=,; printf '%s' "${gitkraken_repos[*]}")"
    gk ws create "$GITKRAKEN_WORKSPACE_NAME" --add-repos "$repo_csv" >/dev/null
    created="1"
    echo "✅ GitKraken workspace created: $GITKRAKEN_WORKSPACE_NAME"
  fi

  if [ -n "$created" ] || [ "${#missing_repos[@]}" -gt 0 ]; then
    gk ws refresh "$GITKRAKEN_WORKSPACE_NAME" >/dev/null 2>&1 || true
  fi
  gk ws set "$GITKRAKEN_WORKSPACE_NAME" >/dev/null
  echo "✅ GitKraken default workspace set: $GITKRAKEN_WORKSPACE_NAME"
}
#endregion 🔖GitKrakenWorkspace

install_gitkraken_desktop
install_gitkraken_cli
configure_gitkraken_workspace

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
const codexLines = [
  "# Codex MCP Server Configuration",
  "# Generated from /workspaces/semio/.mcp.json by .devcontainer/post-attach.sh",
  "",
];
for (const [name, server] of Object.entries(normalizedServers)) {
  codexLines.push(`[mcp_servers.${name}]`);
  codexLines.push(`command = "${escapeTomlString(server.command)}"`);
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
fs.writeFileSync(path.join(codexDir, "config.toml"), `${codexLines.join("\n")}\n`);
EOF

  echo "✅ Synced MCP configs for Windsurf and Codex."
}
#endregion 🔖McpConfig

#region 🔖InstallExtension
if [ "${#IDE_CLIS[@]}" -gt 0 ]; then
  exec 201>"$INSTALL_LOCK_FILE"
  if flock -w 120 201; then
    needs_rebuild=""
    if [ ! -f "$VSIX_PATH" ]; then
      needs_rebuild="1"
    else
      for src in semio-repo/vscode/extension.ts semio-repo/vscode/queries.ts semio-repo/vscode/package.json semio-repo/vscode/vite.config.ts; do
        if [ -f "$src" ] && [ "$VSIX_PATH" -ot "$src" ]; then
          needs_rebuild="1"
          break
        fi
      done
    fi
    if [ -n "$needs_rebuild" ]; then
      (cd semio-repo/vscode && npm run package)
    fi
    if [ -f "semio-repo/vscode/package.json" ]; then
      EXTENSION_PUBLISHER=$(node -p "require('./semio-repo/vscode/package.json').publisher")
      EXTENSION_NAME=$(node -p "require('./semio-repo/vscode/package.json').name")
    fi
    if [ -n "$EXTENSION_PUBLISHER" ] && [ -n "$EXTENSION_NAME" ]; then
      EXTENSION_ID="${EXTENSION_PUBLISHER}.${EXTENSION_NAME}"
    fi
    if [ -f "$VSIX_PATH" ]; then
      installed_any=""
      for ide_cli in "${IDE_CLIS[@]}"; do
        install_output="$("$ide_cli" --install-extension "$VSIX_PATH" --force 2>&1)" || true
        if echo "$install_output" | grep -Fq "$WSL_ERROR"; then
          continue
        fi
        if [ -n "$EXTENSION_ID" ]; then
          list_output="$("$ide_cli" --list-extensions 2>&1)" || true
          if echo "$list_output" | grep -Fq "$WSL_ERROR"; then
            continue
          fi
          if echo "$list_output" | grep -Fqx "$EXTENSION_ID"; then
            echo "✅ Extension installed via $ide_cli"
            installed_any="1"
          fi
        else
          echo "✅ Extension installed via $ide_cli"
          installed_any="1"
        fi
      done
      if [ -z "$installed_any" ]; then
        echo "⚠️  No IDE CLI could install the extension from this attach session."
      fi
    else
      echo "⚠️  Extension file not found at $VSIX_PATH"
    fi
    flock -u 201
  else
    echo "⚠️  Timed out waiting for extension install lock, skipping extension install for this attach."
  fi
  exec 201>&-
fi
#endregion 🔖InstallExtension

#region 🔖McpConfig
sync_mcp_client_configs
#endregion 🔖McpConfig

echo "✅ Post-attach setup complete."
#endregion 🔖PostAttach
