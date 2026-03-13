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
  <alias>
    <family>sans-serif</family>
    <prefer>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <alias>
    <family>serif</family>
    <prefer>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <alias>
    <family>monospace</family>
    <prefer>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
</fontconfig>
FONTCONFIG
  sudo fc-cache -f
}
#endregion 🔖EmojiFonts
#region 🔖Startup
echo "Setting up semio development environment..."
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
npm install
#region 🔖GeminiCli
echo "Installing Gemini CLI..."
npm install -g @google/gemini-cli
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
uv sync
#endregion 🔖Python
#region 🔖Go
echo "Building Go binaries..."
cd ./semio-repo/cli
go build
cd ../..
#endregion 🔖Go
#region 🔖Dotnet
echo "Restoring .NET packages..."
dotnet restore net/Semio.sln
#endregion 🔖Dotnet
#region 🔖Rust
echo "Adding Rust wasm target..."
rustup target add wasm32-unknown-unknown || true
echo "Configuring Rust wasm settings..."
mkdir -p "$HOME/.cargo"
cat <<'CONFIG' > "$HOME/.cargo/config.toml"
[target.wasm32-unknown-unknown]
rustflags = ["--cfg", "getrandom_backend=wasm_js"]
CONFIG
#endregion 🔖Rust
#region 🔖Playwright
echo "Installing Playwright browsers..."
mkdir -p "$WORKSPACE/node_modules/.cache/ms-playwright"
PLAYWRIGHT_BROWSERS_PATH="$WORKSPACE/node_modules/.cache/ms-playwright" npx playwright install --with-deps chromium
#endregion 🔖Playwright
#region 🔖GitHooks
echo "Configuring git hooks and agent hook configs..."
./semio-repo/cli/cli configure || true
#endregion 🔖GitHooks
#region 🔖VSCode
echo "Building semio VSCode extension..."
cd semio-repo/vscode
npm run build
npm run package
cd ../..
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
    "semio-repo": {
      "command": "/workspaces/semio/semio-repo/cli/cli",
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
        "engine.py",
        "--mcp-stdio"
      ]
    },
    "coda": {
      "command": "uv",
      "args": [
        "--directory",
        "/workspaces/semio/semio-coda/py",
        "run",
        "coda.py",
        "--mcp-stdio"
      ]
    }
  }
}
MCPCONFIG
#endregion 🔖Antigravity
echo "Development environment ready!"
#endregion 🔖PostCreate
