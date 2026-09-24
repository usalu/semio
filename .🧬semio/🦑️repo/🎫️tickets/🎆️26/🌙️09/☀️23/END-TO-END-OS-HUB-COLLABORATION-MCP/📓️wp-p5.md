# WP-P5 Structural Pack-Schema Identity, process3d Derived Pack, Derived Text DSLs

Slice P5, session 10. Follows `📓️wp-p4.md` (Open items) and `📓️wp-w1.md` §4.2.

## Status

- [x] 1. `os_pack::schema_hash` structural + recursive (nested records, enums, collections, options, composed children/links), cycle-safe, schema-ordered; laws (nested flip, independent blake3 recomputation, stability native vs wasm32); w1 (+h4) messaged
- [x] 2. process3d `Process3dSnapshot` on the derived pack, retained reader ported onto framework retained pack readers, handcrafted codec deleted, identity law; w1 messaged
- [x] 3. converted kinds' text DSLs: derive (`DslArtifact`) or documented reason

## Findings

- The hub never recomputes the pack-schema hash. `🌎️hub/🏗️bootstrap` and `📇️directory` only compare descriptor
  strings (`SocketHelloV1.pack_schema_hash` against `descriptor.pack_schema_hash`). The only pins are the MCP probe constants:
  Rust `PROBE_PACK_SCHEMA_HASH` (`🌉️mcp/🏠️workspace/🦀️.rs`) and `MCP_PROBE_PACK_SCHEMA_HASH` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`).
  There is no TS/Go reimplementation. stdio codecs pin a sha256 of their committed schema file, which is a different identity and is unaffected.
- `encode_document` writes `schema_hash(spec)` into every `.spk` manifest. The new hash therefore changes the bytes of every
  pack, including the pinned pack-hex fixtures (composed-pack-schema, durable-group outcomes and the decision hash derived from them).
- The derive (P6) no longer emits `ArtifactDsl`/`ArtifactPack`. Text can still come from the same derived spec through
  `dsl::parse(body, &Self::__dsl_spec(), ..)`/print (the block/remodel pattern), so task 3 is possible.

## Changes

### 1. Structural pack-schema identity (landed; w1, h4 and r4 messaged)

- `🎒️pack/🌱️value/🦀️.rs` region `🔖️SchemaHash`: new `PackSchemaGraph { records }`, `PackSchemaField`, `PackSchemaShape`.
  `PackSchemaGraph::of(spec)` first discovers every `fn() -> RecordSpec` once, so it is cycle-safe. It then runs Moore
  partition refinement, a bisimulation quotient that makes duplicated or recursive spec fns collapse to the same graph. Records are
  numbered breadth-first from the root in schema order: fields by id, enum variants by (ordinal, tag), statements by keyword.
  `canonical_bytes()` is the documented encoding, and `to_json()` is its language-neutral form.
  `schema_hash = blake3(canonical_bytes)`. Hashed: id, key, optional, flatten, full shape tree (enum tables, tuple
  arity, units, ref kinds, embed languages, nested records). Not hashed: keyword, layout, position, call_name, defines (text-only).
- New fixture `🎒️pack/🌱️value/🧫️fixtures/🔑️schema-hash/🔣️.json` (shape tag table, 5 hand-authored graphs covering flat, nested,
  nested-inner-field-changed, recursive and every-shape, plus pinned canonical hex and hash). New laws in `🧪️tests/🔬️unit`:
  graph equals the fixture; an independent test-side encoder over the fixture JSON produces the same bytes; third-party `blake3`
  over those bytes equals the pin and `schema_hash`; re-running gives the same result; a nested change flips the hash (inner field,
  enum ordinal/tag, optional, tuple arity, list-of-record inner, statement keyword, embed vs embedFrom); recursive and duplicated
  spec fns give one graph; text-only presentation is ignored.
- P4's composed law (`🏪️store/🧪️tests/🧩️composed-pack-schema`) is rewritten. The fixture carries the full 4-record graph
  (snapshot, ArtifactChild, ArtifactLink, LinkPin), canonical hex, blake3 pin, and a new `renamedLinkRoleSchemaHash`. A rename
  inside `ArtifactLink` now flips the hash. The pack hex of both documents is re-pinned.
- MCP probe pins updated to the new hash `0302ac7c…728e`: Rust `PROBE_PACK_SCHEMA_HASH` and hub `📜️script.ts`
  (`MCP_PROBE_PACK_SCHEMA_HASH` and `probePackSchemaHash`).
- The schema-hash laws live in a kernel integration test (`[[test]] pack_schema_hash`,
  `🎒️pack/🌱️value/🧪️tests/🔬️schema-hash/🦀️.rs`) so they also run on `wasm32-wasip2` under wasmtime. The kernel `--lib`
  test target does not compile for wasm32 because of peers' native-only directory tests (`claim_local_hub_credential_with`,
  `admit_document_socket`, see generated/kernel-schema-hash-wasm32.txt of the first attempt).
- Durable-group fixtures (for r4): store `🗄️durable-group/🧫️fixtures/🔣️.json` (3 outcome packs and sha256s, unsignedJson,
  decisionSha256 `477e038b…3e8b`) and db `📓️durable-group-journal/🔣️.json` (decisionSha256).

### 2. process3d on the derived path (landed; w1 messaged)

- `🏭️process/…/🧊️process3d/🦀️.rs`: `WorkingSolid` and `ProcessMeasure` derive `dsl::DslEnum`; `Stock` and `ProcessStep` derive
  `dsl::DslRecord`; lengths carry `#[dsl(unit = "m")]`. A single `tagged_variant_field!` macro gives MeasureRecipe/WorkingSolid/
  ProcessMeasure their one-statement `DslField` (it replaces the hand-written MeasureRecipe impl).
