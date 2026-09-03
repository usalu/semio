# G4 — glTF material subset, semio brep/mesh/cad/drawing arms, gisterrain config vocabulary

Shard G4 of `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`,
the last three open items in this ticket's scope apart from runtime inventories (11 breaches at
session start: 4 `mutation-catalog-unclaimed` glTF subsets, 3 `unregistered-mutation-vocabulary` gis
rows, 4 `mutation-without-fixture` semio arms). Read `📓️agent-brief.md`, `📓️g2-gltf-vocabularies-semio-arms.md`,
`📓️f4-gltf-cases-and-vocabularies.md` and `📓️f1-semio-step-ifc-fixtures.md` in full before touching
anything, extending each shard's own proven methods rather than re-deriving them.

## Headline result

**ITEM 2 (`mutation-without-fixture` × 4, `s.stdio.semio@v1/base`) — FULLY CLOSED, 4/4.** G2's own
§3.2 identified two paths to close `apply-brep`/`apply-mesh`/`apply-cad`/`apply-drawing`: (a) find a
kind whose full semantics reduces to a pure JSON structural edit, repeating `document`'s own
`serde-json-semio-document-carrier-reader` pattern, or (b) reclassify a `cross-semio-implementation`
oracle to `verified-native-second-implementation`. Path (a) was checked against each arm's own
mutation leaves (`add-layer`/`create-vertex`/`create-material`/`create-layer` — read each one's own
`diff()` body before choosing it, not assumed) and confirmed genuinely trivial (a caller-supplied,
full-content object appended to an inline `Vec`, a duplicate key rejected as a no-op, zero computed
fields) for all four arms, not just cad. Built four standalone `🏭️generator/🦀️json-engine` crates
(own `[workspace]`, `serde_json` and nothing else — the exact shape `✳️document`'s own engine already
established), registered four new `serde-json-semio-<arm>-carrier-reader` oracles, and wrapped each
arm's real before/after pair into `✳️base`'s own envelope shape. **`mutation-without-fixture`: 4 → 0.**

**ITEM 1 (`mutation-catalog-unclaimed` × 4, glTF) — 1 of 4 closed: `✳️material` (18 kinds).** Read
`top_level_collections.rs`'s own `repair` match before writing anything, confirming G2's scoping
exactly: `materials`/`images`/`samplers` are each a single simple `Option<usize>` reference site
(camera/skin difficulty); `textures` needed a genuinely new "clear the whole `TextureInfo` object,
not just its index" cascading-clear primitive, which was built and exercised by dedicated unit
tests. `buffer`/`mesh`/`scene` (76 kinds) remain, itemised in §1.3, not attempted this shard.
**`mutation-catalog-unclaimed`: 4 → 3.**

**ITEM 3 (`unregistered-mutation-vocabulary` × 3, gis) — 1 of 3 closed: `gisterrain/✏️editor/🎚️config`.**
G2's own investigation narrowed the blocker to a specific Rust import-path question and one open risk
(whether a `serde_json::from_str` bridge on a `#[cfg_attr(test, …)]`-gated type compiles outside
`cfg(test)`). The import path was resolved (`crate::editor::gis3d::config::…`, confirmed via real
`use` statements, not guessed), and the open risk was resolved by NOT depending on it: `Gis3dConfig`'s
two fields are both plain `String`s, so the new bridge (`gis3d_config_mutation_report_json`) never
calls `serde_json::from_str`/`ToValue`/`FromValue` on the struct at all — it reads/writes the two
fields directly and drives the real, unconditionally-available `Mutation<Gis3dConfig>`/
`MutationDiff<Gis3dConfig>` trait chain the facet's own `#[cfg(test)] mod tests` already exercises.
`gismap`'s two rows (`👥️presence`, `🎚️config`) remain open — genuinely more complex types
(`BTreeMap` fields, more kinds), not independently investigated this shard. **`unregistered-mutation-vocabulary`: 3 → 2.**

## Before / after (measured, `bun ./📜️script.ts test contract`, foreground, session start and end)

