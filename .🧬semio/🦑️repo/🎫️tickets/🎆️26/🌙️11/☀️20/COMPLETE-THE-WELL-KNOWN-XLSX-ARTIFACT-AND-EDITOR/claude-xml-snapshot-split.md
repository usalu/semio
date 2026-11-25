# XML Snapshot Crate Split (source-boundary refusal)

## Policy
- Refusal "Rust crate Source boundary exceeded before compiler: semio-s-artifact-stdio-xml-snapshot (active lines 4073/4000)" comes from the admission code in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts`.
- Counter: `readRustSourceClosureV1` (`.../📥️source/🟦️.ts`). The sum of `activeLines` of every source file reachable from the crate entry through `mod`/`#[path]`. Each file counts as `selectRustSourceV1(text, cfg).split("\n").length`, so inactive cfg code is excluded. Embedded files (`include_str!`) count 0. A crate also breaches on files/bytes limits.
- Remedy: split along domain lines into separate crates. Reuse `scan-closure-lines.mjs` / `count-rust-closure.mjs` (node 24 with `--experimental-strip-types`) to count.

## Before
`semio-s-artifact-stdio-xml-snapshot` = 4073 lines: model/retained/ownership (~1290), text codec (~1470), binary codec (282), sqlite codec (~1030).

## Why not one crate per io codec
The codecs implement foreign traits (`ArtifactDsl`, `ArtifactPack`, `ArtifactSqliteSnapshot`, ...) and inherent `impl XmlSnapshot` for `XmlSnapshot`. The orphan rule forces those impls to live with `XmlSnapshot`. So the split is by layer: the pure document model moves down, `XmlSnapshot` and the codecs stay.

## After
- New crate `semio-s-artifact-stdio-xml-snapshot-document` (1211 lines), `🧬️schema/📸️snapshot/📃️document/📦️packages/🦀️rust`. It mounts `📸️snapshot/🦀️.rs` (document model, boundary validation), `💡️markup-facts`, `🔤️lexemes` (new), `🧭️position`, `🧬️retained`, `🧺️ownership`.
- `semio-s-artifact-stdio-xml-snapshot` (2835 lines), same path and same name, so no dependent changes. It mounts `🧬️state/🦀️.rs` (new, holds `XmlSnapshot`), the mutation-support, conformance and the text/binary/sqlite codecs. It re-exports the document items by explicit `pub use` at the unchanged path `standards::v1_0::subsets::base::schema::snapshot`.
- New module `🔤️lexemes` takes `skip_ws`, `parse_name`, `parse_attr_value(_quoted)`, `xml_unescape_text`, `is_name_*`, `parse_doctype` out of the text codec, because `impl From<&str> for XmlDoctype` must live with `XmlDoctype`. The text codec re-imports them via `pub(crate) use`.
- Visibility widened by necessity: `XmlNodeList(pub Vec<XmlNode>)` (was `pub(crate)`). `position` stays `pub(crate)` in the facade. `lexemes` is a new public module.
- Cargo: new workspace member in the xml `Cargo.toml`. The snapshot Cargo.toml gains the dependency. The document Cargo.toml has dev-deps quick-xml, serde_json, test-fixtures.

## Changed files
- xml `Cargo.toml`, `Cargo.lock`
- snapshot `📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}`
- `🧬️schema/📸️snapshot/🦀️.rs` (model only), `🧬️state/🦀️.rs` (new), `🔤️lexemes/🦀️.rs` (new), `🧺️ownership/🦀️.rs` (one visibility)
- `🧬️schema/📸️snapshot/📃️document/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}` (new)
- `🚪️io/📝️text/📸️snapshot/🦀️.rs` (lexeme fns and From impl removed, re-import)
- 67 other `Cargo.lock` files that contain the snapshot crate (xlsx, hub, plugins). Each got only the added `-snapshot-document` package block plus one dependency line, derived from the index version with `relock-min.py`. The script output is byte-identical to what cargo produced for a reference lock (beams). A first `cargo metadata --offline` loop rewrote many locks wholesale. I reverted that to the minimal edit, so the diff is +1044/-0 over 76 lock files, 8 of which are other agents' own changes.

## Checks (nightly-2026-07-20, `-j 4`, own target dir, deleted afterwards)
- `cargo check --offline -p snapshot -p snapshot-document -p xml`: OK.
- xlsx workspace `cargo check --workspace`: OK (exit 0).
- xml workspace `--workspace --all-targets`: libs OK, the snapshot and document test targets compile. Test targets of `xml-schema` (13 errors: `xml_escape_attr` pub(crate), retained-test derive APIs) and `xml-mutations` (2 errors: `XmlDiff::inverse`, `KIND_ID`) fail. These are not caused by this change: they are in symbols or tests not touched here (xml_escape_attr stayed pub(crate); the rest are retained/diff/editor API drift).
- Line counts: snapshot 2835, document 1211.

## XLSX dependency closure scan (631 crates, lib targets, aarch64 macos cfg)
- No production crate over 4000.
- Over with `cfg(test)`: `semio-framework-mesh-engine` 4871.
- Near (>3600): framework-mesh-engine 3856, framework-tool-run (test) 3860, ui-contract-values 3699 (3852 test), os-channel 3635, framework-value-intrinsic (test) 3629.
- Scanner could not evaluate (needs generated-token observer or compiler ownership): ~29 entries such as `semio-os-kernel-dsl-derive-expand`, energy-model native-derive, plugin-reactor-lifecycle (test).
