#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖️PostCreate
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"
cd "$WORKSPACE"

echo "Setting up compose (Bun + Nx)…"

#region 🔖️EmojiFonts
configure_emoji_fonts() {
  echo "Configuring emoji font fallback..."
  sudo mkdir -p /etc/fonts
  sudo tee /etc/fonts/local.conf >/dev/null <<'FONTCONFIG'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
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
</fontconfig>
FONTCONFIG
  sudo fc-cache -f || true
  echo "Emoji font configuration updated."
}
#endregion 🔖️EmojiFonts

#region 🔖️Ownership
echo "Fixing ownership of mounted config directories..."
sudo chown -R vscode:vscode /home/vscode/.cache || true
sudo chown -R vscode:vscode /home/vscode/.config/gh || true
sudo chown -R vscode:vscode /home/vscode/.gitkraken || true
sudo chown -R vscode:vscode /home/vscode/.local/share/GitKrakenCLI || true
sudo chown -R vscode:vscode /home/vscode/.local/share/gk || true
#endregion 🔖️Ownership

sudo apt-get update && sudo apt-get install -y ripgrep jq unzip || true
configure_emoji_fonts

echo "Marking workspace as safe for git..."
git config --global --add safe.directory "$WORKSPACE"
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    git config --global --add safe.directory "$WORKSPACE/$path"
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi

export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"
export PATH="$BUN_INSTALL/bin:$PATH"
if ! command -v bun >/dev/null 2>&1; then
  echo "Installing Bun…"
  curl -fsSL https://bun.sh/install | bash
  export PATH="$HOME/.bun/bin:$PATH"
fi

echo "Installing JS deps with Bun…"
bun install

echo "Building repo client binary…"
go build -o '🧰️framework/🛍️product/🦑️repo/🔨️module/💻️client/client' './🧰️framework/🛍️product/🦑️repo/🔨️module/💻️client/🔌️mcp/⚡️implementation/🐹️go'

echo "Running workspace setup (polyglot toolchains + VSIX + hooks)…"
bun nx run workspace:setup

echo "Development environment ready!"
# #endregion 🔖️PostCreate
