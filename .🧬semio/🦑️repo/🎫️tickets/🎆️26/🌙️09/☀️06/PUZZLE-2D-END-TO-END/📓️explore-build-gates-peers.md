# Explore: build gates + live peers (puzzle 2d)

Read-only exploration. No files edited, no builds run, no git writes, no ticket state changed.

## 1. Build gates

**Crate manifest** `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`
- `name = "semio-s-plugin-puzzle"`, `[package.metadata.component] package = "semio:puzzle"` (line 13).
- One crate serves puzzle2d/3d/5d (`description` line 8). `[lib] crate-type = ["cdylib","rlib"]`, `path = "🦀️.rs"`.
- `default = ["plugin-entry"]` feature gates the `#[no_mangle] semio_plugin_install_bundle` wasm export — OFF only when the 🎪️demonstrator embeds this crate (comment above `[features]`).
- `[target.'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'.dependencies]` isolates `wasm-bindgen`/`js-sys`/`web-sys` (the `BoardSession` browser bridge) away from the wasm32-wasip2 component build — component target must stay free of third-party runtime deps.
- `[package.metadata.semio.playground]` declares three variants: `puzzle2d` (app `s.puzzle.puzzle2d@1/*#editor`, ports react 6012/wgpu 6112, `engines = ["./✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust"]`), `puzzle3d`, `puzzle5d`.
- `[dependencies]` puzzle depends directly on `semio-s-plugin-stdio` (path dep, `features = ["full-artifact-catalog"]`), `semio-framework-plugin` (`features = ["component-guest"]`), plus geometry/graph/os-kernel/schema/ui-contract/ui-scene/dispatch-macros/3d/value-derive.

**Plugin identity** — `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs:55` — `Plugin::<PuzzleApps>::builder("puzzle")` matches `Cargo.toml`'s `package = "semio:puzzle"` (the host prefixes `semio:`). No drift found (this is the check the memory `project-plugin-id-drift-builder-vs-component` warns about).

