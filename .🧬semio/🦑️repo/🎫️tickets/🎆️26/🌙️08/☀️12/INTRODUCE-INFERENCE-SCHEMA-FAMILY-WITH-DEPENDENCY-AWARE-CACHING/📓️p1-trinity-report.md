# P1 — trinity `recompute_derived` deletion + flat-position inference (executor report)

## What changed

### 1. New inference slug: `🎛flat-position`
Created `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flat-position/{🦀️component.rs,🟦️component.ts}`.

- `JackFlatPositionUv { u: f64, v: f64 }` + `JackFlatPosition { positions: BTreeMap<String, JackFlatPositionUv> }`, both plain `derive(Clone[, Copy], Debug, Default, PartialEq, Serialize, Deserialize)` — no hand-rolled `Default` needed (unlike `🧭topology`'s `cycle_free` special case): an empty snapshot naturally yields an empty `positions` map, which is honestly the default.
- `compute_flat_position(&JackSnapshot) -> JackFlatPosition` ports `Graph::recompute_derived`'s BFS/seed algorithm verbatim, operating on a `BTreeMap<String, &Node>`/`BTreeMap<String, &Edge>` built from the snapshot (mirrors exactly how `Graph::from_fixture` indexes into its own `BTreeMap`s, so iteration order — and therefore output — is byte-identical to the old method). Two private helpers `has_incoming_from_remaining` / `extend_from_seed` are the direct ports of the deleted `Graph::has_incoming_from_remaining` / `Graph::extend_flat_positions_from_seed`.
- A pure-fn leaf (no `InferredField`), matching `🧭topology`'s sanctioned shape and doc-comment rationale (single whole-snapshot BFS pass — a merkle dep-chain would cost more than the fold it caches).
- 4 tests ported 1:1 from the deleted `🗿️artifacts/🔌️jack/🦀️component.rs` tests, re-expressed against `compute_flat_position` directly instead of `Graph::recompute_derived` + property-bag reads: `flat_position_bfs_walks_from_root` (was `derived_flat_position_bfs`), `flat_position_covers_disconnected_components` (was `derived_flat_position_covers_disconnected_components`), `flat_position_handles_cycles_without_looping` (was `derived_flat_position_handles_cycles_without_looping`), `flat_position_empty_snapshot_yields_default` (was `recompute_derived_noop_on_empty_graph`).

### 2. Wired into the family
`💡️inferences/🦀️component.rs` (family root): added `flat_position: JackFlatPosition` field (`#[state(inferred)]`) to `JackInference`, wired into `Inference::infer`, added its `InferenceFieldSpec { id: "s.trinity.jack.inference.flatPosition", reads: &["nodes", "edges", "root_node_id"] }`. Added `inference_matches_compute_flat_position_directly` test (mirrors `inference_matches_compute_topology_directly`); the pre-existing `inference_determinism_law`/`inference_default_law` already cover the new field for free since they exercise the whole `JackInference::infer`.

Updated the family-root cross-language leaves to mirror the new field: `🟦️component.ts`, `🔣️component.json`, `🔗️component.graphql`, `🛰️component.proto` (all now declare `flatPosition`/`JackFlatPosition` alongside `topology`/`JackTopology`).

`📦️packages/🦀️rust/📦️glue.rs`: added a `pub mod flat_position { #[path=".../💡️inferences/🎛flat-position/🦀️component.rs"] mod component; pub use component::*; }` block inside the jack `inferences` mount, immediately after the existing `topology` block — same shape, same `#[path="."]` self-mount pattern.

### 3. Deleted
- `Graph::recompute_derived`, `Graph::has_incoming_from_remaining`, `Graph::extend_flat_positions_from_seed` — `🗿️artifacts/🔌️jack/🦀️component.rs` (were :289-356 pre-edit).
- `TrinityRamError::DerivedPropertyReadonly` variant — same file (was :65-66).
- Its mutation use site — `🧬️mutations/🦀️component.rs`'s `validate_set_data_property`: removed the `if def.kind == PropertyKind::Derived { return Err(DerivedPropertyReadonly {..}) }` block; the existence check that used to piggyback on the same `let Some(def) = …` binding is now a plain `if !defs.iter().any(...)` so the (now-unused) `def` binding doesn't trip an unused-variable warning. Left the *other*, unrelated `PropertyKind::Derived` skip in `validate_property_bag_trinity` alone — it's generic manifest infra (skips type-checking any manifest property declared `"derived"`, for whichever kind), not something introduced for `flatPosition` specifically, and no longer reachable now that no manifest declares any derived property, but removing generic infra wasn't asked for and isn't specific to this deletion.
- The manifest's `flatPosition`/`"derived"` property declaration on node kind `Piece` — `🗿️artifacts/🔌️jack/🛂️manifest.jsonnakagin.manifest.json` (trinity's own fixture, the canonical source APA's W3 relocated to this path). Surgical 6-line removal preserving the file's original pretty-printed formatting (see `## Concurrent-churn observations` below for a mistake-and-fix on this).
- 6 tests removed from `🗿️artifacts/🔌️jack/🦀️component.rs` (ported to the new slug, or made redundant by the deletion): `derived_flat_position_bfs`, `derived_flat_position_covers_disconnected_components`, `derived_flat_position_handles_cycles_without_looping`, `recompute_derived_noop_on_empty_graph`, `graph_op_rejects_derived_property_set`; and the `flatPosition`/`PropertyKind::Derived` assertion line inside `manifest_nakagin_has_piece_and_connection` (test kept, assertion line removed).

### 4. Call-site conversions (all 14 ground-truth occurrences)
| # | file | what changed |
|---|---|---|
| 1 | `🎛️apps/🔌️jack/📌️panels/🔍️inspection/🦀️component.rs` | `flat_position_uv` now reads `JackFlatPosition.positions.get(node_id)` instead of `node.properties["flatPosition"]`; `fixture_with_derived` deleted, replaced by a direct `compute_flat_position(fixture)` call in `render`. |
| 2,3 | `🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs` | Both `graph.recompute_derived()` calls (prod `run_main` + 1 test) dropped — nothing in this file ever read `flatPosition`, so it was a pure no-op removal. |
| 4,5 | `🎛️apps/♻️rewrite/🌍️world/🦀️component.rs` | `rebuild_engine`'s call dropped (confirmed dead — the rest of the function reads `node.x`/`node.y` directly, never `flatPosition`; the only consumer of `self.graph.fixture_json()`'s serialized properties bag is the WASM bridge, and no TS/JS in the plugin reads `flatPosition` from it — grepped, zero hits). Test `nakagin_flat_position_derived` converted to call `compute_flat_position(&g.to_fixture())` and read `.positions.get(id)` instead of node properties. |
| 6 | `🎛️apps/♻️rewrite/📌️panels/🔍️inspection/🦀️component.rs` | Same pattern as #1: `flat_position_uv` re-typed to take `&JackFlatPosition`; `fixture_with_derived` deleted, replaced by `compute_flat_position(&fixture)` using the fixture already parsed earlier in `render`. |
| 7,8,10,11 | `🗿️artifacts/🔌️jack/🦀️component.rs` (test calls) | Deleted along with their containing tests (see above). |
| 9 | same file, test name `recompute_derived_noop_on_empty_graph` | Deleted (ported as `flat_position_empty_snapshot_yields_default` in the new slug). |
| 12 | `🎛️apps/♻️rewrite/📌️panels/🔍️inspection/🦀️component.rs` | Same as #6 (this was the same call counted twice in the ground-truth table — panel + its own call site). |
| 13 | `🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🧮️executor/🦀️component.rs` (post-APA-move path) | `mini_graph()`'s call dropped — confirmed dead, no `flatPosition` reference anywhere else in this file. |
| 14 | `🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` | `nakagin_graph()`'s call dropped — same, confirmed dead. |

