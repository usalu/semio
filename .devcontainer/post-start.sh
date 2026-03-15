#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖PostStart
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"
#region 🔖EmojiFonts
configure_emoji_fonts() {
  sudo mkdir -p /etc/fonts
  sudo tee /etc/fonts/local.conf >/dev/null <<'FONTCONFIG'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <!-- Add emoji font families to generic font families -->
  <alias>
    <family>sans-serif</family>
    <prefer>
      <family>Noto Sans</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <alias>
    <family>serif</family>
    <prefer>
      <family>Noto Serif</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <alias>
    <family>monospace</family>
    <prefer>
      <family>Noto Sans Mono</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  
  <!-- Ensure emoji font is found for emoji characters -->
  <match target="pattern">
    <test name="lang">
      <string>en</string>
    </test>
    <test name="family">
      <string>emoji</string>
    </test>
    <edit name="family" mode="prepend">
      <string>Noto Color Emoji</string>
    </edit>
  </match>
  
  <!-- Force emoji rendering for color emoji -->
  <match target="pattern">
    <test name="family">
      <string>Noto Color Emoji</family>
    </test>
    <edit name="fontformat" mode="assign">
      <string>TrueType</string>
    </edit>
    <edit name="scalable" mode="assign">
      <bool>true</bool>
    </edit>
  </match>
</fontconfig>
FONTCONFIG
  sudo fc-cache -f
  echo "✅ Emoji font fallback configured."
}
#endregion 🔖EmojiFonts
#region 🔖Startup
#endregion 🔖Startup
#region 🔖Ownership
sudo chown -R vscode:vscode /home/vscode/.cache 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.claude 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.codex 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.codeium 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.gitkraken 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.local/share/GitKrakenCLI 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.local/share/gk 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.cursor-server 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.antigravity-server 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.vscode-server 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.windsurf-server 2>/dev/null || true
echo "✅ Fixed ownership for persisted volume mounts."
#endregion 🔖Ownership
#region 🔖EmojiFonts
configure_emoji_fonts
#endregion 🔖EmojiFonts
#region 🔖ClaudeAuth
CLAUDE_HOME="/home/vscode"
CLAUDE_DIR="${CLAUDE_HOME}/.claude"
CLAUDE_JSON="${CLAUDE_DIR}/.claude.json"
CLAUDE_JSON_BACKUP="${CLAUDE_DIR}/.claude.json.backup"
CLAUDE_JSON_LINK="${CLAUDE_HOME}/.claude.json"
CLAUDE_JSON_BACKUP_LINK="${CLAUDE_HOME}/.claude.json.backup"
mkdir -p "$CLAUDE_DIR"
if [ -f "$CLAUDE_JSON_LINK" ] && [ ! -L "$CLAUDE_JSON_LINK" ]; then
  if [ ! -f "$CLAUDE_JSON" ]; then
    mv "$CLAUDE_JSON_LINK" "$CLAUDE_JSON"
  else
    rm "$CLAUDE_JSON_LINK"
  fi
fi
if [ -f "$CLAUDE_JSON_BACKUP_LINK" ] && [ ! -L "$CLAUDE_JSON_BACKUP_LINK" ]; then
  if [ ! -f "$CLAUDE_JSON_BACKUP" ]; then
    mv "$CLAUDE_JSON_BACKUP_LINK" "$CLAUDE_JSON_BACKUP"
  else
    rm "$CLAUDE_JSON_BACKUP_LINK"
  fi
fi
if [ -f "$CLAUDE_JSON" ] && [ ! -e "$CLAUDE_JSON_LINK" ]; then
  ln -s "$CLAUDE_JSON" "$CLAUDE_JSON_LINK"
fi
if [ -f "$CLAUDE_JSON_BACKUP" ] && [ ! -e "$CLAUDE_JSON_BACKUP_LINK" ]; then
  ln -s "$CLAUDE_JSON_BACKUP" "$CLAUDE_JSON_BACKUP_LINK"
fi
echo "✅ Normalized Claude Code auth storage."
#endregion 🔖ClaudeAuth
#region 🔖GitOwnership
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    sudo chown -R vscode:vscode "$WORKSPACE/$path" 2>/dev/null || true
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi
echo "✅ Fixed ownership for workspace + submodules."
#endregion 🔖GitOwnership
#region 🔖GitSafe
git config --global --add safe.directory "$WORKSPACE"
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    git config --global --add safe.directory "$WORKSPACE/$path"
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi
echo "✅ Marked workspace + submodules as safe.directory for git."
#endregion 🔖GitSafe
#region 🔖PythonVenv
if [ -d "$WORKSPACE/.venv" ]; then
  source "$WORKSPACE/.venv/bin/activate"
fi
echo "✅ Activated Python virtual environment."
#endregion 🔖PythonVenv
echo "✅ Environment ready."
#endregion 🔖PostStart
