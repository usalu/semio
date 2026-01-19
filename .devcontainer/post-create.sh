#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
set -e

echo "Setting up semio development environment..."

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
npx playwright install --with-deps chromium

echo "Setting up git hooks..."
npm run prepare || true

echo "Building semio VSCode extension..."
cd js/vscode
npm run build
npm run package
cd ../..

echo "Development environment ready!"
