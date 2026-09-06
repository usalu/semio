# 🧪️ Wave A1 — puzzle-2d mutation fixtures (2026-09-06)

Scope delivered: the `replace-node-handle` diff bug, a real-world (Nakagin / concrete-forest) vector
per mutation, refusal coverage under contract D6, the regenerated `🥒️.feature`, the Python second
implementation, and the puzzle fixture-lint discovery fix.

## 1. Corpus after this wave

**75 committed vectors over 26 mutation kinds** (was 26). Every one of them is registered in
`🔮️oracle/🔣️.json`'s `mutationCatalogs[0].vectors[].scenarios[]`, mounted by `#[path]` in
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/🦀️.rs`, listed exactly once in the feature's three
`Examples` tables, and registered by id in both adapters.

| mutation | applied | rejected | no-op | real-world scenario |
|---|---|---|---|---|
| `create-node` | 2 | 1 | 0 | `🌱️appends-a-capsule-to-the-tower` |
| `delete-node` | 2 | 1 | 0 | `🚫️deletes-the-tambour-and-severs-its-ten-edges` |
| `move-node` | 2 | 1 | 0 | `📍️moves-a-capsule-across-the-shaft` |
| `replace-node-geometry` | 2 | 1 | 0 | `🔳️squares-the-concrete-forest-seed` |
| `change-node-kind` | 2 | 1 | 0 | `🏗️rekinds-a-capsule-j-as-a-capsule-l` |
| `edit-node-text` | 2 | 1 | 0 | `✏️recodes-a-capsule-id-code` |
| `change-node-icon` | 2 | 1 | 0 | `🎨️swaps-a-capsule-icon` |
| `scale-node` | 2 | 1 | 0 | `📏️scales-a-capsule-by-three-halves` |
| `change-node-visible` | 2 | 1 | 0 | `🙈️hides-a-capsule` |
| `change-node-locked` | 2 | 1 | 0 | `🔒️locks-the-first-storey-tambour` |
| `change-node-root` | 2 | 1 | 0 | `🌳️promotes-the-base-to-root` |
| `change-node-anchor` | 2 | 1 | 0 | `⚓️derives-a-capsule-pose-from-its-door-edge` |
| `add-node-handle` | 2 | 1 | 0 | `➕️adds-a-third-slot-door-to-the-tambour` |
| `remove-node-handle` | 2 | 1 | 0 | `🚫️removes-a-tambour-door-and-severs-its-capsule-edge` |
| `replace-node-handle` | 1 | 1 | 0 | `🔌️rekinds-an-unconnected-tambour-door` |
| `connect-handles` | 2 | 0 | 1 | `🪢️rewires-the-capsule-the-subgraph-left-loose` |
| `disconnect-handles` | 2 | 1 | 0 | `✂️severs-a-capsule-from-the-first-storey-tambour` |
| `replace-edge-geometry` | 2 | 1 | 0 | `🧮️reposes-a-capsule-door-edge` |
| `change-edge-kind` | 2 | 1 | 0 | `🏷️kinds-a-capsule-door-edge-as-a-link` |
| `change-edge-tips` | 2 | 1 | 0 | `🖇️tips-a-capsule-door-edge` |
| `change-edge-visible` | 2 | 1 | 0 | `🙈️hides-a-capsule-door-edge` |
| `change-edge-locked` | 2 | 1 | 0 | `🔒️locks-the-base-to-tambour-edge` |
| `change-manifest-id` | 2 | 0 | 0 | `📦️repoints-the-tower-at-its-example-manifest` |
| `connect-kind-compatibility` | 2 | 0 | 0 | `🤝️admits-the-reverse-tambour-circular-pair` |
| `disconnect-kind-compatibility` | 2 | 1 | 0 | `💔️withdraws-the-tambour-rectangular-pair` |
| `replace-kind-catalogs` | 3 | 0 | 0 | `📇️installs-the-tower-handle-catalog` |
| **total** | **52** | **22** | **1** | |

The four kinds with no refusal vector are exactly the four whose diff builders have no
missing-target branch: `change-manifest-id` and `replace-kind-catalogs` address the document-root
`meta` singleton, and `connect-handles` / `connect-kind-compatibility` answer a warning-level
`mutation.no-op` on a duplicate rather than an error. `connect-handles` carries the no-op vector; the
other three warning branches are already reachable from the applied vectors' own no-op guards.

### The board the real-world vectors run on

`⬅️before` for 25 of the 26 real-world vectors is the **First-Storey-Tambour subgraph of
`📚️examples/🏗️nakagin-capsule-tower`**: 12 nodes, 10 edges, 30 handles, the tower's own 14-row
`kindCompatibility` relation and its `camera.zoom = 0.2407`. Every id, node kind, coordinate, `text`
id-code, icon kind, handle kind and handle angle is the value the shipped `🗣️.dsl.semio` carries —
derived by parsing that asset, never typed by hand. The subgraph is the hub tambour
`17d5dec8-…` plus its ten neighbours plus `f537171c-…`, the one capsule whose real edge
(`08a57668-…`) the selection leaves outside, which is what makes a REAL `connect-handles` vector
possible: it restores the tower's own wire, id and pose included. `replace-node-geometry` uses
`📚️examples/🌲️concrete-forest`'s `seed-left-001` instead, because it is the only shipped node that
declares a `shape` at all.

The disambiguating vectors the oracle note asked for are all present:
`replace-kind-catalogs` with `null` (`🗑️clears-the-installed-handle-catalog`, whose `⬅️before` is the
after-state of the install vector), `remove-node-handle` on a connected handle,
`delete-node` with ten attached edges, `connect-handles` with a duplicate id (no-op),
`create-node` with a duplicate id (fatal).

⚠️ **`remove-node-handle` severs exactly one edge, not more.** The real tower is a tree: no handle in
`🏢️tower/🗣️.dsl.semio` carries two edges (measured — maximum handle degree is 1 across all 179
edges), so a >1-edge handle cascade is not expressible on real data. `delete-node` covers the
multi-edge cascade instead (ten edges in one mutation).

## 2. The `replace-node-handle` bug

`🧬️mutations/🔌replace-node-handle/🔺️diff/🦀️.rs` ran `if next == *node { … no-op … }` on an
unmodified clone, **before** the loop that performs the replacement — so the verb answered
`mutation.no-op` with an empty diff for every input. Fixed by moving the guard after the loop, so:

- absent node → `mutation.target-missing` on the node id (unchanged),
- absent handle → `mutation.target-missing` on the handle id (unchanged),
- a replacement equal to the handle it replaces → `mutation.no-op` (now reachable, and only then),
- otherwise a real `nodes.patched` replacement.

`↩️inverse/🦀️.rs` needed no change: it captures the BASE handle at `handle_id` and re-issues
`replace_node_handle(node_id, handle_id, base_handle)`, which restores it exactly whenever the
payload keeps the handle id — the case the new vector exercises. (It does *not* invert a replacement
that also renames the handle; nothing in this vocabulary can, and no vector claims it does.)

The buggy fixture `⏸️rekind-handle-1-is-noop` is **retired** and replaced by
`🔌️rekinds-an-unconnected-tambour-door`: the second tambour's `:sl0_d1` door — unconnected on this
board — re-kinded from `door tambour left` to `door tambour right`, a kind the tower's own relation
admits. That single vector settles all three readings the oracle refused to choose between.
`🐍️.py` implements the verb accordingly and no longer refuses it.

## 3. Feature file

Regenerated in full. The two exhaustive tables now carry **one real-world vector per kind** (26 rows
each) and all four stale hex-suffixed paths are gone — the four odd directory names were **renamed to
the honest names the feature already used**, not the other way round:

| leaf | was | now |
|---|---|---|
| `➖remove-node-handle` | `🚫️removes-handle-2-e261f3` | `🚫️removes-handle-2-and-severs-edge` |
| `💔disconnect-kind-compatibility` | `🚫️removes-handle-5fccd0` | `🚫️removes-handle-kind-pair` |
| `📚replace-kind-catalogs` | `📇️installs-handle-a0eaf8` | `📇️installs-handle-kind-catalog` |
| `🤝connect-kind-compatibility` | `🤝️adds-handle-kind-d59cb5` | `🤝️adds-handle-kind-pair` |

⚠️ **A third table was unavoidable, and it is `🔱️trinity/🔌️jack`'s own `@id-spec-vector` pattern.**
The task asked for every scenario directory to be listed once in *both* tables; the framework forbids
it. `mutationCoverageBreaches`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:1565`) reads a
`mutate-<x>` scenario id as a claim that `<x>` is a declared mutation KIND and raises
`mutation-kind-undeclared` for anything else, and `materializeScenario` (`:459`) builds the scenario
id as `<base>-<row.id>`, so a second row per kind in the `mutate` table is a contract breach by
construction. Refusal vectors are excluded for a second, independent reason: `conformance` requires
`status == "applied"` and `footprint` measures a diff a refusal deliberately does not commit.

