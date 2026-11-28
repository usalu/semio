# 🚀️ Development Speed — Clean Long-Term Proposals

These proposals come from friction that was actually observed while getting the sourcing plugin working end to end (2026-11-27). They are ranked by the time they would have saved today.

## Where Today's Time Went

| Cost | Evidence |
|---|---|
| A ~25 min wasm cycle per hypothesis | revert fault: 8 hub rebuilds; boot invariant: 4; Open File: 2 without any signal |
| Broken build infrastructure | 05:39–11:47 no wasm at all. Disk full → torn files in `.cache/b/r/m` (view.json, sealed snapshot, spill blobs, NUL-filled `units-*.jsonl`) |
| Shared-worktree collisions | peer re-broke retirement twice (15:51, 16:46); `🗑️generated` wiped mid-run; a stale `build.lock` removed under a live build |
| One shared dev server | every rebuild restarts 6081 and kills other agents' journeys; agents poll for "ready" |
| Opaque faults | `"(reference 0ce86205)"` is a message hash; invariants without owner/type; guest `eprintln!` invisible in the browser |
| Brittle journeys | a display-name rename (KVH → Konstruktionsvollholz) broke ~70 German steps |
| Manual chores | descriptor freshness, staging, exporting wasm, native-cargo env juggling |

## 1. Native Integration Tier for Plugin Runtime (Biggest Lever)

Today we have unit tests that take seconds and wasm journeys that take 30+ min. There is nothing in between, so every runtime hypothesis costs a full wasm cycle.

- **Native host harness:** a `semio-plugin-host-native` test harness that drives a plugin or hub component compiled as a native rlib through the *same* host traits the browser shim uses: turns, grants, retirement, publication receipts and media-in.
- **Fixtures from journeys:** scenario fixtures (boot → setActiveExample → curate → revert / discard / Open File) mirror journey steps 1:1. They are schema-driven, so the same scenario runs natively and in the browser.
- **Payoff:** the boot invariant, the revert fault, the discard receipt bug and the Open File livelock would each have reproduced in seconds to minutes. The TS "source-text" tests that grep Rust code become behavioural tests.

## 2. Self-Healing, Crash-Safe Build Store

- **Verify on read:** every content-addressed record (configuration views, sealed snapshots, spill blobs) is checked against its digest when read. On a mismatch it is evicted and treated as a miss; it is never fatal.
- **Uniform torn-record handling:** append logs (`*.jsonl` telemetry/history) skip torn or NUL-padded lines and compact them, as `readUnitHistoryV1` now does. This becomes a shared reader used by *every* journal.
- **Atomic writes:** write to a temp file, fsync, rename, then fsync the directory, as one shared primitive everywhere.
- **Disk watermarks:** repo GC kicks in at a soft limit (e.g. < 25 GB free) *before* ENOSPC. It evicts by LRU and lane owner, and never force-unmounts an image during writes.
- **`repo doctor --repair`:** scans the stores, evicts corrupt entries and reports them. Agents run it instead of hand-crafting `perl` NUL scans.

## 3. Isolated Dev Hosts per Agent

- **Leased instances:** `repo service-host lease sourcing --agent <id>` returns a private port and a private staged-modules directory. Rebuilds hot-swap *that* instance only.
- **Hot component swap:** the plugin host reloads one component (wasm or descriptor) without a process restart, and journeys reattach.
- **Ready signal:** a structured "ready / failed-at-boot(fault)" endpoint replaces log grepping and 45 s polling loops.

## 4. Coordination Primitives in the Repo MCP

- **Intent leases:** an agent declares the files or modules it is editing (`lease_claim(paths, purpose, ttl)`). Others see a warning *before* editing, plus a live feed of who is changing what. This would have prevented today's retirement edit war without blocking anyone.
- **Generated-output namespaces:** each agent gets its own scratch area (`🗑️generated/<agent>/`). Wiping another agent's namespace is refused.
- **Build queue as a service:** replaces ad-hoc `mkdir build.lock`. It provides a queue position and ETA, owner-aware stale-lock recovery, and coalescing, so 4 queued hub rebuilds become 1 when no sources changed in between.
- **Agent status board:** each agent's current task, holds and waits, readable by coordinators. This removes the duplicate investigations seen today.

## 5. Faster Wasm Builds