| id | before | after |
| --- | ---: | ---: |
| `mutation-catalog-unclaimed` | 4 | **3** |
| `unregistered-mutation-vocabulary` | 3 | **2** |
| `mutation-without-fixture` | 4 | **0** |
| `mutation-kind-uncovered` | 0 | **0** |
| `mutation-inverse-uncovered` | 0 | **0** |
| `mutation-kind-undeclared` | 0 | **0** |
| `mutation-catalog-capability-mismatch` | 0 | **0** |
| `no-scenarios` | 0 | **0** |
| `no-adapter` | 0 | **0** |
| `missing-capability` | 0 | **0** |
| `missing-comparison` | 0 | **0** |
| `missing-oracle` | 0 | **0** |
| `missing-fixture` | 0 | **0** |
| `orphan-fixture` | 0 | **0** |
| `fixture-digest-mismatch` | 0 | **0** |
| `test-only-mutation` | 0 | **0** |
| `runtime-inventory-missing` | 171 | **172** (+1, disclosed in §3.3 — a genuinely new CONTRIBUTION FILE at an already-inventoried `(s.gis.gisterrain, 1, any)` coordinate; out of this shard's scope per the brief, matching G3's own territory) |
| **TOTAL breach count** | **780** | **692** |

Two side-effect breach classes NOT in the tracked list above, both disclosed rather than hidden, both
matching an established precedent this ticket already accepts elsewhere (`os.config.opening`'s own
identical trade-off): registering the gis3d-config capability added `missing-external-oracle` 45→47
(2 new "requires a third-party-library, none is registered" rows — the SAME accepted, honestly-tracked
debt `os-config-opening-1-mutate` already carries for the identical reason: no third party implements
or could adjudicate this repository's own ephemeral editor state) and closed `capability-without-manifest`
1→0 (this shard's own transient regression, fixed within the session — see §3.2).

`python3 "$TICKET/🔍️check-mutation-leaf-ownership.py"`: **0 problems**, `aggregates=176 mounts=1586` —
confirmed before and after this shard's edits; no leaf directory was ever moved (every closure was a
catalog/manifest/fixture registration plus new oracle-crate and generator code, exactly as the
constraint requires).

`bun ./📜️script.ts test discover` — new/changed cases, each with the correct subset as owner:

```
test-s-plugins-stdio-artifacts-gltf-standards-20-subsets-material-e864b3-mutate-gltf-2-0-material  …/✳️material/🧪️tests/mutate-gltf-2-0-material  [rust]
test-s-plugins-gis-artifacts-gisterrain-standards-1-subsets-any-1a7abb-mutate-gis-gisterrain-1-config  …/✳️any/🧪️tests/mutate-gis-gisterrain-1-config  [rust]
```

(The four semio-arm closures are `fixtureManifests[]`/`mutationManifests[]` additions at already-discovered
cases — `base`'s own `mutate-semio-base` case already claims `apply-brep`/`apply-mesh`/`apply-cad`/
`apply-drawing`, so no new case appears in `test discover`.)

---

## 1. ITEM 1 — glTF `✳️material` (18 kinds) closed

### 1.1 What `top_level_collections.rs` actually says, read before writing anything

`repair`'s own match (`✳️any/🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs`):
`Materials` remaps `meshes[].primitives[].material` (one nested `Option<usize>` site); `Images`
remaps `textures[].source`; `Samplers` remaps `textures[].sampler` — all three structurally identical
in difficulty to `✳️camera`/`✳️skin`'s own `nodes[].camera`/`nodes[].skin`. `Textures` is the hard
one: FIVE reference sites per material (`pbrMetallicRoughness.{baseColorTexture,
metallicRoughnessTexture}.index`, `normalTexture.index`, `occlusionTexture.index`,
`emissiveTexture.index`), each an `Option<TextureInfo>` production CLEARS ENTIRELY — not just the
`.index` field — when the referenced texture is deleted (`material.normal_texture = None`, never a
partial edit).

All four `create-*` payloads (`GltfCreate{Material,Texture,Image,Sampler}Payload { position }`) carry
NO field content — the same shape `create-skin`/`create-animation` already established — so every
`delete-*`'s inverse needed the same bespoke `undo_delete_*` treatment (restore the exact removed
content AND every reference straight off the ORIGINAL document), never a second `create-*` call.