- `📸️snapshot/🦀️.rs`: rewritten. `Process3dSnapshot` derives `dsl::DslRecord` (`extension = "process3d"`). `ArtifactDsl` is spec-driven
  (`dsl::parse`/`dsl::print`) and `ArtifactPack` is `encode_document`/`decode_document` over `__dsl_spec()` with `record_spec()`.
  Deleted: the hex/bracket text codec, the format-2 binary codec, and the ~1000-line `Process3dRetainedSnapshotReader`
  with its stock cursor.
- New typed owner `Process3dMountedSnapshotOwner` (`🔖️MountedTypedSnapshotOwner`). Typed frames (root, workshop, machine,
  capability, root/machine/capability lists, rule statements) fill domain values directly. Bounded leaf values (pose, stock,
  steps, children, parameters, rules, recipe) are built from value tokens and converted through the derived `DslField` when they
  complete. There is no whole-document record and no batch decoder. On close, partial machines and capabilities are handed back
  into the candidate for incremental retirement.
- New framework `store::mounted_pack_rt::{RetainedTypedPackSession, RetainedTypedPackOwner, RetainedTypedPackCloseStep}`
  (`🏪️store/🧾️document/📥️mounted-pack/🦀️.rs`). It is the artifact-neutral version of generation3d's session and is gated by an
  artifact header (process3d: its semio pack header) before any semantic allocation. Decoded sizes are bounded by
  `ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES`. Measured: `canonical` as the segment limit refuses deflated segments
  (`LimitExceeded("segment length exceeds max_segment_len")`). generation2d/3d carry the same latent limit, and p6 has been told.
- `🧬️mutations/💾️binary/🦀️.rs`: `Process3dSnapshotDecodeAuthority` now streams the envelope hex pack nibble by nibble into the
  session (the generation3d authority shape). The mutation-wire primitives (`write_/read_*`, string/pose/solid/child/measure/step
  cursors) moved here as region `🔖️MutationWirePrimitives`, since only the mutation wire uses them now.
