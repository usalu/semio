"""🧾️ C12 14c: row 17 (catching-up status set for T4) + the handover state of the queued proofs (idempotent)."""
p = "/Users/ueli/Documents/semio/.tmp-ticket/📓️wp-c12.md"
s = open(p, encoding="utf-8").read()
anchor = "**Session 14c log**"
rows = """| 17 | "Catching up" execution-target status (coordinator: T4 set) | PREPARED `wp-c12/catchup-status/c12-catchup-status-patch.py` (live dry run **15 planned / 0 problems**): vocabulary fixture (`🔏️document-execution-target-lease-v1` `expected.status`/`statusRoles` + `catching-up`, en "Catching up with the hub…" / de "Gleiche mit dem Hub ab…", role `status`), TS twin (code, text, role, progress stage `catch-up` = tail messages delivered/retained), Rust twin (the enum had drifted to 5 of 8 codes → all 9 with texts + roles) + NEW Rust law replaying the corpus, hub oracle (inventory + role rule), worker `flushPendingBackbone` reports `catching-up` progress per delivered tail message; the notice's Cancel closes the document (`docAbort` → `cancelled`); the mounted surface clears it. Overlay proof (`s14-c12-overlay`, with c12-splice applied too): tsc os **0**, worker + os root laws **382/382** (incl. the TS vocabulary law replaying the corpus), hub `execution-target-lease-check` **status=9 passed** (captures `s14-c12-captures/{tsc-overlay-catchup-1,vitest-overlay-catchup-1,lease-oracle-overlay-1}.txt`). Rust law **written, not run** (os kernel lib tests; L1's T4 native) |
| 18 | Handover (slot freed 05:2x) | still queued, detached: `wp-c12/c12-idle-sequence.sh` (pid 76394) waits for idle native/wasm/overlay lanes, then the rule-20 boot to Home (`boot-14c-3.txt`) and the c12-splice Rust scratch proof (ui check, ui-scene + writer `--lib` tests, os tsc; `splice-scratch-3.txt`); progress in `.🧬semio/🌐hub/s14-c12-captures/sequence.txt`. Verification list after the chain (coordinator): collab en/de on 7800/p24, STEP 15 cuts 15/60 s (recovery), dd1/dd2 reload probes, late join; the overlay `s14-c12-overlay` (+ `.c12-build`/`.c12-target` once the proof ran) is C12's to delete after L1 lands the splice set |

"""
if "| 17 | \"Catching up\" execution-target status" not in s:
    assert s.count(anchor) == 1
    s = s.replace(anchor, rows + anchor)
    open(p, "w", encoding="utf-8").write(s)
print("ok")
