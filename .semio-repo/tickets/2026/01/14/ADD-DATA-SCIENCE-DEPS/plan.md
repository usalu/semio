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
