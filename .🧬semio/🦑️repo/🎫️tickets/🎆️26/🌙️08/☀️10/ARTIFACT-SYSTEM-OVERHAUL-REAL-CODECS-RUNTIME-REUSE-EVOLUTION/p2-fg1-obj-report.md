# P2-FG1 — `stdio.obj` (Wavefront OBJ 3.0) — real grammar/protocol/fixtures/conformance report

## Scope

Artifact: `🧊️obj`, standard `3.0`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/`.
Followed `📖️grammar-recipe.md` literally. Every syntax fragment below was either copied verbatim
from a real committed pilot file (json/csv/zip/png/txt/binary) or traced directly from `obj`'s own
real Rust codec — nothing guessed.

## 1. Snapshot facet

**Grammar** (`🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`) — rewritten
from an unparseable ABNF placeholder to the real dialect. Models `decode_obj`/`parse_face_vertex`
(`⚙️engine/🦀️component.rs:81-218`/`57-74`) exactly: a keyword-statement grammar
(`v`/`vt`/`vn`/`f`/`o`/`g`/`usemtl`/`mtllib`/`s`), any order, matching the real DECODER (not just
the encoder's documented normal form). Key decisions:
- `number = FLOAT | INT` everywhere a coordinate appears — Rust's own whole-valued-`f64` `Display`
  prints `1.0` as `"1"`, so real emitted text is INT-shaped as often as FLOAT-shaped.
- `face-vertex = INT "/" INT "/" INT | INT "/" "/" INT | INT "/" INT | INT`, longest-alternative-
  first (this dialect's ordered-choice alternation commits to the first alternative whose full
  symbol sequence matches, confirmed by direct reading of `match_production_tracked`/
  `match_sequence_tracked` — each alternative restarts from the ORIGINAL `pos` on failure, i.e.
  genuine per-alternative backtracking, just no cross-symbol backtracking within one alternative).
- `object-stmt = "o" LINE`, `group-stmt = "g" LINE`, `usemtl-stmt = "usemtl" LINE`,
  `mtllib-stmt = "mtllib" LINE` — the ticket brief's own named gap (`o`/`g` names are real
  "rest-of-line" content). Confirmed directly against `📖️grammar/🦀️component.rs`'s own worked
  `LINE` doc comment, which explicitly names "obj's `o`/`g` names" as the intended use case.
- `comment none` declared — real `#` lines are genuine retained content
  (`unknown_statements`), not framework comment trivia; without this directive the shared lexer
  would silently strip them before the grammar ever saw them.
- `unknown-stmt = LINE` as the LAST alternative in `statement` — one shared fallback catching both
  `#`-comment lines and genuinely-unrecognized keyword lines, matching `decode_obj`'s own single
  `unknown_statements` sink for both cases (the ticket brief's "nothing silently dropped" rule).
- `face-stmt = "f" face-vertex face-vertex face-vertex face-vertex*` — the dialect has no "at
  least N" quantifier, so the real >=3-vertex n-gon rule is modeled as 3 required + a `*` tail.

**Protocol** (`.../📸️snapshot/💾️binary/📡️component.protocol.semio`) — `stdio.obj` is TEXT-NATIVE
(confirmed directly: `ArtifactPack::encode_pack_with`, `📸️snapshot/🦀️component.rs:217-227`, wraps
`engine::encode_obj(self).into_bytes()` straight in the SEMIO envelope, no structured
`pack_rt::encode_document`). Modeled exactly like json's own snapshot protocol: `framing record` +
`chain payload utf8`, payload-only (the SEMIO envelope itself is described once, framework-side,
and is never re-inlined per artifact — confirmed `use semio.envelope` still doesn't resolve at
walk time).

## 2. Mutations facet

`ObjMutation` is `#[derive(dsl::DslOps)]` (P6 derive path — `obj`'s whole snapshot tree is plain
structs/`Vec`/`Option<T>`, zero data-carrying enums), with handcrafted `OpText`/`OpBinary` wrapping
the generic `dsl::print`/`dsl::variants_binary::encode_op` machinery — the SAME already-real
situation `stdio.txt`/`stdio.zip` document.

**Grammar** — every token traced from a REAL `print_op()` call over a `demo_mutation_cases()`
fixture (added to `🧬️mutations/🦀️component.rs`, see §4), not guessed — e.g. real
`InsertVertex{index:1, vertex:ObjVertex{x:9.0,y:9.0,z:9.0,w:Some(1.0)}}.print_op()` is exactly
`"insert-vertex index=1 vertex { x=9 y=9 z=9 w=1 }"`. A genuine finding worth flagging: unlike
`stdio.zip`'s own `SetSnapshot`/`AddEntry`/`SetEntryExtra` (which needed the `REST` raw-span
fallback because `ZipEntry` itself has further nested `Vec<Record>` fields), **every one of
`obj`'s leaf record types (`ObjVertex`/`ObjTexCoord`/`ObjNormal`/`ObjFaceVertex`/`ObjGroup`/
`ObjObject`/`ObjUsemtlRange`/`ObjSmoothingRange`/`ObjUnknownStatement`) is genuinely flat — zero
further nested `Vec<Record>` fields** — so `SetSnapshot`'s whole `ObjSnapshot` payload is modeled
PRECISELY, field-by-field, with no `REST` fallback anywhere in this grammar. Confirmed end-to-end
by `ops_grammar_conformance_law` recognizing real `print_op(SetSnapshot{...})` output.

