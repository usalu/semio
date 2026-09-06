# Remodeling artifact — mutation catalog & fixture-test audit

Scope: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/`.
Read-only audit. No builds run (host constraint). All claims below are either a direct file
read/grep or a static path-resolution check; anything not directly verified is marked UNVERIFIED.

## 0. Executive summary — two compile-breaking regressions found, live

This plugin is currently in a **broken, non-compiling state** on disk (static evidence only — no
build was run, per the ban on compilers this hour). Two independent, systemic defects:

1. **Async/sync trait-signature mismatch (P0).** All 35 `RemodelingMutation` kinds implement
   `protocol::MutationKind::diff/inverse/label` (and 18 of them `::target`) as `async fn`, but the
   framework trait (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs:215-235`)
   declares all four as plain sync `fn`. Same bug on `store::ArtifactDsl::parse_dsl/print_dsl` (sync
   in the trait at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4897-4901`) at 3 impl
   sites. **123 signature mismatches** in the mutation kinds alone (35+35+35+18). This is the exact
   "blanket async codemod" pattern that was the block plugin's root compile blocker. `🧩️puzzle`'s
   `🧊️3d` subset has **zero** such mismatches (verified) — remodel is the outlier.
2. **Dangling fixture paths from an un-synced rename (P0).** Every one of the 34 non-`commit-
   reconstruction` mutation kinds' fixture-case directory was renamed (descriptive slug →
   `<slug>-<6-hex>`, e.g. `🎥️adds-stream-c-bound-to-cam-b` → `🎥️adds-stream-c-458900`) but the three
   places that reference the OLD names by literal path were not updated: the `#[path]` mounts in the
   crate wiring file, the `include_str!` calls in the Rust test adapter, and the `asset://` Examples
   table in the `.feature` file (the Python reference has the same stale names). Confirmed via
   `git show --stat 3a6a9d6bfc` — the current HEAD commit renamed 16 of these directories (the other
   18 were renamed earlier) without touching any of the three referencing files.

Both are almost certainly the effect of two different automated/other-session changes landing on
this plugin concurrently (an async codemod, and a fixture-directory rename) rather than local
authoring — consistent with "other agents are working on the same repo." Neither can be fixed here
(read-only), but both are exact, line-numbered, and reproducible without a build.

## 1. Mutation catalog — authoritative count: 35

Source of truth: `🧬️schema/🧬️mutations/🦀️.rs:20-56`, the `RemodelingMutation` enum
(`#[derive(dsl::Mutations)]`). 35 variants, one directory each under `🧬️schema/🧬️mutations/`
(37 dirs total minus `💾️binary` and `📝️text`, which are shared wire-format/grammar scaffolding for
the whole enum — ABNF/EBNF/ANTLR grammars, Kaitai/Spicy binary specs, `.ts`/`.proto` — not mutation
kinds, and correctly carry no `🧪️tests`).

| # | Variant | kebab id | dir |
|---|---|---|---|
|1|CreateStream|create-stream|🌱create-stream|
|2|DeleteStream|delete-stream|🪓delete-stream|
|3|ChangeStreamSync|change-stream-sync|⏱️change-stream-sync|
|4|AddStreamFrame|add-stream-frame|➕add-stream-frame|
|5|RemoveStreamFrame|remove-stream-frame|➖remove-stream-frame|
|6|ReplaceStreamSource|replace-stream-source|🔁replace-stream-source|
|7|CreateAsset|create-asset|🧷create-asset|
|8|DeleteAsset|delete-asset|🗞️delete-asset|
|9|CreateCameraCalibration|create-camera-calibration|🔭create-camera-calibration|
|10|UpdateCameraCalibration|update-camera-calibration|🛠️update-camera-calibration|
|11|DeleteCameraCalibration|delete-camera-calibration|🚫delete-camera-calibration|
|12|CreateRigExtrinsic|create-rig-extrinsic|⛓️create-rig-extrinsic|
|13|DeleteRigExtrinsic|delete-rig-extrinsic|✂️delete-rig-extrinsic|
|14|UpdateRigExtrinsic|update-rig-extrinsic|🔩update-rig-extrinsic|
|15|CreateGcp|create-gcp|🧿create-gcp|
|16|DeleteGcp|delete-gcp|🚮delete-gcp|
|17|AddGcpObservation|add-gcp-observation|🔎add-gcp-observation|
|18|RemoveGcpObservation|remove-gcp-observation|🚷remove-gcp-observation|
|19|UpdateIngestParams|update-ingest-params|🥣update-ingest-params|
|20|UpdateFeatureParams|update-feature-params|🌠update-feature-params|
|21|UpdateMatchParams|update-match-params|🪢update-match-params|
|22|UpdateSfmParams|update-sfm-params|🧮update-sfm-params|
|23|UpdateDenseParams|update-dense-params|🌁update-dense-params|
|24|UpdateMeshParams|update-mesh-params|🕸️update-mesh-params|
|25|UpdateMotionParams|update-motion-params|🏎️update-motion-params|
|26|UpdateGeoParams|update-geo-params|🌐update-geo-params|
|27|ReplaceJob|replace-job|🏗️replace-job|
|28|ReplaceSparse|replace-sparse|⭐replace-sparse|
|29|ReplaceDense|replace-dense|☁️replace-dense|
|30|ReplaceMeshResult|replace-mesh-result|🧱replace-mesh-result|
|31|ReplaceTrajectory|replace-trajectory|🛣️replace-trajectory|
|32|ReplaceTracks|replace-tracks|🚂replace-tracks|
|33|ReplaceGeoProducts|replace-geo-products|🗾replace-geo-products|
|34|ReplaceQc|replace-qc|🧾replace-qc|
|35|CommitReconstruction|commit-reconstruction|🏁commit-reconstruction|

`💾️binary`/`📝️text` at the ticket's request: confirmed non-mutation-kind support dirs, correctly
have no `🧪️tests`. `🔁replace-stream-source` DOES have a `🧪️tests` dir with a fixture (contrary to
the ticket brief's assumption) — the only true structural exception is `commit-reconstruction`.

## 2. Diff / inverse / tests matrix

Every one of the 35 kinds has both `🔺️diff/` and `↩️inverse/` sibling dirs (verified by directory
listing of all 37 entries under `🧬️mutations/`). 34/35 also have a `🧪️tests/` dir; only
`🏁commit-reconstruction` lacks one, and its `🦀️.rs`/`↩️inverse`/`🔺️diff` are hand-written rather
than `dsl::MutationLeaf`-derived boilerplate.

Every kind's `🧪️tests/` holds **exactly one** fixture case (not "cases", plural — the catalog is
1-deep, not exhaustive over edge cases), each a 5-file quintet: `📸️snapshot/⬅️before/🔣️.json`,
`🦠️mutation/🔣️.json`, `🔺️diff/🔣️.json`, `📸️snapshot/➡️after/🔣️.json`, `🎯️outcome/🔣️.json`, plus a
`🦀️.rs` component-test file mounted by the crate wiring.

**Every fixture's `before.json` is byte-identical** (md5 `c08b3cfb3359a081d855cfb220841106`) across
all 34 kinds — a single shared "populated scene" fixture (2 streams, 1 camera calibration, 1 rig
extrinsic, 1 GCP with 1 observation, a tiny sparse/dense cloud, a box mesh) matching the
`populated_scene_fixture()` builder duplicated in `🧬️mutations/🦀️.rs`'s own `#[cfg(test)]` module.

Realism verdict: **moderate structural breadth, shallow scale** — every optional/collection field
is touched at least once (good for schema-shape coverage), but every payload is minimal: dense
cloud = 2 points, sparse cloud = 4 points (`⭐replace-sparse` fixture), mesh mutations reference a
content-addressed child handle rather than embedding real geometry, camera intrinsics are plausible
but a single fixed set (`fx=fy=1000, cx=512, cy=384, brownConrady`), and there is exactly one stream
pair, one GCP, one rig. Nothing approaches "real-world photogrammetry scale" (no multi-hundred-frame
streams, no multiple cameras/rigs, no realistic distortion coefficient sets, no GCP clusters). Every
outcome file inspected is the trivial `{"status": "applied"}` — there is **no fixture exercising a
refusal/error path** for any of the 34 "happy path" kinds; `commit-reconstruction` is the only kind
whose vector is a refusal (`mutation.invalid-reconstruction-sparse`).

**Gap priority for "exhaustive mutations with real-world tests":**
1. No error/refusal-path fixtures for 34/35 kinds (only the happy path is exercised).
2. No fixture at any kind exercises scale (many streams/frames/cameras/GCPs) — every kind shares
   the same toy base scene.
3. Two fixture-case directory names are non-descriptive placeholders: `⏱️change-stream-sync/🧪️tests/
   t038` and `🕸️update-mesh-params/🧪️tests/t039` (every other kind has a descriptive
   `<emoji><slug>-<hash>` name) — looks like leftover auto-generated naming, worth renaming for
   consistency (not a correctness bug).
4. Single-case-per-kind coverage means no kind has more than one committed vector; the harness
   files (`.feature`/`.rs`/`.py`) all assume exactly one case per kind, so adding more cases would
   require harness changes too, not just new fixture dirs.

## 3. Test-harness wiring — fixture discovery is compile-time-literal, not glob/asset-driven

Harness location: `🧪️tests/📸️mutate-remodeling-1/` — `🦀️.rs` (Rust adapter, 420 lines),
`🥒️.feature` (Gherkin, 180 lines), `🐍️.py` (independent second implementation, 497 lines),
`🧫️fixtures/` (3 files local to this case, used only by `commit-reconstruction`).

**Discovery mechanism — hardcoded literal paths, not a directory scan:**
- Rust: `fixture_text(kind)` (`🦀️.rs:110-289`) is a `match` over 35 kind strings, each arm calling
  `include_str!("../../🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/...")` — a **compile-time**
  macro with the fixture-case slug spelled out literally.
- Feature: the `Scenario Outline`'s `Examples` table (`🥒️.feature:69+`) has one row per kind with
  `dir`/`fixture` columns built into `asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/...` URIs
  — same literal-slug pattern, just data instead of code.
- Python: `VECTORS` dict (`🐍️.py:59+`) maps kind → `(dirname, fixture, wireTag)` tuples, same
  literal slugs, joined into an `asset://` URI at `🐍️.py:110-111`.

**All three currently reference the PRE-rename slugs and are therefore all stale** (spot-checked
`create-stream`: all three say `🎥️adds-stream-c-bound-to-cam-b`; on-disk is
`🎥️adds-stream-c-458900`). Full check of the Rust file's 102 `include_str!` targets (34 kinds × 3
files + 3 local `commit-reconstruction` files): **99/102 dangling, only the 3 local
`commit-reconstruction` ones resolve** (script + output below). Since `include_str!` is evaluated
at compile time, this is not a test failure but a **crate-level compile error** for the whole
generated test host.

