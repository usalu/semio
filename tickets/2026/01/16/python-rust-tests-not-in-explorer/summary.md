# Summary: Python and Rust Tests Not Appearing in VS Code Test Explorer

## Problem
Python and Rust tests were not appearing in the VS Code test explorer.

## Root Causes

### Python
1. Virtual environment `.venv` did not exist
2. pytest was not installed
3. Root `pyproject.toml` lacked pytest configuration
4. Missing `--import-mode=importlib` required for `*.test.py` naming convention
5. Missing system library `libEGL` for PySide6

### Rust
1. `rust-analyzer.cargo.target` set to `wasm32-unknown-unknown` excluded `#[cfg(test)]` modules

## Fixes Applied

### Python
1. Created `.venv` via `uv venv`
2. Synced workspace packages via `uv sync --all-packages`
3. Installed pytest via `uv pip install pytest pytest-cov deepdiff`
4. Added to root `pyproject.toml`:
   - `test` dependency group
   - `[tool.pytest.ini_options]` with `testpaths`, `python_files`, and `addopts`
5. Installed `libegl1` system package for PySide6

### Rust
1. Removed `"rust-analyzer.cargo.target": "wasm32-unknown-unknown"` from `.vscode/settings.json`

## Result
- 50 Python tests now discoverable (8 from semio, 42 from engine)
- Rust tests should be discoverable after rust-analyzer restart (Reload Window)

## Files Modified
- `pyproject.toml` - Added pytest configuration
- `.vscode/settings.json` - Removed WASM target
