# Audit S11 — Verification Gates, Zero-Touch, Build Health

Auditor (Sonnet 5, read-only), session 11, 2026-09-25 00:48–01:2x CEST. No edits, no builds/tests run.
All findings below are either a cited file path + mtime/timestamp, a `git` read-only command, or a direct
grep/find result executed in this session. Session 11 itself was only ~20 minutes old at write time
(started 00:40; `date` read 00:48:55) — every session-11 `wp-*.md` is still at skeleton/TODO/PENDING
stage with zero fresh captures, so "most recent measured result" below is session-10 (2026-09-23/24)
evidence unless noted.

**Methodology caveat (important):** `git log -1 --format=%ci -- <path>` is **not usable** for
per-file freshness here. `HEAD` (`fe0033d12a`, actual commit time `2026-09-24 22:36:35 +0200`) touches
`🌎️hub`, the MCP module, and `✏️s` simultaneously — a single auto-commit sweeping the whole tree, with
a synthetic message-embedded date (`🎆️26🌙️06☀️04` = 2026-06-04, six commits behind wall-clock time),
confirming project memory "Auto-Commit Message Date Is Fake". I used **file mtimes vs. capture mtimes**
instead (`find <src> -newer <capture>`), which is the reliable technique for this repo.

---

## 1. Gate census by outcome

### Outcome 1 — os `s` frontend + plugins

| Gate | Launch/nx target | Last result | Capture | Timestamp | Verdict |
|---|---|---|---|---|---|
| `framework-os` typecheck | `@semio-tech/framework-os:typecheck` | 373/373 pass (WP-O1b), superseding G18's 76-error red from the same day | `.tmp-ticket/wp-o1b.md` | 2026-09-23 17:24 | **STALE-GREEN** |
| `os-mcp` vitest | `os-mcp:test` | 52/52 | `wp-o1b.md` | 2026-09-23 17:24 | STALE-GREEN |
| `plugin-registry:check` | `plugin-registry:check` | 60/61 (1 skip) | `wp-o1b.md` | 2026-09-23 17:24 | STALE-GREEN |
| `os-host-rs` vitest | (same suite family) | 373/373 | `wp-o1b.md` | 2026-09-23 17:24 | STALE-GREEN |
| `renderer-wgpu:test` | `renderer-wgpu:test` | 1282 pass / **~89 fail** (WP-O1c), still open in WP-O1d ("0 failures" target not reached; captures end mid-run) | `.tmp-ticket/wp-o1d/generated/wgpu-full-serial4.txt`, `unique-fails.txt` | 2026-09-23 19:41 | **RED** |
| 35-kind spawn/mutate/undo/redo inside `dev s` | (WP-S14/S15 matrix) | S14 "second run" partial (stalled once already) | `wp-s14.md` | 2026-09-23 17:08 | STALE-PARTIAL; S15 (session 11) has not started its rerun — status table all `(filling)` |
| `dev s` cold-boot latency | O2c graph/serve profiling | cold graph 31.3 s CPU / 98 s wall under load; warm serve 3/3 cache | `wp-o2c.md` | 2026-09-23 19:39 | STALE-GREEN, and explicitly "under load" (not a clean-machine number) |

Session-11 evidence: none yet (`wp-s15.md`, `wp-r8.md`, `wp-w2.md` all skeleton, mtimes 00:3x–00:48).

### Outcome 2 — hub backend

| Gate | Target | Last result | Capture | Timestamp | Verdict |
|---|---|---|---|---|---|
| `os-hub:test-quick` | `os-hub:test-quick` | 229/231 | `.tmp-ticket/wp-h1.md` | 2026-09-23 17:43 | **STALE-GREEN** |
| hub `documents::` / gateway | cargo test | 9/9, framework-server lib 88/88 | `.tmp-ticket/wp-c4d/generated/test-hub-documents.txt` | 2026-09-23 18:10 | STALE-GREEN |
| hub TS parity/neighbour/server | vitest | green (ts-parity/ts-neighbour/ts-server) | `.tmp-ticket/wp-h2/generated/ts-*.txt` | 2026-09-23 23:58 | STALE-GREEN |
| `/readyz` fresh-hub boot | `readyz.json` | ready, catalog A bound by content hash | `.tmp-ticket/wp-h1b/generated/readyz.json` | 2026-09-23 18:16 | STALE-GREEN |
| live Postgres/Neo4j two-client | `two-client-postgres-1.txt`/`two-client-neo4j-1.txt` | present but **blocked**: coordinator logged Docker Desktop engine hung, live pg/neo4j proofs paused pending user restart | `work-packages.md` 15:05 note | 2026-09-24 15:05 | **NEVER-CONFIRMED / EXTERNALLY BLOCKED** |

