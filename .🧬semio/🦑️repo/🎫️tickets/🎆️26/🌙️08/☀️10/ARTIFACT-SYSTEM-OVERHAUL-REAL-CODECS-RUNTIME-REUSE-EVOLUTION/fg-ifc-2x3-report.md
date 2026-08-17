# ifc/2x3 — real grammar/protocol + JSON-transfer-ban elimination — report

**Scope**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/**` only (2x3, never touched `4`).

## What was wrong (confirmed by W0/F6/STATUS.md census before this session)

1. All 6 `.grammar.semio`/`.protocol.semio` files (snapshot/diff/mutations × text/binary) were still
   the pre-Phase-2 ABNF placeholder stub (`dialect grammar stdio.ifc.2x3.snapshot` / `payload = *OCTET`
   hex-dump shape) — never upgraded to the real dialect.
2. `Ifc2x3Mutation`'s `OpText::print_op`/`parse_op` and `OpBinary::encode_op`/`decode_op` were literal
   `serde_json::to_string`/`from_str`/`to_vec`/`from_slice` — the program's own census named this file
   explicitly as "the only remaining" `POLICY_STDIO_JSON_TRANSFER_BAN` violation.
3. `Ifc2x3Diff` had **no `protocol::DiffCodec` impl at all** — confirmed in STATUS.md as "the 1
   remaining breach" of `dsl-migration/diff-completeness` across all 32 stdio standards.

## What changed

- **Grammar/protocol files (6)**, real Part-21 dialect, restated (not `use`d) from `4`'s own
  already-landed family, adapted to this standard's actual types (`Part21Header`/`Part21Instance`/
  `Part21Value`, not `4`'s own `IfcHeader`/`IfcEntity`/`IfcValue`):
  - `📸️snapshot/📝️text/📖️component.grammar.semio` — real ISO 10303-21 exchange-file grammar
    (`comment line none` + `comment block "/*" "*/"` + `string single doubled`, COMPLEX-instance
    `instance-body` alternation).
  - `📸️snapshot/💾️binary/📡️component.protocol.semio` — `framing record` + `chain payload utf8`
    (text-native, matches `4`'s own snapshot protocol shape).
  - `🔺️diff/📝️text/📖️component.grammar.semio` — real `schema=`/`header=`/`removed=`/`upserted=`
    token grammar for `Ifc2x3Diff`'s actual field shape (no collection-triple needed — this diff has
    no `modified` list, only `removed`+`upserted`, unlike `4`'s id-keyed `entities` triple).
  - `🔺️diff/💾️binary/📡️component.protocol.semio` — real `header fixed 2` (`format`/`flags`) +
    `chain payload bytes` frame.
  - `🧬️mutations/📝️text/📖️component.grammar.semio` — real `keyword key=value` grammar for the 5
    `Ifc2x3Mutation` variants.
  - `🧬️mutations/💾️binary/📡️component.protocol.semio` — real `header fixed 2` (`format`/`tag`) +
    `chain payload bytes` frame.
- **`🔺️diff/🦀️component.rs`**: added the full hand-rolled `protocol::DiffCodec` impl (didn't exist
  before) — `print_diff`/`parse_diff` (text) and real `encode_diff`/`decode_diff` (binary, varint/
  length-prefixed, `store::pack_rt`/`store::ByteReader`, zero serde/JSON). Added the `Part21Value`
  9-variant tag-scheme codec (text + binary, isomorphic to `4`'s own `IfcValue` scheme:
  `U`/`D`/`I[n]`/`R[n]`/`S[hex]`/`E[hex]`/`F[n]`/`A[items]`/`T[hex,[items]]`), `Part21Header`/
  `Part21Instance` codecs (incl. real COMPLEX-instance, multi-entity support), and
  `demo_diff_cases()`. Extended the existing test module (not a new file) with
  `diff_codec_text_binary_roundtrip_law`.
- **`🧬️mutations/🦀️component.rs`**: replaced the `serde_json::{to_string,from_str,to_vec,from_slice}`
  `OpText`/`OpBinary` impls with real hand-rolled text (`keyword key=value` lines) and binary
  (`format u8 | tag u8 | payload`) codecs, reusing the diff sibling's `pub(crate)` primitives (same
  intra-artifact-reuse split `4` uses). Added `demo_mutation_cases()` and extended the existing test
  module with `op_text_binary_roundtrip_law`.