**Protocol** — `OpBinary::encode_op`/`decode_op` already forward to
`dsl::variants_binary::encode_op`/`decode_op` (confirmed by direct reading: calls
`os_pack::encode_record_body`, never `print_op` — NOT the F6 text-as-binary shortcut). Modeled
`format u8` + `ordinal varint` as two real fixed fields, the record-body tail as one opaque
`chain bytes` — the SAME `txt-opbinary-record-body-wire-is-framework-generic` mechanism gap
`stdio.txt`'s own pilot first named (framework-generic wire, not artifact-specific, out of this
wave's ownership boundary). `opbinary_binary_upgraded = false` — already real before this wave.

## 3. Diff facet

`ObjDiff` is hand-rolled (`protocol::DiffCodec`, NOT `#[derive(dsl::DslDiff)]`) — confirmed 3b
tri-state blocker per that file's own module doc comment: `ObjVertexDiff::w`/`ObjTexCoordDiff::w`/
`ObjDiff::mtllib` are all `Option<Option<T>>`, which the derive's `classify_field` cannot express.

**Grammar** — traced field-by-field from the real hand-rolled `print_obj_diff`/`enc_*` functions
(`🔺️diff/🦀️component.rs` lines 927-1346): index-keyed collection triples
(`vertices`/`texcoords`/`normals`/`faces`), name-keyed triples (`groups`/`objects`, reusing one
`group-value`/`group-diff-value` shape for both since `ObjGroup`/`ObjObject` are structurally
identical), the uniform `[0]`/`[1,<value>]` tri-state tag, and the `hex` framework macro for every
encoded string (names/materials/`mtllib`/retained raw source lines).

**Protocol** — `DiffCodec::encode_diff`/`decode_diff` is `self.print_diff().into_bytes()` — the
text bytes VERBATIM, by the file's OWN documented design ("same simplification `WriterDiff`/
gif89a/svg's hand-rolled `DiffCodec`s use ... without inventing a second, denser wire format").
This decision already existed in the codebase before this wave touched it (confirmed: the doc
comment and the hand-rolled `DiffCodec` impl were already present). Modeled the protocol file
honestly to match: `framing record` + `chain payload utf8`, same text-native shape as the
snapshot's own protocol file — not a fabricated denser binary frame that would misrepresent what
the real Rust code actually does. `diffcodec_binary_upgraded = false`.

## 4. Test-fixture plumbing (real, not framework files)

Promoted `base_snapshot()`/`sweep_a()`/`sweep_b()`/`demo_mutation_cases()` (mutations) and
`sweep_a()`/`sweep_b()`/`demo_diff_cases()` (diff) from private `mod tests`-local helpers to
`#[cfg(test)] pub(crate) fn` at module scope, matching `stdio.zip`'s own exact convention — the
single source of truth reused by both the artifact's own round-trip tests AND
`⚙️engine`'s cross-file `conformance_laws` module. Added one more `SetMtllib{mtllib: Some(...)}`
case to `demo_mutation_cases()` to also exercise the Some-branch of that field (was
`None`-only before).