All 14 ground-truth occurrences accounted for (some rows in the ground-truth table double-counted a single call site across the "line" column; the actual distinct call sites converted/removed: 10, distinct files touched: 9).

## Files touched

**Created:**
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flat-position/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flat-position/🟦️component.ts`

**Updated:**
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️component.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️component.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️component.graphql`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️component.proto`
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🛂️manifest.jsonnakagin.manifest.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🧮️executor/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/🔌️jack/📌️panels/🔍️inspection/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/📌️panels/🔍️inspection/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs`

**Removed:** nothing at the file level (all deletions above are in-file: methods, error variant, tests, manifest property entry).

## Contradicts ground truth?

None found. APA's per-artifact path map (jack, not split into rewrite) matched the brief's Correction 1/2 exactly. One thing the brief didn't spell out and that I had to resolve myself: the manifest's `PropertyKind::Derived`/`flatPosition` declaration is NOT literally at `🗿️artifacts/🔌️jack/🦀️component.rs:507` — that line is a *test* asserting the declaration exists; the actual declaration lives in the manifest JSON data fixture `🗿️artifacts/🔌️jack/🛂️manifest.jsonnakagin.manifest.json` (trinity-owned, in-bounds) which is compiled into a **framework-owned, out-of-bounds mirror** `🧰️framework/🔨️modules/🧮️math/🤖️generated/🦀️nakagin.rs` (`NAKAGIN_MANIFEST_JSON` embedded string) + its TS twin — see below.

