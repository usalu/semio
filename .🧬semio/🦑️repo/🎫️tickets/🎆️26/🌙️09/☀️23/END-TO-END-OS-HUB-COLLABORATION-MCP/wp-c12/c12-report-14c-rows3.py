"""🧾️ C12 14c (after the usage cut, panics and the 01:14 sweep): appends rows 11–14 to the Session 14c table of 📓️wp-c12.md
(idempotent)."""
p = "/Users/ueli/Documents/semio/.tmp-ticket/📓️wp-c12.md"
s = open(p, encoding="utf-8").read()
anchor = "**Session 14c log**"
rows = """| 11 | Reload path (priority 1) root cause | debug-gated harness run `c12collab-8010-dbg2` (23:56, temporary `[DEBUG] c12 gate` — removed, 0 left) + probe rounds with per-user data dirs (`c12reload-dd1/dd2`, serves via `ensureDevServe` + `S_DATA_DIR`): after a reload the worker DOES relay (pair/pack/reservation ready, batches acked). STEP 6's Check In PASSES since S18's fix; its remaining red is a harness defect (the "updated" cell has minute resolution — creation and check-in in one minute look unchanged). STEP 8's red = reload after a Check In → "Document restore failed … initializer-failed … duplicate mutation id" (dd2 round 5) = the SeedHistory bug → fix + law + amendment LANDED in T3 (L1). Second defect: a reloaded editor can mount before catch-up and the first whole-text SET wipes earlier text (dd1 round 2: `m0` lost for both) → removed by the splice set (item 2); host should also hold typing until caught up (follow-up) |
| 12 | Actor child faults on an inbound frame (priority 2, one design with S18) | worker Commands branch calls S18's `requestDocumentActorRecoveryV1(state, "inbound-frame")` instead of rethrowing into the malformed-frame bootstrap rejection (`wp-c12/c12-inbound-recovery-edit.py`, landed 00:12, tsc os 0 `tsc-os-14c-4`); law "recovers a child that refuses an inbound hub frame" (actor-lost, frontier kept, in-flight back at the front, no rebuild); worker laws **137/137** (`.🧬semio/🌐hub/s14-c12-captures/vitest-worker-14c-5.txt`, 05:06). Child payload diagnostic set landed (L1). Live 15 s / 60 s cut re-run waits for the chain (hubs down). HEAD carries S18's recovery describe block twice (double apply) → S18 |
| 13 | c12-splice revert (L1 T3, E0433) → repaired | cause: the editor hunks used `text_splice::` without importing it → import hunk added; the command law `✂️text-splice/🧪️tests/🔬️unit/🦀️.rs` (deleted from the payload by auto-commit `dfe2687`) restored byte-exact from `5bcb2da` (matches the current command + Rust `from_edit`). Live dry run **113 planned / 0 problems**; overlay `.🧬semio/🌐hub/s14-c12-overlay` (T14 `overlay.py create`, 30 s): set applied, TS twin laws **41/41**; Rust scratch proof (`splice/scratch-proof.sh`: ui check, ui-scene + writer `--lib` tests, os tsc; private build-dir `.c12-build` seeded with registry units) queued, gated on idle native + wasm lanes (rule 25b) → capture `.🧬semio/🌐hub/s14-c12-captures/splice-scratch-2.txt`; first attempt stopped at once (would have run beside L1 native + ST2 wasm) |
| 14 | Captures | since the 01:14 sweep every C12 capture lives in `.🧬semio/🌐hub/s14-c12-captures/` (rule 26); `wp-c12/generated/*` of 14c are gone (sweep) — the numbers above were recorded before it |

"""
if "| 11 | Reload path (priority 1) root cause" not in s:
    assert s.count(anchor) == 1
    s = s.replace(anchor, rows + anchor)
    open(p, "w", encoding="utf-8").write(s)
print("ok")
