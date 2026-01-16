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

6. **Updated References** in `go/repo/cmd_benchmark.go`:
   - TypeScript: `benchmark.ts` → `semio.benchmark.ts`
   - Python: `benchmark.py` → `semio.benchmark.py`
   - Go: `benchmark/main.go` → `semio_benchmark.go`
   - Rust: `--example benchmark` → `--bin semio-benchmark`
   - Report output: `temp/benchmark.csv` → `reports/benchmark.csv`

## Notes

- The .NET project (`net/Semio.Benchmark/`) already followed the convention with a separate project, so no changes were needed
- The Go benchmark file uses `//go:build ignore` to allow it to coexist in the same directory as the semio package
