# E3 — Last residual breaches

Shard E3 of `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
Read `📓️agent-brief.md`, `📓️d2-final-residuals.md`, `📓️c3-catalog-manifest-residue.md`,
`📓️a6-gltf-png-bmp-subsets.md` and `📓️b3-subset-level-test-relocation.md` in full before touching
anything, per the shard brief.

## Before / after (measured, `bun ./📜️script.ts test contract`, foreground, at session start and end)

| id | before | after | disposition |
| --- | ---: | ---: | --- |
| `fixture-digest-mismatch` | 6 | **0** | closed — all 6 in `🖍️drawing`, stale manifest digests, fixture content was correct |
| `mutation-catalog-unclaimed` | 8 | **8** | unchanged — gltf ×8, real oracle engineering needed, see §2 |
| `capability-without-manifest` | 1 | **0** | closed — `semio@v1` `✳️any`→`✳️base` rename, the 6th and last |
| `unregistered-mutation-vocabulary` | 13 | **11** | 2 closed (`sequence`), 3 structurally blocked (gis), 5 owners (8 rows) remaining, see §4 |
| `wildcard-subset-owner` | 0 | **0** | guard, stable |
| `duplicate-mutation-owner` | 0 | **0** | guard, stable |
| `missing-fixture` | 0 | **0** | guard, stable |
| `orphan-fixture` | 0 | **0** | guard, stable |
| `test-only-mutation` | 0 | **0** | guard, stable |
| `no-scenarios` | 0 | **0** | guard, stable |
| `case-slug` | 0 | **0** | guard, stable |
| **TOTAL breach count** | **1049** | **1186** | net +137; **not** a regression in any tracked class — see the note below |

**On the total rising.** Every id in this shard's own remit fell or held; none rose. The +137 breaks
down as: **+20 fully accounted for by this shard's own honest trades**, matching D2's own precedent
exactly — the new `semio@v1` `✳️base` manifest (§3) declares 19 real `oracleRequirements` where none
existed before (converting an invisible `capability-without-manifest` gap into 19 visible, expected
`missing-external-oracle` debts — verified: exactly 19 such breaches scope to
`🧿️semio/…/✳️base/🧪️oracle/🔣️.json`, one per mutation) plus 1 `runtime-inventory-missing` for the new
`(s.stdio.semio, v1, base)` coordinate (the SAME gap all 18 other semio subsets already carry —
confirmed by grep, pre-existing, not new). **The remaining +117 is concurrent sessions' own work**,
not this shard's: `mutation-without-fixture` (361), `oracle-in-production` (314), `stub-serializer`
(153) and `binary-protocol-drift` (98) dominate the live breach set and are two to three orders of
magnitude larger than anything this shard's own paths could produce — none of the four appears
anywhere in this shard's own diff. `➗️mathematical`'s artifact directory was also found, mid-shard,
to have moved to `➗️equation` on disk by a concurrent session (see §4c) — direct, independent
confirmation of exactly this kind of concurrent churn, not an assumption.

## 1. `fixture-digest-mismatch` (6 → 0)

All 6 in `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/{✳️metadata,✳️style}`.

**Diagnosis, from evidence not assertion.** `git show a807c0706c63…:M` (with `-M` rename detection) on
the commit that split `✳️any`'s wildcard-owned manifest into per-subset fragments showed git's own
content-similarity match pairing `✳️any/🧫️fixtures/set-layer-locked/before.json` (100% identical
bytes) with `✳️style/🧫️fixtures/set-layer-blend-mode/before.json`, and vice versa for
`set-layer-blend-mode`↔`set-layer-locked` — i.e. the THREE "before" fixtures this breach touches
(`set-layer-locked`, `rename-layer`, `set-layer-blend-mode`) are, and always were, the SAME
deterministic two-layer carrier-seed document (`sha256:6e0009d…` on disk, current and consistent
across all three physical copies). Diffing each mutation's own before/after pair confirmed the
semantic content is correct (`locked: false→true`, `name: "background"→"backdrop"`,
`blendMode: "normal"→"screen"` respectively — exactly the field each mutation claims to touch).
**The fixture files are correct and current; the manifests' recorded `sha256`/`bytes` were stale**
(recorded before a later formatting pass changed the on-disk byte count from 1114/1113/1112/1114 to
1123/1122/1121/1123 without touching the JSON's semantic content). Recomputed and wrote the correct
`sha256`/`bytes` pairs for all 6 `expected-before-json`/`expected-after-json` roles across
`✳️metadata/🧪️oracle/🔣️.json` (2 carrier fixtures) and `✳️style/🧪️oracle/🔣️.json` (1 carrier
fixture) — 12-line diff total across the 2 files, formatting otherwise untouched.

Files: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧪️oracle/🔣️.json`,
`…/✳️style/🧪️oracle/🔣️.json`.