**Comparison with `🧩️puzzle`'s `🧊️3d` subset (`🧪️tests/🧊️mutate-puzzle-3d-1/🦀️.rs`):** puzzle's
harness does NOT hardcode per-kind fixture paths at all. Its `vector()` function
(`🦀️.rs:113-124`) reads the kind AND the five file paths (`before`/`mutation`/`diff`/`after`/
`outcome`) out of the `.feature` scenario's own doc-string JSON via `ctx.doc_json()`, then resolves
each at runtime with `ctx.fixture_json(&spec.str("before"))` (an `asset://`-backed runtime
resolver). A fixture-directory rename there only requires updating the `.feature` file's doc
strings — a data change, not a compile-time literal in three separate files. Puzzle's fixture-case
naming is also hash-free (`↔️slides-reference-1`, a plain incrementing suffix), so it never
collides with the hash-suffix scheme remodel's fixtures were just migrated to. **Architecturally,
remodel's harness is the more fragile of the two**, and this rename is exactly the failure mode
that fragility predicts.

**`commit-reconstruction` exclusion — verified legitimate, not an oversight.** Both the `.rs` module
doc (`🦀️.rs:17-21`) and the `.feature` description (`🥒️.feature:38-46`) state the same reason: its
`diff` reads process-global staging state (`commit_staged_remodeling_reconstruction`,
`durable_staged_remodeling_asset`) that a static `(before, mutation, after)` triple structurally
cannot carry. Its vector is instead assembled once from two OTHER kinds' committed leaf fixtures
(`replace-job`'s before + `replace-sparse`'s payload) and exercises the kind's own documented
refusal path (`mutation.invalid-reconstruction-sparse`) rather than a real commit. The `.feature`
file itself flags a real, acknowledged weakness: `commit-reconstruction`'s inverse restores only
`job` and the six result slots, never `assets`/`durable_artifacts`, so the inverse law is untested
for a commit that actually published new assets. **Recommendation: keep it excluded from the
static-triple harness as-is; the honest fix is a harness capability to seed/inspect the process-
global staging state, not a fixture change.**

