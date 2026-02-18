#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖PostAttach
set -e
VSIX_PATH="semio-repo/vscode/semio-repo.vsix"
EXTENSION_PUBLISHER=""
EXTENSION_NAME=""
EXTENSION_ID=""
INSTALL_LOCK_FILE="/tmp/semio-post-attach-extension-install.lock"
WSL_ERROR="Command is only available in WSL or inside a Visual Studio Code terminal."

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
      for bin in /vscode/vscode-server/bin/linux-x64/*/bin/remote-cli/code; do
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

mapfile -t IDE_CLIS < <(find_working_clis)
#endregion 🔖DetectIDE

#region 🔖InstallExtension
if [ "${#IDE_CLIS[@]}" -gt 0 ]; then
  exec 201>"$INSTALL_LOCK_FILE"
  if flock -w 120 201; then
    if [ ! -f "$VSIX_PATH" ] || [ "$VSIX_PATH" -ot "semio-repo/vscode/extension.ts" ]; then
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

#region 🔖WindsurfMcpConfig
WINDSURF_MCP_DIR="/home/vscode/.codeium/windsurf"
WINDSURF_MCP_FILE="${WINDSURF_MCP_DIR}/mcp_config.json"
mkdir -p "$WINDSURF_MCP_DIR"
cat > "$WINDSURF_MCP_FILE" <<'EOF'
{
    "mcpServers": {
        "semio-repo": {
            "command": "./semio-repo/cli/cli",
            "args": [
                "mcp"
            ]
        }
    }
}
EOF
#endregion 🔖WindsurfMcpConfig

echo "✅ Post-attach setup complete."
#endregion 🔖PostAttach
