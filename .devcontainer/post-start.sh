#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
set -e

WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"

echo "🔄 Starting semio development environment..."

echo "🔐 Fixing ownership for persisted volume mounts..."
# Claude Code and Codex volumes mount as root - fix ownership for vscode user
sudo chown -R vscode:vscode /home/vscode/.claude 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.codex 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/gh 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/cursor 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/antigravity 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.codeium 2>/dev/null || true

echo "🔐 Fixing ownership for workspace + submodules (prevents git dubious ownership)..."
# Workspace itself is usually fine, but submodules sometimes aren't.
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    sudo chown -R vscode:vscode "$WORKSPACE/$path" 2>/dev/null || true
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi

echo "✅ Marking workspace + submodules as safe.directory for git..."
git config --global --add safe.directory "$WORKSPACE"

if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    git config --global --add safe.directory "$WORKSPACE/$path"
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi

echo "🐍 Activating Python virtual environment..."
if [ -d "$WORKSPACE/.venv" ]; then
  # shellcheck disable=SC1091
  source "$WORKSPACE/.venv/bin/activate"
fi

echo "✅ Environment ready!"