## Concurrent-churn / self-inflicted-mistake observations

- **Self-inflicted, caught and fixed before the gate**: my first pass at editing the manifest JSON fixture used a Python `json.load`/`json.dump` round-trip, which silently reformatted the file from 1174 pretty-printed lines down to 1 compact line (semantically identical, but a diff footprint 200x bigger than the actual change). Caught via `git diff --stat` before running the gate. Fixed by restoring `HEAD`'s pretty-printed content (`git show HEAD:<path>`, read-only, no working-tree state touched) and re-applying a surgical 6-line text removal instead. Final diff: `1 file changed, 6 deletions(-)`. Lesson for future agents: never round-trip a large hand-formatted JSON fixture through a generic serializer — edit it as text.
- **Two `#[path = "."]` typos self-caught during the glue.rs edit**: pasted `🏅️标准` (wrong CJK glyphs) instead of `🏅️standards` twice while wiring the new mount — both caught immediately via `grep -n "标准"` and fixed before the gate ran. Neither reached a build.
- No other session's edit landed on any of the 17 touched files during this wave, per `git log --oneline -3` spot-checks before each edit and the `📦️glue.rs` re-read immediately before mounting `flat_position` (topology's own mount block was byte-identical to the ground truth's snapshot, confirming no concurrent churn there).
- **Framework-plugin blocker is NOT cleared** — still red at gate time, confirmed directly (see Verification below), with a *different* error signature than `📌️important.md`'s documented E0499/E0560/E0609: now `E0308` (`kind: self.kind` expected `String`, found `ArtifactKindId`) + 2×`E0599` (`no method named 'plugin' found for struct 'std::string::String'`) in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🦀️component.rs`. UCAS is still mid-propagating (the error shape moved between my first and later gate attempts within the same session — it did not exist verbatim in the ticket's documented form, so it is a live-in-progress rename, not a stale snapshot). Not touched, not ours, `🔌️plugin/🦀️component.rs` is UCAS-frozen to us.
- **A second, independent live blocker surfaced during the gate**: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` has an **uncommitted, in-flight 1-line addition** (`git diff --stat` on that path: `1 file changed, 1 insertion(+)`, confirmed unrelated to any of my edits — I never touched this file) importing `assert_mutation_diff_absorb_law`/`assert_mutation_inverse_law` from `store::os_store::test_support`, neither of which exists yet (`error[E0432]`). This is inside the SAME crate (`semio-s-plugin-trinity` — rewrite and jack are both mounted into one crate via `📦️glue.rs`), so it blocks the whole crate's `(lib test)` target regardless of our own code's correctness. Matches the SMO "currently editing trinity" note in `📌️important.md`'s peer table — SMO is mid-adding these `os_store` test-support helpers elsewhere and hasn't landed them yet.
- **A third, independent blocker**: `semio-s-plugin-stdio` (a genuine transitive dependency of trinity — trinity's `io_registry` composer entries write `stdio.csv`/`stdio.json`/`stdio.md`/`stdio.png`/`stdio.svg` dialects) is red with `error[E0004]` non-exhaustive `match` on `SemioSubsetSnapshot`/`SemioDiff` (missing `Table`/`Graph` arms) in `✏️s/🔌️plugins/🗄️stdio/…/🧿️semio/…/🚪️io/🦀️component.rs`. This is exactly `📓️status.md`'s documented "stdio BLOCKED — UCAS's 🧿️semio roster restructure 13→18 subsets in flight (adds text/table/spatial-object/graph/kit)" — the error count even changed between two consecutive retries (2 errors → 4 errors), confirming it's a live, moving edit, not stale.
- Retried `cargo check -p semio-s-plugin-trinity --all-targets` 3× (not on a fixed interval — each retry naturally spaced by the ~5-15 min build time under this machine's heavy concurrent load from 5+ other sessions). Every retry showed **our own crate's `(lib)` target compiling clean** (only pre-existing warnings, zero from any file we touched) while the specific *external* blocker rotated each time (rewrite-mutations E0432 → framework-plugin E0308/E0599 → stdio E0004×2 → stdio E0004×4) — strong, repeated evidence the red is 100% concurrent churn in other sessions' in-flight files, never our own.

## Framework mirror (`🧰️framework/`) — out of bounds, flagged not fixed

`🧰️framework/🔨️modules/🧮️math/🤖️generated/🦀️nakagin.rs` (and its TS twin `🟦️nakagin.ts`) still embed `flatPosition`/`"kind":"derived"` in their baked-in `NAKAGIN_MANIFEST_JSON` string — a stale mirror of the JSON fixture I edited. `math`'s own `build.rs` only regenerates this file when it's *missing*, not on every source change (it sets `cargo:rerun-if-changed` on the JSON source but the actual `bun ./📜️script.ts generate` invocation is gated behind `if !generated.is_file()`), so editing the trinity-owned JSON source alone does not resync it. Regenerating requires running math's own codegen (`bun nx run @semio-tech/framework-math:generate`), which is explicitly out of my boundary (`🧰️framework/` is off-limits regardless of mechanism — hand edit or generator run). Not touched. Flagging for whoever owns `🧰️framework/🔨️modules/🧮️math/`: the compiled-in nakagin manifest will keep declaring `flatPosition` as a derived node property until that codegen is re-run, even though the JSON source of truth and all trinity-side enforcement of it (`DerivedPropertyReadonly`) are now gone. This is inert (nothing writes `flatPosition` anymore, so the stale declaration is never exercised) but not coherent, and is the honest state of things given the boundary.

## Verification commands run

All runs used `CARGO_TARGET_DIR="<ticket>/🎯️target"`.

### Run 1 — `cargo check -p semio-s-plugin-trinity --all-targets` + `cargo test -p semio-s-plugin-trinity --lib` (full, `scratch-p1-trinity-mygate.txt`)

Trinity's own lib target — clean, no errors from our code:
```
   Checking semio-s-plugin-trinity v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust)