- **`⚙️engine/🦀️component.rs`**: added a real non-empty `demo_ifc2x3_snapshot()` (declares
  `FILE_SCHEMA(('IFC2X3'))` so `decode_ifc2x3`'s own gate accepts it — a concurrent edit in this same
  live tree had left a stub `demo_ifc2x3_snapshot() -> empty_ifc2x3_snapshot()` that I replaced with
  the real one), the 5-role `register_pilot_languages()` call (Document/Ops/Diff/Pack/Spr, `diff`'s
  `protocol: None` matching the 5-role scheme's own shape), and the 6 conformance-law tests
  (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
  `diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) in a `conformance_laws`
  submodule of the existing test module.
- **New fixtures** (own standard-local copy, `🏅️standards/🔖️2x3/📚️examples/🎬️demo/🖼️assets/`, not
  the artifact-level `ifc/📚️examples/` shared with standard `4`'s UI-facing demo entry — out of this
  standard's ownership): `🗣️example.dsl.semio`, `🎒️example.pack.semio`, generated via a temporary
  `#[ignore]`d test that called the real `store::ArtifactDsl::print_dsl`/`store::ArtifactPack::encode_pack`
  directly, run once, then deleted (no `[DEBUG]`/temp code remains in the tree).

## `register_schema_spec` — not called (mechanism gap, not a mistake)

`Part21Value` is a genuine data-carrying enum with no `DslField` impl (identical root cause `4`'s own
`IfcValue`/`IfcSnapshot`/`IfcDiff` doc comment documents), so no `fn() -> RecordSpec` exists for
`Ifc2x3Snapshot`/`Ifc2x3Diff`. Filed as `mechanism_gaps`, not fabricated.

## Verification

- `cargo check -p semio-s-plugin-stdio --lib` — clean (only pre-existing unrelated warnings across
  other artifacts, none from ifc/2x3).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::ifc"` — **82 passed, 0 failed**, covering both
  `v4` and `v2x3`, including all 6 new conformance-law tests for `v2x3`
  (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
  `diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) and the two new
  round-trip law tests (`diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) — **1834 passed, 4 failed**; all 4 failures
  are in `bcf`/`dwg`/`pptx`/`xlsx` (unrelated artifacts, other concurrent sessions' in-progress work
  per this ticket's own repo-rules digest — classified by file path, not chased, zero overlap with
  `ifc`).
- `grep -rn serde_json` inside `🏗️ifc/🏅️standards/🔖️2x3/` — zero live calls, only historical mentions
  inside doc comments/grammar-file prose explaining what was replaced.
- `bun run ./📜️script.ts policy` — grepped the full high-priority breach report for every
  `ifc.*2x3`/`2x3.*ifc` match: **zero** hits for `dsl-migration/diff-completeness`,
  `stdio-artifacts/json-transfer-ban`, `handcrafted-grammar/*`, `grammar-parseability`,
  `protocol-parseability`, `fixture-honesty`, or `language-registration`. The only remaining ifc/2x3
  mentions anywhere in the report are pre-existing, unrelated taxonomy/composer/os-state-authority
  findings (emoji-prefix variation selectors on directory names, `ComposerEntry`/`OnceLock` shape —
  none touched by this ticket's own checklist, none in this session's scope).

## One stale entry this session could not remove (out of ownership boundary)

`📜️script.ts`'s `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` still contains
`"stdio/ifc/standards#2x3-subsets-any-schema-mutations-component"`. Since the file no longer uses
`serde_json` anywhere, the policy script's own logic (`📜️script.ts:9970-9979`) now classifies this as
a `priority: "low"` **stale-allowlist** breach ("is allowlisted... but no longer uses serde_json"),
not a real violation — confirmed by reading the exact check body. Per the ticket's own repo rules I
must not touch `📜️script.ts`; this stale entry needs the ticket's periodic policy-shrink pass to
remove it (same mechanism the recipe's own §4 checklist describes: "your wave's closer or the
program's own periodic graduation pass appends/prunes your standard's entries").

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- New: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`
- New: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio`

Not touched: `📦️glue.rs`, `📜️script.ts`, any SDK/schema/dsl/protocol/registry module, `🧪️fixture-sweep`,
`🏪️store`, and `🏗️ifc/🏅️standards/🔖️4/**` (the other, already-completed standard).
