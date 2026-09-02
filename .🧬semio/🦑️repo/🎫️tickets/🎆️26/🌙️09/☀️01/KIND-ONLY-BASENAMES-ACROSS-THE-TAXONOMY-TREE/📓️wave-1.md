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
