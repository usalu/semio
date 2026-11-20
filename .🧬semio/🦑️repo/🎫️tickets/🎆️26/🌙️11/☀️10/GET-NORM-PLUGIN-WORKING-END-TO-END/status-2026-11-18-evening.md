# Status — 2026-11-18 21:31

## Progress

| Area | State |
|---|---|
| Five norm components built and staged | done: `norm` 21:26, `norm-eurocode` 19:24, `norm-eurocode-materials` 19:28, `norm-eurocode-structural` 20:40, `norm-energy` 21:03 |
| Five play lanes (din4108, din18599, en1990, en1992, en1995) | active |
| Standards play server | `http://127.0.0.1:6033/` (frozen watch, play serve envelope); grid shows all 15 norm cards |
| **din4108 boots in play** | **yes**: chrome renders (Editor/Viewer, example "Compliant ETICS dwelling", Artifact/Catalogue/Inspection/Chat panels) within about 1 min in unthrottled Chromium |
| din4108 document body | blocked by a guest panic: `std::thread::spawn` in the norm compliance background worker (wasm32 has no threads). Fixed (inline evaluation on wasm32); `norm` rebuilding now |
| Other 14 apps | staged with the earlier fixes. The two latest framework fixes (displaced-surfaces wrap guard, thread-less worker) still need rebuilding for the four extension components |

## Expected finish

- `norm` rebuild with the worker fix: about 21:55. din4108 verification: about 22:00.
- If din4108 renders: rebuild the four extension components (about 25–40 min each, sequential because of the 16 GiB machine), reactivate lanes, verify the other 14 apps. Estimate about 00:30–01:30.
- Every further guest bug costs one rebuild cycle of about 25 min.

## Problems faced and fixed (this session)

Build and packaging (details in `norm-plugin-split.md`, `norm-assembly-audit.md`, `status-2026-11-18.md`):
1. wasm function limit → norm split into five plugins, identities renamed per plugin.
2. Component size above 512 MiB → shared norm link adapter strips names (577 → 285 MB).
3. Stale describe and transpile tools, unclassified window actions, two-segment fault codes, undeclared viewer utility, din18599 members size.

Play / dev server:

4. Puzzle engine pulled into scoped play → scope-conditional alias.
5. Dev server crashed on `/__semio/agent-bridge` → rendezvous plugin mounted.
6. Watch over the shared tree hung the server → frozen serve. The serve envelope expired after 3.5 h → serve under the play target.

Framework boot chain (all framework-wide, so every plugin benefits; details in `norm-boot-context-refusal.md`):

7. Return-table/publisher birth split across turns.
8. Turn entry equality check.
9. Unreported origin-refresh spend in receipts.
10. Lifecycle slot table needed 659 MB → boxed slots.
11. Return-budget ledger double counting; bind check against the persistent ledger.
12. 30 s activation deadline killed boot in throttled tabs → 1 h (renderer stock).
13. Bridge `poll` failure receipt read from `failure.payload`.
14. Turn never sliced → adaptive slice capped at 8 s.
15. Worker batched several events into one poll → sequential sub-turns.
16. Concurrent crossings rejected with "turn in flight" → per-actor FIFO queue in the shard worker.
17. Displaced-surfaces scan wrapped around the instance table forever → wrap guard.
18. Compliance worker `std::thread::spawn` panics on wasm32 → inline on thread-less targets (rebuilding).

## Anticipated

- More first-open issues per norm family: example loading, viewport 3D surfaces (hidden pane cannot render WebGL/WGPU), other `std::thread`/time APIs in shared norm crates.
- The temporary bridge patch (1 h deadline) must be replaced by restaging the renderer stock everywhere. All `[DEBUG]` probes must be removed at the end.
- The idle per-turn origin-refresh cost still consumes the lifetime grant, mitigated by the larger stock. Proper per-epoch renewal is a follow-up design item.

## Resource use

- **Native builds** are the bottleneck: one component at a time, because parallel component builds share the target directory and 16 GiB RAM (an earlier parallel run was SIGKILLed). Units are cached, so only the changed closure recompiles.
- **One Sonnet agent** drives the framework diagnosis. Each round needs a real browser result, so rounds are sequential by nature.
- **Fleet:** unavailable (remote cache unreachable, no builder), so I fell back to native tools.
- **Browser:** the built-in pane is hidden (throttled to about 1 wake per minute). Verification therefore runs in an unthrottled headless Chromium.

## Update 2026-11-19 03:10

- `norm` build blocker `process.source.read-bytes-exceeded` fixed: the window-config test fixture moved into a cfg-gated module (`norm-source-boundary.md`). `norm` staged at 02:04.
- The exchange retirement tail works: commands 2–6 retire and route in both din4108 and en1990.
- `norm-eurocode` was rebuilt with all fixes (02:51), and the en1990 lane is reactivated. en1990 boots its full page frame (Eurocode 0, example "High Consequence Office").
- **Shared blocker in every plugin:** command 7 (`setActiveExample`) decodes but never retires. The two suspected causes are a silently retired refusal, and a surface capture whose reader is never closed (`displaced true` forever). `[DEBUG]` probes for both are in the `norm` build that is running now. Details are in `norm-boot-context-refusal.md`.
- Build queue: `norm` (running), then `norm-eurocode-structural`, `norm-eurocode-materials`, `norm-energy`.

## Update 2026-11-19 04:05

- Build fixes this round:
  - `semio-plugin-plugin-program` gained a `semio-plugin-abi` dependency, needed by the `[DEBUG]` surface probes.
  - The window-config crate was split by a concurrent agent, and the retry loop rode it out.
  - `norm` staged at 03:52.
- din4108 probe (`🗑️generated/din4108-probe2.log`): seq 7 `setActiveExample` decodes, then turns into `failed-decoded` and retires forever. The host declares `plugin.command-ingress-stalled` (9753 continuations). The actor then stops without publishing its UI surfaces.
  - No `refused:` line and no displaced-surface lines were logged.
  - Two bugs: the cause of the setActiveExample failure, and a failed-command retirement that never terminates. Both are with the diagnosis agent.
- `norm-eurocode-structural` is building, then `norm-eurocode-materials`, then `norm-energy`.

## Update 2026-11-19 06:45

- **Root cause of the seq 7 failure.** The adaptive per-turn wall slice in the bridge source rewrote `deadlineUs` on every turn. The guest treats the deadline as operation identity: seven `validate()` sites refuse "original command receiving requires its exact immutable source and caller". The slice is removed from `🌉️component-bridge-source/🟦️.ts`, and fuel bounds the turns. The fix is staged with `norm` 05:49.
- `handle_action_invocation` lets an undeclared `setActiveExample` through as a mutation (catalogue route), applied by the static analysis agent (`norm-set-active-example.md`).
- **Current stall.** For seq 7 every turn logs `gate ingress.resume: ready false page false` with an ample grant (8.28 M items), no yield and about 49.8 M fuel. The only pending work is `command_ingress CommandPending(seq 7)`. `resume()` (ingress slot settle or return-slot lend) never becomes ready. This is with the diagnosis agent.
- Correction: earlier probes 7 and 8 silently dropped new `[DEBUG]` lines because of a filter regex bug in the probe script. Probe 9 is authoritative.
- Extension components are rebuilding with the bridge fix.
