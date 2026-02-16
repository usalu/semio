# Ticket

## Todos
# Plan: Add Data Science Dependencies

## Objective
Add pandas and other data science libraries to the dev dependencies of the main `.venv` to enable seamless Jupyter notebook execution.

## Steps

1. **Identify current dependency configuration**
   - Located: `pyproject.toml` at the repo root
   - Uses `[dependency-groups]` section with `dev` group
   - Already has Jupyter, notebook, and ipykernel

2. **Add data science libraries**
   Add the following to the `dev` dependency group:
   - `pandas` - Data manipulation and analysis
   - `numpy` - Numerical computing
   - `matplotlib` - Plotting and visualization
   - `seaborn` - Statistical visualization
   - `scipy` - Scientific computing
   - `scikit-learn` - Machine learning

3. **Sync the virtual environment**
   Run `uv sync` to install the new dependencies

## Files to modify
- `pyproject.toml` - Add dependencies to `[dependency-groups]` dev section

## Changes

## Log
# Log: Add Data Science Dependencies

## 2026-01-14

### Task
Add pandas and other data science libraries to dev dependencies of the main .venv for Jupyter notebook support.

### Actions Taken

1. **Located configuration file**
   - Found `pyproject.toml` at repo root
   - Identified `[dependency-groups]` section with `dev` group
   - Already had: jupyter, notebook, ipykernel, ruff, black, debugpy, pre-commit

2. **Added data science dependencies**
   Added to `pyproject.toml`:
   - pandas>=2.2.0
   - numpy>=2.0.0
   - matplotlib>=3.9.0
   - seaborn>=0.13.0
   - scipy>=1.14.0
   - scikit-learn>=1.5.0

3. **Synced virtual environment**
   - Ran `uv sync` to install new packages
   - Successfully installed: pandas, numpy, matplotlib, seaborn, scipy, scikit-learn
   - Also installed transitive dependencies: joblib, pytz, threadpoolctl

### Result
Data science libraries are now available in the dev environment for Jupyter notebooks.

---

### Follow-up: Missing ipykernel

**Issue:** After initial sync, ipykernel and other jupyter packages were missing.

**Cause:** `uv sync` without flags doesn't include dev dependencies by default.

**Fix:** Ran `uv sync --group dev` to install all dev dependencies including:
- jupyter, notebook, ipykernel
- pandas, numpy, matplotlib, seaborn, scipy, scikit-learn
- ruff, black, debugpy, pre-commit

All packages now installed correctly.

## Summary
# Summary: Add Data Science Dependencies

Added the following data science libraries to the dev dependencies in `pyproject.toml`:

- **pandas** (>=2.2.0) - Data manipulation and analysis
- **numpy** (>=2.0.0) - Numerical computing
- **matplotlib** (>=3.9.0) - Plotting and visualization
- **seaborn** (>=0.13.0) - Statistical visualization
- **scipy** (>=1.14.0) - Scientific computing
- **scikit-learn** (>=1.5.0) - Machine learning

The virtual environment has been synced with `uv sync` and all packages are installed.

Jupyter notebooks can now use these libraries seamlessly within the main `.venv`.
