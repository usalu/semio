# Final summary — Hub Spaces, Live Presence and Collaborative Studios

Ticket `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS`. Coordinator: Opus 5 main chat.
23 worker lanes (Sonnet 5) + 5 read-only scouts/audits (Haiku 4.5), run in 6 waves with a barrier
between each. Every lane's own report is in this folder as `📓️<lane>-report.md`, with raw command
output as `🧪️<lane>-*.txt`.

## What was asked, and what is true now

| Requirement | State |
|---|---|
| Running `s` with running plugins | **Fixed and verified.** `semio-s-plugin-space` could not compile to `wasm32-wasip2` at all (tokio's `net` feature was reaching wasm through `os-host-full` → kernel `sync`); lane 4-E target-gated it. `space` and `writer` now produce real `.wasm` modules and both shells boot with 59 plugin crates in the e2e harness. 20 of 33 catalogue crates build to wasm; the 13 that fail are attributed crate-by-crate to other tickets. |
| Hub with persistence for spaces | **Done and verified.** Directory rewritten as an append-only event log (`hub_directory_event`) with SQL projections, a `decide`/`DirectoryService` command path, and `rebuild_projections` replay. Restart-survival is asserted by a real integration test that boots the actual binary twice against one `OS_HUB_DATA` (lane 3-E). |
| Live presence between users in the same app/viewer/editor | **Done and verified at the hub.** Presence is scoped `(space, document, surface)` with `?surface=` carried out of band on the document WS; commands still fan out document-wide. Proven by `presence_roster_is_scoped_per_surface` and by lane 3-E's three-client test (two peers see each other, a third on another surface does not, yet still receives the command relay). Browser-side presence chrome exists in both shells but is not yet observed end to end (see below). |
| Home app = overview table of all spaces (create/open/delete) | **Built.** The Home surface's virtual-file-system scene was replaced with a real table fed by the event-sourced directory read model unioned with local-only spaces, with create/rename/delete/share/open commands relayed as `os.directory.*`. Creating a space from the browser and seeing it appear is **verified end to end** (e2e STEP 1). |
| Space app = table of all artifacts (create/open/delete) | **Built**, with a new plugin-owned `s.space` artifact (the space's artifact index), four mutations with proper outcome codes, a members panel, and viewer/editor surfaces. Unit-verified; browser-verified only as far as the e2e reached. |
| Studios sharable for collaborative editing | **Built** (invite by email, roles, invite links, share tokens; membership is directory-owned and event-sourced). Hub-side sharing is verified by integration test; the browser share flow is where the e2e currently stops. |
| Artifacts open in a viewer or an editor | **Built.** `os.open-artifact`/`open-artifact-with` now carry `documentId`+`spaceId` so opening an artifact actually opens its document — closing a gap the previous ticket had left open. |
| Edits saved and checked into vcs | **Built and a real bug fixed.** Auto check-in on idle/volume, explicit `#s-checkin`, checkpoint-on-close, status pill, `TouchArtifact` on the space index. Lane 3-G fixed a genuine framework store defect where **a checkpoint taken after edits arrived over the hub was rejected** ("invalid edit reference"), and threaded checkpoint authors through `history_command` (every checkpoint had been authorless). |
| Dev configs for user 1 and user 2 on different ports, same hub | **Done.** A `users` dimension in the playground registry generates `🛠️dev🖥️s👤️1⚛️react` (6072) / `👤️2` (6073) and wgpu twins (6067/6068), plus `🧭️compound🖥️s👥️users🗄️os-hub`, `🛠️dev🗄️os-hub🛡️admin` (8790) and `⚖️gate🌎️collab-e2e`. A plugin-build lease lets the second `dev s` process skip the build and serve the first one's modules, so both users really can run at once. |
| Hub admin page (all spaces, management, active connections) | **Done.** A React SPA under `🌎️hub/🔨️modules/🛡️admin` (nx `os-hub-admin`), served by the hub binary at `/admin`, with Overview/Spaces/Users/Connections/Documents/Events pages, live connection updates over `/directory/ws`, kick and rebuild actions, en+de. Verified: `GET /admin` → 200, `GET /admin/api/overview` → JSON. |

## Test results at close

| Suite | Result |
|---|---|
| `cargo test -p semio-hub --lib` | **11 passed / 0 failed** |
| `cargo test -p semio-hub --bin os-hub` | **18 passed / 0 failed** |
| `cargo test -p semio-s-plugin-space --lib` | **210 passed / 0 failed** (was 124/15 when the crate first linked) |
| `cargo test -p semio-framework-os-kernel --lib` | **988 passed / 0 failed** |
| `cargo test -p semio-framework-plugin --lib` | 218 passed / 5 failed — all 5 attributed to other tickets in `📓️w4-d-report.md` |
| `cargo check -p semio-framework-os-renderer-wgpu` | **0 errors** (had never compiled during this ticket until lane 3-I) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib` | 314 passed / 2 failed — both pre-existing since 2026-08-06 |
| `cargo build -p semio-s-plugin-space --target wasm32-wasip2` | **succeeds**, produces the `.wasm` |
| `HUB_E2E=1 bun nx run os-hub-ts:test` | **passes** — the 7-step hub collaboration scenario against the real binary |
| `bun nx run os-hub-admin:test` | **5 passed / 0 failed** |
| `bun nx run @semio-tech/plugin-registry:check` | **passes** — launch.json is fresh |
| react renderer vitest | 322 passed / 9 failed — the same 9 pre-existing failures three separate lanes independently confirmed |

## Bugs found and fixed that were not part of the brief

Several were blocking the product for everyone, not just this ticket:

1. **`EditorApp`/`ViewerApp` reported a placeholder `APP_ID` of `"surface"`**, which a live ownership check compared literally — so **every button click on every editor/viewer surface in the entire product was silently rejected**. Introduced by the per-subset surfaces work, whose own summary admits its browser end-to-end never ran. Fixed by lane 4-G (`ArtifactApp::instance_id`).
2. **The hub set no CORS headers at all**, while the hub and every shell are different origins by design — so `POST /auth/sessions` was blocked and identity could never resolve in a browser. Fixed by lane 4-H.
3. **`TableWindowKit` serialized rows as bare positional arrays**, which the renderers cannot read — every `TableWindowKit` table in the repo (16 DIN report windows, several stdio subsets, energy, mathematical) was rendering broken. Fixed by lane 3-F.
4. **Checkpoint-after-relay was broken** in the store (above), and **all checkpoints were authorless**. Fixed by lane 3-G.
5. **`s` could not be built for the browser at all** (tokio/wasm). Fixed by lane 4-E.
6. **9 of 11 `worker.postMessage` call sites in `ShellHost` posted un-encoded objects**, throwing `TypeError` on both shells at runtime. Fixed by lane 4-F.
7. The hub's own dev script **replaced `process.env` wholesale**, silently discarding the launcher's `OS_HUB_PORT`/`OS_HUB_DATA`, and its import path was broken so `bun nx run os-hub:build` failed outright. Fixed by lane 0-B, which also resolved the hub-vs-`s` port collision on 6070 (hub is now 8787 everywhere).
8. `PluginBuilder::document_app` carried an unnecessary `SemanticMutation` bound that blocked the space plugin and at least one unrelated plugin from compiling (lane 2-0); `semio-s-plugin-puzzle` needed the matching `SemanticMutation` twin (lane 3-H).

## What is NOT done — stated plainly

- **The browser end-to-end passes 2 of its 8 steps.** Final state: **STEP 1 PASS** (user1 creates a
  space from Home; user2's Home shows the same row) and **STEP 7 PASS** (admin API + `/admin` HTML).
  STEPS 2–6 and 8 fail, each with its real failure text recorded rather than skipped. The trajectory is
  the honest measure of the work: `📓️w3-c-report.md` 0/8 with nothing rendering → `📓️w4-f` (buttons
  reachable, `TypeError` gone) → `📓️w4-g` (surface dispatch unblocked) → `📓️w4-h` 1/8 → `📓️w4-i` 2/8.
  **Current blocker, diagnosed but deliberately not fixed:** a `plugin instance busy` /
  `readHistory: missing HistorySnapshot frame` retry storm inside
  `🧱️elements/PluginRuntime/🟦️component.tsx`. Lane 4-I traced it to specific lines, then found that file
  `git status`-modified by a **concurrent peer session** mid-rewrite (the functions it identified no
  longer exist there) and stopped rather than edit someone else's in-flight file. That is the correct
  call under this repo's concurrency rules, and it is where the next session should start.
- Along the way lane 4-I fixed three further real bugs: seeded dialog arguments were dropped by
  `effectiveActionArgs` (so sharing reached the hub with an empty `spaceId` and got a correct 403); the
  vite dev entry used a `./`-relative script so **any hard navigation to a nested SPA route 404'd**; and
  the new `/spaces/{id}` route branch lacked the idempotency guard its sibling had, causing an infinite
  document-reopen loop (dozens of WS open/close cycles, now one).
- **Postgres and neo4j directory backends are written to parity but have never been compiled**, because
  the `🛢️db` crate declares `sqlite`/`postgres`/`neo4j` as empty features with no optional drivers wired
  (pre-existing since 2026-08-12, peer-owned). Every hub lane therefore verified with default features
  only. This also makes `bun nx run os-hub:test*` unusable, since that target hardcodes `--all-features`.
- **The wgpu shell's collaboration wiring is compiler-verified and unit-tested but never observed
  running.** Its 9 identity/directory/presence tests pass; no native window was driven.
- **Auto check-in, checkpoint-on-close and `TouchArtifact` were ported to React only.** The native wgpu
  shell has no history/uncommitted-edit tracking to hang them on (lane 3-A's finding).
- **5 `semio-framework-plugin` and 2 wgpu renderer test failures remain**, each attributed with commit
  evidence to `FULL-STDIO` (still open) or to work predating this ticket. None were caused here.
- **13 of 33 plugin crates still fail to build for wasm**, all with crate-local bugs unrelated to the
  fix that unblocked `space`; attributed individually in `📓️w4-e-report.md`.
- `verify gate` still fails at its first step (dependency-cruiser, 828 violations), exactly as it did
  before this ticket started — baseline captured in `🧪️w0-gate-baseline.txt`. No new failing step.

## Process notes

- Two audits (`📓️audit-w4-taxonomy.md`, `📓️audit-w4-evidence-and-leases.md`) found **zero lease
  violations** and **zero unbacked claims**: every "passes" in every lane report has a matching log,
  and every lane that could not finish something said so. `🗄️stdio/**` and `📜️world.wit` were never
  touched, since `FULL-STDIO` is still open.
- Authorized foreign touches, all recorded: the `document_app` bound split, the `🏪️store/🔄️sync`
  surface field, two root `package.json` workspace entries, and the `🏪️store`/`🔌️plugin` work taken up
  after the `MUTATION-OUTCOMES` ticket closed at 02:44 and released its leases.
- CLAUDE.md compliance was audited specifically for the rules most likely to slip under time pressure:
  event-sourcing with no CRUD/CRDT, no compat layers, schema-first Rust/TS twins, region structure,
  emoji docstrings, one `📜️script.ts` per bundle, and **en+de for every user-visible string** — all clean.
