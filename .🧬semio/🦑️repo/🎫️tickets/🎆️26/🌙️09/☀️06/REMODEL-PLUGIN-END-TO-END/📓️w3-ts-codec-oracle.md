# W3 — Remodel TypeScript twin: schema fidelity, JSON codec, DSL reader, cross-language fixture oracle

Scope: every `🟦️.ts` under `✏️s/🔌️plugins/📸️remodel/`, that package's `🧪️tests/🟦️.ts` vitest config, and
`.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc`. No Rust, feature, Python or oracle-JSON file was touched.

## 0. Continuation from the rate-limited W3

The first W3 was killed at 01:20–01:25 but its work **had** landed and auto-committed at `5e03e56997`
(`git diff 3a6a9d6bfc HEAD -- …/📸️remodel` shows two `🟦️.ts` files changed):

- `🧬️schema/📸️snapshot/🟦️.ts` grew 30 → 792 lines: the full domain type set plus a `RecordSpec`/`ValueSpec`
  table for every record, with Rust `snake_case` names declared once and both wire renames (`camelOf`,
  `kebabOf`) derived from them.
- the json **import** leaf gained a total spec-driven decoder.

The status log's "W3 had only read" is wrong for those two files. This packet builds on them rather than
restarting; nothing the earlier W3 wrote was discarded.

## 1. Files written

**Schema (`🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/`)**

| file | what it now is |
|---|---|
| `📸️snapshot/🟦️.ts` | +the codec runtime: `RemodelingCodecError`, `decodeValue`/`decodeRecord`, `floatLexeme`, `writeValueJson`/`writeRecordJson`, `decodeRemodelingSnapshot`, `encodeRemodelingSnapshot`, `remodelingSnapshotToJsonText`, `remodelingSnapshotFromJsonText`. `ValueSpec`'s `tuple` gained a float **width** (`w: 32 \| 64`) so `[f32; N]` and `[f64; 3]` narrow and print correctly. |
| `🟦️.ts` | the seven `{ [key: string]: unknown }` escape hatches are **gone**. `RemodelingArtifact` re-uses the snapshot's real types, adds the four UI records with their specs and defaults, and gains the `durableArtifacts` field the old mirror lacked. `remodelingArtifactFromSnapshot`/`ToSnapshot`, `decodeRemodelingArtifact`, `REMODELING_ARTIFACT_SPEC`. |
| `🔺️diff/🟦️.ts` | the eight escape hatches are gone. All **18** fields typed (including `durableArtifacts`, which the old mirror lacked and which the old `assets?: Record<string, ImageAsset>` typed wrongly), `REMODELING_DIFF_SPEC`, `decodeRemodelingDiff`, `remodelingDiffToJsonText`, and a faithful port of the hand-written `MutationDiff` `apply`/`absorb` (`🔺️diff/📝️text/🦀️.rs:79`), plus `remodelingDiffLanes`. |
| `🧬️mutations/🟦️.ts` | was a bare tag-string list. Now **all 35** payload interfaces + `RecordSpec`s, `decodeRemodelingMutation` (tag-aware, total validation), `remodelingMutationOutcome` — a second implementation of every leaf's `diff` including message codes, severities and **guard order** — plus `applyRemodelingMutation`, `commitReconstructionOutcome`, a Rust `DefaultHasher` (SipHash-1-3) port, `imageAssetChildHandle` and `durableRemodelingAsset`. |
| `🧪️tests/🟦️.ts` | **new** — the cross-language fixture oracle. |

**IO (`…/✳️any/🚪️io/`)**

- `🟦️.ts` — was `export {}`. Now the real barrel; also publishes `REMODELING_UNIMPLEMENTED_IO`, a machine-readable
  table of every hop this subset can only refuse and the reason each refusal carries.
- `📥️import/…/🔤️txt/🔖️utf-8/✳️any/🟦️.ts` — **new, a real `.semio` DSL reader** (lexer, parser, spec-driven binder,
  compact `ArtifactRef` lexeme `<id>!<kind>@<standard>/<subset>`, `#[dsl(unit)]` suffix stripping).
- `📤️export/…/🔣️json/…/🟦️.ts`, `📥️import/…/🔣️json/…/🟦️.ts` — thin, correctly-layered facades over the schema
  codec (the generic decoder moved out of the io leaf into the spec file so `🔺️diff`/`🧬️mutations` can use it
  without importing io — the old placement made schema depend on io).
- 12 leaves (`ply`/`las`/`obj`/`stl`/`gltf`/`png`, both directions) + `txt` export + 2 `dwg` leaves: honest stubs
  carrying the reason, mirroring W6's Rust intent (json `Exact`, txt `Exact`, the geometry/raster six `Lossy`
  refusals, `dwg` removed).