So: `@id-mutate` and `@id-inverse` carry the 26 real-world vectors, and `@id-spec-vector` carries the
other 49 (25 kept alpha-board vectors, 22 refusals, `connect-handles` duplicate, catalogs cleared),
with `id | kind | verdict | vector | diff` columns. Every scenario directory appears exactly once
across the three tables. Verified with the repository's own parser: **102 scenarios, 0 errors, 0
duplicate ids.**

`🦀️.rs` host changes: `KINDS` stays at 26 (unchanged); added `SPEC_VECTORS` (49 ids) and a
`spec_vector` handler that asserts the declared verdict, and — for a refusal — that the committed
`🔺️diff/🚫️.absent` sentinel exists and is empty. Also fixed its `round_trip` handler, whose
`SNAPSHOT` const named `🧪️tests/appends-node-c` (no emoji prefix) and therefore could never have
resolved on a non-cached run; it now reads the real tower subgraph.

## 4. Python second implementation — run result

`🐍️.py` changes: implements `replace-node-handle` (apply + inverse); accepts a `null` argument in
`replace-kind-catalogs` and its inverse; rejects a duplicate `create-node` id; treats a duplicate
`connect-handles` id as a no-op; `written()` now drops a member whose argument is `null` as well as
one equal to its default (the Option-shaped members are absent, not null, in every committed
snapshot); `validate()` accepts a node that declares no `shape`, which every real Nakagin node does;
`UNDERDETERMINED` is replaced by an empty `REFUSALS` recording that nothing is refused any more.
Added `SPEC_VECTORS` and `spec_vector_handler`, registered in the oracle role like the others.

