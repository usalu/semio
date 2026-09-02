# A6 — glTF 2.0 / PNG 1.2 / BMP v3 subset judgment

Territory: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🧊️gltf,📷️png,🖼️bmp}`. Breach class: `unsplit-artifact-subset`
(medium) / `wildcard-subset-owner` (high, escalation trap) under `testing/contract`.

## Before / after (measured via `bun ./📜️script.ts test contract` → `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`)

| artifact | unsplit-artifact-subset before | after | wildcard-subset-owner after | duplicate-mutation-owner after |
| --- | --- | --- | --- | --- |
| `s.stdio.gltf@2.0` | 120 | **0** | 0 | 0 |
| `s.stdio.png@1.2` | 15 | **0** | 0 | 0 |
| `s.stdio.bmp@v3` | 5 | **0** | 0 | 0 |

Repo-wide `unsplit-artifact-subset` went from the ticket's baseline down to 25 (other shards' territory,
not mine — my 140 are all gone). `wildcard-subset-owner` and `duplicate-mutation-owner` are 0 repo-wide,
i.e. no collateral damage anywhere else in the tree.

## Verdicts

- **`s.stdio.gltf@2.0` — genuinely splits.** The Khronos glTF 2.0 core spec defines exactly one
  conformance target (a "glTF Asset"), no ISO-style conformance classes the way PDF/A or STEP AP214's
  cc1–cc6 do. But the repo's 120 mutations are not one undifferentiated bag against that one target —
  they partition cleanly along the object taxonomy the spec's own `glTF.schema.json` already uses
  (separate `scene.schema.json`, `node.schema.json`, `mesh.schema.json`, `material.schema.json`, …),
  the same way this taxonomy's own `subsetPolicyIsSingle` doc comment treats `s.cad.cad`'s
  shape/building/energy/structure/drawing/node/reference split as the SHOULD-split case, not a
  conformance-class case. Split into 8 real domain subsets, one mutation category each (counts derived
  from the on-disk mutation directory names, not asserted):
  `scene`(33: scenes+nodes), `mesh`(35: meshes+primitives+morph-targets+accessors),
  `material`(18: materials+textures+samplers+images), `animation`(4), `skin`(4), `camera`(4),
  `buffer`(8: buffers+bufferViews), `asset`(14: asset metadata+document extension data+the
  extensionsUsed/Required mechanism itself). `✳️any` is kept as the shared whole-document substrate
  (io codec, third-party oracle, fixture generator, core schema/snapshot/diff/inference) — it owns no
  mutation any more.

- **`s.stdio.png@1.2` — genuinely single.** RFC 2083/the W3C PNG 1.2 recommendation has no PNG/A, no
  profile, no conformance class. Its critical/ancillary chunk split is a per-chunk optionality inside
  one always-co-decoded image, not an alternate whole-file profile. Recorded
  `"subsetPolicy": "single"` with that rationale.

- **`s.stdio.bmp@v3` — genuinely single.** The Windows BITMAPINFOHEADER-era format has no profile
  system either; its paletted-vs-direct-color/RLE variation is an internal encoding choice the one
  decoder branches on, not an independently-owned scope. Recorded `"subsetPolicy": "single"` with
  that rationale.

`📷️png`'s `✳️any` subset (the ticket's conforming layout exemplar) was not touched.

## What actually moved (glTF)

For each of the 120 mutations: its self-contained `🧬️schema/🧬️mutations/<name>/` leaf (payload
schema, contract, and its own embedded `🧪️tests/<case>/`, including the `.rs` test driver whose
`include_str!` calls are relative to the file and so needed no edits) and its subset-level
`🧫️fixtures/<name>-applied/{before,after}.gltf` pair moved from `✳️any` to the new domain subset's
own `🧬️schema/🧬️mutations/` and `🧫️fixtures/`. The 5-language mutation-vocabulary aggregate
(`🧬️mutations/{🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts}`, previously one 120-entry union) was split into 8
per-subset aggregates by the same mapping (`🔨️a6-split-gltf-aggregators.py`); relative `$ref`/import
paths needed no rewriting since each aggregate now sits at the same depth beside its own subset's
leaves. The Rust barrel (`🧬️mutations/🦀️.rs`, a `pub use super::X::Y` re-export list) was
deliberately left at `✳️any` untouched — see below.

Each moved mutation's own `🔣️.json` had its self-referential `"owner"` path field corrected
(`🔨️a6-move-gltf-mutations.py`). The oracle contribution's `fixtureManifests[].target.subset` and
`.files[].path` were corrected to the new fixture location; `mutationManifests[0].mutations[].subset`
got the real per-mutation override.

## What deliberately did NOT move, and why

- **`🚪️io`, `🧪️oracle`, `🏭️generator`, and the schema core (snapshot/diff/inference types).** These
  operate on the WHOLE glTF JSON document at once — decoding one document is atomic, so there is no
  honest way to give 8 domain subsets their own independent codec/oracle/generator without either
  duplicating the same whole-document logic 8 times (fake modularity) or building a real
  composition-registration layer (new `Dialect`/`SubsetId` entries in `semio_framework_plugin`, the
  way `📄️pdf/✳️a` wraps `✳️base`'s decode/encode with its own conformance-check layer) — genuinely
  Wave-2-scale work, and this ticket's own plan.md already reserves "push artifact-level shared logic
  into subsets" / "gltf's artifact-level inference services" for Wave 2, not this shard.
- **The Rust module tree stayed `subsets::any::schema::mutations::<name>`.** Only the 120 `#[path]`
  string literals in the shared stdio crate mount file
  (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`) were repaired to point at each mutation's new
  physical location (`🔨️a6-fix-rust-path-mounts.py`, one-for-one, each old path matched exactly
  once). `#[path]` exists precisely to decouple a module's Rust name from its file location; renaming
  the module tree too would have rippled into `engine::mutations`'s barrel and any future dispatch
  wiring for zero behavioural gain. The oracle crate mounts only the ONE combined `✳️any/🧪️oracle/🦀️.rs`
  file (confirmed by grep — no per-mutation mounts there), so it needed no changes at all.
- **`mutationManifests`/`fixtureManifests` stayed one combined record at `✳️any`, not split into 8
  physically separate contributions.** `compareInventories`'s runtime-inventory lookup is keyed by
  `manifest.subset` (the manifest's OWN field, not per-mutation): splitting into 8 manifests with
  `subset` set to each real domain would have required a runtime-inventory cache entry for 8 NEW
  `s.stdio.gltf@2.0/<domain>` coordinates that the production bridge cannot emit (no Dialect
  registered there — see above), multiplying the pre-existing `runtime-inventory-missing` breach
  1 → 8. Keeping ONE manifest at `✳️any` with per-mutation `subset` overrides gives every mutation a
  real, non-wildcard owner (which is all `owningSubsetOf`/`wildcard-subset-owner` require) without
  that multiplication.

## The one real, unresolved gap this surfaced (and how it was handled honestly)

Every `🧬️mutations` vocabulary directory needs SOME contribution, owned at that exact subset path,
declaring a `mutationCatalog` (`unregistered-mutation-vocabulary`) — and every declared catalog needs
`kinds` non-empty AND a `🥒️.feature` tagging `@mutations-<id>` (`mutation-catalog-unclaimed` /
`mutation-kind-uncovered`) or it's invalid. Only 7 of glTF's 120 mutations have ever had a real
Cucumber-level differential feature written for them (the artifact-root
`🧪️tests/mutate-gltf-2-0/🥒️.feature`, itself explicit about this: "only 7 are mounted as production
modules... this case covers exactly those 7, honestly smaller than the 120 that exist") — a
PRE-EXISTING gap, not something this shard introduced. Physically relocating the 113 uncovered
mutations' vocabulary out of `✳️any` unavoidably surfaces 8 new `mutation-catalog-unclaimed` findings
(one per new subset directory) — creating 8 real, matching feature-level tests per domain is
substantial new test-authoring, outside this shard's remit, and shared with 15+ other artifacts
already carrying the identical honest gap elsewhere in the repo (checked: 39 total repo-wide after
this change). The alternative — leaving the 8 new vocabulary directories with NO catalog at all — is
explicitly worse by the tool's own design comment ("A declared mutation vocabulary that no feature
claims is worse than an undeclared one" is the reasoning for why `unregistered-mutation-vocabulary`
exists in the first place; being *undeclared* is the state that reads as covered while measuring
nothing). So each new subset got a valid, honestly-labelled, currently-unclaimed catalog
(`🔨️a6-finish-gltf-oracle-manifest.py` + `🔨️a6-repair-gltf-catalogs.py`) instead — visible, not
hidden, and a natural pickup for whoever authors the Wave-2 feature tests.

The original `gltf-2-0-any` catalog (7 kinds, claimed by the untouched artifact-root feature) was
restored inside `✳️any`'s own contribution with `vectors: []` (its mutation directories moved away)
so the existing feature test keeps passing unmodified.

## Compile evidence

- `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — `cargo check`: **`Finished` dev profile**,
  clean (56 pre-existing dead-code warnings only, unrelated to gltf/png/bmp).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust` — `cargo check --target wasm32-wasip2`: fails, but on
  errors with **zero mentions of gltf/png/bmp** across three separate runs, in three DIFFERENT
  unrelated files each time (`⚛️reactor/💼️jobs` `IoPayload: Deserialize`, then
  `semio-framework-ui`/`IconName` `FromValue`/`ToValue` duplicate-definition) — confirmed via
  `git status` that none of those files are touched by this session, consistent with concurrent
  workspace churn from other live sessions (see project memory
  "Concurrent Cargo Workspace Churn"). No "file not found" / unresolved-module errors appeared for
  any of the 120 relocated `#[path]` mounts in any run.

## Scripts (kept in this ticket folder as permanent record)

`🔨️a6-build-subset-mapping.py`, `🔨️a6-move-gltf-mutations.py`, `🔨️a6-split-gltf-aggregators.py`,
`🔨️a6-fix-rust-path-mounts.py`, `🔨️a6-tag-gltf-manifest-subsets.py`,
`🔨️a6-finish-gltf-oracle-manifest.py`, `🔨️a6-repair-gltf-catalogs.py`. Mapping/audit data:
`🗑️generated/a6-gltf-subset-mapping.json`, `🗑️generated/a6-gltf-moved.json`.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/🔣️.json` — declared the 8 real
  subsets + `any` substrate.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — emptied
  `mutationCatalogs`→restored `gltf-2-0-any` only; per-mutation `subset` on all 120
  `mutationManifests[0].mutations[]`; `fixtureManifests[].target.subset` + `.files[].path` repaired.
- New: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️{scene,mesh,material,
  animation,skin,camera,buffer,asset}/🧬️schema/🧬️mutations/{120 mutation leaves + 4 aggregates}`,
  `.../🧫️fixtures/{120 fixture pairs}`, `.../🧪️oracle/🔣️.json` (8 new minimal contributions).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs` — 120 `#[path]` literals repaired.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/🔣️.json` — `subsetPolicy: "single"`
  + rationale.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/🔣️.json` — `subsetPolicy: "single"`
  + rationale.
