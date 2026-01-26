# Summary: Tidy Benchmark Files

Reorganized benchmark files across all language implementations to follow a consistent naming convention.

## Final Structure

| Language | Location |
|----------|----------|
| Python | `py/semio/semio.benchmark.py` |
| JavaScript | `js/semio/semio.benchmark.ts` |
| Rust | `rs/semio/semio.benchmark.rs` |
| Go | `go/semio/semio_benchmark.go` |
| .NET | `net/Semio.Benchmark/Program.cs` (unchanged) |
| Reports | `reports/benchmark.csv` |

## Changes Made

- Renamed 4 benchmark files to follow the `semio.benchmark.*` / `semio_benchmark.*` convention
- Moved Rust benchmark from `examples/` to root of `rs/semio/` and updated Cargo.toml
- Moved Go benchmark from `benchmark/` subdirectory to root of `go/semio/` with `//go:build ignore` constraint
- Created `reports/` directory for benchmark output
- Updated `go/repo/cmd_benchmark.go` with new file paths and commands
