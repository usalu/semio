# Ticket

## Todos

# Plan: Tidy Benchmark Files

## Current State

| Language   | Current Location                 | Target Location                        |
| ---------- | -------------------------------- | -------------------------------------- |
| Python     | `py/compose/benchmark.py`          | `py/compose/compose.benchmark.py`          |
| JavaScript | `js/compose/benchmark.ts`          | `js/compose/compose.benchmark.ts`          |
| Rust       | `rs/compose/examples/benchmark.rs` | `rs/compose/compose.benchmark.rs`          |
| Go         | (none)                           | `go/compose/compose_benchmark.go`          |
| .NET       | `net/Compose.Benchmark/Program.cs` | No change (already follows convention) |
| Reports    | `benchmark.csv`                  | `reports/benchmark.csv`                |

## Tasks

1. **Python**: Rename `py/compose/benchmark.py` → `py/compose/compose.benchmark.py`

2. **JavaScript**: Rename `js/compose/benchmark.ts` → `js/compose/compose.benchmark.ts`

3. **Rust**:
   - Move `rs/compose/examples/benchmark.rs` → `rs/compose/compose.benchmark.rs`
   - Update `Cargo.toml` to add `[[bin]]` section for the benchmark

4. **Go**: Create `go/compose/compose_benchmark.go` (new file - need to determine if existing benchmark logic exists elsewhere or needs to be written)

5. **Reports**:
   - Create `reports/` directory
   - Move `benchmark.csv` → `reports/benchmark.csv`

6. **Update References**: Check and update any scripts or configs that reference the old file paths (e.g., `./repo/cli/cmd_benchmark.go`)

## Notes

- The .NET project already follows the convention with a separate `Compose.Benchmark` project
- Go doesn't have a benchmark file yet - need to check if this should be created or if it's out of scope
- The `temp/benchmark.csv` appears to be a temporary file and can be ignored

## Changes

## Log

# Log: Tidy Benchmark Files

## Actions Performed

1. **Python**: Renamed `py/compose/benchmark.py` → `py/compose/compose.benchmark.py` using `git mv`

2. **JavaScript**: Renamed `js/compose/benchmark.ts` → `js/compose/compose.benchmark.ts` using `git mv`

3. **Rust**:
   - Moved `rs/compose/examples/benchmark.rs` → `rs/compose/compose.benchmark.rs` using `git mv`
   - Updated `rs/compose/Cargo.toml` to add `[[bin]]` section for `compose-benchmark` pointing to the new file
   - Removed empty `rs/compose/examples/` directory

4. **Go**:
   - Moved `go/compose/benchmark/main.go` → `go/compose/compose_benchmark.go` using `git mv`
   - Added `//go:build ignore` build constraint to exclude from package compilation while allowing `go run`
   - Removed empty `go/compose/benchmark/` directory

5. **Reports**:
   - Created `reports/` directory
   - Moved `benchmark.csv` → `reports/benchmark.csv` using `git mv`

6. **Updated References** in `./repo/cli/cmd_benchmark.go`:
   - TypeScript: `benchmark.ts` → `compose.benchmark.ts`
   - Python: `benchmark.py` → `compose.benchmark.py`
   - Go: `benchmark/main.go` → `compose_benchmark.go`
   - Rust: `--example benchmark` → `--bin compose-benchmark`
   - Report output: `temp/benchmark.csv` → `reports/benchmark.csv`

## Notes

- The .NET project (`net/Compose.Benchmark/`) already followed the convention with a separate project, so no changes were needed
- The Go benchmark file uses `//go:build ignore` to allow it to coexist in the same directory as the compose package

## Summary

# Summary: Tidy Benchmark Files

Reorganized benchmark files across all language implementations to follow a consistent naming convention.

## Final Structure

| Language   | Location                                     |
| ---------- | -------------------------------------------- |
| Python     | `py/compose/compose.benchmark.py`                |
| JavaScript | `js/compose/compose.benchmark.ts`                |
| Rust       | `rs/compose/compose.benchmark.rs`                |
| Go         | `go/compose/compose_benchmark.go`                |
| .NET       | `net/Compose.Benchmark/Program.cs` (unchanged) |
| Reports    | `reports/benchmark.csv`                      |

## Changes Made

- Renamed 4 benchmark files to follow the `compose.benchmark.*` / `compose_benchmark.*` convention
- Moved Rust benchmark from `examples/` to root of `rs/compose/` and updated Cargo.toml
- Moved Go benchmark from `benchmark/` subdirectory to root of `go/compose/` with `//go:build ignore` constraint
- Created `reports/` directory for benchmark output
- Updated `./repo/cli/cmd_benchmark.go` with new file paths and commands