**Driver result** (`🗑️generated/a1/driver.py` — loads the committed `🐍️.py` behind a stub
`semio_repo_test` host, expands the committed `🥒️.feature` the way the framework's parser does, and
calls exactly the handler the reference registers for each expanded scenario id):

```
{"expanded": {"mutate": 26, "inverse": 26, "spec-vector": 49}, "failures": []}
```

plus `identity-round-trip` checked separately — **102 / 102 green**. That covers, per vector: the
payload declares its kind, `apply` lands the committed `➡️after` member by member, the observability
law against the committed outcome, and — for every applied vector — `apply` followed by the
reference's OWN computed inverse landing the committed `⬅️before`. Refusal vectors assert the mirror:
the reference must raise, the board must not move, and the D6 sentinel must be empty.

Two findings the driver produced while the corpus was being authored, both now encoded in the
committed vectors rather than worked around:

- `disconnect-kind-compatibility` only round-trips on the relation's **last** row, because
  `connect-kind-compatibility` appends. The vector addresses
  `tambour rectangular bottom → tambour rectangular top`, which is that row.
- `connect-handles`'s inverse is payload-derived, so undoing a duplicate-id no-op REMOVES the edge
  the board already held. The no-op vector's `🦀️.rs` asserts that asymmetry (`assert_ne!`) instead of
  claiming an inverse law that does not hold, and the vector sits in the spec-vector table rather
  than the inverse table.

