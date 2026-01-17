# Log: Python and Rust Tests Not Appearing in VS Code Test Explorer

## Investigation

### Python Test Discovery

**Test Files Found:**
- `py/semio/semio.test.py`
- `py/engine/engine.test.py`

**Configuration:**
- `.vscode/settings.json` has `python.testing.pytestEnabled: true`
- `python.testing.pytestArgs: ["."]` configured
- `python.defaultInterpreterPath: "${workspaceFolder}/.venv/bin/python"`

**Root Cause:**
The virtual environment `.venv` does not exist. The Python interpreter path configured in settings.json points to a non-existent `.venv` directory. Without a valid Python environment with pytest installed, VS Code cannot discover or run Python tests.

### Rust Test Discovery

**Test Files Found:**
- `rs/semio/semio.rs` contains 15 `#[test]` functions in a `#[cfg(test)]` module

**Configuration:**
- `rust-analyzer.linkedProjects: ["rs/semio/Cargo.toml"]`
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
     - `testpaths = ["py/semio", "py/engine"]`
     - `addopts = "--import-mode=importlib"` (critical for `*.test.py` naming convention)

4. **Installed missing system library for PySide6:**
   ```bash
   sudo apt-get install -y libegl1
   ```

5. **Removed Rust WASM target from settings.json:**
   Removed `"rust-analyzer.cargo.target": "wasm32-unknown-unknown"` line

### Results

Python tests now discoverable:
- 8 tests from `py/semio/semio.test.py`
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
