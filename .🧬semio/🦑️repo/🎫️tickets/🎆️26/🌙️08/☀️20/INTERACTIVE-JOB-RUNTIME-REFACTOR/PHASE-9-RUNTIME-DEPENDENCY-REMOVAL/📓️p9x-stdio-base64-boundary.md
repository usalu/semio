# P9x Stdio Base64 Boundary

## Outcome

Stdio no longer declares or calls the third-party `base64` crate directly. All 22 production image payload consumers now use one stdio-owned padded RFC 4648 standard-alphabet encoder, with differential parity against `base64 0.22` across known, boundary, adversarial, and large inputs.

This packet touched only the stdio manifest/glue, the 22 exact GIF/SVG/JPEG/PNG/BMP/TIFF editor/viewer window consumers, and Phase 9 ticket evidence. Renderer, MCP, FEM, shared async/worker-pool code, compression code, and unrelated stdio artifacts were not edited.

## Selection Census

The refreshed stdio direct-runtime census found:

- `base64`: 22 `STANDARD.encode` calls plus 22 `Engine` imports across 22 files; one direct manifest row; encode-only, standard alphabet, padded output.
- `flate2`: two calls in the already completed compression boundary and retained for consumed format behavior.
- `libz-sys`: one native compression implementation boundary, retained for exact Illustrator/legacy stream behavior.
- `serde` and `serde_json`: pervasive schema/wire contracts, not bounded dependency packets.

Base64 was therefore the largest bounded direct dependency whose entire stdio-owned use could be retired without crossing product ownership.

Initial exact call-site evidence: `📝️p9x-stdio-direct-runtime-census.txt`.

## Implementation

- Removed `base64 = "0.22"` from `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`.
- Added `base64_standard(&[u8]) -> String` in `📦️glue.rs`:
  - RFC 4648 standard alphabet;
  - canonical `=` padding;
  - exact preallocation;
  - no external runtime types in the public API.
- Migrated exactly 22 image-view payload construction sites:
  - GIF 87a/89a editor and viewer: 4;
  - SVG any/basic/tiny editor and viewer: 6;
  - JPEG any/baseline editor and viewer: 4;
  - PNG any editor and viewer: 2;
  - BMP any editor and viewer: 2;
  - TIFF any/baseline editor and viewer: 4.

## Semantic Parity

The ticket-local differential binary links the changed production stdio crate and keeps `base64 0.22` only as a test reference. It verifies:

- the seven RFC 4648 progressive `foobar` vectors, including empty input and both padding widths;
- every prefix length from 0 through 4096 of a deterministic adversarial byte stream;
- one deterministic 1 MiB input.

Command:

```sh
CARGO_TARGET_DIR='.../PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-stdio-compression' \
  cargo run --manifest-path '.../🧪️p9v-stdio-compression-harness/Cargo.toml' --bin p9x
```

Result: exit 0, `vectors=7`, `differential_lengths=4097`, `large_bytes=1048576`, `base64_boundary=pass`. Evidence: `📝️p9x-base64-differential.txt`.

## Compiler Gates

All Cargo gates used the Phase 9 ticket-local `🧪️target-stdio-compression` directory.

```sh
cargo check -p semio-s-plugin-stdio --lib --message-format=json
cargo check -p semio-s-plugin-stdio --lib --release --message-format=json
cargo check -p semio-s-plugin-stdio --lib --target wasm32-unknown-unknown --message-format=json
```

| Gate | Build | Errors | Existing warnings |
|---|---:|---:|---:|
| native | success | 0 | 658 |
| release | success | 0 | 658 |
| wasm32-unknown-unknown | success | 0 | 645 |

Structured logs: `📝️p9x-stdio-base64-native.json`, `📝️p9x-stdio-base64-release.json`, `📝️p9x-stdio-base64-wasm.json`; aggregate: `📝️p9x-stdio-base64-gate-counts.txt`.

## Final Census And Dependency Boundary

`📝️p9x-stdio-base64-final-census.txt` records:

```text
before_direct_source_lines=44
before_consumer_files=22
after_direct_manifest_source_lines=0
after_owned_consumer_files=22
```

- `📝️p9x-stdio-base64-direct-tree.txt`: stdio's depth-one normal/build dependency tree has no direct `base64` edge.
- `📝️p9x-stdio-base64-inverse-tree.txt`: `base64` remains transitive through `semio-framework-os-kernel`; that shared boundary is outside this stdio-only packet and was not masked.
- `📝️p9x-stdio-base64-diff-check.txt`: scoped `git diff --check` exits zero.

## Remaining Blockers

- The monolithic stdio test target remains independently blocked by the previously measured 898 primary test-only diagnostics across 241 file/code groups. This packet did not suppress, modify, or reclassify them; exact ownership remains in `📝️p9v-stdio-lib-tests-final.json` and `📝️p9v-stdio-lib-tests-final-errors.tsv`.
- Native/release/WASM production checks are green but retain the existing warning backlog shown above.
- Further owned-codec consolidation requires coordinator arbitration because the remaining direct compression and shared framework codec boundaries overlap prior ownership decisions. No next dependency packet was started.
