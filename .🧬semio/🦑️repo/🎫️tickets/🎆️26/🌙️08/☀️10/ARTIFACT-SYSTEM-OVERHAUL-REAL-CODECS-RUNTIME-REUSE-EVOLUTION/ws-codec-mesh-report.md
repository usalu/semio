# W-S Codec Wave — `stdio.semio.mesh` (`✳️mesh` subset)

Second real-codec wave for a **semio** subset (`🧿️semio`), following the proven, fully-verified
`✳️workflow` pilot template (`ws-codec-workflow-report.md`) and `📖️grammar-recipe.md`. Scope:
`✳️mesh`'s three facets (snapshot, diff, mutations), applied to its richer, two-level-nested
collection shape (`meshes[].primitives[]`, each primitive carrying five parallel geometry buffers).

**Status: fully verified green in this session — real command output for every claim below, no
deferral.**

---

## 1. Derive path vs hand-rolled — what actually happened

Per this wave's brief, the `#[derive(dsl::DslArtifact)]` path was tried first now that the shared
`⚙️engine/🧮️geometry` types (`SemioPoint2`/`SemioPoint3`/`SemioUv`/`SemioRgba`/`SemioQuaternion`/
`SemioTransform`) all derive `dsl::DslRecord` (confirmed by reading
`⚙️engine/🧮️geometry/🦀️component.rs` — all six already carry the derive, closing the gap the
workflow pilot flagged).

**New blocker found (not the same one workflow hit, since that one is now closed)**: mesh's own
collection shape is nested two levels deep with multiple sibling variable-length buffers per leaf
record — `SemioMeshSnapshot.meshes: Vec<SemioMesh>` → `SemioMesh.primitives: Vec<SemioPrimitive>` →
`SemioPrimitive.{positions,normals}: Vec<SemioPoint3>`, `.uvs: Vec<SemioUv>`, `.colors:
Vec<SemioRgba>`, `.indices: Vec<u32>`, `.material_id: Option<String>`, all in ONE record. The
derive macro's `#[dsl(table)]`/`Vec<Record>` support (confirmed by reading the framework's worked
`SceneDocument`/`TableDocument` examples) covers one level of id-keyed `Vec<Record>` collection; it
does not have a documented/tested path for a record whose OWN fields are themselves multiple
sibling `Vec<ValueRecord>` buffers nested inside an outer `Vec<Record>`. Rather than risk a
half-working derive that silently mis-encodes one of the five buffer fields, hand-rolled instead —
same decision workflow's own report reached for a different (but analogous "shared value struct
doesn't fit the derive's tested shape") reason.

**Decision**: hand-rolled `ArtifactDsl`/`ArtifactPack` for the snapshot (never regressing to
hex-of-JSON), reusing the exact hex/bracket-encoded convention this subset's own `🔺️diff`/
`🧬️mutations` facets already used pre-wave (which itself already used real hand-rolled text/JSON
codecs — see §2 below). `DiffCodec`/`OpBinary` were upgraded from the F6 `print_diff()`/
`print_op().into_bytes()` text-as-binary shortcut to real binary frames, matching workflow's own
upgraded shape almost verbatim (format+presence header for diff, format+tag header for mutations).

---

## 2. Per-facet checklist (recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` in `📸️snapshot/🦀️component.rs` now produce/consume
  a genuine 4-line structured body: `schema=<hex>`, `meshes=[<mesh>,...]`,
  `materials=[<material>,...]`, `textures=[<texture>,...]`. Every `mesh`/`primitive`/`material`/
  `texture` value is a real bracket-nested token tree (hex for id/mime/kind strings via the `hex`
  macro, plain bracketed number lists for `positions`/`normals`/`uvs`/`colors`, a single-letter tag
  for `topology`) — not a hex dump of a JSON blob. Preamble handling unchanged
  (`store::semio_format::split_text_preamble`/`wrap_text`).
