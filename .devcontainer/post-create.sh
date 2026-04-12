#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖PostCreate
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"
#region 🔖EmojiFonts
configure_emoji_fonts() {
  echo "Configuring emoji font fallback..."
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
      <string>Noto Color Emoji</string>
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
  echo "✅ Emoji font configuration updated."
}
#endregion 🔖EmojiFonts
#region 🔖Startup
echo "Setting up semio development environment..."
#region 🔖Ripgrep
echo "Installing ripgrep..."
sudo apt-get update && sudo apt-get install -y ripgrep
#endregion 🔖Ripgrep
#endregion 🔖Startup
#region 🔖GitKrakenCliHelpers
install_gitkraken_desktop() {
  if command -v gitkraken >/dev/null 2>&1; then
    echo "GitKraken Desktop already installed."
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
  mkdir -p "$HOME/.gitkraken"
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
  if command -v gk >/dev/null 2>&1 && gk --version >/dev/null 2>&1; then
    echo "GitKraken CLI already installed."
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

  mkdir -p "$HOME/.local/bin" "$HOME/.local/share/GitKrakenCLI" "$HOME/.local/share/gk"
  local temp_dir
  temp_dir="$(mktemp -d)"
  trap 'rm -rf "$temp_dir"' RETURN
  curl -fsSL "$download_url" -o "$temp_dir/gk.zip"
  unzip -q "$temp_dir/gk.zip" -d "$temp_dir"
  install -m 0755 "$temp_dir/gk" "$HOME/.local/bin/gk"
  rm -rf "$temp_dir"
  trap - RETURN

  gk --version >/dev/null 2>&1
  echo "GitKraken CLI installed."
}
#endregion 🔖GitKrakenCliHelpers
#region 🔖Ownership
echo "Fixing ownership of mounted config directories..."
sudo chown -R vscode:vscode /home/vscode/.cache || true
sudo chown -R vscode:vscode /home/vscode/.config/gh || true
sudo chown -R vscode:vscode /home/vscode/.gitkraken || true
sudo chown -R vscode:vscode /home/vscode/.local/share/GitKrakenCLI || true
sudo chown -R vscode:vscode /home/vscode/.local/share/gk || true
#endregion 🔖Ownership
#region 🔖EmojiFonts
configure_emoji_fonts
#endregion 🔖EmojiFonts
#region 🔖GitSafe
echo "Marking workspace as safe for git (devcontainer bind-mount / submodules)..."
git config --global --add safe.directory "$WORKSPACE"
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    git config --global --add safe.directory "$WORKSPACE/$path"
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi
#endregion 🔖GitSafe
#region 🔖Node
echo "Installing npm dependencies..."
if npm install; then
  echo "✅ npm dependencies installed successfully."
else
  echo "⚠️  npm install failed, but continuing..."
fi
#region 🔖ElectronSandbox
echo "Fixing Electron chrome-sandbox permissions for devcontainer..."
CHROME_SANDBOX="$WORKSPACE/node_modules/electron/dist/chrome-sandbox"
if [ -f "$CHROME_SANDBOX" ]; then
  sudo chown root:root "$CHROME_SANDBOX"
  sudo chmod 4755 "$CHROME_SANDBOX"
  echo "✅ Electron chrome-sandbox permissions fixed."
else
  echo "⚠️  chrome-sandbox not found (electron not yet installed), skipping..."
fi
#endregion 🔖ElectronSandbox
#region 🔖GeminiCli
echo "Installing Gemini CLI..."
if npm install -g @google/gemini-cli; then
  echo "✅ Gemini CLI installed successfully."
else
  echo "⚠️  Gemini CLI installation failed, but continuing..."
fi
#endregion 🔖GeminiCli
#endregion 🔖Node
#region 🔖GitKrakenDesktop
install_gitkraken_desktop
#endregion 🔖GitKrakenDesktop
#region 🔖GitKrakenCli
install_gitkraken_cli
#endregion 🔖GitKrakenCli
#region 🔖Python
echo "Setting up Python environment..."
if uv sync; then
  echo "✅ Python environment setup completed."
else
  echo "⚠️  uv sync failed, but continuing..."
