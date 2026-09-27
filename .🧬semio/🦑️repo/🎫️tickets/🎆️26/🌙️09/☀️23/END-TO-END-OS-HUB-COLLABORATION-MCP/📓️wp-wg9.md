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
| 3a | S12-1 open items (de chrome / locale door, sign-in freeze patch) | pending |
| 3b | S12-2 peer cursors on a board-canvas kind | pending |
| 3c | S12-3 the human sees the agent's block | pending (needs item 2 live) |
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
