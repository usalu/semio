# W-S Codec Wave — `stdio.semio.model` (`✳️model` subset)

Second real-codec wave for a **semio** subset, replicating `stdio.semio.workflow`'s proven,
fully-verified P2 pilot pattern (`ws-codec-workflow-report.md`, `📖️grammar-recipe.md`) onto
`model`'s three facets (snapshot, diff, mutations). Written after real, synchronous,
foreground-observed `cargo check`/`cargo test` runs — every number below was watched, not assumed.

---

## 1. Derive path vs hand-rolled — what actually happened

Per this wave's brief, the derive path was tried first now that the shared geometry types
(`SemioPoint2/3`, `SemioUv`, `SemioRgba`, `SemioQuaternion`, `SemioTransform` in
`⚙️engine/🧮️geometry/🦀️component.rs`) all derive `dsl::DslRecord` (confirmed by reading that file —
this was fixed centrally, outside this ticket's own edit scope, after the workflow pilot flagged
the gap).

**New blocker found (not the same one workflow hit):** `model`'s own snapshot type tree is not
just geometry-valued fields — it is built from several **hand-rolled enums with no `dsl::DslField`
bridge**: `SpatialKind`, `ElementClass` (tagged, incl. an `Other{name}` catch-all), `GeometryRef`
(tagged), `PsetValue` (tagged), `RelationKind` (tagged). None of these derive `dsl::DslRecord`/
`DslEnum` (they're plain `serde`-only enums), and — per `f6-final-summary.md` §4.4 (cited by this
subset's own `🔺️diff/🦀️component.rs` doc comment, itself written before this wave) — there is no
generic derive bridge for tagged/hand-rolled enum fields inside a `#[derive(dsl::DslArtifact)]`
struct. Adding `#[derive(dsl::DslRecord/DslEnum)]` to these 5 enums would be a legitimate, narrowly
-scoped fix, but doing so is **outside this wave's `🪆️subsets/✳️model/`-only edit scope only in the
sense that these enums themselves live inside `✳️model/🧬️schema/📸️snapshot/🦀️component.rs`** — i.e.
they ARE in-scope to edit, but adding a derive macro to types whose Rust shape (a `#[serde(tag =
"kind")]` enum with a struct-like `Other { name: String }` variant) has not been independently
confirmed compatible with `dsl::DslEnum`'s exact expected shape was judged too risky to gamble
mid-wave against the hand-rolled path's already-proven correctness. **Decision**: hand-rolled
`ArtifactDsl`/`ArtifactPack` for the snapshot (never regressing to the hex-of-JSON/plain-JSON
shortcut), matching `model`'s own `🔺️diff`/`🧬️mutations` facets' pre-existing hand-rolled
hex/bracket convention (itself modeled on `GifDiff`/`SvgDiff`/`DocxDiff`'s repo-wide precedent) —
same overall decision workflow's pilot made, for a related-but-distinct reason.

**Filed as a mechanism-gap note** (not a blocking gap, since it doesn't block THIS wave — `model`'s
hand-rolled path is fully proven and green): a future wave with time to spend could add
`#[derive(dsl::DslEnum)]` to `SpatialKind`/`ElementClass`/`GeometryRef`/`PsetValue`/`RelationKind`
and re-attempt the derive path for `model`'s snapshot, once someone independently confirms
`DslEnum`'s exact support for a struct-variant catch-all (`Other { name: String }`).

---

## 2. Per-facet checklist (grammar recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` in `📸️snapshot/🦀️component.rs` now produce/consume
  a genuine 4-line structured body: `schema=<hex>`, `spatial=[<node>,...]`, `elements=[<element>,...]`,
  `relations=[<relation>,...]`, every field its own token (hex-encoded strings, bracket-nested
  records, `[0]`/`[1,x]` tri-state for `Option<String>`). Replaces the old
  `serde_json::to_vec`+hex-dump shortcut entirely. Preamble handling unchanged.
- [x] **Real binary pack** — `encode_pack_with`/`decode_pack_with` now call
  `encode_model_snapshot_binary`/`decode_model_snapshot_binary`: `format u8` + varint-length-
  prefixed `schema` UTF-8, then varint spatial/element/relation counts and per-field
  varint-length-prefixed strings, real 8-byte LE `f64`s (10 per `SemioTransform`: 3 translation + 4
  quaternion + 3 scale), and u8-tagged enum discriminants for `SpatialKind`/`ElementClass`/
  `GeometryRef`/`PsetValue`/`RelationKind` (`store::pack_rt`/`store::ByteReader`, no external crate,
  no hand-rolled varint). Replaces the old `serde_json::to_vec`-in-envelope shortcut.
- [x] **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`, real dialect syntax
  (`{ }` grouping, bare `hex` macro, one production per line), matching
  `print_semio_model_snapshot_body` field-for-field.
- [x] **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `header fixed 1
  {format u8}` + real bare `segment schema_len varint` / `segment schema_bytes Array(u8,
  Field(schema_len))` (proven bare form, not the braced form — see the workflow report's §7 fix),
  then one honest opaque `chain payload bytes` tail for `spatial`/`elements`/`relations`
  (`protocol-array-of-records` gap — homogeneous-but-variable-length repeated records with further
  nested variable-length fields, `psets`/`properties`/tagged-enum payloads). The real Rust
  encode/decode stays fully structured past that point.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — descriptive, same production names, not test-parsed.
- [x] **Fixtures** — `📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio` /
  `🎒️example.pack.semio` are genuine `print_dsl`/`encode_pack` output of
  `snapshot::demo_semio_model_snapshot()`, generated via a temporary `#[test] fn
  ws_temp_print_real_fixtures()` in `🎹️composer/🦀️component.rs` that `eprintln!`'d both outputs
  once, bytes copied out precisely via a small Python script parsing the captured stdout (never
  hand-typed), then the temporary test **deleted** (confirmed gone — grep for
  `ws_temp_print_real_fixtures` in the composer file now returns nothing).

### Diff (`🔺️diff/`)

- [x] **Text codec** — already real pre-wave (hand-rolled hex/bracket, per this ticket's earlier
  phase); unchanged in shape, only its private value-codec functions (`enc_str`, `enc_transform`,
  `enc_spatial_node`, `enc_element`, `enc_relation`, all their `dec_*` twins, `encode_option`/
  `decode_option`, `enc_list`/`dec_list`, etc.) were widened to `pub(crate)` so `🧬️mutations` could
  reuse them instead of re-deriving a second copy (same reuse convention workflow's own mutations
  facet uses against its sibling diff facet).
- [x] **Binary upgrade** — was on the F6 `print_diff().into_bytes()` text-as-binary shortcut
  (confirmed by reading the pre-wave file). Now: `format u8` + `presence u8` (bit0=`spatial`,
  bit1=`elements`, bit2=`relations`) as two real fixed header fields, then 0-3
  varint-length-prefixed opaque blobs (the same `enc_spatial_diff`/`enc_elements_diff`/
  `enc_relations_diff` text `print_diff` already emits, now factored into named `pub(crate)`
  functions reused by both `print_diff` and `encode_diff`). Same `protocol-cond-cannot-chain`
  rationale as workflow's own diff facet (a second `if`-guard on a conditionally-decoded field
  hard-errors `eval_cond`) — extended here to a 3-bit presence mask instead of workflow's 2-bit one.
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — real dialect syntax, restates
  `spatial-node`/`element`/`relation` value grammars, the tri-state `option-x` pattern for every
  `Option<T>` diff field including the DOUBLY tri-state `parent_id`/`spatial_id`
  (`Option<Option<String>>`), and the collection-triple pattern (id-keyed `NamedTripleDiff`) for
  all three collections.
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format,
  presence}` + `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added for the conformance-law
  tests — reuses `sweep_a()`/`sweep_b()`, themselves promoted from `mod tests`-local to
  module-scope `#[cfg(test)] pub(crate) fn` (single source of truth for both the facet's own
  `field_sweep`/`between_roundtrip_law`/etc. tests and the composer's conformance laws).

### Mutations (`🧬️mutations/`)

- [x] **Text codec upgrade** — **this facet was on a plain `serde_json::to_string`/`from_str`
  passthrough pre-wave** (not even hex-encoded, unlike the sibling `diff` facet) — confirmed by
  reading the pre-wave file; this is a real, new-to-this-wave deviation from workflow's own
  pre-wave state (workflow's mutations text codec was ALREADY real before its pilot). Now:
  hand-rolled `keyword arg=value ...` grammar (`print_semio_model_mutation`/
  `parse_semio_model_mutation`), one keyword per the 11 real `SemioModelMutation` variants,
  reusing `🔺️diff`'s now-`pub(crate)` value codecs (`enc_spatial_node`, `enc_element`,
  `enc_relation`, `enc_spatial_kind`, `enc_element_class`, `enc_geometry_ref`, `enc_property_set`,
  `enc_relation_kind`, `enc_transform`, `encode_option`/`decode_option`, `enc_list`/`dec_list`) —
  never duplicated a second copy of these primitives.
- [x] **Binary upgrade** — was on a plain `serde_json::to_vec`/`from_slice` shortcut (same root
  cause as the text codec, not the F6 `print_op().into_bytes()` shortcut workflow's own report
  described — a strictly worse starting point). Now: `format u8` + `tag u8` (variant ordinal,
  `OP_KEYWORDS`/`variant_ordinal`, 0-10 matching `parse_semio_model_mutation`'s keyword match) as
  two real fixed fields, then the variant's own `key=value ...` argument text as one opaque
  trailing `bytes` chain — reuses the newly-real `print_semio_model_mutation`/
  `parse_semio_model_mutation` text codec (`print_semio_model_mutation_args` just strips the
  keyword) so there is exactly one source of truth for the argument encoding.
- [x] Grammar/protocol/mirrors, same treatment as the sibling facets — grammar traced verbatim from
  `print_semio_model_mutation`'s real `format!(...)` call sites.
- [x] Moved `sample_transform()`/`fixture()`/`demo_mutation_cases()` out of `#[cfg(test)] mod
  tests` to module-scope `#[cfg(test)]` fns (`base_snapshot` renamed `fixture`, matching
  workflow's own naming; the local `variants` vec inside `op_text_binary_roundtrip_law` renamed
  `demo_mutation_cases`, `pub(crate)`) so `🎹️composer/🦀️component.rs`'s conformance tests can reuse
  them.
- [x] `use protocol::{OpBinary, OpText};` moved from `#[cfg(test)]`-gated to unconditional (same
  fix workflow's own mutations facet needed — `decode_op`'s `Self::parse_op(...)` trait-method call
  needs `OpText` in scope in production code too, not merely under test).

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) written into
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests` block, in a new nested `mod
conformance_laws` — same home workflow's own pilot used, for the same reason (`model` has no
per-standard `⚙️engine/` dir; the shared `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` is a 14-subset
aggregator with no test module of its own, and is out of this wave's edit scope).

### JSON-transfer ban (checklist item 8)

Grepped the three changed `.rs` files (`📸️snapshot/🦀️component.rs`, `🔺️diff/🦀️component.rs`,
`🧬️mutations/🦀️component.rs`) for `serde_json` — **clean**: the only remaining hits are in doc
comments describing the OLD, now-replaced shortcuts (e.g. "replacing the old
`serde_json::to_vec`-in-envelope shortcut"), zero actual `serde_json::` calls remain inside the
`ArtifactPack`/`OpBinary`/`DiffCodec` impl blocks.

### `register_schema_spec` (checklist item, "if unsure, skip and note as follow-up")

**Skipped**, same as workflow: no derivable `RecordSpec` exists for `model`'s hand-rolled types
(see §1's mechanism-gap note above — the 5 tagged enums are the blocker). Filed as a follow-up
rather than fabricated.

---

## 3. Exact files touched

All paths relative to repo root, under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/`.

**Snapshot**: `🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/📸️snapshot/🦀️component.rs`,
`…/📸️snapshot/📝️text/📖️component.grammar.semio`, `…/📸️snapshot/📝️text/🅰️component.g4`,
`…/📸️snapshot/📝️text/🔤️component.ebnf`, `…/📸️snapshot/💾️binary/📡️component.protocol.semio`,
`…/📸️snapshot/💾️binary/🥋️component.ksy`, `…/📸️snapshot/💾️binary/🌶️component.spicy`,
`…/📸️snapshot/💾️binary/🔠️component.abnf`.

**Diff**: `…/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/📖️component.grammar.semio`,
`…/🔺️diff/📝️text/🅰️component.g4`, `…/🔺️diff/📝️text/🔤️component.ebnf`,
`…/🔺️diff/💾️binary/📡️component.protocol.semio`, `…/🔺️diff/💾️binary/🥋️component.ksy`,
`…/🔺️diff/💾️binary/🌶️component.spicy`, `…/🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `…/🧬️mutations/🦀️component.rs`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
`…/🧬️mutations/📝️text/🅰️component.g4`, `…/🧬️mutations/📝️text/🔤️component.ebnf`,
`…/🧬️mutations/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/💾️binary/🥋️component.ksy`,
`…/🧬️mutations/💾️binary/🌶️component.spicy`, `…/🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️model/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`; the fixture-generating temp test was added then removed in the same
session — confirmed absent in the final file).

**New example slug** (outside `✳️model/`, explicitly permitted by the brief, mirrors workflow's own
`📚️examples/🌊️pipeline`):
`📚️examples/🏢️building/🦀️component.rs`, `…/🏢️building/🟦️component.ts`,
`…/🏢️building/🖼️assets/🗣️example.dsl.semio` (genuine `print_dsl` output, not placeholder),
`…/🏢️building/🖼️assets/🎒️example.pack.semio` (genuine `encode_pack` bytes, not placeholder).

Nothing outside these was touched. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, `⚙️engine/🧮️geometry` (already-fixed shared module), and every other
subset/artifact were left untouched, per the brief — confirmed via `git status --porcelain` scoped
to those paths (empty, except the pre-existing `⚙️engine/🧮️geometry/🦀️component.rs` modification
that was already present in the repo BEFORE this session started, done by an earlier session per
the brief's own note, and genuinely not touched again here).

---

## 4. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `spatial`/`elements`/`relations` — homogeneous variable-length repeated records with further nested variable-length fields (`psets`, `properties`, tagged-enum payloads). Opaque trailing `chain payload bytes` after the real `format`+`schema` header, same as workflow's own `nodes`/`edges`. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `spatial`/`elements`/`relations` — THREE independently-optional segments (workflow's diff only had two). Used one opaque `chain payload bytes` with a real 3-bit `presence` bitmask header field instead of chained `Cond`s. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled types). |
| **`semio-tagged-enum-not-dslenum`** (NEW — not in recipe's table, not the same shape as workflow's `semio-shared-value-struct-not-dslfield` gap, which the framework already fixed) | no | `model`'s own 5 hand-rolled tagged enums (`SpatialKind`, `ElementClass`, `GeometryRef`, `PsetValue`, `RelationKind` — all `#[serde(tag = "kind")]`, one with a struct-variant `Other{name}` catch-all) don't implement `dsl::DslField`/`DslEnum`, unlike the now-fixed shared geometry value structs. This blocked the FULL derive path for `model`'s snapshot even though every geometry-valued field itself is now derive-ready. **Recommend**: a future wave could try `#[derive(dsl::DslEnum)]` on these 5 enums (in `✳️model/🧬️schema/📸️snapshot/🦀️component.rs`, in-scope for a `model`-only wave to edit) and re-attempt the derive path — deferred this wave in favor of shipping the fully-proven hand-rolled path within the session's time budget, matching the recipe's own "don't fight the derive" standing instruction for diff/mutations. |

---

## 5. Verified green — real command output, observed in this session

All three commands below were run as **foreground** commands in this session and their real output
was read before writing this section — none of these numbers are assumed or carried over from a
prior session.

1. `cargo check -p semio-s-plugin-stdio` → **0 errors**, clean (486→484 warnings after two small
   `impl protocol::OpText`/`OpBinary` → `impl OpText`/`OpBinary` qualification cleanups; no new
   warnings attributable to this wave's files beyond 2 pre-existing-style ones — an
   `unused import: DiffCodec` in `🔺️diff/🦀️component.rs` and a hidden-lifetime-parameter note in
   `🎹️composer/🦀️component.rs`, both pre-existing patterns copied from the original scaffold, not
   introduced by this wave's logic changes). "Finished `dev` profile ... in 16.51s" on the
   post-fixup run.

   **Note on a transient, non-`model` compile break observed mid-session**: one `cargo check` run
   during this session genuinely failed with 4 errors, all in `🪆️subsets/✳️brep/🧬️schema/
   🧬️mutations/🦀️component.rs` (`enc_face`/`dec_face` not accessible — a private `fn` in `brep`'s
   own `📸️snapshot/🦀️component.rs` that `brep`'s `🧬️mutations/🦀️component.rs` tried to import
   before `brep`'s own concurrent session had made it `pub(crate)`). Confirmed via `git status
   --porcelain` scoped to `✳️brep/` that this subset had many `M` (uncommitted, in-progress) files
   at the time — i.e. another concurrent session's own real-codec upgrade for `brep`, mid-edit, not
   caused by or fixable within this `✳️model`-only wave. A re-run minutes later showed 0 errors
   crate-wide — it self-resolved without any action from this session, exactly the "concurrent
   cargo workspace churn" pattern this repo's own environment note warns about.

2. `cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::model"` →
   **28 passed, 0 failed, 0 ignored**, "finished in 0.15s" (final confirming run). Includes all 6
   conformance-law tests (`committed_facet_files_parse`, `grammar_conformance_law`,
   `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
   `fixture_honesty_law`) plus every pre-existing `model` test (referential-invariant validator,
   ifc/bcf import/export round trips, diff/mutation algebra laws) — all green.

3. `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1869 passed, 0 failed, 3 ignored**,
   "finished in 12.11s" (final confirming run) — **zero regressions anywhere in the crate**.

   **Note on a second, non-`model` transient failure observed mid-session**: the FIRST whole-crate
   run after this wave's own work was complete showed **1 failure**:
   `artifacts::semio::standards::v1::subsets::object::composer::tests::conformance_laws::
   fixture_honesty_law`, panicking on a shipped fixture still containing the literal string
   `"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST"`. Confirmed via `git status --porcelain` scoped to
   `✳️object/` that this is a DIFFERENT subset, also mid-edit (many uncommitted `M` files) by
   another concurrent session doing its own real-codec wave, not `model` and not touched by this
   session. A re-run showed 0 failures — that session's own fixture regeneration landed in the
   interim. **This wave's own `model` fixtures were never placeholders at any point they were
   shipped** — `📚️examples/🏢️building/🖼️assets/*.semio` went straight from "did not exist" to
   "genuine `print_dsl`/`encode_pack` bytes" in one edit, via the temp-test-then-delete method,
   never an intermediate placeholder commit.

**Status: this wave is a genuinely proven, fully green replica of workflow's pilot pattern,
extended onto a subset with a materially different (larger, more-enum-heavy, mutations-facet-
starting-from-raw-JSON) starting shape.**

---

## 6. Notes for the next semio-subset wave

1. **Check the mutations facet's PRE-WAVE state independently of the diff facet's.** This wave's
   biggest surprise vs. workflow's own report: `model`'s `diff` facet was already real (hex/bracket
   text, F6-shortcut binary) but its `mutations` facet was on a **plain `serde_json` passthrough for
   BOTH text and binary** — a strictly less-real starting point than the recipe's own checklist
   assumes ("mutations grammar = the real op-text form ALREADY emitted by `print_op`/`parse_op`
   since F6 — trace it from the real function, never guess"). If the next subset's `print_op`/
   `parse_op` turn out to be `serde_json::to_string`/`from_str` too, don't assume you can "trace the
   real function" — you'll need to invent the real keyword-grammar from scratch, same as this wave
   did, reusing the sibling `diff` facet's (now `pub(crate)`) value codecs rather than duplicating
   them a third time.
2. **Widen a facet's private value-codec `fn`s to `pub(crate)` proactively** the moment a sibling
   facet needs to reuse them — this wave had to retrofit `pub(crate)` onto ~25 functions in
   `🔺️diff/🦀️component.rs` that were originally private, once `🧬️mutations` needed
   `enc_spatial_node`/`enc_element`/`enc_relation`/etc. Do this in one pass, not incrementally.
3. **Tagged hand-rolled enums (not just geometry value structs) can independently block the derive
   path.** Before assuming the derive path is now unblocked just because the shared geometry types
   derive `DslRecord`, check whether the snapshot type ALSO has its own hand-rolled tagged enums
   (`#[serde(tag = "kind")]`-style) — those need their own `#[derive(dsl::DslEnum)]` (untried by any
   wave so far) before the derive path is genuinely viable end-to-end.
4. **Expect transient, unrelated compile/test failures from concurrent sessions working other
   subsets in this same ticket** — this wave hit two (brep compile break, object fixture-honesty
   failure), both self-resolved within minutes without any action needed. Re-run once before
   concluding a failure is real and attributable to your own subset; always confirm via `git status
   --porcelain` scoped to the OTHER subset's directory before spending time investigating.
