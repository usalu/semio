# W4 — `trinity` composes stdio `graph` (2 children)

**ucas-status: blocked-mechanism — did not start the migration. DKM's `math`→`geometry`/`graph` crate-extraction rename is still live/in-progress inside `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/**`, uncommitted, and internally inconsistent (partially applied even within a single file). No trinity file was edited by this pass.**

## Pre-flight check (per brief: verify before starting, STOP if still live)

Per `📌️important.md` §"W4 fan-out tracking": trinity was already skipped once in batch 2b for exactly this reason ("live uncommitted edit found (DKM's own `math`→`geometry`/`graph` crate-extraction rename actively touching `🔱️trinity/🗿️artifacts/🔌️jack/**`"). The brief instructed re-checking `git status`/`git diff --stat` before dispatching this wave, and STOPPING if still live.

```
git status --porcelain -- ✏️s/🔌️plugins/🔱️trinity
```
Result: 6 files staged (index, not committed):
```
M  ✏️s/🔌️plugins/🔱️trinity/🎛️apps/🔌️jack/🎮️commands/🗺️fixture/🦀️component.rs
M  ✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml
M  ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs
M  ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️component.rs
M  ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
M  ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs
```

`git diff --cached --stat` confirms these are the exact edits described in the brief: `Cargo.toml` gained two new deps (`geometry`, `graph`, both pointing at the new standalone framework crates alongside the still-present `math` dep), and 5 jack files switched `math::graph::…`/`math::geometry::…` import paths to `graph::…`/`geometry::…`.

## Why this is judged still-live, not settled-but-uncommitted

1. **Internally inconsistent within a single file.** `…/🗣️language-service/🦀️component.rs`'s staged diff converts only the `manifest` sub-module references (`math::graph::manifest::GraphManifest` → `graph::manifest::GraphManifest`, 4 call sites) but leaves 7 `math::graph::dsl::…` references (lines 6, 119, 270, 414, 455, 491 in the current on-disk file, confirmed by grep) completely untouched, in the same file, same commit-in-progress. A finished, settled rename does not leave two sibling sub-modules of the identical crate half-converted in the same file — this is the signature of a rename mid-execution, stopped partway through a mechanical sweep (`manifest::` done, `dsl::` not yet reached).
2. **Repo-wide grep for the plugin confirms the split is real, not file-local noise**: `grep -rn "math::" ✏️s/🔌️plugins/🔱️trinity` still finds 2 files with live `math::graph::dsl::…` usage — one of them (`🎛️apps/♻️rewrite/🌍️world/🦀️component.rs`) isn't even in the staged set yet, meaning the rename hasn't reached it at all.
3. **Not yet swept by the auto-committer.** `git log -1 --date=iso` on HEAD is `515271bf60`, dated `2026-08-13 13:05:26`. The staged files' mtimes (`stat -f '%Sm'`) are `2026-08-13 14:32:34`–`14:33:15` — over an hour newer than the last auto-commit, and the wall clock at the time of this check was `15:46:40`. The tree has had ample time to auto-commit since 14:33 and has not, consistent with the owning session still holding these files open.

This matches `📌️important.md`'s explicit instruction for this exact scenario: "if it's still live, STOP and report `blocked-mechanism`, do not fight a live rename." No file under `✏️s/🔌️plugins/🔱️trinity/**` was modified by this pass — the working tree is exactly as found.

## Design note for whoever resumes this ticket (not acted on, for context only)

Per `📓️design-full-plan.md` §4 (`trinity→C:graph (jack; rewrite = 2 graph children)`), I read `🗿️artifacts/🔌️jack/🦀️component.rs` and its `🧬️schema` far enough to understand the shape before stopping, since the brief asked for that understanding up front:

- `JackArtifact`/`JackSnapshot` currently hold `manifest: Manifest` (a `TrinityManifest` — the compile-time DSL/kind schema: node kinds, edge kinds, port kinds, property defs) plus `nodes: Vec<Node>`/`edges: Vec<Edge>` (the actual graph instance data, keyed by the manifest's kinds).
- The "2 graph children" the design doc calls for most plausibly map to: (a) the **manifest/schema graph** — the kind-definition structure (`TrinityNodeKindDef`/`TrinityEdgeKindDef`/`TrinityPortKindDef`, now living in the freshly-extracted `graph::manifest` module per the in-flight rename) and (b) the **instance graph** — the actual `nodes`/`edges` scene data, structurally analogous to `dag`'s/`reasoning`'s single content graph. This reading is provisional — I did not go deep enough to commit to field-level mappings, since the migration itself was blocked before starting; the next agent should re-derive this independently against whatever state `math`/`geometry`/`graph` settle into.
- stdio's `graph` subset schema was NOT independently re-read in depth this pass (blocked before the schema-mapping step); `dag-report.md` and `reasoning-report.md` (§ their "Composed-child bridge" sections) are confirmed present at `📓️wave4-reports/` as precedent, per the brief.

## sharedFileRequests

None — no file was written this pass.

## Concurrent-churn observations

- **DKM's `math`→`geometry`/`graph` crate-extraction rename is live inside `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/**` and `📦️packages/🦀️rust/Cargo.toml`**, staged (index) but uncommitted, and partially applied (see above — `graph::manifest::…` converted, `graph::dsl::…` and `geometry::…` in `🌍️world/🦀️component.rs` not yet converted at this snapshot in time). This is the same live edit noted in `📌️important.md`'s W4 tracking section for trinity's earlier batch-2b skip; it has not settled between then and now.
- No `cargo check` was run this pass (per the brief's blocked-mechanism protocol: stop before fighting a live rename, don't baseline against a moving target mid-conversion) — running it now would only measure DKM's in-progress state, not a stable baseline, and any errors observed could not be reliably attributed to trinity's own boundary vs. the rename's unfinished half.

## Files touched this pass

None. `✏️s/🔌️plugins/🔱️trinity/**` is untouched by this session — the working tree's pre-existing staged changes (DKM's, not mine) are exactly as found.

## Recommendation

Re-check `git status --porcelain -- ✏️s/🔌️plugins/🔱️trinity` and re-run the same "internally consistent within each file" spot-check (grep for leftover `math::graph::`/`math::geometry::` after confirming no bare `math::` remains) before dispatching trinity again. Once the rename is fully applied and either committed or at least internally consistent across every occurrence in every touched file, the composition migration (2 composed `graph` children per the design doc) can proceed using the provisional manifest-graph/instance-graph split noted above as a starting hypothesis, to be verified against trinity's actual code once unblocked.

ucas-status: blocked-mechanism
