#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region PostAttach
set -e
echo "🔌 Running post-attach setup..."
VSIX_PATH="js/vscode/semio.vsix"

#region DetectIDE
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
      if [ -x "$cli_path" ]; then
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
  return 1
}

mapfile -t IDE_CLIS < <(find_working_clis)
#endregion DetectIDE

#region InstallExtension
if [ "${#IDE_CLIS[@]}" -gt 0 ]; then
  echo "📦 IDE CLIs detected, installing semio extension..."
  if [ ! -f "$VSIX_PATH" ] || [ "$VSIX_PATH" -ot "js/vscode/extension.ts" ]; then
    echo "📦 Building semio VS Code extension..."
    (cd js/vscode && npm run package)
  fi
  if [ -f "$VSIX_PATH" ]; then
    installed_any=""
    for ide_cli in "${IDE_CLIS[@]}"; do
      if "$ide_cli" --install-extension "$VSIX_PATH" --force; then
        echo "✅ Extension installed via $ide_cli"
        installed_any="1"
      else
        echo "❌ Failed to install extension via $ide_cli"
      fi
    done
    if [ -z "$installed_any" ]; then
      exit 1
    fi
  else
    echo "⚠️  Extension file not found at $VSIX_PATH"
    echo "   Extension build failed; check npm output"
    exit 1
  fi
else
  echo "ℹ️  No working IDE CLI found (antigravity, windsurf, cursor, code, code-insiders)"
  echo "   Extension installation skipped - this is expected outside supported IDEs"
fi
#endregion InstallExtension

echo "✅ Post-attach setup complete!"
#endregion PostAttach