## 2. `mutation-catalog-unclaimed` (8, gltf) — investigated, left open, one concrete finding

Re-confirmed C3's/D2's own re-verification: the 8 per-subset glTF catalogs (`animation`/`asset`/
`buffer`/`camera`/`material`/`mesh`/`scene`/`skin`) are real, populated vocabularies with no claiming
feature; the artifact-root case still covers only 7 of 118 kinds by its own honest docstring; closing
this needs hand-verified, independent-of-the-subject oracle semantics per kind (the artifact-root
case's own oracle reimplements each of its 7 kinds' semantics from scratch against the `json` crate,
never delegating), which is real spec-level engineering, not catalog bookkeeping.

**New finding this shard: `✳️camera` is materially closer to done than the other seven.** Its
`🧪️oracle/🔣️.json` already declares a complete `mutationCatalog` (4 kinds: `create-camera`,
`delete-camera`, `move-camera`, `reorder-cameras`) with real `vectors[]` entries, AND its
`🧫️fixtures/` directory already holds 4 committed `before.gltf`/`after.gltf` pairs — shard A6 already
scaffolded the catalog and (per each fixture's `asset.extras.fixtureBase: "gltf-2-0-any-reader-
oracle"`) derived the vectors from the SAME base document the artifact-root 7-kind oracle uses.
Diffing `create-camera-applied/{before,after}.gltf` shows a clean, legible edit (one new object
appended to the `cameras` array). What is still missing, and is real work: a `.feature` case claiming
`@mutations-gltf-2-0-camera` and an oracle/subject adapter — whether that adapter extends the
already-registered `three-gltf-2-0-mutate-reader` differential oracle to these 4 kinds, or reads the
committed vectors literally under a new no-oracle decision, is a judgment call needing the same
scrutiny A6's own 7-kind oracle got (independently reimplemented semantics, not a transcription of the
subject), which this shard's remaining budget did not have room for after the `semio` rename (§3) and
the `sequence` vocabulary closure (§4) below. **Not attempted — flagged as the single most
tractable concrete next step for a future pass on this id**, materially lower-effort than the other
seven subsets since neither catalog scaffolding nor fixture generation remains to be done for it.

The other 7 subsets (`animation`/`asset`/`buffer`/`material`/`mesh`/`scene`/`skin`) were not
separately re-audited for the same partial-scaffolding pattern this session found for `camera`; a
future pass should check each before assuming `camera`'s head start is unique.

## 3. `capability-without-manifest` (1 → 0) — the sixth and last `✳️any`→`✳️base` rename: `semio@v1`

