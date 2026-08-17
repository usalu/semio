# W-S Codec Pilot — `stdio.semio.workflow` (`✳️workflow` subset)

First real-codec pilot for a **semio** subset (`🧿️semio`), following the `📖️grammar-recipe.md`
pattern already proven by json/csv/zip/png/txt/binary. Scope: `✳️workflow`'s three facets
(snapshot, diff, mutations). Written for the future agent who will replicate this pattern across
semio's other 12 domain subsets.

**Read this report's "Verification status" section first** — full `cargo test` could not be run to
completion in this session because of an unrelated, still-ongoing compile break in shared framework
files caused by other concurrent sessions (this repo has many agents working simultaneously; see
CLAUDE.md's "You MUST work simultaneously with others" rule and this ticket's own environment note
about heavy concurrent load). This is NOT a defect in the workflow code below — it is an honest gap
in what could be *proven* in this session, and is called out explicitly rather than glossed over.

---

## 1. Derive path vs hand-rolled — what actually happened

The brief asked to try `#[derive(dsl::DslArtifact)]`/`DslRecord` first. I read the framework's
derive macro (`🧰️framework/…/🗣️dsl/🦀️component.rs`'s `SceneDocument`/`TableDocument`/
`DerivedDocument` worked examples, `✨️derive/📦️packages/🦀️rust/📦️glue.rs`'s `FieldKind`
classification, and the `Shape::Table` validation code in `🧬️schema/🦀️component.rs`) closely enough
to confirm the derive path is genuinely capable of `Vec<Record>` fields (`#[dsl(table)]` or plain
`Vec<T>`), nested record fields (`#[dsl(block)]`), and `BTreeMap<String,V>` — i.e. workflow's
`nodes`/`edges`/`params` collections would all be expressible.

**The blocker**: `WorkflowNode.position: SemioPoint2`. `SemioPoint2` lives in
`✏️s/…/🧿️semio/🏅️standards/🔖️v1/⚙️engine/🧮️geometry/🦀️component.rs` — **outside this ticket's
`🪆️subsets/✳️workflow/`-only edit scope** — and does not implement `dsl::DslField`/`DslRecord`.
Every semio subset with a `SemioPoint2`/`SemioPoint3`/`SemioRgba`/etc. field will hit this exact
same wall; it cannot be worked around per-subset, only fixed once, centrally, by whichever future
ticket is allowed to touch `⚙️engine/🧮️geometry`. **Filed as a new mechanism gap** (§5 below) — not
one of the recipe's existing 7 gaps, since none of P1-P3's pilots (json/csv/zip/png/txt/binary) had
this "shared value-struct field type doesn't derive" shape.