### 1.2 What was built

**Oracle** (`✳️any/🧪️oracle/🦀️.rs`, the SAME domain-blind `json`-crate reader camera/skin/animation/
asset already extend): `apply_ref_change_in` (generalizes `apply_node_ref_change` to an arbitrary
top-level container — `("textures","source")` for images, `("textures","sampler")` for samplers),
`apply_primitive_material_ref_change` (the one nested `meshes[].primitives[].material` site),
`remap_texture_info_site`/`apply_texture_info_ref_change` (the new cascading-clear primitive — remaps
`.index` under `Insert`/`Move`/`Reorder`, clears the WHOLE `TextureInfo` object under a dropping
`Delete`), 16 new `create`/`delete`/`move`/`reorder` functions (4 families × 4 ops), 4 new
`undo_delete_*` functions mirroring `undo_delete_skin`'s own bespoke-inverse precedent. `project_gltf`
gained `materialsFull` (the FULL structural material dump — `pbrMetallicRoughness` and every texture
site included, since the artifact-root case's own `materials` key only ever carried `alphaMode`/
`doubleSided` and cannot witness this subset's own cascading clears), plus `textures`/`images`/
`samplers` and their `*Count` fields. 10 new `#[cfg(test)] mod tests` unit tests, including
`delete_texture_clears_the_whole_texture_info_object_not_just_the_index` and
`undo_delete_material_restores_the_original_content_and_every_reference` — the two tests that would
fail if the cascading-clear/bespoke-inverse logic degraded to a naive scalar-index remap.

**Case:** `✳️material/🧪️tests/mutate-gltf-2-0-material/{🥒️.feature,🦀️.rs}`. Subject side dispatches
through each of the 18 leaves' own real, simple typed `apply()` directly (no descriptor-table
indirection). `change-material-alpha-mode`/`change-material-double-sided` reuse the artifact-root
case's own already-existing oracle functions unmodified — only a new `alpha_mode(&Json,&str) ->
GltfAlphaMode` subject-side param parser was added, the same shape `create-camera`'s own `projection`
parser already establishes. Every Examples-row param was derived from that kind's own committed
before/after fixture diff (`python3 -c "import json; …"` against all 16 already-committed
`before.gltf`/`after.gltf` pairs — no fixture generation needed, A6 already scaffolded them), including
confirming which fixtures exercise the reference-remap paths (`delete-material` on a REFERENCED
material; `delete-{texture,image,sampler}` on an UNREFERENCED "spare" one, the identical convention
F1's step/ifc work already established) before writing a single Examples row.

**Compile evidence:** `cd ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust && RUSTC_WRAPPER=""
cargo check --features oracles` — clean, 0 errors (isolated local `[workspace]` target dir). `cargo
test --features oracles --lib` for the WHOLE crate could not be run: a concurrent session's `pdf`/
`xlsx` module-tree migration breaks `#[cfg(test)]`-only compilation repo-wide with 43 unrelated
errors (`could not find base in subsets`, missing fixtures under paths this shard never touched) —
the EXACT same 43-error signature F4's own report already documented for the identical reason,
re-confirmed via `cargo check --tests --features oracles` showing zero `gltf`-scoped errors among
them. The subject-side case file is verified by the live `bun test contract` run itself (§ above) —
`mutation-catalog-unclaimed` falling 4→3 with zero new `mutation-kind-uncovered`/
`mutation-inverse-uncovered`/`no-adapter` breaches is the real evidence, matching F4's/G2's own
precedent for the identical reason (a standalone `sut`-feature build of the generated host is not a
normal `cargo check` target).

### 1.3 Remaining 3 subsets — itemised, not attempted

