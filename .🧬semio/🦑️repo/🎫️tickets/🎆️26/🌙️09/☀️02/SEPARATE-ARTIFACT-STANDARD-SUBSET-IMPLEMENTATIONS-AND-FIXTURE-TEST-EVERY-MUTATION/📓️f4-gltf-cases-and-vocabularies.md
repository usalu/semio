# F4 — glTF subset cases and vocabulary closures

Shard F4 of `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
Read `📓️agent-brief.md`, `📓️e3-last-residuals.md`, `📓️a6-gltf-png-bmp-subsets.md`,
`📓️b3-subset-level-test-relocation.md` and `📓️b4-runtime-inventories.md` in full before touching
anything, per the shard brief.

## Headline result

**ITEM 1 (`mutation-catalog-unclaimed` × 8, glTF):** 3 of 8 subsets fully evidenced and closed —
`✳️camera` (4 kinds), `✳️skin` (4 kinds), `✳️animation` (4 kinds) — 12 kinds total, each with a real,
independently-derived oracle extension (not a transcription of the subject), a real subject adapter
dispatching through the actual production `apply()` functions, and a claiming `.feature` with
`mutate-`/`inverse-` coverage for every kind. 5 subsets remain open (`asset`, `buffer`, `material`,
`mesh`, `scene` — 79 kinds), itemised in §3.

**ITEM 2 (`unregistered-mutation-vocabulary` × 11 rows / 9 owners):** 4 of 9 owners closed —
`drawing`, `equation`, `fem2d`, `fem3d` — using E3's own proven `sequence` mechanism (duplicate the
real subset's case up to the shared aggregate owner, reusing its already-manifested capability).
`note` (33 kinds, largest) investigated and left open — same mechanism, real gap in remaining
budget, not a blocker. The 3 `gis` rows: **re-derived independently, and the "structurally
impossible" verdict A9/B4/C3/E3 all inherited does NOT hold against current source** — see §4. What
actually blocks them is real fixture-authoring (none exist), not framework impossibility.

## Before / after (measured, `bun ./📜️script.ts test contract`, foreground, session start and end)

| id | before | after |
| --- | ---: | ---: |
| `mutation-catalog-unclaimed` | 8 | **5** |
| `unregistered-mutation-vocabulary` | 11 | **5** |
| `mutation-inverse-uncovered` | 0 | **0** |
| `mutation-kind-uncovered` | 0 | **0** |
| `mutation-kind-undeclared` | 0 | **0** |
| `mutation-catalog-capability-mismatch` | 0 | **0** |
| `no-scenarios` | 0 | **0** |
| `no-adapter` | 0 | **0** |
| `missing-fixture` | 0 (1 self-inflicted mid-session, fixed — see §2.4) | **0** |
| `mutation-without-fixture` | 361 | **5** (all `s.stdio.semio@v1/base`, E3's/concurrent territory, not mine) |
| `runtime-inventory-missing` | 171 | **171** (guard — B4's own blocked territory, unchanged) |
| `test-only-mutation` | 0 | **0** |
| **TOTAL breach count** | **1186** | **818** |

The total's fall from 1186→818 is dominated by concurrent sessions' own work landing mid-session
(`mutation-without-fixture` alone fell 361→~5, far more than this shard's own paths could produce —
confirmed by scope: none of the closed rows outside gltf/drawing/equation/fem2d/fem3d are in this
shard's diff). This shard's own four tracked ids moved exactly as the table above states and nothing
else in its own diff regressed.

`python3 "$TICKET/🔍️check-mutation-leaf-ownership.py"`: 63 problems repo-wide, all pre-existing in
`🏛️architect/🏛️program`/`🌀️procedural3d` (matches E3's own baseline exactly), **zero** mentioning any
path this shard touched.

`bun ./📜️script.ts test discover`: all 20 new cases discovered, each with the correct subset as
owner:

```
test-s-plugins-stdio-artifacts-gltf-standards-20-subsets-camera-fb9978-mutate-gltf-2-0-camera        …/✳️camera/🧪️tests/mutate-gltf-2-0-camera        [rust]
test-s-plugins-stdio-artifacts-gltf-standards-20-subsets-skin-0c087e-mutate-gltf-2-0-skin            …/✳️skin/🧪️tests/mutate-gltf-2-0-skin            [rust]
test-s-plugins-stdio-artifacts-gltf-standards-20-subsets-animation-818c58-mutate-gltf-2-0-animation  …/✳️animation/🧪️tests/mutate-gltf-2-0-animation  [rust]
test-s-plugins-draw-artifacts-drawing-standards-1-subsets-any-…-mutate-drawing-1-any-metadata        …/✳️any/🧪️tests/mutate-drawing-1-any-metadata     [rust]
test-s-plugins-draw-artifacts-drawing-standards-1-subsets-any-…-mutate-drawing-1-any-structure       …/✳️any/🧪️tests/mutate-drawing-1-any-structure    [rust]
test-s-plugins-draw-artifacts-drawing-standards-1-subsets-any-…-mutate-drawing-1-any-style           …/✳️any/🧪️tests/mutate-drawing-1-any-style        [rust]
test-s-plugins-draw-artifacts-drawing-standards-1-subsets-any-…-mutate-drawing-1-any-transform       …/✳️any/🧪️tests/mutate-drawing-1-any-transform    [rust]
test-s-plugins-mathematical-artifacts-equation-standards-1-subsets-any-…-mutate-equation-1-any-graph      …/✳️any/🧪️tests/mutate-equation-1-any-graph      [rust]
test-s-plugins-mathematical-artifacts-equation-standards-1-subsets-any-…-mutate-equation-1-any-geometry   …/✳️any/🧪️tests/mutate-equation-1-any-geometry   [rust]
test-s-plugins-mathematical-artifacts-equation-standards-1-subsets-any-…-mutate-equation-1-any-equation   …/✳️any/🧪️tests/mutate-equation-1-any-equation   [rust]
test-s-plugins-fem-artifacts-2d-standards-1-subsets-any-…-mutate-fem2d-1-any-{mesh,material,boundary,load,analysis}  …/✳️any/🧪️tests/…  [rust,python]
test-s-plugins-fem-artifacts-3d-standards-1-subsets-any-…-mutate-fem3d-1-any-{mesh,material,boundary,load,analysis}  …/✳️any/🧪️tests/…  [rust,python]
```

---

## 1. ITEM 1 — glTF subset cases

### 1.0 The constraint, honoured throughout

`validate_mutation_leaf_source` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:649`)
requires a mutation leaf's own `owner` field to be an immediate child of its aggregate's mutation
root. Confirmed on disk before writing anything: every camera/skin/animation leaf directory
(`🌱️🎥️create-camera`, `🗑️🧥️delete-skin`, `🔀️🎞️reorder-animations`, …) is still physically at
`✳️any/🧬️schema/🧬️mutations/<leaf>/`, never moved — A6 only relocated fixtures and scaffolded a
claiming catalog per new subset, leaving the leaf directories themselves at `✳️any` exactly as this
constraint requires. This shard reaches every leaf's real `apply()` function by import from the
subset-owned case, never by moving the directory. `python3 "$TICKET/🔍️check-mutation-leaf-ownership.py"`
confirms 0 problems for gltf both before and after.

