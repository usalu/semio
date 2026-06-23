# Ticket

## Todos
# Plan: Python and Rust Tests Not Appearing in VS Code Test Explorer

## Problem
Python and Rust tests are not appearing in the VS Code test explorer.

## Root Causes

### Python
The virtual environment `.venv` does not exist. VS Code's Python test discovery requires:
1. A valid Python interpreter at the configured path
2. pytest installed in that environment

Current settings reference `${workspaceFolder}/.venv/bin/python` which does not exist.

### Rust
The `rust-analyzer.cargo.target` is set to `wasm32-unknown-unknown`. This WASM target:
1. Excludes `#[cfg(test)]` modules during analysis
2. Cannot run tests (no native execution environment)

## Solutions

### Python Solution
Create the virtual environment and install dependencies:
```bash
cd /workspaces/semio
uv venv
uv sync
```

Then refresh the Python test explorer in VS Code.

### Rust Solution
Remove or modify the cargo target setting in `.vscode/settings.json`. Options:

**Option 1: Remove the target setting (recommended for test discovery)**
Remove line 57: `"rust-analyzer.cargo.target": "wasm32-unknown-unknown"`

**Option 2: Use a target that supports both WASM and native**
This requires workspace-level configuration changes in Cargo.toml to support multiple targets.

**Option 3: Use separate VS Code profiles**
One profile for WASM development, another for testing with native target.

## Recommendation
1. Run `uv venv && uv sync` to create the Python environment
2. Remove the `rust-analyzer.cargo.target` line from settings.json if native test discovery is needed, or keep it if WASM compilation analysis is more important

## Changes

## Log
# Log: Python and Rust Tests Not Appearing in VS Code Test Explorer

## Investigation

### Python Test Discovery

**Test Files Found:**
- `py/compose/compose.test.py`
- `py/engine/engine.test.py`

**Configuration:**
- `.vscode/settings.json` has `python.testing.pytestEnabled: true`
- `python.testing.pytestArgs: ["."]` configured
- `python.defaultInterpreterPath: "${workspaceFolder}/.venv/bin/python"`

**Root Cause:**
The virtual environment `.venv` does not exist. The Python interpreter path configured in settings.json points to a non-existent `.venv` directory. Without a valid Python environment with pytest installed, VS Code cannot discover or run Python tests.

### Rust Test Discovery

**Test Files Found:**
- `rs/compose/compose.rs` contains 15 `#[test]` functions in a `#[cfg(test)]` module

**Configuration:**
- `rust-analyzer.linkedProjects: ["rs/compose/Cargo.toml"]`
- `rust-analyzer.cargo.target: "wasm32-unknown-unknown"`

**Root Cause:**
The `rust-analyzer.cargo.target` is set to `wasm32-unknown-unknown` (WebAssembly). Rust tests require a native target to run (e.g., `x86_64-unknown-linux-gnu`). When rust-analyzer is configured for a WASM target, it cannot discover or run tests because:
1. The `#[cfg(test)]` module is excluded when compiling for WASM targets
2. Tests cannot execute on WASM without a runtime environment

## Solutions Documented

See [plan.md](plan.md) for detailed solutions.

**Quick fix for Python:**
```bash
uv venv && uv sync
```

**Quick fix for Rust:**
Remove `"rust-analyzer.cargo.target": "wasm32-unknown-unknown"` from `.vscode/settings.json`

---

## Follow-up Investigation

User reported tests still not appearing after initial investigation.

### Actions Taken

1. **Created Python virtual environment:**
   ```bash
   uv venv && uv sync --all-packages
   ```

2. **Installed pytest:**
   ```bash
   uv pip install pytest pytest-cov deepdiff
   ```

3. **Added pytest configuration to root pyproject.toml:**
   - Added `test` dependency group with pytest, pytest-cov, deepdiff
   - Added `[tool.pytest.ini_options]` section with:
     - `python_files = ["test_*.py", "*_test.py", "*.test.py"]`
     - `testpaths = ["py/compose", "py/engine"]`
     - `addopts = "--import-mode=importlib"` (critical for `*.test.py` naming convention)

4. **Installed missing system library for PySide6:**
   ```bash
   sudo apt-get install -y libegl1
   ```

5. **Removed Rust WASM target from settings.json:**
   Removed `"rust-analyzer.cargo.target": "wasm32-unknown-unknown"` line

### Results

Python tests now discoverable:
- 8 tests from `py/compose/compose.test.py`
- 42 tests from `py/engine/engine.test.py`
- Total: 50 tests collected

Rust tests should now be discoverable after rust-analyzer restart.

---

## Second Follow-up

User reported Rust tests still not appearing after removing WASM target.

### Root Cause
rust-analyzer's Test Explorer integration is disabled by default. It must be explicitly enabled via settings.

### Fix Applied
Added to `.vscode/settings.json`:
```json
"rust-analyzer.testExplorer": true
```

This enables rust-analyzer to populate the VS Code Test Explorer with Rust tests.

## Summary
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
- 50 Python tests now discoverable (8 from compose, 42 from engine)
- Rust tests should be discoverable after rust-analyzer restart (Reload Window)

## Files Modified
- `pyproject.toml` - Added pytest configuration
- `.vscode/settings.json` - Removed WASM target
