STATUS: GREEN (all verified). Rust assert-mode law passed (`semiocodec-spec3`, rc=0). One shared spec-driven record-document codec per language reproduces all 39 committed Rust `*.pack.semio` byte for byte; 13 Python adapters + the TS mesh adapter ported and passing their identity-round-trip locally; codec laws 13/13 (bun) and 13/13 (python).

## What was built (schema-first, multi-implementation)
Root = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/` (the stdio-semio artifact).
- `🚪️io/💾️binary/🎒️record-document/🐍️.py` and `…/🟦️.ts`: ONE module per language, no runtime deps (system `zlib`/`node:zlib` only to INFLATE; the compressor, CRC-32C and BLAKE3 are implemented). Written from the committed wire contract (`🧰️framework/🔨️modules/🎒️pack`), independent of the Rust code:
  - container: `SPK` header (CRC-32C), CRC-framed segments, raw-deflate = the pack's one deflate (single fixed-Huffman block over greedy hash-chain LZ77, 3..258/32 KiB/128 probes), symbols/document/chunk/chunk-table/manifest/end segments, 84-byte footer with BLAKE3 content hash; semio binary envelope.
  - record layer: canonical tag-prefixed record body (sorted field ids, deterministic interning pre-pass, packed numeric runs, TableSoA, Statements/Block/Map/Value, chunked bytes >= 256 KiB), driven by the exported record spec; schema hash recomputed from the graph (BLAKE3 over the canonical bytes) and required to equal the carried hash.
  - document layer: camelCase JSON snapshot <-> record tree (`encode_pack/decode_pack`, `encodePack/decodePack`).
- Exported record specs (schema-first, language-neutral): `🪆️subsets/<subset>/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json` for the 17 record-pack subsets (animation, audio, brep, cad, document, drawing, graph, image, kit, mesh, model, object, presentation, table, text, value, video). Each = `{schema:"semio.pack.record-spec/v1", envelope, schemaHash, graph (= Rust `PackSchemaGraph::to_json`), annotations}`.
  - `graph`/`schemaHash`/`envelope` are produced by the Rust derive; the Rust law `🧿️semio/🧪️tests/🗺️record-spec/🦀️.rs` (new `[[test]] record_spec_law` in `📦️packages/🦀️rust/Cargo.toml`) asserts committed == derived (ignoring `annotations`); `SEMIO_RECORD_SPEC_WRITE=1` prints the documents (`RECORD-SPEC|<subset>|<json>`; used once to create the files).
  - `annotations` hold the five facts the graph cannot say (authored once from the Rust sources by `C/1029-tools/semiocodec/annotate_specs.py`, proven by the 39 fixtures): `single` (31 `block(statements)` fields = one enum value vs `Vec`), `float32` (f32 fields: mesh Rgba/metallic/roughness, drawing Rgba/opacity, audio samples), `tags` (presentation `SlideShape` is tagged `shapeKind`), `verbatim` (enums without `rename_all_fields` keep snake_case payload keys), `nested` (`ArtifactRef.dialect` flattened on the wire; object + kit).
- Tests (language-agnostic): `🧪️tests/🎒️record-document/{🟦️.ts,🐍️.py,🧫️fixtures/🔣️.json}`. Fixture = CRC-32C RFC 3720 vectors, BLAKE3 vectors computed by `@noble/hashes` (3rd party), 4 deflate segments the Rust writer produced, and the 17 subsets with all 39 committed packs + 4 pack<->serde-snapshot pairs. Laws: primitives vs vectors/zlib/noble; every spec hashes to its carried hash; every Rust pack decodes -> re-encodes to the SAME BYTES (also with `omitNulls`); decoded pack == the Rust serde snapshot JSON where one is committed; foreign-subset / flipped-byte / unknown-key / unknown-variant refusals.
  - run: `python3 <…/🧪️tests/🎒️record-document/🐍️.py>` (unittest, pytest-collectable) -> 13 OK (37 s); `bun test ./<…/🧪️tests/🎒️record-document/🟦️.ts>` -> 13 pass (2 s).

## Cross-implementation oracle result
All 39 Rust-written packs (examples, assets, mutate fixtures; incl. 95 KB presentation, 22 KB mesh, 23 KB model tower) round-trip byte-exactly through BOTH independent implementations, container hash (`schemaHash`), BLAKE3 content hash, CRC-32Cs and deflate bytes included. The recomputed schema hash of all 17 exported graphs equals the Rust one (the specs refuse to load otherwise).

## Suites ported (hand pack code deleted, `parse_pack/pack_bytes` now delegate; docstrings updated; dead hand constants removed)
Python: model, table, cad, document, object, presentation, value, text, image, graph, drawing, brep, kit. TypeScript: mesh (`🧪️tests/🔺️mutate-semio-mesh/🟦️.ts`, region `🎒️Pack` replaced by 2 delegating functions).
Deleted: `📦️pack-carrier/🐍️.py` of model, graph, drawing, brep. Per-suite adaptations: graph (geometry words `{"bits"}` <-> numbers around the codec), graph/drawing/kit use `omit_nulls=True` (their text carriers omit absent optionals), object/kit lose the dead `read/write_pack_reference`.
NOT touched: animation, video, audio (export no pack bridge, no pack code) and flow (its Rust subset still has the hand pack, the py hand codec still agrees with it).
Local runs: harness `C/1029-tools/semiocodec/run_suite.py <suite 🐍️.py>` (stub of the repo test host, feature steps fed to the context) -> identity-round-trip PASS for all 13; `bun run C/1029-tools/semiocodec/mesh_identity.ts` -> OK for the 22 KB fixture and the cube. The mutate/inverse/spec-vector handlers were not changed (they need the full host/feature plan).

## Notes / decisions
- f32: documents hold the widened double (what the suites' text parsers produce); the encoder narrows `float32` fields, the decoder returns the widened double (or the shortest decimal with `narrow_f32`/`narrowF32`, used against serde vectors).
- Optional-enum fields (`Option<enum>` = `pcurve`) are a PRESENT field holding an empty statements list on the wire (the `single` annotation drives this both ways).
- Python needs only the stdlib and is 3.9-compatible (system python here is 3.9.6; pin says 3.14).
- No Rust file of `semiodsl` edited. New Rust: only `🧿️semio/🧪️tests/🗺️record-spec/🦀️.rs` + one `[[test]]` entry in `📦️packages/🦀️rust/Cargo.toml`.

## Verification status
- Rust write-mode run `semiocodec-spec1` compiled the top crate (37 min, queue 35 min) and the new test `record_spec_law` PASSED while printing the 17 derived documents; they are the committed spec files (`graph`/`schemaHash`/`envelope` byte-for-byte equal as JSON, re-checked by script; `annotations` are authored on top). `repo cargo` then reported "Original child phase deadline exhausted" after the test had passed.
- Assert-mode run `semiocodec-spec2` failed to compile (peer's value-retirement edit, since reverted). Re-run `semiocodec-spec3` (`C/🗑️generated/1029/semiocodec-spec3.{txt,rc}`): `committed_record_specs_equal_the_derived_record_specs ... ok`, `test result: ok. 1 passed`, rc=0. The committed specs equal the derive.

## Next step for the coordinator
- Run the full host suites for the 14 ported adapters (`repo test` of the mutate-semio-* cases) to compare the Python/TS oracle projections with the Rust subject digests (`packDigest`, `packLength`): these now only need the Rust record pack and the codec to agree, which the 39 fixtures prove.
- Optional: wire `python3 <…/🧪️tests/🎒️record-document/🐍️.py>` and `bun test ./<…/🧪️tests/🎒️record-document/🟦️.ts>` into the stdio-semio test targets.