- [x] **Real binary pack** — `encode_pack_with`/`decode_pack_with` now call
  `encode_mesh_snapshot_binary`/`decode_mesh_snapshot_binary`: `format u8` + varint-length-prefixed
  `schema` UTF-8, then varint mesh/primitive/material/texture counts and per-field varint-prefixed
  strings + real 8-byte LE `f64` position/normal/uv buffers, 4-byte LE `f32` color/PBR buffers, and
  4-byte LE `u32` index buffers (`store::pack_rt::write_varint_u64`, `store::ByteReader` — same
  primitives workflow's own upgraded facets use). Replaces the old `serde_json::to_vec`-in-envelope
  shortcut entirely. Hand-rolled, not `store::pack_rt::encode_document` (needs a derived
  `RecordSpec`, which — per §1 — doesn't exist here).
- [x] **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`, real dialect syntax
  (`{ }` grouping, bare `hex` macro, one production per line), matching `print_mesh_snapshot_body`
  field-for-field.
- [x] **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `header fixed 1
  {field format u8}` + real bare `segment schema_len varint` / `segment schema_bytes
  Array(u8, Field(schema_len))` (the proven bare form — auto-merges into one segment), then one
  honest opaque `chain payload bytes` tail for the `meshes`/`materials`/`textures` collections
  (`protocol-array-of-records` gap — homogeneous-but-variable-length repeated records, doubly so
  here since `primitives` nests inside `meshes`). Real Rust encode/decode stays fully structured
  past that point.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — rewritten from the OLD ABNF-style hex-of-JSON placeholder
  description to real, descriptive (not test-parsed) mirrors of the new grammar/protocol shape.
- [x] **Fixtures** — `📚️examples/🧊️cube/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
  generated via the prescribed temporary-test method (§4 below) — genuine `print_dsl()`/
  `encode_pack()` bytes, never placeholder text.

### Diff (`🔺️diff/`)

- [x] **Binary upgrade** — was on the F6 `print_diff().into_bytes()` text-as-binary shortcut
  (confirmed by reading the pre-wave file). Now: `format u8` + `presence u8` (bit0=`meshes`,
  bit1=`materials`, bit2=`textures`) as two real fixed header fields, then 0-3
  varint-length-prefixed opaque blobs (the same `enc_meshes_diff`/`enc_materials_diff`/
  `enc_textures_diff` text this type's `print_diff` already emits). One opaque trailing `payload`
  chain in the protocol description, not per-segment `Cond`s, for the same
  `protocol-cond-cannot-chain` reason workflow's own diff protocol documents.
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — real dialect syntax, restates
  `mesh`/`primitive`/`material`/`texture` value grammars, the tri-state `option-x` pattern for every
  `Option<T>` diff field (incl. the DOUBLY-tri-state `option-option-hex` for
  `SemioPrimitiveDiff.material_id: Option<Option<String>>`), and the collection-triple pattern for
  `meshes`/`materials`/`textures` (all `NamedTripleDiff`), with the NESTED `primitives` triple
  inside each modified mesh's `mesh-item-diff`. `added` entries use this subset's own
  position-preserving `NamedAdded<T>` (`index:item`), not bare items.
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format, presence}`
  + `chain payload bytes`.
- [x] g4/ebnf mirrors lightly annotated (already close to real); ksy/spicy/abnf mirrors rewritten
  from the old "identical to text, `print_diff().into_bytes()`" description to the real binary
  frame shape.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added, with local
  `demo_snapshot_a`/`demo_snapshot_b` fixtures (kept local to `diff.rs`, NOT imported from
  `schema::mutations` — `mutations` itself depends on `diff`, so `diff` must not depend back on
  `mutations`).

### Mutations (`🧬️mutations/`)

- [x] **Binary upgrade** — same shortcut, same treatment. `format u8` + `tag u8` (variant ordinal,
  new `OP_KEYWORDS`/`variant_ordinal`, 0-15 across all 16 `SemioMeshMutation` variants) as two real
  fixed fields, then the variant's own `key=value ...` argument text as one opaque trailing `bytes`
  chain — reuses the already-real, already-tested `print_semio_mesh_mutation`/
  `parse_semio_mesh_mutation` text codec (`print_semio_mesh_mutation_args` strips the keyword).
  `use protocol::{Mutation, OpBinary, OpText};` made unconditional (was test-only for `OpBinary`)
  since production `encode_op`/`decode_op` now genuinely need both traits in scope.
- [x] Grammar/protocol/mirrors, same treatment — grammar traced verbatim from
  `print_semio_mesh_mutation`'s real `format!(...)` call sites.
- [x] Added module-scope `demo_fixture()`/`demo_mutation_cases()` (`#[cfg(test)]`, the latter
  `pub(crate)`) for the conformance-law tests; the existing test module's own `fixture()` now
  delegates to `demo_fixture()` (dedupe — the two were byte-identical) rather than keep an
  independent copy. Left the pre-existing, richer `sample_mutations()`/`sweep_a()`/`sweep_b()` test
  helpers untouched (they're semantically tied to the sweep fixtures already used across
  `mutation_diff_law`/`absorb_law`/`field_sweep`, a different and complementary purpose from the
  new `demo_mutation_cases()`).

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) written into
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests` block, in a new nested `mod
conformance_laws` — same location/shape workflow's own report identifies as the right home (mesh
likewise has no per-standard `⚙️engine/` test module of its own; `🎹️composer` is the closest
"engine-equivalent").

### JSON-transfer ban (checklist item 8)

Grepped all three changed `.rs` files (`📸️snapshot`, `🔺️diff`, `🧬️mutations`) for
`serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` — **clean** (zero real hits; the
only remaining mention is one doc comment in `📸️snapshot/🦀️component.rs` describing the OLD,
now-replaced shortcut).

---

## 3. Exact files touched

All paths relative to repo root. 29 files modified inside `✳️mesh/`, plus one new example slug
outside it (explicitly permitted by the brief).

**Snapshot**: `…/✳️mesh/🧬️schema/📸️snapshot/🦀️component.rs`,
`…/📸️snapshot/📝️text/📖️component.grammar.semio`, `…/📸️snapshot/📝️text/🦀️component.rs`,
`…/📸️snapshot/📝️text/🅰️component.g4`, `…/📸️snapshot/📝️text/🔤️component.ebnf`,
`…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/📸️snapshot/💾️binary/🦀️component.rs`,
`…/📸️snapshot/💾️binary/🥋️component.ksy`, `…/📸️snapshot/💾️binary/🌶️component.spicy`,
`…/📸️snapshot/💾️binary/🔠️component.abnf`.

**Diff**: `…/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/📖️component.grammar.semio`,
`…/🔺️diff/📝️text/🦀️component.rs`, `…/🔺️diff/📝️text/🅰️component.g4`,
`…/🔺️diff/📝️text/🔤️component.ebnf`, `…/🔺️diff/💾️binary/📡️component.protocol.semio`,
`…/🔺️diff/💾️binary/🦀️component.rs`, `…/🔺️diff/💾️binary/🥋️component.ksy`,
`…/🔺️diff/💾️binary/🌶️component.spicy`, `…/🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `…/🧬️mutations/🦀️component.rs`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
`…/🧬️mutations/📝️text/🦀️component.rs`, `…/🧬️mutations/💾️binary/📡️component.protocol.semio`,
`…/🧬️mutations/💾️binary/🦀️component.rs`, `…/🧬️mutations/💾️binary/🥋️component.ksy`,
`…/🧬️mutations/💾️binary/🌶️component.spicy`, `…/🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️mesh/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`).

