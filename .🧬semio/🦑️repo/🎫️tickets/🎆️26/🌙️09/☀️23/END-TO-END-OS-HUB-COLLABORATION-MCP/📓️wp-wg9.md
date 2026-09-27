# WP-WG9 — wgpu Shell (wasm32 Browser) Collaborates Over the Hub, Session 13

Slice WG9 (session 13, 2026-09-26 19:0x), successor of WG7 (handover [`📓️wp-wg7.md`](📓️wp-wg7.md)). Coordinator = main chat.
Ports: hubs 8050–8059 (inherits 8050: hold 78966 → os-hub 79010, data `.🧬semio/🌐hub/s11-wg7-hub-8050`), serves 6550–6559 (inherits
6552 pid 21493, 6553 pid 50393, `wp-wg7/serve/wg7-serve.ts`). Private cargo target: `.tmp-ticket/wp-wg9/target`. Captures:
`wp-wg9/generated/` (expendable). Durable logs: `.🧬semio/🌐hub/s13-wg9-logs/`. Landing rows: `📓️landing.md` § Session 13 Landing
Window. Guest requests: `wp-w3/requests/wg9.txt`.

## Session 13

| # | Item | Status |
|---|------|--------|
| 1 | Land `wp-wg7/s12-link-expiry-patch.py` (cuts > ~31.5 s expired the link) | **LANDED** — applied 26 19:13; kernel native check + laws `document_link_shortage_tests` 6/6 green 27 05:17 (fleet-b, post rule 24); wasm32 kernel browser `sync` 05:50 + wasip2 05:52 green; TS 3/3; landing row ✓ |
| 2 | Late joiners: lanes dedupe by operation id + hub catch-up tail carries a declared origin (kernel native + wasm32, React twin, hub) | **LANDED** (kernel + hub bootstrap) — kernel native + laws (echo 2/2, backbone parity 1/1) 05:17, wasm32 ×2 05:52, hub `--bins` green (H11 06:00); bin-unit pins reverted 06:32 (uncompilable before 07:00 in the convoys), prepared; live proof after W3's rebuild |
| 3a | S12-1 open items (de chrome / locale door, sign-in freeze patch) | **de chrome live** (s13a/b: B's hub workspace `Öffnen`, `Jemanden einladen`, Sync `Gespeichert`, German expiry line); sign-in patch **applied 14:56** (window 2), renderer wasm32 green 15:12, native check running |
| 3b | S12-2 peer cursors on a board-canvas kind | **red, root-caused, fix prepared**: s13d (puzzle2d, 7800 B3) attach ×2 + roster ✓, but A's heartbeats carry board views with NO pointer (0/293) → `Renderer::handle_pointer_move` skips `presence_pointer` while a board claims the move → `wp-wg9/s13-board-presence-pointer-patch.py` (dry run clean, needs a window) |
| 3c | S12-3 the human sees the agent's block | **DONE live, en + de: agent probes `s13d` de 8/8 and `s13e` en 8/8** (7800 B3, wasm32): roster `User Two · WG9 agent de (KI-Agent)` / `User One · WG9 agent en (AI agent)`, agent `addBlock` via the semio MCP, **the human sees the block in 3.0 s without a reload**, ledger 0→1. Earlier reds = MCP gateway re-mints identical block ids per session (routed to G11) |
| 3d | S12-4 accessibility (2 unnamed `application` nodes) | pending |
| 4 | wasm32 shell A↔B live on several kinds, en + de, presence + agent badge (after W3's all-package publish on 7800) | waiting for W3 |

### Session 13 log

- 19:06 start. Read AGENTS.md, preambles 13 + 12, `📓️wp-wg7.md`. Load 21, 1 rustc, wasm mutex free, 111 GiB free, swap 4.3/5 GB.
  Inherited processes alive: hold 78966 / os-hub 79010 (8050), serves 21493 (6552), 50393 (6553).
- 19:1x dry runs on the current tree: `s12-link-expiry-patch.py` (5 files, `generated/link-expiry-dryrun.diff`),
  `s12-echo-suppression-kernel-patch.py` (1 file + 1 law), `s12-echo-suppression-hub-patch.py` (2 files) — all anchors unique.
  Found: the hub's `FrontierAdvertise` catch-up (`🌎️hub/🏗️bootstrap/🦀️.rs` `handle_frontier_advertise(…, actor.clone())`) stamps the
  receiving socket's actor too — same class as the hello tail; the hub set is extended to name it with the declared catch-up origin
  (`wp-wg9/s13-echo-suppression-hub-patch.py` = WG7's set + that hunk + a law line pinning both call sites).
- 19:13 **item 1 applied** (`s12-link-expiry-patch.py --apply`, 5 files: kernel region, TS twin, schema text, fixture 52 steps, Rust law).
  Native kernel `--lib --tests --features sync,ureq` green 19:19 (6 m 03 s, no new warning).
- 19:20 **item 2 applied as ONE set**: kernel (`s12-echo-suppression-kernel-patch.py --apply`: region 🔁️DocumentEchoSuppression, both
  `Commands` arms admit by id, `applied_op_ids` on both actors, authored ids noted, new law `document_echo_suppression_tests`) + hub
  (`s13-echo-suppression-hub-patch.py --apply`: `HUB_CATCH_UP_ORIGIN = "hub.catch-up"` on the hello tail AND the FrontierAdvertise
  catch-up; bin-unit reconnect + joiner asserts pin it). Native kernel check green 19:28 (4 m 29 s).
- 19:29–20:11 hub check `semio-hub --bin os-hub --tests`: first run's rc lost to a `head` pipe; the re-run (20:11) ended rc=101 on a
  PEER's in-flight edit in `semio-framework-ui` (`♿️accessibility` 20:08: `node.pressed` before the contract field exists) — not mine,
  re-run after the peer's fix.
- TS twin laws (`🟦️.ts` in-source, `--testNamePattern "DocumentLinkShortage|DocumentEchoSuppression"`): **5/5** (3 link incl. the 2 new
  vectors, 2 echo) — `generated/l1-ts-laws.txt`.
- 20:10 rule 22 (memory): stopped the inherited hold 78966 / os-hub 79010 (8050, `stop` file → `HOLD_END stopped on request`) and serves
  21493 (6552) / 50393 (6553). Restart later from `wp-wg7/wg7-hub-8050.sh` / `wp-wg7/serve/wg7-serve.ts`.
- 20:17 kernel laws run 1: link 6/6, echo 2/2, but `os_store::sync::tests::backbone_parity` red — `ingestedMutationIds` read
  `known_op_ids`, which the native `Commands` arm no longer fills directly (only `persist_operations` does, and only with a folder). The
  test-only accessor now reports what the arm filters against (`known_op_ids ∪ applied_op_ids`). Re-run in progress.
- 20:39–21:03 the kernel native re-check and the filtered law re-run each sat > 10 min behind the build-dir lock convoy (no rustc
  child) → stopped (my pids 64125, 83844) per rule 25; chain `wp-wg9/wg9-land-chain.sh` (build-fleet-b, rule 26: kernel check →
  kernel laws → hub check → hub catch-up law → wasm32 kernel checks through the mutex) launched 21:03 — then the ~21:30 usage cut.
  `wp-w3/requests/wg9.txt` written (kernel ×2 reasons). Hub recipe copy `wp-wg9/wg9-hub.sh` (C11's, WG9 names) prepared.
- **09-27 04:58 resume (rule 28).** Reconciled: every hunk of items 1 + 2 is in the tree (kernel `🔄️sync/🦀️.rs` untouched since my
  20:18 edit; hub bootstrap edited by a peer 22:21, my 3 hunks intact; bin-unit 4 pins; echo law file present; TS twin + fixture
  vectors present). Nothing of mine runs (chain 1 died with the session). Load 8.9, 0 rustc, 155 GiB free.
- 05:01 landing chain 2 launched detached (`w2-detach.py`, pid 91474, log `.🧬semio/🌐hub/s13-wg9-logs/land-chain-2.txt`).
- 05:01 chain 2: kernel check red on a PEER's half-written `🌱️value/🔁️codec/🦀️.rs` (replication, 53 lexer errors, file rewritten
  05:04) → re-launched. Chain 3: **kernel native check green 05:12** (fleet-b, 2 m 35 s); filtered laws: 75/76 — the red one,
  `wire_fixtures_stay_byte_identical_across_rust_and_ts`, is LD's in-flight envelope wire change (`observed`/`target`), not mine.
- 05:17 chain 4 (laws narrowed to mine): **kernel check green 05:17; laws 9/9** — `document_link_shortage_tests` 6/6 (incl. the
  retry-at-bound / ceiling vectors), `document_echo_suppression_tests` 2/2, `backbone_parity_scenarios_match_neutral_fixture` 1/1
  (`land-chain-4.txt`).
- 05:17–05:38 hub check `semio-hub --bin os-hub --tests` sat 21 min with no rustc child in a build-fleet-b lock convoy (two other
  cargos idle 32–34 min holding ~700–800 unit locks: 93815, 95358 — not mine) → stopped mine (1823) per rule 25, told main.
  Sign-in follow-up chain (`wg9-signin-chain.sh`, waiting on the landing chain) stopped too (2298) to keep one cargo at a time.
- 05:37 kernel wasm32 hold queued early (`wg9-wasm-kernel-early.sh`, log `wasm-kernel-1.txt`; 2nd in the FIFO behind T13).
- 05:43–05:52 **wasm32 kernel hold: browser `--features sync --target wasm32-unknown-unknown` rc=0 (7 m 02 s), `wasm32-wasip2` rc=0
  (1 m 56 s)** (`wasm-kernel-1.txt`) — items 1 + 2 green on every kernel target. Landing rows added to `📓️landing.md`.
- 06:03–06:23 hub check (30-rustc gate, told main) stalled again in a fleet-b convoy (4 idle cargos, 0 rustc); coordinator routed the
  hub verification to H11: its current-tree `semio-hub --bins` check (build-landing) was **green 06:00** — covers my bootstrap hunk;
  its `--bins --tests` runs stalled/restarted (no result before 07:00). Stopped mine (40976). Per the coordinator's rule (verify or
  revert by 06:40): **06:32 reverted ONLY the bin-unit pins** (unverified test code) to the pre-landing lines; the verified bootstrap
  hunk stays. Re-apply after the rebuild: LAW_EDITS of `wp-wg9/s13-echo-suppression-hub-patch.py` (+ `--revert` option added).
- Deferred past 07:00 (not fitting the window): WG7's renderer-only `s12-hub-sign-in-off-interaction.py` (dry run clean 05:03;
  `wp-wg9/wg9-signin-chain.sh` = dry run → apply → renderer check → laws → renderer wasm32 hold) — native apply allowed after REBUILD
  START (renderer is not guest-linked), its browser build then needs a renderer release after PUBLISH DONE.
- S12-4 finding (design, not landed): the two unnamed `application` nodes are note's ink-canvas surfaces (`scene_surface` in the SDK
  builds a Surface with no accessible name); React's `SurfaceView` renders the same unnamed `role="application"` — parity gap on both
  renderers. The window kind already carries the localized name (`LocalizedLabel::native("Canvas", "Zeichenfläche")`); the fix is a
  host rule on both renderers: a Surface that is its window's body is named by that window kind's label (law over the shared
  accessibility projection + React twin), not a guest change.