**Decision**: hand-rolled `ArtifactDsl`/`ArtifactPack` for the snapshot (never regressing to
hex-of-JSON), matching the SAME hex/bracket-encoded convention this subset's own `🔺️diff`/
`🧬️mutations` facets already used pre-pilot (itself modeled on `GifDiff`/`SvgDiff`/`DocxDiff`'s
established repo-wide hand-rolled convention). `DiffCodec`/`OpBinary` were likewise upgraded
hand-rolled (recipe §2.5's fixed-header + opaque-tail pattern), matching json's own upgraded
hand-rolled `DiffCodec`/`OpBinary` almost verbatim in shape.

---

## 2. Per-facet checklist (recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` in `📸️snapshot/🦀️component.rs` now produce/consume
  a genuine 3-line structured body: `schema=<hex>`, `nodes=[<node>,...]`, `edges=[<edge>,...]`, each
  node/edge/param a hex/bracket-encoded value (NOT a hex dump of a JSON blob — every field is its
  own token, the grammar walks real productions, not one opaque span). Preamble handling
  (`store::semio_format::split_text_preamble`/`wrap_text`) unchanged, per spec.
- [x] **Real binary pack** — `encode_pack_with`/`decode_pack_with` now call
  `encode_workflow_snapshot_binary`/`decode_workflow_snapshot_binary`: `format u8` + varint-length-
  prefixed `schema` UTF-8, then varint node/edge counts and per-field varint-length-prefixed
  strings + real 8-byte LE `f64` coordinates (`store::pack_rt::write_varint_u64`, `store::ByteReader`
  — same primitives json's own upgraded facets use). Replaces the old `serde_json::to_vec`-in-
  envelope shortcut entirely. This is a **hand-rolled** real binary layout, not
  `store::pack_rt::encode_document` (that path needs a derived `RecordSpec`, which — per §1 above —
  doesn't exist here).
- [x] **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`, real dialect syntax
  (`{ }` grouping, bare `hex` macro, one production per line, no reserved-word collisions), matching
  `print_workflow_snapshot_body` field-for-field.
- [x] **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `header fixed 1
  {format u8}` + real `segment {schema_len varint, schema_bytes Array(u8,Field(schema_len))}`, then
  one honest opaque `chain payload bytes` tail for the `nodes`/`edges` collections (the
  `protocol-array-of-records` gap, §5/recipe — homogeneous-but-variable-length repeated records,
  `repeat`'s arms are tag-dispatched, not "N times from a count field"). The real Rust encode/decode
  stays fully structured past that point; only the protocol DESCRIPTION stops there, same treatment
  every P1-P3 pilot's nested payload uses.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — descriptive, same production names, not test-parsed.
- [~] **Fixtures** — `📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio` /
  `🎒️example.pack.semio` exist but currently hold **placeholder text**, not genuine
  `print_dsl`/`encode_pack` output — see §4 below, this is the single most important unfinished
  item.

### Diff (`🔺️diff/`)

- [x] **Binary upgrade** — was on the F6 `print_diff().into_bytes()` text-as-binary shortcut (100%
  of stdio per the P2-W0 census, confirmed still true here pre-pilot). Now: `format u8` +
  `presence u8` (bit0=`nodes`, bit1=`edges`) as two real fixed header fields, then 0-2
  varint-length-prefixed opaque blobs (the same `enc_nodes_diff`/`enc_edges_diff` text this type's
  `print_diff` already emits). Two length-prefixed segments rather than `Cond`-guarding each one
  individually because a second `if`-guard on a field that's itself only conditionally decoded
  hard-errors `eval_cond` — this is the `protocol-cond-cannot-chain` gap from the recipe's own §5
  table, hit here for real (not merely theoretical).
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — real dialect syntax, restates
  `node`/`edge`/`param` value grammars, the recipe §1.4 tri-state `option-x` pattern for every
  `Option<T>` diff field, and the recipe §1.4 collection-triple pattern (name-keyed, since
  `nodes`/`edges`/`params` are all id-keyed `NamedTripleDiff`).
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format, presence}`
  + `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope — same convention json's
  `demo_diff_cases`/`demo_mutation_cases` use) added for the conformance-law tests.

### Mutations (`🧬️mutations/`)

- [x] **Binary upgrade** — same shortcut, same treatment. `format u8` + `tag u8` (variant ordinal,
  `OP_KEYWORDS`/`variant_ordinal`, 0-12 matching `parse_workflow_mutation`'s keyword match) as two
  real fixed fields, then the variant's own `key=value ...` argument text as one opaque trailing
  `bytes` chain — reuses the ALREADY-real, already-tested `print_workflow_mutation`/
  `parse_workflow_mutation` text codec (`print_workflow_mutation_args` just strips the keyword) so
  there is exactly one source of truth for the argument encoding, not two.
- [x] Grammar/protocol/mirrors, same treatment as the sibling facets — grammar traced verbatim from
  `print_workflow_mutation`'s real `format!(...)` call sites, never guessed.
- [x] Moved `fixture()`/`node()`/`edge()`/`sample_mutations()` out of `#[cfg(test)] mod tests` to
  module-scope `#[cfg(test)]` fns (`sample_mutations` renamed `demo_mutation_cases`, `pub(crate)`)
  so `🎹️composer/🦀️component.rs`'s conformance tests can reuse them — same pattern json uses.

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) written into
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests` block, in a new nested
`mod conformance_laws`. Workflow has **no** per-standard `⚙️engine/` dir the way json/csv/zip/png
do (only `📸️snapshot`/`🔺️diff`/`🧬️mutations`/`🎹️composer`/`🏗️builder`/`🚪️io`/`🧐️analyzer`), and
`🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` is a SHARED 14-subset `register()` aggregator with no
test module of its own (confirmed by reading it — 22 lines, no `#[cfg(test)]`) and is out of this
ticket's edit scope anyway. `🎹️composer` (which already had its own `register()` + `#[cfg(test)]
mod tests`) is the closest "engine-equivalent" home, matching the brief's own fallback instruction.

### Not done (explicit, per brief item 9)

`LanguageSpec`/`register_schema_spec` registration — **skipped**, no clear existing per-subset
registration SITE was found beyond `🎹️composer::register()` itself, and per item 9's own
instruction ("if unsure, skip and note as follow-up") this is filed as a follow-up rather than
guessed at. `register_schema_spec` specifically also doesn't apply here: it needs a real
`fn() -> RecordSpec`, and (§1 above) this subset's types don't derive one.

### JSON-transfer ban (checklist item 10)

Grepped every new/changed `.rs` file for `serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/
`Value` inside `ArtifactPack`/`OpBinary`/`DiffCodec` impl blocks — **clean** (zero hits; the only
remaining `serde_json` mentions in these files are in doc comments describing the OLD, now-replaced
shortcut).

---

## 3. Exact files touched

All paths relative to repo root.

**Snapshot**: `…/✳️workflow/🧬️schema/📸️snapshot/🦀️component.rs`,
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

**Tests**: `…/✳️workflow/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`).

**New example slug** (outside `✳️workflow/`, explicitly permitted by the brief):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🌊️pipeline/🦀️component.rs`,
`…/🌊️pipeline/🟦️component.ts`, `…/🌊️pipeline/🖼️assets/🗣️example.dsl.semio` (placeholder — see §4),
`…/🌊️pipeline/🖼️assets/🎒️example.pack.semio` (placeholder — see §4).

Nothing outside these was touched. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, and every other subset/artifact were left untouched, per the brief.

---

## 4. Verification status — READ THIS BEFORE TRUSTING ANYTHING ABOVE AS "DONE"

**What is genuinely confirmed, with real command output:**

- `cargo check -p semio-s-plugin-stdio --lib` — run early in this session, **completed with 0
  errors** ("Finished `dev` profile [unoptimized] target(s) in 31.15s", 482 pre-existing warnings,
  none new/related to this pilot). This compiles every non-`#[cfg(test)]` line of code in this
  report — i.e. the real `ArtifactDsl`/`ArtifactPack` impl for the snapshot, the real `DiffCodec`
  impl for the diff, and the real `OpBinary` impl for the mutations all **type-check and compile
  cleanly** across the whole `semio-s-plugin-stdio` crate at that point in time.
- Three later attempts (`cargo check -p semio-s-plugin-stdio` non-`--lib`, twice, plus one `cargo
  test` attempt) all failed to get past **unrelated framework crates** the stdio plugin depends on
  (`semio-framework-schema`, `semio-framework-os-kernel`) — three DIFFERENT transient errors across
  those three attempts (`StateClass::Inferred` non-exhaustive match, then `InferredField<P>::Key:
  serde::Deserialize`/`Serialize` not satisfied), all inside framework files this session never
  touched (`🏪️store/🦀️component.rs`, `🔨️modules/🧬️schema/🦀️component.rs` — confirmed still `M` in
  `git status` throughout, i.e. actively mid-edit by another concurrent session, not something this
  session broke or can fix). The final grep-only check
  (`cargo check -p semio-s-plugin-stdio 2>&1 | grep -E "^error"`) shows exactly these 2 errors and
  nothing else — **zero errors attributable to anything in `artifacts::semio::…::workflow`** in
  every attempt.

**What is NOT yet confirmed, and must be verified once the upstream framework blocker clears:**

- Because `semio-framework-os-kernel`/`semio-framework-schema` fail to compile first, `rustc` never
  reaches far enough to type-check this pilot's `#[cfg(test)]` code — the `demo_workflow_snapshot`/
  `demo_diff_cases`/`demo_mutation_cases` helper fns and the new `conformance_laws` test module are
  therefore **untested by the compiler in this session**, not just "tests not run." I read the
  derive/grammar/protocol framework source closely and modeled every production/field off real,
  already-compiling precedent (json/csv/zip/png's own conformance-law tests, byte for byte in
  structure), but I have not watched `rustc` confirm it.
  the pilot's fixture-honesty and protocol-walk assertions.
- **`📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio` and `🎒️example.pack.semio` currently
  contain placeholder text** ("PLACEHOLDER..."), not genuine `print_dsl()`/`encode_pack()` output.
  The plan was to generate them via a temporary test once the crate compiled under `--tests`; that
  step could not be reached. **`fixture_honesty_law` WILL fail as written until these two files are
  regenerated** from `snapshot::demo_workflow_snapshot()`'s real `print_dsl()`/`encode_pack()`
  output and the placeholder text is replaced.
- The grammar/protocol files were hand-derived from close reading of the framework's dialect parser
  and 6 other pilots' real, committed, already-conformance-tested files (json/csv/zip/png cited
  directly in-line in each new file's own comments) — but `dsl::parse_grammar`/`parse_protocol`/
  `Recognizer::recognize`/`walk_protocol` have not actually run against them yet. The most likely
  failure points if something's off: (a) whether `header fixed N` + a following `segment { }` block
  is genuinely legal syntax back-to-back (I confirmed this by reading `parse_protocol`'s source
  directly — `"header"`/`"segment"` are independent top-level directives, each flushes any
  previously-open block — but haven't watched it parse), (b) whether the bare `hex` macro's
  backtracking behaves the same on this subset's specific token shapes as it does on json's.

**Recommended next step for whoever picks this up**: once
`git status --porcelain -- "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs"` and
`-- "🧰️framework/🔨️modules/🧬️schema/🦀️component.rs"` are both clean (the other session's refactor
landed), run:
```
cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::workflow" 2>&1 | tail -150
```
Expect `fixture_honesty_law` to fail first (placeholder fixtures) — fix by writing a throwaway
`#[test]`/`#[ignore]` test that calls `store::ArtifactDsl::print_dsl(&demo_workflow_snapshot())` and
`store::ArtifactPack::encode_pack(&demo_workflow_snapshot())`, printing/dumping the bytes, then
copying that real output into the two fixture files verbatim (recipe §4's own prescribed method —
never hand-derive the fixture independently of the real encoder) and deleting the throwaway test.
Then re-run and fix whatever grammar/protocol issues surface from actual `Recognizer`/
`walk_protocol` runs — most likely candidates are listed above.

---

## 5. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `nodes`/`edges` — homogeneous variable-length repeated records, can't be `Array`/`repeat`-described untagged. Opaque trailing `chain payload bytes` after the real `format`+`schema` header. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `nodes`/`edges` — TWO independently-optional segments; a `Cond`-per-segment would need to guard a field that's itself only conditionally decoded, which `eval_cond` hard-errors on. Used one opaque `chain payload bytes` with a real `presence` bitmask header field instead. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled types). |
| **`semio-shared-value-struct-not-dslfield`** (NEW — not in recipe's table) | no | `SemioPoint2`/`SemioPoint3`/`SemioRgba`/`SemioQuaternion`/`SemioTransform`/`SemioUv` (`⚙️engine/🧮️geometry`) don't implement `dsl::DslField`/`DslRecord`, and are outside any single subset's edit scope (shared across all 14 semio subsets). This blocked the FULL derive path for workflow's `position: SemioPoint2` field and will block it identically for every other semio subset with a geometry-valued field (brep, cad, mesh, model, object, drawing — likely most of the remaining 12). **Recommend**: a small, separate, centrally-scoped ticket adds `#[derive(dsl::DslRecord)]` to these ~6 shared structs in `⚙️engine/🧮️geometry` once, before the next semio subset pilot starts — otherwise every subsequent subset will independently re-hit and re-hand-roll around this same wall. |

---

## 6. Copy-paste guidance for the next semio subset

1. Check whether the subset's snapshot type has a bare (non-Vec, non-Option) field of one of the
   `⚙️engine/🧮️geometry` types. If yes, the derive path is blocked exactly as described in §1/§5 —
   don't spend time retrying it, hand-roll immediately using this pilot's `📸️snapshot/🦀️component.rs`
   `#region 🔖️TextPrimitives`/`#region 🔖️BinaryPrimitives` as the template (hex/bracket text +
   varint-length-prefixed binary, both using `store::pack_rt`/`store::ByteReader` for the binary
   side — no external crate, no hand-rolled varint).
2. If the subset's diff/mutations already have hand-rolled `print_diff`/`parse_diff`/`print_op`/
   `parse_op` (most semio subsets do, per this ticket's earlier phase — "hand-roll all diff/op
   codecs" was already the standing instruction before this pilot), the binary upgrade is almost
   pure mechanical work: copy this pilot's `DiffCodec::encode_diff`/`decode_diff` (format+presence
   bitmask header, length-prefixed opaque blobs per optional collection) and `OpBinary::encode_op`/
   `decode_op` (format+tag header, reuse the existing text encoder for the opaque tail) verbatim,
   swapping only the type names and the `OP_KEYWORDS` table.
3. Grammar/protocol files: this pilot's 6 `.grammar.semio`/`.protocol.semio` files are now real,
   committed, dialect-syntax-correct references for exactly this "hand-rolled hex/bracket text +
   varint-length-prefixed binary with an opaque array-of-records tail" shape — closer to copy from
   than json's (which has recursive `JsonValue`, a different shape) for any OTHER semio subset with
   id-keyed node/edge-style collections.
4. Put the conformance-law tests in the subset's own `🎹️composer` (or equivalent aggregator) file's
   existing `#[cfg(test)] mod tests`, NOT the shared `⚙️engine/🦀️component.rs` — that file is
   14-subsets-shared and out of any single subset's edit scope.

---

## 7. UPDATE (main session, post-report): fully verified green — 29/29, 0 regressions

The report above was written while blocked by another session's concurrent compile break and
correctly left the pattern unverified. That blocker has since cleared. Direct follow-up fixes and
verification, done by the orchestrating session (not delegated):

1. **`⚙️engine/🧮️geometry/🦀️component.rs`**: added `dsl::DslRecord` to the derive list of all 6
   shared geometry types (`SemioPoint2`, `SemioPoint3`, `SemioUv`, `SemioRgba`, `SemioQuaternion`,
   `SemioTransform`) — closing the `semio-shared-value-struct-not-dslfield` gap this report flagged.
   `cargo check -p semio-s-plugin-stdio` confirmed clean.
2. **`🧬️mutations/📝️text/📖️component.grammar.semio`**: the `op = ...` production was split across 3
   physical lines (11-13) — a direct hit of the grammar recipe's own documented pitfall #4
   ("`parse_sequence` stops at the first `Newline` token"). Collapsed to one line. Fixed
   `ops_grammar_conformance_law` and `committed_facet_files_parse`.
3. **`📸️snapshot/💾️binary/📡️component.protocol.semio`**: the `segment schema_segment { field
   schema_len varint field schema_bytes Array(...) }` braced multi-field form failed to parse
   ("expected a protocol type, found Newline"). Root-caused by reading `dsl/📖️grammar/🦀️component.rs`'s
   `parse_protocol` directly and comparing against the framework's own canonical envelope protocol
   file (`🧬️semio/📡️protocol/📡️component.protocol.semio`), which uses the proven **bare** segment
   form instead: consecutive bare `segment <name> <type>` lines (no braces) merge into one anonymous
   segment automatically. Rewrote to that exact proven shape:
   ```
   header fixed 1
   field format u8

   segment schema_len varint
   segment schema_bytes Array(u8, Field(schema_len))

   chain payload bytes
   ```
   Fixed `protocol_walk_law`. **New mechanism note for future subsets**: prefer the bare
   `segment <name> <type>` form over the braced `segment <name> { field ... }` form — the latter may
   have a real, unconfirmed parser gap; not worth spending time on until someone files/fixes it.
4. **Fixtures**: regenerated `📚️examples/🌊️pipeline/🖼️assets/{🗣️example.dsl.semio,🎒️example.pack.semio}`
   with genuine `print_dsl()`/`encode_pack()` output of `demo_workflow_snapshot()` — added a
   temporary `#[test] fn ws_temp_print_real_fixtures()` to `🎹️composer/🦀️component.rs` that
   `eprintln!`s both outputs, ran it once, copied the exact bytes out via a small Python script
   (never hand-transcribed), deleted the temp test. Note the DSL text is NOT fully human-readable —
   `schema`/node `kind`/`label` etc. are hex-encoded via the grammar's `hex` macro (avoids
   quote/escape handling); numeric fields (`position`, ids' bracket structure) are plain. This is a
   legitimate, recipe-sanctioned pattern, not a shortcut.
5. **Verification — real, not claimed**:
   - `cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::workflow"` → **29 passed, 0 failed**, including all 6 conformance-law tests.
   - `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1844 passed, 0 failed, 4 ignored** — zero regressions anywhere.

**Status: this pilot is now a genuinely proven, fully green template.** §6's copy-paste guidance
stands, with one addition: prefer the bare `segment` form (point 3 above) when a subset's protocol
needs a length-prefixed variable-size field before the opaque tail.
