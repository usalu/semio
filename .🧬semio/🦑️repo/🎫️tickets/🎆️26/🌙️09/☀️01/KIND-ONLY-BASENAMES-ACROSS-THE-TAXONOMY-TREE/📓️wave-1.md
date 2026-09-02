# 🌳️ Wave 1 — 13 families migrated, and the gate can run again

## What moved

| family | files renamed | references repointed |
|---|---|---|
| `🦀️component.rs` → `🦀️.rs` | 8,036 | 14,245 |
| `🟦️component.ts` → `🟦️.ts` | 4,318 | 4,125 |
| `🔗️component.graphql` → `🔗️.graphql` | 1,488 | 909 |
| `🛰️component.proto` → `🛰️.proto` | 1,487 | 1,105 |
| `📖️component.grammar.semio` → `📖️.grammar.semio` | 491 | 1,363 |
| `📡️component.protocol.semio` → `📡️.protocol.semio` | 449 | 1,228 |
| `🌶️`/`🔠️`/`🥋️`/`🅰️`/`🔤️` `component.<ext>` | 448 each (2,240) | 81 |
| `🟦️component.tsx` → `🟦️.tsx` | 103 | 606 |
| `🚫️component.absent` → `🚫️.absent` | 71 | 135 |

**~18,700 files renamed, ~24,400 references repointed.** Every target name was checked against
`fileKinds` first — all 20 are registered; nothing was invented.

## Three corrections I had to make to my own work

1. **The migrator never updated root-level files.** Its `roots` list included `📜️script.ts` but then
   filtered to `os.path.isdir`, silently dropping it. Result: `📜️script.ts` still imported
   `🟦️component.ts` and the whole CLI died with `Cannot find module`. Fixed by a repo-wide reference
   pass (602 refs in 33 files) — references live outside the renamed trees, so the reference scope must
   be wider than the rename scope.
2. **Blanket renaming over-reached into module collections.** A package-internal directory like
   `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu` holds `🦀️action.rs`, `🦀️arena.rs`, … where the slug IS
   the identity, so its `🦀️component.rs` was a module named "component", not a component leaf.
   Measured against HEAD: **7 such directories out of 8,040** (the other 8,033 were genuine leaves).
   All 7 reverted, 170 reference files repointed back.
3. **I rewrote three frozen records I should not have touched.** `frozenCoordinateEvidenceContracts`
   pin exact bytes AND byte offsets; shortening a path by 9 characters shifts every later coordinate.
   Two evidence fixtures and one `.cursor` plan doc were restored from HEAD. Frozen evidence is a
   RECORD of a past state — it must be excluded from mechanical passes, not repinned.

Also: 4 `sourcePathIdentities` contracts listed BOTH the legacy and canonical path (mid-migration
state); after the rename those collapsed to duplicates and were deduped. One `authorityCatalogSha256`
was repinned — verified first that the HEAD digest equalled the old pin, so the drift was provably mine.

## The gate: from "cannot run" to a full report

Before this wave `bun 📜️script.ts verify taxonomy report` threw before doing any work:

    Normalization requires an explicit repository-boundary decision before authored classification: ♻️mit-bestand/recherche

That guard was added by today's commit `67fb4216b2`, and the repo's own design notes
(`📓️admission-gitlink-57-contract.md`, `📓️gitlink-inventory-refusal-59.md`) name the
declare-and-proceed mechanism as **deliberately unbuilt** — RED-state TDD belonging to another session.
So it was left alone rather than weakened, and `--scope` (which filters observations before the
boundary check) was used instead.

Scoped to `🧰️framework` the gate now runs and reports:

| code | count |
|---|---|
| `reference-edit-required` | 2,889 |
| `normalization-move-required` | 2,699 |
| `frozen-coordinate-evidence-unowned` | 1,541 |
| `semantic-stem-unresolved` | 330 |
| `directory-kind-unresolved` | 279 |
| `reference-syntax-unsupported` | 217 |

**`normalization-move-required` means the tool has its own planner** — it knows the moves it wants.
The remainder of this migration should drive that planner rather than extend the ad-hoc script.

Pre-existing and NOT from this wave: the `app/api/v1/*/route.ts` collisions are Next.js app-router
files whose names that framework imposes.

## Not migrated, deliberately

* **330 `🔣️.json` + `🔣️component.json` pairs** — not duplicates. Hashing showed all 330 differ, same
  key set, different values, the legacy file richer (`binaryTag`, `textOpcode`, more surfaces). Needs
  per-artifact judgement.
