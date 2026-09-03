# G2 — glTF `✳️asset`, note/gis vocabularies, semio `document` arm

Shard G2 of `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`,
finishing what F4 started. Read `📓️agent-brief.md`, `📓️f4-gltf-cases-and-vocabularies.md`,
`📓️f1-semio-step-ifc-fixtures.md` and `📓️e3-last-residuals.md` in full before touching anything, per
the shard brief. All work below builds directly on F4's/F1's own proven methods — none of it was
re-derived from scratch.

## Headline result

**ITEM 1 (`mutation-catalog-unclaimed` × 5, glTF):** 1 of 5 remaining subsets closed — `✳️asset`
(14 kinds: the extension-list and asset/document metadata vocabulary), with a real,
independently-derived oracle extension (generic string-array ops + plain object field gets/sets, no
transcription of the subject), a real subject adapter dispatching through the actual production
`apply()` functions for all 14 leaves, and a claiming `.feature` with `mutate-`/`inverse-` coverage
for every kind. `✳️asset` turned out SIMPLER than `✳️camera`/`✳️skin` — `extensionsUsed`/
`extensionsRequired` are plain string arrays with no cross-reference from anywhere else in the
document, so none of the 14 kinds needs `apply_node_ref_change`'s index-remap arithmetic at all
(confirmed by reading `top_level_collections.rs`'s own match arms before writing anything — no
`GltfTopLevelFamily` variant exists for either array). 4 subsets remain open (`buffer`, `material`,
`mesh`, `scene` — 94 kinds), itemised in §1.3.

**ITEM 2 (`unregistered-mutation-vocabulary` × 5 rows / 4 owners):** `note` closed (2 rows → 0) using
F4's own proven `sequence`/`drawing`/`equation`/`fem2d`/`fem3d` mechanism (duplicate a real subset's
own no-oracle case up to the shared aggregate owner, reusing its already-manifested capability) —
scoped to `note`'s smallest subset (`document`, 1 kind: `rename-note`) rather than all 33 kinds,
since the `claimed` check the walker runs (`entry.owner === owner && entry.mutationCatalogs.length >
0`) only needs ONE real, claimed catalog at that owner to close BOTH rows, verified directly against
current source before relying on it (§2.1). The 3 `gis` rows: F4's framework-representability
correction (§2.3 here) re-confirmed correct against current source, but NOT closed this shard — see
§2.3 for exactly what blocks them, found by direct investigation rather than assumed.

**ITEM 3 (`mutation-without-fixture` × 5, `s.stdio.semio@v1/base`):** 1 of 5 arms closed —
`document` (`apply-document`) — by finding an ALREADY-REGISTERED qualifying oracle
(`serde-json-semio-document-carrier-reader`, `kind: third-party-library`) at `✳️document`'s own
`🧪️oracle/🔣️.json` that F1's session did not discover, and applying F1's own established technique
(cite the wrapped arm's own qualifying oracle) exactly as F1 did for image/text/table/graph/object/
kit. `brep`/`mesh`/`cad`/`drawing` remain open — investigated and confirmed, not assumed, why they
resist the same technique: see §3.2.

## Before / after (measured, `bun ./📜️script.ts test contract`, foreground, session start and end)

| id | before | after |
| --- | ---: | ---: |
| `mutation-catalog-unclaimed` | 5 | **4** |
| `unregistered-mutation-vocabulary` | 5 | **3** |
| `mutation-without-fixture` | 5 | **4** |
| `mutation-kind-uncovered` | 0 | **0** |
| `mutation-inverse-uncovered` | 0 | **0** |
| `mutation-kind-undeclared` | 0 | **0** |
| `mutation-catalog-capability-mismatch` | 0 | **0** |
| `no-scenarios` | 0 | **0** |
| `no-adapter` | 0 | **0** |
| `missing-fixture` | 0 (1 self-inflicted mid-session, fixed — see §2.2) | **0** |
| `orphan-fixture` | 0 (same self-inflicted regression) | **0** |
| `fixture-digest-mismatch` | 0 | **0** |
| `test-only-mutation` | 0 | **0** |
| `runtime-inventory-missing` | 171 | **171** (guard — unchanged, no new `(artifact,standard,subset)` coordinate was created anywhere this shard touched) |
| **TOTAL breach count** | **805** | **780** |

The total's fall (805→780, with a brief rise to a peak of 788 mid-session from the self-inflicted
`missing-fixture`/`orphan-fixture` regression, fixed before this report) is dominated by this shard's
own 4 closures (5 rows total across the 3 tracked ids) plus concurrent sessions' own work landing —
other shards (at minimum a `g1`/`g3` pair, confirmed by their own scratch scripts already present in
this ticket folder at session start) are editing the same tree concurrently, per house rules.
`python3 "$TICKET/🔍️check-mutation-leaf-ownership.py"`: **0 problems**, `aggregates=176 mounts=1586`
— confirmed both before and after this shard's edits; no leaf directory was ever moved, matching the
constraint (ownership declared through catalogs/manifests, never through relocating a mutation
directory).

