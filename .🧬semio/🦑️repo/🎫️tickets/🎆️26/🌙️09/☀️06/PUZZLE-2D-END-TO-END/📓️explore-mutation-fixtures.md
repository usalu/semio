# Explore: Puzzle 2D Mutation Fixtures

Subject: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any` (hereafter `BASE`).
All relative paths below are relative to `BASE` unless stated otherwise.

## 1. The 26 mutation kinds

`BASE/🧬️schema/🧬️mutations/` has 28 leaf dirs; `💾️binary` and `📝️text` are codec facets (skipped), leaving 26 real mutations. Every one of the 26 has `↩️inverse/🦀️.rs`, `🔺️diff/🦀️.rs`, and **exactly one** scenario under `🧪️tests/` (confirmed repo-wide in §5 — this is not unique to puzzle-2d). All 26 committed outcomes are `"status": "applied"` — zero rejected-outcome coverage exists today (verified via `grep -r "status" **/🎯️outcome/🔣️.json`).

| Mutation | Payload Args | Outcome Semantics | Test Scenario | Inv. | Diff |
|---|---|---|---|---|---|
| ⚓change-node-anchor | `id`, `newAnchor: Fixed\|Derived` | Patches node's `anchor` | ⚓️fixed-to-derived | ✓ | ✓ |
| ✂️disconnect-handles | `id` (edge id) | Removes an edge by id | 🚫️removes-edge-1 | ✓ | ✓ |
| ✏️edit-node-text | `id`, `newText?` | Patches node `text` | ✏️retitles-node-a | ✓ | ✓ |
| ➕add-node-handle | `nodeId`, `handle: Puzzle2dHandle`, `index?` | Inserts a handle into a node's `handles` (append if `index` absent); no-op if handle id exists | ➕️appends-handle-3-to-node-b | ✓ | ✓ |
| ➖remove-node-handle | `nodeId`, `handleId` | Removes a handle; **cascades**: severs every edge referencing that handle | 🚫️removes-handle-2-e261f3 | ✓ | ✓ |
| 🆔change-manifest-id | `newManifestId?` | Patches singleton `meta.manifestId` | 📦️repoints-manifest | ✓ | ✓ |
| 🌟change-node-root | `id`, `newRoot?` | Patches node `root` flag | 🌳️promotes-node-a-to-root | ✓ | ✓ |
| 🌱create-node | `node: Puzzle2dNode` (id/anchor/handles/x/y required), `index?` | Real append-only insert; **fatal** `mutation.duplicate-id` if id exists (`🔺️diff/🦀️.rs:9`, see below) | 🌱️appends-node-c | ✓ | ✓ |
| 🎨change-node-icon | `id`, `newIconKind?` | Patches node `iconKind` | 🎨️swaps-node-a-icon | ✓ | ✓ |
| 🏗️change-node-kind | `id`, `newNodeKind?` | Patches node `nodeKind` | 🏷️reassigns-node-a-kind | ✓ | ✓ |
| 🏷️change-edge-kind | `id`, `newEdgeKind?` | Patches edge `edgeKind` | 🏷️rekinds-edge-1 | ✓ | ✓ |
| 👀change-edge-visible | `id`, `newVisible?` | Patches edge `visible` | 🙈️hides-edge-1 | ✓ | ✓ |
| 👁️change-node-visible | `id`, `newVisible?` | Patches node `visible` | 🙈️hides-node-a | ✓ | ✓ |
| 💔disconnect-kind-compatibility | `source`, `target` | Removes a `(source,target)` row from `meta.kindCompatibility` | 🚫️removes-handle-5fccd0 | ✓ | ✓ |
| 📍move-node | `id`, `newX`, `newY` | Sets absolute node position | 📍️moves-node-a | ✓ | ✓ |
| 📏scale-node | `id`, `newScale?` | Patches node `scale` | 📏️doubles-node-a | ✓ | ✓ |
| 📚replace-kind-catalogs | `newCatalogs?: Puzzle2dKindCatalogs` | Whole-value swap of `meta.kindCatalogs` (`None` clears it) | 📇️installs-handle-a0eaf8 | ✓ | ✓ |
| 🔌replace-node-handle | `nodeId`, `handleId`, `newHandle` | **Bug** (see below): the no-op guard fires before the replace loop runs, so this verb is a no-op today no matter the input | ⏸️rekind-handle-1-is-noop | ✓ | ✓ |
| 🔐change-edge-locked | `id`, `newLocked?` | Patches edge `locked` | 🔒️locks-edge-1 | ✓ | ✓ |
| 🔒change-node-locked | `id`, `newLocked?` | Patches node `locked` | 🔒️locks-node-a | ✓ | ✓ |
| 🖇️change-edge-tips | `id`, `newSourceTip?`, `newTargetTip?` | Patches edge tip pair | 🔀️swaps-edge-1-tips | ✓ | ✓ |
| 🗑️delete-node | `id` | Removes node; **cascades**: severs every edge touching any of its handles | 🚫️removes-node-a-and-severs-edge | ✓ | ✓ |
| 🤝connect-kind-compatibility | `source`, `target`, `bidirectional`, `important`, `specificity` | Appends a compatibility row; no-op if pair exists | 🤝️adds-handle-kind-d59cb5 | ✓ | ✓ |
| 🧊replace-node-geometry | `id`, `newShape?`, `newRadius?`, `newWidth?`, `newHeight?` | Whole-value swap of node shape/extent | 🔳️circle-to-rectangle | ✓ | ✓ |
| 🧮replace-edge-geometry | `id`, 8× `f64` (gap/shift/rise/rotation/turn/tilt/x/y, `new`-prefixed) | Whole-value swap of edge connection-pose | 📍️repositions-edge-1 | ✓ | ✓ |
| 🪢connect-handles | `id`, `source`, `target`, `edgeKind?`, 8× `f64` (bare-named), tips | Creates a new edge between two handles (real insert); no-op if id exists | 🪢️adds-second-edge | ✓ | ✓ |

Full argument/citation detail (schema.json + rs line ranges for all 26) is in the background-agent transcript; the table above is the synthesized result. Representative citations: `🌱create-node/🧬️.schema.json:1-119`, `🌱create-node/🦀️.rs:17-21`, `🌱create-node/🔺️diff/🦀️.rs:7-19`.

**Bug found:** `🔌replace-node-handle/🔺️diff/🦀️.rs` (lines 13-21) clones the node into `next` and checks `next == *node` for its no-op short-circuit **before** the replacement loop that would actually mutate `next` runs — so the verb always reports a no-op regardless of input. The mutation's only fixture (`⏸️rekind-handle-1-is-noop`) encodes this buggy behavior as expected, so the test suite currently protects the bug rather than catching it. The 🔮️oracle rationale (`BASE/🔮️oracle/🔣️.json:19`) independently corroborates this: the Python reference *also* refuses to model `replace-node-handle` because the single vector is ambiguous between "unimplemented", "refuses an edge-attached handle", and "refuses an incompatible kind" — all three readings are consistent with a no-op, and the diff-builder bug is one concrete mechanism that would produce exactly that.

Two scenario-naming oddities (not semantic bugs): `💔disconnect-kind-compatibility`'s only scenario is named `🚫️removes-handle-5fccd0` (about a handle, not compatibility) and `📚replace-kind-catalogs`'s is `📇️installs-handle-a0eaf8` (about a handle, not catalogs).

## 2. The fixture-case contract

Source: `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts` (the puzzle plugin's own `bun ./📜️script.ts fixtures lint`, which is written generically enough that it walks and lints *any* `🧬️mutations` tree repo-wide, not just puzzle's).

- **Discovery** (`:41-72`): recursively finds every dir literally named `🧬️mutations`; a leaf only counts if `existsSync(<leaf>/🦠️mutation/🦀️.rs)` (`:63`).
- **`NON_MUTATION_DIRS`** (`:75`): `{"💾️binary", "📝️text"}` — matches the two skipped facets.
- **`CORE_CASE_FILES`** (`:80`): `["🦠️mutation/🔣️.json", "🔺️diff/🔣️.json", "🎯️outcome/🔣️.json", "🦀️.rs"]` — hand-authored, source-of-truth per test case.
- **`DERIVED_CASE_FILES`** (`:85-90`): `.op.semio`/`.spr.semio` under `🦠️mutation/`, `.patch.semio`/`.patch.spr.semio` under `🔺️diff/` — "never hand-authored... produced by `fixtures generate`" (comment, `:82-84`). **`fixtures generate` does not exist as a runnable command anywhere in the repo** — see §6.
- **Snapshot sides** (`:92-96, 206-223`): each of `📸️snapshot/⬅️before` and `➡️after` is either (a) `🔣️.json` core + derived `.dsl.semio`/`.pack.semio`, or (b) exactly one `🔗️component.ref.json` pointing at a canonical example (`artifact`/`standard`/`subset`/`example`/`asset` fields, no `..` or `/`) — never both.
- **`lintCase`** (`:178-225`) is the per-scenario contract: reads `🎯️outcome/🔣️.json`, requires `status` ∈ `{"applied","rejected"}` (`:186`; note the *framework* schema's `MutationOutcomeClass` enum is wider — `applied|no-op|empty|disjoint|rejected`, `🧰️framework/…/🧪️test/🧬️schema/🔣️.json:49`), requires a machine-readable `code` when `rejected` (`:187`). **Contract D6** (`:78-79, 193-204`): when `rejected`, `🔺️diff/🔣️.json` is *replaced* by an empty sentinel `🔺️diff/🚫️.absent` (`:194,198,202-204`) rather than an invented empty patch.
- **`--full` vs default**: without `--full`, missing derived-encoding files are warnings; with it, errors (`:200,221`).
- **Coverage** (`:257-271`): every declared enum variant/leaf must have ≥1 `🧪️tests` case, reported per-tree with `--by-tree`.

**⚠️ Discovery mismatch found**: the discovery filter checks for `<leaf>/🦠️mutation/🦀️.rs` (a leaf-level `🦠️mutation/` subfolder), but puzzle-2d (like gismap, fem2d, cad, block, and most of the repo) puts the mutation's own Rust directly at `<leaf>/🦀️.rs` (bare, no subfolder — confirmed: `🌱create-node/🦀️.rs` exists, `🌱create-node/🦠️mutation/🦀️.rs` does not). Only a newer convention used by `🖍️draw` and several `🗄️stdio` artifacts (`set-snapshot` leaves) actually has `<leaf>/🦠️mutation/🦀️.rs` (92 matches repo-wide, confirmed via `find … -path "*🧬️mutations/*/🦠️mutation/🦀️.rs"`, excluding `🧪️tests/`). **Net effect: `discoverArtifacts()` never adds puzzle-2d's `🧬️mutations` tree to `found` at all** (`leaves.length` is 0 for it), so `bun ./📜️script.ts fixtures lint` silently skips puzzle-2d (and gismap, fem2d, cad, block, …) entirely — this instance of the "taxonomy filename drift blinds discovery" failure mode (see memory) needs fixing (or working around) before relying on this lint to gate new puzzle-2d fixtures.

### The complete template scenario: `🌱create-node/🧪️tests/🌱️appends-node-c`

Files (all under `BASE/🧬️schema/🧬️mutations/🌱create-node/🧪️tests/🌱️appends-node-c/`):
- `🦠️mutation/🔣️.json` — `{"mutation":"createNode","node":{...},"index":null}`
- `📸️snapshot/⬅️before/🔣️.json` — 2 nodes, 1 edge, 1 kindCompatibility row
- `📸️snapshot/➡️after/🔣️.json` — same + `node-c` appended
- `🔺️diff/🔣️.json` — full `Puzzle2dDiff` shape, every field `null` except `nodes.added:[node-c]`
- `🎯️outcome/🔣️.json` — `{"status":"applied"}`
- `🦀️.rs` — 7 `#[test]` fns: `applies_to_committed_after`, `inverse_restores_before`, `committed_json_is_canonical` (×2 snapshots), `declared_outcome_holds`, `produces_committed_diff`, `committed_diff_is_canonical`, `committed_diff_applies_to_after`. This is the per-leaf test that *does* link the plugin crate (`use crate::artifacts::puzzle2d::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation}`) — contrast with §3's cross-language case, which does not.

