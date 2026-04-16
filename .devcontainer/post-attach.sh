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
SKIP_TOOL_INSTALL="${SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL:-}"

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

if [ -n "$SKIP_TOOL_INSTALL" ]; then
  echo "ℹ️  Tool installation skipped."
else
  install_gitkraken_desktop
  install_gitkraken_cli
fi
configure_gitkraken_workspace
if [ -z "$SKIP_TOOL_INSTALL" ]; then
  install_f3d
fi

#region 🔖RepoConfigure
echo "🔧 Syncing repo hook and MCP configuration..."
if [ -x "$REPO_ROOT/repo/client/client" ]; then
  if "$REPO_ROOT/repo/client/client" configure --repo "$REPO_ROOT"; then
    echo "✅ Repo hook and MCP configuration synced."
  else
    echo "⚠️  Repo configure failed, continuing without blocking attach."
  fi
else
  echo "⚠️  Repo CLI binary not found, skipping repo configure."
fi
#endregion 🔖RepoConfigure

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


echo "✅ Post-attach setup complete."
#endregion 🔖PostAttach