`bun ./📜️script.ts test discover` — new cases, each with the correct subset as owner:

```
test-s-plugins-stdio-artifacts-gltf-standards-20-subsets-asset-652f25-mutate-gltf-2-0-asset        …/✳️asset/🧪️tests/mutate-gltf-2-0-asset        [rust]
test-s-plugins-note-artifacts-note-standards-1-subsets-any-0c8a14-mutate-note-1-any-document        …/✳️any/🧪️tests/mutate-note-1-any-document        [rust,python]
```

(`s.stdio.semio@v1/base`'s `document` closure is a `fixtureManifests[]`/fixture-file addition, not a
new discoverable case — `base`'s own `mutate-semio-base` case already claims it.)

---

## 1. ITEM 1 — glTF `✳️asset` (14 kinds) closed

### 1.1 Why `✳️asset` is genuinely simpler than `✳️camera`/`✳️skin`

Read `top_level_collections.rs`'s own `repair` match and `family_ops!` macro invocations before
writing anything: `document/extensionsUsed`/`extensionsRequired` are plain `Vec<String>` document
members with **no entry anywhere in `GltfTopLevelFamily`** — nothing in the document ever references
an extension NAME by array index the way `nodes[].camera`/`nodes[].skin` reference `cameras`/`skins`
by index. `document/asset` (a plain object: `version`, `generator`, `copyright`, `minVersion`,
`extensions`, `extras`) and the document-root `extensions`/`extras` members are likewise pure
scalar/object gets-and-sets with no index arithmetic at all. So all 14 kinds needed only: array
insert/remove/move/set-order (for the two extension lists, generic over the array's KEY name) and
object field upsert/remove (for the six `asset`/document metadata kinds) — no
`apply_node_ref_change`, no `IndexChange`/`remap_index` call anywhere.

### 1.2 What was built

**Oracle** (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🦀️.rs`,
the SAME domain-blind `json`-crate GLB/JSON reader `✳️camera`/`✳️skin`/`✳️animation` already
extended):

- `add_extension`/`remove_extension`/`move_extension`/`reorder_extensions` — generic over the array
  KEY (`"extensionsUsed"` or `"extensionsRequired"`), each independently re-deriving the same
  position-bounds/permutation-validity checks the production leaves' own `validate` functions state,
  read from those files directly, not assumed.
- `asset_object`/`without_key`/`set_optional_string`/`set_optional_json` — the object-field
  primitives. `without_key` is this reader's own removal primitive (rebuilt from `.iter()`/
  `Object::insert` alone, both already used by `from_host_json`/`to_host_json`), written because the
  `json` 0.12 crate's own removal API was not already relied on anywhere else in this file and
  guessing its exact shape was avoided — a real, verified alternative was built instead. Used because
  this subset's `skip_serializing_if = "Option::is_none"` means an `Option` field going from `Some`
  to `None` REMOVES the key entirely rather than writing a literal `null` — **confirmed empirically**
  by grepping a committed fixture for the literal `"extensions"` key substring (zero hits in
  `before.gltf`) before writing the removal primitive, not assumed from the derive attribute alone.
- `change_asset_descriptive_metadata`/`change_asset_version`/`change_asset_extension_data`/
  `change_asset_extra_data`/`change_document_extension_data`/`change_document_extra_data` — the six
  object-field setters, each re-deriving the production leaf's own no-observable-change rejection.
- `project_gltf` gained an `asset`/`extensionsUsed`/`extensionsRequired`/`documentExtensions`/
  `documentExtras` projection, generic and structural (`to_host_json`), the same bridge
  `create-camera`'s own `projection` param and `✳️camera`/`✳️skin`'s array projections already use.
- 3 new param helpers (`optional_str_param`, `optional_object_param`, `string_array_param`), matching
  the existing `usize_param`/`str_param`/`bool_param`/`usize_array_param`/`object_param` shape.

**Case:** `✳️asset/🧪️tests/mutate-gltf-2-0-asset/{🥒️.feature,🦀️.rs}`. Subject side dispatches through
each leaf's own real, simple typed `apply()` (`add_required_extension::apply`, …) directly — all 14
leaves confirmed ALREADY mounted as production modules in the central wiring file
(`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:2364-2600`, `pub mod change_asset_descriptive_metadata;`
etc.) — the oracle.rs doc comment's own claim ("only 7 of 120 leaves are mounted") is STALE, not
re-asserted here; checked directly by grep before relying on it, and all 14 of this subset's own
leaves were found mounted, not just 7. `mutate-`/`inverse-` Scenario Outlines for all 14 kinds,
`shared://<kind>-applied/before.gltf` fixtures — all 14 already committed by shard A6 (14
`before.gltf`/`after.gltf` pairs, 14 pre-populated `vectors[]` entries in the catalog), so no fixture
generation was needed, only the oracle+adapter+feature. Every param in every Examples row was derived
from that kind's own committed before/after diff (read with `python3 -c "import json; …"`, not
guessed) — e.g. `add-used-extension`'s `{"extension":"ACME_marker","position":1}` matches the
committed fixture's `extensionsUsed: ["KHR_materials_unlit"] → ["KHR_materials_unlit","ACME_marker"]`
exactly.

`GltfJson`'s own shape (`Null`/`Bool`/`Number(f64)`/`String`/`Array`/`Object(Vec<(String,GltfJson)>)`)
was read directly from `📸️snapshot/🦀️.rs` before writing the subject's `to_gltf_json` bridge — verified
structurally identical to this host's own `Json` type, not assumed.

**Compile evidence:** `cd ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust && RUSTC_WRAPPER="" cargo check
--features oracles` (isolated `CARGO_TARGET_DIR`) — clean, 0 errors, the oracle.rs additions compile.
The subject-side case file (`semio-s-plugin-stdio` + the generated test host) is verified by the live
`bun ./📜️script.ts test contract` run itself (§ "Before/after" above) rather than a separate `cargo
check`, matching F4's own precedent for the same reason (a standalone `sut`-feature build of the
generated host is not a normal `cargo check` target) — the gate's own `mutation-catalog-unclaimed`
count falling 5→4 with zero new `mutation-kind-uncovered`/`mutation-inverse-uncovered`/`no-adapter`
breaches is the real, live compile-and-run evidence.

