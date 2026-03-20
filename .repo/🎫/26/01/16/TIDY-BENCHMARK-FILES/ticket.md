# Ticket

## Todos

# Plan: Tidy Benchmark Files

## Current State

| Language   | Current Location                 | Target Location                        |
| ---------- | -------------------------------- | -------------------------------------- |
| Python     | `py/semio/benchmark.py`          | `py/semio/semio.benchmark.py`          |
| JavaScript | `js/semio/benchmark.ts`          | `js/semio/semio.benchmark.ts`          |
| Rust       | `rs/semio/examples/benchmark.rs` | `rs/semio/semio.benchmark.rs`          |
| Go         | (none)                           | `go/semio/semio_benchmark.go`          |
| .NET       | `net/Semio.Benchmark/Program.cs` | No change (already follows convention) |
| Reports    | `benchmark.csv`                  | `reports/benchmark.csv`                |

## Tasks

1. **Python**: Rename `py/semio/benchmark.py` → `py/semio/semio.benchmark.py`

2. **JavaScript**: Rename `js/semio/benchmark.ts` → `js/semio/semio.benchmark.ts`

3. **Rust**:
   - Move `rs/semio/examples/benchmark.rs` → `rs/semio/semio.benchmark.rs`
   - Update `Cargo.toml` to add `[[bin]]` section for the benchmark

4. **Go**: Create `go/semio/semio_benchmark.go` (new file - need to determine if existing benchmark logic exists elsewhere or needs to be written)

5. **Reports**:
   - Create `reports/` directory
   - Move `benchmark.csv` → `reports/benchmark.csv`

6. **Update References**: Check and update any scripts or configs that reference the old file paths (e.g., `./repo/cli/cmd_benchmark.go`)

## Notes

- The .NET project already follows the convention with a separate `Semio.Benchmark` project
- Go doesn't have a benchmark file yet - need to check if this should be created or if it's out of scope
- The `temp/benchmark.csv` appears to be a temporary file and can be ignored

## Changes

## Log

# Log: Tidy Benchmark Files

## Actions Performed

1. **Python**: Renamed `py/semio/benchmark.py` → `py/semio/semio.benchmark.py` using `git mv`

2. **JavaScript**: Renamed `js/semio/benchmark.ts` → `js/semio/semio.benchmark.ts` using `git mv`

3. **Rust**:
   - Moved `rs/semio/examples/benchmark.rs` → `rs/semio/semio.benchmark.rs` using `git mv`
   - Updated `rs/semio/Cargo.toml` to add `[[bin]]` section for `semio-benchmark` pointing to the new file
   - Removed empty `rs/semio/examples/` directory

4. **Go**:
   - Moved `go/semio/benchmark/main.go` → `go/semio/semio_benchmark.go` using `git mv`
   - Added `//go:build ignore` build constraint to exclude from package compilation while allowing `go run`
   - Removed empty `go/semio/benchmark/` directory

5. **Reports**:
   - Created `reports/` directory
   - Moved `benchmark.csv` → `reports/benchmark.csv` using `git mv`

6. **Updated References** in `./repo/cli/cmd_benchmark.go`:
   - TypeScript: `benchmark.ts` → `semio.benchmark.ts`
   - Python: `benchmark.py` → `semio.benchmark.py`
   - Go: `benchmark/main.go` → `semio_benchmark.go`
   - Rust: `--example benchmark` → `--bin semio-benchmark`
   - Report output: `temp/benchmark.csv` → `reports/benchmark.csv`

## Notes

- The .NET project (`net/Semio.Benchmark/`) already followed the convention with a separate project, so no changes were needed
- The Go benchmark file uses `//go:build ignore` to allow it to coexist in the same directory as the semio package

## Summary

# Summary: Tidy Benchmark Files

Reorganized benchmark files across all language implementations to follow a consistent naming convention.

## Final Structure

| Language   | Location                                     |
| ---------- | -------------------------------------------- |
| Python     | `py/semio/semio.benchmark.py`                |
| JavaScript | `js/semio/semio.benchmark.ts`                |
| Rust       | `rs/semio/semio.benchmark.rs`                |
| Go         | `go/semio/semio_benchmark.go`                |
| .NET       | `net/Semio.Benchmark/Program.cs` (unchanged) |
| Reports    | `reports/benchmark.csv`                      |

## Changes Made

- Renamed 4 benchmark files to follow the `semio.benchmark.*` / `semio_benchmark.*` convention
- Moved Rust benchmark from `examples/` to root of `rs/semio/` and updated Cargo.toml
- Moved Go benchmark from `benchmark/` subdirectory to root of `go/semio/` with `//go:build ignore` constraint
- Created `reports/` directory for benchmark output
- Updated `./repo/cli/cmd_benchmark.go` with new file paths and commands