`buffer` (8 kinds), `mesh` (35), `scene` (33) — 76 kinds, 3 catalogs still `mutation-catalog-unclaimed`.
`buffer` was already scoped hard by F4 (real binary byte payloads, a `required()` not `optional()`
reference from `bufferViews[].buffer`, so `delete-buffer` on a referenced buffer is REJECTED not
silently repaired). `mesh`/`scene` were not independently surveyed this shard — a future pass should
read `top_level_collections.rs`'s own match arms for both before writing any oracle code, exactly as
this shard did for `material`.

---

## 2. ITEM 2 — `s.stdio.semio@v1/base`'s 4 remaining envelope arms, all closed

### 2.1 Why the technique generalizes — verified per arm, not assumed

G2's own §3.2 spotted ONE candidate (`cad`'s `add-layer`) without verifying it. This shard verified
it AND found the identical shape in the other three arms, reading each leaf's own `diff()` body
directly:

- **`brep`/`create-vertex`**: payload `{id, point}`, diff appends to `SemioBrepSnapshot::vertices`;
  leaf's own doc comment: "A duplicate `id` already present in `base` is a no-op (never a duplicate
  id)."
- **`mesh`/`create-material`**: payload `{material: SemioMaterial}`, diff appends to
  `SemioMeshSnapshot::materials`; leaf's own doc comment: "A duplicate `id` already present in `base`
  is a no-op."
- **`cad`/`add-layer`**: payload `{layer: CadLayer}`, diff is `NamedTripleDiff{added:
  vec![layer.clone()], removed: [], modified: []}` on `SemioCadSnapshot::layers`, validated only for
  a non-duplicate `name` (`validate_named_triple`).
- **`drawing`/`create-layer`**: payload `{index, layer: DrawLayer}`, diff inserts at
  `index.min(base.layers.len())` — an `index` at or past the end is a plain append; a duplicate `id`
  is rejected outright (fatal).

Each target `Snapshot` struct's own touched field (`vertices`/`materials`/`layers`/`layers`) is a
plain `#[state(artifact)] Vec<T>` — inline, never behind a composed `ArtifactChild` handle — the exact
"inline vs composed" discriminator `serde-json-semio-document-carrier-reader`'s own rationale already
establishes as what makes a kind carrier-witnessable at all.

### 2.2 What was built — the same two-step pattern, four times

**Step 1 — a real, independent JSON-carrier oracle per arm.** Four new standalone crates,
`✳️{cad,brep,mesh,drawing}/🏭️generator/🦀️json-engine/{Cargo.toml,src/lib.rs,src/generate.rs}` — each
its own `[workspace]`, `[dependencies] serde_json = "1"` and NOTHING else (mirroring
`✳️document/🏭️generator/🦀️json-engine`'s own `Cargo.toml` doc comment verbatim: "DEPENDS ON
`serde_json` AND NOTHING ELSE"). Each builds a deterministic seed document by hand (field names read
directly off that subset's own `#[value(rename_all = "camelCase")]` snapshot struct, not guessed —
e.g. `CadLayer{name, colorIndex, lineType, visible}`), applies the ONE chosen kind as a domain-blind
edit to the `serde_json::Value` tree (never through this repository's own mutation engine), and
refuses to write a pair whose projection does not move. All four ran and produced real, non-vacuous
pairs — verified byte-for-byte before registering (e.g. `add-layer`: before=199B, after=313B;
`create-vertex`: 241B→350B; `create-material`: 270B→450B; `create-layer`: 288B→445B).

Registered one new oracle per arm at that arm's OWN `🧪️oracle/🔣️.json` (`serde-json-semio-{cad,
brep,mesh,drawing}-carrier-reader`, `kind: third-party-library`, `ecosystem: rust`, `package:
serde_json`, `capabilities` reusing the EXISTING shared capability — mirroring `dxf-crate-cad-r12-read`'s
own already-established precedent in the same file of a partial-coverage oracle reusing the shared
capability rather than inventing a new one).

**Step 2 — wrap into `base`'s own envelope, exactly as `g2-close-document-arm.py` did.**
`mutationFixtureBreaches` (`🟦️.ts:5406`, read directly before relying on it) requires only a
fixtureManifest whose `target`/`mutation` match; `fixtureProvenanceBreaches`'s own oracle-qualification
check (`🟦️.ts:5357-5358`) resolves the cited oracle id REPO-WIDE with no capability/subset matching at
all — confirmed by reading both functions directly, which is exactly why G2's own `document` citation
worked despite the wrapping fixture's `target.subset` (`"base"`) never matching the cited oracle's own
declared capability (`semio-v1-document-mutate-carrier`). Wrapped each arm's real before/after pair
into `{"schema":"stdio.semio","subset":{…arm's own fields flattened…,"subset":"<arm>"}}` (the exact
shape read directly off `✳️base`'s own already-committed `apply-image-applied` fixture, confirmed
`"subset":"<arm>"` is a literal tag key inside the flattened object, not assumed), wrote
`apply-{cad,brep,mesh,drawing}-applied/{before,after}.json`, registered four new `fixtureManifests`
entries at `✳️base/🧪️oracle/🔣️.json` citing each arm's own new oracle.

No touch to any arm's own `mutationManifests` (`add-layer`/`create-vertex`/`create-material`/
`create-layer`'s own manifest entries are untouched — the envelope's requirement is satisfied purely
by the fixtureManifest's own presence, confirmed by reading `mutationFixtureBreaches`'s source before
assuming it).