fi
#endregion 🔖Python
#region 🔖Go
echo "Building Go binaries..."
if cd ./repo/cli && go build; then
  echo "✅ Go CLI built successfully."
  cd ../..
else
  echo "⚠️  Go build failed, but continuing..."
  cd ../..
fi
#endregion 🔖Go
#region 🔖Dotnet
echo "Restoring .NET packages..."
if dotnet restore Monorepo.sln; then
  echo "✅ .NET packages restored successfully."
else
  echo "⚠️  .NET restore failed, but continuing..."
fi
#endregion 🔖Dotnet
#region 🔖Rust
echo "Adding Rust wasm target..."
if rustup target add wasm32-unknown-unknown; then
  echo "✅ Rust wasm target added."
else
  echo "⚠️  Rust wasm target addition failed, but continuing..."
fi
echo "Configuring Rust wasm settings..."
mkdir -p "$HOME/.cargo"
cat <<'CONFIG' > "$HOME/.cargo/config.toml"
[target.wasm32-unknown-unknown]
rustflags = ["--cfg", "getrandom_backend=wasm_js"]
CONFIG
echo "✅ Rust wasm configuration completed."
#endregion 🔖Rust
#region 🔖KiroLsp
echo "Installing Kiro LSP servers..."
if npm install -g typescript-language-server typescript pyright; then
  echo "✅ LSP servers installed."
else
  echo "⚠️  LSP server installation failed, but continuing..."
fi
# Symlink into /usr/local/bin so kiro's clean PATH finds them (nvm bin not in kiro PATH)
for bin in typescript-language-server pyright-langserver pyright; do
  src="$(npm bin -g 2>/dev/null)/$bin"
  [ -f "$src" ] && sudo ln -sf "$src" "/usr/local/bin/$bin" && echo "✅ Linked $bin" || echo "⚠️  Could not link $bin"
done
#endregion 🔖KiroLsp
#region 🔖Playwright
echo "Installing Playwright browsers..."
mkdir -p "$WORKSPACE/node_modules/.cache/ms-playwright"
if PLAYWRIGHT_BROWSERS_PATH="$WORKSPACE/node_modules/.cache/ms-playwright" npx playwright install --with-deps chromium; then
  echo "✅ Playwright browsers installed successfully."
else
  echo "⚠️  Playwright installation failed, but continuing..."
fi
#endregion 🔖Playwright
#region 🔖GitHooks
echo "Configuring git hooks and agent hook configs..."
if ./repo/cli/cli configure; then
  echo "✅ Git hooks configured successfully."
else
  echo "⚠️  Git hooks configuration failed, but continuing..."
fi
#endregion 🔖GitHooks
#region 🔖VSCode
echo "Building semio VSCode extension..."
if cd repo/vscode && npm run build && npm run package; then
  echo "✅ VSCode extension built successfully."
  cd ../..
else
  echo "⚠️  VSCode extension build failed, but continuing..."
  cd ../..
fi
#endregion 🔖VSCode
#region 🔖Antigravity
echo "Installing Antigravity server..."
curl -sL "https://raw.githubusercontent.com/hucaico/demo-devcontainer/main/server-install.sh" -o /tmp/server-install.sh
chmod +x /tmp/server-install.sh
/tmp/server-install.sh || true
echo "Configuring Antigravity MCP Server..."
mkdir -p /home/vscode/.gemini/antigravity
cat << 'MCPCONFIG' > /home/vscode/.gemini/antigravity/mcp_config.json
{
  "mcpServers": {
    "repo": {
      "command": "/workspaces/semio/repo/cli/cli",
      "args": [
        "mcp"
      ]
    },
    "semio": {
      "command": "uv",
      "args": [
        "--directory",
        "/workspaces/semio/semio/engine",
        "run",
        "main.py",
        "--mcp-stdio"
      ]
    },
    "coda": {
      "command": "uv",
      "args": [
        "--directory",
        "/workspaces/semio/coda/assistant",
        "run",
        "main.py",
        "--mcp-stdio"
      ]
    }
  }
}
MCPCONFIG
#endregion 🔖Antigravity
echo "Development environment ready!"
#endregion 🔖PostCreate