## 5. Real fixtures

- `📚️examples/🎬️demo/🖼️assets/example.obj` — rewritten from a leftover `"Hello, stdio.txt!"`
  placeholder to a real 2-triangle-quad Wavefront OBJ sample exercising every statement kind
  (`mtllib`, `v`/`vt`/`vn`, `o`/`g`/`usemtl`/`s` transitions, `f` triangles, a `#` comment, and one
  genuinely-unrecognized directive line). **Note**: `*.obj` is repo-wide `.gitignore`d
  (`.gitignore:401`) — this file is NOT tracked by git (pre-existing repo rule, out of scope to
  change), but it is real content on disk, matching every sibling artifact's own
  `example.<ext>`/DSL-fixture pairing convention (stl/png/zip all keep this same raw-asset-plus-DSL-
  fixture pair, and their raw asset is likewise never referenced from Rust — it's a static demo
  asset only).
- `🗣️example.dsl.semio` — GENUINE `print_dsl(demo_obj_snapshot())` output (captured via a
  temporary `[DEBUG]`-prefixed test that called the real encoder directly, copied verbatim, then
  deleted before finishing — never hand-derived).
- `🎒️example.pack.semio` — GENUINE `encode_pack(demo_obj_snapshot())` bytes, same capture method.
- `demo_obj_snapshot()` (new, in `⚙️engine/🦀️component.rs`) — parses a new `DEMO_OBJ_TEXT` const
  (matching `example.obj` verbatim) via the real `decode_obj`, then re-decodes its own `encode_obj`
  output once to land on the documented SECOND-generation fixed point (this module's own retention
  law) — so `print_dsl(demo_obj_snapshot())` is genuinely stable, verified by
  `fixture_honesty_law`.

## 6. Conformance laws + registration