So a new scenario needs exactly 6 files: the 5 JSON above plus a `🦀️.rs` following this template (the JSON is `include_str!`'d, never re-typed as Rust literals).

## 3. The cross-language differential case `◻️mutate-puzzle-2d-1`

Location: `BASE/🧪️tests/◻️mutate-puzzle-2d-1/` — `🦀️.rs` (292 lines), `🥒️.feature` (173 lines), `🐍️.py` (517 lines).

- **Does not link the plugin crate — confirmed.** `🦀️.rs:38-42` mounts `🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` by `#[path]` and duplicates the 26-kind `KINDS` list by hand (`:53-80`, comment: "duplicated, not imported, because this host must not link the plugin crate"). It reads the 5 committed files per vector via `ctx.fixture_json`/`asset://` URIs and asserts two metamorphic laws in-role: `mutation_is_observable` (footprint moved) and `footprint_law` (before/after diff = declared diff fields, `:184-213`), since the feature has no `@oracle-` implementation tag to compare against, per its own doc comment (`:11-15`).
- **Wiring**: no `[[test]]` entry exists in `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` for this file — it is **not** a Cargo integration test. It is discovered and driven by the separate repo-wide test module `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/` (its own `🏃️runner/🦀️.rs` + `📜️script.ts`), which builds a resolved run-plan (`.🧬semio/🦑️repo/⚡️cache/tests/work/…/📋️plan.json`) from the `.feature`'s `Examples` tables and asset URIs, and is distinct from `bun ./📜️script.ts test` in the puzzle package (`TestScript`, `:19-23`), which just runs `runCargoTestBudgeted(["semio-s-plugin-puzzle"], …)` — i.e. plain `cargo test/nextest -p semio-s-plugin-puzzle`, covering the per-leaf `🧪️tests/<scenario>/🦀️.rs` files from §2, not this cross-language case.
- **`.feature`** (`🥒️.feature`): one `@capability-puzzle-2d-1-mutate @oracle-puzzle-2d-python-independent @comparison-ordered-json-v1` feature with 3 scenario groups: `Scenario Outline … mutate-<id>` (26 rows), `Scenario Outline … inverse-<id>` (26 rows), and one plain `Scenario … identity-round-trip`. Each row's `vector` column is a literal path like `🌱create-node/🧪️tests/🌱️appends-node-c`.

**🚫 Stale vector paths found.** 4 of the 26 `vector` paths in `🥒️.feature`'s Examples tables point at scenario directories that **do not exist on disk** — the actual directories carry random hex suffixes that don't match the feature file:

| Mutation | Feature references | Actual directory |
|---|---|---|
| ➖remove-node-handle | `…/🚫️removes-handle-2-and-severs-edge` | `…/🚫️removes-handle-2-e261f3` |
| 💔disconnect-kind-compatibility | `…/🚫️removes-handle-kind-pair` | `…/🚫️removes-handle-5fccd0` |
| 📚replace-kind-catalogs | `…/📇️installs-handle-kind-catalog` | `…/📇️installs-handle-a0eaf8` |
| 🤝connect-kind-compatibility | `…/🤝️adds-handle-kind-pair` | `…/🤝️adds-handle-kind-d59cb5` |

Verified both ways (`test -d` on the feature's literal paths → all 4 missing; `ls` on the real `🧪️tests/` dirs → the hex-suffixed names are what's actually there). The cached run plan `.🧬semio/🦑️repo/⚡️cache/tests/work/test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-◻️mutate-puzzle-2d-1-oracle-python/📋️plan.json:170-201` (and further down for the other 3) shows the plan was built with these same stale paths, so **a fresh (non-cached) run of this case would fail 4 of its 26+26 rows at fixture-resolution time**, not at law-assertion time. The real scenario directories carry mtimes of 2026-09-01 (`stat` on `➖remove-node-handle/🧪️tests/🚫️removes-handle-2-e261f3` → `Sep 1 22:25:41`), while the `.feature` file was last touched 2026-09-05 19:04 by commit `b0dfa0f09b` ("🔄️Repair artifact owner taxonomy and dependent Gherkin, test, normalization, and manifest references") — whatever repair pass that commit ran, it did not fix these 4 references. **Any recipe that adds/renames scenarios (§6) must also regenerate this feature file's Examples tables**, and this pre-existing breakage should be fixed regardless.

## 4. The Python second implementation `🐍️.py`

- **Registration**: `BASE/🔮️oracle/🔣️.json` (`oracles[0]`, `:6-51`) — `id: "puzzle-2d-python-independent"`, `ecosystem: "python"`, `comparisonProfiles: ["ordered-json-v1"]`, `kind: "verified-native-second-implementation"` (promoted from `cross-semio-implementation` per `[2026-09-02, D1]` in the rationale text, ticket `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`), `fixtureCoverage.vectors: 26`. `noOracleDecisions: []` — this registration *supersedes* an earlier no-oracle decision.
- **Invocation**: registered as `python` under `adapters` in the run-plan (`📋️plan.json:26`) alongside `rust`; driven by the same repo test module, imports `from semio_repo_test import Adapter, Outcome` (`🐍️.py:38`) — a shared, dependency-free test host shim, not the plugin.
- **Refusals — confirmed both**:
  - `UNDERDETERMINED = {"replace-node-handle"}` (`:82`) — `apply_mutation`/`inverse_mutation` both `raise AssertionError(... UNDERDETERMINED_REASON)` for it (`:205-206, 295-296`). Reason: 3 different rules (unimplemented / handle attached to an edge / incompatible kind) all produce the same no-op, and nothing committed disambiguates them.
  - `inverse-replace-kind-catalogs` (`:344-356`) — refuses to guess whether the verb accepts a `null` argument, since the committed vector only exercises *installing* a catalogue, never removing one; cites `mutate-puzzle-5d-1`'s `null-catalogs-is-noop` vector as the sibling that actually settles this.
- **Cached last run**: `.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-◻️mutate-puzzle-2d-1-oracle-python/🔣️.json` — contains only `{"kind":"semio-test-output","testId":"…::◻️mutate-puzzle-2d-1","cacheKey":"a316f9aedc9259eb88f9fdab95e00c70"}` with an empty `📦️artifacts/` dir (mtime 2026-09-05 06:27-06:30). **No pass/fail/error detail is recorded in the cache** — it's a cache-hit pointer, not a result log; the matching `…/work/…/📋️plan.json` (949 lines) is the resolved run plan, not an outcome. Given the plan's stale fixture paths (§3), whether this cached entry reflects a genuinely green run or a run that predates the scenario renames could not be determined from the cache alone.

## 5. Sibling comparison (delegated to a background agent; findings synthesized)

**Key finding: every mutation kind in every artifact examined (fem2d, gismap, cad, block-2d/3d/5d, procedural, writer, and more — 1,590+ `🧬️mutations/*/🧪️tests` dirs surveyed) has exactly one test scenario.** Puzzle-2d's "1 scenario per mutation" is the repo-wide norm, not a puzzle-2d deficiency. `block/🖐️5d` has more mutation *kinds* (41) but, like puzzle-2d, 100% `"applied"` outcomes — not qualitatively richer.

The qualitatively richest sibling is **`🔱️trinity/🔌️jack`** (only 8 kinds, but real coverage diversity):
- 4 of 8 scenarios are genuine `"status": "rejected"` with structured `code` (`mutation.invariant`, `mutation.duplicate-id`, `mutation.target-missing`) + `path`, e.g. `🌉️create-edge/🧪️tests/🚫️rejects-an-edge-3b7777/🎯️outcome/🔣️.json:1-7`.
- Rejected scenarios use the `🔺️diff/🚫️.absent` sentinel (confirmed 0-byte file) instead of `🔺️diff/🔣️.json` — exactly the D6 contract from §2.
- Node/edge ids are drawn from jack's own shipped Nakagin example (`"shaft-to-capsule-a"`, graph `"name":"Capsule Stack"`) rather than synthetic `node-a`/`node-b` placeholders — the real-world-derived naming puzzle-2d's own `📚️examples` could similarly feed (see §6).
- Directory shape is otherwise identical to puzzle-2d's own (`🦠️mutation/🔣️.json`, `📸️snapshot/⬅️before|➡️after/🔣️.json`, `🔺️diff/🔣️.json`, `🎯️outcome/🔣️.json`, `🦀️.rs`).

**Recommendation for §6**: emulate jack's outcome-status diversity (`applied` no-op + `rejected` with `code`/`path`) and its `🚫️.absent` convention, and pull node/edge ids from puzzle-2d's own `📚️examples` the way jack pulls from its Nakagin manifest — not `block`'s larger-but-uniform kind count.

## 6. Recipe for adding fixture scenarios

**No fixture-generator exists for puzzle-2d.** The `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/REGISTER-FIXTURE-GENERATORS-IN-NX-AND-LAUNCH` ticket registered 38 `🏭️generator/📜️script.ts` projects in Nx/launch.json — `grep -i puzzle` on its `📓️inventory.md` returns **zero hits**; puzzle is entirely out of that ticket's scope. Those 38 generators are for *third-party-generated* binary/text fixture classes (mesh, gltf, obj, gif, jpeg, png, …), driven by the repo test module's `FixtureScript` (`🧰️framework/…/🧪️test/📜️script.ts:1069-1166`, subcommands `verify|audit|reproduce|generate`) — but that `generate` path only iterates fixtures where `class === "third-party-generated"` (`:1148`). Puzzle-2d's mutation fixtures are `class` = hand-authored specification vectors (confirmed: `BASE/🔮️oracle/🔣️.json` rationale says "all 130 of its fixtures are handcrafted specification vectors", 130 = 26 kinds × 5 core files), so this command does not apply to them at all. Separately, the puzzle package's own `bun ./📜️script.ts fixtures` (§2) only implements the `lint` subcommand — any other subcommand (including the `generate` its own `DERIVED_CASE_FILES` comment references, `📜️script.ts:82-84`) hits the `default:` branch and exits 1 with a usage error (`:246-250`). **`fixtures generate` is aspirational text in a comment, not a working command**, for both the derived-codec files (.op/.spr/.patch.semio) and for new hand-authored scenarios.

Given that, adding N new scenarios per mutation is a hand-authoring exercise, following the §2 template and drawing real-world content from the two shipped examples:

1. **Get real document JSON from an example.** `📚️examples/🌲️concrete-forest/🦀️.rs:30-35` and `📚️examples/🏗️nakagin-capsule-tower/🦀️.rs` show the pattern: `document_json()` calls `crate::artifacts::puzzle2d::dsl::parse_dsl(DSL_TEXT)` (reading `🖼️assets/🌲️forest/🗣️.dsl.semio` or the tower's equivalent) then `dsl::ToValue::to_value(&projection)` → `dsl::json::to_json_string`. There is no plain `🔣️.json` snapshot of either example committed anywhere — only `.dsl.semio`/`.pack.semio`/`.spr.semio`/`.op.semio`. To get a real `⬅️before/🔣️.json`, write a small throwaway test/bin *inside the plugin crate* (which, unlike the `◻️mutate-puzzle-2d-1` cross-language case, may link the crate) that calls this same `parse_dsl` + `to_value` + `to_json_string` chain and dumps the result — this is exactly the fixture-authoring gap the missing `fixtures generate` command would otherwise fill.
2. **Pick one mutation, hand-author the payload** (`🦠️mutation/🔣️.json`) against a node/edge/handle id that actually exists in the example JSON — mirroring jack's practice (§5) of using real ids (`shaft`, `capsule-a`) instead of synthetic ones.
3. **Compute `➡️after` and `🔺️diff` from production code, not by hand.** The same throwaway crate-linked harness can call `apply_puzzle2d_mutation`/the mutation's own `diff()` (as the `🌱️appends-node-c/🦀️.rs` template does in its own `#[test]`s, §2) to get an authoritative after-snapshot and diff, then commit those outputs as the two JSON files — never hand-derive them, since `produces_committed_diff`/`committed_diff_applies_to_after` in the template assert the committed diff against exactly this production computation.
4. **Write `🎯️outcome/🔣️.json`.** For a normal case, `{"status":"applied"}`. To add rejected-outcome coverage (currently 0/26, see §5), pick a case production actually rejects (e.g. `create-node` with a duplicate id → `mutation.duplicate-id`, per `🌱create-node/🔺️diff/🦀️.rs:9`) and write `{"status":"rejected","code":"mutation.duplicate-id"}` plus a `🔺️diff/🚫️.absent` sentinel instead of `🔺️diff/🔣️.json` (contract D6, §2).
5. **Write the scenario's `🦀️.rs`** by copying `🌱️appends-node-c/🦀️.rs`'s 7 `#[test]` fns verbatim and swapping only the mutation name/kind-specific assertions (node count, which fields moved, etc).
6. **Update `◻️mutate-puzzle-2d-1/🥒️.feature`'s two Examples tables** to add a row per new scenario id — and while doing so, fix the 4 pre-existing stale `vector` paths found in §3.
7. **Update the Python reference `🐍️.py`** if the new scenario exercises a kind or branch it doesn't yet cover (e.g. a first non-`UNDERDETERMINED` `replace-node-handle` case, or the `replace-kind-catalogs`-with-null-argument case the Python file explicitly refuses today, §4) — otherwise no change needed since `fixtureCoverage.vectors` in `🔮️oracle/🔣️.json:44-49` would need bumping from 26 to the new total.
8. **Before trusting `bun ./📜️script.ts fixtures lint`** to gate this work, be aware it currently never discovers puzzle-2d's mutations tree at all (§2's discovery-mismatch finding) — either fix `discoverArtifacts`'s leaf filter to also accept the bare `<leaf>/🦀️.rs` convention, or validate new scenarios purely against the `lintCase` file-shape rules (§2) manually until that's fixed.

## Evidence index (key files read in full or in relevant part)
- `BASE/🧬️schema/🧬️mutations/🌱create-node/{🔣️.json,🧬️.schema.json,🦀️.rs,↩️inverse/🦀️.rs,🔺️diff/🦀️.rs,🧪️tests/🌱️appends-node-c/*}`
- `BASE/🔌replace-node-handle/🔺️diff/🦀️.rs` lines 13-21 (bug)
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts` (full, 287 lines)
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` (full)
- `BASE/🧪️tests/◻️mutate-puzzle-2d-1/{🦀️.rs (292 lines),🥒️.feature (173 lines),🐍️.py (excerpts)}`
- `BASE/🔮️oracle/🔣️.json` (oracle registration + rationale)
- `.🧬semio/🦑️repo/⚡️cache/tests/{results,work}/test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-◻️mutate-puzzle-2d-1-oracle-python/*`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json` (excerpts: MutationOutcomeClass, ManifestMutation, FixtureBundle)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` (FixtureScript, DslScript excerpts)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/REGISTER-FIXTURE-GENERATORS-IN-NX-AND-LAUNCH/{🎫️ticket.json,📓️summary.md,📓️inventory.md}`
- `BASE/📚️examples/{🌲️concrete-forest,🏗️nakagin-capsule-tower}/🦀️.rs`
- Background-agent transcripts: full 26-mutation catalog (schema/rs line citations per kind); sibling richness survey across fem2d/gismap/cad/block/trinity-jack (1,590+ test dirs surveyed).