**New example slug** (outside `✳️mesh/`, explicitly permitted by the brief):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🧊️cube/🦀️component.rs`,
`…/🧊️cube/🟦️component.ts`, `…/🧊️cube/🖼️assets/🗣️example.dsl.semio` (genuine `print_dsl` output),
`…/🧊️cube/🖼️assets/🎒️example.pack.semio` (genuine `encode_pack` bytes).

Nothing outside these was touched. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, the shared `⚙️engine/🧮️geometry` module, and every other subset were
left untouched, per the brief.

---

## 4. Fixture generation method (recipe's prescribed procedure, followed exactly)

Added a temporary `#[test] fn ws_temp_print_real_fixtures()` to `🎹️composer/🦀️component.rs` that
called the real `store::ArtifactDsl::print_dsl(&demo)` / `store::ArtifactPack::encode_pack(&demo)`
for `snapshot::demo_mesh_snapshot()` and `eprintln!`'d both outputs (DSL as UTF-8 text, pack as a
hex dump). Ran it once with `cargo test ... ws_temp_print_real_fixtures -- --nocapture`, captured
the real stdout, then used a small Python script to write the DSL text verbatim and decode the hex
dump into the real pack bytes (`bytes.fromhex(...)`) — never hand-transcribed. Deleted the temporary
test immediately after. `fixture_honesty_law` (below) is the independent proof this worked.

