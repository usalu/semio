#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region PostStart
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"
#region Startup
echo "🔄 Starting semio development environment..."
#endregion Startup
#region Ownership
echo "🔐 Fixing ownership for persisted volume mounts..."
sudo chown -R vscode:vscode /home/vscode/.claude 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.codex 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/gh 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/cursor 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/antigravity 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/openai 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.codeium 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.vscode-server 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.windsurf-server 2>/dev/null || true
#endregion Ownership
#region ClaudeAuth
echo "🔐 Normalizing Claude Code auth storage..."
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
#endregion ClaudeAuth
#region GitOwnership
echo "🔐 Fixing ownership for workspace + submodules (prevents git dubious ownership)..."
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    sudo chown -R vscode:vscode "$WORKSPACE/$path" 2>/dev/null || true
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi
#endregion GitOwnership
#region GitSafe
echo "✅ Marking workspace + submodules as safe.directory for git..."
git config --global --add safe.directory "$WORKSPACE"
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    git config --global --add safe.directory "$WORKSPACE/$path"
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi
#endregion GitSafe
#region PythonVenv
echo "🐍 Activating Python virtual environment..."
if [ -d "$WORKSPACE/.venv" ]; then
  source "$WORKSPACE/.venv/bin/activate"
fi
#endregion PythonVenv
echo "✅ Environment ready!"
#endregion PostStart