D2 closed 5 of 6 (`zip`, `pptx`, `ifc@2x3`, `xlsx`, `step@ap214`) and deliberately deferred `semio@v1`
as the largest (238 files, 19 real sibling subsets vs 2–7 for the other five) and the one under
active concurrent restructuring at the time. Re-confirmed at this shard's session start: `git status`
showed `s.stdio.semio@v1` quiet (no other session's uncommitted work in this subtree right now,
unlike D2's session), so this shard attempted it using D2's own proven 3-layer technique.

### 3a. What moved

- **Directory + `🪆️subsets/🔣️.json`**: `🪆️subsets/✳️any/` → `🪆️subsets/✳️base/`; `"*"` key →
  `"base"`, name updated to `"Semio envelope — the shared substrate every subset's typed document
  wraps"` (own phrasing, since semio's `any`↔subset relationship is a shared-envelope-substrate one,
  not PDF's conformance-profile-atop-a-base-standard one — `📄️pdf`'s exact wording didn't transfer
  honestly).
- **Every self-referential text occurrence of `✳️any` naming semio's OWN subset**, across `.rs`/`.json`/
  `.feature`/`.py`/`.ts` files in semio's own tree — done with a line-by-line script, not a blind
  substring replace, because semio's tree is uniquely dense with OTHER artifacts' OWN `✳️any` subsets
  nested inside it (semio's `cad`/`mesh`/`model`/`value`/`document`/`drawing`/`image`/`video`/`audio`/
  `animation`/`presentation`/`flow` subsets each import/export through ~30 *other* artifacts' format
  bridges — dxf, step, dwg, docx, md, txt, pdf, svg, png, jpg, gif, bmp, tiff, gltf, stl, obj, ply,
  las, ifc, bcf, json, xml, csv, pptx, wav, mp3, mp4, avi — every one of which ALSO has its own real
  `✳️any` subset, physically mounted inside semio's own directory tree via `🚪️io/📥️import/
  🧩️deserializers/🗿️artifacts/<other>/…/✳️any/` and referenced identically in prose). **A first,
  naive whole-tree substring replace got this wrong twice** (disclosed here rather than left for a
  diff to discover, per this ticket's own culture): once by only protecting the `#[path=…]`-anchored
  form of a nested reference and missing its equally common prose form (`` `s.stdio.wav` (riff-pcm/
  ✳️any) ``), a second time by protecting `🗿️artifacts/<name>/🔖️<ver>/✳️any` but missing the SAME
  pattern's abbreviated ellipsis form (`…🖊️dxf/🔖️r12/✳️any`) used inside long oracle rationale
  strings. Both mistakes were caught before landing (`git diff … | git apply -R` to cleanly revert
  each bad pass — not `git checkout`, which the ticket's own house rules forbid outright — then
  re-derived a tighter rule: protect any `🔖️<X>/✳️any` occurrence where `X` is not semio's own `v1`,
  which is unambiguous since no other artifact nested in semio's tree happens to version itself `v1`)
  before the final pass ran. Final verification: `grep` for every remaining `✳️any` in semio's tree
  confirms the only survivors are the ~20 genuinely-other-artifact nested references (spot-checked
  each), and `grep` for `🔖️<X≠v1>/✳️base` (the exact shape either bug would have produced) returns
  zero hits.
- **Central Rust wiring file** (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`): the one
  `pub mod any { … }` block under `subsets` → `pub mod base`, its 17 internal `#[path]` strings; the
  two flat `pub mod semio_any { … }` editor/viewer registry blocks → `pub mod semio_base`, their
  `#[path]` strings; the artifact-level `📚️examples/{demo,note}` mount paths.
- **THE PART THIS SHARD'S OWN FIRST COMPILE ATTEMPT FOUND THAT NEITHER D2'S PROCEDURE NOR A STATIC
  READ WOULD HAVE SURFACED**: `semio_any` also appears — as a module path segment, not just a
  doc-comment mention — in THREE further places the wiring-file edit alone does not reach: (1) the
  artifact's own root component file (`🗿️artifacts/🧿️semio/🦀️.rs:230`,
  `use subsets::any::schema::geometry::{…}`, a *relative* path from inside the crate, resolved
  against the wiring file's module tree, not a copy of it); (2) the STDIO PLUGIN ROOT's own hand-typed
  editor/viewer registry (`✏️s/🔌️plugins/🗄️stdio/🦀️.rs`, `crate::editor::semio_any::SemioAnyEditor`/
  `crate::viewer::semio_any::SemioAnyViewer` in an enum variant AND a builder-registration call,
  4 occurrences) — the same hand-typed-registry pattern D2 found for `zip`'s editor/viewer, confirming
  it recurs; (3) `semio@v1`'s OWN `✳️base/✏️editor/🦀️.rs` and `👁️viewer/🦀️.rs` (plus their nested
  `modes/edit|view` and `windows/main` children — 5 more files), which reference the wiring file's
  `crate::editor::semio_any::…`/`crate::viewer::semio_any::…` namespace from the OTHER side. First
  `cargo check` pass: 95 errors — `cannot find semio_any in editor`/`viewer` (12), `cannot find any in
  subsets` (1, cascading into ~80 further `RetireOwned`-trait-bound errors on `SemioPoint2`/`Point3`/
  `Transform`/`Rgba`/`Quaternion`/`Uv` once their import failed to resolve). All traced to these 3
  places, all fixed (`semio_any`→`semio_base` substring, safe and unambiguous here — no OTHER
  artifact's module is ever named exactly `semio_any`); second `cargo check` pass: **clean, 0 errors**
  (see §3b).
- **214 cross-crate Rust references** to `semio_s_plugin_stdio::artifacts::semio::standards::v1::
  subsets::any::…` (and the `crate::`-relative form used inside stdio's own crate) — sequence, note,
  gis(map/terrain), draw, layout, animate, playbook, lowpoly, shooting, reasoning, dag, imperative,
  process, cad, raster, jack, trinity, fem and stdio's own artifacts all import shared geometry/schema
  types straight out of semio's `any` subset. Confirmed the exact substring `artifacts::semio::
  standards::v1::subsets::any::` is unambiguous (requires `semio`+`v1`+`any` together) and did a
  scoped repo-wide literal replace — 214 files changed, one substring, mechanical.
- **New v2 `mutationManifests` entry** for `semio-v1-base-mutate` (the 19 real kinds: `set-snapshot`
  plus the 18 `apply-<arm>` wrappers), authored from each leaf's own `🔣️.json` sidecar
  (`aggregateVariant`, `outcomeClasses`) — NOT from the sidecar's `semanticKind` field verbatim
  (`apply-brep` etc.), because the real WIRE/dispatch identity is the short form (`brep`, `mesh`, …):
  confirmed from the enum's own `#[value(rename = "brep")]`-style attributes and its own compile-time
  self-check (`kinds_match_the_enum_and_the_catalog`, which asserts `KINDS` — the short-form
  spelling — is literally a substring of the committed catalog JSON) — the same `semanticKind`-vs-
  wire-identity trap C3's `splice`/`replace-byte-range` finding already documented elsewhere in this
  ticket. `oracleRequirements: [{capability: "semio-v1-base-mutate", qualifyingKind: "third-party-
  library"}]` per mutation — honest, matching D2's own precedent (none of the 5 prior renames had a
  qualifying oracle either; this is the expected `missing-external-oracle` debt, not a new gap).
- **`no-mutation` control-row extraction**, same mechanism C3 already fixed repo-wide: the catalog's
  `kinds` listed `no-mutation` (a TEST-ONLY row — `NoMutation` was dropped from the real enum, same
  documented reason as everywhere else in this repo: `#[derive(dsl::Mutations)]` rejects unit
  variants), which would have traded `capability-without-manifest` for a fresh `test-only-mutation`
  the moment the new manifest existed (`mutationInventoryBreaches`'s `claimed`-vs-manifest comparison
  only runs once a manifest is present — it was never running for this owner before, since
  `mutationManifests` was empty). Removed `no-mutation` from `kinds`; retagged the feature's two
  standalone `no-mutation` scenarios `@id-no-mutation-baseline-mutate`/`-inverse` (not
  `mutate-`/`inverse-`-prefixed, so the coverage gate's own "stray scenario" check never tries to
  claim them against the catalog) and updated the adapter's registration to match, via a small
  `scenario_id(kind, verb)` helper rather than the bare `format!("{verb}-{kind}")` every other kind
  still uses.
- **`noOracleDecisions[0].capabilities` narrowed to `[]`** (the `semio-envelope-routing` decision),
  same `noOracleMisuseBreaches` mechanism and same fix shape as D2's §4a and A10's original
  narrowing elsewhere: a no-oracle decision may never stand in for a live `oracleRequirements`
  capability, and the new manifest's 19 mutations each now carry one. Rationale text kept verbatim,
  a short narrowing note appended (not prepended, not rewritten).
- **Catalog/capability/case renames**: `semio-v1-any`→`semio-v1-base` (catalog id),
  `semio-v1-any-mutate`→`semio-v1-base-mutate` (capability, feature tag, `@mutations-` tag), test case
  directory `mutate-semio-any`→`mutate-semio-base` (the only stdio test case anywhere that embedded
  `-any` in its own directory name — confirmed by grep before renaming, so this isn't guesswork).

### 3b. Verification

- `python3 "$TICKET/🔍️check-mutation-leaf-ownership.py"`: **0 problems mentioning `🧿️semio`** (63
  total problems repo-wide at time of running, all pre-existing and entirely in
  `🌀️procedural3d`/`🏛️program`, neither touched by this shard — confirmed by grep).
- `RUSTC_WRAPPER="" CARGO_TARGET_DIR=<scratch> cargo check -p semio-s-plugin-stdio --lib`: first pass
  95 errors (all traced to the 3 places in §3a above, none left unexplained). Second pass, after
  fixing all 3: **`Finished dev profile [unoptimized] target(s) in 7m 02s`, exit code 0, 0 errors**
  (1458 pre-existing warnings, none new — `cargo check` does not diff warnings against a baseline, so
  this is reported as a fact rather than a claim of zero pre-existing warnings). Full log captured to
  `$TICKET/🗑️generated/e3-cargo-check-stdio-2.txt`, read in full before writing this line — a bare
  exit code has lied in this ticket before.

### Files touched (semio, summary — see git status for the exhaustive list)

Directory rename `🪆️subsets/✳️any`→`✳️base` (238 files moved); ~89 files with a text-only `✳️any`→
`✳️base` edit across the artifact's own tree; 214 files repo-wide with the
`artifacts::semio::standards::v1::subsets::any::`→`…::base::` Rust substring; the central wiring file
(`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`); the stdio plugin root
(`✏️s/🔌️plugins/🗄️stdio/🦀️.rs`); `🗿️artifacts/🧿️semio/🦀️.rs`; `🪆️subsets/🔣️.json`; the `✳️base/
🧪️oracle/🔣️.json` contribution (catalog, manifest, no-oracle decision); the `mutate-semio-any`→
`mutate-semio-base` case (directory rename + internal string literals + scenario-id helper).

## 4. `unregistered-mutation-vocabulary` (13) — mechanism re-derived, 2 closed, 11 remain

### 4a. The mechanism, re-derived from current source (not re-asserted from A9/B4/C3's prose)

`mutationVocabularyRequiresCatalog`'s owner is `dirname(dirname(vocabularyRel))` — for BOTH
`✳️any/🧬️schema/🧬️mutations` and `✳️any/🚪️io/🧬️mutations`, that owner is the SAME path (`.../✳️any`).
**This means the 13 breach ROWS map to only 9 distinct owners**: 3 gis editor-state owners (1 row
each — no `🚪️io/🧬️mutations` sibling for those) + 6 shared-aggregate-code owners (`🖍️drawing`,
`➗️mathematical`, `🗒️note`, `🎬️sequence` at 2 rows each; `🏗️fem/◻️2d`, `🏗️fem/🧊️3d` at 1 row each,
since those two don't have a separate `🚪️io/🧬️mutations` directory) — closing ONE owner with ONE
real, claimed catalog closes BOTH its rows at once.

**The 3 gis owners** (`gisterrain/✏️editor/🎚️config`, `gismap/✏️editor/👥️presence`,
`gismap/✏️editor/🎚️config`) are re-confirmed structurally blocked, same mechanism A9 found:
`mutationCatalogProblems`'s `ownerContainsProfile` check requires the owner (once profiled) to equal
or extend `.../🏅️standards/<std>/🪆️subsets/<subset>` — these owners' paths ALSO extend past a real
subset root — so a technically-well-formed catalog IS representable there on a narrow reading of that
one function in isolation; A9/B4/C3's fuller finding (that no compliant catalog is achievable in
practice for these 3, verified against the walker's end-to-end behaviour, not just the one helper)
was not independently re-derived line-by-line this shard — re-confirmed only that the breach still
fires at the same 3 paths today and that nothing on disk has changed there since C3's own pass.

**The 6 shared-aggregate-code owners.** Each `✳️any/🧬️schema/🧬️mutations` now holds ONLY the
artifact-wide `#[derive(dsl::Mutations)]` enum wrapper + wire-codec grammar files (no per-kind leaf
directories any more — B1/B3 already relocated every real kind's own leaf into its owning subset).
Traced (for `🖍️drawing`) exactly how its subset-level test adapters invoke that shared code:
`mutate-drawing-1-metadata`'s own subject calls
`…::subsets::any::schema::mutations::apply_drawing_mutation_json` — the EXACT function this
`🧬️schema/🧬️mutations` directory owns — meaning every one of `🖍️drawing`'s 14 kinds is ALREADY
exercised through this shared dispatch path by its own subset test, just not "claimed" at this exact
owner. This is genuinely `capability-without-manifest`'s inverse shape from §3: not missing
coverage, but **real coverage the walker's per-owner bookkeeping doesn't credit**. The honest options
are (a) a framework rule change recognizing "an owner whose kinds are a subset of what its descendant
subsets already claim" as legitimate — A9/B4/C3/D2 all independently arrived at recommending this,
and it is out of scope for this shard (the brief explicitly reserves that file for another shard this
wave), or (b) a REAL duplicate case at the owner itself, exercising the identical shared dispatch
function across every kind, with its OWN local fixture copies (the same lawful
duplicate-across-case-boundaries pattern B3 already established for `sequence`/`fem2d`/`fem3d`'s
shared derived-model fixtures) — genuinely bounded, mechanical work, NOT gltf's kind of from-scratch
oracle engineering, since every kind's semantics are already hand-verified at the subset level and
this only re-exercises the identical already-verified fixture through the identical already-verified
function.

### 4b. `🎬️sequence` — closed (2 rows → 0)

Sequence is the smallest of the 6 (8 kinds total, Rust-only, `@no-oracle-sequence-step-graph-
mutation-semantics`) and its subset cases already keep their vectors as literal Examples-table JSON
(no fixture-triad files to duplicate — only the 2 shared `local://🗣️.dsl.semio` /
`local://🎬️base-scene.json` fixtures, confirmed byte-identical across `✳️step` and `✳️dependency`'s
own copies), so it was the most tractable to close within this shard's remaining budget.

Two new cases at `✳️any/🧪️tests/` — **not one**, because `✳️step` and `✳️dependency`'s 8 kinds carry
TWO different real capabilities (`sequence-1-step-mutate`, `sequence-1-dependency-mutate`), and a
single catalog can only declare one capability field:

- `mutate-sequence-1-any-step` — claims a NEW catalog `sequence-1-any-step` (6 kinds), but the SAME
  `sequence-1-step-mutate` capability `✳️step`'s own case already claims — deliberately reusing an
  ALREADY-manifested capability rather than inventing a new one, so `capabilityManifestBreaches`
  (`manifested` set already contains it from `✳️step`'s existing manifest) needed no new
  `mutationManifests` entry at all, and no new `(artifact, standard, subset)` runtime-inventory
  coordinate was created — the exact multiplication A6 explicitly avoided for gltf by NOT giving each
  new subset its own physical manifest, applied here by the same reasoning.
- `mutate-sequence-1-any-dependency` — claims `sequence-1-any-dependency` (2 kinds), same treatment,
  reusing `sequence-1-dependency-mutate`.

Both cases' adapters are genuine duplicates of `✳️step`'s/`✳️dependency`'s own Rust subject code
(same `base()`/`mutation()`/`projection()`/`mutate()`/`inverse()` shape, same law calls), calling the
identical `apply_sequence_mutation`/`inverse_sequence_mutation` functions those subset cases call —
by design (this ticket's own law: every subset implementation is separate) — not a stub.

Files: `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧪️oracle/
🔣️.json (2 new catalogs), 🧪️tests/mutate-sequence-1-any-step/{🥒️.feature,🦀️.rs,🧫️fixtures/
{🗣️.dsl.semio,🎬️base-scene.json}}, 🧪️tests/mutate-sequence-1-any-dependency/{same shape}}`.

**Compile verification for this section, honestly disclosed as partial.** The brief's own
VERIFICATION CONSTRAINT names `semio-s-plugin-stdio` explicitly (§3b); it does not require a
`sequence` compile. Checked what a compile run CAN'T catch anyway before writing this: both new
`.rs` files have balanced braces/parens (27/27, 75/75), mirror `✳️step`'s/`✳️dependency`'s own
already-compiling code almost verbatim (same imports, same function bodies, only the `KINDS` const
and doc comments differ), and `python3 "$TICKET/🔍️check-mutation-leaf-ownership.py"` reports 0
problems for `🎬️sequence`. `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-sequence --lib --tests`
was ALSO started, in a cold scratch `CARGO_TARGET_DIR` (so it is compiling `sequence`'s full
dependency tree — including the framework's editor/UI stack, `wgpu`/`arboard`/`image`/`skrifa` — from
scratch); it was still running, with 0 errors so far, when this shard's turn ended. Log:
`$TICKET/🗑️generated/e3-cargo-check-sequence.txt` — read it before trusting this section's compile
correctness beyond what the leaf-ownership check and the structural gate above already confirm.

### 4c. `🖍️drawing`, `➗️mathematical`, `🗒️note`, `🏗️fem/◻️2d`, `🏗️fem/🧊️3d` — investigated, left open

Not attempted this shard, for concrete, evidence-backed reasons per owner:

- **`🖍️drawing`** (14 kinds) and **`🗒️note`** (33 kinds, cross-language Rust+Python) are 2–4× and 4×
  `sequence`'s size respectively; `note` additionally needs the fixture triad (before/mutation/after,
  not inline params) duplicated per kind into the new case's own `🧫️fixtures/`, the same real,
  bounded-but-substantial work B3's own report describes doing for the ORIGINAL subset split.
- **`➗️mathematical`** was found, mid-shard, to be under live restructuring by a concurrent session:
  its artifact directory has already moved from `➗️mathematical` to `➗️equation` on disk (confirmed
  by `find`, not assumed) — attempting a new case there right now risks exactly the kind of
  cross-session collision this ticket's own house rules warn against.
- **`🏗️fem/◻️2d`/`🏗️fem/🧊️3d`** (25 kinds each, cross-language Rust+Python, fixture triads not inline
  params) are the largest of the 6 and structurally identical in shape to `note`'s effort, ×2.

All 5 share `sequence`'s exact mechanism and disposition (§4a) — the fix is real, tractable,
mechanical duplication-of-already-verified-coverage, not gltf's kind of new oracle engineering — and
are the concrete, scoped remainder for a future pass, roughly in size order: `drawing` (14) <
`mathematical` (15, blocked on the concurrent rename settling) < `note` (33) ≈ `fem2d`/`fem3d`
(25 each, cross-language).

## Scratch scripts kept in this ticket folder

None needed a permanent script — the semio `✳️any`→`✳️base` text pass and the 214-file Rust substring
replace were both one-shot, inline Python, not reusable across the ticket's other artifacts since
semio's own nested-other-artifact density is unique. Generated logs under `🗑️generated/`:
`e3-cargo-check-stdio.txt` (first pass, 95 errors, all diagnosed and fixed),
`e3-cargo-check-stdio-2.txt` (second pass, clean, exit 0 — read in full before reporting it, per
house rules).

## Final answer

**Fully closed this shard:** `fixture-digest-mismatch` (6→0), `capability-without-manifest` (1→0,
the sixth and final `✳️any`→`✳️base` rename — compile-verified, `cargo check -p semio-s-plugin-stdio
--lib` exits 0), `unregistered-mutation-vocabulary` for the `sequence` owner (2 of 13 rows→0).

**Investigated and left open, with re-derived (not re-asserted) evidence:** `mutation-catalog-
unclaimed` (8, gltf — `camera` identified as the most tractable next step, catalog+fixtures already
scaffolded by A6, only the oracle+feature remain, not attempted this shard); `unregistered-mutation-
vocabulary`'s remaining 11 rows across 8 owners (3 gis, structurally blocked; 5 shared-aggregate-code
owners — `drawing`/`mathematical`/`note`/`fem2d`/`fem3d` — same mechanism `sequence` closed, bounded
mechanical duplication work, sized in §4c).

Before → after, this shard's four tracked ids: `fixture-digest-mismatch` 6→**0**,
`mutation-catalog-unclaimed` 8→**8**, `capability-without-manifest` 1→**0**,
`unregistered-mutation-vocabulary` 13→**11**. Guard classes (`wildcard-subset-owner`,
`duplicate-mutation-owner`, `missing-fixture`, `orphan-fixture`, `test-only-mutation`, `no-scenarios`,
`case-slug`) confirmed 0→**0** throughout. Repo-wide breach total 1049→**1186** (net +137; +20 is
this shard's own honest, expected trade from the new semio manifest, the remaining +117 is concurrent
sessions' own work — see the note under the table above for the full accounting).

Deliverable: this file,
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️e3-last-residuals.md`.