**Stale claim worth flagging:** `🔮️oracle/🔣️.json`'s `rationale` field asserts "All 34 kinds'
forward and inverse scenarios were executed standalone ... all 68 passed on the first run." Given
the fixture-directory rename postdates whatever commit that claim was written against, and every
`include_str!`/`asset://` reference is now dangling, **that pass claim cannot currently be true** —
UNVERIFIED whether it was true when written; it is unverifiable now without a build.

## 4. `#[path]` mount resolution — script + results

Script (read-only, no edits made by it): `🐍️mount-check.py` in this ticket folder. It resolves
every `#[path = "..."]` in `📦️packages/🦀️rust/🦀️.rs`, `✏️s/🔌️plugins/📸️remodel/🦀️.rs`, and every
`🦀️.rs` under the artifact tree, relative to the declaring file, and reports non-existent targets
(a bare `"."` self-mount is always treated as resolving).

```
$ python3 🐍️mount-check.py
# scanned .rs files: 261
# total #[path] attributes: 394
# dangling (non-'.' targets that do not exist): 34
```

All 394 `#[path]` attributes live in `📦️packages/🦀️rust/🦀️.rs` (the wiring-only file; the second
entry file `✏️s/🔌️plugins/📸️remodel/🦀️.rs` has none). All 34 dangling ones are the per-fixture-case
test-module mounts, one per non-`commit-reconstruction` kind, e.g.:

