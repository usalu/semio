# Ticket

## Todos
# Plan - Consolidate Python Environment at Root

Consolidate the Python environment for the entire monorepo into a single `.venv` at the repository root using `uv` workspaces.

## Steps

1. **Initialize Root Python Workspace**
   - Create `pyproject.toml` at the repo root.
   - Define `uv` workspace with `py/semio` and `py/engine` as members.
   - Add `jupyter`, `notebook`, and `ipykernel` to a root `dev` dependency group.
   - Move shared dev tools (`ruff`, `black`, `debugpy`) to the root workspace if appropriate, or keep them as is if `uv` handles it efficiently.

2. **Adjust Subproject Configurations**
   - Update `py/semio/pyproject.toml` and `py/engine/pyproject.toml` to ensure compatibility with the root workspace.
   - Standardize `requires-python` where possible or ensure the root satisfies all.

3. **Configure VS Code**
   - Update/Create `.vscode/settings.json` to point `python.defaultInterpreterPath` to `${workspaceFolder}/.venv/Scripts/python.exe` (on Windows).

4. **Environment Setup**
   - Run `uv sync` from the repo root to generate the unified `.venv`.
   - Remove local `.venv` directories in `py/semio` and `py/engine` if they exist.

5. **Update Documentation**
   - Update `AGENTS.md` and `README.md` to reflect the new Python environment structure.
   - Document the use of `uv` at the root for Python management.

6. **Verification**
   - Verify that `jupyter` is available.
   - Verify that `py/semio` can be imported in `py/engine` when running from the root environment.
   - Run tests for both subprojects using the root environment.

## Changes

## Log
- Opened ticket and created plan.
- Created root `pyproject.toml` with `uv` workspace members and `jupyter` dependencies.
- Updated subproject `pyproject.toml` files to remove redundant tools and add workspace dependencies.
- Updated VS Code and Devcontainer configuration to point to root `.venv`.
- Refactored engine build scripts to use `uv run`.
- Ran `uv sync` from root and removed subproject `.venv` folders.
- Integrated root workspace into `dependabot.yml`.
- Updated documentation in `AGENTS.md` and `README.md`.
- Verified `jupyter` and package imports.

## Summary
# Summary - Consolidate Python Environment at Root

The Python environment for the entire monorepo has been consolidated into a single `.venv` at the repository root using `uv` workspaces.

## Changes

### Environment Management
- Created a root `pyproject.toml` defining a `uv` workspace with `py/semio` and `py/engine` as members.
- Centralized shared development tools (`ruff`, `black`, `debugpy`, `pre-commit`) at the repository root.
- Added `jupyter`, `notebook`, and `ipykernel` as development dependencies at the root.
- Updated `dependabot.yml` to include the workspace root and all Python packages.

### Subproject Updates
- Refactored `py/semio/pyproject.toml` and `py/engine/pyproject.toml` to remove redundant dev dependencies and use root workspace tools.
- Added `semio` as a workspace dependency to `py/engine`.
- Updated `py/engine/generate-schemas.ts` and `py/engine/build.ts` to use `uv run` for cross-platform execution within the workspace.

### Configuration & Tooling
- Updated `.vscode/settings.json` and `.devcontainer/devcontainer.json` to use the root `.venv` as the default interpreter.
- Updated `.devcontainer/post-create.sh` and `.devcontainer/post-start.sh` to initialize and activate the root environment.
- Updated `AGENTS.md` and `README.md` documentation to reflect the centralized environment.

### Cleanup
- Removed local `.venv` directories in `py/semio` and `py/engine`.
- Verified `jupyter` and inter-package imports work correctly via `uv run`.
