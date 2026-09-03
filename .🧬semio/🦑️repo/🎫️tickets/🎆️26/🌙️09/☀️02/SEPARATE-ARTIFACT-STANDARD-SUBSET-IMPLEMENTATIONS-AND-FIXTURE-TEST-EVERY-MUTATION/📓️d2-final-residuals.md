# D2 — Final residual breaches, plus one incomplete artifact

Shard D2 of `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
Read `📓️agent-brief.md`, `📓️c3-catalog-manifest-residue.md`, `📓️b5-capability-without-manifest.md`,
`📓️b4-runtime-inventories.md`, `📓️a9-mutation-catalog-integrity.md` and `📓️a10-oracle-honesty.md` in
full before touching anything, per the shard brief.

## Before / after (measured, `bun ./📜️script.ts test contract`, foreground, at session start and end)

| id | before | after | disposition |
| --- | ---: | ---: | --- |
| `capability-without-manifest` | 6 | **1** | 5 of 6 `✳️any→✳️base` renames closed (zip, pptx, ifc@2x3, xlsx, step@ap214); `semio@v1` deferred, see §1 |
| `mutation-catalog-unclaimed` | 8 | **8** | unchanged — gltf ×8, needs real oracle engineering, see §2 |
| `unregistered-mutation-vocabulary` | 13 | **13** | unchanged — re-verified, structurally blocked, see §3 |
| `no-oracle-covers-mutation` | 5 | **0** | closed, see §4a |
| `reimplementation-registered-as-third-party` | 2 | **2** | unchanged — needs a framework rule change, handed to D1, see §4b |
| `fixture-file-missing` | 1 | **0** | closed, see §5 |
| `wildcard-subset-owner` | 0 | **0** | guard, stable |
| `duplicate-mutation-owner` | 0 | **0** | guard, stable |
| `missing-fixture` | 0 | **0** | guard, stable (transient spike to 4, all `s.stdio.space.home`, a concurrent session's own in-flight relocation — confirmed via `git status` showing that session's uncommitted add/delete/rename mid-flight; not mine, settled back to 0 by session end) |
| `orphan-fixture` | 0 | **0** | guard, stable |
| `test-only-mutation` | 0 | **0** | guard, stable (I transiently reopened this to 4 myself — see §1's honesty note — and closed it back out before finishing) |
| `no-scenarios` | 0 | **0** | guard, stable |
| **TOTAL breach count** | **1953** | **1088** | net −865; the overwhelming majority (`missing-external-oracle` alone fell ~888, from a concurrent session's own oracle-registration work I did not touch) is not attributable to this shard — see the note below |

The repo is under heavy concurrent load from other sessions throughout this shard (`missing-fixture`
transients, a `unregistered-mutation-vocabulary` count that stayed exactly at B4's 13 while
`missing-external-oracle` fell by nearly 900 elsewhere, `s.stdio.semio@v1`'s uncommitted diff
shrinking from 373 files to 13 mid-session). Every number in the table above is a live
re-measurement at hand-off time, not an assumption.

## 1. `capability-without-manifest` — the `✳️any` → `✳️base` renames (5 of 6 closed)

B5 diagnosed this precisely and C3 confirmed the diagnosis but declined to attempt it (no compile
verification available, ~1,000 files, one target actively being restructured by another session).
Both were right to be cautious — the rename turned out to have a THIRD layer neither of them
surfaced, on top of the two they already knew about:

1. **Directory rename** (`✳️any` → `✳️base`), **`🪆️subsets/🔣️.json`** (`"*"` key → `"base"`, matching
   `📄️pdf`'s own precedent exactly — verified against `📄️pdf`'s own file before touching anything).
2. **Every file's own text** referencing the literal emoji-prefixed string `✳️any` (JSON `owner`
   fields, Rust doc comments, fixture paths) — mechanical, safe, scoped per artifact.
3. **THE PART NEITHER B5 NOR C3 FOUND: a ~14,000-line central Rust wiring file**
   (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`, plus a smaller twin in the oracle crate,
   `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/📦️lib.rs`) that declares the ENTIRE crate's
   module tree via nested `#[path = "…"]` mounts, including a literal `pub mod any { … }` Rust
   module wrapping every subset's `schema`/`io`/`viewer`/`editor` facets, PLUS a repo-wide "shim"
   convention (`pub use super::standards::<ver>::subsets::any::schema::*;`) that lets legacy
   `crate::artifacts::<x>::schema::*` paths keep resolving, PLUS (for two of the five, discovered
   only by making the compiler say so) a `crate::editor::zip::any::…` / `crate::viewer::zip::any::…`
   editor/viewer-registry path that isn't derived from the taxonomy at all — someone typed `any` by
   hand when they wired zip's editor windows. **None of this three-layer structure is visible from
   the JSON/TypeScript side the ticket's own judge (`test contract`) measures** — it is pure Rust
   compile-time wiring, invisible to the gate, and exactly why B5/C3 were right to want "room to also
   touch and recompile the Rust dialect constants" before attempting it blind.

