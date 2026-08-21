# P6d PCG Job

## Result

`PcgJob` is a synchronous persistent state machine. Its checkpoint owns the matrix, right-hand side, `x/r/z/p/Ap`, Jacobi diagonal, reductions, iteration/residual state, and fine-grained row/entry/vector cursors.

Stages are diagonal initialization, initial SpMV/residual/precondition, then bounded iteration SpMV/update/precondition/direction. Each sparse entry or vector element consumes fuel and checks deadline/cancellation. Stale operation/generation contexts fault before mutation.

The coarse-quality lane publishes the first solution/residual/reaction/absolute-contour preview below `max(final_tolerance, 1e-3)`. Final commits retain the requested tolerance and convergence state. Hot micro-steps return `Yield`; they do not repeatedly serialize whole solution vectors.

`pcg` is now a non-interactive batch adapter over the same job implementation, so reference and interactive paths cannot drift algorithmically.

`serde_json` uses `float_roundtrip`, making serialized checkpoints byte-stable across restore/re-encode and preserving bit-exact replay.

## Verification

Ticket-local solver harness commands:

```text
CARGO_TARGET_DIR="$PWD/<ticket>/🧪️target-p6-harness" cargo test --manifest-path "<ticket>/🧪️harness-p6/Cargo.toml" -- --nocapture
CARGO_TARGET_DIR="$PWD/<ticket>/🧪️target-p6-harness" cargo test --release --manifest-path "<ticket>/🧪️harness-p6/Cargo.toml" -- --nocapture
CARGO_TARGET_DIR="$PWD/<ticket>/🧪️target-p6-harness" cargo check --target wasm32-unknown-unknown --manifest-path "<ticket>/🧪️harness-p6/Cargo.toml"
```

Results:

- debug: 22 passed, 0 failed;
- release: 22 passed, 0 failed;
- wasm32-unknown-unknown: success; three pre-existing `semio-framework-async` qualification warnings;
- adversarial 20,000-equation diagonal PCG step with one fuel unit: below 8 ms in debug and release;
- batch size 1 versus 97: bit-exact solution and stats, also bit-exact with the `pcg` adapter;
- checkpoint restore: byte-stable checkpoint plus bit-exact final solution/stats;
- stale generation and pre-cancelled token: terminal outcome without state mutation;
- coarse preview: emitted below `1e-3` and before `1e-12` final tolerance.

Evidence: `📝️p6-solver-harness-debug-tests.txt`, `📝️p6-solver-harness-release-tests.txt`, `📝️p6-solver-harness-wasm-check.txt`, `📝️p6-pcg-coarse-preview.txt`, and `📝️p6-checkpoint-focused.txt`.

