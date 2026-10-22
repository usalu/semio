#!/usr/bin/env bash
# 🧪 Lab: the styling builder uv sequence against the managed pinned interpreter (find, sync, run) and the negative control with the old --no-managed-python flag. Usage: uv-builder-lab.sh <lab dir> <styling python project dir>
set -u
LAB="$1"; PROJECT="$2"; UV="$(command -v uv)"
rm -rf "$LAB/interp" "$LAB/env"; mkdir -p "$LAB/interp" "$LAB/env"
cp -cR "$HOME/.local/share/uv/python/cpython-3.14.5-macos-aarch64-none" "$LAB/interp/" 2>/dev/null || cp -R "$HOME/.local/share/uv/python/"cpython-3.14*-* "$LAB/interp/"
base=(env -i HOME="$HOME" PATH=/usr/bin:/bin UV_NO_CONFIG=1 UV_PYTHON_DOWNLOADS=manual UV_PYTHON_INSTALL_DIR="$LAB/interp")
FOUND="$("${base[@]}" "$UV" python find --no-config --no-python-downloads --managed-python 3.14)"; echo "find: ${FOUND:-none}"
cd "$PROJECT"
"${base[@]}" UV_PYTHON="$FOUND" UV_PROJECT_ENVIRONMENT="$LAB/env/e" "$UV" sync --no-config --no-python-downloads --managed-python --locked --group build --no-install-project --offline --project . >"$LAB/sync.log" 2>&1; echo "sync(managed): exit=$?"
"${base[@]}" UV_PYTHON="$FOUND" UV_PROJECT_ENVIRONMENT="$LAB/env/e" "$UV" run --no-config --no-python-downloads --managed-python --locked --no-sync --offline --group build --project . python -I -B -c "import hatchling,sys;print('python',sys.version_info[:2],'hatchling ok')" 2>&1 | tail -1
"${base[@]}" UV_PYTHON="$FOUND" UV_PROJECT_ENVIRONMENT="$LAB/env/e2" "$UV" sync --no-config --no-python-downloads --no-managed-python --locked --group build --no-install-project --offline --project . >"$LAB/control.log" 2>&1; echo "sync(--no-managed-python, control): exit=$?"; tail -2 "$LAB/control.log"