* **Asset/member collections** (`🔣️<icon>.svg`, `🔤️<subset>.woff2`, `🦀️<module>.rs`) — the slug is the
  identity there; the policy engine already treats `🖼️assets` members as structural.

# 🌊️ Wave 2 — the bulk is done

| family | renamed | refs repointed |
|---|---|---|
| `🔣️component.json` → `🔣️.json` | 9,333 (330 collisions correctly skipped) | 13,264 |
| `🔣️payload.schema.json` → `🔣️.schema.json` | 1,670 | 3,736 |
| `🦀️test.rs` → `🦀️.rs` | 125 | 56 |
| `🟦️test.ts` → `🟦️.ts` | 122 | 10 |
| `🗣️example.dsl.semio` → `🗣️.dsl.semio` | 99 | 452 |
| `🟨️host-shim.js` → `🟨️.js` | 66 | 159 |
| `🎮️demo.cmd.semio` → `🎮️.cmd.semio` | 52 | 159 |
| `🟦️glue.ts` → `🟦️.ts` | 6 | 50 |

## Where the count actually stands

| | start | now |
|---|---|---|
| canonical basenames in the governed trees | 8,351 | **38,502** |
| emoji-prefixed semantic basenames | 33,372 | **3,223** |
| — of those, asset/member collections (NOT breaches) | — | 2,202 |
| **real remaining breaches** | 33,372 | **1,021** |

The 2,202 are `🔤️latin.woff2`, `🔣️<icon>.svg` and similar, where the slug IS the identity and
`policyEmojiSiblingIdentityIsStructural` already treats the members as structural. Collapsing them to
`🔤️.woff2` would destroy the collection, so they are not breaches and were never touched.

Of the 1,021 real ones: 330 json descriptor pairs, ~250 multi-sibling families, and a ~440 long tail
(`🦀️backend.rs`, `🦀️surface.rs`, `🎨️globals.css`, `🌐️index.html`), some of which will turn out to be
module collections under the same sibling test that spared the 7 in wave 1.

## Contract maintenance the renames forced

Renaming files invalidates four kinds of pinned authority, each of which had to be repaired honestly:

* `authorityCatalogSha256` — repinned, but only after verifying the HEAD digest equalled the OLD pin,
  which proves the drift was mine and not someone else's.
* `sourcePathIdentities` — contracts listing BOTH legacy and canonical paths collapsed to duplicates.
* `sourcePathPattern` — regexes still naming `📦️glue\.rs`. **A first attempt to fix these silently
  did nothing**: I edited the raw JSON text, where a regex backslash is stored escaped as `\\.`, so my
  pattern never matched. Caught by re-running the gate and seeing the identical error; fixed on the
  parsed object instead.
* `generatorContracts.inputPatterns` — must be unique AND lexically ordered.

## Statute status

No new statute was needed — `physicalLeafRendering` and `_treePurityComment` already state the rule,
and `fixedFilenameContracts` / `configurableEntryContracts` already make "mandatory" decidable rather
than a matter of taste. What was missing is ENFORCEMENT REACH, and the gate now runs where it
previously threw before doing any work at all.

# 🌊️ Wave 3 — the collisions and the unregistered kind

## Resolved by fleet

**330 collided descriptor pairs.** Authority was settled by evidence, not preference: the legacy
`🔣️component.json` files were hand-authored in commit `a8d1caf`, the canonical `🔣️.json` files were
generic scaffold stubs from `f7b265d`/`67fb4216`. The decisive cross-check was blind — comparing each
file's `requiredLanguageSurfaces` against the files actually present on disk: **legacy matched in
307/330 directories, the stub in 0/330**. One field went the OTHER way (`payloadSchema` in 23 pdf
directories, where the legacy value named no existing file), which is what a real check looks like as
opposed to a blanket rule. All 330 merged, validated against the derive's rules, 0 undecidable.
Verified independently here: **0 `🔣️component.json` remain**.

**5 sibling families executed and test-verified** — `🔣️cases.json` (21) and `🔣️vectors.json` (15) to
bare `🔣️.json`; `🔣️wire.schema.json` (14) into a registered `🧬️wire/` directory, which is the
statute's OWN remedy ("semantic concerns live in registered emoji-plus-slug directories");
`🔣️schema.json` (9) to `🔣️.schema.json` after confirming they are genuine draft-07 schema documents
mis-extensioned; `🟦️.test.ts` (19) to `🟦️.ts` after confirming the runner invokes them by literal path,
not by `*.test.ts` glob. That last check mattered — a glob would have silently stopped running them.