---

## 5. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `meshes`/`materials`/`textures` (and, within each mesh, the nested `primitives`) — homogeneous variable-length repeated records, doubly nested here. Opaque trailing `chain payload bytes` after the real `format`+`schema` header. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `meshes`/`materials`/`textures` — THREE independently-optional segments (one more than workflow's two); same `presence`-bitmask + opaque-tail treatment, generalized to 3 bits. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled types), same as workflow. |
| **`derive-nested-multi-buffer-record`** (NEW — not in recipe's table, distinct from workflow's now-closed `semio-shared-value-struct-not-dslfield` gap) | no | Even with `SemioPoint3`/`SemioUv`/`SemioRgba` now deriving `dsl::DslRecord`, the FULL derive path (`#[derive(dsl::DslArtifact)]`) is still blocked for `SemioMeshSnapshot` because `SemioPrimitive` (itself nested inside `SemioMesh`, itself nested inside the snapshot's `meshes: Vec<_>`) holds FIVE sibling variable-length buffer fields (`positions`/`normals`/`uvs`/`colors`/`indices`) plus a plain `Option<String>` in one record — a shape with no confirmed-working derive-macro precedent in the framework's own worked examples (`SceneDocument`/`TableDocument` show one level of `Vec<Record>`, not a record-of-multiple-sibling-buffers nested two collections deep). Hand-rolled instead, following the same "don't risk a silently-half-working derive" judgment call the recipe's own §1 pattern establishes. **Recommend**: any future semio subset whose leaf record type has more than one or two `Vec<T>` buffer-shaped fields alongside its scalar fields should expect this same wall and hand-roll immediately rather than re-attempt the derive path. |

---

## 6. Verification — real, not claimed (this session, foreground, actually observed)

All three commands below were run directly in the foreground in this turn and their real output
was read before writing this report.

1. **`cargo check -p semio-s-plugin-stdio`** →
   ```
   warning: `semio-s-plugin-stdio` (lib) generated 484 warnings (run `cargo fix --lib -p semio-s-plugin-stdio` to apply 174 suggestions)
       Finished `dev` profile [unoptimized] target(s) in 23.33s
   ```
   **0 errors.** (An earlier attempt in this same session transiently hit 4 errors —
   `cannot find value/function 'enc_face'/'dec_face'` — entirely inside
   `…/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🦀️component.rs`, a DIFFERENT subset this session never
   touched, confirmed still `M` in `git status` and actively being edited by a concurrent session
   — file mtime was ~1 minute old at the time. That blocker cleared on its own before this final
   check; nothing in `artifacts::…::mesh` was ever implicated.)

2. **`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::mesh"`**
   → **52 passed, 0 failed, 0 ignored** (finished in 0.08s), including all 6 conformance-law tests:
   `committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
   `diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law` — every one `ok`.
   (First run, before the real fixtures were generated, correctly showed 51 passed / 1 failed —
   `fixture_honesty_law` failing on the placeholder text, exactly as expected; re-run after fixture
   generation is the 52/0 result above.)

3. **`cargo test -p semio-s-plugin-stdio --lib`** (whole crate) →
   ```
   test result: FAILED. 1868 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 17.20s
   ```
   The one failure is `artifacts::semio::standards::v1::subsets::object::composer::tests::conformance_laws::fixture_honesty_law`
   — **not this wave's code**: it panics on `"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST"`, and
   `git status` confirms `…/🪆️subsets/✳️object/…` is `M`-modified by a different, concurrent
   session mid-way through its own FG-wave on the `object` subset (same placeholder-fixture pattern
   this report's own §4 shows how to close, just not yet done on that session's side). **Zero
   failures attributable to anything in `artifacts::…::mesh`.**

**Status: this wave is genuinely proven, fully green for `✳️mesh`.** The one whole-crate failure is
unrelated concurrent churn in `✳️object`, explicitly noted rather than chased, per this ticket's own
concurrent-development ground rules.