```
📦️packages/🦀️rust/🦀️.rs:140: path='../../🗿️artifacts/📸️remodeling/.../🌱create-stream/🧪️tests/🎥️adds-stream-c-bound-to-cam-b/🦀️.rs'
    resolved -> .../🌱create-stream/🧪️tests/🎥️adds-stream-c-bound-to-cam-b/🦀️.rs   (MISSING; real dir is .../🎥️adds-stream-c-458900/)
```

(full 34-line list at lines 140,153,166,179,192,205,218,231,244,257,270,283,296,309,322,335,348,
361,374,387,400,413,426,439,452,465,478,501,514,527,540,553,566,579 — one per kind, same rename as
§3). This means **the plugin crate itself fails to compile**, independent of the test-harness
breakage in §3 (that harness is a separate generated-host crate; this is the plugin's own crate).

## 5. `async fn` drift vs sync framework traits

Framework trait signatures (verified, all plain sync `fn`, no `async`):
- `protocol::Mutation<P>::diff/inverse` — `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:152-153`
- `protocol::MutationKind<P,Op>::diff/inverse/label/target` — `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs:219-233`
- `store::ArtifactDsl::parse_dsl/print_dsl` — `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4900-4901`
- (by contrast, `store::ArtifactPack::pack_at_checkpoint` genuinely IS `async fn` — VCS I/O — so not
  every async-looking method here is a bug; only the ones on the four names above are.)

Remodel impl sites, all declared `async fn` against these sync traits (grep, non-test files only):

| trait method | # remodel impls declared `async fn` | expected |
|---|---|---|
| `MutationKind::diff` | 35 (all kinds) | sync `fn` |
| `MutationKind::inverse` | 35 (all kinds) | sync `fn` |
| `MutationKind::label` | 35 (all kinds) | sync `fn` |
| `MutationKind::target` | 18 (kinds that override it) | sync `fn` |
| `ArtifactDsl::parse_dsl`/`print_dsl` | 3 impls × 2 methods = 6 (`🧬️schema/📸️snapshot/🦀️.rs:84,92`; `✏️editor/🎚️config/🦀️.rs:94,102`; `✏️editor/👥️presence/🦀️.rs:45,56`) | sync `fn` |

**123 mismatches in the mutation-kind impls alone**, e.g. `🌱create-stream/🦀️.rs:29,32,35`:
```rust
impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for CreateStream {
    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> { ... }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> { ... }
    async fn label(&self) -> String { ... }
}
```
against a trait that declares `fn diff(...) -> MutationOutcome<...>;` (no `async`, no `Future`
return type) — an `async fn` impl of a non-async trait method is a hard `E0053`-class compile
error in Rust, not a runtime issue. Every one of the 35 kind files has this same shape (all-`fn`
grep at `🧬️mutations/*/🦀️.rs`, one occurrence each of `diff`/`inverse`/`label`, 18 of `target`).

Comparison plugin `🧩️puzzle` (`🧊️3d` subset, same `MutationKind` trait, same derive machinery):
**zero** `async fn diff/inverse/label/target` anywhere in its `🧬️mutations/*/🦀️.rs` (verified by the
same grep) — e.g. `🎯move-reference/🦀️.rs:22` is plain `fn diff(...)`. This confirms remodel is a
local regression, not a framework-wide convention change; `🌀️procedural` was not independently
checked (UNVERIFIED, out of budget) but the puzzle comparison alone is sufficient to establish drift.

`async fn` totals for context: 801 occurrences under the artifact tree total, 36 in `🧪️tests/`
dirs (fine — feature-test scaffolding), 131 in non-test files. Most of the 131 are legitimate
(command handlers under `✏️editor/🎮️commands/*` are async per the framework's own interaction
convention — UNVERIFIED whether every one of those is correctly async, out of scope for this pass);
the 123 counted above (schema/mutation layer + ArtifactDsl) are the ones proven wrong against a
concretely sync trait.

## 6. Priority gap list

1. **P0 — fix the async/sync trait mismatch** (§5): revert `async` on `MutationKind::diff/inverse/
   label/target` in all 35 `🧬️mutations/*/🦀️.rs` files and on `ArtifactDsl::parse_dsl/print_dsl` in
   the 3 sites listed. Plugin will not compile until this lands.
2. **P0 — resync fixture-path references after the rename** (§3, §4): update the 34 `#[path]`
   mounts in `📦️packages/🦀️rust/🦀️.rs`, the 99 dangling `include_str!` targets in
   `🧪️tests/📸️mutate-remodeling-1/🦀️.rs`, the `.feature` Examples table, and the Python `VECTORS`
   dict to the current hash-suffixed directory names — or, better, adopt puzzle's `asset://` +
   doc-string pattern so a future rename doesn't repeat this break.
3. **P1 — add error/refusal-path fixtures** for the 34 happy-path-only kinds (§2).
4. **P1 — add at least one larger-scale fixture** (multi-camera rig, many-frame stream, populated
   GCP network) to validate the mutations against something closer to real photogrammetry scale.
5. **P2 — rename the two placeholder fixture-case dirs** `t038`/`t039` to descriptive slugs matching
   the other 32 kinds' naming convention.
6. **P2 — re-verify or retract** the `🔮️oracle/🔣️.json` "all 68 passed" claim once the crate compiles
   again; it is not currently a true statement about the repo's state (§3).

## Appendix — commands/scripts used

- `🐍️mount-check.py` (this ticket folder) — `#[path]` resolution checker, output reproduced in §4.
- `git show --stat --format='' 3a6a9d6bfc -- <🧬️mutations path>` confirmed the current HEAD commit
  renamed 16 fixture-case directories (e.g. `🌱create-stream`'s) without touching the harness files.
- md5 comparison across all 34 `📸️snapshot/⬅️before/🔣️.json` files confirmed the single shared base
  fixture (§2).