**Freshness check (measured):** `find "🌎️hub" -newer wp-h2/generated/test-quick-4.txt` → **24** `.rs`/`.ts`
source files under `🌎️hub` are newer than the last recorded hub test-quick run (e.g.
`🗿️artifact-authority/🦀️.rs`, `🔐️auth/📤️command/🦀️.rs`, `💡️inference/🏃️runtime/🦀️.rs`). Session-11
WP-H9 (`.tmp-ticket/wp-h9.md`, written 2026-09-25 00:33) has all 6 items at **TODO**, no re-run yet.
**Verdict: hub is STALE-GREEN with confirmed drift, not re-verified this session.**

### Outcome 3 — browser + wgpu collaboration

| Gate | Target | Last result | Capture | Timestamp | Verdict |
|---|---|---|---|---|---|
| collab-e2e (browser, two users) | Playwright collab-e2e | best recorded: **3/10** (WP-O3); unit-level presence overlay PASS (WP-C3/C3b) | `wp-o3.md`, `wp-c3b.md` | 2026-09-23 14:2x–14:43 | **RED** (live) / STALE-GREEN (unit) |
| STEP 14 peer-cursor asserts | same harness | "rides on O3's collab-e2e run", blocked at the time by a taxonomy error (since fixed) | `wp-c3b.md` | 2026-09-23 14:24 | NEVER-CONFIRMED |
| Kernel sync actor↔worker parity | `sync` suite | 60/60 (WP-C2b), TS parity 7 + neighbours 9 | `wp-c2b.md` | 2026-09-23 17:34 | STALE-GREEN |
| Checkpoint/re-projection live 10/10 | WP-C8 | "live 10/10 **waits on catalog**" — not yet run | `wp-c8.md` | 2026-09-24 17:56 | **NEVER-RUN** (blocked on W2) |
| Zero-touch clean-state `dev s`+hub, timed | WP-C10 item 4 | **ANALYSIS IN PROGRESS**, no measurement | `.tmp-ticket/wp-c10.md` | 2026-09-25 00:34 | **NEVER-RUN** |

Session-11 WP-C10 records all 5 of its own items as PENDING/IN-PROGRESS; item 1 explicitly notes
"hub 7800 not ready at 00:4x" — the live collab gate is transitively blocked on WP-W2
(`.tmp-ticket/wp-w2.md`: hub status **BOOTING**, item 3 "final rebuild" **WAITING**).
**Verdict: collaboration's live proof has never been green; this is the weakest of the four outcomes.**

### Outcome 4 — AI over the semio MCP

| Gate | Target | Last result | Capture | Timestamp | Verdict |
|---|---|---|---|---|---|
| `client-e2e` | MCP client e2e | 30/38 (WP-GJ2); root-cause of the `inference_run` hang still "running" in WP-GJ3 | `wp-gj2.md`, `wp-gj3.md` | 2026-09-23 13:34 / 18:40 | **RED** |
| MCP surface probe | 28 tools / 7 resources / 5 prompts, `inference_list` 74 rows | pass | `wp-gj2.md` | 2026-09-23 13:34 | STALE-GREEN |
| `semio-framework-os-mcp` lib tests | cargo test | large run captured (`mcp-lib-2.txt`, 47.9 KB) | `.tmp-ticket/wp-g9/generated/mcp-lib-2.txt` | 2026-09-24 20:44 | STALE-GREEN, freshest of the four outcomes |
| hub-agent-participant | 17/17 continuation target | not yet reconfirmed this session; G10 item 3 lists it PENDING | `.tmp-ticket/wp-g10.md` | 2026-09-25 00:34 | **NEVER-RUN** (session 11) |
| generic inference quartet live | G10 item 2 | PENDING; G9's commit-binding landing (item 1) still **IN PROGRESS** | `wp-g10.md` | 2026-09-25 00:34 | **NEVER-RUN** |

**Freshness check (measured):** `find <mcp module> -newer wp-g9/generated/mcp-lib-2.txt` → only **3**
files changed (`💡️inference/💼️jobs/🦀️.rs`, and two `🧪️tests/🔬️quick` files) — much smaller drift than
hub's 24, but they sit exactly in the inference-quartet code path G10 is about to land.
**Verdict: MCP is the freshest outcome (smallest source drift since last green run) but its two headline
live proofs (client-e2e, hub-agent-participant) have never both been green together.**

---

## 2. Zero-touch

