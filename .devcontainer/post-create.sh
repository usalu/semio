#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"

echo "Setting up semio development environment..."

echo "Fixing ownership of mounted config directories..."
sudo chown -R vscode:vscode /home/vscode/.config/gh || true

echo "Marking workspace as safe for git (devcontainer bind-mount / submodules)..."
git config --global --add safe.directory "$WORKSPACE"
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    git config --global --add safe.directory "$WORKSPACE/$path"
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' \
        | awk '{print $2}')
fi

echo "Installing npm dependencies..."
npm install

echo "Setting up Python environment..."
uv sync

echo "Building Go binaries..."
cd go/repo
go build -o repo
cd ../..

echo "Restoring .NET packages..."
dotnet restore net/Semio.sln

echo "Adding Rust wasm target..."
rustup target add wasm32-unknown-unknown || true

echo "Configuring Rust wasm settings..."
mkdir -p "$HOME/.cargo"
cat <<'EOF' > "$HOME/.cargo/config.toml"
[target.wasm32-unknown-unknown]
rustflags = ["--cfg", "getrandom_backend=wasm_js"]
EOF

echo "Installing Playwright browsers..."
mkdir -p "$WORKSPACE/node_modules/.cache/ms-playwright"
PLAYWRIGHT_BROWSERS_PATH="$WORKSPACE/node_modules/.cache/ms-playwright" npx playwright install --with-deps chromium

echo "Setting up git hooks..."
npm run prepare || true

echo "Building semio VSCode extension..."
cd js/vscode
npm run build
npm run package
cd ../..

echo "Development environment ready!"
