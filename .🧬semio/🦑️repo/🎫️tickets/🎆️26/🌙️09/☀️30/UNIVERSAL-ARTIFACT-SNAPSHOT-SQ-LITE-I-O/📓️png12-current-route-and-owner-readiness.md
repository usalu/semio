# PNG 1.2 Current Route and Owner Readiness

Read-only source audit; no Cargo or runtime receipt. Owner is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any`.

## Exact execution candidate

Registered no-argument route is `@semio-tech/stdio-png-rs:test-snapshot-sqlite-native`. Its script delegates package `semio-s-artifact-stdio-png` to the canonical artifact runner, selecting `--lib sqlite_snapshot_ --no-fail-fast`. Snapshot root mounts SQLite tests at lines 385–386 and SQL implementation at 388–389. Primary Rust test file has nine matching test functions and mounts `🚦️audit/🦀️.rs` at 123–124; the audit module has five matching test functions. Thus fourteen authored mounted test candidates, not fifteen: snapshot root's `sqlite_snapshot_codec` at 362 is a trait method. Actual selection/count requires Root's execution.

## Mounted and adjacent authority

Pack opt-in exists. Active SQL implements typed `to_sqlite_database` and `from_sqlite_database`; no native encode/decode/preflight overrides are mounted. Adjacent snapshot `🚦️native/🦀️.rs` and engine `🚪️io/🚦️native/🦀️.rs` remain separate candidate authority. Adjacent native control initializes from semantic `max_value_bytes`, rather than settling an actual cumulative allocation stage; do not treat it as paid mounted coverage. Active SQL's `entities` helper at 12 obtains an ordinary `ordered_rows` reference vector, and reconstruction has an ordinary `BTreeMap` text identity index at 96. These need paid frontier authority before a paid ownership claim.

## Authored semantic ownership

The SQL schema has sixteen concrete domain tables: document, palette and entries, transparency and alpha entries, gamma, chromaticity, sRGB, physical dimensions, modification time, background, text, pixels, unknown chunks and byte children, and chunk sequence. There are no JSON, native-file, or compressed-carrier columns. Unknown chunk kind is four authored octet columns; payload bytes are ordered child rows. Pixels have coordinates and RGBA octets. Palette entries, transparency alpha, text, unknown chunks, bytes, and chunk sequence carry ordinals and explicit parent references. Text/unknown sequence variants have checked nullable references; inactive transparency/background companions are explicitly constrained. Width and height retain the full u32 range, chromaticity/gamma/dimensions retain u32 integers, and sample companions retain u16. SQL fidelity is distinct from external PNG normalization; the mounted tests explicitly include this distinction.

## Source prerequisites, not measured compiler errors

Audit test helper `🚦️audit/🦀️.rs:10` returns `Result<PngSnapshot,String>` directly from the now-typed native trait method. This is a concrete typed signature mismatch candidate. Nine primary and five audit tests remain authored; audit exercises independent pngjs and node:zlib behavior, including Latin-1/iTXt, CRC/order/flags, and standalone Deflate restrictions.

Removed `dsl::json` references remain in thirty mutation production/test files. Production paths include replace-palette, change-header, change-srgb-intent, change-physical-dims, change-background, insert/remove-unknown-chunk, replace-pixels, change-gamma, change-timestamp, remove/insert/replace-text-chunk, change-transparency, and change-chromaticities. These are ordinary mutation JSON terminals, not permission to restore a legacy DSL JSON export. Exact first-party JSON authority and member policy must be ported at their actual terminals. No compiler error count is inferred from this source census.

Primary native input/output ceiling laws still set semantic `max_value_bytes:1` at lines 96 and 102. Preserve semantic SQL witnesses; review their intended owned backing role when the actual native provider is mounted, using the genuine owning baseline before fixture corrections.