`🔮️oracle/🔣️.json`: `mutationCatalogs[0].vectors` rewritten (26 vectors, 75 scenarios) and
`oracles[0].nativeSecondImplementation.fixtureCoverage.vectors` bumped `26 → 75`. C1's five new
`oracles[]` entries were present when I wrote and are preserved untouched.

## 5. Fixture lint — ⚠️ the fix turns a falsely-green gate red

The leaf predicate accepted a leaf only when `<leaf>/🦠️mutation/🦀️.rs` existed — the
`🖍️draw`/`🗄️stdio` shape. Most of the repository, puzzle-2d included, writes the leaf's Rust bare at
`<leaf>/🦀️.rs`. **The predicate appeared twice**, and fixing only the one the ticket named would have
changed nothing measurable: `discoverArtifacts` decides whether a *tree* is walked at all, and
`declaredMutations` decides whether that tree's *leaves* are seen. With only the first fixed, the
first run reported `211 trees · 92 mutations · 92 covered` — 211 trees and 92 leaves between them,
because puzzle-2d and every sibling using the bare shape contributed zero leaves and were therefore
linted into a green nothing. Both now go through one shared `leafMutationFile()` helper.

Before (first run, `discoverArtifacts` fixed only) and after (both fixed):

```
🧬️ 211 artifact mutation trees ·   92 mutations ·   92 covered ·   0 uncovered   → ✅️ exit 0
🧬️ 211 artifact mutation trees · 1935 mutations · 1225 covered · 710 uncovered   → ❌️ 784 error(s), exit 1
```

**puzzle-2d passes: 26/26 leaves covered, zero findings** — `grep -c puzzle` over the full `--by-tree`
output is 0, in both the uncovered-tree list and the error list. The 784 errors and 103 uncovered
trees are entirely other owners' pre-existing debt that this lint has never been able to see:

- ~710 are `no 🧪️tests cases` on mutation leaves that ship no fixture at all — dominated by
  `✏️s/🔌️plugins/🗄️stdio` (obj 20/21, gif 19/20, pdf 18/18, semio 18/19, dxf 17/18, …) plus
  `🧰️framework/🛍️products/💻️os/🔨️modules/{🔁️workflow 18/18, ♾️infinite/🎲️board/…/🕸️dag 14/14,
  🌊️flow/🌿️vcs 10/10, 🏪️store 6/6}` and `🌍️gis/🗺️gismap/…/✏️editor/🎚️config 7/7`.
- 34 in `✏️s/🔌️plugins/🔋️energy` — every one of its `🎯️outcome/🔣️.json` files omits `status`
  (`must be "applied" or "rejected", got undefined`). That is the ticket 🔋️ENERGY-PLUGIN-END-TO-END's
  own corpus and it is malformed against contract D6.
- 5 in `🗄️stdio` pdf `4️⃣1.4/🖨️x/📉️collapse-page-size/🔄️round-trips-the-concrete-inverse` — an empty
  scenario directory (no mutation, diff, outcome, `🦀️.rs` or before-snapshot).
- 1 in `✏️s/🔌️plugins/📸️remodel`.

None of that is caused by this wave and none of it is in reach of my write region; the honest
statement is that `fixtures lint` was reporting green over ~95% of the repository's mutation leaves
and now reports what is actually there. **If a peer wave depends on this command exiting 0, it will
break** — that is the finding, not a regression.

The 10,146 warnings are the repo-wide missing `.op.semio`/`.spr.semio`/`.patch.semio` derived
encodings; `fixtures generate` is aspirational text in a comment, not a runnable command (see
`📓️explore-mutation-fixtures.md` §6), so they stay warnings and only `--full` fails on them. The 50
new bundles carry exactly the 12-node source shape the framework's own `expectedVectorBundle`
requires — verified independently against that function's rules for all 75 puzzle-2d bundles (0
invalid), and the rewritten `mutationCatalogs[0]` was validated by calling the framework's own
`mutationCatalogProblems()` directly: **`[]`**.

