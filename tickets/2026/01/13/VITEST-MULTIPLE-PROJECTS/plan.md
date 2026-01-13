# Plan: Investigate Vitest Multiple Projects Warning

## Objective
Investigate why the VS Code Vitest extension reports "multiple projects" and suggest solutions.

## Steps
1. Find all vitest/vite config files in the repository (excluding node_modules)
2. Analyze which configs actually contain test configuration
3. Review current VS Code settings for any vitest configuration
4. Propose solutions to resolve the warning

## Expected Deliverables
- List of all relevant config files
- Analysis of which configs are actually used for testing
- Recommended solutions with pros/cons