**build.rs** (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs`) — does NOT read the 2d examples/manifests. It only walks `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons`, indexes every `.svg` by its (emoji-stripped) stem into a `BTreeMap`, copies each into `OUT_DIR/🌱️metabolism/...`, and emits `OUT_DIR/🧩️metabolism.rs` with a `board_metabolism_icon_svg(key) -> Option<&'static str>` match arm table (`include_str!` per icon). `println!("cargo:rerun-if-changed=…icons")` is the only rerun trigger — the 2d example trees (`concrete-forest`, `nakagin-capsule-tower`) are NOT inputs to this build script; they're loaded at runtime, not codegen'd.

**`📜️script.ts` router** (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts`)
- `wasm` → `runWasmPackWebBuild` (the `@semio-tech/puzzle-wasm` browser `BoardSession` bridge, `noDefaultFeatures: true`, `PUZZLE_BOARD_SKIP_WASM_BUILD` escape hatch).
- `test` (default command) → `runCargoTestBudgeted(["semio-s-plugin-puzzle"], this.repoRoot)` — one native cargo-test invocation for the whole crate (2d+3d+5d together), no per-artifact test filter.
- `describe` → `describePluginComponent(this.repoRoot, "semio-s-plugin-puzzle", join(this.root, "..", ".."))`, imported from `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:483`. Its docstring: builds the wasm32-wasip2 component and re-emits `🛂️.descriptor.semio` + `🔣️.json` "at this plugin's own owner root" — i.e. `✏️s/🔌️plugins/🧩️puzzle/🛂️.descriptor.semio` + `🔣️.json` (both present, sizes 4.3MB / 5.4MB, mtimes 2026-09-01 10:54 / 2026-09-04 11:17 — the `.json` is 3 days newer than the descriptor, meaning `describe` has NOT been re-run since at least 2026-09-04, and everything committed since then — including the 2026-09-05 03:53/19:04/22:02 puzzle-crate edits — postdates it).
- `fixtures lint` — a repo-wide (not puzzle-specific) coverage gate: discovers every `🧬️mutations` tree, cross-checks enum variants vs mutation leaves vs `🧪️tests` case file sets (`CORE_CASE_FILES`, `DERIVED_CASE_FILES`, snapshot before/after either inline-encoded or a `🔗️component.ref.json` reference). Exits non-zero on any "error" finding; missing derived encodings are only a warning unless `--full`.

**`📋️project.json`** (same dir) registers nx targets `wasm`, `test`, `test-quick`, `test-long`, `test-exhaustive`, `fixtures-lint`, `describe`, all `cwd`-scoped to the rust package dir, all `bun ./📜️script.ts <cmd>` with `forwardAllArgs: true` (except `wasm`). `project.json` contains no logic itself, consistent with CLAUDE.md's script.ts-only rule.

**Root `📜️script.ts`**
- `class TestScript` at line 19056, registered `.register("test", TestScript)` at line 22635 (top-level `test` command, distinct from the per-package one above).
- `DevScript` registered at line 22613 (`dev` command; `dev 2d`/`dev puzzle2d` resolves through the generated playground catalog, not a hand-list — see below).
- `runGate()` (the `verify` command's dependency/freshness gate) at ~line 11011-11022: runs `dependency-cruiser`, then **`bun nx run @semio-tech/plugin-registry:check`** (line 11022) as "generated catalog freshness", then renderer/plugin lint, then `ui-styling-tokens:check-no-px`, `framework-rs:check`, `ui-rs:check`.
- Playground catalog regeneration: line 168 `runCmdStatus("bun", ["nx","run","@semio-tech/plugin-registry:generate"], …)` inside `dev`'s startup path — if the catalog comes back empty it errors "playground catalog is empty after registry generate — check @semio-tech/plugin-registry" (line 171). Line 375 calls the same generate target elsewhere; line 512 tells the developer to run `plugin-registry:generate` if an unknown playground alias is requested.

**wasm32-wasip2 component build mechanics** — NOT in puzzle's own files; owned by os-dev:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:85-86`: `PLUGIN_WASM_TARGET = "wasm32-wasip2"`, `PLUGIN_WASM_STACK_BYTES = 8 * 1024 * 1024`.
- `pluginCargoArgs()` (line 108-112): `["rustc","-p",packageName,"--target","wasm32-wasip2","--profile",profile,"--","-C","link-arg=-zstack-size=8388608"]`, `+= ["-C","strip=none"]` when `SEMIO_PLUGIN_SYMBOLS=1`.
- `pluginWasmProfile()`/`selectComponentWasmProfile` (root lib `🟦️.ts:2968-2971`) picks `wasm-dev` unless `mode === "ship"` or `SEMIO_PLUGIN_PROFILE` overrides (puzzle's own `wasm` script forces `wasm-release` for the browser bridge; the wasip2 component build defaults to `wasm-dev`).
- Root `Cargo.toml` `[profile.wasm-dev]` (line ~258): `inherits = "dev"`, `codegen-units = 1` only — no `debug = false` override baked in. `CARGO_PROFILE_WASM_DEV_DEBUG=false` is an **environment-variable override a developer sets themselves**, not code in the repo; the sibling ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️findings-2026-09-05.md:433` records it cuts plugin-build RSS "8.6 GB → ~165 MB" and is needed on this host (low free swap).
- `[profile.wasm-release]` (ship profile for the component): `opt-level = "s"`, `lto = "thin"`, `codegen-units = 1`, `strip = "symbols"`, `incremental = false`, `trim-paths = "object"` — all commented with rationale in root `Cargo.toml`.

**Descriptor files** — `✏️s/🔌️plugins/🧩️puzzle/🛂️.descriptor.semio` (4,295,038 B, mtime 2026-09-01 10:54) and `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` (5,385,008 B, mtime 2026-09-04 11:17). `describe` regenerates both at this "owner root" (one level above `📦️packages`). Given the 09-04/09-01 mtimes and the 09-05 crate-source commits above, **the descriptor pair is stale relative to HEAD** — `describe` must be re-run after the crate builds (matches the ticket's own definition-of-done item 4).

**Registry check** — `bun nx run @semio-tech/plugin-registry:check`, implemented in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (comment at line 197 references a real historical bug where a mismatched path made `plugin-registry:check` report a plugin "as having no—" catalog entry). This is the single-source plugin/playground/framework catalog codegen check invoked both by root `verify` and (per every other plugin's `📜️script.ts` docstring, e.g. line 15 in trinity/remodel/raster/flow/process/norm/cad/demonstrator/block/dag) as the thing whose descriptor-gate warning tells a developer to run `describe`.
- **Taxonomy** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` line 2862: `"distribution-puzzle": { "emoji": "🧩️", "slugPattern": "^puzzle$", "allowEmojiOnly": false }`. Line 17927: `"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any": "🔮️oracle"` — the taxonomy already declares this exact subset path as owning an oracle-role folder (consistent with `🔮️oracle/🔣️.json` existing under that path per the 2026-09-05 19:04 commit diff, item 2 below). I did not find a literal `memberNames` array enumerating puzzle's own component/example set the way line 5357 does for a different (`writer-languages`-adjacent) tree — puzzle's own catalog membership is generated (`plugin-registry:generate`), not hand-listed in taxonomy.json.

## 2. Test discovery

**Repo-test protocol** — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`: schema v2, unit of coverage is `artifact × standard × smallest owning subset × runtime mutation × expected outcome class × fixture × subject implementation × platform`, requiring a qualifying third-party oracle per mutation (`QualifyingOracleKind`: third-party-library/cli/standards-reference-tool; `SupplementalOracleKind` such as `cross-semio-implementation` explicitly does NOT discharge the requirement).

**Cached result for puzzle 2d** — `.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-◻️mutate-puzzle-2d-1-oracle-python/🔣️.json`:
```json
{ "kind": "semio-test-output", "testId": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any::◻️mutate-puzzle-2d-1", "cacheKey": "a316f9aedc9259eb88f9fdab95e00c70" }
```
Directory mtime 2026-09-05 06:30; its `📦️artifacts` subdir is **empty** and this identity file carries no pass/fail/status field — the cache entry only proves a run was *attempted* for this testId/cacheKey, not that it passed. I could not find pass/fail state anywhere else for this exact testId:
- `.🧬semio/🦑️repo/⚡️cache/tests/reports/latest/` (mtime 2026-09-05 06:42, i.e. 12 min *after* the puzzle cache entry) has `🏁️done`, `📊️summary.json` (`{"level":"exhaustive","cases":3,"scenarios":44,"executed":44,"passed":44,"failed":0,"errored":0,"byImplementation":{"python":{"passed":44,"failed":0,"errored":0}}}`), `📈️metrics.json`, `📤️results.jsonl` (44 lines) — but **zero occurrences of "puzzle"** in `results.jsonl`; the only cases it covers are `gis`/`gismap`/`gisterrain` (confirmed by grep and by reading the first `results.jsonl` line, testId `…🌍️gis/🗿️artifacts/🗺️gismap…`). So this is a scoped, unrelated report run that happens to have finished 12 minutes after the puzzle cache entry — **it does not confirm or deny the puzzle 2d oracle-python result**.
- I did not locate any other aggregate report file covering puzzle. **Conclusion: whether the puzzle-2d oracle-python fixture last passed or failed cannot be confirmed from cache alone; only that a run was attempted 2026-09-05 06:30.** (Per CLAUDE.md, this is reported as unconfirmed rather than assumed.)

**Command that runs it**: `runCargoTestBudgeted` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1619`) is what puzzle's own `test` subcommand calls; the cache's `test-s-plugins-puzzle-…-oracle-python` naming and the schema's Python-oracle-role wiring indicate the *oracle* leg is driven by a separate Python harness under the repo-test coordinator (root `class TestScript` at `📜️script.ts:19056`), not the Rust `cargo test` leg — I did not find time to trace that Python invocation's exact CLI inside this budget; flagging as a gap for a follow-up read of `📜️script.ts:19056+` if precise command text is needed.

**Taxonomy fileKinds / drift risk**: `fileKinds` block starts at taxonomy.json:1460 (alphabetical emoji-keyed file-kind registry: `abnf`, `absence-marker`, `antlr`, `archive`, `assembly-source`, `audio`, `badge-definition`, `bcf-data`, `binary`, …). I did not find a `◻️2d`-specific fileKinds entry (the `◻️2d` string appears as a *path segment* — artifact-kind slug — at taxonomy.json lines 5640/6832/7953/11034/17491/17765/17915-17920/17927, not as a `fileKinds` key), so the memory-flagged "fileKinds/taxonomy drift returns 0 silently" risk class is about path-shape membership, not a missing file-extension entry — no drift observed in the puzzle-2d path entries checked.

## 3. History (last 5 days)

`git log --stat -15 -- "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d"`:
- `3a6a9d6bfc` (2026-09-05 22:02, "🚩️591") — pure renames, 24 files changed, **0 insertions/0 deletions**: several `🧪️tests` case dirs renamed to hash-suffixed names (e.g. `🚫️removes-handle-kind-pair` → `🚫️removes-handle-5fccd0`, `📇️installs-handle-kind-catalog` → `📇️installs-handle-a0eaf8`, `🤝️adds-handle-kind-pair` → `🤝️adds-handle-kind-d59cb5`) — this is the "hash-truncated test dirs" pattern also called out by the sibling FEM ticket's drift report ("class k").
- `b0dfa0f09b` (2026-09-05 19:04, "🚩️590") — large repo-wide normalization commit; touches puzzle 2d files: editor `🦀️.rs` (47 lines removed), `🖼️assets` renames (`🧪️forest`→`🌲️forest`, `🧪️tower`→`🏢️tower`), `📚️examples` `🟦️.ts`/`🦀️.rs` small edits, and **`✳️any/🔮️oracle/🔣️.json` rewritten (52 lines changed)** — the oracle registration file for this exact subset.
- `fe7c8a8f8b` (2026-09-05 03:53, "🚩️589") not directly touching `◻️2d` in this filtered log (its diff was against `📦️packages`, see below), but is same-day.

`git log --stat -8 -- "✏️s/🔌️plugins/🧩️puzzle/📦️packages"`:
- `b0dfa0f09b` (19:04) — `📜️script.ts` (TS test file, +/-), `Cargo.toml` (+2/-1), **`🦀️.rs` 122 lines changed** (crate root).
- `fe7c8a8f8b` (03:53) — `📋️project.json` (+8), `📜️script.ts` (TS, +15/-... ), `Cargo.toml` (+8/-... wait see below), `build.rs` (+/-51 lines), rust `📜️script.ts` (+2/-1), **`🦀️.rs` 578 lines changed** — this is the big crate-root rewrite day.
- `03100691d5` (2026-09-03 18:13, "🚩️587") — repo-wide BRep/hub/plugin catalog commit, same day window as the ticket's stated "Brep Kernel Dependency Free Runtime" theme.

Net: the puzzle crate root (`🦀️.rs`) and `build.rs` were rewritten substantially twice in the last 3 days (578 then 122 line diffs), and the 2d artifact's test-case directories were renamed (hash-suffix scheme) as recently as 3 hours before this exploration — consistent with an active, still-settling rename/normalization wave, not a stable baseline.

**Uncommitted changes**:
- `git status --short -- "✏️s/🔌️plugins/🧩️puzzle"` → **clean, no output** (puzzle tree itself has zero uncommitted changes right now).
- `git status --short -- "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"` → 3 modified + 1 untracked, **all under `🖨️describe`/`🌐️browser-bundle`**, not puzzle-specific:
  - `M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts`
  - `M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts`
  - `M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`
  - `?? 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️fixtures/` (new dir)
  This is exactly the `describePluginComponent` script puzzle's `describe` command calls — a peer is mid-edit on the shared describe machinery right now. The FEM ticket's own drift report independently flagged "peer edits in plugin's browser-bundle subtree" as the one non-clean external gate, corroborating this is a live, shared, in-progress edit rather than puzzle-caused.

## 4. Live peers

**Open tickets under `🎆️26/🌙️09/☀️05` and `☀️06`** (status field read from each `🎫️ticket.json`):
- `☀️06`: `FEM-PLUGIN-END-TO-END` (open), `ENERGY-PLUGIN-END-TO-END` (open), `PUZZLE-2D-END-TO-END` (this ticket, open), `REMODEL-PLUGIN-END-TO-END` (open).
- `☀️05`: `WINDOWS-CHECKOUT-ILLEGAL-FILENAMES` (closed), `RASTER-PLUGIN-END-TO-END` (open), `S-END-TO-END` (open, no `📓️status.md` found), `MERGE-PRINT-AND-MIT-BESTAND-INTO-CURRENT-BRANCH` (closed), `DRAW-PLUGIN-END-TO-END` (open), `BLOCK-PLUGIN-END-TO-END` (open), `CONFINE-DWG-CODECS-TO-ARTIFACT-I-O` (closed).

**Status-file mtimes (now = 2026-09-06 04:46-04:47 CEST, so "last 3 hours" = since ~01:46)**:
- `ENERGY-PLUGIN-END-TO-END/📓️status.md` — 04:46:18 (updated seconds ago, actively live).
- `REMODEL-PLUGIN-END-TO-END/📓️status.md` — 04:39:41 (live).
- `RASTER-PLUGIN-END-TO-END/📓️status.md` — 04:37:57 (live).
- `FEM-PLUGIN-END-TO-END/📓️status.md` — 01:13:10 (stale relative to the 3h window's tail, but within it).
- `DRAW-PLUGIN-END-TO-END/📓️status.md` — 2026-09-06 00:56:13 (>3h old, not live).
- `BLOCK-PLUGIN-END-TO-END/📓️status.md` — 2026-09-05 19:31:53 (long stale).

REMODEL's own log (line 22) independently states: "Host at 99% swap, load 206, 57 rustc — builds still forbidden" as of its 00:56 entry, and its line 20 records launching two Opus workers "gated: no cargo unless swap < 45 GB and load < 60" — i.e. REMODEL is deliberately holding off on cargo right now, consistent with heavy host contention.

**Shared-file risk relative to puzzle**: none of FEM/ENERGY/REMODEL/RASTER's status logs mention touching puzzle, stdio, or the plugin-host crate directly by path (their "stdio"/"plugin" mentions are their own dependency-closure notes, e.g. REMODEL's dev-boot closure `{remodel, stdio}` and FEM's closure `{fem, stdio}` — same shared `semio-s-plugin-stdio` crate puzzle also depends on, so a stdio rebuild any of them trigger is shared cost/benefit, not a file conflict). The one confirmed **file-level** shared edit is the `🔌️plugin/🖨️describe` + `🌐️browser-bundle` uncommitted changes noted in §3 — some peer (ticket unclear, not attributable from status.md logs alone) is actively modifying the exact `describe` script puzzle's own `describe` command depends on.

**Live processes** (`ps aux | grep -E "cargo|rustc|bun"`, `uptime` at capture time: `load averages: 76.33 86.37 56.53`, up 1:58):
- This ticket's own `cargo check -p semio-s-plugin-puzzle --lib --tests --message-format=short` (PID 47883, started 4:36AM) is running now and holds `target-p2d-e2e/debug/.cargo-lock` (confirmed via `lsof`).
- It is currently compiling `semio_s_plugin_stdio` for **native** (PID 49779, 74% CPU, 1:38 CPU-time accrued, `-Z threads=8`, target dir `/Users/ueli/Documents/semio/target-p2d-e2e/debug/deps`) — i.e. the check is progressing, not stalled.
- A second, unrelated `rustc --crate-name semio_s_plugin_stdio` process (PID 47589, 94.7% CPU) is building for **wasm32-wasip2** under `/Users/ueli/semio-demonstrator-target/wasm32-wasip2/wasm-dev/deps` — a different (demonstrator) target dir, some other session's build, not ours.
- 21 total rustc processes, 14 cargo processes at capture time; other crate names seen: `wit_component`, `wit_parser`, `wasmtime_internal_component_macro`, `semio_framework_ui_runtime`/`ui`, `db`, `tracing_attributes`, `prettyplease`, `async_trait`, `ipnet`.

**Target directories found** (all under repo root, not `/Users/ueli/`):
| dir | size | notable rmeta |
|---|---|---|
| `target-p2d-e2e` (ours) | 8.8G | `libsemio_s_plugin_puzzle-518d381a564a2d45.rmeta` @ 2026-09-05 15:57:41; `libsemio_s_plugin_stdio-1900961824929183.rmeta` @ 2026-09-05 14:46:08 — matches this ticket's own status.md note that it was seeded from `target-p3d-e2e`'s Sep-5 15:58 check-mode puzzle rmeta. |
| `target-lowpoly-e2e` | 11G | (not inspected further) |
| `target-lowpoly-boot` | 2.3G | |
| `target-s-e2e` | 3.5G | |
| `target-energy-e2e` | 1.6G | |
| `target-raster` | 874M | |
| `/Users/ueli/semio-demonstrator-target` | (shared demonstrator target, actively building stdio for wasm right now, see above) | |
| `.../scratchpad/target-remodel` (private, another session's scratchpad) | | |

No `target-p3d-e2e` directory itself was found still present at this path (likely already cleaned up or under a different session's scratchpad) — the ticket's own status.md description of it as the seed source is taken as accurate provenance, not independently re-verified here.

## 5. Known breakage classes — checked against puzzle 2d, all clean

- **`PluginCloseStep::AwaitingInput` non-exhaustive matches**: zero hits of `PluginCloseStep` anywhere under `✏️s/🔌️plugins/🧩️puzzle` (broad and 2d-scoped grep both empty). The pattern exists and is exhaustively matched in the framework itself (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` lines 6818, 11737, 13767, 14287, 16368, 16385, 16398, 16418, 16424; `🧵️retained-command/🦀️.rs:609`) but puzzle does not touch this enum directly — not a currently-live risk for puzzle 2d specifically.
- **`✳️base`→`🧱️base` rename in stdio**: 482 raw hits of the substring "base" in stdio (not all are the rename target — not decomposed further given time budget). An initial `grep -rln "✳️base"` against puzzle's 3d/5d editor/wasm files and `🎮️commands/🧵️retained/🦀️.rs` returned 4 filenames, but a follow-up line-level grep of those same 4 files for `"base"` found only unrelated identifiers (`base_revision`, `baseline`, `base_sum`, `base_weight`, `base_version`, struct fields named `base`, etc.) — **no actual `✳️base` emoji-path references exist in puzzle**; the initial file-level match was a false positive (grep matched on encoding/composition, not content — reconfirmed with a per-file, per-line pass). Nothing in `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d` matched even at the file level.
- **`#[value(...)]` without derive**: 79 occurrences of `#[value(` in `◻️2d/🦀️.rs` and 1 in the schema root `🦀️.rs`; spot-checked `✳️any/✏️editor/🎚️config/🦀️.rs` which has a genuinely **hand-written** `impl dsl::ToValue for Puzzle2dFillText` / `impl dsl::FromValue for Puzzle2dFillText` pair (explicitly documented as intentional: "matching the pre-migration serde Serialize/Deserialize pair's wire shape"), which is a deliberate hand-rolled impl, not a missing-derive bug. Not exhaustively checked across all 20+ mutation leaves in the time budget — flagged as spot-checked-clean, not fully audited.
- **`to_dsl_value` recursion (the store::to_dsl_value self-call trap)**: 2 files reference `to_dsl_value` in `◻️2d`: `🧬️mutations/🦀️.rs:407` is a comment describing the *pattern* (`dsl::to_dsl_value(&self.0)`/`dsl::from_dsl_value(...)`), not a live call; `✳️any/✏️editor/🎚️config/🦀️.rs:309` calls `dsl::to_dsl_value(self)` inside `impl store::ArtifactPack for Puzzle2dConfig::encode_pack_with` — this calls the free function from a *different* trait's method (`ArtifactPack`, not `ToValue`), so it is not the self-recursive `ToValue`-calling-itself trap the memory warns about. Verified clean.
- **Emoji filename collisions**: not separately re-audited here (out of scope given time budget vs. the git-history rename wave already documented in §3, which is the more concrete, currently-relevant filename-churn risk for puzzle 2d test case dirs).

## 6. launch.json — puzzle-related entries (`.vscode/launch.json`)

Note: `launch.json` mtime was 2026-09-06 04:38 (actively being written by some session) and briefly failed strict JSON parse when sampled — all entries below were read via targeted `grep`/`sed`, not a full parse, so treat line numbers as approximate to the moment of reading.

Puzzle entries found (all `"group": "3_dev"`, orders ~220-232 for dev, higher for build):
- `🛠️dev🧩️puzzle🩻️2d⚛️react` (order 220) — `bun run dev:puzzle:2d`, env `PUZZLE_2D_PLAY_PORT=6012`, `SEMIO_RENDERER=react`.
- `🛠️dev🧩️puzzle🩻️2d🧊️wgpu🌐️wasm` (order 220.1) — same command, `PUZZLE_2D_PLAY_PORT=6112`, `SEMIO_RENDERER=wgpu`.
- `🛠️dev🧩️puzzle🩻️2d🧊️wgpu🖥️native` (order 220.2) — `bun ./🧰️framework/.../🧊️wgpu/📦️packages/🦀️rust/📜️script.ts native puzzle2d`.
- `🛠️dev🧩️puzzle🏙️3d⚛️react` / `🧊️wgpu🌐️wasm` / `🧊️wgpu🖥️native`, and the same triad again per-example (`🎛️concrete🌲️forest`) for 3d and 5d, plus 5d's second example (`🎛️capsule🌙️dream`).
- `🛠️dev📖️storybook🧩️puzzle🩻️2d`, `…🧊️3d`, `…🕐️5d` — storybook variants.
- `📦️build🧩️puzzle🌉️board` — the wasm-pack `@semio-tech/puzzle-wasm` browser bridge build.
- `📦️audit🧩️puzzle🛂️publication-authority` — the publication-authority audit (matches ticket DoD item 3's `publication-authority-audit Puzzle2dPlayApp`).
- `📦️build🧩️puzzle🏙️3d🎛️concrete`, `📦️build🧩️puzzle👯️5d🎛️concrete`, `📦️build🧩️puzzle👯️5d🎛️capsule` — per-example asset builds.

No puzzle-specific `🧪️test🧩️puzzle…`, `🖨️describe🧩️puzzle…`, or `📇️registry:check` launch entries exist — `test`/`test-quick`/`test-long`/`test-exhaustive`/`describe`/`fixtures-lint` are invoked via the nx targets in `📋️project.json` (§1) or the root `verify` gate, not individually registered in `launch.json`; other plugins in the `🧪️test<plugin>📚️examples` naming family that DO have launch entries (e.g. `🧪️test🖍️draw📚️examples`, `🧪️test🧱️block📚️examples`) suggest that IS the existing convention for a per-plugin example-test launch entry, and puzzle has no equivalent one yet — worth following that exact naming (`🧪️test🧩️puzzle📚️examples`) if a launch entry is added.

## Summary (10 lines)

1. Crate `semio-s-plugin-puzzle` (`semio:puzzle`) builds 2d+3d+5d together; `Plugin::builder("puzzle")` matches `Cargo.toml` package id — no drift.
2. `build.rs` only generates a metabolism-icon lookup table; it does not read the 2d example/manifest trees.
3. `describe` regenerates `✏️s/🔌️plugins/🧩️puzzle/🛂️.descriptor.semio`+`🔣️.json`; both are stale vs. HEAD (mtimes 09-01/09-04 vs. crate-root rewrites on 09-05) — must re-run after the crate builds.
4. wasm32-wasip2 build uses `wasm-dev` profile + `-C link-arg=-zstack-size=8388608`, driven by os-dev's `pluginCargoArgs`, not by anything in puzzle's own files; `CARGO_PROFILE_WASM_DEV_DEBUG=false` is a developer-set env var (not repo code) needed on this low-swap host.
5. Cached puzzle-2d oracle-python test result (2026-09-05 06:30) carries no pass/fail data of its own, and the one aggregate report generated 12 min later covers gis/gismap only — **puzzle 2d's actual test pass/fail status is unconfirmed from cache**.
6. Puzzle 2d's `🦀️.rs` crate root was rewritten twice in 3 days (578 then 122 line diffs, 09-05 03:53 and 19:04) and its `🧪️tests` case dirs were hash-renamed as recently as 09-05 22:02 — an unsettled area, not a stable baseline.
7. `git status` on the puzzle tree itself is clean; the one live uncommitted peer edit touching a shared dependency is in `🔌️plugin/🖨️describe` + `🌐️browser-bundle` (the exact script puzzle's `describe` command calls).
8. Three sibling tickets (ENERGY, REMODEL, RASTER) are actively live right now (status.md updated within the last 10 minutes); host load is 76-86 with ~1:58 uptime; REMODEL explicitly gates its own cargo use on load<60/swap<45GB.
9. This ticket's own `cargo check -p semio-s-plugin-puzzle` is running now (PID 47883, holds `target-p2d-e2e`'s cargo lock) and is actively compiling native `semio_s_plugin_stdio` — progressing, not stalled.
10. Checked breakage classes (`PluginCloseStep`, `✳️base` rename, `#[value]`-without-derive, `to_dsl_value` self-recursion) are all clean in puzzle 2d on spot-check; no dedicated `test`/`describe`/`registry` launch.json entries exist yet for puzzle (only `dev`/`storybook`/`build`/`publication-authority`).
