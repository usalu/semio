#!/bin/bash
set -e

echo "🔄 Starting semio development environment..."

echo "🐍 Activating Python virtual environment..."
if [ -d "py/engine/.venv" ]; then
    source py/engine/.venv/bin/activate
fi

echo "✅ Environment ready!"
