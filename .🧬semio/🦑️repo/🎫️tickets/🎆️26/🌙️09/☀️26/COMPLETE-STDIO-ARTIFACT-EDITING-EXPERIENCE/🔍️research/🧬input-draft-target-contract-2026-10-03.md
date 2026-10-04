# Input Draft Target Contract Checkpoint

## Implemented boundary

`InputProps` schema version 5 carries optional opaque `draftTarget: UiText`. The field defaults on decode, is omitted by the serde wire when absent, is generated as `draftTarget?: string | null` for TypeScript, and occupies typed-value ordinal 12. `InputBuilder::draft_target` is the fluent construction path.

The neutral typed-wire fixture covers absent and present targets. The Rust law decodes that JSON fixture through serde, round-trips the same `InputProps` through the first-party typed `Value` wire, and compares the emitted serde field. The builder law independently proves fluent carriage into the component wire.

Revisioned `TableWindowKit` short cells author a length-prefixed semantic tuple containing controller id, row key, logical column, and action id. The tuple excludes command arguments, so a new revision for the same cell retains its draft while changes to controller, row, column, or action reset it. The producer does not inspect or strip arbitrary argument fields.

The current CSV producer supplies positional keys (`row-{index}`). The identity is stable only while the semantic row at that position is unchanged. A structural insertion, deletion, reorder, or replacement must publish a different row identity or make the queued commit refuse/reset; otherwise the same positional target could preserve a draft for a different record. This cut does not guess structural identity from revision-bearing command arguments. The root lane is auditing queued structural-conflict behavior separately.

Stdio snapshot Details short scalar and key controls author a length-prefixed controller/action/address tuple. If either the visible value or target exceeds the bounded inline representation, the existing paged text-draft surface remains authoritative. PDF 1.7 inspector inputs author controller/action/page/object/field identities.

The retained browser typed normalizer admits `draftTarget` in the Input field vocabulary and owns its optional text in the normalized component. Its law sends the shared native-shaped typed fixture through `RetainedUiTypedCursor`, covering the actual intake phase that rejected the live CSV preview.

## Resident accounting

Adding `Option<UiText>` changes the native64 layout. The exact neutral witness records `Option<UiText> = 516`, `InputProps = Component = 3,752`, and `UiPatchOp = 7,072` bytes. The effective operation and node-record increase is 480 bytes. Resident document credit changes from `128 × 6,592 = 843,776` to `128 × 7,072 = 905,216`, and the sixty-four-slot aggregate changes to `57,933,824`. A populated 128-node root now costs `921,266` bytes. The six-surface retained set costs `3,561,580`, and the deliberately double-buffered order costs `7,123,160`.

The fixed resident backing remains `2,077,760`: node bodies live in the retained paged authority rather than an inline 1,024-record arena. The first focused run disproved an initial inline-arena assumption; the corrected fixture preserves the independently measured fixed backing while updating only values derived from record layout.

## Owned source inventory

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧾️typed/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧫️fixtures/🧾️typed/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🧪️tests/🧪️typednodefields-preflights-every-capture-before-transfer-under-private-r/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️component-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️builder-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/{🧫️fixtures,🧬️schema}/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/🧪️tests/🩹️patch/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/{🧫️fixtures,🧬️schema}/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/{🗃️fixed,🌳️root,🔄️refresh}/{🧫️fixtures,🧬️schema}/🔣️.json`
- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-window-kits/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` (test assertion updated to the structured `ValueError.message` API)

## Validation

