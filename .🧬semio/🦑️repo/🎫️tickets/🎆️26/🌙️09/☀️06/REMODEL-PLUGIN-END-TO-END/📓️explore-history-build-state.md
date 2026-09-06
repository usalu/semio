# 📓️ Remodel — history and current build-gate state

Read-only exploration, no cargo/bun run, no git writes. All paths relative to repo root.

## 1. Git history of `✏️s/🔌️plugins/📸️remodel` since 2026-08-25

15 commits touch this path (`git log --date=iso --stat`). Commit messages are the known fake
per-session templates — only dates and diffstats are trustworthy signal.

| date | commit | files | +/- | what the diffstat shows |
|---|---|---|---|---|
| 2026-08-26 13:50 | a03e2597 | 2 | +61/-4 | `🧪️oracle/🔣️component.json`, `mutate-remodel-1/component.feature` |
| 2026-08-26 23:17 | d03b1fdb | 1 | +378 | new `🧪️oracle/🔣️component.json` |
| 2026-08-27 11:04 | a8d1caf4 | 3 | +364/-10 | `✏️editor/🦀️component.rs` +228 lines, 2 new fixture/schema JSONs |
| 2026-08-27 17:14 | d394744 | 5 | +5/-5 | oracle file rename `🔣️component.json`→`🔣️.json` (taxonomy normalization), tiny content edits |
| 2026-08-28 09:20 | f7b265d5 | 24 | +714/-5 | 17 new `payload.schema.json` files under `🧬️mutations/*` (mutation-catalog buildout), `mutate-remodel-1` renamed to `.feature`/`.rs` taxonomy names |
| 2026-09-01 13:51 | 67fb4216 | 300 | +2589/-1080 | introduces `📦️packages/🦀️rust/📦️glue.rs` (+315/-… wiring rewrite); largest genuinely-new-code commit in the window |
| 2026-09-01 22:47 | f15dbf0d | 40 | +88/-88 | small mechanical edits, no core files |
| 2026-09-02 12:19 | 21fbcd35 | 517 | +613/-613 | **this is the commit that first tracks the plugin-root `🔣️.json` (557 KB) and `🛂️.descriptor.semio`** (see §2) |
| 2026-09-02 13:31 | e5465a2 | 98 | +283/-116 | `Cargo.toml` +2 (adds a dependency), editor `🎚️config/🦀️.rs` +18 |
| 2026-09-02 15:18 | a807c07 | 1 | +3/-3 | trivial |
| 2026-09-02 17:38 | 96aa4f8c | 763 | +16648/-15952 | **the `📸️remodel`→`📸️remodeling` artifact rename** (directory move `🗿️artifacts/📸️remodel`→`🗿️artifacts/📸️remodeling`, 660 files, main `🦀️.rs` +558 lines) — see §3 for the full ticket account of this commit |
| 2026-09-03 12:49 | 7ad363fd | 11 | +1373/-163 | E1 shard: Python second-implementation + `.feature` fixture additions for all 35 mutation kinds, **zero Rust files edited** (see §3) |
| 2026-09-05 03:53 | fe7c8a8f8b | 357 | +412/-411 | repo-wide rename churn (this is the commit where `graph`/`number`/`geometry`/`pixels`/`pack`/`schema`/`plugin`/`dispatch-macros`/`value-derive` all last changed — see §4) |
| 2026-09-05 19:04 | b0dfa0f09b | 174 | +619/-602 | main `🦀️.rs` +52/-, `Cargo.toml` +2/-1, deletes a 409-line stale test file, TS `script.ts` +9 |
| 2026-09-05 22:02 | 3a6a9d6bfc (HEAD, ticket start) | 192 | 0/0 | pure fixture-directory renames (hash-suffix slugs), no content change |

Full per-commit diffstat saved at
`/private/tmp/claude-501/-Users-ueli-Documents-semio/0b03f376-5325-4732-a1fa-4765f6959f2a/scratchpad/remodel-log-full.txt`
(scratchpad, not the ticket folder — delete-safe).

## 2. Uncommitted state right now