Added a `conformance_laws` module inside `⚙️engine/🦀️component.rs`'s own `mod tests` (not any
framework file), mirroring `stdio.zip`'s exact structure: `committed_facet_files_parse`,
`grammar_conformance_law`, `ops_grammar_conformance_law`, `diff_grammar_conformance_law`,
`protocol_walk_law`, `fixture_honesty_law` — all 6 pass. `protocol_walk_law` asserts
`consumed == bytes.len()` for all three facets (no `backward`/`jump` in any of `obj`'s protocol
files, so the ordinary full-consumption law holds, unlike zip's relaxed `<=`).

Registered the full 5-role `LanguageSpec` set (Document/Ops/Diff/Pack/Spr, all
`dsl::passthrough_hooks`) in `register_pilot_languages()` — previously only the Document role was
registered. Added `register_schema_specs()`: `dsl::registry::register_schema_spec("stdio.obj",
ObjSnapshot::__dsl_spec)` — real, since `ObjSnapshot` genuinely derives `#[derive(dsl::DslRecord)]`.
`ObjDiff` has NO such call (hand-rolled, no `__dsl_diff_spec` exists) — filed as a `mechanism_gaps`
entry rather than fabricating a spec.

## 7. JSON-transfer elimination

Grepped `obj`'s own `.rs` files for `serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/
`Value` inside `ArtifactPack`/`OpBinary`/`DiffCodec` impl blocks — clean. The only `serde_json`
mentions anywhere under this artifact's tree are inside STALE COMMENTS in the old placeholder
`.abnf`/`.g4`/`.ebnf`/`.spicy`/`.ksy` schema-representation files (out of this ticket's scope —
only `.grammar.semio`/`.protocol.semio` are the recipe's required deliverables) describing a wire
shape the real Rust code never actually used.

## 8. Verification

`cargo test -p semio-s-plugin-stdio --lib "artifacts::obj::"` — **30 passed, 0 failed** (includes
all 6 new conformance-law tests, all pre-existing `⚙️engine`/mutations/diff/analyzer/composer/mesh
import-export tests).

`cargo test -p semio-s-plugin-stdio --lib` (whole crate) — **1710 passed, 4 failed, 3 ignored**.
All 4 failures are in `artifacts::md::...`/`artifacts::xml::...` — unrelated concurrent-session
churn (confirmed by file path: neither is `obj`, both are other FG-wave artifacts under active
edit in this shared tree, matching the ticket's own documented concurrent-churn warning). Zero
`obj`-attributable failures.

`bun run ./📜️script.ts policy` — full repo-wide report grepped for `🧊️obj`/`stdio.obj`: only 2
pre-existing `os-state-authority/item-scope-global` hits in `🎹️composer/🦀️component.rs` (a
`OnceLock`, unrelated to grammar/protocol/fixture/registration/json-transfer, pre-existing, not
introduced by this wave). Zero hits anywhere in the report for
`POLICY_GRAMMAR_PARSEABILITY`/`POLICY_PROTOCOL_PARSEABILITY`/`POLICY_FIXTURE_HONESTY`/
`POLICY_LANGUAGE_REGISTRATION`/`POLICY_STDIO_JSON_TRANSFER_BAN` — zero new breaches.

## 9. Mechanism gaps (already-known, applied per precedent, not rediscovered/fixed)

| id | engine area | symptom | how obj hits it | workaround applied |
|---|---|---|---|---|
| `txt-opbinary-record-body-wire-is-framework-generic` | protocol/pack | Past `format`/`ordinal`, `OpBinary`'s record-body wire (varint symbol table + self-describing tag+value) isn't expressible by `Array`/`Ref`/`repeat` | `ObjMutation`'s `OpBinary` (derive-driven, same as `stdio.txt`) | `format`/`ordinal` genuinely byte-walked; record-body tail is one opaque `chain bytes` |
| `register-schema-spec-needs-recordspec` | `dsl::registry::register_schema_spec` | Requires `fn() -> RecordSpec`; a hand-rolled (non-`DslDiff`) diff type has none | `ObjDiff` (3b tri-state blocker, hand-rolled `DiffCodec`) | Skipped `register_schema_spec("stdio.obj#diff", ...)`; only the snapshot's real `ObjSnapshot::__dsl_spec` is registered |

No NEW/undocumented mechanism gap was discovered by this wave — both rows above are pre-existing,
already-known entries from the recipe's own consolidated table (§5), applied per the recipe's own
"already worked around by 6 real pilots, just apply the same pattern" instruction.

## 10. Deviations from a literal recipe-checklist reading

- The mutations grammar's `SetSnapshot` block is modeled PRECISELY rather than falling back to
  `REST` the way `stdio.zip`'s own `SetSnapshot`/`AddEntry`/`SetEntryExtra` do — a genuine,
  traced-not-assumed finding that `obj`'s own nested record types are all flat (see §2), not a
  deviation from the recipe's own rules (the recipe explicitly prefers precise modeling wherever
  finite; `REST` is presented as the fallback for genuinely unbounded nesting, which `obj` never
  has).
- `example.obj` is real content but untracked by git (`.gitignore:401: *.obj`) — a pre-existing,
  out-of-scope repo rule; noted rather than worked around.
- Added one extra `SetMtllib{mtllib: Some(...)}` case to the pre-existing `demo_mutation_cases()`
  fixture set (previously `None`-only) so the grammar's optional-field branch has real coverage.

## Files touched (all within `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/**`)

- `🏅️standards/🔖️3.0/⚙️engine/🦀️component.rs` — `demo_obj_snapshot()`/`DEMO_OBJ_TEXT`, 5-role
  registration, `register_schema_specs()`, `conformance_laws` module (6 tests).
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` — rewritten.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` — rewritten.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` — rewritten.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — fixtures promoted to
  `pub(crate)`, `demo_mutation_cases()` added, one new demo case.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` — rewritten.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` — rewritten.
- `🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — fixtures promoted to
  `pub(crate)`, `demo_diff_cases()` added.
- `📚️examples/🎬️demo/🖼️assets/example.obj` — rewritten (real content; gitignored/untracked).
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — genuine fixture.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — genuine fixture (new file).
