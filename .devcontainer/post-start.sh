#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
set -e

echo "🔄 Starting semio development environment..."

echo "🐍 Activating Python virtual environment..."
if [ -d ".venv" ]; then
    source .venv/bin/activate
fi

echo "✅ Environment ready!"
