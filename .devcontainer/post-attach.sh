#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region PostAttach
echo "🔌 Running post-attach setup..."
VSIX_PATH="js/vscode/semio.vsix"

#region DetectIDE
# Function to find the first working CLI binary
find_working_cli() {
  # Check in PATH first
  # We use which -a and test each because /usr/local/bin/code might be a broken wrapper
  for cmd in windsurf cursor code-insiders code; do
    for bin in $(which -a "$cmd" 2>/dev/null); do
      if "$bin" --version &>/dev/null; then
        echo "$bin"
        return 0
      fi
    done
  done

  # Check known remote-cli locations
  # Windsurf
  for bin in /home/vscode/.windsurf-server/bin/*/bin/remote-cli/windsurf; do
    if [ -x "$bin" ] && "$bin" --version &>/dev/null; then
      echo "$bin"
      return 0
    fi
  done

  # VS Code / Cursor
  for bin in /vscode/vscode-server/bin/linux-x64/*/bin/remote-cli/code; do
    if [ -x "$bin" ] && "$bin" --version &>/dev/null; then
      echo "$bin"
      return 0
    fi
  done

  return 1
}

VSCODE_BIN=$(find_working_cli)
#endregion DetectIDE

#region InstallExtension
if [ -n "$VSCODE_BIN" ]; then
  echo "📦 IDE CLI detected ($VSCODE_BIN), installing semio extension..."
  if [ ! -f "$VSIX_PATH" ] || [ "$VSIX_PATH" -ot "js/vscode/extension.ts" ]; then
    echo "📦 Building semio VS Code extension..."
    (cd js/vscode && npm run package)
  fi
  if [ -f "$VSIX_PATH" ]; then
    if "$VSCODE_BIN" --install-extension "$VSIX_PATH" --force; then
      echo "✅ Extension installed successfully!"
    else
      echo "❌ Failed to install extension via $VSCODE_BIN"
      exit 1
    fi
  else
    echo "⚠️  Extension file not found at $VSIX_PATH"
    echo "   Extension build failed; check npm output"
    exit 1
  fi
else
  echo "ℹ️  No working IDE CLI found (windsurf, cursor, code, code-insiders)"
  echo "   Extension installation skipped - this is expected outside supported IDEs"
fi
#endregion InstallExtension

echo "✅ Post-attach setup complete!"
#endregion PostAttach