## A missing statute, found and written

`🛂️descriptor.semio` (89 files, 44 tracked) had **no `🛂️` entry in `fileKinds` at all** — the files
were not merely misnamed, they were unclassifiable. Registered following the existing `.semio` pattern
(`📖️.grammar.semio`, `🎮️.cmd.semio`, `🗣️.dsl.semio`):

```json
"semio-descriptor": { "emoji": "🛂️", "extensionChains": [".descriptor.semio"], "role": "asset" }
```

plus the `fileKindResolutionRules` entry the schema requires (it rejected the kind until the rule
existed — the registry is self-checking). Both twins then moved together: 89 `🛂️.descriptor.semio`
and 89 `🔣️.json`.

## Two families correctly NOT migrated

* **`🔣️typology.json` / `🔣️modelDefinition.json` / `🔣️transformation.json` (57)** — pinned by
  `semanticPathProjectionCatalogContracts["cad-model-catalog-v1"]`, whose golden oracle is one of the
  frozen fixtures that must not be edited. The agent executed the rename, discovered the pin, and
  **reverted byte-for-byte**. Blocked, not decided against.
* **`🟨️plugin-worker.js`** — 58 on disk, **0 tracked**. Gitignored build-cache output that the current
  tooling deletes on sight as proven-stale. Never a repository breach.

## Where it stands

| | start | now |
|---|---|---|
| canonical basenames | 8,351 | **38,926** |
| true leaf breaches | 33,372 | **~130 tracked** |
| structural collections (slug IS identity) | — | 356 |

What remains is a tail of singletons and genuine content identity — `🖼️abbau-aufbau-masterarbeit-
grundriss.jpg`, `🗣️nakagin-capsule-tower.dsl.semio` — where the slug names the content, exactly as
`🔤️latin.woff2` does. Collapsing those to `🖼️.jpg` would destroy the identity the file exists to carry.

# 🌊️ Wave 4 — the last class, and why it is staged rather than executed

`📦️glue.rs` **106 → 0** and `📦️index.ts` **57 → 2**, with every `Cargo.toml` `[lib] path` /
`[[bin]] path` and `package.json` `exports` repointed. Independently verified here:
`cargo metadata --no-deps --offline` **resolves the whole workspace**, so all 106 manifests still parse.
This is the `configurableEntryContracts` path the taxonomy itself declares — the entry file is
configurable, so renaming it and updating the declared `configurationSources` is the sanctioned move,
not a workaround.

## What the remaining 121 tracked breaches actually are

Triaged rather than counted:

| class | count | disposition |
|---|---|---|
| named fixture specimens in `🧫️fixtures/` | ~105 | statute remedy known, staged below |
| genuine convention names | ~15 | `🟦️vitest.setup.ts`, `🔣️retirement.json`, `🔣️fixture.schema.json`, `🟦️board-session.ts`, … |
| slug containing a dot | 1 | `🔣️hexagonal-cut-concrete-forest-left.model.json` — `.model.json` is not a registered chain |

## The fixture-specimen remedy is known and conformant

The statute's own sentence supplies it: *"semantic concerns live in registered emoji-plus-slug
directories."* `semanticDirectoryKinds.test-fixture-member` registers exactly that — emoji `🧪️`, an
OPEN slug pattern (`^[\p{L}\p{N}]+(?:-[\p{L}\p{N}]+)*$`), so no registry addition is needed. (Note
`members-of-fixtures` is a CLOSED 5-name registry, but it governs only those five special cases, not
the general member rule — 1,505 specimen directories already exist under the open pattern, against 383
bare files.) The precedent is exact:
`🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json`.

Dry-run: **44 specimens move cleanly**, 1 rejects on a dotted slug.

    🖼️rathaus-ahlen-grundriss.bmp  ->  🧪️rathaus-ahlen-grundriss/🖼️.bmp
    🔊️bauen-mit-bestand-ausschnitt.wav -> 🧪️bauen-mit-bestand-ausschnitt/🔊️.wav

## Why it is NOT executed in this wave