## 6. ⚠️ Blocking finding for wave D — outside my region, ALREADY FIXED minimally

`cargo test -p semio-s-plugin-puzzle` **could not build at all** before this wave, for reasons that
have nothing to do with puzzle-2d: eleven `#[cfg(test)] #[path = …]` scenario mounts in
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/🦀️.rs` named directories that do not exist on disk —
the same hex-suffix rename drift commit `b0dfa0f09b` left in the 2d feature file, but in **puzzle 3d
(8 mounts) and puzzle 5d (3 mounts)**, where it hits `mod` resolution instead of a fixture path:

| artifact | mount named | actual directory |
|---|---|---|
| 5d `🗑️delete-part` | `🚫️removes-part-a-and-severs-fastener` | `🚫️removes-part-a-and-5f0581` |
| 5d `➖remove-part-grip` | `🚫️removes-grip-1-and-severs-fastener` | `🚫️removes-grip-1-and-5d2306` |
| 5d `💔disconnect-kind-compatibility` | `🚫️removes-grip-pair` | `🚫️removes-grip-1464bd` |
| 3d `🗑️delete-object` | `🚫️removes-object-a-and-severs-attraction` | `🚫️removes-object-a-and-ce36fb` |
| 3d `➕add-object-vortex` | `🌀️appends-vortex-3-to-object-b` | `🌀️appends-vortex-3-to-e60441` |
| 3d `➖remove-object-vortex` | `🚫️removes-vortex-2-and-severs-attraction` | `🚫️removes-vortex-2-8436d0` |
| 3d `🧮replace-attraction-geometry` | `📍️repositions-attraction-1` | `📍️repositions-43523e` |
| 3d `🖇️replace-reference-source` | `🖇️repoints-reference-1-source` | `🖇️repoints-017eb5` |
| 3d `🤝connect-kind-compatibility` | `🤝️adds-vortex-kind-pair` | `🤝️adds-vortex-kind-664041` |
| 3d `💔disconnect-kind-compatibility` | `🚫️removes-vortex-kind-pair` | `🚫️removes-vortex-a24eec` |
| 3d `📚replace-kind-catalogs` | `📇️installs-vortex-kind-catalog` | `📇️installs-vortex-a9d291` |

I repointed all eleven `#[path]`s at the directories that actually exist — the minimal change that
lets the crate's test build resolve — and left the module names and the 3d/5d directories alone. The
**honest** fix is the one applied to 2d here (rename the directories to the names their own feature
files already use, then update `🔮️oracle/🔣️.json`, the feature `Examples` tables and
`🔣️taxonomy.json`); that belongs to whoever owns puzzle 3d/5d and is NOT done.

## 7. Files touched

**In region**

- `…/✳️any/🧬️schema/🧬️mutations/🔌replace-node-handle/🔺️diff/🦀️.rs` — the one Rust fix.
- `…/🧬️schema/🧬️mutations/**/🧪️tests/**` — 50 new scenario bundles (300 files), 4 directories
  renamed, 1 retired (`🔌replace-node-handle/🧪️tests/⏸️rekind-handle-1-is-noop`). All 75 scenario
  `🦀️.rs` files pass `rustfmt --edition 2021 --check`.
- `…/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🥒️.feature` — regenerated (235 lines, 3 outlines + identity).
- `…/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🦀️.rs` — `SPEC_VECTORS`, `spec_vector`, `round_trip` path fix,
  module doc. `rustfmt --check` clean (it was not before; the file is now formatted).