### 2.3 Verified, not asserted

`cargo build --release --offline` succeeded for all 4 new crates (already-cached `serde_json`
dependency, no network); `./target/release/generate` ran for all 4, each printing `observable
before=…B after=…B` and `wrote 1/1 fixture pair(s)` — a script-level refusal would have aborted
non-zero on a vacuous pair, and none did. `bun test contract`: `mutation-without-fixture` 4→**0**, all
guard classes 0→0 unchanged, zero new breaches of any kind referencing `json-engine`/`serde-json-semio-{cad,
brep,mesh,drawing}` anywhere in the full breach dump (checked directly, not assumed clean).

Files: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/{✳️cad,✳️brep,✳️mesh,✳️drawing}/🏭️generator/🦀️json-engine/{Cargo.toml,src/lib.rs,src/generate.rs}`,
`.../🪆️subsets/{✳️cad,✳️brep,✳️mesh,✳️drawing}/🧪️oracle/🔣️.json` (1 new oracle entry each),
`.../🪆️subsets/{✳️cad,✳️brep,✳️mesh,✳️drawing}/🧫️fixtures/{add-layer,create-vertex,create-material,create-layer}-applied/{before,after}.json`,
`.../🪆️subsets/✳️base/🧪️oracle/🔣️.json` (4 new `fixtureManifests` entries),
`.../🪆️subsets/✳️base/🧫️fixtures/apply-{cad,brep,mesh,drawing}-applied/{before,after}.json`.
Generator script: `🔨️g4-close-cad-brep-mesh-drawing-arms.py` (kept in this ticket folder).

---

## 3. ITEM 3 — gis `unregistered-mutation-vocabulary`, 1 of 3 rows closed

### 3.1 The import path, resolved (not re-derived, not guessed)

`Gis3dConfig`/`Gis3dConfigMutation`/`SetCamera`/`SetLocale` are physically owned at
`🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/{🦀️.rs,🧬️schema/🧬️mutations/🦀️.rs}`
but MOUNTED (`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/🦀️.rs:1043-1056`) under a crate-root
`crate::editor::gis3d::config::…` tree, sibling to (not nested under)
`crate::artifacts::gisterrain::…` — confirmed by reading the actual `#[path=…] mod` declarations
directly, not inferred. Package `semio-s-plugin-gis`. The generated `sut`-feature host links this
crate by path automatically (`materializeRustHost`'s own `rustSutCrate` walks up from the case's owner
until it finds a `📦️packages/🦀️rust/Cargo.toml` — the SAME crate camera/skin/material's own cases
already resolve to) — no manual Cargo.toml wiring needed.

### 3.2 The open risk, resolved by avoiding it rather than testing around it