Each specimen carries 5-22 references and **many live in Rust files** (`🏗️nakagin-capsule-tower.ifc`
alone: 22 references, 5 of them `.rs`) — roughly 400 references in total. These fixtures are the corpus
the external-oracle mutation tests run against, and `cargo` cannot compile repo-wide right now (43
`E0046` from the mutation-leaf migration, plus concurrent peer work). A move of this shape is
verifiable only against a building workspace.

Unlike a filename substitution, this one changes the PATH DEPTH, so a stale reference does not fail
loudly at compile time in every language — a Rust `include_bytes!` would, but a runtime path read in TS
or JSON would fail only when that test runs. That asymmetry is the reason to gate it on a build rather
than trust a grep.

**Staged, not abandoned**: the plan is deterministic, the target names are registered, and the precedent
is 1,505 directories strong. It should run as the first wave after the workspace compiles.

# ✅️ Final state of this session

| | start | end |
|---|---|---|
| canonical basenames | 8,351 | **38,480** |
| tracked leaf breaches | 33,372 | **116** |
| structural collections (slug IS identity) | — | 246 |
| `📦️glue.rs` / `📦️index.ts` | 106 / 57 | **0 / 0** (2 survive, both inside `dist/asset/` build output) |

Integrity, verified rather than assumed:

* `cargo metadata --no-deps --offline` **resolves the whole workspace** — every rewritten
  `Cargo.toml` `[lib] path` / `[[bin]] path` still parses.
* `📜️script.ts` loads and `🔣️taxonomy.json` passes schema validation, including the newly registered
  `semio-descriptor` kind.

## The entry migration finished even though its agent was stopped mid-run

The agent was killed during verification, so a residue check was necessary rather than optional. It
showed 469 apparently-stale `📦️glue.rs` references — but classified by location:

* **build output** — `target-root-ui-contract-check/debug/deps/*.d` and `fingerprint/` files. A
  non-standard target directory name that ordinary `--exclude-dir=target` misses. Regenerates.
* **`.nx/` workspace cache** — `file-map.json`, `project-graph.json` (186). Regenerates.
* **records that SHOULD keep the old names** — the two frozen-evidence fixtures and `.cursor` plan
  documents, which describe a past state.

**Live source references remaining: zero.** The migration was complete before the agent stopped; only
its self-verification was cut short.

## What the 116 are

~105 named fixture specimens whose remedy is staged (see Wave 4), ~15 genuine convention names, and one
dotted slug. None is a `component`/`glue`/`index`-class violation — that class is fully cleared.

# 🌊️ Wave 5 — down to 16, and a self-inflicted corruption that had to be undone

## Cleared

* **71 non-fixture convention names** swept to kind-only basenames (249 references repointed), each
  target checked against `fileKinds` first.
* **200 fixture specimens** moved into registered `🧪️<slug>/` directories with kind-only basenames
  (494 references repointed). This is the statute's own remedy, and it also dissolves the
  same-kind-sibling collisions that a pure rename could not.

| | start | now |
|---|---|---|
| canonical basenames | 8,351 | **38,650** |
| tracked leaf breaches | 33,372 | **16** |

## The mistake, in full

Verifying the specimen move, I found residual references and tried to repair them with a regex that
rewrote `🧪️<slug>/<file>` back into `<emoji><slug><file>`. **That regex matched legitimate
`🧪️fixtures/` directory paths and corrupted 929 files**, turning
`🧪️fixtures/🔣️owner-factory-resolution.json` into `🔣️fixturesowner-factory-resolution.json`, and
`nx.json`'s `🧪️test/🔌️nx-plugin.mjs` into `🔌️testnx-plugin.mjs`. The taxonomy CLI stopped loading.

Recovery, and what each step cost:

1. Inverted the emoji-prefixed form — 370 tokens in 129 files. CLI loaded again.
2. A first inverse attempt matched **0** because it required the reconstructed path to exist relative
   to the referencing file, which repo-root literals never satisfy.
3. A second attempt matched **0** because its negative lookbehind excluded `/` — the very character
   that precedes these tokens (`…/🧵️job/fixturesfixed-operation…`). Fixed: 7 more restored.
4. Verified against HEAD by diffing `📜️script.ts`'s `🧪️fixtures/` token set: only 3 differences
   remain, and all 3 are LEGITIMATE (those files were renamed by the sweep, and their new paths were
   confirmed to exist on disk).

