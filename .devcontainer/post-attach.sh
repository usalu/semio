#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only

echo "🔌 Running post-attach setup..."

VSIX_PATH="js/vscode/semio.vsix"

VSCODE_BIN=""
if command -v code &> /dev/null; then
    VSCODE_BIN="code"
elif command -v code-insiders &> /dev/null; then
    VSCODE_BIN="code-insiders"
fi

if [ -n "$VSCODE_BIN" ]; then
    echo "📦 VS Code CLI detected, installing semio extension..."
    if [ ! -f "$VSIX_PATH" ]; then
        echo "📦 Building semio VS Code extension..."
        (cd js/vscode && npm run package)
    fi
    if [ -f "$VSIX_PATH" ]; then
        "$VSCODE_BIN" --install-extension "$VSIX_PATH" --force
        echo "✅ Extension installed successfully!"
    else
        echo "⚠️  Extension file not found at $VSIX_PATH"
        echo "   Extension build failed; check npm output"
    fi
else
    echo "ℹ️  VS Code CLI not available (running in headless/CLI mode)"
    echo "   Extension installation skipped - this is expected outside VS Code"
fi

echo "✅ Post-attach setup complete!"