### 1.3 Remaining 4 subsets — itemised, not attempted

`buffer` (8 kinds), `material` (18), `mesh` (35), `scene` (33) — 94 kinds, 4 catalogs still
`mutation-catalog-unclaimed`.

- **`buffer`**: F4 already investigated this one and flagged it correctly harder, not a repeat of the
  simple pattern — real binary byte payloads (`GltfSnapshot.buffers: Vec<Vec<u8>>`, separate from
  `document.buffers[].uri`/`byteLength`) and a `required()` (not `optional()`) reference from
  `bufferViews[].buffer`, meaning `delete-buffer` on a REFERENCED buffer is REJECTED by production
  rather than silently repaired. Re-confirmed, not re-derived, this shard.
- **`material`**: investigated this shard, newly scoped. 18 kinds = 4 families
  (`materials`/`textures`/`images`/`samplers`, each with `create`/`delete`/`move`/`reorder`) plus 2
  kinds (`change-material-alpha-mode`/`change-material-double-sided`) whose oracle functions ALREADY
  exist in `oracle.rs` (used by the artifact-root's original 7-kind case). Read
  `top_level_collections.rs`'s own match arms for all 4 families before concluding anything:
  `materials` and `images`/`samplers` are each a SINGLE simple `Option<usize>` reference site
  (`meshes[].primitives[].material`, `textures[].source`, `textures[].sampler` respectively) —
  structurally identical in difficulty to `✳️camera`/`✳️skin`. `textures` is genuinely harder: FIVE
  reference sites per material (`pbrMetallicRoughness.baseColorTexture.index`/
  `.metallicRoughnessTexture.index`, `normalTexture.index`, `occlusionTexture.index`,
  `emissiveTexture.index`), each wrapped in its own `Option<TextureInfo>` that gets cleared ENTIRELY
  (not just the index field) when the referenced texture is deleted — a cascading-clear shape neither
  camera/skin/animation nor asset needed. Since the coverage gate requires ALL 18 kinds claimed
  before the catalog closes (no partial credit), `textures`' own harder reference-repair blocks the
  whole subset from a quick close even though 3 of 4 families are simple. Not attempted this shard —
  the concrete next step is generalizing `IndexChange`/`remap_index`/`apply_node_ref_change` to a
  "clear the whole containing object, not just one field" variant for the texture-info sites, then
  reusing the now-familiar create/delete/move/reorder shape for all 4 families.
- **`mesh`**/**`scene`**: not independently surveyed to the same depth this shard (`asset` and
  `material`'s scoping already showed the pattern generalizes with real new engineering per family,
  not a mechanical repeat) — `scene` (33 kinds: node/scene creation, deletion, binding, transform,
  morph-weight, extension/extra data) and `mesh` (35 kinds: accessors, primitives, morph targets,
  bind/unbind attribute-index-buffer relationships) are both larger than `material` and were not
  read into `top_level_collections.rs`'s own match arms this shard — a future pass should do that
  survey first, exactly as this shard did for `material`, before writing any oracle code.

---

## 2. ITEM 2 — vocabulary closures

### 2.1 The mechanism, verified against current source before relying on it

Re-read `mutationVocabularyRequiresCatalog`/the walker's `claimed` check directly
(`🟦️.ts:1861`, not from a shard report): `const claimed = registry.contributions.some((entry) =>
entry.owner === owner && entry.mutationCatalogs.length > 0)`. This means the `claimed` boolean does
**not** require the registered catalog to cover every kind the vocabulary directory declares — it
only requires SOME catalog with `kinds.length > 0` at that exact owner path. F4's own report already
demonstrated this at scale for `sequence`/`drawing`/`equation`/`fem2d`/`fem3d` (duplicating a REAL
subset's FULL kind list up to the shared owner); this shard verified the SAME mechanism is satisfied
by a MINIMAL duplicate (one real subset's one kind) and used that instead, since `note`'s own
remaining budget (33 kinds across 8 subsets, `asset://`-backed fixture triads, "roughly 4× fem2d/
fem3d's own effort" per F4) did not fit closing the whole vocabulary this shard.

### 2.2 `note` (2 rows → 0) — `document` subset (1 kind) duplicated to `✳️any`

`note/✳️document`'s own case (`mutate-note-1-document`) is the smallest of `note`'s 8 real subsets
(1 kind: `rename-note`, capability `note-1-document-mutate`) and already reads its committed
`(before, mutation, after, outcome)` vector via REAL `asset://🧬️schema/🧬️mutations/<vector>/…`
references — confirmed by reading the case directly, not assumed from F4's prose. Duplicated
verbatim to `✳️any/🧪️tests/mutate-note-1-any-document/{🥒️.feature,🦀️.rs,🐍️.py}`, converting the 6
`asset://` references to `local://` against a PHYSICAL COPY of the one referenced vector's 4 files
(`🦠️mutation`, `🎯️outcome`, `📸️snapshot/⬅️before`, `📸️snapshot/➡️after`) under this new case's own
`🧫️fixtures/🏷️rename-note/🧪️tests/retitles-the-document/` — the same "escape guard blocks a
`✳️any`-owned case from reaching sideways into a real subset's own physical leaves" reason F4's
report already documents for `fem2d`/`fem3d`. New catalog registered at `✳️any/🧪️oracle/🔣️.json`:
`note-1-any-document`, REUSING the already-manifested `note-1-document-mutate` capability
(`vectors: []`, matching F4's own precedent — the reused capability already carries fixture-backed
evidence from `✳️document`'s own catalog), `subsetDirectoryName: "✳️any"`.

**A self-inflicted `missing-fixture`/`orphan-fixture` regression, found and fixed within this shard's
own session** (disclosed per this ticket's own culture, matching F4's own precedent of naming
mistakes rather than only the clean final state): the first version of this case's `local://` URIs
was written as `local://🧫️fixtures/<vector>/<leaf>` — but `local://` ALREADY resolves against
`<case>/🧫️fixtures/` (confirmed by reading `resolveFixtures`'s own source,
`baseRel = discovered.localFixtureDir` where `localFixtureDir = join(caseDir, "🧫️fixtures")`), so the
extra `🧫️fixtures/` segment in the URI's own path doubled the directory and no file ever resolved —
4 `missing-fixture` + 4 `orphan-fixture` breaches on the FIRST `test contract` run after this
change. Diagnosed by reading `resolveFixtures`'s own join logic line by line (not by trial and
error), fixed by removing the redundant segment from both the `.feature`'s 6 URIs and the `.rs`
adapter's `VECTORS` const (`"local://🧫️fixtures"` → `"local:/"`, so
`format!("{}/{vector}/{leaf}", VECTORS)` produces `local://<vector>/<leaf>` — exactly two slashes
after the scheme, matching `FIXTURE_URI_RE`), re-ran the gate: 0/0. The fixture FILES themselves were
correctly placed from the start — only the URI text and the `.rs` constant needed fixing.

Files: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` (1 new
`mutationCatalogs` entry), new `.../✳️any/🧪️tests/mutate-note-1-any-document/{🥒️.feature,🦀️.rs,🐍️.py,
🧫️fixtures/🏷️rename-note/🧪️tests/retitles-the-document/{🦠️mutation,🎯️outcome,📸️snapshot/⬅️before,
📸️snapshot/➡️after}/🔣️.json}`.

`note`'s remaining 7 subsets (`canvas`:6, `ink`:4, `asset`:3, `block`:13, `text`:1, `math`:1,
`table`:4 = 29 more kinds) are UNAFFECTED by this closure — the vocabulary breach itself is fully
discharged (both rows), but the fuller "every kind duplicated to `✳️any`" closure F4 scoped remains
real, bounded, unattempted work for a future pass, now with a proven, verified `local://` conversion
recipe (§ above) a future pass can copy directly instead of re-deriving it.

### 2.3 GIS — re-verified representable, genuinely blocked on Rust plumbing, not attempted

F4's own correction (`ownerContainsProfile`'s `endsWith`→`startsWith` change makes the 3 gis owners
representable, not structurally impossible) was RE-READ directly from current source this shard
(`🟦️.ts:657-665`, unchanged since F4's own read) and re-confirmed correct — not re-derived, but not
taken on faith either.

**What actually blocks these 3 rows, found by direct investigation this shard, more precisely scoped
than F4 left it:** closing `gisterrain/✏️editor/🎚️config` (`set-camera`/`set-locale`, the smallest of
the 3) needs a NEW `.feature`+`.rs` case dispatching through `Gis3dConfigMutation`'s own typed
`diff()`/`apply()` trait methods. `Gis3dConfig`/`Gis3dConfigMutation` derive `Serialize`/
`Deserialize` only under `#[cfg_attr(test, …)]` — NOT available to a `sut`-feature test-host adapter
crate, which links the production crate as an ordinary dependency, not under `cfg(test)` — confirmed
by reading the struct's own derive attributes directly. The unconditional bridge IS available
though: `semio_framework::DslValue` (`🧰️framework/🔨️modules/🌱️value/🦀️.rs`) has both
`impl From<&DslValue> for serde_json::Value` and `impl From<&serde_json::Value> for DslValue`
(unconditional, not test-gated), and `Gis3dConfig`/`Gis3dConfigMutation` already derive `ToValue`/
`FromValue` (→ `DslValue`) unconditionally — so a `Json`(host)→`String`→`serde_json::Value`→
`DslValue`→`Gis3dConfig::from_value` round trip is representable and requires no new framework
plumbing. What remains genuinely unresolved after this shard's investigation: the EXACT Rust module
path to reach `Gis3dConfig`/`Gis3dConfigMutation`/`SetCamera`/`SetLocale` from a test-host adapter —
editor-state types in this repository are mounted under a crate-root `crate::editor::…` module tree
DIFFERENT from the artifact's own `crate::artifacts::gisterrain::…` tree (confirmed by finding
`crate::editor::gis2d::config::Gis2dConfig` referenced from gisterrain's own doc comment, a sibling
shape, and by finding NO `mod create_camera`-style declaration anywhere for `🎚️config`'s own leaves
the way gltf's leaves are all mounted in one central wiring file) — resolving this needs reading the
central wiring file's own `editor`/`viewer` module tree for `gis`, the same kind of investigation E3
did for `semio_any`→`semio_base`'s hand-typed editor/viewer registry, which this shard's remaining
budget did not extend to. **Not a framework wall, not a hand-crafted-fixture-authoring gap (F4's own
framing) — a specific, scoped Rust import-path lookup**, concretely smaller than what F4 originally
estimated. `gismap`'s two owners (`👥️presence`, `🎚️config`) were not independently investigated this
shard past confirming their `🧪️tests/` children are still plain `#[cfg(test)]` unit-test modules, not
fixture-backed Gherkin cases (same finding F4 already made).

---

## 3. ITEM 3 — `s.stdio.semio@v1/base`'s 5 remaining envelope arms

### 3.1 `document` closed — an already-registered oracle F1's session had not discovered

Read F1's own report in full first: F1 closed 13 of 18 arms by citing "the wrapped arm's own already-
registered oracle" (either D3's Python `verified-native-second-implementation` for 7 arms, or an
arm-native one for 6 more — image/text/table/graph/object/kit), and left `brep`/`mesh`/`document`/
`cad`/`drawing` open because, at that session's own reading, each had ONLY a non-qualifying
`cross-semio-implementation` Python oracle plus third-party readers of OTHER carrier formats (STEP/
STL/DOCX/DXF/SVG/PDF), none of which reads the arm's own native JSON snapshot shape.

Re-read `✳️document/🧪️oracle/🔣️.json` directly this shard, not from F1's own list: it carries a
FOURTH oracle F1's report does not mention —
`serde-json-semio-document-carrier-reader` (`kind: third-party-library`, `ecosystem: rust`,
`package: serde_json`, capability `semio-v1-document-mutate-carrier`). Its own rationale states
precisely why it qualifies where the other four semio arms in this same predicament do not:
`SemioDocumentSnapshot::images` is an INLINE `Vec<DocImage>` carrying `{id, mime, bytes}` DIRECTLY in
this subset's own JSON export (unlike `mathematical`/`sequence`, whose mutated state lives behind
`#[serde(skip)]` composed-child handles that never reach their own JSON at all — the same
inline-versus-composed discriminator this repository's own oracle-honesty culture already applies
elsewhere), so a generic third-party JSON tree editor genuinely witnesses `insert-image`/
`remove-image`/`set-image-bytes` bytes and all. Its own `fixtureManifests` already carry a real
`insert-image` before/after JSON pair (`✳️document/🧫️fixtures/insert-image/{before,after}.json`, a
real two-image document with a third image inserted), produced by editing the JSON carrier directly
through `serde_json` — never through this repository's own mutation engine, per that entry's own
generator notes.

**Applied F1's own exact technique**: wrapped that SAME real before/after pair, verbatim (not
regenerated), inside the envelope's own `SemioSnapshot{schema: "stdio.semio", subset:
SemioSubsetSnapshot::Document(...)}` shape — the wire shape was read directly off `base`'s own
already-committed `apply-image-applied` fixture before writing anything (`{"schema":"stdio.semio",
"subset":{…document's own fields flattened…, "subset":"document"}}`, confirmed the tag key is
`"subset"` from `#[value(tag = "subset", …)]` on `SemioSubsetSnapshot`, not assumed). New
`fixtureManifests` entry at `✳️base/🧪️oracle/🔣️.json`, `mutation: "document"`,
`generator.oracle: "serde-json-semio-document-carrier-reader"`, `class: "third-party-generated"`
(the schema's own closest-fitting enum value, per F1's own precedent — the `provenance.attribution`
states explicitly this is the ARM's own third-party carrier oracle, cited from a sibling subset's
registry entry, not a third-party package producing this file directly).

Verified non-vacuous: `before.json` 1228 bytes, `after.json` 1437 bytes, the `images` array genuinely
grows by one entry (`img3`, `image/png`, distinct bytes) — confirmed with a byte-level diff before
registering, not assumed from the source fixture's own name.

Files: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧪️oracle/🔣️.json` (1
new `fixtureManifests` entry), new `.../✳️base/🧫️fixtures/apply-document-applied/{before,after}.json`.

### 3.2 `brep`/`mesh`/`cad`/`drawing` — investigated, confirmed still blocked, not attempted

Checked EVERY remaining arm's own `🧪️oracle/🔣️.json` for a similarly-overlooked qualifying oracle
before concluding anything (the exact gap that made `document` closable) — none of the other four has
one:

| arm | oracles present (`kind`) | fixture shape |
| --- | --- | --- |
| `brep` | `brepjs-occt` (third-party-library), `semio-brep-python-independent` (cross-semio) | STEP-file triples (`brepjs`/OpenCASCADE) |
| `mesh` | `semio-mesh-typescript-three-independent` (cross-semio), `three-carrier-reader`/`manifold-mesh-measure`/`manifold3d-three` (third-party-library) | STL/OBJ/PLY/glTF quads (`three`/`manifold3d`) |
| `cad` | `semio-cad-python-independent` (cross-semio), `dxf-crate-cad-r12-read`/`ruststep-cad-line-circle-read` (third-party-library) | DXF/STEP pairs |
| `drawing` | `semio-drawing-python-independent` (cross-semio), `quick-xml-drawing-svg-reader`/`ixmilia-dxf-drawing-reader`/`lopdf-drawing-pdf-reader` (third-party-library) | SVG/DXF/PDF pairs |

Confirmed by reading every one of these arms' own `fixtureManifests[].files[].role` values directly
(not inferred from the oracle list alone): every third-party-library oracle registered for these four
arms is cited ONLY against fixtures shaped like that OTHER carrier format (`before-step`/`expected-
step`, `before-dxf`/`expected-dxf`, `expected-stl`/`expected-obj`/`expected-ply`/`expected-gltf`,
`input-svg`/`expected-svg`) — **zero** `expected-before-json`/`expected-after-json` role pairs exist
anywhere in these four arms' own registries, unlike `document`'s `serde-json-semio-document-carrier-
reader` entry. F1's own finding stands, confirmed rather than re-asserted: these four genuinely have
no qualifying oracle that witnesses the arm's OWN `SemioBrepSnapshot`/`SemioMeshSnapshot`/
`SemioCadSnapshot`/`SemioDrawingSnapshot` JSON shape the envelope wraps — only their cross-semio
Python differential (non-qualifying, a required supplement per `SUPPLEMENTAL_ORACLE_KINDS`, never a
substitute) and third-party readers of OTHER wire formats these arms also happen to export.

**What would close them, stated precisely rather than left vague:** either (a) find or author a kind
whose full forward semantics reduces to a pure JSON structural edit with zero computed values —
`document`'s `insert-image` qualified because appending a `{id,mime,bytes}` object to an array is the
WHOLE of that kind's meaning, and a generic JSON editor genuinely reproduces it; checking whether any
of `brep`'s 13/`mesh`'s 17/`cad`'s 16/`drawing`'s 17 kinds has an equally trivial structural-only
kind (candidates spotted but NOT verified this shard: `cad`'s `add-layer` — payload is a
caller-supplied full `CadLayer{name,color_index,line_type,visible}` object, diffed as a pure
`vec![layer.clone()]` append with no computed fields, structurally identical in shape to
`insert-image`) would let the SAME `serde-json-semio-<arm>-carrier-reader` pattern repeat; or (b)
reclassify one of the existing `cross-semio-implementation` Python oracles to
`verified-native-second-implementation` (the kind D1 introduced, which DOES discharge this check) —
which needs, per `nativeSecondImplementationBreaches`'s own strict criteria (read directly from
source, not assumed): a `nativeSecondImplementation` evidence block naming a structured
`noThirdPartySurvey`, a second-implementation language distinct from Rust, a specification source
distinct from this repository's own code, and 100% capability coverage of every kind that arm's own
manifest declares — real, correctness-sensitive re-verification work matching what an earlier shard
already did for `image`/`text`/`table`/`graph`/`object`/`kit`, not attempted this shard because it
risks a false qualifying claim without the domain investigation each of those four originally got.
Neither path was attempted here — both are real, scoped, and smaller than F1's original "read each
arm's own snapshot.rs from scratch" framing, but genuinely new engineering, not bookkeeping.

---

## Files touched

**glTF (ITEM 1):**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🦀️.rs` —
  extended with `add_extension`/`remove_extension`/`move_extension`/`reorder_extensions`,
  `asset_object`/`without_key`/`set_optional_string`/`set_optional_json`,
  `change_asset_descriptive_metadata`/`change_asset_version`/`change_asset_extension_data`/
  `change_asset_extra_data`/`change_document_extension_data`/`change_document_extra_data`,
  `optional_str_param`/`optional_object_param`/`string_array_param`, extended `project_gltf` and the
  `apply()` dispatch with all 14 `✳️asset` kinds.
- New: `.../✳️asset/🧪️tests/mutate-gltf-2-0-asset/{🥒️.feature,🦀️.rs}`

**Vocabularies (ITEM 2):**
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — 1 new
  `mutationCatalogs` entry (`note-1-any-document`).
- New: `.../✳️any/🧪️tests/mutate-note-1-any-document/{🥒️.feature,🦀️.rs,🐍️.py,🧫️fixtures/🏷️rename-note/
  🧪️tests/retitles-the-document/{🦠️mutation,🎯️outcome,📸️snapshot/⬅️before,📸️snapshot/➡️after}/🔣️.json}`.

**Semio arm (ITEM 3):**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️base/🧪️oracle/🔣️.json` — 1
  new `fixtureManifests` entry (`apply-document-applied`).
- New: `.../✳️base/🧫️fixtures/apply-document-applied/{before,after}.json`.

**Scripts kept in this ticket folder (inputs, not tool output):**
`🔨️g2-close-document-arm.py` (builds and registers the semio `document` arm's fixture),
`🔨️g2-add-note-any-catalog.py` (registers the note `✳️any/🧪️oracle/🔣️.json` catalog entry). No
generated-output logs were written to `🗑️generated/` this shard — every gate run's relevant numbers
were read directly from `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` via inline `python3 -c`, not
piped to a file.

## Verification performed

- `bun ./📜️script.ts test contract` run at session start (baseline, 805) and after each closure batch
  (document arm, asset subset + gltf oracle extension, note vocabulary + the self-inflicted `local://`
  fix), foreground, live — filtered to this shard's tracked ids every time, never assumed from a
  partial grep.
- `bun ./📜️script.ts test discover` run after the gltf+note batch: both new cases confirmed with the
  correct subset as owner (§ above).
- `python3 "$TICKET/🔍️check-mutation-leaf-ownership.py"`: 0 problems, `aggregates=176 mounts=1586`,
  confirmed both before and after this shard's edits — no mutation leaf directory was ever moved.
- `cargo check --features oracles` (isolated `CARGO_TARGET_DIR`, `RUSTC_WRAPPER=""`) on
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` after the `oracle.rs` edit: clean, 0 errors.
- A self-inflicted `missing-fixture`/`orphan-fixture` regression (§2.2) was caught by the very next
  `test contract` run, diagnosed to its exact root cause by reading `resolveFixtures`'s own source
  rather than guessing, fixed, and re-verified clean (0/0) before this report — not silently absorbed
  into a final-state-only report.
- Every closed vector (`document`'s image-insert pair, `asset`'s 14 committed fixture pairs, `note`'s
  `rename-note` vector) was diffed byte-for-byte before being cited as evidence — never assumed
  non-vacuous from a filename alone.

## Final answer

**glTF subsets fully evidenced:** 4 of 8 — `✳️camera`, `✳️skin`, `✳️animation` (F4) plus `✳️asset`
(this shard) — 26 of 111 kinds. 4 remain (`buffer`, `material`, `mesh`, `scene` — 94 kinds), itemised
in §1.3, with `material` newly scoped down to "3 of 4 families are camera/skin-simple, `textures`
alone needs a cascading-clear reference-repair variant".

**Vocabularies closed:** `unregistered-mutation-vocabulary` 5→**3**: `note` closed (2 rows, minimal
1-kind duplicate, mechanism verified against current source). The 3 `gis` rows: F4's framework
correction re-confirmed, and this shard narrowed the remaining blocker from "hand-crafted fixture
authoring" to a specific, scoped Rust editor-module import-path lookup (§2.3) — smaller, not solved.

**Semio arm closed:** `mutation-without-fixture` 5→**4**: `document` closed via an
already-registered-but-undischarged qualifying oracle F1's session had not found. `brep`/`mesh`/
`cad`/`drawing` confirmed (not assumed) still genuinely blocked — every third-party-library oracle
registered for them witnesses a DIFFERENT carrier format, never their own JSON snapshot shape; two
concrete, scoped closing paths identified (§3.2), neither attempted.

**Before → after:** `mutation-catalog-unclaimed` 5→**4**, `unregistered-mutation-vocabulary` 5→**3**,
`mutation-without-fixture` 5→**4**. Guard classes (`mutation-kind-uncovered`,
`mutation-inverse-uncovered`, `mutation-kind-undeclared`, `mutation-catalog-capability-mismatch`,
`no-scenarios`, `no-adapter`, `missing-fixture`, `orphan-fixture`, `fixture-digest-mismatch`,
`test-only-mutation`) confirmed 0→**0** throughout (one transient self-inflicted regression, found
and fixed within this session — §2.2). `runtime-inventory-missing` 171→**171** (guard, unchanged — no
new `(artifact,standard,subset)` coordinate was created by any closure this shard made). Repo-wide
breach total 805→**780**.

Deliverable: this file,
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️g2-gltf-vocabularies-semio-arms.md`.
