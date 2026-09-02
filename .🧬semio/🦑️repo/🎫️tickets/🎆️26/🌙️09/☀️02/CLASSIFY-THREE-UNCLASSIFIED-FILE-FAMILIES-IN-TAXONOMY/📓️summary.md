# Classify Three Unclassified File Families In Taxonomy

## Decisions (per family)

### A — `🎒️example.pack.semio` and its siblings (`.bin`, `.las`, `.ply`, `.zip`)

Investigated the whole family together since they all live in the same
`📚️examples/<slug>/🖼️assets/` (or `🧫️fixtures/`) locations. Findings, per extension:

- `.pack.semio` — genuinely unclassified. Registered a new `fileKinds` entry
  `semio-snapshot` (`emoji: 🎒️`, `extensionChains: [".pack.semio"]`, `role: asset`) plus its
  `fileKindResolutionRules` entry `physical-semio-snapshot-pack-semio`. This was the only
  extension in the family that needed a NEW registration.
- `.bin` — **not actually unclassified**: `.bin` is already owned by the registered `binary`
  kind (`emoji: 💾️`). The two `🎒️example.bin` files were simply using the wrong emoji.
- `.las` / `.ply` — **not actually unclassified**: both extensions are already owned by the
  registered `mesh-model` kind (`emoji: 🧊️`, alongside `.glb/.gltf/.obj/.stl`). The `☁️` emoji
  used on disk for the `las`/`ply` stdio artifacts was never a registered file-kind emoji —
  only a directory/artifact-name emoji.
- `.zip` — **not actually unclassified**: already owned by the registered `archive` kind
  (`emoji: 🗜️`).

