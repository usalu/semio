# SPR Testkit Mutation Metadata Ownership

`ephemeral-transfer-preparation-native-1.log` exposed 26 source-authority failures. The Rust mutation leaves already belong to SPR command testkit, SPR testkit support, Store testkit, and Store presence-retirement testkit. Their descriptor and schema pairs still lived under fixture-data owners.

The exact descriptor/schema pairs now live beside their Rust owners. Each descriptor changed only its `owner` path. Payload schemas, semantic kinds, opcodes, binary tags, outcomes, aggregate variants, and schema identifiers remain unchanged. Aggregate law inputs remain in `🧫️fixtures`.

## Exact Moves

Each row moves both `🔣️.json` and `🧬️schema/🔣️.json` from the source directory to the destination directory.

| Previous directory | Source-owner directory |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️testkit/🧬️mutation-laws/🧬️mutations/➕️add-counter` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️testkit/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️testkit/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️testkit/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️testkit/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/📔️registry/🧬️mutations/📛️rename-mini` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️testkit/📔️registry/🧬️mutations/📛️rename-mini` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/➕️add-counter` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧩️support/🧬️mutation-laws/🧬️mutations/➕️add-counter` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧩️support/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧩️support/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧩️support/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧩️support/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/🔢️set-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🧮️demo/🧬️mutations/🔢️set-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/🗑️delete-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🧮️demo/🧬️mutations/🗑️delete-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/➕️add-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🧮️demo/🧬️mutations/➕️add-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/↩️restore-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🧮️demo/🧬️mutations/↩️restore-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🪤️lossy/🧬️mutations/🔢️set-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🪤️lossy/🧬️mutations/🔢️set-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/🔢️set-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🚦️severity/🧬️mutations/🔢️set-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/⚠️set-warning-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🚦️severity/🧬️mutations/⚠️set-warning-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/🚫️set-error-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🚦️severity/🧬️mutations/🚫️set-error-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/🛑️set-fatal-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🚦️severity/🧬️mutations/🛑️set-fatal-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/↩️restore-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🚦️severity/🧬️mutations/↩️restore-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/⏱️timestamped/🧬️mutations/🔢️set-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/⏱️timestamped/🧬️mutations/🔢️set-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/⏱️timestamped/🧬️mutations/↩️restore-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/⏱️timestamped/🧬️mutations/↩️restore-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🛂️validated/🧬️mutations/🔢️set-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🛂️validated/🧬️mutations/🔢️set-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🛂️validated/🧬️mutations/↩️restore-n` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🧬️mutations/🛂️validated/🧬️mutations/↩️restore-n` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧫️fixtures/🧬️mutations/🔢️set-value` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️testkit/🧬️mutations/🔢️set-value` |

## Updated Metadata Readers

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🧪️fixture-mutations-set-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-demo-add-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-demo-delete-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-demo-restore-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-demo-set-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-lossy-set-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-severity-restore-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-severity-set-error-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-severity-set-fatal-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-severity-set-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-severity-set-warning-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-timestamped-restore-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-timestamped-set-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-validated-restore-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-validated-set-n/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-four-times/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-sequence/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-then-notify-foreign/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-twice/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-missing-counter/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-observed-counter/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-rejected-counter/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-unchecked-counter/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🔣️.json`: six mutation-family `root` links now identify the actual Store testkit owners.

## Validation

- Static source-authority audit: 26 mutation leaves, 26 colocated descriptors, 26 colocated payload schemas, and exact descriptor `owner` equality: pass.
- Direct metadata-reader resolution: all 26 updated `include_str!` paths resolve to their colocated descriptors: pass.
- Native validation remains queued through the existing shared Cargo target; no duplicate build was started.
