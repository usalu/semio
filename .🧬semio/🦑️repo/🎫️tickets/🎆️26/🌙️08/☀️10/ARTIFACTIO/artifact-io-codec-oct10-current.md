# Artifact IO Codec Audit — Current Source, 2026-10-10

Read-only source audit. No production changes, Git mutations, runtime verification, or tests were performed. Conclusions apply to sampled source hashes below; concurrent changes can invalidate them. Wallet/control/Media/VCS intentionally excluded.

## Exact Unresolved Ownership

1. **Print schema contains actual record serialization.** `print/🧬️schema/🧬️mutations/🦀️.rs:14–27` implements `ToValue` and `FromValue` for `ChangeChartValue`, chooses field names, omission rules, and unknown-field rejection. Its `DslRecord` derive and opcode annotation at lines 7–8 additionally attach DSL representation to the semantic owner. `print/🧬️schema/🔀️diff/🦀️.rs:13–27` implements the equivalent projection/admission for `ChartEdit`; `ChartDiff` derives those codecs at line 30. Move representation implementations to receiving text IO; retain event fields and diff/inverse/purity methods in schema. `DslValue` event data alone is not evidence of a violation. Existing receiving evidence: `print/🧪️tests/🧬️chart-mutations/🦀️.rs` invokes these contracts at lines 7, 23, 32, 43 and tests JSON diff-wire fixtures around 348–352. Preserve omission versus explicit null, unknown keys, inverse and guarded composition, and the committed wire fixture tests when extracting.

2. **Generation3d schema still directly generates representation implementations.** `🧬️schema/📸️snapshot/🦀️.rs:11`, `🧬️schema/🦀️.rs:20,33`, and mutation leaves such as `✂️disconnect-synapse/🦀️.rs:10` derive `ToValue`/`FromValue`. These are real generated trait implementations, confirmed by `framework/🌱️value/✨️derive/🦀️.rs:44–54`, rather than semantic fields merely named text/binary. IO consumers invoke these projections: `🚪️io/🦀️.rs:307,314` serialize/admit the snapshot and `🚪️io/📝️text/🧬️mutations/🦀️.rs:156,183` project/admit widget input. Extraction of text/binary files has therefore not completed separation of schema-derived codecs. Keep native types/field laws; put projection/admission in IO. Existing receiving tests are registered from `🚪️io/💾️binary/🧬️mutations/🦀️.rs:87–89` and `🚪️io/📝️text/📸️snapshot/🗂️catalogue/🧪️tests/🔬️unit/🦀️.rs` uses a value roundtrip. Completion needs source ownership checks plus portable fixtures and independent oracle coverage of mutation, snapshot, omission, numeric payloads and rejection after extraction. Those checks were not run.

3. **Generation3d live representation helper remains in schema.** Feature-gated `🧬️schema/🦀️.rs:501–503` declares `gumball_widget_json` and projects a widget through `ToValue`; live `✏️editor/🎮️commands/🧭️transforms/🦀️.rs:600` still calls `schema::gumball_widget_json`. This is a concrete stale semantic-owner entry point. Schema also exposes JSON-string APIs `add_widget:336` and `reorganize:424`, forwarding to host, with real command consumers. Prefer typed host operations and move boundary decoding to IO. Distinguish these forwarding APIs from codecs: their method bodies do not themselves parse JSON. Inspection and set-widget-input tests already exercise host descriptor creation, but new boundary rejection/typed command coverage must verify the replacement.

## Resolved and Non-Findings

Replication canonical extraction is visibly real: `🚪️io/📝️text/🦀️.rs:17–18` registers canonical IO; that file now owns `ArtifactCanonicalJsonTree` projections and field ordering. `🎮️mutation/🧵️canonical/🦀️.rs` retains identity, authority, retirement. These semantic/retirement mechanisms are not representation violations. Existing IO-owned test `🚪️io/📝️text/🧵️canonical/🧪️tests/🦀️.rs:12` uses committed JSON fixture, serde JSON admission, backing-pointer and budget/pause laws. It was inspected, not executed.

Generation3d actual binary mutation implementations are registered under `🚪️io/💾️binary/🧬️mutations/🦀️.rs:43,58`, importing an IO-owned DSL mirror. IO-owned nested `🧬️mutations` and `🧬️schema` are representation facet schemas, not semantic-owner violations. Preview text mutation fields, binary64 semantic scalar names, catalogue localized text, fixture encodings and test-only serde use are not codec ownership evidence.

## Additional Replication Frontier (Broader Than Schema/Mutation)

`🔗️causal/🦀️.rs:981–1296` still implements HLC, envelope, frontier, exact batch and operation-vector byte serialization. This is genuine binary implementation beside causal semantics, not merely calls to `OpBinary` at 953–955. `🟦️.ts:419,430,519,786,988` similarly implements wire primitives, presence and HLC directly in module root. This is a broader IO ownership cleanup candidate, outside the strict schema/mutation finding. Existing Rust causal tests at 617–646 cover batch roundtrip and exact-limit fixtures; TypeScript and wire fixtures are declared by module source. No claim that all consumers or tests pass. Relocation must preserve strict varints, exact reencoding, limits, trailing bytes and cross-language fixtures without compatibility shims.

## Source Snapshot

Paths below are repository-relative. Length is absolute Unicode character count (not UTF-8 bytes); hashes are SHA-256.

| Source | Absolute Path Chars | SHA-256 |
|---|---:|---|
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 128 | `0d94544558c9b622538c4ee558149ce0d37c79a055a075aace0e957c0fc429cc` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` | 128 | `17c3b660b889aa3dd6595b03e7d65291fd7c3e18462db87d92cfeaac2f5bd905` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` | 139 | `e511c045b93c5fe3af20f1ebffe9d4cf7aeda0fcdd179cff21bdfc2e79c4c80f` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🦀️.rs` | 161 | `c896ade8f9e00dc3641c491d5c9517743906675386bfab9a571b5688d5efcbb6` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | 145 | `24abf1c83a9de083122ce92ead1d7316995d290ca3e50f358f6e409a4adc1253` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` | 143 | `12ff90f8b4a7ac909dc1e9bff586b40d9d8f259b917245af016ad4fdba641dc9` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️transforms/🦀️.rs` | 152 | `016603573233e6ab0d8b7ba6203b2c8a6fc645a1e491c98550796d7b0f2209d2` |
| `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🦀️.rs` | 92 | `ff1e72f53429bc8dd2517b35def678da0bb71386c4304827a9c7d7582f05ca41` |
| `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🦀️.rs` | 93 | `0a720691ed019b74bab53ad0b43844fde6054e27746d05bf4ec16aca892e5828` |
| `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs` | 78 | `6f8ccd74ec5a2be804661002f57d11ec7f9b92828547101a37e62e1f60a45eea` |
| `🧰️framework/🔨️modules/📡️replication/🟦️.ts` | 69 | `575efd98cd75855b28fa3f4941ccae1713cebb3097be76204ec3a962f715fed4` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🦀️.rs` | 85 | `f9fca70b76a5faf0f48460eedd69b982180807451778715fe7cd0d964aa1b729` |
| `🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🦀️.rs` | 80 | `77ca6c39da9c317e67e3f1f7b1f93d95e327d1360606f2204a7ff5cface58b8b` |