- **14:0x resume (rule 31: 7800 READY on catalog B3, renderer `wasm-release` built by the chain 13:49).** Live verification wave:
  - frame-worker bundle was stale against the tree (`check-frame-worker` red: the link/echo TS twins in `🛍️products/💻️os/🟦️.ts`) →
    regenerated via the package script (+ browser-boot), `check-frame-worker` fresh (14:00).
  - catalog-exact modules (B3 `e3c0c98e…`): note materialized (`s13-wg9-b3-note`, served wasmSha256 `57728154…` == catalog), puzzle +
    draw in progress (`wp-wg9/wg9-b3-modules.sh`, log `b3-modules-1.txt`).
  - 7800 seed (user1 author, user2 member author): space **`01a0e2bd-225f-78d4-b150-63abd12c961f`** ("WG9 wasm32 collab s13"), note
    **`artifact-b4e94943e15d0987d01413c2b349016f`** (ready at once), 2d.puzzle creation running (`seed-puzzle.txt`).
  - serve **6552** = note variant on the B3 note root (`wp-wg9/wg9-serve.sh` → `serve/wg9-serve.ts`, variant-aware copy of WG7's;
    pid **7136**, private rendezvous `s13-wg9-bridge-6552`).
  - run `s13a` (A en-US / B de-DE, outage on) launched 14:05 (`wg9-collab-run.sh`, log `collab-s13a.txt`, captures `wp-wg9/generated/collab-s13a-*`).
