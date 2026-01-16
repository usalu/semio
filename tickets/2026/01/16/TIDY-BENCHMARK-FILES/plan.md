# Plan: Tidy Benchmark Files

## Current State

| Language | Current Location | Target Location |
|----------|------------------|-----------------|
| Python | `py/semio/benchmark.py` | `py/semio/semio.benchmark.py` |
| JavaScript | `js/semio/benchmark.ts` | `js/semio/semio.benchmark.ts` |
| Rust | `rs/semio/examples/benchmark.rs` | `rs/semio/semio.benchmark.rs` |
| Go | (none) | `go/semio/semio_benchmark.go` |
| .NET | `net/Semio.Benchmark/Program.cs` | No change (already follows convention) |
| Reports | `benchmark.csv` | `reports/benchmark.csv` |

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

6. **Update References**: Check and update any scripts or configs that reference the old file paths (e.g., `go/repo/cmd_benchmark.go`)

## Notes

- The .NET project already follows the convention with a separate `Semio.Benchmark` project
- Go doesn't have a benchmark file yet - need to check if this should be created or if it's out of scope
- The `temp/benchmark.csv` appears to be a temporary file and can be ignored