- `…/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🐍️.py` — see §4.
- `…/✳️any/🔮️oracle/🔣️.json` — `mutationCatalogs` and `oracles[0].…fixtureCoverage.vectors` only.
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts` — one new `leafMutationFile()` helper, used
  by both `discoverArtifacts` and `declaredMutations` (§5).

**Outside my declared region, and why**

- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/🦀️.rs` — the only place scenario `🦀️.rs` files are
  mounted; task 2 requires new scenarios to be mounted. 75 puzzle-2d mounts (was 26) plus the eleven
  3d/5d repairs of §6. The diff contains nothing but `#[cfg(test)]`/`#[path]`/`mod tests_…` lines.
  **Do not run `rustfmt` in write mode on this file** — it recursively reformats every
  `#[path]`-mounted module, including files owned by waves B1/B2/B3.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` —
  `semanticDirectoryMemberKinds.members-of-tests.memberNames` is a closed registry; an unregistered
  scenario directory is a hard breach (`🏗️builder/🟦️.ts:40`, and
  `mutationCatalogProblems` requires `directoryName` minus its leading emoji to equal the catalog
  `id`). 50 names added, 5 retired names removed, nothing else touched — a peer's
  `plugin-publication-authority` entry and regenerated `wgpu-frame-worker` `catalogSha256`, both
  uncommitted in the working tree when I arrived, are preserved byte for byte.

## 8. What the main session must run, and what to route where

1. `cargo test -p semio-s-plugin-puzzle` — 75 puzzle-2d scenario modules, ~500 `#[test]` fns. The
   `➡️after` and `🔺️diff` leaves were computed by a Python mirror of the 26 Rust diff builders and of
   `MutationDiff::apply` (`🗑️generated/a1/model.py`), **not by the production Rust**, because this
   wave may not run cargo. If any `produces_committed_diff` / `applies_to_committed_after` fails, the
   likely cause is a diff-apply ordering detail I inferred rather than read
   (remove → patch → add → reorder); route the failure back here with the assertion text and it is a
   one-line regeneration, not a fixture rewrite.
2. `bun ./📜️script.ts fixtures lint` from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust` — already run.
   **puzzle-2d is clean; the command now exits 1 on other owners' pre-existing debt** (§5). Decide
   whether that stays red or whether the lint gets a per-owner scope; do NOT "fix" it by reverting
   the leaf predicate, which would only restore a gate that measured nothing.
3. The repo test-module contract gates (`mutationVectorRegistryBreaches`, `mutationCoverageBreaches`)
   — not runnable end to end from this wave, but their two decisive rules were checked directly:
   `mutationCatalogProblems()` returns `[]` for the rewritten catalog, and all 75 bundles match
   `expectedVectorBundle`'s node set exactly. The feature parses through the repository's own
   `parseFeature()` with 0 errors, 102 scenarios, 0 duplicate ids.
4. **Route to whoever owns puzzle 3d/5d**: the eleven drifted scenario directory names of §6.
5. **Route to C1**: `🔮️oracle/🔣️.json`'s `fixtureCoverage.vectors` is now 75; any new oracle entry's
   own coverage figure should say 75 too, and the corpus now contains refusal vectors with no
   `🔺️diff/🔣️.json`, which a jsonpatch-style diff oracle must skip rather than fail on.
6. **Route to A2**: `🧬️mutations/🔣️.json` is being replaced with a `oneOf`; the feature's old
   paragraph about that file being a snapshot-schema copy has been dropped from the prose, so nothing
   contradicts A2's work.
7. **Route to 🔋️ENERGY-PLUGIN-END-TO-END**: all five of its committed `🎯️outcome/🔣️.json` files omit
   `status`, which contract D6 requires (§5). Its own gate could not see this before today.

## 9. Scripts kept

Under `🗑️generated/a1/`: `dsl_parse.py` (the `.dsl.semio` table parser), `model.py` (the Rust
diff-builder mirror), `scenarios.py` (the vector table), `emit.py` (writes the bundles),
`wire.py` (feature tables, adapter lists, catalog entries, mounts, taxonomy names),
`feature.py` (writes `🥒️.feature`), `driver.py` (the Python differential run of §4). They regenerate
the whole corpus deterministically; `emit.py` does not run `rustfmt`, so re-run
`rustfmt --edition 2021` over the scenario `🦀️.rs` files after any regeneration.