**Devcontainer wiring is intact and unregressed** since the 2026-09-19 Z1 fix: `.devcontainer/devcontainer.json:36`
still runs `postCreateCommand: ["bun","nx","run","workspace:setup"]`, and
`📋️project.json` → `targets.setup.dependsOn` still contains all nine `deps-*` plus `setup-git`,
`prepare`, `repo-mcp:build`, `@semio-tech/framework-os-mcp-rs:build` (verified live, matches Z1's fix
verbatim). This was **not re-executed** end-to-end in this audit (setup is a real build, out of scope
for a no-build read-only pass) — it is a structural check only.

**Never proven end-to-end, on any platform, in six days of ticket history:**
- WP-C10 item 4 ("Zero-touch clean-state `dev s` / `▶️start` with local hub, timed") is the
  session-11 owner of exactly this claim and reports **ANALYSIS IN PROGRESS**, not measured.
- Its predecessor WP-O3b's scope was literally "Zero-touch hub by default: `dev s`/`▶️start` brings up
  local hub + local-bootstrap credentials" — carried forward through O3b→C7→C8→C10 without ever
  landing a green, timed, single-command result.
- Every capture I found across `.tmp-ticket` and `.tmp-ticket-0918` (hundreds of files) is from **this
  machine, macOS (darwin)**. I found **zero evidence of a native-Windows or native-Linux run** of
  `workspace:setup` / `dev s` / hub boot anywhere in the ticket history — only the devcontainer path is
  cross-platform by construction (and even that hasn't been run inside an actual container this
  session). AGENTS.md requires zero-touch parity across devcontainer, native Windows, macOS, and Linux;
  this is a live, unaddressed gap, not just a stale one.
- The MCP zero-touch leg (WP-GJ2, 2026-09-23) reported "zero-touch `ensureMcpBinary` (content-hashed
  staging)" as done, but that predates the client-e2e hang that is still open (§1) — so "MCP connects
  zero-touch" and "MCP works once connected" are two different claims and only the first is evidenced.

## 3. Launch registration drift

- `.vscode/launch.json` (6899 lines) and `.vscode/🧩️launch.seed.jsonc` (3816 lines) were both last
  touched 2026-09-24 19:3x–19:4x and committed in the 22:36:35 sweep — actively maintained, not stale
  files.
- One concrete drift instance is on record and was **fixed same day**: `work-packages.md` 14:10 note —
  `test-package-body-policy` was missing from both files (AGENTS.md launch-registration rule); it was
  registered under `4_build` (`🧹clean🧩️taxonomy📦️package-body-policy`, order `206.175`).
- The 2026-09-19 Z1 audit (`.tmp-ticket-0918/📓️z1-zero-touch-and-launch-rows.md`) found and fixed, at
  that time: wrong renderer defaulting on bare `dev` alias rows, 11 duplicate launch names, 2 unresolved
  `nx run` targets, 3 broken compounds, and landed "327 configurations, 118 project:target pairs, 0
  unresolved, 0 duplicates". **That resolver probe has not been re-run since** — six days and (per
  §4) thousands of file changes later, `launch.json` has roughly doubled in size and its
  unresolved/duplicate counts are unverified. This is a re-verification gap, not a known break.
- No new drift was introduced by session 11 itself: every session-11 slice (`wp-h9`, `wp-c10`, `wp-g10`,
  `wp-t12`, `wp-w2`, `wp-r8`, `wp-s15`, `wp-wg7`, `wp-u5`) is still at skeleton/TODO stage (all written
  00:3x–00:48), so no new permanent commands have landed yet to check.
- T10/T11/G9's landing scripts (`wp-t10/switch.py`, `manifest-align.py`, `wp-g9/g9-apply-commit-binding.py`)
  are correctly **not** in launch.json — they are one-off ticket-folder codemods per AGENTS.md, not
  permanent product commands. No drift there.

## 4. Uncommitted tree size / half-landed edits

- `git status --short | wc -l` = **2741**.
- `git diff --stat` (vs `HEAD` = `fe0033d12a`, actual commit time 2026-09-24 22:36:35 +0200) = **2722
  files changed, 10770 insertions(+), 7707 deletions(-)** in the ~2h12m since that commit — consistent
  with an ~8-slice fleet editing concurrently on one shared worktree (no worktree isolation, per rules).
- This confirms the auto-commit pattern is real (see Methodology caveat above): the message-embedded
  date (`2026-06-04`) is frozen/synthetic while the actual git commit timestamp is live and correct.
  Future auditors should treat `git log --format=%ci` (the commit metadata timestamp) as ground truth
  and the message text's embedded date as decorative/stale, and should default to file-mtime diffing
  (`find <dir> -newer <capture>`) for per-path freshness, not `git log -1 -- <path>` (every commit
  touches most of the tree).