...
warning: `semio-s-plugin-trinity` (lib) generated 51 warnings (run `cargo fix --lib -p semio-s-plugin-trinity` to apply 45 suggestions)
```
(zero hits anywhere in the 10298-line log for `flat_position`, `💡️inferences`, `🎛flat-position`, or `inspection` as an error/warning source — grepped directly.)

`(lib test)` target blocked by SMO's in-flight, uncommitted edit (confirmed via `git diff --stat` on that exact path = `1 file changed, 1 insertion(+)`, a file we never touched):
```
error[E0432]: unresolved imports `store::os_store::test_support::assert_mutation_diff_absorb_law`, `store::os_store::test_support::assert_mutation_inverse_law`
   --> .../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:117:109
    | ...no `assert_mutation_diff_absorb_law` in `os_store::component::test_support`
    | ...no `assert_mutation_inverse_law` in `os_store::component::test_support`
error: could not compile `semio-s-plugin-trinity` (lib test) due to 1 previous error; 49 warnings emitted
CHECK_EXIT=101
```

`cargo test` additionally blocked downstream by the framework-plugin repo-wide blocker (evolved error shape, still red, not ours):
```
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:1114:23
1114 |                 kind: self.kind,
     |                       ^^^^^^^^^ expected `String`, found `ArtifactKindId`