- Laws: `📸️snapshot/🧪️tests/🔬️retained-mounted-laws` (pack identity plus dsl/pack equivalence; one-grant round trip of a document
  with every solid/measure/recipe/rule kind that takes ≥256 grants; foreign header rejected before semantic allocation; an
  interrupt every 13 grants closes to terminal-empty with exact allocation release; no batch-decoder edge in the region). The
  string-cursor law moved to `🧬️mutations/💾️binary/🧪️tests/🔬️retained-laws`.
- Text fixtures regenerated through `regenerate_example_fixtures`, which now writes to `$PROCESS3D_FIXTURE_OUT` instead of a
  hard-coded old ticket path: `🖼️assets/🎬️demo/🗣️.dsl.semio`, `🖼️assets/🌲️concrete-forest/🗣️.dsl.semio`, `PROCESS_3D_PLATE_EXAMPLE_TEXT`.
- Slip: I ran `git rm --cached` once on the deleted `🔬️retained-structural-laws/🦀️.rs`, which stages its deletion in the index.
  The file is gone on disk as intended. No other git command was used.

### 3. Derived text DSLs (landed for all P4 kinds, tests below)

- The derive cannot emit `ArtifactDsl` (P6 made it handcrafted by design). The handcrafted impl can still be the spec-driven
  engine over the same derived record the pack uses (`dsl::parse`/`dsl::print`, the block/remodel pattern). That is what every
  converted kind now does, so text and pack are two encodings of one spec:
  - snapshot's own spec: writer (+ `attach_writer_document_text`), cad (+ `require_exact_children`), lowpoly, sequence, forms
    (+ envelope identity check and `validate`), animate presentation, layout, norm en1990, din18599, process3d;
  - private pack record, through new `print_pack_record_text`/`parse_pack_record_text` beside it: playbook, equation,
    procedure, wires, jack, raster, energy. For wires, jack and energy the record conversions were factored into
    `from_snapshot`/`into_snapshot`, now shared by pack and text.
- Every hand-built text codec is deleted: hex/bracket primitives, `print_*_snapshot_body`/`parse_*_snapshot_body`, the JSON-hex helpers.
- Fixtures converted once by a temporary test that parsed each committed fixture with the old codec and printed it with the new
  one (removed afterwards; outputs under generated/text-fixtures*). 34 `🗣️.dsl.semio` assets: writer (2), cad, lowpoly,
  sequence (5), animate, playbook, layout, en1990, din18599, equation, procedure, wires, jack, raster, energy (15). process3d's 2
  assets and `PROCESS_3D_PLATE_EXAMPLE_TEXT` came from its own `regenerate_example_fixtures`.
- Handcrafted grammars (`📖️.grammar.semio` + `🔤️.ebnf` + `🅰️.g4`) for the facets whose grammar described the old hex format: playbook,
  forms, layout, lowpoly (its grammar predated the mesh child), equation, procedure and raster. The other kinds keep their
  generic `payload = OCTET+` placeholder, which is unchanged in kind.
- Tests rewritten where they exercised the old codec's failure modes: lowpoly (5 derived-grammar error/escape laws), layout
  (`parse_dsl_reports_derived_grammar_errors`), writer (`writer_dsl_prints_readable_scalar_fields`; the example law now compares
  the committed asset byte-exactly to the printer output).
- Not updated (peer-owned narrative): `✒️writer/…/🧪️tests/✒️mutate-writer-1/🐍️.py`, `🥒️.feature` and `🔮️oracles/🔣️.json`. They describe
  the old hex carrier as an identity-round-trip gap, and that description is now stale.
