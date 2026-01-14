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