### 1.1 `✳️camera` (4 kinds) — closed

Catalog `gltf-2-0-camera` (`create-camera`, `delete-camera`, `move-camera`, `reorder-cameras`),
capability `gltf-2-0-mutate` — already scaffolded by A6, with 4 committed `before.gltf`/`after.gltf`
fixture pairs at `✳️camera/🧫️fixtures/<kind>-applied/`.

**Oracle work (the real gap A6/E3 left).** `✳️any/🧪️oracle/🦀️.rs` (the SAME domain-blind `json`-crate
GLB/JSON reader the artifact-root's 7-kind case already uses) is extended with 4 independently
reimplemented functions — `create_camera`/`delete_camera`/`move_camera`/`reorder_cameras` — that
re-derive, from the format's own rule (an index collection shrinks/grows/moves/permutes, every
reference to it tracks the same motion), the exact four-branch remap arithmetic
`✳️any/🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs`'s own `repair`/`family_ops!`
apply to every `nodes/{i}/camera` reference — verified against the 4 committed fixture pairs' own
before/after diffs (e.g. `create-camera`: position=1 insert, no existing ref bumped since none was
≥1; `delete-camera`: index=0 delete, the one referencing node's `camera` field becomes absent;
`move-camera`/`reorder-cameras`: index=1↔0 swap, referencing node's value follows). This reader never
calls into the leaf's own production code (`✳️any/🔨️modules/…/top_level_collections.rs`) — it is a
genuine second producer. `project_gltf` gained a `cameras`/`nodes[].camera` projection (a generic
`json::JsonValue`⇄host-`Json` structural bridge, `to_host_json`/`from_host_json`, reused so
`create-camera`'s own `projection` param round-trips through the same code the projector reads back).
6 new Rust unit tests added directly in `oracle.rs` (`create_and_delete_camera_round_trip`,
`delete_camera_clears_the_referencing_node`, `move_camera_is_its_own_inverse_with_swapped_arguments`,
`reorder_cameras_swap_is_self_inverse`, plus the existing suite unaffected).

**Case:** `✳️camera/🧪️tests/mutate-gltf-2-0-camera/{🥒️.feature,🦀️.rs}`. Subject side dispatches
through each leaf's own real, simple typed `apply()` (`create_camera::apply`, …) directly — no
descriptor-table indirection needed, unlike the artifact-root's older 7-kind style. `mutate-`/
`inverse-` Scenario Outlines for all 4 kinds, `shared://<kind>-applied/before.gltf` fixtures (case
owner = `✳️camera`, so `shared://` resolves against `✳️camera/🧫️fixtures/` directly — no fixture
duplication needed).

**Compile evidence:** `cd ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust && RUSTC_WRAPPER="" cargo
check --features oracles` — clean, 0 errors (isolated `CARGO_TARGET_DIR`). `cargo test --features
oracles --lib` for the WHOLE crate could not be run: a concurrent session's `pdf`/`xlsx` module-tree
migration (`lib.rs` shows `MM` — modified-in-both-index-and-worktree — mid-edit right now) breaks
`#[cfg(test)]`-only compilation repo-wide with 43 unrelated errors (`could not find base in
subsets`, missing `.tex`/`.xlsx` fixtures under paths this shard never touched). Not chased, per
house rules ("check whether it precedes your edits before blaming yourself... poll rather than
chase") — confirmed via `git status` that `📦️lib.rs` is mid-edit by another session, not by this
shard. `cargo check` (the production-feature build, unaffected by the test-only breakage) is the
verification A6 itself relied on for the same reason and is clean.

### 1.2 `✳️skin` (4 kinds) — closed

Catalog `gltf-2-0-skin` (`create-skin`, `delete-skin`, `move-skin`, `reorder-skins`), capability
`gltf-2-0-mutate`. Same shape as camera: `document/skins` is the other top-level family (besides
cameras) a bare node scalar field (`nodes[].skin`) points at, so the SAME `IndexChange`/
`remap_index`/`apply_node_ref_change` machinery introduced for camera was generalized (renamed from
camera-specific names) and reused, not duplicated.

**The one real complication, handled honestly, not routed around.** `create-skin`'s own production
payload (`GltfCreateSkinPayload { position: usize }`) carries NO field content — it can only ever
recreate an empty default skin. This means `delete-skin`'s inverse cannot be expressed as a second
`create-skin` call the way camera's content-bearing `create-camera` could invert `delete-camera` —
production dispatches `delete-skin`'s real inverse through `DeleteSkinMutation`'s own diff-based
`Restore` variant, which this domain-blind independent reader has no typed access to (and reaching
it from an external test-adapter crate would need a new `semio-framework-os-kernel` Cargo
dependency this ticket must not add itself — the same constraint A6's own oracle.rs doc comment
already names for a different case). Fixed with a bespoke `undo_delete_skin` on BOTH sides —
oracle: `undo_delete_skin(mutated, original)` splices `document/skins` and every `nodes[].skin`
reference straight off the ORIGINAL pre-mutation bytes; subject: the identical field-level restore
directly against the typed `GltfSnapshot`. Both mirror `undo_create_scene`'s own precedent in the
artifact-root's oracle (a bespoke non-catalog inverse for the one kind whose forward mutation isn't
exactly invertible through a sibling kind).

**Case:** `✳️skin/🧪️tests/mutate-gltf-2-0-skin/{🥒️.feature,🦀️.rs}`, same shape as camera. 4 new oracle
unit tests (`create_and_delete_skin_round_trip`, `delete_skin_clears_the_referencing_node`,
`move_skin_is_its_own_inverse_with_swapped_arguments`, `reorder_skins_swap_is_self_inverse`,
`undo_delete_skin_restores_the_original_content_not_an_empty_substitute`).

### 1.3 `✳️animation` (4 kinds) — closed

Catalog `gltf-2-0-animation` (`create-animation`, `delete-animation`, `move-animation`,
`reorder-animations`), capability `gltf-2-0-mutate`. Simpler than camera/skin:
`GltfTopLevelFamily::Animations => {}` is the ONE empty arm in `repair`'s own match — no node scalar
field, nor any other family, ever points AT an animation by index (only the reverse:
`animations[i].channels[j].target.node` points at a node) — so these 4 kinds need no
`apply_node_ref_change` step at all, confirmed by reading `top_level_collections.rs`'s own match
arms before writing anything. `project_gltf` gained an `animations`/`animationCount` projection (same
structural `to_host_json` bridge). `create-animation`'s payload also carries no content, so
`delete-animation`'s inverse gets the same bespoke `undo_delete_animation` treatment as skin's
`undo_delete_skin` — simpler here since there is no node reference to splice back, only
`document/animations` itself. 3 new oracle unit tests.

**Case:** `✳️animation/🧪️tests/mutate-gltf-2-0-animation/{🥒️.feature,🦀️.rs}`.

### 1.4 Remaining 5 subsets — itemised, not attempted

`asset` (14 kinds), `buffer` (8), `material` (18), `mesh` (35), `scene` (33) — 79 kinds, 5 catalogs
still `mutation-catalog-unclaimed`. Investigated `buffer` far enough to know precisely why it is a
step up in difficulty from camera/skin/animation, not a repeat of the same pattern:

- `document/buffers`/`document/bufferViews` carry an actual byte payload
  (`GltfSnapshot.buffers: Vec<Vec<u8>>`, separate from `document.buffers[].uri`/`byteLength`), unlike
  cameras/skins/animations which are pure JSON structure. `create-buffer`'s own payload
  (`GltfCreateBufferPayload { position, bytes: Vec<u8> }`) carries real binary content that the
  independent JSON-tree oracle would need to encode as a `data:` URI to stay comparable —
  mechanically doable but real new work, not a copy of the camera pattern.
- `document/bufferViews[].buffer` is a `required()` reference (`top_level_collections.rs`'s own
  `Buffers` arm), not `optional()` like `nodes[].camera`/`nodes[].skin` — deleting a REFERENCED
  buffer is REJECTED by production, not silently cleared. The committed `delete-buffer-applied`
  fixture confirms this empirically (a real 4468-byte referenced buffer[0] survives; an unreferenced
  4-byte `spareBuffer`[1] is the one actually deleted) — the oracle's own validation would need the
  same distinction, not just an index-remap.

`material`/`mesh`/`scene` were not independently surveyed this shard (buffer alone showed the pattern
generalizes with real new engineering per family, not a mechanical repeat) — a future pass should
expect each to need its own family-specific semantics read from `top_level_collections.rs`'s own
match arms before any oracle code is written, exactly as this shard did for cameras/skins/animations.

---

## 2. ITEM 2 — vocabulary closures

### 2.1 The mechanism (E3's own, re-applied, not re-derived)

`mutationVocabularyRequiresCatalog`'s owner is `dirname(dirname(vocabularyRel))`; the walker's actual
claim check (`🟦️.ts:1861`, re-read directly from source, not from a shard report) is: `const claimed
= registry.contributions.some((entry) => entry.owner === owner && entry.mutationCatalogs.length >
0)`. For drawing/equation/fem2d/fem3d, this owner is `✳️any` (the shared aggregate mutation root),
while the real per-kind leaves stay physically owned by each real subset (`✳️metadata`, `✳️graph`,
`✳️mesh`, …) — B3's own earlier split. The fix: duplicate each real subset's own no-oracle mutation
case up to `✳️any`, reusing that subset's ALREADY-manifested capability (so no new v2
`mutationManifests` entry, no new `(artifact, standard, subset)` runtime-inventory coordinate — the
exact multiplication A6 explicitly avoided for gltf, applied here by the same reasoning), and declare
a matching `mutationCatalogs` entry at `✳️any/🧪️oracle/🔣️.json`.

### 2.2 `drawing` (2 rows → 0)

4 real subsets (`metadata`:3, `structure`:4, `style`:4, `transform`:3 = 14 kinds, 4 distinct
capabilities), each already a self-contained no-oracle case reading its own committed
`(before, mutation, after)` vectors via `include_str!` (not `asset://` — the `drawing-mutation-semantics`
no-oracle decision's own text names exactly this as what blocks a third-party reference: "vectors are
not declared as `asset://` fixtures… a Python reference cannot read them at all"). Duplicated all 4
verbatim (script: `🔨️f4-duplicate-drawing-any-cases.py`) — only `include_str!` paths adjusted (one
extra `../✳️<subset>` segment to reach the sibling subset's physical leaf) and the `@mutations-`
tag/catalog id renamed `<x>-1-<subset>` → `<x>-1-any-<subset>`. New cases:
`✳️any/🧪️tests/mutate-drawing-1-any-{metadata,structure,style,transform}/`. 4 new catalog entries
added to `✳️any/🧪️oracle/🔣️.json` (script: `🔨️f4-add-drawing-any-catalogs.py`), `vectors: []` (E3's
own precedent — the reused capability already carries fixture-backed vectors from the original
subset's own catalog, so `mutation-without-fixture` is unaffected). All 42 `include_str!` references
verified resolving on disk (9/12/12/9 per subset) before running the gate. `noOracleDecisions[0].capabilities` at `✳️any` was already `[]` (narrowed by an earlier shard)
— no touch needed.

### 2.3 `equation` (2 rows → 0)

Confirmed on disk, independently of any prior shard's prose: the artifact directory has moved
`➗️mathematical` → `➗️equation` (matches E3's own mid-shard finding). 3 real subsets (`graph`:10,
`geometry`:4, `equation`:1 = 15 kinds, 3 capabilities), same `include_str!`-based no-oracle shape.
Duplicated all 3 (reused the generalized `🔨️f4-duplicate-any-cases.py`). New cases:
`✳️any/🧪️tests/mutate-equation-1-any-{graph,geometry,equation}/`. 3 new catalog entries (script:
`🔨️f4-add-equation-any-catalogs.py`). 66 `include_str!` references (43+18+5) verified resolving.

**One pre-existing, unrelated finding surfaced, disclosed rather than silently absorbed into the
count:** 9 `missing-external-oracle` breaches appeared at `✳️graph`/`✳️geometry`'s own
`🧪️oracle/🔣️.json` (`capability equation-1-mutate-uncarried … none is registered`) — confirmed NOT
caused by this shard: neither file was touched (`git status` shows no edits to either), and the
capability string (`equation-1-mutate-uncarried`) never appears in any file this shard wrote. Almost
certainly the SAME concurrent `mathematical`→`equation` rename E3 already flagged as in-flight,
landing mid-session.

### 2.4 `fem2d` / `fem3d` (1 row each → 0)

Structurally identical (per B3's own note), one shared capability across ALL 5 subsets each
(`fem2d-1-mutate`/`fem3d-1-mutate` — not per-subset, unlike drawing/equation), cross-language
Rust+Python differential, fixture TRIAD (not inline params) via a single shared derived-model
fixture (`🏗️timber-portal-frame.snapshot.json` / `🧊️steel-frame.snapshot.json`) plus per-kind
committed `(before, mutation, after, diff, outcome)` vectors reached through `include_str!`.

**One real complication found and handled, not glossed over.** Each subset's case carries a THIRD
scenario type beyond `mutate-`/`inverse-`: `@id-spec-vector`, replaying committed vectors via REAL
`asset://🧬️schema/🧬️mutations/<dir>/…` references — resolved against the case OWNER at runtime, not
`include_str!`. Moving the case to `✳️any` would break this: the escape guard blocks a `✳️any`-owned
case from reaching sideways into `✳️mesh`'s own physical leaves the way `asset://` requires. Since
the coverage gate (`mutationCoverageBreaches`) only requires `mutate-<kind>`/`inverse-<kind>`
scenario ids — not `spec-vector-<kind>` — the duplicated cases drop the `@id-spec-vector` Outline and
its one `subject::spec_vector` registration line entirely; the dropped Outline's own replay evidence
stays intact, undiminished, at the ORIGINAL subset-owned case (nothing deleted, nothing lost, just
not re-duplicated). The shared derived-model fixture is genuinely copied (not referenced) into each
new case's own `🧫️fixtures/`, matching B3's own precedent exactly.

**A self-inflicted `missing-fixture` bug, found and fixed within this shard's own session.** The
first version of the duplication script's own provenance doc-comment happened to contain the literal
substring `asset://-fixtured` inside a plain English sentence ("the extra `@id-spec-vector`
asset://-fixtured Outline dropped") — the walker's feature-text scanner matches `asset://` ANYWHERE
in the file, including prose, and tried to resolve `asset://-fixtured` as a real URI, producing 10
`missing-fixture` breaches (one per new case) on the FIRST `test contract` run after this change.
Diagnosed immediately (`grep asset:// <the feature file>` showed the exact self-authored sentence),
rephrased the script's own note text to avoid the literal scheme prefix, regenerated all 10 cases
from scratch, re-ran the gate: 0 `missing-fixture`. Disclosed here per this ticket's own culture of
naming mistakes rather than only the clean final state.

New cases: `✳️any/🧪️tests/mutate-{fem2d,fem3d}-1-any-{mesh,material,boundary,load,analysis}/`
(10 total, each with feature + Rust + Python + one copied fixture file). 2 new catalog files (5
catalog entries each, script: `🔨️f4-add-fem-any-catalogs.py`). All 10 Python files
`python3 -m py_compile`d — pass. All `include_str!` references verified resolving (5/15/15/35/55 per
subset × 2 artifacts = 250 checks, 0 missing).

### 2.5 `note` — investigated, left open

33 kinds, 8 real subsets (`document`:1, `canvas`:6, `ink`:4, `asset`:3, `block`:13, `text`:1,
`math`:1, `table`:4), 8 distinct capabilities. Same mechanism applies, but note's own mutate/inverse
scenarios use REAL `asset://🧬️schema/🧬️mutations/<vector>` references (confirmed by reading
`✳️document/🧪️tests/mutate-note-1-document/{🥒️.feature,🦀️.rs}` directly — `const VECTORS: &str =
"asset://🧬️schema/🧬️mutations"`), not `include_str!` like drawing/equation, and NOT a single shared
derived-model fixture like fem2d/fem3d — every one of the 33 kinds has its own physical
`(before, mutation, after, outcome)` fixture quad under its own subset's
`🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>/`. Closing this the honest way (matching B3's own
precedent for the ORIGINAL note split, and E3's own sizing) needs physically duplicating up to
33×4 = 132 fixture files into 8 new cases' own `🧫️fixtures/`, converting every `asset://` reference
to `local://` against the copy, across 8 capabilities — real, bounded, mechanical work, but roughly
4× fem2d/fem3d's own effort and did not fit this shard's remaining budget after the gltf oracle work
and drawing/equation/fem2d/fem3d closures above. Concretely scoped for a future pass; nothing about
it is blocked, only unattempted.

---

## 3. GIS re-verification — the inherited verdict does NOT hold

A9 → B4 → C3 → E3 all inherited the same claim: the 3 gis editor-state owners
(`🏔️gisterrain/…/✏️editor/🎚️config`, `🗺️gismap/…/✏️editor/👥️presence`,
`🗺️gismap/…/✏️editor/🎚️config`) are "structurally impossible to register" because
`mutationCatalogProblems`'s owner-profile check is anchored with `endsWith` at the real subset root,
which a nested owner (`…/✳️any/✏️editor/🎚️config`) can never satisfy. E3's own report is explicit
that this was re-confirmed, not re-derived: *"Confirmed this line is unchanged since A9's own read
(still at 🟦️.ts:658 with the identical `owner.includes(PROFILE_MARKER)`/`endsWith` logic)."*

**That is no longer what the source says.** Read directly, this session
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:657-665`):

```ts
function ownerContainsProfile(owner: string, standardDirectoryName: string, subsetDirectoryName: string): boolean {
  const markerIndex = owner.indexOf(PROFILE_MARKER);
  if (markerIndex < 0) return false;
  const profile = `${PROFILE_MARKER}${standardDirectoryName}/🪆️subsets/${subsetDirectoryName}`;
  const suffix = owner.slice(markerIndex);
  return suffix === profile || suffix.startsWith(`${profile}/`);
}
```

`git log -S "function ownerContainsProfile"` confirms this replaced a bare
`owner.endsWith(...)` check (visible in the same commit's own diff hunk). For
`🏔️gisterrain/…/✳️any/✏️editor/🎚️config` (owner path contains
`/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config`): with `standardDirectoryName: "🔖️1"`,
`subsetDirectoryName: "✳️any"`, `profile = "/🏅️standards/🔖️1/🪆️subsets/✳️any"`, and the owner's own
suffix literally `startsWith(profile + "/")` — **`ownerContainsProfile` returns `true`**. A catalog
declared there with `subsetDirectoryName: "✳️any"` passes `mutationCatalogProblems` cleanly. Further
confirmed `discoverTestContributions` (`🟦️.ts:875`) walks the WHOLE repo for any directory literally
named `🧪️oracle` containing a `🔣️.json`, with `owner = dirname(<that directory>)` — a NEW
contribution file physically at `.../✏️editor/🎚️config/🧪️oracle/🔣️.json` would be discovered with
`owner` exactly equal to the vocabulary's own owner, which is precisely what
`unregisteredMutationVocabulary`'s `claimed` check (`entry.owner === owner`) requires.

**So registration is representable.** What is genuinely NOT available, checked directly on disk for
all 3 owners before concluding anything: **no committed fixture vectors exist**. `gisterrain`'s
`🎥️set-camera`/`🗣️set-locale` leaves carry only `🔣️.json` (the leaf descriptor) and `🦀️.rs` — no
`🧪️tests/<case>/` subdirectory at all, unlike drawing's kinds which already had committed
`(before, mutation, after)` triples ready to `include_str!`. `gismap`'s two owners are the same:
their only `🧪️tests/` children are the `🧪️tests/🧬️direct-leaves` A9 itself renamed (plain Rust
`#[cfg(test)]` unit-test modules asserting internal invariants, not fixture-backed
before/mutation/after vectors a Gherkin case can replay).

**Concrete next-step, correctly scoped this time.** A real gis registration needs: (1) a NEW
`🧪️oracle/🔣️.json` contribution physically at each owner (catalog + a v2 `mutationManifests` entry —
`capabilityManifestBreaches` requires one for any NEW capability, confirmed by reading
`🟦️.ts:4980-5000` — plus an honest `noOracleDecisions` entry, mirroring the closely-analogous
`os.config.opening` precedent this session found already live at
`🧰️framework/🛍️products/💻️os/🎚️config/🧪️oracle/🔣️.json` — same shape: no third party implements
editor/UI ephemeral state, same `qualifyingKind: "third-party-library"` honest debt, same
`include_str!`-based no-oracle case), and (2) HAND-CRAFTED fixture vectors for `set-camera`/
`set-locale` (gisterrain, gismap-presence: 1-2 kinds) and gismap-config's 6 kinds
(`set-layer-visibility`, `set-vector-style`, `set-camera`, `set-lod-mode`, `set-render-mode`,
`set-layer-stroke-scale`, `set-locale`) — reading `Gis3dConfig`'s own snapshot schema first, the same
kind of from-scratch authoring A6 did for gltf's camera fixtures, not a duplication of existing
evidence. That second half is why this was not completed this shard: it is real, un-blocked,
scoped, tractable work — just not bookkeeping, and the fixture-authoring alone (not the now-proven-
representable registration) is what actually remains.

---

## Files touched

**glTF (ITEM 1):**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🦀️.rs` —
  extended with `create_camera`/`delete_camera`/`move_camera`/`reorder_cameras`,
  `create_skin`/`delete_skin`/`move_skin`/`reorder_skins`, `create_animation`/`delete_animation`/
  `move_animation`/`reorder_animations`, `undo_delete_skin`, `undo_delete_animation`,
  `IndexChange`/`remap_index`/`apply_node_ref_change` (generalized from camera-only naming),
  `to_host_json`/`from_host_json`, `object_param`/`usize_array_param`, extended `project_gltf` with
  `cameras`/`skins`/`animations`/`nodes[].camera`/`nodes[].skin`, 15 new `#[cfg(test)]` unit tests.
- New: `.../✳️camera/🧪️tests/mutate-gltf-2-0-camera/{🥒️.feature,🦀️.rs}`
- New: `.../✳️skin/🧪️tests/mutate-gltf-2-0-skin/{🥒️.feature,🦀️.rs}`
- New: `.../✳️animation/🧪️tests/mutate-gltf-2-0-animation/{🥒️.feature,🦀️.rs}`

**Vocabularies (ITEM 2):**
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — 4
  new `mutationCatalogs` entries.
- New: `.../✳️any/🧪️tests/mutate-drawing-1-any-{metadata,structure,style,transform}/{🥒️.feature,🦀️.rs}`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`
  — 3 new `mutationCatalogs` entries.
- New: `.../✳️any/🧪️tests/mutate-equation-1-any-{graph,geometry,equation}/{🥒️.feature,🦀️.rs}`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — 5 new
  `mutationCatalogs` entries.
- New: `.../✳️any/🧪️tests/mutate-fem2d-1-any-{mesh,material,boundary,load,analysis}/{🥒️.feature,🦀️.rs,🐍️.py,🧫️fixtures/🏗️timber-portal-frame.snapshot.json}`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — 5 new
  `mutationCatalogs` entries.
- New: `.../✳️any/🧪️tests/mutate-fem3d-1-any-{mesh,material,boundary,load,analysis}/{🥒️.feature,🦀️.rs,🐍️.py,🧫️fixtures/🧊️steel-frame.snapshot.json}`

**Scripts kept in this ticket folder (inputs, not tool output):**
`🔨️f4-duplicate-drawing-any-cases.py`, `🔨️f4-add-drawing-any-catalogs.py`,
`🔨️f4-duplicate-any-cases.py` (generalized, reused for equation), `🔨️f4-add-equation-any-catalogs.py`,
`🔨️f4-duplicate-fem-any-cases.py`, `🔨️f4-add-fem-any-catalogs.py`.

## Verification performed

- `bun ./📜️script.ts test contract` run at session start (baseline) and after every closure batch
  (camera, skin, animation, drawing, equation, fem2d+fem3d), each time reading the full breach dump
  and filtering to the paths just touched before moving on — never assumed from a partial grep.
- `bun ./📜️script.ts test discover` run after every batch; final run confirms all 20 new cases with
  the correct subset as owner (§ above).
- `python3 "$TICKET/🔍️check-mutation-leaf-ownership.py"` run at session end: 63 problems, all
  pre-existing in `🏛️architect`/`🌀️procedural3d`, 0 in any path this shard touched.
- `cargo check --features oracles` (isolated `CARGO_TARGET_DIR`, `RUSTC_WRAPPER=""`) on
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` after every gltf oracle.rs edit: clean, 0
  errors, every time.
- `python3 -m py_compile` on all 12 new/copied `🐍️.py` files (10 fem cases + 2 pre-existing
  round-trip files, confirmed unaffected): pass.
- Every `include_str!` reference in every new/duplicated `.rs` file verified resolving on disk with a
  standalone Python path-normalization check before running the gate (not left for the gate to
  discover) — 0 missing across all batches.
- A self-inflicted `missing-fixture` regression (§2.4) was caught by the very next `test contract`
  run, diagnosed to its exact cause, fixed, and re-verified clean — not silently absorbed into a
  final-state-only report.

## Final answer

**glTF subsets fully evidenced:** 3 of 8 — `✳️camera`, `✳️skin`, `✳️animation` (12 of 111 kinds).
5 remain (`asset`, `buffer`, `material`, `mesh`, `scene` — 79 kinds), itemised in §1.4.

**Vocabularies closed:** 4 of 9 owners — `drawing`, `equation`, `fem2d`, `fem3d` (6 of 11 rows).
`note` investigated and scoped, not attempted (§2.5). The 3 `gis` rows: verdict corrected from
"structurally impossible" to "representable but unauthored" (§3) — a materially different, smaller,
and now-precisely-scoped remainder.

**Before → after:** `mutation-catalog-unclaimed` 8→**5**, `unregistered-mutation-vocabulary`
11→**5**. Guard classes (`mutation-kind-uncovered`, `mutation-inverse-uncovered`,
`mutation-kind-undeclared`, `mutation-catalog-capability-mismatch`, `no-scenarios`, `no-adapter`,
`missing-fixture`, `test-only-mutation`) confirmed 0→**0** throughout. `mutation-without-fixture`
361→**5** (not this shard's own reduction — concurrent work, confirmed by scope). Repo-wide breach
total 1186→**818**.

Deliverable: this file,
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️f4-gltf-cases-and-vocabularies.md`.