- Registered generator: `NX_DAEMON=false CARGO_TARGET_DIR=<ticket>/🗑️generated/draft-target-ui-contract-target bun nx run @semio-tech/ui-contract-rs:generate --skip-nx-cache` — **1/1 passed** and refreshed the generated TypeScript mirror. Receipt: `🗑️generated/input-draft-target-generate-1.log`.
- Exact UI contract laws: `NX_DAEMON=false CARGO_TARGET_DIR=<ticket>/🗑️generated/draft-target-ui-contract-target bun nx run @semio-tech/ui-contract-rs:test-quick --skip-nx-cache -- draft_target` — **5/5 passed**, 228 skipped, cache skipped. Receipt: `🗑️generated/input-draft-target-ui-contract-exact-4.log`.
- Stdio production assembly: `NX_DAEMON=false CARGO_TARGET_DIR=<ticket>/🗑️generated/draft-target-stdio-target bun nx run @semio-tech/stdio-assembly-rs:check --skip-nx-cache` — **passed**, including `semio-s-artifact-stdio-contract` and `semio-s-plugin-stdio`; cache skipped. Receipt: `🗑️generated/input-draft-target-stdio-check-1.log`.
- Table semantic-address law: `NX_DAEMON=false CARGO_TARGET_DIR=<ticket>/🗑️generated/draft-target-framework-plugin-target bun nx run @semio-tech/framework-plugin:test --skip-nx-cache -- table_cell_draft_target_tracks_semantic_address_without_revision` — **1/1 passed**, 977 skipped, cache skipped. Receipt: `🗑️generated/input-draft-target-table-window-kit-exact-1.log`.
- PDF inspector mount: `NX_DAEMON=false CARGO_TARGET_DIR=<ticket>/🗑️generated/draft-target-framework-plugin-target bun nx run @semio-tech/stdio-pdf-rs:test --skip-nx-cache -- --features component-app-assembly selected_object_fields_commit_through_the_value_argument` — **1/1 passed**, 705 skipped, cache skipped. Receipt: `🗑️generated/input-draft-target-pdf-inspector-exact-4.log`.
- Browser retained normalization: `NX_DAEMON=false bun nx run @semio-tech/framework:test-quick --skip-nx-cache` — **6 files / 306 tests passed**, plus the 25-law retained strip oracle, cache skipped. Receipt: `🗑️generated/input-draft-target-browser-normalization-1.log`.
- Native64 layout law: `NX_DAEMON=false CARGO_TARGET_DIR=<ticket>/🗑️generated/draft-target-ui-contract-target bun nx run @semio-tech/ui-contract-rs:test-quick --skip-nx-cache -- instance_lifetime_ui_patch_storage_first_payload_does_not_reserve_logical_capacity` — **1/1 passed**, 232 skipped, cache skipped. Receipt: `🗑️generated/input-draft-target-patch-layout-green-1.log`.
- Resident accounting group: `NX_DAEMON=false CARGO_TARGET_DIR=<ticket>/🗑️generated/draft-target-ui-contract-target bun nx run @semio-tech/ui-contract-rs:test-quick --skip-nx-cache -- resident` — **14/14 passed**, 219 skipped, cache skipped. Receipt: `🗑️generated/input-draft-target-ui-resident-focused-2.log`.
- Full UI contract quick suite: `NX_DAEMON=false CARGO_TARGET_DIR=<ticket>/🗑️generated/draft-target-ui-contract-target bun nx run @semio-tech/ui-contract-rs:test-quick --skip-nx-cache` — **233/233 passed**, zero skipped, cache skipped. Receipt: `🗑️generated/input-draft-target-ui-contract-quick-green-2.log`.

Earlier PDF attempts exposed concurrent Nx graph movement, a stale `ValueError::contains` assertion, and a featureless invocation that selected zero UI tests. The final invocation enables `component-app-assembly` and is green. The first broad UI run correctly exposed stale allocation fixtures caused by this field: `UiPatchOp` measured `7,072` bytes while the fixture still expected `6,592`. The allocation witness, resident authority fixtures, and their schemas now carry the measured layout.

## Remaining acceptance boundary

The React acknowledgement queue and end-to-end browser acceptance belong to the root lane. The native and retained browser contract now accept the identity, but rapid CSV input still requires the live preview to prove twenty queued edits settle in order and a semantic target change clears the draft. Positional CSV row keys also require the structural-conflict audit described above. Long table cells and oversized Details values continue through their paged text-draft components rather than this inline input field.