`git status --porcelain -uall -- '✏️s/🔌️plugins/📸️remodel'` → **empty**. Nothing is modified or
untracked under remodel as of HEAD `3a6a9d6bfc` (this ticket's start commit).

`✏️s/🔌️plugins/📸️remodel/🔣️.json` and `🛂️.descriptor.semio` ARE tracked (`git ls-files` lists both) and
match HEAD exactly (`git diff HEAD` on them is empty). Their last commit is **21fbcd35 (2026-09-02
12:19)** — before the `96aa4f8c` remodel→remodeling rename and before every later rust-source commit
(`fe7c8a8f8b`, `b0dfa0f09b`). Per
`.../🌙️09/☀️02/UNIFIED-ARTIFACT-NAMING-AND-DEDUPLICATION/📓️rename-remodeling-and-emoji-ids.md`, the
rename commit deliberately left these two generated files untouched ("will regenerate on next
build") and confirmed via `strings`/`grep -a` that the descriptor still contains the pre-rename
`"3d.remodel"`/`"3D Remodel"` bytes. **Conclusion: the committed descriptor pair is stale relative to
source and has not been regenerated since before the artifact rename landed.** This matches the
S-END-TO-END catalog audit (§3) which independently dates remodel's served wasm to 2026-08-17.

No ticket dated today (2026-09-06) other than this one's own `📓️status.md` mentions remodel; no
other live session appears to be editing remodel right now (consistent with the clean git status).

## 3. Ticket-tree history (grep for "remodel" in `*.md` under 26/08 and 26/09)

- **26/08/12 DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS** — wave M3d (per
  `📓️status.md:1060` and `📓️wave-m3d-remodel-family-report.md`) moved the `algebra`/`optimize`/`lie`/
  `signal`/`spatial` math domains **out of `🧮️math` and wholesale into remodel's own schema** as
  `crate::algebra`/etc. (`🧬️schema/➕️algebra-internals/…`, 6259 LOC verbatim-copied), repointed remodel's
  8 engine-topic files (`mesh`/`motion`/`sfm`/`geo`/`feature`/`camera`/`reconstruction`/`dense`) from
  `math::` to `crate::`, added a real `RemodelRelativeCameraPose` `InferredField` impl, and — after a
  cross-wave interaction with a concurrent `cas`→`mathematical` migration that reopened remodel's
  `math` dependency — relocated the residual `MatG`/`VecG` generic-matrix symbols into the new
  `🔢️number` framework module (this is why remodel's Cargo.toml today depends on `number` instead of
  `math`, see §4). Reported result at the time: `semio-s-plugin-remodel` **41 → 0 cargo-check errors**
  (487 passed / 2 failed, both in photogrammetry algorithm code, out of scope) — **unverified now**,
  predates all subsequent commits.
- **26/08/16 ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET** — inventory only: remodel has exactly **1
  subset (`✳️any`)**, 1 editor, 1 viewer, **0 viewer modes**; sizing wave bucketed remodel into the
  last/smallest packet (P9, alongside writer/mathematical/vcs/animate/shooting/sequence/architect/
  process/note/sourcing).
- **26/09/02 UNIFIED-ARTIFACT-NAMING-AND-DEDUPLICATION** — `📓️rename-remodeling-and-emoji-ids.md` is
  the authoritative account of commit `96aa4f8c`: `📸️remodel`→`📸️remodeling` artifact rename (plugin
  directory/crate/id/label deliberately kept as `remodel` — only the artifact-scoped identifiers
  became `remodeling`). Explicitly logged that `cargo check -p semio-s-plugin-remodel --target
  wasm32-wasip2` **could not get a clean run that session** — 12 retries, error count oscillating
  47→27→23→19→23→23, root-caused to `🧰️framework/🔨️modules/🕸️graph`'s auto-generated
  `🌉️generated-value-bridge.rs` trailing a concurrent, in-flight `🔱️trinity ♻️rewrite→♻️rewriting` /
  `🖍️draw` rename on another live session — **not remodel's own bug**. (See §4: this file today
  contains only `rewrite_lhs` references, no `draw_layers`, and its git history stops at `fe7c8a8f8b`
  2026-09-05 03:53 — structural evidence, not a compile-verified one, that this specific gate has
  since cleared.)
- **26/09/02 SEPARATE-ARTIFACT-STANDARD-SUBSET-…** — `📓️e1-remodel-shooting-layout.md`: closed all 91
  `missing-external-oracle` breaches across `layout`/`shooting`/`remodeling` (35 remodel mutation
  kinds, 34 covered — `commit-reconstruction` structurally excluded by design). Added a Python
  second-implementation + `asset://`-declared fixtures per kind; **zero Rust files edited** (matches
  commit `7ad363fd`, 2026-09-03, 11 files/+1373/-163).
- **26/09/02 COMPLETE-SEMIO-END-TO-END terra-\* audits** — remodel rows:
  - `📓️terra-all-plugin-artifact-runtime-matrix.md:101`: `` `remodel`: remodeling | O64 | none | none | same ``
  - `📓️terra-plugin-artifact-runtime-census.md:57`: `remodel | 2 | present | source-only`
  - `📓️terra-all-plugin-provider-expansion-blueprint.md:46`: remodel listed among ~32 plugins with
    `unavailable-no-receipt` — no package-owned native factory receipt currently emitted.

## 4. Build-gate dependency chain (no compile run — file/commit evidence only)

Remodel's `Cargo.toml` deps and their latest touching commit:

| dep | path | latest commit (date) |
|---|---|---|
| stdio (`full-artifact-catalog`) | `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust` | b0dfa0f09b (2026-09-05 19:04) |
| os-kernel | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` | 3a6a9d6bfc (2026-09-05 22:02) |
| graph | `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust` | fe7c8a8f8b (2026-09-05 03:53) |
| geometry | `🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust` | fe7c8a8f8b |
| number | `🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust` | fe7c8a8f8b |
| pixels | `🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust` | fe7c8a8f8b |
| pack | `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust` | fe7c8a8f8b |
| io-base64 | `🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust` | 21fbcd3538 (2026-09-02 12:19) |
| schema | `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust` | fe7c8a8f8b |
| plugin | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust` | fe7c8a8f8b |
| dispatch-macros | `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust` | fe7c8a8f8b |
| value-derive | `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust` | fe7c8a8f8b |

Sibling E2E gate reports (all dated 2026-09-05, all fresher than remodel's own last cargo attempt):

- **`26/09/05/S-END-TO-END/📓️stdio-check-census.md`** (most authoritative, direct measurement): at
  04:12 both native and wasm32-wasip2 `stdio` checks hit **EXIT 101** on a single external error —
  `semio-framework-os-kernel` E0432 `super::schema::DirectorySpaceDetailV1` unresolved import
  (attributed to a concurrent, in-flight COMPLETE-SEMIO-END-TO-END directory-schema rewrite). **By
  18:21 update: both cleared** — native `cargo check -p semio-s-plugin-norm --lib` (compiles stdio as
  a dep) EXIT 0/0 errors/16 warnings; wasm32-wasip2 `cargo check -p semio-s-plugin-stdio --target
  wasm32-wasip2 --keep-going` EXIT 0 after 86m52s, 0 errors (189 warnings in `semio-framework-plugin`).
  Conclusion recorded there: **`semio-s-plugin-stdio` compiles clean on both targets as of 18:21
  2026-09-05**, i.e. before HEAD's last 3 remodel-touching commits landed same evening/next day.
  Residual stdio-only gap noted: missing owner descriptor pair (needs a wasm build + `describe`).
- **`26/09/05/BLOCK-PLUGIN-END-TO-END/📓️status.md:26`**: independently confirms the same os-kernel
  E0432 clear at 04:29 ("the one remaining `DirectorySpaceDetailV1` reference is under
  `#[cfg(test)]`"), and at :26 separately reports a **serde→`ToValue`/`FromValue` migration** in flight
  in stdio with **225 errors measured at 04:12** by peer session semio-4f — this predates and is
  presumably folded into the 18:21 stdio-census green result above; no later re-measurement of this
  specific migration by name was found in any 09-05/09-06 ticket.
- **graph `generated-value-bridge`**: `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🌉️generated-value-bridge.rs`
  today contains only `rewrite_lhs` references (no `draw_layers`), mtime 2026-09-03 19:44, last
  committed at `fe7c8a8f8b` (2026-09-05 03:53). No 09-05/09-06 ticket mentions this file or
  `generated-value-bridge`/`rewrite_lhs`/`draw_layers` at all — absence of complaint plus the missing
  `draw_layers` reference is consistent with (but does not prove, unverified without a compile) this
  gate having cleared since the 09-02 report.
- **`26/09/05/S-END-TO-END/📓️explore-per-plugin-blockers.md:48`** (per-plugin table row, verbatim):
  `` remodel | remodeling | `📸️remodel/🗿️artifacts/📸️remodeling` | 3/0 | 0/0 | n/y | n/y | n | 1+0 | OK | catalog audit: missing mutation module | Mutation module ownership compile error family. ``
  — columns are Mig E/V, Batch E/V, Fac E/V, Proof, Ex, Stub, Descriptor, Ticket status, Concrete
  blocker; the audit's own words: a "mutation module ownership compile error family."
- **`26/09/05/S-END-TO-END/📓️explore-catalog-build-state.md:51`**: `remodel | plugin | 📸️remodel | Y |
  2026-08-17 20:02 | Y | Y | Y | remodel | -` — served core wasm dated **2026-08-17**, i.e. 19 days
  stale relative to HEAD, confirming §2's descriptor-staleness finding independently.
- **`26/09/05/S-END-TO-END/📓️explore-action-migration-recipe.md:413`**: remodel listed among plugins
  "with 0 actions or non-compiling crates," blocked "mostly by `#[path]`/mutation-module E0433/E0599
  errors from the `✳️base`→`🧱️base` rename in flight."
- DRAW and RASTER 09-05 status files were checked for graph/stdio-error mentions; DRAW's status.md has
  none, RASTER's history report only references the generic BLOCK-derived "shared framework churn"
  signature (60 errors: E0433/E0046/Result-return/async-trait/Label-From/ToValue), not anything
  remodel-specific.

**Target-dir artifact check**: no `target-remodel*` directory exists. `target/debug/.fingerprint/`
has two remodel fingerprint dirs (`semio-s-plugin-remodel-f670e4f905010827`,
`…-cd71643ca2b3ad55`), both **empty** (dir mtime 2026-09-05 20:28) — some cargo invocation touched
these very recently but produced no cached unit output. No `*remodel*` files anywhere under
`target/wasm32-wasip2` or `target/wasm-dev`. **No successful remodel build artifact exists anywhere in
the shared target trees right now.**

**Current cargo/rustc processes** (`ps -eo pid,ppid,etime,args`, none touching remodel): peer `-p`
targets in flight are `procedural` (wasm build), `semio-hub` (×3, lib/test/check variants), `space`
(check), `norm` (test), 2× full-workspace `cargo check --workspace --keep-going --all-targets`,
`semio-framework-os-kernel` (test ×2), `demonstrator` (test), `raster` (check), `block` (check),
`lowpoly` (test), `stdio` (wasm build), `energy` (check), `draw` (check ×2), `gis` (test),
`semio-framework-os` (test). No `-p semio-s-plugin-remodel` invocation is running.

## 5. Cross-plugin codemod fallout — remodel's own async-fn counts

Counted via `grep -rc` over `*.rs` under `✏️s/🔌️plugins/📸️remodel` (261 `.rs` files total, 803 `async
fn` occurrences total):

| pattern | files | occurrences |
|---|---|---|
| `async fn print_dsl` | 4 | 4 |
| `async fn parse_dsl` | 4 | 4 |
| `async fn diff` | 37 | 37 |
| `async fn inverse` | 71 | 71 |
| `async fn handle` | 43 | 44 |
| `async fn encode_op` | 4 | 4 |
| `async fn decode_op` | 4 | 4 |
| `async fn render` | 13 | 20 |

The 4 `print_dsl`/`parse_dsl` files are exactly: `🧬️schema/📸️snapshot/🦀️.rs`,
`🧬️schema/📸️snapshot/📝️text/🦀️.rs`, `✏️editor/🎚️config/🦀️.rs`, `✏️editor/👥️presence/🦀️.rs` — the same
4-file shape BLOCK's coordinator found and fixed per-subset (block has 3 subsets × 4 = 12; remodel has
1 subset (`✳️any`) × 4 = 4, consistent).

Compared against the actual sync trait signatures (confirmed by direct grep, not memory):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4900-4901`: `fn parse_dsl(text: &str) ->
  Result<Self, TextError>;` / `fn print_dsl(&self) -> String;` — **sync**, no `async`.
- Same file, lines 10282/10295 etc. (multiple impl sites): `fn encode_op(&self) -> Result<Vec<u8>,
  crate::os_spr::ProtocolError>` / `fn decode_op(...)` — **sync**.
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:128,152-153`: `fn inverse(&self, base: &P) ->
  Self;` and `fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>; fn inverse(&self, base: &P) ->
  Vec<Self>;` — **sync** trait methods.

This confirms remodel carries the same `async fn` vs sync-trait mismatch (E0053-shaped) that BLOCK's
coordinator found and stripped in its own 12 files, and that BLOCK's status.md explicitly named
remodel as one of "12 other plugins" with the same fallout — **not independently compile-verified for
remodel**, but the file/line-level trait evidence and the file-count match are exact. `async fn
handle`/`async fn render` (43+13 files) were not cross-checked against their trait file
(`🔌️plugin/🦀️.rs:26623,26972` per DRAW's ticket, a `#[path]`-mounted file not read in this pass) —
flagged unverified.

## 6. Host state at end of this exploration

- `vm.swapusage`: **64895.75 MB used / 65536 MB total (99.0%), 640.25 MB free.**
- `uptime` load averages: **206.74, 188.43, 165.94** (1/5/15-min).
- rustc processes: **57**. cargo processes: **70**.

This is a maximally stressed host — no build should be launched by this or any other ticket until
swap and load recover substantially, independent of anything remodel-specific.
