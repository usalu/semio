# 📓️ W2-W-geometry — wire-witness conversion of dxf, dwg, ply, obj, las, stl, bcf, epw

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-W-geometry, 2026-09-30. Brief: `🧭️plan.md` "W2-W brief",
recipe `📓️w2-s-report.md` F10, design §6/§11. glTF untouched.

## 1. Outcome

| check (per artifact, `--under ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<a>`) | before | after |
|---|---|---|
| `schema mutation-payloads` | dxf 26, dwg 12, ply 12, obj 8, las 6, stl 4, bcf 20, epw 6 = **94** | **0** in all eight; every leaf witnessed (dxf 18/18, dwg 2/2, ply 9/9, obj 21/21, las 14/14, stl 6/6, bcf 13/13, epw 12/12) |
| `schema mutation-inputs` | 0 | **0** (after §2.3's three new labels) |
| strict Ajv (`🧪️w2-s-stdio-check.ts`) | — | 867 leaves / 87 aggregates compile, 0 failures |
| `cargo check` 8 crates native + `--target wasm32-wasip2` | — | ok (only pre-existing warnings in peers' files) |
| `cargo test --lib` 8 crates (private `target-nde-w2w-geometry`, `CARGO_INCREMENTAL=0`) | — | bcf 38, dwg 71, dxf 36, epw 27, las 46, obj 44, ply 46, stl 44 — all pass, incl. every `semio_payload_law_*` |
| oracle crate unit tests (`--features oracles`, filter scope) | — | 71 pass (dwg 11, dxf smoke 2 = all 18 kinds forward+inverse, epw 6, las 7, obj 6, stl 10, …) |
| lint suite `test mutation-payload-parity` | — | 42 pass |

Case runs (`bun ./📜️script.ts parity exhaustive --case <c>`, oracle + subject + parity):

| case | result |
|---|---|
| 🌦️mutate-epw-energyplus | 50/50, parity 25/25 |
| 🔺️mutate-stl-ascii | 26/26, parity 13/13 |
| 🎩️mutate-las-1-0 / 📼️…-vlr / 📍️…-points | 34/34 p17/17 · 12/12 p6/6 · 12/12 p6/6 |
| 📐️mutate-obj-3-0 / 🎨️…-material | 78/78 p39/39 · 8/8 p4/4 |
| 🧱️mutate-ply-1-0 | 38/38, parity 19/19 |
| 🔀️mutate-bcf-2-1-snapshot | 4/4, parity 2/2 |
| 🔀️mutate-bcf-2-1 / 🔀️…-viewpoint | every oracle and subject scenario passes (34/34, 16/16); pipeline parity 2/17 and 1/8 — **pre-existing**, §4.2 |
| 🖊️mutate-dwg-ac1024 / 🖊️…-ac1018 | oracle 5/5; subject `set-version-info` rows fail in `encode_dwg` — **pre-existing**, §4.1 |
| 📰️mutate-dxf-r12 / 📊️…-tables / 🧩️…-entities / 🧱️…-blocks | every row now decodes on both sides; failures and parity 0/N are **pre-existing** (asset mismatch + an unwired pipeline), §4.3 |

`contract` phase: aborts repo-wide on 2,648 pre-existing breaches (none in these eight artifacts), as W2-S recorded — WRITTEN BUT
UNVERIFIED for that phase only.

## 2. What changed

### 2.1 Generic decode, production inverse (Rust)

- Every aggregate gained a `//#region 🚪️Reachability` bridge beside `apply_*` (the generated test host links only the artifact
  crate and cannot name the private `protocol` alias):
  - `decode_<x>_mutation_payload(kind, json) -> Result<Agg, String>` = `os_pack::json::from_json_str` → derive-generated
    `from_payload_value(kind, payload)`. Name aligned with W2-W-media's `decode_<x>_mutation_payload`.
  - `inverse_<x>_mutation(base, op) -> Vec<Agg>` = `Mutation::inverse` (dwg already had it).
  - Aggregates: `EpwMutation`, `StlMutation`, `LasMutation`, `ObjMutation`, `PlyMutation`, `DwgMutation`, `DxfMutation`,
    `BcfMutation`.
- All 17 Rust adapters (epw 1, stl 1, las 3, obj 2, ply 1, dwg 2, dxf 4, bcf 3): the hand-mapped params→op code
  (`mutation_from_spec`, `mutation_of`, `snapshot_of`, `json_to_*`, `hex_decode`, …) and the hand-mirrored subject inverses
  (`inverse_of`, reused oracle inverse specs) are **deleted**. The subject decodes the row through the bridge and inverts
  through the production inverse computed against the pre-mutation snapshot. The dxf/las/bcf adapter families are now
  identical files per family.

### 2.2 Feature rows → wire (`🧪️w2-w-geometry-rows.py`, idempotent)

| artifact | rows changed | wire shape now |
|---|---|---|
| epw | 4 | `insert-record.record` = `EpwRecord` object (35 named columns); `set-snapshot.snapshot` gains `schema`, records as objects |
| stl | 2 | `set-snapshot` = `{snapshot: {schema, solidName, triangles}}` (a real whole-document replacement incl. the name) |
| las | 6 | `set-snapshot` = full `LasSnapshot` (`LasHeader` wire incl. structural fields as the old adapter built them); VLR `data` = byte arrays |
| obj | 6 | `set-snapshot`: `schema`, `usemtl` (not `usemtlRanges`), no `mtllib: null` (skipped when `None`) |
| ply | 10 | cells = `PlyValue` wire `{kind, value}` / `{kind: "list", value: […]}`; `set-snapshot` gains `schema` |
| dwg | 4 | `set-snapshot` = `{snapshot: DwgSnapshot}`; defaulted members omitted (decode identical) |
| dxf | 34 | `{index, layer: DxfLayer}`, `{name, style: DxfStyle}` (`fontName`), `{index, entity: {"circle": {…}}}`, `{index, block}`, `{name, headerVar: {name, groupCode, value: {kind: "point", …}}}`; `set-snapshot` = whole R12 `DxfSnapshot` |
| bcf | 4 | `set-snapshot` = `{snapshot: BcfSnapshot}`; viewpoint `snapshot` = PNG byte array |

`no-mutation` sentinel scenarios (both mutate and inverse baselines) removed from epw, stl, obj, ply, dwg ×2, dxf, bcf, with
their adapter registrations (Rust + bcf TS) and prose. Each case keeps its identity round trip. The bcf `⏸️no-mutation-applied`
fixture pair, its `fixtureManifests` entry and its generator recipe are deleted.

### 2.3 Rust was the outlier — 16 leaf roots camelCase

The brief's wire is camelCase; TS twins, graphql and the features already spelled these roots camelCase while the leaf structs
emitted snake_case (W2-S-B2 had judged them consistent; the TS twins disagree). Per the approved repo rule Rust was fixed:
`#[value(rename_all = "camelCase")]` on dxf `SetHeaderVar`, dwg `SetVersionInfo`, ply `InsertRow`/`RemoveRow`/`SetRowProperty`,
obj `SetSmoothingGroups`/`SetUnknownStatements`, bcf `SetComment`/`SetTopicMarkup`/`InsertComment`/`RemoveComment`/
`InsertViewpoint`/`RemoveViewpoint`/`SetViewpointCamera`/`SetViewpointComponents`/`SetViewpointSnapshot`; their 16 leaf schemas'
root properties/`required` renamed (formatting byte-preserved). The three ply `elementName` inputs got `x-semio-ui` labels
(en/de) since the glossary only knew `element_name`. Repo scan: no other consumer of the snake keys.

### 2.4 Oracles read the same wire

- epw: `EPW_RECORD_COLUMNS`, `record_cells`/`record_wire`, `epw_snapshot_wire`; inverse specs built from it.
- stl, las: `oracle_inverse_spec` → `Result<Option<Json>>` (`None` = nothing to undo, no `no-mutation`); las gained the full
  `LasHeader`/`LasVlr` wire codec beside its unchanged projection.
- obj: `usemtl`, `lineIndex` retained per unknown statement (the independent parse now records the 0-based line), `oracle_round_trip`.
- ply: cells read/written as `PlyValue` wire (tag checked against the declared property type), `ply_snapshot_wire`,
  `oracle_round_trip`; the adapter's inverse specs are derived from the base document instead of hard-coded rows.
- dwg: `set-snapshot` reads the `DwgSnapshot` wire; `oracle_inverse_spec`+`documentHex` replaced by `oracle_restore(base, mutated, spec)`.
- dxf: wire codecs for entity (externally tagged), layer, style, linetype, block, header var, and `snapshot_drawing` (a fresh
  `dxf::Drawing` built from the snapshot alone; its ensured default tables dropped); undo as `Undo::{Nothing, Apply, Original}`;
  `oracle_round_trip`. The new whole-replacement `set-snapshot` is EQUAL between oracle and subject (projection diff).
- bcf: byte-array snapshots/parts, `schema` in the snapshot wire, `Option` inverse, `oracle_round_trip`; hex helpers deleted.
- Unit tests of every touched oracle module updated to the wire (no `no-mutation` left).

### 2.5 Lint: re-exported vocabularies

DWG AC1018 owns no leaves; its aggregate schema is `allOf → ac1024` (as its Rust `pub use`s). `mutationPayloadParityReport`
(`🧪️test/🧬️schema/📋️orchestration/🟦️.ts`) now maps a feature row to leaves under the owner's standard **or** under the
standard of any aggregate the owner's `🧬️mutations/🔣️.json` re-exports (`allOf` of `$ref`s, no own `oneOf`). Only this one
document in the repo has that shape. Evidence: dwg 0 findings, both ac1018 rows witness ac1024 leaves; lint suite 42/42; the
file itself type-checks clean (41 `tsc` errors are all in other, transitively imported modules). A corpus case was not added:
the corpus exercises row parsing, not tree mapping.

## 3. Verification commands (all run, results seen)

`schema mutation-payloads|mutation-inputs --under <artifact>` ×8; `bun 🧪️w2-s-stdio-check.ts`; gated `cargo check -p <8 crates>`
native and `--target wasm32-wasip2`; `cargo test -p <8 crates> --lib`; `cargo test --manifest-path …/🔮️oracles/📦️packages/🦀️rust/Cargo.toml
--features oracles --lib -- epw stl las obj dwg dxf ply bcf`; `bun ./📜️script.ts test mutation-payload-parity`; `parity exhaustive`
for all 17 cases above. Outputs in `🗑️generated/w2w-geometry/`.

## 4. Open items (pre-existing, not introduced here)

1. **dwg `set-version-info` cannot be encoded.** Since commit `56b837a` (09-24) `encode_dwg` refuses any non-preamble-only
   document whose version is not `AC1024` ("this writer emits AC1024"), yet both DWG features drive `set-version-info` to
   `AC1032` / `AC1018` on the real R2010 container. The old hand mapping built the identical op, so both subjects failed before
   too. Decision needed: refuse such versions at apply (invariant + rows), or let the R2004+ writer stamp other R2004-family
   sentinels. The AC1018 case's whole premise depends on the latter.
2. **bcf pipeline parity.** `decode_bcf` drops viewpoint `camera`/`components` when reading the jszip-generated `⬅️before.bcf`
   pairs, so every row that keeps viewpoint-01 differs from the committed `➡️after.bcf` (1–2 diffs, always those two members);
   set-snapshot and identity (and the snapshot subset) are equal. Mutation semantics are unaffected; the codec needs the fix.
3. **dxf cases.** (a) `asset://🚏️bus-shelter/🖊️.dxf` resolves to the subsets' 445 KB **R2000** asset (INSBASE at origin, one
   layer, one entity, no DIMS/NOTES/DASHED), while every row was authored against the 9 KB R12 example — so the remove/set
   rows are refused by both sides and `remove-header-var` moves nothing. The oracle smoke test on the R12 example passes all 18
   kinds. (b) The `semantic-dxf-r12-v1` profile declares the `dxf-r12-reader-compare-v1` pipeline (since `b0dfa0f`, 09-05) that
   needs `expected-dxf`/`actual-dxf` artifacts no DXF adapter emits → parity 0/N. (c) Even the identity round trip differs in
   `linetypes` (the `dxf` writer's normal form). Re-homing the DXF cases onto their fixture pairs is a separate task.
4. Plugin-level `cargo check --target wasm32-wasip2 -p semio-s-plugin-stdio` not run (the eight member crates were); descriptor
   regeneration stays with the coordinator (no snake keys found in `🛂️.descriptor.semio`).
5. obj's reader-oracle corpus still carries a `no-mutation-no-op` identity recipe (`🏭️generator`, fixture manifest) — not a
   feature row, not lint-visible; left for the owner.

## 5. Files

- Ticket script (kept): `🧪️w2-w-geometry-rows.py`. Scratch: `🗑️generated/w2w-geometry/` (left for the coordinator's sweep).
- Mutations (bridge): `<a>/…/🧬️schema/🧬️mutations/🦀️.rs` for epw, stl, las (🎩️header), obj (📐️geometry), ply, dwg (🔟ac1024),
  dxf (📰️header), bcf (🖊️markup).
- Leaf structs + schemas (§2.3): the 16 leaves' `🦀️.rs` and `🧬️schema/🔣️.json`.
- Features: 14 `🥒️.feature` (epw, stl, las header + vlr, obj geometry, ply, dwg ×2, dxf ×4, bcf markup + snapshot; las points, obj material and bcf viewpoint rows were already wire after §2.3).
- Adapters: 17 `🦀️.rs` + bcf markup `🟦️.ts`.
- Oracles: `🔮️oracles/🦀️.rs` of epw, stl, las 🎩️header, obj 📐️geometry, ply, dwg 🔟ac1024, dxf 📰️header, bcf 🖊️markup, and
  their unit/smoke tests; epw + bcf `🔮️oracles/🔣️.json`; bcf `🏭️generator/📜️script.ts`.
- Deleted: bcf `🖊️markup/🧫️fixtures/⏸️no-mutation-applied/{⬅️before.bcf,➡️after.bcf}`.
- Lint: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts` (§2.5).