- **Split the hub component:** 293 MB with ~860 units. Use shared framework components with component-model linking, so a change to the sourcing app does not relink the framework.
- **Persistent incremental targets:** keep warm, quota-managed target dirs per profile instead of deleting cold dirs to save disk. Make the repo unit cache work across plain-cargo and `repo cargo` lanes.
- **Source snapshot at build start:** builds compile a sealed snapshot. Peer edits mid-build then neither break nor silently miss the build, and the build reports exactly which source revision it contains.
- **Reliable fleet offload:** with offload working, heavy wasm builds go remote and the 16 GB host stops OOM-killing. A local build becomes the fallback, not the default.

## 6. Observable Faults

- **Structured faults:** faults carry `{invariant id, owner type, ids, turn, receipt numbers}`. No more temporary `[DEBUG]` patches to learn the owner type.
- **Reference lookup:** the reference maps to a stored diagnostic record (stack plus context) under `.cache`, so `(reference x)` is lookup-able.
- **Guest output visible:** guest stderr and traces are routed to the browser console and the host log, with a per-session trace toggle settable at runtime, because env vars do not reach the browser.

## 7. Robust, Faster Journeys

- **Stable selectors:** select by stable schema IDs (row id, panel id, command id) rather than display text. Labels used for assertions come from the product's i18n catalog, so renames cannot break steps.
- **Affected-only runs:** run only the journeys whose dependency graph changed (`repo affected`). Shard steps across isolated hosts and contexts; today en+de run serially at ~30 min.
- **Rapid-input steps:** keep journey steps that fire inputs back to back, so lost-update bugs (like `set-filter-module`) surface early.

## 8. Remove Manual Chores

- **Descriptors as build outputs:** staging writes the committed `🛂️.descriptor.semio`/`🔣️.json` itself, and a check target fails on drift. No more manual copying, and no false "MATCH" against stale wasm.
- **Plain-cargo entry:** a `repo cargo --native` (or launcher) that sets the plain-cargo env consistently (wrapper, build dir, profile from the view), instead of per-ticket `native-profile.toml` and env lists.
- **Ticket scaffolding:** a shared rebuild/stage/serve/journey pipeline lives in the framework (not copied per ticket), with a progress and cancel API.

## 9. Lessons From the Evening (2026-11-27 18:00 – 2026-11-28 01:30)

- **Test crates rot silently:** `surface-fixture` (7 compile errors), `tests-14-2` (86), `tests-15` (9 failures) and snapshot-read (124) all broke from peers' API changes without anyone noticing. The fix is a per-crate "compiles + passes" gate that runs automatically after any change to a dependency (`repo affected` → `cargo check --tests`), with the breaking agent notified.
- **Vacuous tests:** history-patch laws passed for months because a seam was `cfg(test)`-gated in another crate. Laws should assert that they observed something (counters greater than 0). Seams belong behind a shared `*-testing` feature, never `cfg(test)` across crates.
- **Source-text tests:** TS tests that grep Rust source were the only coverage for three runtime fixes. They should be replaced with behavioural tests on shared fixtures. `OriginalJobFixtureCaller::close_decoder` / `receipted_decoder` and `CancelProbe` now exist, so use them.
- **Compile time mistaken for a hang:** a 900 s "hang" was compilation under load. Test runners should split build from run and report "compiling unit N/M" versus "test X running for T s".
- **Stale waiters and duplicate runs:** agents left 15+ `until grep …` loops on deleted files, and twice re-launched a test that was still running. The fix is a repo-MCP `watch(job-id)` primitive with an owned lifetime, plus refusing a second identical cargo invocation per agent.
- **Hidden fault ownership:** two stalls (rapid input, typed-operation settlement) were faults with no owning caller. The framework should guarantee that every operation reaches a terminal outcome, with a debug-build watchdog that names the operation and stage after N turns without progress.
- **Cancellation semantics:** a job that never checked cancel was invisible until the return path changed. Add a lint or test that every `InteractiveJob::step` calls `cx.cancelled_turn()` first.

## Suggested Order

1. Store self-healing and disk watermarks (§2): small, prevents total stalls.
2. Coordination leases, generated namespaces and the build queue (§4): stops collisions.
3. Isolated dev hosts with hot swap (§3).
4. The native integration tier (§1): the largest speed-up, the most work.
5. Structured faults (§6) and journey selectors (§7).
6. Hub component split and offload (§5).
7. Chore removal (§8).
