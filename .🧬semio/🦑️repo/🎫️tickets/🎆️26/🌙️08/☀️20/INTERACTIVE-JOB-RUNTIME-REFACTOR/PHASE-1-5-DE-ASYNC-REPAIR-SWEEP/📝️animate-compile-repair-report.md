# Animate Compile Repair

## Outcome

The `semio-s-plugin-animate` backlog was reduced from 1,296 compiler errors and 13 warnings to a clean package build at the final stable shared-framework boundary. The repair restores the generated schema families, closes the dynamic enum at its semantic declaration boundary, removes stale suspension at exact compiler spans while retaining real asynchronous work, and preserves the native, browser-WASM, and WASI component surfaces. The canonical component descriptor SHA-256 is `5fff7e3ac148177243275445e12535fd89c433f6fa50316572bcdda9b3d97590`.

## Implementation

- Restored generated `Animations` and `Sobjects` collections and the corresponding closed dynamic-enum wiring.
- Repaired stale future/value mismatches across generated schema, mutation, command, example, IO, editor, and viewer code.
- Kept genuine suspension for host requests, rendering, storage, and asynchronous test setup.
- Replaced thread-local Present storage with process-visible `OnceLock<RwLock<_>>` state and retained mutation visibility across runtime threads.
- Repaired the WASM Present store/factory and target dependency boundary.
- Repaired optional preview-window compilation and the synchronous `ApplicationHandler` boundary around asynchronous renderer creation.
- Added the test-only async macro dependency required by generated async tests.
- Removed stale direct `comemo`, `ecow`, `fontdb`, and `base64` dependencies and retained owned `String` data at public boundaries.
- Cleared all Animate-owned focused clippy findings. The few lint attributes are scoped to deliberate schema-first nested modules, framework-required command signatures returning the upstream large `Fault`, and test modules that intentionally precede generated definitions.

## Verification

| Gate | Result | Evidence |
| --- | --- | --- |
| Canonical descriptor generation | PASS | `📝️r74-animate-final-stable-describe-wasip2.txt` |
| Native all-target tests after descriptor generation | PASS, 307/307 | `📝️r75-animate-final-post-descriptor-native-all-targets.txt` |
| All-features/all-targets no-run | PASS | `📝️r56-animate-final-all-features-all-targets-norun.txt` |
| Native release library check | PASS | `📝️r71-animate-final-stable-native-release.txt` |
| `wasm32-unknown-unknown` library check | PASS | `📝️r72-animate-final-stable-wasm32-unknown-unknown.txt` |
| `wasm32-wasip2` library check | PASS | `📝️r73-animate-final-stable-wasm32-wasip2.txt` |
| Focused text tests | PASS, 32/32 | `📝️r67-animate-final-text-and-example-tests.txt` |
| Focused generated example tests | PASS, 3/3 | `📝️r69-animate-final-example-tests.txt` |
| Strict `-D warnings` | BLOCKED upstream, exactly 49 diagnostics | `📝️r70-animate-final-stable-native-strict-warnings.txt` |
| Focused Animate clippy | PASS, zero Animate warnings | `📝️r66-animate-final-clippy-lib-tests.txt` |
| Format | PASS | `📝️r65-animate-final-fmt-check.txt` |
| Interactive progress/cancellation torture gate | PASS, 6/6 | `📝️r53-interactive-job-torture-gates.txt` |
| Direct-dependency and `[DEBUG]` ratchets | PASS | `📝️r54-animate-ratchets-debug-suspension.txt`, `📝️r55-animate-direct-dependency-tree.txt` |

The strict command reached no Animate-owned diagnostic. It stopped in `semio-framework-plugin` with exactly 49 upstream warnings promoted to errors, including unused imports/doc comments, unnecessary qualifications, ambiguous glob re-exports, dead reactor fields/functions, and two redundant reference clones. No broad lint suppression was added.

The final package matrix used these commands with `RUSTC_WRAPPER=`, `CARGO_BUILD_JOBS=4`, and `CARGO_TARGET_DIR="$TICKET/target-animate"` where applicable:

```sh
bun "$REPO/s/plugins/animate/📜️script.ts" describe
cargo test -p semio-s-plugin-animate --all-features --all-targets --no-run
cargo test -p semio-s-plugin-animate --all-targets
cargo test -p semio-s-plugin-animate text -- --nocapture
cargo test -p semio-s-plugin-animate examples -- --nocapture
cargo check -p semio-s-plugin-animate --lib --release
cargo check -p semio-s-plugin-animate --lib --target wasm32-unknown-unknown
cargo check -p semio-s-plugin-animate --lib --target wasm32-wasip2
RUSTFLAGS='-D warnings' cargo check -p semio-s-plugin-animate --lib
cargo clippy -p semio-s-plugin-animate --lib --tests -- -D warnings
cargo fmt --all --check
```

The ordinary upstream warning census remained visible: the strict boundary reports 49 `semio-framework-plugin` diagnostics, while focused clippy also reports upstream framework and stdio warnings. Animate itself emits none.

The interactivity behavior command ran the framework job engine used by the plugin:

```sh
RUSTC_WRAPPER= CARGO_BUILD_JOBS=4 CARGO_TARGET_DIR="$TICKET/target-animate" cargo test -p semio-framework-job torture_job_ -- --nocapture
```

It proved continuous previews, the per-step watchdog ceiling, cancellation below the 8 ms p99 exit gate, deterministic replay across worker counts, and checkpoint/restore equivalence.

## Ratchets

- Direct stale dependencies: 0.
- Animate `[DEBUG]` markers: 0.
- Remaining Animate suspension census: 146 `async fn` declarations and 240 `.await` sites; these compile on native and both required WASM targets and represent retained asynchronous boundaries rather than stale value/future mismatches.

## Isolation

All package compilation used the ticket-local `target-animate` directory. No workspace-wide build was launched by this repair. The final process census found no active `cargo`, `rustc`, or `bun` process using the isolated target; its 36 GiB can be safely reclaimed after handoff. See `📝️r76-animate-final-target-census.txt`.