error[E0599]: no method named `plugin` found for struct `std::string::String` in the current scope
    --> .../🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:1164:27 (and :1167:27)
error: could not compile `semio-framework-plugin` (lib) due to 3 previous errors; 40 warnings emitted
TEST_EXIT=101
```

### Run 2 — retry, `cargo check -p semio-s-plugin-trinity --all-targets` only (`b7qcf744z` background task, full tail captured)
Different external blocker this time — trinity's transitive dependency `semio-s-plugin-stdio` red on UCAS's live `🧿️semio` roster restructure:
```
error[E0004]: non-exhaustive patterns: `&...SemioSubsetSnapshot::Table(_)` and `&...SemioSubsetSnapshot::Graph(_)` not covered
  --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:77:15
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error; 592 warnings emitted
```

### Run 3 — retry, same command (`scratch-p1-trinity-retry3.txt`)
Same stdio blocker, error count moved 1→4 (confirms it's a live edit, not a stale snapshot):
```
error[E0004]: non-exhaustive patterns: `&...SemioDiff::Table(_)` and `&...SemioDiff::Graph(_)` not covered  (×3)
error[E0004]: non-exhaustive patterns: `&...SemioSubsetSnapshot::Table(_)` and `&...SemioSubsetSnapshot::Graph(_)` not covered  (×1)
error: could not compile `semio-s-plugin-stdio` (lib) due to 4 previous errors; 590 warnings emitted
EXIT=101
```

### Disk-space sanity check (the ticket's prior blocker)
```
$ df -h /System/Volumes/Data
Filesystem      Size    Used   Avail Capacity
/dev/disk3s5   926Gi   471Gi   391Gi    55%
```
Resolved — not a factor in any of the above.

### `test-quick` nx target
Not run — the crate's own `(lib)` target compiles clean but every attempt at a full `--all-targets`/`--lib test` build was blocked by one of the three external, confirmed-not-ours issues above before reaching test execution. Did not attempt `nx run <trinity>:test-quick` since it would hit the identical compile wall.

## Honest pass/fail

**Partial pass.** All 6 task items (create inference, wire it, delete `recompute_derived`+co, convert all 14 ground-truth call sites, port/extend the 4+2 tests, gate) are done at the source level, and `semio-s-plugin-trinity`'s own `(lib)` target — the actual crate our edits live in — compiles clean with zero errors and zero warnings attributable to any file we touched, across 3 independent gate attempts. **I could not get a fully green `cargo check --all-targets` or `cargo test --lib`** — every attempt was blocked by one of three confirmed-external, confirmed-not-ours, confirmed-live (not stale) issues: SMO's in-flight `♻️rewrite/…/🧬️mutations/🦀️component.rs` edit (same crate, blocks the test target), the framework-plugin repo-wide blocker (evolved shape, still red), and UCAS's `🧿️semio` roster restructure making trinity's transitive `stdio` dependency red. None of these are ours to fix, all are named and out of bounds per `📌️important.md`/`📓️status.md`. Our own tests (the 4 new `flat-position` tests, the 3 family-root law tests — 2 pre-existing + 1 new) have **never actually executed** — no test binary was ever produced, since the crate never fully compiled in test mode. rustc's single E0432 (only error reported, from SMO's file, not a fatal parse-level failure) is suggestive that name/type resolution proceeded far enough to see the rest of the crate including our new test module without finding further errors — but that is circumstantial, not a substitute for a real run. I cannot honestly claim the tests pass, only that: (a) their logic was manually traced against the ported algorithm using the exact same fixtures/expected values as the deleted originals, and (b) no compiler diagnostic of any kind (error or warning) was ever attributed to our files across 3 gate attempts. Re-verification once SMO's `os_store::test_support` helpers land is the coordinator's call.