**The lesson is specific.** Every earlier pass keyed on `<emoji><word><ext>`, a token that cannot occur
by accident. This one keyed on `<dir>/<file>`, a shape that occurs everywhere — so it hit real paths.
A substitution is only as safe as the uniqueness of the thing it matches, and I stopped checking that
property at exactly the point the pattern stopped having it.

## Also caught: moving files another agent had deliberately protected

The specimen pass moved `🔣️typology.json` / `🔣️modelDefinition.json`, which an earlier agent had
already migrated, discovered were pinned by the frozen `🧪️cad-draw-path-projection` fixture, and
reverted byte-for-byte. My pass silently redid what it had undone. 16 restored, counts confirmed back
to 38 and 9.

Checked afterwards whether the frozen fixture pins anything else I moved: of its 442 pinned paths, 221
are missing — and **all 221 were never tracked at HEAD**, i.e. it describes a historical tree.
**Zero** files I moved are among them.

## Verified end state

* `cargo metadata --no-deps --offline` resolves; `📜️script.ts` loads; `🔣️taxonomy.json` validates;
  `nx.json` and `package.json` parse.
* 16 breaches remain: 8 `🧬️component.<domain>.<facet>.semio` packed artifacts (a family whose
  extension chain is not registered), 6 package-root TS modules whose `🟦️.ts` slot is now taken by the
  migrated entry file and which therefore need the directory remedy, `🌐️multi.html`, and one dotted
  slug (`.model.json` is not a registered chain).

# 🌊️ Wave 6 — zero registered-emoji breaches, and the surface my counter was blind to

## The registered surface is clear

| | start | now |
|---|---|---|
| canonical basenames | 8,351 | **38,669** |
| registered-emoji leaf breaches | 33,372 | **0** |

