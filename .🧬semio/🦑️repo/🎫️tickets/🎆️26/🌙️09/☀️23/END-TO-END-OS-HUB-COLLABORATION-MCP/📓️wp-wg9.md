# WP-WG9 — wgpu Shell (wasm32 Browser) Collaborates Over the Hub, Session 13

Slice WG9 (session 13, 2026-09-26 19:0x), successor of WG7 (handover [`📓️wp-wg7.md`](📓️wp-wg7.md)). Coordinator = main chat.
Ports: hubs 8050–8059 (inherits 8050: hold 78966 → os-hub 79010, data `.🧬semio/🌐hub/s11-wg7-hub-8050`), serves 6550–6559 (inherits
6552 pid 21493, 6553 pid 50393, `wp-wg7/serve/wg7-serve.ts`). Private cargo target: `.tmp-ticket/wp-wg9/target`. Captures:
`wp-wg9/generated/` (expendable). Durable logs: `.🧬semio/🌐hub/s13-wg9-logs/`. Landing rows: `📓️landing.md` § Session 13 Landing
Window. Guest requests: `wp-w3/requests/wg9.txt`.

## Session 13

| # | Item | Status |
|---|------|--------|
| 1 | Land `wp-wg7/s12-link-expiry-patch.py` (cuts > ~31.5 s expired the link) | dry run clean on the current tree (19:1x) |
| 2 | Late joiners: lanes dedupe by operation id + hub catch-up tail carries a declared origin (kernel native + wasm32, React twin, hub) | dry runs clean (kernel + hub) (19:1x); React lane landed in S12 |
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