- Also fixed (p6's pointer): process3d's retained resumable work re-derived its extent (an O(len) scan) on every step. That made
  the operation quadratic, and the framework already computes the extent once in Preflight. The per-step recomputation is removed.

## Test Results

| crate | command | result | capture |
|---|---|---|---|
| semio-framework-os-kernel | cargo test --lib schema_ composed_pack_schema | 34 passed | generated/kernel-schema-hash-4.txt |
| semio-framework-os-kernel | cargo test --test pack_schema_hash (native) | 4 passed | generated/kernel-schema-hash-native.txt |
| semio-framework-os-mcp | cargo test --lib probe_pack_schema_hash | 1 passed (after pin update) | generated/mcp-probe-hash-2.txt |
| semio-s-artifact-process-process3d | cargo test | 360 passed, 3 ignored | generated/process3d-test-2.txt |
| semio-s-plugin-process (+ metal, wood, robotic, concrete) | cargo check --tests | ok | generated/check-*.txt |
| semio-framework-os-kernel | cargo nextest --lib --test pack_schema_hash (after mounted session) | 1134/1134 | generated/kernel-lib-nextest-2.txt |
| semio-s-artifact-process-process3d | cargo test (after extent fix) | 360 passed, 3 ignored | generated/process3d-test-3.txt |
| writer, cad, lowpoly, sequence, forms, presentation, playbook, layout, en1990, din18599 | cargo test | 176 / 433 (1 ignored) / 299 / 209 / 200 / 329 / 158 / 395 / 156 / 173, all passed | generated/text-swap-test-1.txt (+ -2 for writer/lowpoly/layout/forms/playbook after test and grammar edits) |
| energy, equation, wires, jack, procedure, raster | cargo test | 6292 (1 ignored) / 392 / 193 / 167 / 149 / 227 + 1 fail. The fail is `raster_standalone_control_max_plus_one_…` (mutation binary, not text), which passed alone | generated/text-swap-test-3.txt, -4.txt |
| semio-framework-os-kernel | cargo nextest --lib | 1134/1134 passed | generated/kernel-lib-nextest.txt |
| semio-framework-os-kernel-db | cargo nextest --lib | 700/702. Two allocation-fault laws failed (db_engine catalog over-allocation, db_artifact close-fault) and passed on rerun, see db-rerun | generated/db-lib-nextest.txt |
| semio-framework-os-kernel-db | nextest rerun of the 2 fails plus `durable` | 12/12 passed | generated/db-rerun.txt |

| 18 crates (process3d, process plugin and the 16 text-converted kinds) | cargo check --target wasm32-wasip2 (mutex) | all Finished, 0 errors | generated/wasm-check-process.txt |
| semio-framework-os-kernel | cargo test --target wasm32-wasip2 --test pack_schema_hash (mutex, wasmtime runner) | NOT RUN. The test target build also compiles the kernel's native-only `pack`/`spr` CLI bins, which are cfg'd out on wasm32 (`os_pack::cli`/`os_spr::cli` not found), so the build fails before any test runs. Cross-target stability is argued, not measured: the canonical bytes contain no pointer values or target-dependent integers (ordinals, varints and strings only), and `fn` addresses are only used as discovery keys, collapsed by the bisimulation quotient | generated/kernel-schema-hash-wasm32.txt |

## Open

- The wasm32 run of the schema-hash laws needs the kernel CLI bins gated behind `required-features` or a non-wasm target (the kernel owner's call).
- generation2d/3d mounted sessions keep a latent `max_segment_len = canonical` limit (reported to p6). Both could move onto `RetainedTypedPackSession`.
- The writer `mutate-writer-1` narrative (`🐍️.py`, `🥒️.feature`, `🔮️oracles/🔣️.json`) still describes the retired hex carrier.
- Descriptors and catalogs must be regenerated by w1 (every kind's pack_schema_hash changed, and the text examples changed).

- h4's GIS receipt rejection (`gis/native-codecs/v1: exact private receipt rejected`) is not caused by the hash. Both sides of that
  comparison are computed live from the same codec. `cargo test -p semio-s-plugin-gis --test native_codecs` passes 3/3 on the current tree
  (generated/gis-native-codecs.txt). The remaining candidates are the catalog `version` against gis CARGO_PKG_VERSION, or a hub built
  while the kernel briefly failed to compile (about 06:30, my `mounted_pack_session` path bug, fixed). h4 has been told.

## Processes

