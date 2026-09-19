# G1 — Goal-level gap audit (post waves 1–3, 2026-09-19)

Read-only audit, slice G1 (Sonnet). Cross-checks every `📓️*.md` in this ticket (all 8 audits + 20 worker
reports read in full) against the current tree: file existence, git status, a handful of directly-verifiable
claims (spot checks below). No cargo/nx builds run; no files edited except this one.

**Spot checks performed** (all confirmed against the live tree, not just report text):
- `git status --short | wc -l` → 632 modified/untracked paths at audit time — a very large uncommitted
  surface; treat every "fixed" claim below as "fixed in the working tree", not "fixed and merged."
- `✏️s/🔌️plugins/🗟️artifacts` — confirmed **gone** (P1's claimed removal landed).
- `.mcp.json` — both `repo` and `semio` entries present, `semio` carries the 5-scope `--scopes` arg M1/M2
  describe.
- `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp` — exists, built 22:14 09-18 (M1's fix landed on disk).
- No `login`/`Login` hit anywhere under `💻️os/🔨️modules/📺️renderer` — confirms no auth/login UI exists.
- `🛂️SpaceAdministration/🟦️.tsx` and `🏛️ShellHost/🟦️.tsx` are the only "invite" hits — a real but thin surface.
- `.devcontainer/Dockerfile` exists (152 lines listed dir); `.devcontainer/post-create.sh` does **not** exist
  (confirmed by M1's own finding that a Go test asserting its existence is already dead).
- `git log --oneline -5 -- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf` shows no commit since `3250e6cb90`
  (09-15) even though H1/P2/S1 all describe an in-flight edit there as of 09-18/19 — that edit is still
  uncommitted (part of the 632).

---

## 1. Outcome-by-outcome status

### Outcome 1 — a working `os s` frontend with all plugins/artifacts

**Runtime-verified (actually booted + interacted with, by a worker in this ticket):**
- 8 of 34 plugins have a **dated, console-clean, state-changing interaction** on record this month:
  `raster`, `forms`, `note`, `fem` (2d+3d), `energy`, `layout`, `remodel`, `draw` (`📓️s1-plugin-coverage-matrix.md`
  rows). These are the closest thing to "working plugin in the os" that exists today.
- The **host-mode multi-plugin design** (hub session lists all 60 catalog rows, lazy per-artifact wasm
  install) is real and unit-verified: `bun nx run @semio-tech/framework:test` 141/141,
  `🎬️host-activation` 6/6, `🎬️activation-owner` 5/6+3, `wgpu-plugin-install` 5/5
  (`📓️o1-multi-plugin-hub.md` §5, `📓️o2-activation-follow-ups.md` §4). **Not** runtime-verified: no `dist/<profile>/🔌️plugin-modules/`
  exists for the `s` hub variant, so no worker has ever opened a foreign-kind artifact inside a live `dev s`
  session (`📓️o2-activation-follow-ups.md` §4 "Live `dev s` probe — NOT run").
- `dev multi` deleted (confirmed dead code removed, `📓️o1-multi-plugin-hub.md` §1).

**Test-verified only (compiles/unit-tests green, never driven through a browser this month):**
- 12 previously-UNTESTED plugins (`writer`, `mathematical`, `vcs`, `animate`, `sequence`, `architect`,
  `reasoning`, `norm`, `playbook`, `imperative`, `dag`, `space`): P2 got 41/46 of their crates to `cargo
  check` green; B1a/B1b got 10 of 12 to boot in a real browser this session, but **zero of the 12 pass the
  interaction bar** (a dispatched verb that mutates the document + undo) — every one is refused by
  `InteractiveJobClassification::BatchOnlyPendingRewrite` or missing `command_from_action`
  (`📓️b1a-dormant-plugin-boots.md`, `📓️b1b-dormant-plugin-boots.md`). `reasoning` and `norm` are within one
  fix each (§2 below).
- `space` itself — the hub plugin — compiles clean (all 3 crates + wasm32-wasip2) but has **no boot evidence
  at all this month**; `dev s served` resolves a 132-Nx-task cold rebuild that no worker has let finish
  uncontended (`📓️b1b-dormant-plugin-boots.md` §"space", `📓️s1-plugin-coverage-matrix.md` ranked #1).

**Claimed but unverifiable from the tree:**
- The MCP capability catalog (`capabilities_search`) is claimed fixed in spirit by nothing — it is
  independently confirmed broken (0 hits for all 34 plugins) by both `📓️m1-mcp-servers-start.md` §6.1 and
  `📓️s1-plugin-coverage-matrix.md`'s headline finding, root-caused to `architect`'s duplicate capability id
  (`architect.s.architect.program@1/*#editor.setAdjacencyKind`) plus ~20 plugins with missing descriptor
  fields (`artifactSchema`/`windowKindId`/`executionProtocol`) or no `🔣️.json` at all (`block`, `stdio`,
  `playbook` before today's fix, all `imperative`/`process`/`sourcing` extensions). **Not fixed by any
  worker in this ticket** — flagged in both audits, in nobody's slice.

**Missing entirely:**
- A cold, uncontended `dev s` timing measurement — still nobody's, called out by both
  `📓️audit-multi-plugin-hub.md` plan item 5 and `📓️o1-multi-plugin-hub.md`/`📓️o2-activation-follow-ups.md` §5/§6.
- wasm32 (browser) lazy plugin install has a compiled door (`📓️o2-activation-follow-ups.md` §2) but **no
  artifact-open relay to call it from** — `handle_open_artifact_relay`/`switch_to_app`/`open_document` are
  still `#[cfg(not(target_arch = "wasm32"))]` only. Un-gating that relay for the browser wgpu shell is
  unclaimed by any slice.
- wgpu↔React parity (chord/camera 17/37, no Actions/Search pane bodies on wgpu) — owned by a sibling ticket
  (`WGPU-RENDERER-REACT-PARITY`), not this one, but it is the dominant blocker for "a usable wgpu-backed hub"
  per `📓️audit-os-frontend.md` P0-2.

### Outcome 2 — a working server hub backend (db, presence, auth, directory)

**Runtime-verified:**
- A fresh, non-symlinked `OS_HUB_DATA` boots the hub to `/readyz` with real zero-touch defaults: sqlite
  directory, filesystem document store, a local-bootstrap credential that authenticates, `/directory/spaces`
  answering `[]` (`📓️h1-hub-build-and-boot.md` §6.2, real curl output).
- `cargo build`/`os-hub:build-dev` succeeds and produces a 265 MB binary (`📓️c1-collaboration-e2e.md` §4.2).

**Test-verified only:**
- `semio-hub`'s own 2 `E0560` errors are fixed (confirmed: not reproduced by H1, P2 or C1 this session).
- `semio-framework-server` (the generic "server" product) is now green — 73/73 tests — but is depended on by
  **nothing** (`grep -rln semio-framework-server --include=Cargo.toml` → only its own manifest + root
  workspace list, per `📓️h2-server-crate-and-wave3-memo.md` §B.1). W3a de-closed its ten seams (a real,
  substantial rewrite, 943 insertions) but this is still pure framework work with zero hub callers.
- `cargo test -p semio-hub --lib`/`--bin os-hub` — **not re-run and re-confirmed green in this ticket**
  after H1's fixes; H1's own report has `<!--H1-TESTS-->` still an empty placeholder in §5. This is a real
  gap: the headline "hub compiles and boots" claim in §6.2 of H1's report is solid, but "hub's own test
  suite is green" is not independently confirmed on the current tree by any report in this ticket.

**Claimed but unverifiable from the tree:**
- `cargo check -p semio-hub` on **default features** currently fails — not on hub's own code, but
  transitively through `semio-s-artifact-stdio-pdf`, whose peer edit (`PDF-ARTIFACT-SPEC-COMPLETE`) is
  still uncommitted at audit time (last commit to that path is `3250e6cb90`, 09-15; H1/P2/S1 all independently
  observed the same 45-error break at different times spanning 09-18 evening through 09-19). **P3 (stdio-pdf
  callers) is the slice meant to close this and its report is an empty stub** (`📓️p3-stdio-semio-pdf-callers.md`
  — every section reads `_in progress_`). So "hub builds on default features" is currently **false**, and the
  designated fixer has not started.
- Postgres/neo4j directory backends — `📓️audit-hub-backend.md` reports the drivers now compile in
  (real `sqlx-postgres`/`neo4rs` deps), but no worker in this ticket re-ran `cargo check -p semio-hub
  --features postgres,neo4j` to confirm it actually links. Unverified either way.

**Missing entirely:**
- **Wave 3 (hub as `ServerInstance`) is at step 3 of 14** (`📓️h2-server-crate-and-wave3-memo.md` §B.7):
  W3a did the generic de-closing; W3b (`HubInstance: ServerInstance` + durable stores, steps 4–5) is an
  **empty stub** (every section `_pending_`). Steps 6–14 (socket grants into the gateway, one presence
  implementation, one policy system, six `ServerModule` impls, shrinking `bootstrap.rs` from 8522 lines,
  a real TS twin) have not been started by anyone. This is a 13,000–16,000 line migration; ~950 lines of it
  (W3a) are done.
- A durable, hub-owned implementation of `AuthorityStore`/`ProjectionStore`/`BlobStore`/`SessionStore` over
  `db::Database`/`SqliteDirectory` (§B.5 item 3) — zero lines written.
- **The zero-budget bug fix in `📜️script.ts` (§2 of H1's report) is real code that landed**, but it was a
  workaround for a symptom; the actual root architectural question (does the hub *need* `semio-hub`
  dependent on `semio-framework-server` at all, or should Wave 3 be abandoned) is still open per H2's own
  memo — it recommends "execute Wave 3" but flags it as a multi-week migration with no guarantee anyone
  picks it up next.

### Outcome 3 — working collaboration between users over the hub

**Runtime-verified:**
- The wire protocol itself: `cargo test -p semio-framework-replication --lib` 274/274, re-run and confirmed
  identical by both the phase-0 audit and H2 (`📓️h2-server-crate-and-wave3-memo.md` Part A).
- Three real root-cause defects fixed and unit-tested by C1: the `readHistory`/"missing HistorySnapshot
  frame" mislabelling (a guest-side `instance busy` refusal hidden by a bad client-side error message,
  §1 of `📓️c1-collaboration-e2e.md`), a bounded retry ladder for it (4 new tests, real),
  and a **process-aborting store defect** in `ArtifactCodec::print_mirror`/`apply_ops_binary` that dropped a
  parsed envelope without detaching its owners — this is described as aborting the process (native) or
  wasm-trapping the browser shell on **every artifact bootstrap install**, and was caught only because C1
  wrote the bootstrap-ordering test the brief asked for. This is the single most consequential bug fix found
  across the whole ticket: `cargo test -p semio-framework-os-kernel --lib --features sync bootstrap` 4/4,
  two of which were red before the fix.

**Test-verified only:**
- The 10-step `collabRunScenario` (`🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts`) now has two new assertions
  (a real cross-tab round-trip counted by `ServerFrame::Commands` frames, and a hub-restart-with-an-
  unacknowledged-in-flight-edit step) — but **the scenario has never been run end to end in this ticket**.
  C1's own report has `<!--C1-SCENARIO-->` filled with the new step *code*, but no pass/fail counts; C1b
  (meant to run it) has every section (`<!--C1B-RUNLOG-->` through `<!--C1B-FILES-->`) empty. The last
  *actual number* on record for this scenario anywhere is still `26/08/17`'s **2 of 8** (pre-dating the new
  steps 9–10 and every fix C1 made this session).
- A structural blocker to running it at all was found and worked around, not fixed: `os-hub:dev` cancels its
  own trusted-catalog build because `buildBudgetMs() === 0` is misread as "already expired"
  (`📓️c1-collaboration-e2e.md` §4). H1's report claims a fix for this exact bug (§2, "a zero build budget is
  unlimited, not expired") — so the fix likely exists in the tree, but **no report in this ticket confirms
  the collaboration E2E was actually re-run against the fixed hub**.

**Claimed but unverifiable from the tree:**
- The wgpu native shell's collaboration path — compiled and unit-tested only, "never observed running" per
  the 08-17 ticket and not touched by any worker here.
- Presence session-colour wire extension — C1 found it "already landed" (a file move made the original
  cross-lane note stale), and confirmed `cargo check -p semio-framework-plugin` green, but the actual
  `cargo test -p semio-framework-ui --lib --features wgpu presence` **cannot even compile** right now due to
  an unrelated, in-flight peer rename of the `🖱️ui` wgpu module tree (`crate::wgpu::events`/`widgets`/`chrome`
  unresolved). So presence-with-colour is unverified end-to-end today, blocked on a different, unnamed
  worker's rename.

**Missing entirely:**
- A live, two-browser run of the full 10-step scenario against the fixed hub, on this tree, with a number
  attached. This is the single most important missing data point for "is collaboration working" and it does
  not exist anywhere in this ticket's 20+ reports.
- Document-level (not directory-level) restart-with-concurrent-edits was *added as a test step* but never
  *run*.

### Outcome 4 — working AI integration over MCP

**Runtime-verified:**
- Both `.mcp.json` servers now answer `initialize → notifications/initialized → tools/list → resources/list`
  from a Claude-Code-shaped client, confirmed with a real handshake probe
  (`🐍️m1-mcp-handshake.ts`, output quoted in `📓️m1-mcp-servers-start.md` §4): `repo` answers 9 tools + 8
  resources; `semio` answers 27 tools (added `inference_run`) + 8 resources, and a real
  `resources/read semio://workspace` against a live headless workspace bound to the repo root.
- `inference_run` genuinely dispatches through the real `ArtifactInferenceRouter` for a previously
  `channel.not-wired` service (`wfc`'s `s.wfc.wfc3d.solve`), not just the pre-existing GIS Map special case
  — verified with real committed `🔣️.json` descriptors over a real `--folder` workspace
  (`📓️m2-agent-surface-and-inference.md` §4.2, 6/6 tests). **Caveat, stated honestly by M2 itself**: the
  guest half (compiled `.wasm` → `semio.infer`) is never exercised — the test stops at the `ArtifactChannel`
  seam with a mock.
- The chat panel echo-mock is deleted; a real bridge-frame-backed conversation view exists
  (`AgentToolCall`/`AgentToolResult`/`AgentMessage`, `semio://ui/agent-messages`), verified server-side (10
  new tests) and TypeScript-clean, but **never verified rendering pixels in a running shell** — M2 states
  this explicitly ("no browser probe was run").

**Test-verified only:**
- The credential-seal false positive (R1) is fixed and regression-tested (9 carrier names still trip it, 13
  benign host-harness names, including `CLAUDE_CODE_*`, do not) — this is what let the very handshake probe
  above run at all, so it is effectively also runtime-verified via M1's own probe.
- The `os-mcp` crate's full `--lib` suite: M2 measured `316 passed, 21 failed` and classified all 21 as
  pre-existing/peer-owned, not caused by its slice. **M3, whose entire job was "drive this to green," is an
  empty stub** — every section reads "(filled below)". So the crate is still red at 21 failures with nobody
  currently fixing it.

**Claimed but unverifiable from the tree:**
- `capabilities_search` still returns 0 hits for every plugin (same root cause as Outcome 1's catalog gap:
  `architect`'s duplicate capability id + ~20 descriptor-drift skips). Named as a known gap in M1 §6.1 and
  independently in S1; not this ticket's fix target, but it directly limits what an MCP agent can discover
  and do via `capabilities_search`/`capabilities_describe` today (the gateway-only fallback still answers
  structurally, so tool calls that don't need catalog lookups work; ones that do, don't).
- Wgpu renderer chat panel — still echoes locally (parity-debt note added, not fixed; `📓️m2-agent-surface-and-inference.md` §7).

**Missing entirely:**
- No LLM/model-provider is wired anywhere, by design (confirmed independently by the phase-0 audit and M2's
  new README section) — "AI integration" here means exposing the OS to an external agent over MCP, which is
  the ticket's actual stated goal, so this is not a gap against the stated goal, just worth stating plainly
  so nobody expects an in-app model call.
- The `repo` MCP server's tool-surface reconciliation is done (9 canonical tools, matching the documented
  set) but the **stale, self-contradicting `26/09/06` ticket** that claims a different, already-shipped state
  is still closed and uncorrected in ticket metadata (M1 added an annotation note but did not reopen/fix the
  ticket itself — this is explicitly named as left-standing in M1 §6.2).

---

## 2. Ranked remaining work items

Excluded per instruction (already in flight): **H1, W3b, D1, P3, C1b, M3, A1, V1, T2, T3, T4, B1a, B2b,
B3a–d**.

### P0 — blocks a goal outcome outright, small enough for one slice

1. **`architect`'s duplicate MCP capability id blocks the catalog for all 34 plugins.**
   File: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/…/✳️any/✏️editor/🦀️.rs` (the
   `setAdjacencyKind` action/window-kind registration — exact line not yet found by any report; `grep -n
   setAdjacencyKind` in that tree is the entry point). Fixing this one duplicate id is, per
   `📓️s1-plugin-coverage-matrix.md`'s own words, "the single highest-leverage catalog fix in the repo" —
   it currently zeroes out `capabilities_search` for every plugin, not just architect's. One slice: find
   the duplicate registration (two window kinds or two apps both declaring the same action id on the same
   capability path), rename/scope one of them, re-run `capabilities_search` and confirm >0 hits.

2. **`reasoning`'s retained-publication authority admits only `MoveNode` — one file from a full interaction
   pass.** File: `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:303`.
   Swap the bespoke ~700-line cursor's `factory()` for the framework's generic
   `bounded_config_store_one_item_preparation_factory::<Snapshot, Mutation>`, following the `dag`/`trinity`
   precedent named explicitly in `📓️b1b-dormant-plugin-boots.md` §"Exact next steps" item 1. This would make
   `reasoning` the first of the 12 dormant plugins to pass the full interaction bar.

3. **`norm`'s missing `command_from_action` bridge — the cheapest win in the whole dormant-plugin batch.**
   File: `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` (verbs are already `Migrated` with a factory and
   publication contracts at lines 408-423; only the `{action,args}` bridge is missing). `📓️b1b-dormant-plugin-boots.md`
   estimates ~20 lines, modeled on `✏️s/🔌️plugins/🗒️note/…/✏️editor/🦀️.rs:313`.

4. **Run the 10-step collaboration scenario end to end, once, against the fixed hub, and record a number.**
   No file changes required if H1's build-budget fix and C1's three fixes are already in the tree (they
   appear to be, per git status showing uncommitted work matching both reports) — this is a single
   measurement slice: boot `dev s` for two simulated users + a local hub with H1's fix active, run
   `collabRunScenario` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts`), and
   report pass/fail per step. This is the single most important missing data point in the entire ticket —
   every other collaboration claim is built on top of "the pieces are fixed," never "the scenario passes."

5. **`imperative`'s Actions pane is completely empty — zero dispatchable verbs.** File: the app's window
   definition under `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/…/✳️any/✏️editor/🦀️.rs` — add
   `.window_kind_actions(...)`/`.window_kind_action_refs(...)` calls for both window kinds (F10 in
   `📓️b1b-dormant-plugin-boots.md`); currently the app declares 10 actions in its manifest but they reach no
   window, so every shell dispatch is `undeclared-action` before the classification gate is even reached.

### P1 — real gap, not blocking the others, one slice each

6. **`🧱️block` has zero committed MCP descriptor and an unresolved 09-05 compile question.**
   File: `✏️s/🔌️plugins/🧱️block/🔣️.json` (missing entirely — `a1-catalog-skips.txt`: `NotFound: no
   committed descriptor`) plus its `📸️snapshot/🦀️.rs`'s `async fn print_dsl` vs. sync `ArtifactDsl` trait
   mismatch (E0053) marked "investigating" in its own ticket's last line, never confirmed fixed since.
   One slice: write the descriptor, resolve the E0053, confirm a fresh wasm build (its sibling `puzzle`
   rebuilt today; `block` has not rebuilt since 09-05).

7. **`space`'s cold `dev s` build has never been let run to completion uncontended.** No code fix — this is
   a scheduling/measurement slice: reserve a quiet window (no concurrent cargo/rustc), run `dev s served` or
   the full activation chain to completion, record wall-clock and disk cost, and produce the first-ever
   activation receipt (`dist/runtime/react/dev/s/activation`) so subsequent slices (browser open-foreign-kind
   probes, the collaboration scenario) stop needing to build from cold every time.

8. **wgpu's artifact-open relay never compiles for `wasm32` at all**, so the browser wgpu shell has no lazy
   plugin-install caller despite the install door itself compiling clean on that target
   (`📓️o2-activation-follow-ups.md` §0.5/§5). Files: `handle_open_artifact_relay`/`switch_to_app`/
   `open_document` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
   (currently `#[cfg(not(target_arch = "wasm32"))]`). This is described by its own author as "its own packet"
   — pulls in persistence bindings, the hub transport and `system_fs` for the browser target.

9. **`🌎️hub/🧪️tests/🧱️foundation-source`'s launch entry is missing**, a pre-existing failure named by M1
   §6 item 4: add `⚖️gate🧱️hub-foundations📐️source` to `.vscode/🧩️launch.seed.jsonc` and regenerate
   `.vscode/launch.json`. Small, mechanical, unowned.

10. **The `26/09/06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE` ticket is closed with a summary that
    actively contradicts the live tree** (claims `💻️client/**` removed, 9-tool Go/Rust parity, a
    `SEMIO_REPO_IMPLEMENTATION` switch — none of which exist). M1 added a dated annotation but did not
    reopen or correct it. One slice: reopen it, correct the summary or fold its still-true intent into a
    fresh ticket, so the next person doesn't trust a false "closed" status over the working tree (the repo's
    own "live predicate, not derived artifact" convention).

11. **`🔋️energy`'s undocumented `#[ignore = "[DEBUG] W3-1b probe"]`** — flagged but not removed by P1 (P1
    removed a different, similar one in the same file; this second one at `…/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs:318`
    per the phase-0 audit's Q3 still needs a look — confirm with `grep -n '\[DEBUG\] W3' ✏️s/🔌️plugins/🔋️energy`
    whether P1's Task 4 fix already covered it or whether a second stray ignore remains).

12. **`.mcp.json`'s `semio` entry's tool-count drift documentation** — 27 tools now (was 20 in the README),
    `inference_run` added. One slice: update `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md`'s "twenty
    stable tools" language to match the current 27, so the next auditor doesn't have to recount from source.

### P2 — polish / follow-up, not urgent

13. **`verify taxonomy`/`verify taxonomy report` is unrunnable at HEAD** — throws inside
    `planMoveReferenceAuthority` on a stale frozen-coordinate digest for
    `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json`
    (P1's finding #6, not fixed by P1 or V1's excluded slice — confirm V1 didn't already cover this before
    picking it up).
14. **`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧫️fixtures/🔣️.json`** is wholesale stale
    (26 of 29 dependency aliases no longer exist) — P1 fixed one dangling name but flagged the rest as the
    fixture-sweep owner's work.
15. **`🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` and
    `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts`**
    are syntactically repaired (T1) but still semantically gutted from the 09-14 migration incident —
    `compilePrintDocument` lost its `spawn(tectonic, …)` call, `BuildScript` has an empty body. Reconstructing
    intent needs the print/coordinator owner, not a generic TS slice.
16. **`🔀️dispatch` macro cannot close a `Send`-future port** (W3a §8) — a ~30-line additive branch would let
    `dyn_enum_close!` handle all ten server-product ports instead of hand-written delegation; would simplify
    hub's future `HubInstance` work (W3b/step 4+) considerably. Worth doing before W3b goes much further.
17. **`ui.chat.*` translation keys in `🖱️ui`** are now dead (M2 deleted their only caller) — a `🖱️ui`-scope
    cleanup, not urgent.

---

## 3. Cross-cutting risks — user-facing flows with no clear owner

- **Auth/login UI in os**: confirmed **zero** hits for `login`/`Login` anywhere under
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer`. The hub has a real session-mint/credential
  system (`SessionMintResponse`, `LocalHubCredential`, the broker-proof scheme) and the os client consumes
  it (`📇️directory/🔌️client/🦀️.rs`), but there is no browser-facing UI for a human to authenticate against a
  *remote* hub (as opposed to the local-bootstrap dev credential every ticket's boot recipes use). If the
  product goal includes a hosted/shared hub with real users signing in, this UI does not exist and no ticket
  in this audit's scope owns it.
- **Space/directory browsing UI**: `🛂️SpaceAdministration/🟦️.tsx` exists and is real (admin connections page,
  per the hub-backend audit), but it is an **admin** surface, not an end-user "browse my spaces / switch
  space" flow. `space`'s own `home`/`studio` apps are the closest thing to end-user space browsing and
  `space` itself has never been booted this month (§1 Outcome 1).
- **Invite/share flow**: hub-side invite redemption exists (`/directory/invites/{token}/redeem`, confirmed
  in the hub-backend audit's route table) and is unit-tested at the hub level, but the only UI-side "invite"
  hits in the whole renderer tree are `SpaceAdministration` and `ShellHost` — no dedicated invite-composition
  or accept-invite screen was found by this audit. Worth a targeted grep-and-read by whoever picks this up;
  not fully characterized here.
- **Hub deployment config**: `OS_HUB_STORAGE_BACKEND`/`OS_HUB_DIRECTORY_BACKEND` default to zero-touch
  fs/sqlite, which is correct for dev, but nothing in this ticket addresses a real multi-tenant deployment
  story (TLS termination, a non-loopback bind scope beyond what `/readyz`'s `bindScope: "loopback"` reports,
  secrets management for postgres/neo4j URLs). Out of scope for every slice audited here — flag for whoever
  owns "ship the hub somewhere real."
- **Devcontainer zero-touch**: `.devcontainer/Dockerfile` exists; `.devcontainer/post-create.sh` does
  **not** (confirmed by both this audit's own `ls` and M1's independent finding that a Go test asserting its
  existence is already dead code). If devcontainer-based zero-touch onboarding is a goal, this is an
  unowned, silently-broken piece — nobody in this ticket's 20+ reports touches it.
- **i18n en/de coverage**: better than most cross-cutting risks — every new UI surface added in this ticket
  (M2's agent chat panel, O2's plugin-install bands) explicitly added EN+DE strings through the existing
  `registerUiTranslationBundles`/`shell_chrome_string` mechanisms. The risk is narrower than "missing i18n":
  it is **drift between the wgpu and React chrome string tables** as new features land on one renderer
  before the other (e.g. M2's chat panel is React-only; the wgpu chat panel still echoes locally and was not
  given the same EN/DE keys because it has nothing real to translate yet).
- **Mobile layout**: no plugin or shell surface in this audit's scope discusses a phone/tablet layout; the
  `mobile`/`responsive`/`viewport` grep hits found are all incidental (test names, unrelated component
  props), not a deliberate responsive-layout system. If mobile is a stated goal anywhere in the product
  direction, it has no owner and no start in this tree.

---

## 4. One honest meta-finding

A large fraction of this ticket's own reports are **empty stubs mid-fleet-death**: `📓️p3-stdio-semio-pdf-callers.md`,
`📓️w3b-hub-instance-and-durable-stores.md`, `📓️m3-mcp-tests-and-wgpu-agent-panel.md`, `📓️b2b-dormant-plugin-interactions.md`
are all "(filled below)"/"_pending_" placeholders with zero measured content, and `📓️c1-collaboration-e2e.md`'s
C1b continuation (the part that was supposed to actually *run* the collaboration scenario) is the same. The
status.md's own "Relaunch" history (three coordinator sessions, workers dying mid-slice twice) explains why,
but it means several of this ticket's most load-bearing claims — "hub boots," "collaboration scenario
passes," "os-mcp suite is green" — currently rest on code that exists in the working tree but has never been
exercised end to end by anyone who wrote down a number. §2 item 4 (run the collaboration scenario once) is
the highest-leverage single action available to close that gap for Outcome 3; the equivalent for Outcome 2
is finishing P3 (excluded here as already in flight) and then re-running `cargo test -p semio-hub`.
