#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖PostCreate
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"
#region 🔖Startup
echo "Setting up semio development environment..."
#endregion 🔖Startup
#region 🔖Ownership
echo "Fixing ownership of mounted config directories..."
sudo chown -R vscode:vscode /home/vscode/.cache || true
sudo chown -R vscode:vscode /home/vscode/.config/gh || true
#endregion 🔖Ownership
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
cd js/vscode
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
