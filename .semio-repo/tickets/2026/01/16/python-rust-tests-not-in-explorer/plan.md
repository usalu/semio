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
