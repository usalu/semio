#!/usr/bin/env bash
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
cd "$R" || exit 1
export UV_NO_CONFIG=1 UV_PYTHON_PREFERENCE=only-managed UV_PYTHON_DOWNLOADS=manual PYTHONDONTWRITEBYTECODE=1
export UV_PROJECT_ENVIRONMENT="$R/.cache/b/d/m/python/environments/workspace-3b97251b6599d245"
export UV_PYTHON_INSTALL_DIR="$R/.cache/b/t/m/python/interpreters" UV_PYTHON=3.14
export UV_CACHE_DIR="$R/.cache/b/d/m/python/uv-cache"
repo tools exec uv -- python install 3.14
repo tools exec uv -- sync --locked --project . --group dev --group test
echo "exit $?"