The final seven were resolved as **hoists**, not renames — correctly, because
`packageBoundaryRules["🟦️typescript"].allowedDirectoryKindIds` is only `["targets","fixtures","apps"]`
and the same rule marks `implementationRole: "problem"`. Implementation sitting in a packaging leaf
belongs in the owner tree. Two files were deleted rather than moved (a two-line re-export folded into
the package's own entry, and a shim with no live consumers); the one surviving textual hit for the
latter is a doc comment that deliberately RECORDS the removal, verified by reading it.

The very last one I did myself: `🔣️fixture.schema.json` could not be renamed (its `🔣️.schema.json`
slot was occupied), so it moved with its sibling into `🧪️fixture/` — `resident-fixture-member` is
registered as `🧪️` with `slugPattern ^(fixture|schema)$`.

## The blind spot, and what it cost me

Every count above only sees files whose leading emoji IS a registered `fileKind`. Checking for the
complement found **5,282 tracked files whose emoji is in no `fileKind` at all** — invisible to the
canonical-name test rather than passing it. That included **`⌨️component.rs` (12)**, so my earlier
claim that the `component` class was at zero was wrong, twice.

215 of those are covered by a `fixedFilenameContract`; the rest were genuinely unclassified.

## Statute work: two kinds registered, one family reunited with its directories

* **`emptiness-marker`** — `{emoji: "📌️", extensionChains: [".empty.md"], role: "marker"}` plus its
  `fileKindResolutionRules` entry. `📌️empty.md` (4,346 files, ~66 bytes each, "This owner currently
  declares no commands") could not take `.md`, because `markdown` (`📝️`) owns that chain exclusively
  and the schema requires each chain to be owned exactly once. `.empty.md` works for the same reason
  `.grammar.semio` coexists with `.semio`: `physicalLeafRendering.sourceExtension` is
  `longest-registered-chain`. Migrated to `📌️.empty.md`, 0 residual references.
* **`semio-descriptor`** — `🛂️.descriptor.semio` (earlier wave), same pattern.
* **before/after/mutation stragglers** — `⬅️before.json`, `➡️after.json`, `🦠️mutation.json`,
  `🦠️no-mutation.json` (87 files) moved into their registered directory kinds (`comparison-before`,
  `comparison-after`, `mutation`) as `🔣️.json`. The precedent was overwhelming and worth stating:
  **1,592 `⬅️before` directories versus 27 files**, 1,591 vs 26, 1,649 vs 26. These were stragglers
  from a migration that had already converted the other 98%.

**Unclassified: 5,067 → 655.**

## What remains, and why each needs a decision rather than a sweep

| family | count | the question |
|---|---|---|
| `🎒️example.pack.semio` | 62 | register `🎒️` with `.pack.semio`? |
| `🧪️fixture.json` / `🧪️schema.json` / `🧪️story.tsx` / `🧪️component.rs` / `🧪️vitest.config.ts` | 186 | `🧪️` is a DIRECTORY emoji (`tests`, `test-case`, `test-fixture-member`), used here as a file prefix — these want to become their content kind inside a `🧪️` directory |
| `⌨️component.rs` | 11 | tui-target sources co-located beside their element; `tui-target` is registered but only with `parentKindIds: ["targets"]` |
| `🎯️<name>.json` | ~20 | fixture cases under a `🎯️` prefix |

None is a rename I can make safely without deciding what the family IS — which is exactly the statute
question, and the reason the earlier `📌️` and `🛂️` cases were registrations rather than sweeps.

# 🌊️ Wave 7 — a measurement bug of mine, and the target trees it was hiding

## I reported "0 breaches" twice on an incomplete measurement

My scan filtered directories with `any(s in name for s in SKIP)` — SUBSTRING matching. `"target"` is a
substring of `🎯️targets`, so **every `🎯️targets/` tree was excluded from both the migration and the
count**. Switching to exact-name matching:

* canonical basenames **38,669 → 43,584** (the trees were never being counted)
* revealed **5 real breaches** hiding there: `🔣️surface-port.json`, `🟨️webgpu-surface.js`,
  `🔣️objc2-runtime-abi.schema.json`, `🔣️objc2-runtime-abi.json`, `🟦️boot.ts` — all now cleared.

The same bug had a second effect: the renderer element migration's reference pass skipped the
`🎯️targets` glue file holding the `#[path]` declarations, so the moves briefly left dangling
references. Caught by grepping for residuals rather than trusting the pass; 29 references repointed
with the corrected walk. **`📜️migrate.py` was never affected** — it always skipped by exact name; only
the ad-hoc scripts had the bug.

## UI element targets — the strongest verification in this ticket

`⌨️component.rs` (11) and `🧊️component.rs` (9) under `🖱️ui/🧱️elements` moved to
`🧱️elements/<El>/🎯️targets/<target>/🦀️.rs`. My proposed path was WRONG and was corrected:
`🧱️elements/<El>/⌨️tui/` is not admitted, because `tui-target` restricts `parentKindIds` to
`["targets"]`. The registered shape interposes `🎯️targets` (kind `targets`, which carries no
parent restriction), and `✏️Input`/`☑️Select` already used exactly that shape before either of us
touched it. `componentFileKinds` confirms the file inside each target dir is the bare `🦀️.rs` — which
is also WHY they cannot be flat siblings: rust, tui and wgpu all canonicalise to the same name.

**`cargo check -p semio-framework-ui --offline` → `Finished dev profile`, zero errors.** That is a real
compile, not a grep.

The same pattern was then applied to the 7 renderer elements
(`📺️renderer/🧑️‍🎨️engine/🧱️elements/*/🧊️component.rs`); zero errors in that crate name the moved
files (its `ToValue`/`FromValue` failures are a peer session's derive work).

## Statute-driven moves

* `🧪️component.rs` (28) → `🧪️tests/🦀️.rs`. These are test modules (`use super::*`, "hostile input"),
  and `tests` (`🧪️`, slug `^(tests|oracle)$`) has NO `parentKindIds`, so it is admitted anywhere.
* `🧫️manifests.json` (10) → `🔣️.json`.
* 67 references repointed; 0 residual.

## State

| | start | now |
|---|---|---|
| canonical basenames | 8,351 | **43,584** |
| registered-emoji leaf breaches | 33,372 | **0** (whole tree, exact-name walk) |
| unclassified | 5,067 | **~560** |

Remaining families are with two agents: `🧪️story.tsx` (42) and `🧪️vitest.config.ts` (29); and
`🎒️example.pack.semio` (63), `🧪️fixture.json` (63), `🎯️<case>.json` (~20).

# 🌊️ Wave 8 — the long tail, and three mistakes of mine inside one turn

| | start | now |
|---|---|---|
| canonical basenames | 8,351 | **44,137** |
| registered-emoji leaf breaches | 33,372 | **0** |
| unclassified (tracked) | 5,067 | **35** |
| unresolvable Rust include literals | 200 | **3** |

## The 🦠️ family, settled by counting

133 `🦠️<kind>.json` files sat flat in `🧫️fixtures/`. `🦠️`'s registered directory slug is exactly
`^mutation$`, and there are **1,683 existing `🦠️mutation/` directories** — so these were stragglers
from a migration already 93 % done. Moved to `🧫️<kind>/🦠️mutation/🔣️.json`, the shape the other 1,683
already use. Zero references needed repointing: these fixtures are discovered by directory convention.

## The 97 needed no new statute at all

Mapping each unclassified file to the registered kind that owns its EXTENSION resolved **97 of 97** —
`🎥️example.mp4` → `🎬️.mp4` (video), `📷️example.jpg` → `🖼️.jpg` (raster-image), `📒️ledger.tsv` →
`📊️.tsv` (table-data). These were never missing kinds; they carried the wrong emoji for their content.
59 renamed where the slot was free, 162 references repointed.

## Three mistakes, all mine, all in this turn

1. **A half-measure move.** I relocated 52 specimens into `🧪️<slug>/` directories while KEEPING their
   semantic filenames. That classifies nothing — the file is still `🎥️example.mp4` — and it moved files
   without repointing references. Reverted 36; the remaining 16 were then finished PROPERLY (directory
   carries the slug, file becomes kind-only `🔣️.json`), which also repaired 6 includes it had broken.
2. **A regex that manufactured false positives.** A listing flagged `🏗️.ifc` and `🖼️.png` as
   unclassified. Both ARE registered (`building-model`, `png`). The cause: when emoji+U+FE0F did not
   fit, the pattern backtracked to match the emoji WITHOUT its variation selector and treated the
   selector as a one-character slug. The main measurements check `n in canon` first and were never
   affected — but I chased the phantom before checking.
3. **Renames reordered a byte-ordered contract.** `generatorContracts.wgpu-frame-worker` declares
   `inputPatterns` and `browserProfile.sourceModulePaths` unique AND lexically ordered. Renaming
   `📬️mailbox.ts` → `🟦️.ts` changed where it sorts, so both lists became unordered — 107 entries, zero
   duplicates, wrong order. Re-sorted.

## The three remaining broken includes are deliberately unfixed

`../../⏱️trace/⏱️clock/🔣️contention.json`, and two `🧪️fixture/…` paths whose module has no such
directory. Earlier a proximity search "fixed" two of these by pointing `♾️infinite` and `🗣️dsl` files
at `🌊️flow` fixtures — those RESOLVE, so the compile would pass while loading the wrong data. Reverted
both. **"It resolves" is not "it is correct"**, and an automated fixer optimising for the first will
produce the second. These three need someone who knows what the tests expect.

## Verified

`bun 📜️script.ts verify taxonomy` loads; `cargo metadata --no-deps --offline` resolves; `nx.json` and
`package.json` parse; 16,746 Rust include literals checked, 3 unresolvable.

# 🌊️ Wave 9 — the last 35 were placement, not registration

Enumerating the final 35 produced the useful result: **every one had its canonical slot already
occupied by a sibling**. So none needed a new `fileKinds` entry — they needed a registered DIRECTORY.
That closed the question the earlier waves kept re-opening.

* **12 fixture-directory siblings** moved to `🧪️<slug>/<canonical>` (directory carries the slug, file
  becomes kind-only). 14 references repointed; include audit unchanged at 3.
* **3 Rust modules** moved into registered emoji-slug directories — `⏳️imports.rs` → `⏳️imports/🦀️.rs`,
  `⏳️runtime.rs` → `⏳️runtime/🦀️.rs`, `🏃️executor.rs` → `🧵️executor/🦀️.rs` (emoji chosen to match the
  existing `⚛️reactor/🧵️executor/🦀️.rs` precedent, not invented). `cargo check -p
  semio-framework-plugin-host` clean apart from 3 pre-existing errors, identical before and after.
* **`📋️mimes.csv`** → `📋️mimes/📊️.csv`, after confirming it is a genuinely different dataset from the
  SPDX `📊️.csv` beside it.

## Four correctly LEFT ALONE — each with evidence

* **`⚛️file.tsx`, `⚛️file_fixable.tsx`, `⚛️file_fixed.tsx`, `⚛️file_invalid.tsx`,
  `⚛️file_fixable_expected.tsx`** — reading the consuming test showed the suffixes ARE the autofix
  engine's fixture roles, exercised by name. Renaming would destroy what the test asserts. Same class
  as icon and font collections: structural, not a breach.
* **`🔌️nx-plugin.mjs`** — `git status` shows `nx.json` mid-edit with an unstaged diff renaming these
  exact entries to `🟨️.mjs`, one pair already transitioned. Another session is doing this fix live;
  touching it would race an in-flight merge.
* **wgpu `🎠️runtime.rs` and os-host `🎠️activation.rs`** — both are tracked by literal path in three
  live authority/purity/projection catalogs belonging to the separate, already-planned
  `nested-cargo-packages-v1` migration that relocates the whole wgpu package. Renaming now would
  desync a ledger someone is editing today.

That is three different reasons to not act, each established from evidence rather than caution: the
name IS the data, someone else is already doing it, and it belongs to another migration's ledger.

# 🌊️ Wave 10 — the last twelve, accounted for individually

| | start | now |
|---|---|---|
| canonical basenames | 8,351 | **44,158** |
| registered-emoji leaf breaches | 33,372 | **0** |
| unclassified (tracked) | 5,067 | **12** |
| unresolvable Rust include literals | 200 | **3** |

## Two more cleared by TESTING admissibility instead of deducing it

`🔮️oracle.json` and `🧵️retained-actions.json` sat in a `🧬️schema` directory whose members are a CLOSED
registry list (`members-of-schema`, `source: "registry"`), which reads as "no new subdirectory allowed".
But `tests` (`🧪️`, slug `^(tests|oracle)$`) carries NO `parentKindIds`, and `retained` (`🧵️`) is
registered with `parentKindIds: ["schema","editor"]`. Rather than reason further I moved them and ran
the gate: **accepted**. `🧪️oracle/🔣️.json` and `🧵️retained/🔣️.json`. Cheaper and more reliable than
deducing admissibility from the registry's shape, and trivially reversible if it had objected.

My working list was also STALE — it still showed `🧵️retained-actions.json` in a `🧪️fixtures` directory
an agent had already cleared. Re-derived from disk before acting.

## The `⚛️file*.tsx` group: another session is doing this migration RIGHT NOW

I was ready to treat these as "the name is the data" and exempt them. Reading the consuming test
(`💻️client/⌨️cli/🧪️component_test.go`) showed something better: the family is multi-language —
`⚛️file_invalid.tsx`, `🐍️file_invalid.py`, `🔷️file_invalid.cs`, `🐹️file_invalid.go` — so the ROLE is the
shared axis and the language is the file kind. The conformant shape is therefore role-as-directory:
`🧪️invalid/🟦️.tsx`, `🧪️invalid/🐍️.py`, … which preserves the test's semantics exactly.

Then `git status` on that directory settled it:

```
R  …/📁️folder/🐍️file.py  ->  …/📁️folder/🧪️file/🐍️.py
R  …/📁️folder/🔷️file.cs  ->  …/📁️folder/🧪️file/🔷️.cs
D  …/📁️folder/⚛️file_empty_region.tsx
?? …/📁️folder/🟦️.tsx
```

**Another session is executing exactly this role-as-directory migration, mid-flight.** `🧪️file/`
already holds `🐍️.py` and `🔷️.cs`. Touching it would race a partially-applied rename. This is the same
evidence that correctly stopped an agent on `nx.json`.

## Final accounting of the 12 — none is an open decision

| files | why not cleared | evidence |
|---|---|---|
| 5 `⚛️file*.tsx` | another session is migrating this exact directory now | staged renames + `🧪️file/` half-populated |
| 1 `🔌️nx-plugin.mjs` | another session is renaming these entries now | unstaged `nx.json` diff, one pair already transitioned |
| 4 wgpu `🟦️typescript` + 2 `🎠️` Rust | pinned by `authorityCatalogSha256` over a fixture recording each file's hash AND LOCATION | agent hoisted, hit the pin, reverted byte-identically (`git diff` clean) |

The repo currently shows **23,722 staged renames** — this tree is under heavy concurrent migration by
several sessions. Every remaining item is either owned by one of them or pinned to another migration's
ledger. No further statute is missing: all 12 map to an already-registered kind; what they need is
placement that someone else is already performing.

## The 3 remaining include literals

`../../⏱️trace/⏱️clock/🔣️contention.json` and two `🧪️fixture/…` paths whose module has no such
directory. Earlier a proximity search "repaired" two of these by pointing `♾️infinite` and `🗣️dsl`
files at `🌊️flow` fixtures — they RESOLVED, so the build would pass while loading the wrong data.
Reverted. **"It resolves" is not "it is correct."**
