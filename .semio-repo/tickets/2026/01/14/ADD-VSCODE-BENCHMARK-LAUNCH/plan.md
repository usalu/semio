# Plan: Add VSCode Benchmark Launch Configuration

## Objective
Add a launch configuration to `.vscode/launch.json` that runs the benchmark script to recompute all benchmarks.

## Analysis
- The benchmark script is located at `scripts/benchmark.ts`
- It runs benchmarks for TypeScript, Python, Go, C#, and Rust
- Output is written to `temp/benchmark.csv`
- Other launch configurations use `npx tsx` to run TypeScript scripts

## Implementation Steps
1. Add a new launch configuration entry to `.vscode/launch.json`
2. Use the same pattern as other npm/tsx-based configurations (e.g., "analyze", "fix")
3. Run the benchmark script using `npx tsx scripts/benchmark.ts`

## Configuration Details
```json
{
  "name": "benchmark",
  "type": "node",
  "request": "launch",
  "runtimeExecutable": "npx",
  "runtimeArgs": [
    "tsx",
    "scripts/benchmark.ts"
  ],
  "cwd": "${workspaceFolder}",
  "console": "integratedTerminal"
}
```
