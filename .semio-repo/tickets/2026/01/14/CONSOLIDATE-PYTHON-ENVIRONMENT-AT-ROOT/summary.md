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