- 14:02–14:16 **run `s13a` (7800 B3, note, A en-US / B de-DE, outage on): 11/23.** Green: painted ×2, hub sign-in ×2 (first attempt,
  B's chrome German: `Öffnen`, `Jemanden einladen`), local Add Text ×2, **15 s cut never freezes A (worker answers 6 / 2 ms)**, B's long
  cut expires and speaks German (`Die Verbindung war zu lange unterbrochen…`, 13 + 13d ✓ — the locale door is live). Red: neither
  browser ever reached the hub document socket — after `execution-target/manifest` 200 + `active-checkpoint/pair` 200 no `open-plan`,
  no socket, 0 document frames; the link expired 60 s after attach (`retired document-backbone owner after terminal fault`).
  **Root cause (measured): the served module root's `🧵️shard/🟨️shard-worker.js` dates from 2026-09-25 07:54 and has no `codec` case**
  — `wg7-catalog-module.ts` copies `🧵️shard` from the shared release root (`🔌️plugin/📦️packages/🟦️typescript/dist/release/🔌️plugin-modules`,
  last materialized 09-25), so the browser component codec (`codec.pack-schema-hash`, the identity the actor dials with,
  `document_pack_schema_hash`) failed and the browser actor refused to dial (silently: `schedule_reconnect` on every attempt). The
  component itself is fine: `wg9-codec-probe.mjs` (jco on V8, the served B3 note) answers `e415edbf…` == the catalog pin (wasmtime).
  Fix for the ticket serve: `wp-wg9/wg9-module-glue.ts` writes the CURRENT `shardWorkerSource()` (with `case "codec"`) into all three
  roots (14:59). **Product finding (routed to main):** every wgpu RELEASE serve that mounts the shared release module root has the same
  stale shard worker → cannot dial any hub document until that root is re-materialized.
- 14:56 landing window 2: sign-in-off-interaction patch — dry run clean on the 14:56 tree, **applied 14:56** (`wg9-signin-chain.sh`, log
  `signin-chain-2.txt`): renderer wasm32 check holding the wasm mutex; native check + laws queued in the `native` lane (4 ahead).
- 14:59 run `s13b` (same doc, shard refreshed) launched (`collab-s13b.txt`).
- 14:58–15:07 **run `s13b` (same note, shard refreshed): 18/23 — the wasm32 shell collaborates on 7800 B3.** Attach ✓ A (`Persisted`)
  / B (`Gespeichert` — the step-3 red is the probe's regex lacking the German label, fixed), roster `User One · User Two` ✓, **A→B edit
  seen in 1.3 s, B→A in 4.6 s** ✓, frames name the document ✓×2 (A 154 sent / 35 received, B 135 / 33), 15 s cut never freezes A
  (1–2 ms) ✓, offline edit admitted and shown `Pending (1)` ✓, B's long cut expires and speaks German ✓×2. Before/after vs `s13a`
  (11/23, 0 document frames): the refreshed shard worker is the whole difference for attach/presence/edits.
  **Red 12/12b/12c — new kernel defect, decoded on the wire** (`generated/collab-s13b-frames-A.json`): A's edit `edit-69c5…#0` was sent
  +282.1 s over the still-open socket (Playwright's offline switch does not close an open WebSocket) and COMMITTED (hub head 3, B shows
  it), but its `Ack` died with the socket. On relink (+295.8 s: `open-plan` → `socket-grants` → ws) the Welcome tail carried that op;
  the actor had re-queued it from `pending_batches` and RESENT it re-stamped (HLC logical 2 → 3) → hub `Rejected: conflict: replayed
  operation` → rollback of the whole batch → `rollback_envelope` of the batch's history transition (`transition-106b…`, inverse 0 bytes
  by design) produced an empty `semio.history.transition` payload → guest `module.vcs` "deserialize error: truncated at offset 0" →
  the shell retired the document (terminal fault). Same code in the native actor. Fix prepared (window 2 has no room for its native +
  wasm32 checks): `wp-wg9/s13-committed-settlement-patch.py` — `settle_committed_envelopes` (a tail/relay op still in the outbox or a
  pending batch is committed → leaves both, retention released, never resent) in both `Commands` arms; `rollback_envelope` → `None` for
  a history transition (all 5 rollback loops `extend`); laws `a_committed_own_operation_settles_the_outbox_and_its_pending_batch`,
  `both_actors_settle_committed_operations_before_admission`, `a_history_transition_has_no_inverse_rollback`. Dry run clean (3 files,
  5 loops, `generated/settlement-dryrun.diff`). Reported to main 15:1x.
  Red 14 (B's card empty after the expiry, sync `Remote: getrennt` = never relinked): the card was closed when read — probe/observation,
  the never-relinks fact holds.
- 15:12 run `s13c` (late joiner: two fresh browsers on the note with hub head ≥ 3; no outage) launched (`collab-s13c.txt`).
- 15:12–15:19 **run `s13c` (late joiner): B — a fresh de-DE browser attaching the note that already holds 3 committed blocks — shows
  all 3 right after attach (`4b B … blocks 3 = hub head 3` ✓, `Gespeichert`) — the echo-suppression fix works in the wasm32 shell**
  (s12i-reopen showed 0 of 3). A's attach did not start: the Sync card's `Remote` choice appeared after the probe's 10 s budget under
  load (the 120 s projection has it) → A-dependent steps red (10/15). Probe budgets widened (45 s / 30 s).
- 15:18:52 **kernel lost-Ack fix APPLIED** (coordinator rule 34 exception): `wp-wg9/s13-committed-settlement-patch.py --apply` (4 files:
  kernel sync, echo law, unit law line, parity fixture + scenario `lost-ack-committed-op-settles`; dry run clean ×3; rustfmt parse ✓;
  Ajv admits the parity fixture, 8 scenarios, `wp-wg9/wg9-parity-ajv.ts`). Native filtered law run + wasm32 kernel checks queued
  (`settlement-checks-1.txt`); landing row + `wp-w3/requests/wg9.txt` written; LD told the hunks (LD does not write the file).
- 15:2x LD's finding for the Rust actors (not mine to land now, recorded): both `on_hub_frame` Welcome None/Tail arms refuse while
  `artifact_rebootstrap_required` ("returned tail without a canonical pair"); the hub serves pairs only via `GET …/active-checkpoint/pair`,
  so every `RebootstrapRequired` leaves a native/wgpu document refused + reconnecting forever. React's worker accepts the tail for an
  actor-bound rebuild and re-seeds from the pair route after `Session`; the Rust twin needs the shell to re-seed the guest on a rebuild
  (`📓️wp-ld.md` 15:0x/15:2x) → WG9/WG10 follow-up after the chain.
- 15:2x serve **6553** = puzzle2d variant on the B3 puzzle root (pid **46265**; ticket runtime root `s13-wg9-runtime-puzzle2d` — the
  variant has no Nx release activation); probe gained `WG7_MODE=cursors` (board pixel diff while the peer's pointer moves, control pass
  for noise); run `s13d` (puzzle `artifact-a2ba205a8076d987a9c0acc3f46b7a34`) launched.
- 15:22–15:35 **run `s13d` (puzzle2d board, `WG7_MODE=cursors`): 11/17.** Painted ×2, sign-in ×2, attach ✓ A `Persisted` / B
  `Gespeichert`, roster `User One · User Two` ✓, frames name the document ×2 (A 300 sent, B 459). **Peer cursor ✗ both ways**: B's board
  region identical while A's pointer crossed A's board (noise 0, deltas 0/0/0; screenshot `generated/collab-s13d-cursor-*.png`).
  Decoded A's 293 presence heartbeats (`decodePresencePeer`): every one carries both board views (`2d-overview`, `2d-detail`: camera,
  size) and **none a pointer**. Root cause (renderer, target-neutral): `🧊️renderer/🦀️.rs` `handle_pointer_move` calls
  `shell.handle_pointer_move_for` — the only writer of `presence_pointer` — only when no published scene surface claims the move, so
  the pointer froze exactly when it entered a board. WG8's Shell law set `presence_pointer` directly and never exercised the move path.
  Fix prepared: `wp-wg9/s13-board-presence-pointer-patch.py` (renderer notes `presence_pointer` for every move before routing; law
  `every_pointer_move_is_the_presence_pointer_even_over_a_painted_board`), dry run clean 15:36 — lands in the next window (renderer
  only). Steps 5–8 (edits) are note-specific in the probe (no puzzle edit control addressed) — not a puzzle defect.
- 15:31–15:32 **settlement fix wasm32 green**: kernel browser `--features sync --target wasm32-unknown-unknown` rc=0 (2 m 14 s),
  `wasm32-wasip2` rc=0 (1 m 13 s) (`settlement-checks-1.txt`); native law run still queued in the native lane.
- 15:26 sign-in patch native renderer check red on a PEER break (`🔌️plugin/🖥️host/🦀️.rs:2670` E0061, H11's in-flight edit);
  coordinator 15:40: KEEP applied, re-run after H11's fix → re-queued (`wg9-renderer-native.sh`, log `signin-native-2.txt`).
- 15:40 agent probe `s13a` (de, human user2, wasm32 note on 7800 B3, delegated agent over the stdio semio MCP) launched
  (`wg9-agent-probe.ts`, log `agent-s13a-de.txt`).
- 15:40–15:54 **agent probes** (`wg9-agent-probe.ts`, delegated agent over the stdio semio MCP, human in the wasm32 shell on 7800 B3):
  `s13a` de 6/8 (agent own principal ✓, `addBlock` SUCCEEDED ✓, head 5→6 ✓; roster ✗, block ✗), `s13b` en 5/8 (the human's attach
  never started — probe flow flake, 0 document frames), **`s13c` de 7/8: the wgpu roster shows the agent AS an agent in German —
  `WG9 agent de (KI-Agent) · User Two` ✓**, commit ✓ (head 7→9), block ✗ (5→5). Frame capture added to the probe; decoded: the hub
  RELAYED the agent's op `edit-a4f7f8a2c62bb816` to the human (+105.2 s) — but its block ids (`note-text-28c048233e148b27`,
  `text-240b64443a171215-0`) are byte-identical to the previous agent session's op `edit-958c9bbd22f09618` (s13a), so it adds nothing
  new: **the semio-MCP gateway's note instance mints the same block ids in every fresh agent session** (deterministic seed) — a
  collision the hub accepts as a distinct op. Routed to main/G11 (MCP id seeding). 1.4 s after the relay the human's store authored an
  auto checkpoint `Commit` (tag 2, "auto", User Two) of the prior agent op `edit-958c…` — expected auto-commit behaviour.
  Also visible (screenshot `agent-s13a-de.png`): Artifact rows paint `Texttext` (two labels overlap) and the Sync card rows still
  overlap their icons/uri (known since S12, renderer layout).
- 15:56 fresh note **`artifact-0d6bf21e83b1230dee6b30604a7dc4c2`** (no prior agent block) → agent probe `s13d` (de) to prove the live
  agent-block leg without a collision.
- 15:56–16:00 **agent probes on fresh notes: `s13d` de 8/8, `s13e` en 8/8** — human (wasm32, user2 de / user1 en) holds the note
  `Gespeichert`/`Persisted`; roster shows the delegated agent AS an agent in the human's tongue (`KI-Agent` / `AI agent`); the agent's
  `addBlock` (stdio semio MCP, own principal `agent:01a0e328…` / `agent:01a0e32a…`) lands in the human's Artifact panel **0→1 in 3.0 s**
  without a reload; hub head 0→1. Docs `artifact-0d6bf21e83b1230dee6b30604a7dc4c2`, `artifact-e22571c92ee7284684321fe13cc1c7b4`.
