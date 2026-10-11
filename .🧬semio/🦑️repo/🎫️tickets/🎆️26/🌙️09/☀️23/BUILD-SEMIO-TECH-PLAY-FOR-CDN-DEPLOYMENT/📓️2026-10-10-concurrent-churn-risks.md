# 📓️ Concurrent Churn Risks For The Cold Play Release Build — 2026-10-10

Read-only audit. No source edited, no build run, no git mutation. Scratch exports live in the session scratchpad, not in the ticket. Repo root `/Users/ueli/Documents/semio`.

## Verdict

The cold release build is exposed to a committed, half-finished rename in the shared `value` and `job` crates. Twenty-five non-test Rust files still use names that no longer exist (`JobPayloadCloseStep`, `SnapshotRetirementStep`, one-argument `retirement::owned_retirement`). Those files include the OS runtime (`💻️os`) and many plugin crates. The break is at HEAD, so the next `build-fresh` that compiles those crates will fail unless the owning ticket finishes the rename first.

## 1. Peer tickets modified since 2026-10-06 (open ones that touch the build path)

| Ticket (folder under `🌙️09/☀️..` or `🌙️10/☀️..`) | Goal | What it changes | Mid-refactor? |
|---|---|---|---|
| `UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O` (09/30) | runningframework | `value` retirement API (`SnapshotRetirementStep`→`RetirementStep`, `owned_retirement`→`admit_owned_retirement`), `job` (`JobPayloadCloseStep`→`InteractiveJobCloseStep`), new `value/🫴️receiving`, `io/🪶️sqlite-snapshot`, binary/native receiving | Yes. Likely the source of the renames in section 3. |
| `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT` (08/11) | runningframework/products/repo | Layering moves in `os/🏪️store`, `os/📡️spr`, `os/🔌️plugin`; deleted `pack-error-producer-proposal` drafts; references all three renamed symbols | Yes, open since 08/11. |
| `NON-DESTRUCTIVE-HISTORY-EDITING` (09/30) | runningframework | UI reactor/host treasury receipts (`ui-current`, 10/10), path-budget decision owner | Yes. |
| `DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES` (10/08) | runningframework | Removes `drawing_selection_inverse`/`drawing_selection_diff` from draw; log says "WRITTEN, NOT YET COMPILED", peer `stdio-pdf` E0061 and `stdio-png` missing file | Yes, compile errors reported by its own log. |
| `BIM-PLUGIN` (10/08) | architect | New plugin `🏙️bim` (about 14k churned lines), adds framework geometry primitives, uses `admit_owned_retirement` | Yes, foundation wave in progress. |
| `PROCEDURAL-FEATURE-COMPLETE` (06/08) | runningframework/os/plugins | OS SDK request admission, JSON write cursor and retained-clone scalar leaves in `os` runtime | Yes. |
| `PROCEDURAL-3D-END-TO-END` (09/09) | runningframework/os | wgpu `🧊️wgpu` draw pipelines for world3d primitives | Yes. |
| `WGPU-RENDERER-REACT-PARITY` (09/17) | runningframework/os | Public WGPU boot progress/cancellation in browser host, `ui/targets/wgpu` | Yes; browser-side. |
| `PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D` (09/17) | runningframework/os/plugins | puzzle 2d/5d crates, feature gate `component-app-assembly`, registries | Yes; large parallel waves. |
| `RASTER-PLUGIN-END-TO-END` (09/05) | runningframework/os/plugins | Shared pixel selection job, native `PixelSelectionJob`, layer transforms | Yes. |
| `COMPLETE-DRAW-VECTOR-EDITING-EXPERIENCE` (09/26) | plugins/draw | Draw export (PNG/SVG/PDF) through owned serialization, font run work | Yes. |
| `COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE` (09/26) | stdio | XLSX canonical save and grid editing, `InsertCell` mutation (facets across schema/proto/graphql) | Yes. |
| `FORMS-PLUGIN-END-TO-END` (09/16) | plugins/forms | Forms import/export, extension routing | Yes; localized, mostly plugin-local. |
| `ARTIFACTIO` (08/10) | runningframework/os/plugins | Pack JSON turn capacity; native `UI10` runtime failure in `pack/json` | Yes; reports a native Pack JSON runtime fault. |
| `ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS` (08/17) | runningframework | Open WASI warnings in `os/🖥️host` and `os/🪐space` (empty line after doc comment) | Yes; warnings may become errors under the build. |
| `END-TO-END-TAXONOMY-NORMALIZATION` (08/17) | runningframework/repo | Energy artifact move (195-file pre-apply proof, production move not committed) | Yes; path-moves pending. |
| `KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE` (09/01) | runningframework/repo | Basename and package-metadata ownership packet, evidence only | Probably the cause of many `🔣️.json` deletions; not verified. |
| `REPO-PATH-BUDGET` (10/01) and `WINDOWS-CHECKOUT-PATH-LENGTH-FIX` (10/06) | runningframework / zerotouch | Path-length renames of case directories; 96 copied trees removed | Mechanical renames; a peer rename can hide path breaks. |
| `PRINT-VISUALIZATION-LIBRARY` (09/05) | runningprint | Bun bootstrap ownership audit | Low risk for play. |
| `END-TO-END-OS-HUB-COLLABORATION-MCP` (09/23) | runningframework/os | wgpu shells, hub ports 8050-8059 | Low risk for the build; runtime ports only. |
| `DEMONSTRATOR-END-TO-END-ALL-APPS` (08/28) | r2603 | Nx graph failures on Windows (`createDependencies` sourceFile paths) | Old; the Nx graph issue may recur. |
| `TEACHING-ARCHITECTURE-DEPLOY-READY` (10/02), `MOBILE-SHORT-TITLES` (10/05), `LAYERED-OVERVIEW-GRID-AND-SWIPE-ON-MOBILE` (10/05, closed), `RESOLVE-MERGE-CONFLICTS` (10/06), `DASHBOARD-LAUNCH-COCKPIT` (closed) | various | Teaching app, mobile titles, merge scripts | Unrelated to play, not audited in depth. |

