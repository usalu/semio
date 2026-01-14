#!/bin/bash
set -e

echo "🚀 Setting up semio development environment..."

echo "📦 Installing npm dependencies..."
npm install

echo "🐍 Setting up Python environment..."
uv sync

echo "🔧 Building Go binaries..."
cd go/repo
go build -o repo
cd ../mcp
go build -o mcp
cd ../cli
go build -o cli
cd ../..

echo "🔨 Restoring .NET packages..."
dotnet restore net/Semio.sln

echo "🦀 Adding Rust wasm target..."
rustup target add wasm32-unknown-unknown || true

echo "🎭 Installing Playwright browsers..."
npx playwright install --with-deps chromium

echo "📋 Setting up git hooks..."
npm run prepare || true

echo "✅ Development environment ready!"