- I did **not** attempt a full dangling-symbol sweep across a 2722-file diff — out of the 45-minute
  budget. I spot-checked the one concrete lead I had (G18's 2026-09-22 ShellHost TS errors,
  `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`): the file and its sibling directories
  (`📇️directory-bootstrap`, `🌱️artifact-creation`, `🪪️host-bootstrap`, two `dialog-origin`/`presence-scope`
  browser variants) still exist at the same paths. I did not diff line-by-line against G18's cited
  error locations (would require opening an ~8000-line file, disproportionate to the budget) — this is
  an honest coverage gap in this audit, not a clean bill of health. WP-R8 gate 1e
  (`renderer-react typecheck`) is the right place to close it with a real re-run.

## 5. Gaps, ranked

**P0**
1. **Hub gate is stale with confirmed drift, zero session-11 re-verification.** 24 hub source files
   changed since the last `os-hub:test-quick` (229/231, 2026-09-23 17:43); WP-H9's 6 items are all
   TODO. Acceptance: `os-hub:test-quick` + hub TS parity/neighbour/server suites green, captured with a
   timestamp newer than every touched hub source file. Owner: **H9**.
2. **Collaboration has never had a live green run.** Best recorded collab-e2e is 3/10 (unit-level
   presence PASS only); C8's "10/10 waits on catalog"; C10 item 1 explicitly PENDING because hub 7800
   isn't ready. Acceptance: collab-e2e 10/10 + STEP 14 peer-cursor asserts, captured live against a
   fully-published hub 7800. Owner: **C10**, blocked on **W2**.
3. **Zero-touch end-to-end has never been measured as one timed run**, on any platform, and cross-platform
   parity (native Windows/Linux) has literally zero evidence anywhere in the ticket history despite an
   explicit AGENTS.md requirement. Acceptance: one timed, launch-row-only run from clean state to
   `s` + local hub + semio MCP, captured on macOS native and inside the devcontainer at minimum.
   Owner: **C10** (hub/collab leg), **S15** (`dev s` leg), flag Windows/Linux as **NEW**.

**P1**
4. `renderer-wgpu:test` is RED (~89 failures as of 2026-09-23 19:41, WP-O1c/O1d unresolved), and the
   wgpu shell outcome (WG7, all items TODO this session) depends on it. Acceptance: 0 failures,
   re-run after T12/W2 land. Owner: **WG7** / **R8**.
5. AI/MCP's two headline live proofs (client-e2e 30/38, hub-agent-participant) have never been green
   together; G9's commit-binding landing (G10 item 1, IN PROGRESS) touches the same inference/jobs code
   the last green lib-test run covered. Acceptance: client-e2e 38/38 + hub-agent-participant 17/17 +
   one live agent-edit-seen-by-human capture, post-landing. Owner: **G10**.
6. The launch-registration resolver probe (unresolved/duplicate/compound checks) has not been re-run
   since 2026-09-19, across a launch.json that has roughly doubled in size since. Acceptance: re-run
   the resolver, 0 unresolved / 0 duplicates / all compounds resolve, freshly captured. Owner: **R8**
   or **NEW**.

**P2**
7. Historical test-infra debt from K2 (2026-09-18/20) — orphan vitest suites
   (`@semio-tech/presentation-react` 147 tests collecting nothing, `@semio-tech/print` target not
   starting since 09-14, `actor/cold-pair` orphan suite) — has no closure evidence in `work-packages.md`.
   Acceptance: re-run K2's ownership-gate census; confirm each cited suite now collects >0 tests or is
   explicitly retired. Owner: **R8**.
8. Multi-step manual sequences (W2's describe-all→generate→check→restage→publish chain; T12's contract
   rerun) exist only as prose across preambles/wp files, not as one registered launch row/nx target.
   Once stable, collapse into a single command per AGENTS.md's zero-touch + launch-registration rules.
   Owner: **W2** / **T12**, non-urgent while landing is still mid-flight.

---

Sources: `.tmp-ticket/📓️work-packages.md`, `📓️session-11-preamble.md`, `📓️landing.md`, `📓️wp-{c10,g10,h9,t12,w2,r8,s15,wg7,u5}.md` and their `generated/` captures; `.tmp-ticket-0918/📓️{g4-zero-touch-and-run-paths,z1-zero-touch-and-launch-rows,v1-verification-gates,k2-test-infrastructure,g18-tree-compile-health,audit-build-infra}.md`; `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `.devcontainer/devcontainer.json`, `📋️project.json`; `git status --short`, `git diff --stat`, `git log -1`.
