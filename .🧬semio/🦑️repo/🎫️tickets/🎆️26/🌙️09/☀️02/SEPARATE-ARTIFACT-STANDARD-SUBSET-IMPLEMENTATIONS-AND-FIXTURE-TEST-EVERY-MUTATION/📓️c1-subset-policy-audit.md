# C1 — `subsetPolicy: "single"` audit

Shard C1's remit: shard B5 applied the `unsplit-artifact-subset` escape hatch
(`"subsetPolicy": "single"` in `🪆️subsets/🔣️.json`) to 65 artifact/standard pairs in one bulk pass.
Some are genuinely single-scope; others paper over a real, evidenced split. This file audits all 68
`🔣️.json` files currently carrying `subsetPolicy` (65 from B5's pass + 3 from later shards), applying
the deciding question per artifact: does its mutation vocabulary address ONE indivisible scope, or
several separable ones — using, in order, (a) the published standard's own conformance classes where
one exists, (b) what the mutations actually do against the snapshot schema, (c) what the repository
itself already says.

## Headline

**6 of the 68 were wrong and are now split — `s.cad.cad@1` and `s.stdio.ifc@4` (the two the coordinator
named explicitly) plus four more `s.norm.*` artifacts this audit's own evidence trail turned up
(`en1990`, `din4108`, `iso16757`, `vdi3805`). The other 62 were verified genuinely single-scope and left
untouched.**

| verdict | count |
| --- | --- |
| SINGLE-JUSTIFIED (left as-is) | 62 |
| SPLIT (executed this shard) | 6 — `s.cad.cad@1`, `s.stdio.ifc@4`, `s.norm.en1990@1`, `s.norm.din4108@1`, `s.norm.iso16757@1`, `s.norm.vdi3805@1` |

## Gate — before/after (`bun ./📜️script.ts test contract`, full foreground runs)

"Before" is the live breach snapshot immediately preceding this shard's edits (state left by shard B5
and the rest of wave 1/2; captured from `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`, 3 minutes old
at the time this shard started — not the ticket's original baseline, which is long since superseded).
"After" is the LAST of four full foreground gate runs taken while iterating (see §4, "the race", for why
more than one was needed), re-read from the same cache file each time.

| class | before | after | net |
| --- | --- | --- | --- |
| `unsplit-artifact-subset` | 0 | 0 | 0 |
| `wildcard-subset-owner` | 0 | 0 | 0 |
| `duplicate-mutation-owner` | 0 | 0 | 0 |
| `test-only-mutation` | 51 | 12 | −39 |
| `mutation-catalog-unclaimed` | 8 | 8 | 0 |
| `capability-without-manifest` | 6 | 6 | 0 |
| **TOTAL (repo-wide, all classes)** | **2034** | **1993** | **−41** |

`unsplit-artifact-subset`/`wildcard-subset-owner`/`duplicate-mutation-owner` are byte-for-byte unchanged
at 0 — every one of the six real splits executed below went in without opening a single new breach in ANY
of these three classes (verified per-artifact in §5), and they were already at 0 before this shard started
(shard B5 silenced `unsplit-artifact-subset` for everything by declaring `subsetPolicy: "single"`
everywhere; this shard's job was to check whether that declaration was honest, not to move the counter).
`test-only-mutation` fell 51 → 12 and TOTAL fell 2034 → 1993 between the first and last gate run of this
session — neither move is this shard's doing: the only `test-only-mutation` row touching any of the six
artifacts this shard split, at either end, is `s.norm.en1990@1/any: the test catalog claims mutation
change-annex, which no manifest owns` — a pre-existing v1-catalog/production spelling mismatch unrelated
to subsetting (this shard did not touch `change-annex` or the test catalog), present unchanged before and
after. (An `ifc@4`/`ifc@2x3` `no-mutation` cluster was present mid-session — see §Gate's earlier
intermediate run — and gone by the final run: fixed by a concurrent shard's v1-catalog work, not this
one's.) Other live sessions in this shared tree are concurrently fixing `test-only-mutation` and other
unrelated classes; this shard's own contribution is fully captured by the three subsetting classes staying
flat at 0, which is the honest signal for THIS shard's scope.

## 1. The two artifacts the coordinator named explicitly

### `s.cad.cad@1` — SPLIT (20 mutations → 8 subsets)

The framework's own source comment, immediately above the `unsplit-artifact-subset` breach definition
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:4666`), names this exact
artifact as the worked example of one that has NOT been split: it "addresses shape, building, energy,
structure, drawing, node and reference scopes through ONE `*` bucket." B5's rationale for declaring it
single directly contradicted this — it even quoted the same mutation ids (`create-shape-model`,
`create-building-model`, `create-energy-model`, …) while calling them "the SAME single value model."

Verified against the snapshot: `s.cad.cad` has four independently create/delete-able MODEL types
(`create-shape-model`/`delete-shape-model`, `create-building-model`/`delete-building-model`,
`create-energy-model`/`delete-energy-model`, `create-structure-classic-model`/
`delete-structure-classic-model`), plus `drawing` (2D annotation over the models), `node` (the
placement/hierarchy tree) and `reference` (a node's link to a model — hidden/locked/width/move/
replace/media). One mutation, `change-active-model-definition`, selects which of the four MODEL types
is the active pane and belongs to none of them individually — given an 8th `document` scope rather than
forced into one of the framework comment's 7 named ones.

**Mapping** (all 20 mutations, smallest owner):

| subset | mutations |
| --- | --- |
| `shape` | create-shape-model, delete-shape-model |
| `building` | create-building-model, delete-building-model |
| `energy` | create-energy-model, delete-energy-model |
| `structure` | create-structure-classic-model, delete-structure-classic-model |
| `drawing` | create-drawing, delete-drawing |
| `node` | create-node, delete-node, rename-node |
| `reference` | change-reference-hidden, change-reference-locked, change-reference-width, move-reference, replace-references, replace-reference-media |
| `document` | change-active-model-definition |

**Mechanics** (following shard A5's proven pattern for `s.architect.program`, read from
`$TICKET/📓️a5-architect-program-subsets.md` before doing any of this): the physical mutation
directories, `🚪️io`, `🧬️schema` kernel and `🏭️generator` stay under `✳️any/` — they are genuinely one
shared codec/dispatch for one document, not 8 independent implementations, exactly as A5 found for
program's 66 registers. What moved:

- `🏅️standards/🔖️1/🪆️subsets/🔣️.json` — replaced the single `"*"` entry with the 8 real subsets above,
  each with a written rationale; removed `subsetPolicy`/`subsetPolicyRationale`. The shared
  `ioFidelity`/`ioFidelityDrops`/`inferences`/`importDialects`/`exportDialects`/`examples` fields (which
  describe the WHOLE document's codec, not any one subset) are carried forward identically on all 8
  entries, since they remain true of each; the stale `"kind": "owning"` and `partialMutations` fields
  (the latter already referencing nonexistent mutation ids like `create-object`/`delete-object` — a
  pre-existing defect, not touched) were dropped rather than propagated.
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — added `"subset"` to all 20 entries in
  `mutationManifests[0].mutations`. `manifest.subset` itself stays `"any"` (matching the directory,
  satisfying `mutationManifestProblems`'s path cross-check); `owningSubsetOf` reads the per-mutation
  override first, so every mutation now resolves to a real, non-wildcard subset and
  `isWildcardSubsetFor` never fires.

No Rust, TypeScript, or test files were touched — nothing else references `mutation.subset`, and
`mutationCatalogs` (the separate test-catalog block in the same file) was left alone, matching A5.

### `s.stdio.ifc@4` — SPLIT (10 mutations → 1 real subset, renamed off the wildcard spelling)

The coordinator's evidence: `s.stdio.ifc@2x3` (the same artifact, a sibling standard) already declares
real subsets `cv20`/`sav`/`cobie` (IFC Coordination View 2.0, Structural Analysis View, COBie) beside its
own `any` bucket — proof this format genuinely has Model View Definition-shaped conformance classes.
Declaring `ifc@4` single therefore misstates the format, even though `ifc@4`'s current 10 mutations
(`insert-entity`, `remove-entity`, `set-entity-name`, `set-entity-arg`, `insert-entity-arg`,
`remove-entity-arg`, `set-file-description`, `set-file-name`, `set-file-schema`, `set-snapshot`) turn out
to ALL be generic, schema-agnostic Part-21/STEP entity-graph edits — the same kind of universal layer
`ifc@2x3`'s own `any` bucket holds (`set-snapshot`, `set-header`, `remove-instance`, `upsert-instance`),
with no MVD-specific mutation implemented yet for IFC4.

Cross-standard `declaredSubsets`/`subsetPolicyIsSingle` are keyed per `artifact@standard`
(`🟦️.ts:2650`), so `ifc@2x3`'s real subsets do not mechanically interact with `ifc@4`'s — the gate would
not have caught this on its own. This is a case where the honest fix is a rename, not an empty scaffold:
inventing empty `cv20`/`sav`/`cobie`-equivalent directories under `ifc@4` with zero real mutations in them
would have created new REAL siblings next to the wildcard `any` bucket that still held all 10 manifested
mutations — precisely the `wildcard-subset-owner` HARD-failure trap shard B5's own report documented
hitting and reverting for 6 similar `✳️any`-named artifacts (`zip`, `pptx`, `ifc-2x3-any` itself, `step`,
`xlsx`, `semio`), which recommended exactly this remediation: "rename each of these … `✳️any` → e.g.
`✳️base`, matching the PDF precedent."

**Mechanics**: `🏅️standards/🔖️4/🪆️subsets/🔣️.json` — replaced the single `"*"` entry with one real,
non-wildcard subset `base`, named and rationale explaining it is the shared entity-graph layer future
IFC4 MVD subsets would sit beside, not a "not yet split" placeholder; removed `subsetPolicy`. The
directory itself stays named `✳️any` (physical rename was not needed — `subsetCoordinatesOfOwner` only
governs `standardDirectoryName`/`subsetDirectoryName` cross-checks, not the semantic `subset` id) and the
oracle registration's capability id (`ifc-4-any-mutate`) was left as-is (unique, so no collision; renaming
it would ripple into the Rust oracle dispatcher and is out of this shard's bounded scope — flagged as a
minor follow-up, not a defect). `🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/🔣️.json` —
`mutationManifests[0].subset` changed from `"any"` to `"base"` (all 10 mutations share this one real
subset — there is nothing narrower to give them yet).

## 2. `s.norm.din16798@1` — SINGLE-JUSTIFIED, verified

Read the artifact's own feature file
(`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/mutate-din16798-1/🥒️.feature`),
which argues this is "the widest FLAT vocabulary in the plugin — all sixty-two kinds are `change-<field>`
on one indoor-climate record, with no collection, no composed child and no positional addressing
anywhere." Verified independently against the snapshot schema
(`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`): every one of the 62 properties is a top-level
scalar (`type: number|string`, `x-semio-state: artifact`), `additionalProperties: false`, zero nested
objects, zero arrays. This is true. Left untouched.

## 3. The rest of `s.norm.*` — applied the same test, found real differences

Every sibling artifact's snapshot schema was read directly (not inferred from mutation-id spelling
alone) to check for arrays (positional-addressing collections) or Rust module boundaries mapping to a
published standard's own parts — din16798's own deciding test, made mechanical.

| artifact | mutations | array/composed-child evidence found | verdict |
| --- | --- | --- | --- |
| `s.norm.en1990@1` | 10 | `qK: array<En1990QkEntry>` — a real, positionally-addressed (insert/remove/reorder) collection of variable-action entries, separate from 5 scalar fields | **SPLIT** — `combination` (5) / `variable-actions` (5) |
| `s.norm.en1991@1` | 32 | `properties` schema check: 0 arrays, all top-level scalars (`additionalProperties: false`) despite covering bridge/silo/wind/fire/crane/snow/accidental/construction domains in field-name prefixes only | SINGLE-JUSTIFIED |
| `s.norm.en1992@1`…`en1999@1` (8 artifacts) | 35,17,22,20,22,22,49,26 | same check: 0 arrays in every one of the 8 remaining EN schemas | SINGLE-JUSTIFIED (all 8) |
| `s.norm.din18599@1` | 13 | 0 arrays, all scalar | SINGLE-JUSTIFIED |
| `s.norm.din4108@1` | 22 | `layers: array<Din4108LayerDocument{thicknessM, lambdaWMk}>` — matches `change-layer-lambda`/`change-layer-thickness`/`insert-layer`/`remove-layer`/`reorder-layers` exactly; 17 remaining fields are flat scalars | **SPLIT** — `envelope` (17) / `layers` (5) |
| `s.norm.iso16757@1` | 21 | Rust struct types each top-level field by ISO 16757 PART module: `catalogue`/`selection`: `part_1::*`; `dictionary`: `part_4::Dictionary`; `part_number_rule`/`part_number_inputs`/`script_limits`/`exchange_process`: `part_5::*` — the published standard's OWN conformance-class boundary, in the artifact's own source | **SPLIT** — `part-1` (14) / `part-4` (2) / `part-5` (5) |
| `s.norm.vdi3805@1` | 19 | Rust source's own `// #region Part1` / `// #region Geometry` / `// #region Functions` boundaries; `geometry`/`curves` are independently-keyed `BTreeMap` registers, distinct from the catalogue's product list | **SPLIT** — `catalog` (10) / `geometry` (6) / `curves` (3) |

For en1991 through en1999 and din18599 the domain-prefixed field names (`bridge-`, `silo-`, `wind-`,
`crane-`, …) are surface vocabulary only — every one of them is a flat scalar property directly on the
artifact object, verified with a recursive `"type": "array"` scan of each schema
(`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`) that returned zero hits for all 9. Structurally
identical to `din16798`: no collection, no composed child, no positional addressing. The Eurocodes DO
have real published Parts as separate documents, but that fact alone is not evidence THIS artifact
under-splits — the artifact's own field shape shows it genuinely modeled every one of those parts as
scalar facts on one combined record, not as separable sub-documents. That is a legitimate authoring
choice (matching how `din16798` legitimately combines many topics into one flat record) and is
distinguishable from `iso16757`/`vdi3805`, where the Rust source itself draws the boundary by declaring
distinct part-typed fields.

## 4. Cross-artifact mechanical check — did I miss another `ifc`-shaped case?

The coordinator's `ifc` evidence generalizes to a mechanical test: for every artifact with MORE THAN ONE
declared standard, does any sibling standard already carry real (non-wildcard) subsets while another
carries `subsetPolicy: "single"`? Ran this across ALL 68 `🔣️.json` files with `subsetPolicy` plus every
sibling standard directory of the same artifact (68 files scanned, all standards of all artifacts
considered, not just the 68 flagged ones). Result: **exactly one hit, `s.stdio.gif`** — `89a` already
carries real subsets `base`/`graphic-control`/`comment`/`application` (declared by an earlier shard this
ticket, from GIF89a's own extension-block structure) while `87a` carries `subsetPolicy: "single"`.
Verified this is correct, not a second miss: GIF87a (1987) predates the Graphic Control/Comment/
Application Extension blocks GIF89a (1989) introduced — the format genuinely has no narrower conformance
class to split against for `87a`. Confirmed the mutation vocabulary matches: `87a`'s 11 mutations
(background/global/image/pixel/screen/snapshot nouns) carry no comment/application/graphic-control
mutation at all. Left untouched. `s.stdio.ifc` was the only OTHER hit before this shard's own edit fixed
it (§1); after fixing `ifc@4` this check now returns only the legitimate `gif` case.

## 5. The remaining 62 — spot-verified, no further action

The other 62 audited artifacts split into two evidence patterns, both checked directly rather than
assumed:

**Flat scalar records, matching `din16798`'s own test** (0 arrays in the snapshot schema, or the
one array present is UI/selection state such as `selectedIds`, not a domain collection): all remaining
`s.norm.*` covered in §3, plus the small `s.stdio.*` formats (`ply`, `html`, `epw`, `gif@87a`, `mp4`,
`mp3`, `binary`, `txt`, `csv`, `tsv`, `md`, `png`, `wav`, `dwg@ac1018`, `dwg@ac1024`, `bmp`, `deflate`,
`stl`) — each already scoped to one specific format/version with no sibling conformance class published
for it, confirmed via §4's cross-standard check finding no real sibling for any of them. `png@1.2` in
particular is this ticket's own conforming exemplar for subset LAYOUT (cited in the shared brief) and was
independently confirmed single: PNG's optional ancillary chunks (`tEXt`, `pHYs`, `tRNS`, …) are
extensibility, not conformance profiles the way PDF/A vs PDF/X are.

**One repeating child type, no second independently-meaningful entity** — verified against each
artifact's own snapshot schema's top-level `object`/`array` properties (not inferred from mutation-id
verbs alone, since `create-x`/`delete-x` verbs read as "collection evidence" even when there is only ever
ONE kind of `x`): `s.lowpoly.lowpoly` (`objects` only), `s.raster.raster` (`layer`/`layers` only),
`s.energy.model`, `s.space.home`, `s.demonstrator.playground`, `s.gis.gisterrain` (trivial, ≤2
mutations). Where there is no SECOND entity type to split against, there is nowhere smaller to move a
mutation to — the artifact is not under-split, it is fully split already (down to its one real subset).

**Multiple internal collections, but no external standard and no independently-viable sub-document** —
verified by checking for a Rust `#[path]`/`// #region` boundary mapping to a PUBLISHED standard's own
part/profile structure (as `iso16757`/`vdi3805` had) and finding none: `s.block.2d/3d/5d`
(handles/vortices/grips + compatibility + attributes + authors — all facets of ONE block-kit
specification; `s.block.3d`'s own `🦀️.rs` region markers are `Document`/`VortexKindCatalogComposition`/
`WindowView`/`Snapshot` — implementation organization, not standard parts), `s.puzzle.2d/3d/5d`
(nodes/edges, objects/attractions/targetVolumes/references, parts/fasteners — one puzzle-graph document
each; `s.puzzle.3d`'s regions are `Errors`/`Scale`/`Document`/`Snapshot` — same pattern), `s.layout.layout`
(paragraphStyles/characterStyles/stories/links/pages/spreads — one desktop-publishing document),
`s.remodel.remodel` (streams/assets/gcps — one photogrammetry reconstruction job), `s.shooting.shooting`
(assets/savedCameras/shots — one shot list), `s.process.process3d` (steps/toolSolids — one CAM process,
tools referenced from within steps), `s.procedural.procedural2d/3d`, `s.forms.forms`, `s.playbook.playbook`,
`s.dag.dag`, `s.assembly`, `s.flow.flow`, `s.gis.gismap`, `s.vcs.vcs`, `s.animate.present`,
`s.reasoning.wires`, `s.writer.writer`, `s.imperative.imperative`, `s.trinity.rewrite`, `s.trinity.jack`,
`s.space.space`, `s.sourcing.curate` — every one is a semio-native format with no external standard to
draw a conformance-class line against, and every internal collection found only makes sense bundled with
the others (a compatibility rule refers to handle kinds that live beside it; a shot references cameras and
assets that live beside it). This is the same distinction that makes `s.cad.cad`'s split correct and
these artifacts' non-split correct: `s.cad.cad`'s building/energy/structure models are each
independently-viable document types with their own external analogues (BIM, energy simulation, structural
analysis); none of the collections in this group are.

Not individually re-derived to A5's level of rigor for the register-by-register accounting (that is a
20-30 minute deep-dive per artifact, as `iso16757`/`vdi3805`/`cad` each took), but the deciding evidence
(no arrays beyond UI selection state, or no cross-standard real sibling, or no external
conformance-class-shaped Rust module boundary) was checked directly against the schema/source for every
one, not assumed from B5's rationale text.

## 6. What changed, file by file

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/🔣️.json` — 8 real subsets replace `"*"`.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — per-mutation `subset` added to all 20 entries.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/🔣️.json` — `"*"` renamed to real subset `base`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — `mutationManifests[0].subset` `"any"` → `"base"`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/🔣️.json` — 2 real subsets (`combination`, `variable-actions`) replace `"*"`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — per-mutation `subset` added to all 10.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/🔣️.json` — 2 real subsets (`envelope`, `layers`) replace `"*"`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — per-mutation `subset` added to all 22.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/🔣️.json` — 3 real subsets (`part-1`, `part-4`, `part-5`) replace `"*"`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — per-mutation `subset` added to all 21.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/🔣️.json` — 3 real subsets (`catalog`, `geometry`, `curves`) replace `"*"`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — per-mutation `subset` added to all 19.

No mutation was deleted, no manifest was weakened, no physical mutation directory was moved. Every split
uses shard A5's proven mechanism (per-mutation `subset` override in the manifest, real names in
`🪆️subsets/🔣️.json`, physical kernel left in place since it is genuinely shared) rather than a fresh
physical fork, because none of these six artifacts has (or needs) a genuinely separate implementation per
subset — they share one codec/dispatch for one document, same as `s.architect.program` did.

## 7. A race with a concurrent session, caught and corrected

Mid-shard, a foreground gate run showed `wildcard-subset-owner` at 10 (all on `s.stdio.ifc@4`) after it
had read 0 on the prior run. Diagnosed via direct file read (not trusting the gate's cached snapshot
alone): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/🔣️.json`'s
`mutationManifests[0].subset` had reverted from `"base"` back to `"any"` — a concurrent session's write to
the same shared file raced mine (this ticket has multiple live shards touching oracle manifests; a
diff of the file's own git history shows an unrelated auto-commit from an earlier shard's session, though
the specific overwrite of the `subset` field is not in that commit's diff — most likely a different,
still-uncommitted concurrent edit). Per the shared brief's house rules, did not chase or revert the other
session's work — re-read the file, reapplied only the one field this shard owns (`subset: "base"`), and
reran the gate to confirm `wildcard-subset-owner` returned to 0 with no other regression. `s.cad.cad` and
the four `norm.*` files were checked for the same symptom at the same time and found intact.

## Answer to the four questions asked

- **How many of the 68 confirmed single vs split**: 62 SINGLE-JUSTIFIED (verified, left untouched), 6
  SPLIT (executed) — `s.cad.cad@1`, `s.stdio.ifc@4`, `s.norm.en1990@1`, `s.norm.din4108@1`,
  `s.norm.iso16757@1`, `s.norm.vdi3805@1`.
- **Before/after numbers**: table in "Gate" above — `unsplit-artifact-subset`/`wildcard-subset-owner`/
  `duplicate-mutation-owner` unchanged at 0/0/0 (this shard's own signal); `test-only-mutation` 51 → 12
  and repo TOTAL 2034 → 1993, both driven by concurrent unrelated shards, not this shard's edits
  (verified per-artifact, §5/§Gate); `mutation-catalog-unclaimed`/`capability-without-manifest` unchanged
  at 8/6.
- **Markdown path**: this file,
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️c1-subset-policy-audit.md`.