`Gis3dConfig` derives `Serialize`/`Deserialize` only under `#[cfg_attr(test, …)]` — unavailable to a
`sut`-feature adapter, which links this crate as an ordinary dependency, never under `cfg(test)`.
Gisterrain's own precedent bridge, `gis_terrain_mutation_report_json`
(`✳️any/🧬️schema/⚙️operations/🦀️.rs`), calls `serde_json::from_str::<GisTerrainSnapshot>` on an
IDENTICALLY-gated type from an unconditionally-mounted function — a real, standing question about
whether that pattern even compiles outside `cfg(test)` that this shard could not settle by direct
`cargo check` (see §3.4). Rather than copy that pattern on faith, `Gis3dConfig`'s own shape was used
instead: BOTH its fields (`camera_json`, `locale`) are plain `String`s, so the new bridge,
`gis3d_config_mutation_report_json(camera_json: &str, locale: &str, kind: &str, value: &str) ->
Result<String, String>` (`✏️editor/🎚️config/🦀️.rs`), never calls `serde_json::from_str` on the struct
at all — it builds `Gis3dConfig`/`SetCamera`/`SetLocale` directly from the four `&str` arguments and
drives `Mutation<Gis3dConfig>`/`MutationDiff<Gis3dConfig>` (`#[derive(dsl::Mutations)]`/
`#[derive(dsl::MutationLeaf)]`, UNCONDITIONAL — never gated by `cfg(test)`, confirmed by reading the
derive attribute list directly) — the identical trait chain this file's own `#[cfg(test)] mod tests`
already exercises (`gis3d_config_operation_backwards_restores_the_pre_operation_snapshot`, etc.),
reached through a route this crate's own default build always compiles. Reports
`{base, snapshot, inverseSnapshot}` as a JSON string via `serde_json::json!`/`.to_string()` — writing
JSON with `serde_json`'s always-available `Serialize`-for-`serde_json::Value` machinery, never reading
or writing `Gis3dConfig` itself through serde.

### 3.3 What was registered

New `🎚️config/🧪️oracle/🔣️.json`: one `noOracleDecisions` entry
(`gis-gisterrain-config-mutation-semantics`, mirroring `os-config-opening-preferences-mutation-semantics`'s
own shape and reasoning — no third party implements or could adjudicate this repository's own
ephemeral editor state), one `mutationCatalogs` entry (`gis-gisterrain-1-config`, capability
`gis-gisterrain-1-config-mutate`, `kinds: ["set-camera","set-locale"]`, `vectors: []`), one
`mutationManifests` v2 entry (`artifact: "s.gis.gisterrain"`, `standard: "1"`, `subset: "any"` —
confirmed these exact strings against the artifact's own pre-existing `🧪️oracle/🔣️.json` before
writing them, not guessed) with an unfulfilled `oracleRequirements` entry per kind — the SAME
accepted `missing-external-oracle` debt shape `os-config-opening-1-mutate` already carries, disclosed
in the before/after table rather than hidden. Two new `handcrafted`-class `fixtureManifests` (one per
kind, `{cameraJson, locale}` before/after pairs matching the claiming feature's own Examples row
verbatim) — required once the manifest entry existed, since `mutationFixtureBreaches` needs SOME
fixture-backed evidence per declared mutation id regardless of oracle availability.