**What made this attemptable now that wasn't earlier**: the repo-wide `semio-framework-plugin` build
that blocked B4's whole shard is fixed (confirmed: `cargo check -p semio-framework-plugin --offline`
now finishes clean, only warnings — someone else's in-flight serde migration, referenced in B4's
report, has landed). That gave me a real compiler to argue with, which is what actually found layer 3
above; I would not have found it by reading alone.

**One real mistake made and caught mid-repair**: a scoped text-replace script I ran with `find
✏️s/…/🏗️ifc` (the whole `ifc` artifact, not just `2x3`) leaked 20 file edits into `ifc@4`'s untouched
`✳️any` subset before I'd even started the wiring-file work. Caught immediately via
`git diff --stat` on the exact directory, reverted with `git checkout --` on just those 20 paths (my
own same-turn, uncommitted mistake — not another session's live work, so safe per the ticket's own
distinction). A second, subtler instance of the identical mistake happened INSIDE the wiring file
itself: a 14-line-number-targeted patch for the `super::subsets::any::…` shim pattern mis-attributed
two lines to `step` that actually belonged to `ifc@4`'s own `v4` engine shim (the two blocks sit at
deceptively similar relative offsets within their respective enclosing scopes). Caught by re-deriving
the real block boundaries with `awk` and comparing against the file's ACTUAL content rather than my
earlier line-number bookkeeping, reverted those 2 lines, then correctly finished the real fix. Both
mistakes are disclosed here rather than left for a diff to discover, per this ticket's own culture
(see C3's §0).

**Verification, in order, each one real:**
- `python3 $TICKET/🔍️check-mutation-leaf-ownership.py`, filtered to the 5 artifacts: **0 problems**
  (baseline was already noisy — 929 pre-existing problems entirely in `🌀️procedural3d`/`🏛️program`,
  neither touched by me — confirmed by filtering the tool's own output to my 5 artifacts specifically).
- `cargo check -p semio-s-plugin-stdio --offline` (isolated `CARGO_TARGET_DIR`, `RUSTC_WRAPPER=""`,
  per house rules): iterated from 22 real errors down to **zero** errors attributable to zip, pptx,
  ifc@2x3, xlsx or step@ap214. The build's ONE remaining failure
  (`semio-framework-3d`, "cannot find attribute `serde`") is a different, unrelated, concurrent
  session's own in-flight migration — confirmed via `git status` showing uncommitted modifications in
  `🧰️framework/🔨️modules/🧊️3d` I never touched, and confirmed the same failure mode B4 already
  documented for a sibling crate.
- `bun ./📜️script.ts test contract`: `capability-without-manifest` 6→1,
  `wildcard-subset-owner`/`duplicate-mutation-owner`/`unsplit-artifact-subset`/
  `mutation-manifest-invalid`/`oracle-capability-mismatch` all confirmed 0 both before and after.
- Manifests for the 5 written via a small script kept in this folder
  (`🔨️d2-write-base-manifests.ts`, same `manifestFromLeafDescriptors` merge logic B5's own leftover
  tool used, retargeted at the 5 post-rename capabilities) — `--dry` run first (0 skips, all 5 ready),
  then applied for real.
- **Regression caught and fixed within this same session**: writing the 5 new manifests
  mechanically re-opened the exact `no-mutation`-control-row pattern C3 already solved elsewhere
  (§1a of `📓️c3-catalog-manifest-residue.md`) — for these 5 specifically, since they never had a v2
  manifest before, the comparison had simply never run. Reused C3's own left-behind tool
  (`🔨️c3-fix-no-mutation-scenarios.py`) on the 4 affected feature files (`step-ap214-base` never had
  the pattern — C3's own earlier pass on `step-ap214-any`'s catalog already stripped it, and that fix
  carried through the rename), dropped `no-mutation` from the 4 catalogs' `kinds`. Verified
  `test-only-mutation` 4→0.

**`semio@v1` deferred, not attempted.** Confirmed 373 uncommitted paths under its `🏅️standards/🔖️v1`
tree at session start (another session's live restructuring, matching C3's own observation), settling
to 13 by session end — still real, still concurrent, still a different session's own subset-level
catalog work in files I never touched. At 238 files and 19 real sibling subsets it is also, by a wide
margin, the largest and structurally most complex of the 6 (18 real subsets beyond `presentation` and
`brep`, versus 2–7 for the other five). The technique is now fully proven — directory + JSON + the
3-layer Rust wiring, compile-verified end to end — which makes a future attempt materially lower-risk
than when B5/C3 assessed it, but attempting a 6th, largest, most-actively-touched rename after an
already-long session was a deliberate stop, not an oversight. **This is the one clearly-scoped
follow-up item from this shard.**

### Files touched (renames)

For each of `🎒️zip@2.0`, `🎞️pptx@ecma-376`, `🏗️ifc@2x3`, `📕️xlsx@ecma-376`, `📐️step@ap214`:
- `🪆️subsets/✳️any/` → `🪆️subsets/✳️base/` (directory move).
- `🪆️subsets/🔣️.json`: `"*"` → `"base"` key, name text updated to the `📄️pdf`-style "base surface"
  phrasing.
- Every file under the artifact's own standard directory referencing `✳️any` (owner fields, doc
  comments): text-replaced to `✳️base`. Also the artifact's own catalog `id`/oracle `id`s that
  embedded `-any` (`ifc-2x3-any`→`ifc-2x3-base`, `step-ap214-any`→`step-ap214-base`,
  `ruststep-ifc-2x3-any-mutate`→`…-base-mutate`, `ifcopenshell-ifc-2x3-any-differential`→`…-base-…`,
  etc.) — `zip`/`pptx`/`xlsx`'s own capability strings never embedded `-any`, so nothing to rename
  there.
- New `mutationManifests` entries (`🔨️d2-write-base-manifests.ts`) declaring the 5 renamed subsets'
  mutations, `qualifyingKind: third-party-library` (honest — none of the 5 owners has one yet; this
  is the expected `missing-external-oracle` trade, same as B5's own 83).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs` (the central wiring file): every `#[path]` string
  targeting one of the 5 renamed subsets, the 5 corresponding `pub mod any {` → `pub mod base {`
  (nested-schema style, verified against every remaining `pub mod any {` in the file to confirm none
  was missed and none of the 31 renamed blocks belonged to an untouched artifact), the 5 artifacts'
  `super::standards::<ver>::subsets::any::…` and inline `super::subsets::any::…` "shim" `pub use`
  lines, and zip's own hand-typed `editor::zip::any::…` / `viewer::zip::any::…` editor/viewer-registry
  paths.
- `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/📦️lib.rs`: the same `pub mod any {` → `pub mod
  base {` + path-string fix, 5 blocks.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🦀️.rs`,
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🦀️.rs`: one stray artifact-root doc-comment `✳️any` each.
- 141 further `🦀️.rs` files across the 5 artifacts' own subtrees carrying the fully-qualified
  `crate::artifacts::<x>::standards::<ver>::subsets::any::…` form (repo-wide scoped substring
  replace, one exact prefix per artifact+standard — verified never to collide with an unrelated
  artifact sharing a version identifier, e.g. `pptx`/`xlsx` both use `v_ecma_376`).
- Deliberately left unchanged everywhere: the Rust runtime `SubsetId("*")` / `Dialect` constants
  (confirmed this is `📄️pdf`'s own precedent too — its `PDF14_DIALECT` still says `SubsetId("*")`
  despite its directory and Rust module both being `base`; the wildcard test-registry string and the
  wildcard Rust dispatch value are two independent axes), and the cosmetic `ZipAnyEditor`/
  `create_zip_any_editor`-style type/function NAMES (harmless leftover naming, not a path segment,
  doesn't affect compilation or the gate).

## 2. `mutation-catalog-unclaimed` (8, gltf) — re-verified, still correctly left open

Re-read C3's §2 in full and spot-checked its factual claims against the live tree rather than trusting
them blind: `✳️any/🧪️tests/mutate-gltf-2-0` still covers exactly 7 of 118 kinds (confirmed by reading
its own docstring, unchanged), the v2 manifest still declares all 118 with real per-subset
attribution, and the 8 per-subset catalogs (`animation`/`asset`/`buffer`/`camera`/`material`/`mesh`/
`scene`/`skin`) are still real, populated, unclaimed vocabularies sitting beside real fixture
directories. Nothing on disk changed since C3's pass that would let a fresh attempt succeed where
theirs correctly declined to force one.

**Left open, all 8, for the same reason C3 gave and I independently confirm**: closing this honestly
needs 8 new subset-level test cases with real, hand-verified oracle semantics for the 111
not-yet-covered kinds (glTF's `mesh`/`primitive`/`morph-target` vocabulary alone is 34 kinds) — real
engineering, not catalog bookkeeping. A case with `deferredKinds` covering all 118, or an oracle that
doesn't actually check anything, would satisfy the gate while producing exactly the hollow outcome
this ticket's second law exists to prevent. Not attempted here, for the same reason C3 didn't: it is
squarely a dedicated pass's worth of work, not something to improvise alongside five compile-verified
Rust renames in one shard's remaining budget.

## 3. `unregistered-mutation-vocabulary` (13) — re-verified, all three prior shards' dispositions confirmed unchanged

Read A9, B4 and C3's investigations of this id in full (A9 found the structural root cause and fixed
30 of the original 37; B4 re-verified A9's 33 leftover and investigated 10 post-split "should be
empty" candidates that turned out not to be empty; C3 re-verified both and reported the 43→13 drop
as a concurrent session's own progress, unrelated to any of the three shards' own work).

Current 13 = the same 3 gis + 10 post-split entries B4/C3 already catalogued, confirmed by direct
comparison of today's live breach scopes against C3's own final list. Re-derived the mechanism myself
from the current source (not by re-reading the prior reports' prose) to make sure nothing had drifted:

- **3 gis editor-state owners** (`gisterrain`'s `✏️editor/🎚️config`, `gismap`'s `✏️editor/👥️presence`
  and `✏️editor/🎚️config`): `mutationCatalogProblems`
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:658`) forces any
  owner whose path contains `/🏅️standards/` into "profiled" mode, which then requires
  `owner.endsWith('/🏅️standards/${std}/🪆️subsets/${subset}')` exactly — an owner with trailing
  segments past the subset root (`…/✏️editor/🎚️config`) can never satisfy an `endsWith` anchored at
  the subset root. Confirmed this line is byte-identical to A9's own read. No compliant catalog is
  representable at the walker-computed owner, full stop.
- **10 post-split note/draw/mathematical/sequence/fem2d/fem3d entries**: read B1's own report
  (`📓️b1-per-subset-catalog-scoping.md`) to confirm it had actually landed (it had — every real
  subset of note/draw/mathematical/sequence now owns its own correctly-scoped v1 catalog +
  capability). The remaining breach is specifically the ARTIFACT-LEVEL `✳️any/🧬️schema/🧬️mutations`
  directory for each of these 6 artifacts, which still holds LIVE, SHARED code (the artifact-wide
  aggregate mutation enum + wire codec every subset's own leaf imports) but by design now owns ZERO
  v1 catalog of its own — B1 deliberately emptied it once ownership moved to the real subsets. I
  traced the exact walker logic myself
  (`mutationVocabularyRequiresCatalog`/`unregistered-mutation-vocabulary` at `🟦️.ts:1794` and
  `:1817`): the check requires this OWNER's OWN `mutationCatalogs.length > 0`, and does not care
  whether a DESCENDANT subset's catalog already claims the same kinds. Registering a second,
  artifact-level catalog here that duplicates kinds the real subset catalogs already correctly own
  would need either genuinely new scenarios+fixtures for every kind (substantial, redundant
  engineering — the same proportion problem as gltf above) or a `deferredKinds`/empty-oracle shortcut
  that is exactly the hollow pattern this ticket exists to prevent. `s.fem.2d`/`s.fem.3d` are a
  slightly different sub-case (their `✳️any` genuinely still owns all 25 mutations in the v2
  manifest, not yet migrated into the 5 already-scaffolded subset dirs the way note/draw/mathematical/
  sequence were) — same walker mechanism, same conclusion.

**Left unregistered, all 13**, per A9/B4/C3's own reasoning, now independently re-derived from the
walker's current source rather than re-asserted from their prose. The concrete, correctly-scoped
follow-up (not attempted here, and arguably framework territory rather than any one shard's): teach
`mutationVocabularyRequiresCatalog`'s `claimed` check to also accept an owner whose OWN kinds are a
subset of what its DESCENDANT subset catalogs already correctly claim — recognizing "shared code a
family of subsets imports" as a legitimate zero-catalog state, the same way A9 recommended exempting
framework-internal fixture trees with no `.feature` surface at all.

## 4a. `no-oracle-covers-mutation` (5 → 0)

All 5 (`binary-raw-mutate`, `jpg-jfif-1-01-baseline-mutate`, `dwg-ac1018-mutate`, `dwg-ac1024-mutate`,
`tiff-6-0-baseline-mutate`) were the exact same shape A10 already fixed for
`mathematical`/`sequence`/`draw`: a pre-existing, architecturally sound `noOracleDecisions` entry
(rationale: no credible third-party library exists for a domain-less raw byte buffer / a proprietary
CAD container) collided with a v2 `mutationManifest` that a mechanical leaf-descriptor write (B5's or
C3's) had since given the same capability, complete with a hard `qualifyingKind: third-party-library`
requirement. Read `noOracleMisuseBreaches`
(`🟦️.ts:5091`) directly: it fires whenever ANY `noOracleDecisions[].capabilities` entry appears
anywhere in `registry.mutationManifests` (`mutation.capability` or any `oracleRequirements[].capability`),
regardless of whether the manifest existed when the decision was written — a decision can never stand
in for a real, live oracle requirement.

Applied A10's own precedent exactly: narrowed each decision's `capabilities` to `[]`, appended a note
explaining why (kept the original rationale text intact, since it still correctly explains why no
third-party library is credible for these 5 domains — the gap this creates is expected to STAY open,
not get filled). Verified: all 5 files carry 0 `oracles` entries (confirmed the underlying gap is
real, not just newly-exposed), and the resulting diffs are minimal (2 lines changed per file,
formatting untouched — learned from C3's own incident about re-serializing whole JSON files).

## 4b. `reimplementation-registered-as-third-party` (2, both `🏗️ifc`) — confirmed unchanged, handed to D1

Both are exactly A10's own documented, deliberately-left-open finding: `ifc/2x3/✳️base` (post-rename)
and `ifc/4/✳️any` each genuinely carry BOTH a hand-written predicting dispatch (correctly flagged) AND
a real, independently-produced `ifcopenshell-*-differential` third-party oracle (a real Python script
shelling out to IfcOpenShell 0.8.4.post1, verified by A10 with a measured byte-level round-trip). The
detector (`reimplementationOracleBreaches`) operates at file granularity
(`join(contribution.owner, "🧪️oracle", "🦀️.rs")`) rather than per-oracle-entry, so the legitimately
independent `ifcopenshell-*` registration gets swept in with the file's OTHER (correctly-flagged)
`ruststep-*` entry. **Per this shard's own brief, I did not touch
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` — shard D1 is
concurrently changing the oracle qualifying-kind rules there.** The sanctioned fix
(`judgedByProbes`: registered `probes` + `comparisonPipelines` with a `qualified` status, the same
shape as the PNG precedent) is a rule-level schema retrofit, not a per-file data fix — **handing this
to D1**: the fix is either (a) make `reimplementationOracleBreaches` operate per oracle ENTRY instead
of per file, or (b) land the `judgedByProbes` escape hatch for these two. Confirmed my rename of
`ifc@2x3`'s `✳️any`→`✳️base` did not change this breach's shape at all (still exactly 2, same file
identities, just the directory renamed underneath — re-verified the breach scope path updated to
`…/✳️base/🦀️oracle.rs` correctly and the count stayed 2, not 3 or 1).

## 5. `fixture-file-missing` (1 → 0)

`📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🧪️oracle/🔣️.json`'s `real-world-hexagonal-cut-concrete-
forest-left` fixture manifest pointed at
`../../../../../🧫️fixtures/🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp` (5 `..` segments, i.e.
`📐️step/🧫️fixtures/…` — an artifact-root fixtures directory that does not exist). Every SIBLING
`fixtureManifests[].files[].path` in the same file uses `../🧫️fixtures/…` (1 `..`, i.e.
`✳️cc6/🧫️fixtures/…`, where the file genuinely lives — confirmed present, and its SHA-256 matches the
manifest's pinned digest exactly, byte-for-byte). A10 had attempted the identical fix earlier in the
ticket (their §"Stragglers" entry references the same fixture) but the live file still showed the
broken 5-`..` path at this shard's session start — either A10's fix didn't land, or a later edit
(the "another live session physically moved semio's tests" churn C3 flagged, or similar) reverted it.
One-line path correction, re-verified against the sha256 before touching it.

## Files touched (schemas — item 6, partial, honestly disclosed)

`📐️step`'s 38 mutation directories across `✳️base`/`✳️cc1`…`✳️cc6` all declare
`"payloadSchema": "🔣️.schema.json"` with no such file on disk — a real gap this shard's brief asked
me to close, though it does not currently trip the gate (`payloadSchema` is not existence-checked by
`test contract` today — confirmed by grepping the live breach set for anything step/schema-related:
nothing). Authored the 10 missing schemas for `✳️base` (the foundational ISO 10303-21 envelope
mutations every conformance class re-exports — `set-snapshot`, `remove-entity`, `insert-entity`,
`insert-entity-arg`, `remove-entity-arg`, `set-entity-arg`, `set-entity-name`, `set-file-schema`,
`set-file-description`, `set-file-name`), read directly from each mutation's own `🦀️.rs` struct and
`StepSnapshot`/`StepValue`/`StepEntity`/`StepHeader`/`StepFile{Schema,Name,Description}`'s real field
types and `#[value(rename_all = "camelCase")]` attributes (confirmed the mutation PAYLOAD structs
themselves carry no `rename_all`, so their own top-level field names stay snake_case; only the NESTED
`Step*` types are camelCased — verified against `value_derive`'s own `field_wire_name` source, not
assumed). Draft-07, `$ref`/`$defs`, matching `📷️png`'s own exemplar shape exactly (confirmed PNG uses
`$ref` throughout despite one shard-brief note elsewhere suggesting otherwise for a different
artifact's style choice).

**The 28 remaining (`✳️cc1`×4, `✳️cc2`–`✳️cc5`×5 each, `✳️cc6`×4) are NOT done.** Each conformance
class's `set-snapshot`/`set-product-identity`/`set-shape-representation`/`demote-shape-
representation` share KIND NAMES across subsets but not necessarily payload SHAPES (each subset's own
`StepSnapshot`-family types can differ), so authoring these honestly needs the same close-read-then-
write pass as `✳️base`, six more times — real, bounded, mechanical work, but a further ~28 struct
reads plus schema authoring is a second session's worth on its own, and the ticket's second law
("give each mutation a fixture-backed test vector") for even the 10 I did author is only partially
met: I verified the schemas are structurally accurate against the Rust source, but did not add new
`🧪️tests/<case>` directories under each mutation's own tree in the PNG exemplar's full shape (most of
`✳️base`'s mutations already have real coverage via the subset-level `🧪️oracle/🔣️.json`
`fixtureManifests` + the artifact-level `mutate-step-ap214` case's Examples tables — a different,
pre-existing evidence shape, not the per-leaf `🧪️tests/<case>/{🦠️mutation,📸️snapshot,🔺️diff,
🎯️outcome}` one the exemplar shows). Flagging both gaps explicitly rather than claiming the exemplar
shape was reached.

Files: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/
🧬️mutations/{📄set-snapshot,🗑remove-entity,🧩insert-entity,➕insert-entity-arg,➖remove-entity-arg,
🔧set-entity-arg,✏set-entity-name,🏷set-file-schema,📝set-file-description,📛set-file-name}/
🔣️.schema.json` (10 new files).

## Scratch scripts kept in this ticket folder (per house rules)

`🔨️d2-write-base-manifests.ts` (the 5-artifact manifest writer, same merge logic as B5's own
`🔨️b5-write-manifests-from-leaves.ts`, retargeted), `🩹️d2-narrow-no-oracle-decisions.py` (the §4a
`noOracleDecisions.capabilities` narrowing script, A10's precedent applied to the 5 new collisions),
`🔨️d2-gen-step-base-schemas.py` (the §"schemas" generator — the `$defs` for `StepValue`/`StepEntity`/
`StepHeader`/`StepFile{Schema,Name,Description}`/`StepSnapshot` plus the 10 mutation schemas; reusable
almost as-is for the 28 `cc1`–`cc6` schemas still outstanding, once each subset's own struct shapes
are read). The line-targeted Rust wiring-file patches (the 14-line-number shim fixes, the `pub mod
any`→`pub mod base` scoped rename, the 5-prefix path-string replace) were single-use, run inline and
not kept as standalone tools — they were artifact-specific line-number edits, not a reusable pattern
like B5/C3's tools.

## Final answer

**Fully closed this shard:** `no-oracle-covers-mutation` (5→0), `fixture-file-missing` (1→0), and
`capability-without-manifest` for 5 of its 6 remaining owners (6→1) — the last being the sole
follow-up item with a concrete, now-far-lower-risk path (`s.stdio.semio@v1`, `✳️any`→`✳️base`, same
proven 3-layer technique: directory + JSON, then the central Rust wiring file's `pub mod any`/shim
paths, then any hand-typed editor/viewer registry paths, verified with
`🔍️check-mutation-leaf-ownership.py` + `cargo check -p semio-s-plugin-stdio`).

**Investigated and honestly left open, with re-derived (not re-asserted) evidence:**
`mutation-catalog-unclaimed` (8, gltf — needs real per-kind oracle engineering across 111 kinds, a
dedicated pass's worth of work), `unregistered-mutation-vocabulary` (13 — structurally blocked by the
test-registry walker's own design, needs a framework rule change to recognize "shared code with
zero-catalog-by-design descendants" as legitimate), `reimplementation-registered-as-third-party` (2,
both `🏗️ifc` — needs the oracle-qualifying-kind rule change D1 is already mid-flight on; explicitly
not touched per this shard's own brief).

**Guard classes confirmed stable throughout** (0→0, including one transient regression I introduced
and fixed within the same session): `wildcard-subset-owner`, `duplicate-mutation-owner`,
`missing-fixture`, `orphan-fixture`, `test-only-mutation`, `no-scenarios`, `unsplit-artifact-subset`,
`mutation-catalog-capability-mismatch`, `mutation-manifest-invalid`, `oracle-capability-mismatch`.

Before → after, repo-wide breach total: **1953 → 1088** (net −865; the great majority is concurrent
sessions' own progress — `missing-external-oracle` alone fell ~888 from work this shard never
touched — not a claim about this shard's own size of contribution, which is fully itemized above).

Deliverable: this file,
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️d2-final-residuals.md`.
