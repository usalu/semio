#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region PostAttach
set -e
echo "🔌 Running post-attach setup..."
VSIX_PATH="js/vscode/semio.vsix"
EXTENSION_PUBLISHER=""
EXTENSION_NAME=""
EXTENSION_ID=""
EXTENSION_VERSION=""
EXTENSION_DIR_NAME=""
WSL_ERROR="Command is only available in WSL or inside a Visual Studio Code terminal."

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
    if [ -f "js/vscode/package.json" ]; then
      EXTENSION_PUBLISHER=$(node -p "require('./js/vscode/package.json').publisher")
      EXTENSION_NAME=$(node -p "require('./js/vscode/package.json').name")
      EXTENSION_VERSION=$(node -p "require('./js/vscode/package.json').version")
    fi
    if [ -n "$EXTENSION_PUBLISHER" ] && [ -n "$EXTENSION_NAME" ]; then
      EXTENSION_ID="${EXTENSION_PUBLISHER}.${EXTENSION_NAME}"
    fi
    if [ -n "$EXTENSION_ID" ] && [ -n "$EXTENSION_VERSION" ]; then
      EXTENSION_DIR_NAME="${EXTENSION_ID}-${EXTENSION_VERSION}"
    fi
    installed_any=""
    for ide_cli in "${IDE_CLIS[@]}"; do
      install_output="$("$ide_cli" --install-extension "$VSIX_PATH" --force 2>&1)"
      install_status=$?
      if echo "$install_output" | grep -Fq "$WSL_ERROR"; then
        echo "❌ Failed to install extension via $ide_cli"
        echo "$install_output"
        continue
      fi
      if [ "$install_status" -ne 0 ]; then
        echo "❌ Failed to install extension via $ide_cli"
        echo "$install_output"
        continue
      fi
      if [ -n "$EXTENSION_ID" ]; then
        list_output="$("$ide_cli" --list-extensions 2>&1)"
        list_status=$?
        if echo "$list_output" | grep -Fq "$WSL_ERROR"; then
          echo "❌ Failed to verify extension via $ide_cli"
          echo "$list_output"
          continue
        fi
        if [ "$list_status" -ne 0 ]; then
          echo "❌ Failed to verify extension via $ide_cli"
          echo "$list_output"
          continue
        fi
        if echo "$list_output" | grep -Fqx "$EXTENSION_ID"; then
          echo "✅ Extension installed via $ide_cli"
          installed_any="1"
        else
          echo "❌ Extension not visible via $ide_cli"
          echo "$list_output"
        fi
      else
        echo "✅ Extension installed via $ide_cli"
        installed_any="1"
      fi
    done
    if [ -n "$EXTENSION_DIR_NAME" ]; then
      extensions_dirs=()
      if [ -d "/home/vscode/.vscode-server/extensions" ]; then
        extensions_dirs+=("/home/vscode/.vscode-server/extensions")
      fi
      if [ -d "/home/vscode/.cursor-server/extensions" ]; then
        extensions_dirs+=("/home/vscode/.cursor-server/extensions")
      fi
      if [ -d "/home/vscode/.windsurf-server/extensions" ]; then
        extensions_dirs+=("/home/vscode/.windsurf-server/extensions")
      fi
      if [ -d "/home/vscode/.antigravity-server/extensions" ]; then
        extensions_dirs+=("/home/vscode/.antigravity-server/extensions")
      fi
      if [ "${#extensions_dirs[@]}" -gt 0 ]; then
        temp_dir="$(mktemp -d)"
        unzip -q "$VSIX_PATH" -d "$temp_dir"
        for extensions_dir in "${extensions_dirs[@]}"; do
          target_dir="${extensions_dir}/${EXTENSION_DIR_NAME}"
          if [ -d "$target_dir" ]; then
            rm -rf "$target_dir"
          fi
          if [ -d "$temp_dir/extension" ]; then
            mkdir -p "$extensions_dir"
            cp -R "$temp_dir/extension" "$target_dir"
            if [ -d "$target_dir" ]; then
              extensions_json="${extensions_dir}/extensions.json"
              node -e "const fs=require('fs');const id=process.argv[1];const version=process.argv[2];const dir=process.argv[3];const rel=process.argv[4];const file=process.argv[5];const list=fs.existsSync(file)?JSON.parse(fs.readFileSync(file,'utf8')):[];const exists=list.some(e=>e&&e.identifier&&e.identifier.id===id&&e.version===version);if(!exists){list.push({identifier:{id},version,location:{'$mid':1,path:dir,scheme:'file'},relativeLocation:rel,metadata:{isApplicationScoped:true,isMachineScoped:false,isBuiltin:false,installedTimestamp:Date.now(),source:'vsix',private:true}});}fs.writeFileSync(file,JSON.stringify(list));" "$EXTENSION_ID" "$EXTENSION_VERSION" "$target_dir" "$EXTENSION_DIR_NAME" "$extensions_json"
              echo "✅ Extension installed via $extensions_dir"
              installed_any="1"
            fi
          fi
        done
        rm -rf "$temp_dir"
      fi
    fi
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