**Package / launch**

- `📦️packages/🟦️typescript/🧪️tests/🟦️.ts` — `include` extended with `🗿️artifacts/**/🧬️schema/🧪️tests/🟦️.ts`.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` — `🧪️test🏺️remodel📚️examples` →
  `bun nx run @semio-tech/remodel-js:test`, inserted after `🧪️test🔱️trinity📚️examples` in both files, same shape and
  group as the seven siblings. **No `🧪️test…🦀️rust`/describe entry was added**: `grep '🧪️test.*🦀️rust'` matches
  nothing in `launch.json`, so no plugin has that shape to follow.

## 2. Schema-first question, answered: the JSON Schema leaves are NOT authoritative

`🧬️schema/🔣️.json` (260 lines) declares `RemodelingArtifact`'s field list correctly but declares **seven of its
`$defs` as bare `{"type": "object", "title": "…"}` with no `properties`** — `MediaStream`, `ImageAsset`,
`CalibrationState`, `ReconstructionParams`, `GroundControlPoint`, `ReconstructionJob`, `ReconstructionResults` —
i.e. the JSON Schema carries the *same* escape hatch the old TS mirror did, and is its source. It also **omits
`durableArtifacts` entirely** from both `required` and `properties`, and types `assets` as
`additionalProperties: {$ref: ImageAsset}` when Rust has `BTreeMap<String, ArtifactChild<SemioImageSnapshot>>`.
So the JSON Schema is a stub, Rust is authoritative, and this packet derives from Rust
(`🗿️artifacts/📸️remodeling/🦀️.rs` §🔖️Domain, `🧬️schema/📸️snapshot/🦀️.rs`, `🔺️diff/🦀️.rs`, the 35 payload structs)
with the committed fixtures pinning the casing. **Regenerating those JSON Schema leaves is a real outstanding
item and belongs to whoever owns `describe`/schema generation** — it is not a TS file, so out of this scope.

## 3. Type-drift findings

| # | severity | finding |
|---|---|---|
| D1 | **HIGH** | **The committed fixtures predate `RemodelingSnapshot::durable_artifacts`.** All 68 snapshot JSONs (34 × before/after) and all 34 `🔺️diff/🔣️.json` omit the key. Both Rust structs carry it and neither has `skip_serializing_if`, so `serde_json::to_value(decoded)` re-emits `"durableArtifacts": {}` / `: null`. **This means the Rust test `committed_json_is_canonical` and `committed_diff_is_canonical` in all 34 `🧪️tests/*/🦀️.rs` must currently fail**, and `create-asset`'s `produces_committed_diff` must fail on the extra populated lane. Not a TS bug — a Rust-lane fixture regeneration. |
| D2 | **HIGH** | Following from D1: `create-asset`'s committed **after-snapshot** is semantically stale, not merely non-canonical. Rust's `create_asset` diff inserts `durable_artifacts[handle.child_id] = durable_remodeling_asset(payload.asset)`; the committed after has `assets` updated but no durable entry at all. TS reproduces the Rust behaviour and therefore disagrees. Asserted explicitly (see §5). |
| D2b | MEDIUM | The `commit-reconstruction` fixture dir `🧫️fixtures/🏁️commit-reconstruction/` ships `⬅️before.json` and `➡️after.json` but **not** the `🦠️commit-reconstruction-mutation.json` its own `🥒️.feature` (lines 54-58) names. The oracle test assembles that vector from committed sibling bytes exactly as the feature file describes rather than hardcoding one. |
| D9 | **HIGH** | **`🧫️fixtures/🏁️commit-reconstruction/{⬅️before,➡️after}.json` are stale on three counts** (found by the first oracle run, then confirmed directly). (a) They carry `job.stage` lexemes `"dense-reconstructing"` and `"completed"`, **neither of which is a `ReconstructionStage` variant** — `grep` finds neither string anywhere in `🗿️artifacts/📸️remodeling/🦀️.rs`, so Rust's own `serde_json::from_str::<RemodelingSnapshot>` rejects both files too. (b) `before != after` (they differ in `job` and `results`), although the `🥒️.feature` (lines 57-60) states the after-document *is* the before-document unchanged, because the documented answer is `mutation.invalid-reconstruction-sparse` and a refused commit must leave the scene untouched. (c) The `🦠️…-mutation.json` that same feature file names is not on disk. Committed `fe7c8a8f8b` 2026-09-05 03:53. |
| D3 | MEDIUM | Old `🧬️schema/🟦️.ts` and `🔺️diff/🟦️.ts` both typed `assets` as `Record<string, ImageAsset>`. Rust is `BTreeMap<String, ArtifactChild<SemioImageSnapshot>>` — a `{childId, target:{artifactId, dialect:{artifactKind, standard, subset}}}` handle, confirmed verbatim by every fixture. Fixed. |
| D4 | MEDIUM | Old `🧬️schema/🟦️.ts` and `🔺️diff/🟦️.ts` were both missing `durableArtifacts` outright (the diff mirror had 17 of 18 fields). Fixed. |
| D5 | LOW | Old `🧬️schema/🟦️.ts` typed `frameCursor.streamId?: string`. Rust is `Option<String>` with `#[serde(default)]` on the container and serialises as an explicit `null`, not an absent key. Now `string \| null`. |
| D6 | LOW | Old `🧬️mutations/🟦️.ts` listed 34 tags and stated `commitReconstruction` "has no TS triad leaf yet". All 35 are now typed; the 35th's *success* path is a declared boundary (§4), not an omission. |
| D7 | INFO | `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` also omits `durable-artifacts` and carries a mesh child id (`remodeling-mesh-901ccade3f60f8f1`) that is neither `remodeling-mesh-constant-empty` nor `-constant-box`, so it does not equal `default_remodeling_scene()`'s handle. Everything else in it matches the plugin's own defaults exactly (asserted). |
| D8 | INFO | Every one of the 170 committed fixture JSON floats is **exactly f32-representable**, so no float-lexeme ambiguity exists between the `f32` and `f64` writers on this corpus. The width distinction is still implemented, because it is real for any new vector. |

## 4. Declared boundaries (what is deliberately not implemented, and why)

1. **`commitReconstruction`'s success path.** A real commit publishes out of the plugin's *process-global*
   staging registries (`commit_staged_remodeling_reconstruction`, `durable_staged_remodeling_asset`,
   `staged_remodeling_mesh_chunk_count`) which no document-level twin can observe. All four of its refusal
   paths (`invalid-reconstruction-sparse`/`-asset`/`-mesh`, and the sparse one the feature file names as the
   only committed vector) are real; anything that would actually commit raises `RemodelingUnsupportedError`.
   The same applies to `createAsset`/`deleteAsset` invoked with a `__remodeling_asset_stage__:` /
   `__remodeling_mesh_stage__:` key — every non-staging branch of both is real.
2. **No DSL printer.** The txt *reader* is real; the *writer* needs `dsl::print`'s layout rules (block/table
   selection, column-header synthesis, unit re-suffixing, per-type number lexemes, the `_` placeholder policy),
   which live in `🧰️framework/…/🗣️dsl/` and have no TS twin anywhere in the repo.
3. **DSL reader grammar surface** — preamble, `key=value` scalars with unit suffixes, quoted text, blocks, map
   literals, and tables whose cells are scalars. A table cell that is itself a `TABLE`/`BLOCK`, and the `_`
   placeholder, **throw** rather than guess (no committed remodel asset exercises either: the demo's `streams`
   table, whose header declares `frames:TABLE source:BLOCK`, has zero rows).
4. **The twelve geometry/raster io leaves and `dwg`** stay `export {}` with the reason in the docstring.

## 5. The cross-language fixture oracle

`🧬️schema/🧪️tests/🟦️.ts`. Every case directory is **globbed off disk** (`readdirSync` over
`🧬️mutations/*/🧪️tests/*/`); no hash-suffixed name is transcribed, so W2b's in-flight renames cannot break it.

Per vector, nine assertions: total-validation decode of the committed quartet; TS apply == committed after;
TS-produced diff == committed diff; produced outcome status == committed `🎯️outcome`; committed diff carries
before → after; the produced diff writes exactly the lanes the committed diff writes; encode/decode identity on
both snapshots; the re-emitted bytes differ from the committed bytes *only* by `durableArtifacts`; and the
artifact ↔ snapshot round trip. Plus suite-level: coverage of every wire tag except `commitReconstruction`
(asserted as *exactly* that one), the commit-reconstruction refusal vector, and the two example assets.

D1/D2 are encoded as assertions, not tolerated: `DURABLE_ARTIFACT_DRIFT` is computed from disk as the
`create-asset` directory and the suite asserts that the set of vectors whose `durableArtifacts` disagrees is
*exactly* that list — so the drift cannot silently grow, and it disappears the moment the Rust lane regenerates
the fixtures.

**Independent-oracle note.** The strongest cross-language check here is `imageAssetChildHandle`: a from-scratch
TypeScript port of Rust's `std::collections::hash_map::DefaultHasher` (SipHash-1-3, keys `(0,0)`, with
`impl Hash for str`'s `0xff` terminator). It reproduces both committed content addresses —
`remodeling-asset-45070beb0101de64` (asset-a) and `remodeling-asset-75b20f8d69a86e9a` (asset-b) — bit-exactly,
which validates the Rust content-addressing derivation from a genuinely separate implementation.

## 6. Test output

`bun nx run @semio-tech/remodel-js:test` — **EXIT 0, 383/383 passing**, log at `🗑️generated/w3-ts-test.txt`.

```
 RUN  v4.1.10 /Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel

 Test Files  3 passed (3)
      Tests  383 passed (383)
   Duration  2.04s

 NX   Successfully ran target test for project @semio-tech/remodel-js
```

(Nx's "flaky task" banner is Nx noticing the *previous* run's two failures followed by a pass; the suite is
deterministic. The 383 are 34 vectors × 11 assertions = 374, plus 2 suite-level, 2 commit-reconstruction,
3 example-asset and the 2 pre-existing `📚️examples` file tests.)

### Re-run after W2b's fixture-directory rename — still 383/383

Between the two runs W2b renamed the two placeholder case dirs (`t038` → `⏱️shifts-stream-a-5b442c`,
`t039` → `🔳️doubles-the-c245d5`). The suite was re-run against the renamed tree with **no edit to the test
file** and stayed at 383/383, exit 0 — which is the point of globbing the case directories off disk instead of
transcribing their hash-suffixed names. The committed log is that second run.

### The first run's two failures were both real, and both are D9

Run 1 was 381/383. The two failures were `RemodelingCodecError: .job.stage: expected one of … got
"dense-reconstructing"` / `… got "completed"` raised by the decoder against the two
`🧫️fixtures/🏁️commit-reconstruction/` documents — i.e. the oracle found D9 on its first execution. Rather
than skipping those files, the suite now **asserts the staleness**: it pins both invalid stage lexemes, asserts
both documents refuse to decode, asserts `before != after`, and asserts the mutation file is absent. All four
assertions flip red the moment the pair is regenerated. The refusal vector itself is now assembled from a valid
committed sibling (`replace-job`'s before-document + `replace-sparse`'s sparse payload), exactly the
construction the feature file describes.

### The byte-level oracle is not vacuous

`remodelingSnapshotToJsonText` reproduces the committed bytes **exactly**, not merely the parsed structure —
independently verified outside vitest:

```
emitted lines: 346  committed lines: 345
byte-identical after dropping the durableArtifacts line: true
sample float lines: "syncOffsetMs": 12.5, | "syncOffsetMs": 0.0, | "fx": 1000.0, | "fx": 800.0,
                    | "minSharpness": 0.25 | "gsdM": 0.0625,
```

The one-line delta is exactly D1's `"durableArtifacts": {},`. This holds for all 68 snapshot files and for 33 of
the 34 diff files (`create-asset`'s diff writes a populated `durableArtifacts` lane, D2, and is compared
structurally instead).

## 7. Disagreements between TypeScript and the committed Rust fixtures

Exactly two, both traced to a Rust-lane fixture staleness and both asserted rather than papered over:

1. **`create-asset`** — TS applies the Rust leaf faithfully and produces
   `durableArtifacts["remodeling-asset-75b20f8d69a86e9a"] = { kind: "image", mime: "image/jpeg", width: 640,
   height: 480, chunks: ["ZnJhbWUtYg=="] }`. The committed after-document has no such entry. TS is right, the
   fixture is stale (D1/D2).
2. **`🧫️fixtures/🏁️commit-reconstruction/`** — both documents are undecodable by *either* language (D9).

Nothing else in the 34-vector corpus disagrees: every apply, every produced diff, every lane set, every outcome
status and every re-emitted byte matches.

## 8. Handover / not mine to fix

- **Rust lane (fixture regeneration):** regenerate all 68 snapshot and 34 diff JSONs with `durableArtifacts`, and
  rebuild `🧫️fixtures/🏁️commit-reconstruction/` (valid stages, `after == before`, plus the missing
  `🦠️…-mutation.json`). Until then the Rust `committed_json_is_canonical` / `committed_diff_is_canonical` tests
  in all 34 `🧪️tests/*/🦀️.rs` cannot pass. Both TS drift lists are computed from disk and will go empty on their
  own once this lands.
- **Schema-generation lane:** `🧬️schema/🔣️.json` (and the snapshot/diff/mutation `🔣️.json` siblings) still declare
  seven `$defs` as empty objects and omit `durableArtifacts`. §2.
- **W6 (io):** the twelve geometry/raster TS leaves and the two `dwg` leaves are honest stubs naming the reason;
  if the Rust `dwg` directories are deleted, delete the TS ones with them.