Closed in the window (no action): `DASHBOARD-TUI-WORKFORCE`, `UPDATING-ARCHITECTURE-QUIZ-WORDING`, `TICKET-OVERSIZED-ARTIFACT-PURGE`, `FIXTURES-ARE-TESTING-EXAMPLES-ONLY` (10/06), Zwischenbericht tickets (print goal).

## 2. Git churn since 2026-10-07

- 8 commits: 3 on 10/08, 3 on 10/09, 2 on 10/10 (latest `c44e964518f` at 09:05). Commit messages are auto-generated, so churn was aggregated by path.
- Name-status totals over those commits: A 26841, M 29011, D 3898, R 765 (line counts, with repeats across commits).
- Top churn by path (top 4 segments):
  - `✏️s/🔌️plugins/🏙️bim` 14213, `✏️s/🔌️plugins/🗄️stdio` 9231, `✏️s/🔌️plugins/📕️norm` 3905, `✏️s/🔌️plugins/🏛️architect` 2368, `✏️s/🔌️plugins/🧩️puzzle` 1643, `✏️s/🔌️plugins/🏗️fem` 1195, `✏️s/🔌️plugins/🌀️procedural` 1060.
  - Framework: `🛍️products/💻️os/🔨️modules/🔌️plugin` 1124, `🏪️store` 851, `🦑️repo/.../📚️library` 470, `💻️os/📺️renderer` 253, `💻️os/🌊️flow` 245, `💻️os/♾️infinite` 219, `🌱️value/♻️retirement` 157, `🌱️value/🧬️retained-clone` 106, `🎒️pack/🔤️json` 80, `📡️replication/🎮️mutation` 74, `🚪️io/🪶️sqlite-snapshot` 58, `🧊️3d/📐️brep` 35+.
- Deleted paths: 1853 unique in source (outside ticket folders). 1833 are gone; 20 were re-added later. Most deletions are relocations (for example `♾️infinite/🖼️canvas` moved to `🔨️modules/🖼️canvas`, which is intact and referenced correctly by the play Vite builder), plus plugin `🎮️commands` and schema twin moves.

## 3. Uncommitted work (`git status --short`, 10 entries)