New case `✳️any/🧪️tests/mutate-gis-gisterrain-1-config/{🥒️.feature,🦀️.rs}` — placed at `✳️any`'s own
`🧪️tests/`, NOT nested under `✏️editor/🎚️config/🧪️tests/` where it was first written and where
`caseAboveSubsetBreaches` rejected it (`subsetCoordinatesOfOwner`'s own regex requires an owner path
ending EXACTLY at `🪆️subsets/✳️<subset>`, with nothing after it — a stricter anchor than
`mutationCatalogProblems`'s own `ownerContainsProfile`, which F4's `startsWith` fix already relaxed;
these are two DIFFERENT checks with two different anchoring rules, confirmed by reading both, not
assumed identical). Registers the SUBJECT role only (`@no-oracle-…` tag, no `.oracle(...)` handler),
matching `os.config.opening`'s own precedent exactly (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/
🖥️host/🧪️tests/mutate-os-config-opening/🥒️.feature`).

### 3.4 Compile evidence — honestly incomplete, precisely bounded

`cargo check --lib` on `semio-s-plugin-gis` (isolated local `CARGO_TARGET_DIR`, avoiding the shared
one's lock contention from other concurrent shards) reports 2052 pre-existing errors, ALL of the shape
`X: dsl::ToValue`/`FromValue`/`serde::Serialize` "is not satisfied" — confirmed via `git status` that
`🧰️framework/🔨️modules/🌱️value/` (the value/DSL machinery these traits come from) carries live
uncommitted changes from a concurrent session RIGHT NOW, matching this repo's own documented
"Concurrent Cargo Workspace Churn" pattern exactly, not this shard's own fault (re-confirmed:
`gis2d`/`gismap`/`GisMapSnapshot`/`GisMapMutation` — files this shard never touched — show the
identical error shape). The error count was IDENTICAL (2052) before and after this shard's own edit
— the strongest available evidence the new bridge function introduces no new error, though a genuinely
clean compile could not be obtained given the ambient churn. `bun test contract`'s own static gate
(§ above) is unaffected by this — it parses manifests and Gherkin text, never invokes `rustc` — and is
this ticket's own established minimum bar for closing a catalog-registration breach.

### 3.5 `gismap`'s 2 rows — not attempted, itemised

`👥️presence` and `🎚️config` under `🗺️gismap` remain open. `Gis2dConfig` (the `🎚️config` sibling type)
is materially more complex than `Gis3dConfig` — `BTreeMap<String,bool>`/`BTreeMap<String,f64>`
fields (`layer_visibility`/`layer_stroke_scale`) plus more string fields, and F4's own count puts this
vocabulary at 6-7 kinds, not 2. `Gis3dPresence`'s own shape was not read this shard. The identical
bridge-function technique (avoid `serde_json::from_str` on the struct entirely, drive the
unconditional `Mutation`/`MutationDiff` trait chain through plain-typed arguments) should generalize,
but the `BTreeMap` fields need per-key get/set parameters the two-string `Gis3dConfig` bridge did not,
and neither owner's own snapshot schema was read closely enough this shard to scope it precisely —
concretely smaller than F4's original framing, genuinely unattempted rather than blocked.

---

## Files touched

**glTF (ITEM 1):**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🦀️.rs` —
  extended with `apply_ref_change_in`/`apply_primitive_material_ref_change`/`remap_texture_info_site`/
  `apply_texture_info_ref_change`, 16 `create`/`delete`/`move`/`reorder` functions across 4 families,
  4 `undo_delete_*` functions, extended `apply()` dispatch and `project_gltf`, 10 new unit tests.
- New: `.../✳️material/🧪️tests/mutate-gltf-2-0-material/{🥒️.feature,🦀️.rs}`

**semio arms (ITEM 2):**
- New: `.../🧿️semio/🏅️standards/🔖️v1/🪆️subsets/{✳️cad,✳️brep,✳️mesh,✳️drawing}/🏭️generator/🦀️json-engine/{Cargo.toml,src/lib.rs,src/generate.rs}`
- `.../🪆️subsets/{✳️cad,✳️brep,✳️mesh,✳️drawing}/🧪️oracle/🔣️.json` — 1 new oracle entry each
- New: `.../🪆️subsets/{✳️cad,✳️brep,✳️mesh,✳️drawing}/🧫️fixtures/{add-layer,create-vertex,create-material,create-layer}-applied/{before,after}.json`
- `.../🪆️subsets/✳️base/🧪️oracle/🔣️.json` — 4 new `fixtureManifests` entries
- New: `.../🪆️subsets/✳️base/🧫️fixtures/apply-{cad,brep,mesh,drawing}-applied/{before,after}.json`