So decision item 3 in the assignment ("is `🎒️` one kind with two chains, or is `example` a
specimen name needing a directory") had a third answer the assignment didn't anticipate: two of
the four extensions weren't `🎒️`'s to claim in the first place — they belong to kinds already
registered under different emoji. `example` itself is never a specimen name: every
`🖼️assets` directory holds at most one file per kind (verified: zero basename collisions across
every affected directory, both before and after every rename below), so the specimen distinction
already lives entirely in the parent `📚️examples/<slug>/` or `🧫️fixtures/` directory name — exactly
per the STATUTE ("semantic concerns live in registered emoji-plus-slug DIRECTORIES"). This was
independently confirmed on disk: the sibling `🗣️example.dsl.semio` file in the same directories had
*already* been renamed to bare `🗣️.dsl.semio` by a concurrent session's live kind-only-basename
migration (`git status` showed the `R` rename mid-flight) — proving the "drop the semantic word,
kind-only basename" remedy is the established, currently-being-applied convention for this exact
shape, not a guess.

**Executed**: registered `semio-snapshot`, then renamed all 107 affected files
(101 `.pack.semio`, 2 `.bin`, 1 `.las`, 1 `.ply`, 2 `.zip`) to their bare kind-only basenames
(`🎒️.pack.semio`, `💾️.bin`, `🧊️.las`, `🧊️.ply`, `🗜️.zip`) in place, and repointed every
reference (183 files: mostly Rust `include_str!`, a few `📜️script.ts` string literals).

### B — `🧪️fixture.json` (and its `🧪️schema.json` sibling)

Not a missing file kind at all — `.fixture.json` and `.schema.json` are already registered
(`json-fixture` / `json-schema`, both emoji `🔣️`). The on-disk files used the WRONG emoji
(`🧪️`, test-tube) for what those extensions already own. The registry already anticipates the
correct shape: `semanticDirectoryKinds.resident-fixture-member` (`emoji: 🧪️`,
`slugPattern: ^(fixture|schema)$`) describes a DIRECTORY named `🧪️fixture` or `🧪️schema`
containing a kind-only `🔣️.json` (or `🔣️.schema.json`) leaf — not a flat file. Confirmed this is
the live, already-in-use convention: found 15 pre-existing `🧪️fixture/`/`🧪️schema/` directories
elsewhere under the same `🔨️modules` trees, each holding exactly a bare `🔣️.json` (and
`🔣️.schema.json` where relevant) leaf — e.g.
`🧰️framework/🔨️modules/🎯️action-bus/🧹️wire-retirement/🧪️fixture/{🔣️.json,🔣️.schema.json}`.

No registry changes were needed for this family.

**Executed**: moved all 96 flat files (63 `🧪️fixture.json` + 33 `🧪️schema.json`, all confirmed
under a `🔨️modules` root) into `🧪️fixture/🔣️.json` / `🧪️schema/🔣️.json`, zero collisions
(no directory of that name pre-existed at any of the 96 sites). Repointed every reference
(66 files: Rust `include_str!`, TS dynamic `import()`, and two `📜️script.ts` `readFileSync`
call sites).

### C — `🎯️<case-name>.json` under `🧪️fixtures/`

Directory remedy, using already-registered vocabulary — `semanticDirectoryKinds.fixture-case`
(`emoji: 🧫️`, open slug `^(?!fixtures$|examples$)[a-z0-9]+(?:-[a-z0-9]+)*$`,
`parentKindIds: [fixtures, test-tube-fixtures]`) is an exact structural match: our `🧪️fixtures`
directories resolve as `test-tube-fixtures`, which `fixture-case` explicitly lists as an
admitted parent. Every one of the 30 case names (`child-owner-isolation`,
`retained-command-limits`, `grant-frontier`, `retained-edit-limits`, `interactive-jobs`,
`retained-command-dispositions`) already matches the slug pattern. No registry changes needed.

**Executed**: moved all 30 files into `🧫️<case-name>/🔣️.json` (or `🔣️.schema.json` for the 6
files that had a `.schema.json` sibling sharing the same case name), zero collisions. Repointed
every reference (25 files, all Rust `include_str!` plus one `📜️script.ts` site).

## Registry diff

Only ONE new pair was added to `🔣️taxonomy.json` (family A's `.pack.semio`); families B and C
needed no registry changes at all — both were already-anticipated shapes (`resident-fixture-member`,
`fixture-case`) that simply hadn't been applied to these specific locations yet.

```
fileKinds.semio-snapshot = {"emoji": "🎒️", "extensionChains": [".pack.semio"], "role": "asset"}
fileKindResolutionRules.physical-semio-snapshot-pack-semio =
  {"extensionChain": ".pack.semio", "fileKindId": "semio-snapshot", "priority": 0}
```

## Verification

- `loadCatalogTaxonomy()` + `validateTaxonomy()` (the schema-only checker, bypassing the
  pre-existing, unrelated `generatorContracts["wgpu-frame-worker"]` missing-output breakage that
  blocks the full `loadTaxonomy()`): **0 problems**, both before and after every edit;
  `fileKinds` count went from 84 to 85.
- `bun 📜️script.ts verify taxonomy` (bare): still fails ONLY with
  `expected report or enforce, got undefined` — the same "healthy" bar the precedent
  `26/09/01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE` ticket used, for the same reason:
  `verify taxonomy report`/`enforce` themselves are currently blocked repo-wide by TWO pre-existing,
  unrelated issues outside this ticket's scope — (1) an un-resolved `♻️mit-bestand/recherche`
  gitlink repository-boundary decision, and (2) a missing catalog entry in
  `🧫️fixtures/🔣️ticket-important-exact-mutations.json` for ticket
  `26/07/12/ENFORCE-WINDOW-APP-PANEL-AND-PLUGIN-CONTRACTS-AT-COMPILE-TIME`'s
  `👥️presence/📌️important.md`. Neither touches any file in this ticket's three families.
- Repo-wide `rg -F` sweep (excluding `.🧬semio`, `.cursor`, `♻️mit-bestand`) for every one of the
  33 (family A) + 2 (family B) + 9 (family C) old basenames: **zero residual references** after
  repointing, for all three families.
- All new paths confirmed present on disk (spot-checked + `find` sweeps confirming zero flat
  `🧪️fixture.json`/`🧪️schema.json`/`🎯️*.json` files remain anywhere in the governed trees).
- `cargo check -p semio-s-plugin-architect --offline` launched in the foreground for a
  family-C-touched crate; see `🔬️cargo-check.md` for the captured result (build is slow under this
  repo's shared `sccache`, per prior session notes, so it may still be running when this summary is
  written).

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — added `semio-snapshot`
  file kind + resolution rule.
- 107 files renamed (family A), 96 files renamed (family B, as 96 new `.../🔣️.json` leaves inside
  96 new directories), 30 files renamed (family C, as 36 new leaves inside 30 new directories —
  6 case names carry both a `.json` and `.schema.json` leaf).
- 183 (family A) + 66 (family B) + 25 (family C) = 274 files had a stale reference repointed to
  the new path.

Full old→new path pairs are preserved as scratch JSON under this ticket's own working directory
in the session scratchpad (`family_a_moves.json`, `family_b_moves.json`, `family_c_moves.json`)
for traceability; not copied into the ticket folder since they are reconstructable from the
`rg`/`find` sweeps above and are pure intermediate working data.

## Cargo check result (family C representative crate)

`cargo check -p semio-s-plugin-architect --offline`, run in the foreground (auto-backgrounded by
the harness after its 400s timeout, monitored to completion): exits with **10626 pre-existing
errors**, all of the shape `E0308 mismatched types … expected Vec<ArchitectConfigMutation>, found
future` / "consider `await`ing on the `Future`" — i.e. the repo's already-documented,
repo-wide async/non-suspending convention debt (unrelated prior session note:
"~53k async fns non-suspending, repo didn't build"). None of the visible errors name the renamed
fixture file, its new `🧫️child-owner-isolation/🔣️.json` path, or any `include_str!` failure —
if the file were missing or the path malformed, `include_str!` would raise its own distinct
"couldn't read file" error, which does not appear. This crate does not compile before or after
this ticket's change, for reasons entirely unrelated to it (matches the pattern already flagged
for `semio-framework-os-kernel` elsewhere in this repo).