- Modified: `✏️s/Cargo.lock`, `🌎️hub/Cargo.lock`, `🎓️teaching/Cargo.lock` (relock, expected); `⏯️tool-run/.../Cargo.toml` and `🛠️tool-machine/.../Cargo.toml` (duplicate-key removal, expected); `.🧬semio/.../💬️prompts/🐙️ueli.md`.
- Modified on the release path: `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts`. The `FreshBuildScript` now calls `runOwnedNxInvocationV1(this.repoRoot, PLAY_FRESH_BUILD_ARGS, this.invocation)` instead of `new NxScript(...).run(...)`.
  - Checked: `runOwnedNxInvocationV1` exists and is committed (`🚀️bootstrap/📜️script.ts:425`). `PLAY_FRESH_BUILD_ARGS` and `playFreshBuildEnvironment` exist in `🆕️fresh-build/🟦️.ts`. `this.invocation` is `protected` on `Script`, so it is accessible. The new path routes through `ScriptRouter.register("nx", NxScript)`, which matches the `["nx", ...args]` shape.
  - Not compiled or run. The coordinator should run `build-fresh` through this exact line before trusting it.
- Untracked: this ticket's own `📓️2026-10-10-redeploy-fleet-rules.md` and `🔁️run-fresh-2026-10-10.sh`. `🖱️ui/🧬️schema/` now holds only three empty subdirectories (`🎚️ring-press`, `📦️prepared-close`, `🧊️feature-ownership`), left after their `🔣️.json` files were deleted in commits. No compile impact found.
- Tool-machine Cargo.toml: the removed `semio-framework-value` path line is the same path as root `Cargo.toml:337` (workspace). The remaining `semio-framework-value = { workspace = true }` at line 25 resolves to that path, so this is safe.
- All 194 `📦️packages/🦀️rust` workspace member paths in root `Cargo.toml` exist.

## 4. Verified breakages at HEAD (committed, not uncommitted)

All three renames are in the latest commits (`48e47561a9e` 10/10 04:03 and `c44e964518f` 10/10 09:05 were the pickaxe hits).

### A. `JobPayloadCloseStep` renamed to `InteractiveJobCloseStep` (job crate)

- Defined now: `🧰️framework/🔨️modules/🧵️job/🦀️.rs:1252` (`pub enum InteractiveJobCloseStep`).
- Old name: no definition and no alias anywhere in the repo. Checked for `as JobPayloadCloseStep`.
- Still referenced in 25 non-test `.rs` files:
  - Framework: `💻️os` (4 files), `🕸️graph/⏯️layout-run` test module.
  - Plugins: `🀄️wfc` 10, `🔋️energy` 2, `🏗️fem` 3, `🎞️animate` (for example `🏠️host/🧰️owned/🦀️.rs:542-561` uses `semio_framework_job::JobPayloadCloseStep::Pending/Complete`), `🗄️stdio`, `🖨️raster`, `📏️layout`, `🧩️puzzle`.

### B. `SnapshotRetirementStep` removed from value crate (`🌱️value/♻️retirement/🦀️.rs`)

- Replaced by `RetirementStep` (commit `48e47561a9e` changed `close_step` returns to `RetirementStep`). No alias exists.
- Still referenced:
  - Framework `💻️os`: `🖥️host/🦀️.rs:551,555` (`store::SnapshotRetirementStep::Complete/Pending`), `🏪️store/🪆️child/🧵️owner/♻️retirement/🪆️child/🦀️.rs:2,4,25-27` (`use semio_framework_value::{SnapshotRetirementStep,…}`).
  - 77 non-test `.rs` files in plugins: `🖍️draw` 9, `🌀️procedural` 6, `🀄️wfc` 6, `🧩️puzzle` 5, `🗄️stdio` 5, `🖨️raster` 5, `🔱️trinity` 5, `🪐️space` 4, `🏭️process` 4, `✒️writer` 3, `🪵️sourcing` 2, and others.

### C. `owned_retirement(value)` removed from value crate