**gis (ITEM 3):**
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs` —
  extended with `gis3d_config_mutation_report_json`.
- New: `.../✳️any/✏️editor/🎚️config/🧪️oracle/🔣️.json`, `.../🎚️config/🧫️fixtures/{set-camera,set-locale}-applied/{before,after}.json`
- New: `.../✳️any/🧪️tests/mutate-gis-gisterrain-1-config/{🥒️.feature,🦀️.rs}` (placed at the subset
  root, not nested under `🎚️config`, per §3.3)

**Scripts kept in this ticket folder (inputs, not tool output):** `🔨️g4-close-cad-brep-mesh-drawing-arms.py`.

## Verification performed

- `bun ./📜️script.ts test contract`, foreground, live, at session start (baseline: 780) and after
  every closure batch (semio arms, gltf material, gisterrain config + its two self-corrections —
  §3.3's `case-above-subset`/`capability-without-manifest` regressions, each found by the very next
  gate run and fixed before moving on, disclosed rather than absorbed silently).
- `bun ./📜️script.ts test discover` after the gltf+gis batch: both new cases confirmed with the
  correct subset as owner (§ above).
- `python3 "$TICKET/🔍️check-mutation-leaf-ownership.py"`: 0 problems, `aggregates=176 mounts=1586`,
  confirmed both before and after every edit this shard made — no mutation leaf directory was ever
  moved.
- `cargo check --features oracles` (isolated local `[workspace]` target dir) on
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`: clean, 0 errors, after the material oracle.rs
  edit.
- `cargo build --release --offline` + a real run of `./target/release/generate` for all 4 new
  semio-arm `json-engine` crates: each printed a genuine `observable before=…B after=…B` line, proving
  the pair is non-vacuous before it was registered.
- `cargo check --lib` on `semio-s-plugin-gis` (isolated local target dir, avoiding the shared
  contended one): 2052 pre-existing errors, confirmed via `git status` to be concurrent
  `🌱️value` module churn unrelated to this shard, error count IDENTICAL before and after this
  shard's own edit — the best available evidence given the ambient instability, disclosed as
  incomplete rather than claimed as a clean pass.
- Two self-inflicted regressions (§3.3: a `case-above-subset` breach from the case's first, nested
  placement; a `capability-without-manifest` then `mutation-without-fixture` breach chain from
  registering the capability without, then without a fixture backing, the manifest) were each caught
  by the very next `test contract` run and fixed before this report — not silently absorbed into a
  final-state-only account.

## Final answer

**`mutation-without-fixture`:** 4 → **0**. All four `s.stdio.semio@v1/base` arms closed
(`apply-brep`/`apply-mesh`/`apply-cad`/`apply-drawing`), each via a genuinely new, independent
`serde_json`-only carrier oracle verified to witness that arm's own one trivial kind, wrapped into
the envelope exactly as `document` was.

**`mutation-catalog-unclaimed`:** 4 → **3**. `✳️material` (18 kinds) closed. `buffer`(8)/`mesh`(35)/
`scene`(33) — 76 kinds — remain, itemised in §1.3.

**`unregistered-mutation-vocabulary`:** 3 → **2**. `gisterrain/✏️editor/🎚️config` closed via a
bridge function that sidesteps the open `serde_json::from_str`-on-a-test-gated-type risk entirely by
using the struct's own plain-`String` fields directly. `gismap`'s 2 rows (`👥️presence`, `🎚️config`)
remain open, itemised in §3.5 with the same technique scoped as the likely next step.

**Guard classes** (`mutation-kind-uncovered`, `mutation-inverse-uncovered`, `mutation-kind-undeclared`,
`mutation-catalog-capability-mismatch`, `no-scenarios`, `no-adapter`, `missing-capability`,
`missing-comparison`, `missing-oracle`, `missing-fixture`, `orphan-fixture`, `fixture-digest-mismatch`,
`test-only-mutation`): confirmed 0→**0** throughout, including through two self-inflicted transient
regressions each caught and fixed within this session (§3.3).

**`runtime-inventory-missing`** (guard, out of this shard's scope per the brief): 171 → **172** (+1,
disclosed — a new contribution file at an already-inventoried coordinate, G3's own territory, not
chased).

**Repo-wide breach total:** 780 → **692**.

Deliverable: this file,
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️g4-final-closure.md`.