- The removed signature was `pub fn owned_retirement<T: RetireOwned>(value: T) -> Box<dyn ErasedSnapshotRetirement>`.
- Replacement is `admit_owned_retirement(value, grant)` (returns `Result`) plus `owned_retirement_birth_bytes`. Different arity and return type, so this is a signature break, not a rename.
- Live call sites (non-test): `✏️s/🔌️plugins/🌀️procedural/.../🌀️generation2d/.../✏️editor/🦀️.rs:1324` is one example, `self.media_retirement = Some(semio_framework_value::retirement::owned_retirement(value));`. About 20 more files across `🗄️stdio` (6), `🧩️puzzle`, `🖨️raster`, `🖍️draw`, `🔱️trinity`, `📐️cad`, `📏️layout`, `🏭️process`, `🎬️sequence`, `🌀️procedural`.

Attribution: the three renamed symbols appear in tickets `FIXTURES-ARE-TESTING-EXAMPLES-ONLY` (10/06 closed, many hits), `UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O` (09/30, open), `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT` (08/11, open), `PROCEDURAL-FEATURE-COMPLETE` (06/08, open), and `BIM-PLUGIN` (10/08, open). The likely source is the UNIVERSAL and CLEAN-ARCHITECTURE pair. This is inferred from the ticket text, not confirmed in git authorship.

## 5. Highest-risk paths for the cold build

1. `🧰️framework/🔨️modules/🌱️value/♻️retirement/` and `🌱️value/📦️packages/🦀️rust/🦀️.rs` (retirement API renames, A/B/C above). Blocks every consumer.
2. `🧰️framework/🔨️modules/🧵️job/🦀️.rs` (`InteractiveJobCloseStep` rename, A).
3. `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/` and `💻️os/🖥️host/🦀️.rs` (dangling `SnapshotRetirementStep`, B; 851 churned lines in store).
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` (1124 churned lines) and `💻️os/🎚️config/` (145): OS runtime that the play host loads.
5. `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/` and `🎒️pack/🔤️json/` (shared decode/encode churn; ARTIFACTIO native Pack JSON fault).
6. `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/` and `🖱️ui/⌨️tui/` (wgpu/TUI churn; the WGPU and PROCEDURAL-3D tickets).
7. `✏️s/🔌️plugins/🏙️bim/` (14k lines, foundation wave still in flight; new plugin may fail the play build if it is included).
8. `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts` (uncommitted release-path change, section 3).

## 6. Limits

- Plugin crates may or may not be in the play composition. Play's own source does not name plugin folders; the composition lives under `🌎️hub/🧩️compositions/`. The breakages above are real in the tree, but whether the play release compiles each affected crate was not traced.
- `#[cfg(test)]` modules inside `.rs` files were not separated from production code. Only `/🧪️tests/` paths were excluded. Some counts may include inline test modules.
- No compile was run, per instructions. Every breakage above is a source-level finding, not a build result.
- The rule "tickets modified since 2026-10-06" was taken from directory mtimes. Some folders (odd month names `🌩️10`, `🌄️10`, `🌅08`) are stray copies of other tickets. Only `🌙️..` and `🌄️/🌩️` tickets were checked.

## 7. Own-session incident (disclosed)

My first Bash command accidentally wrote `git log` output (about 10 MB) into `BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/.keep`. I removed that file immediately. The `🗑️generated` folder is now empty again. No other file in the ticket was touched.

## Summary of files read

- Ticket notes: 27 folders under `🎫️tickets/🎆️26/` (newest markdown heads and `🎫️ticket.json`).
- Git: `log --since=2026-10-07` (name-only, name-status, pickaxe `-S`), `status --short`, `diff` of the 7 modified paths.
- Source spot checks: `🧵️job/🦀️.rs`, `🌱️value/♻️retirement/🦀️.rs`, `🌱️value/📦️packages/🦀️rust/🦀️.rs`, `💻️os/🖥️host/🦀️.rs`, `💻️os/🔨️modules/🏪️store/🪆️child/...`, `🌀️procedural/.../🌀️generation2d/.../✏️editor/🦀️.rs`, `🎞️animate/.../🏠️host/🧰️owned/🦀️.rs`, `🚀️bootstrap/📜️script.ts`, `🏢️semio-tech/.../📦️site/📜️script.ts`, `🆕️fresh-build/🟦️.ts`, root `Cargo.toml`.
