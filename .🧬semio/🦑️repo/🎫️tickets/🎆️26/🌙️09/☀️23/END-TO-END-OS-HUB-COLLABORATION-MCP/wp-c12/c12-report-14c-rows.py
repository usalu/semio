"""🧾️ C12 14c: appends the SeedHistory + 8010 rows to the Session 14c table of 📓️wp-c12.md (idempotent)."""
p = "/Users/ueli/Documents/semio/.tmp-ticket/📓️wp-c12.md"
s = open(p, encoding="utf-8").read()
anchor = "**Session 14c log**"
rows = """| 6 | STEP 8 root cause (H13 capture + code) → prepared set for L1's T3 | a replica folds every hub-tail operation as a remote edit NAMED AFTER ITS MUTATION ID (`store::edit_from_operation_envelope`); writer's store initializer (SeedHistory) seeds the entry id AND each forward's id → the same id twice → `duplicate mutation id`, document unloadable. Same copy in 7 more plugin initializers (gismap, jack wire-runtime, draw owned, raster, generation2d, generation3d, process3d) + the SDK bounded initializer; only the framework's two hydrations had the guard. Set `wp-c12/seed/c12-seed-history-patch.py` (**dry run 15 planned / 0 problems**, 22:49): ONE rule `ArtifactStoreInitializationRuntime::seed_edit_operation(entry_id, id)` (an operation named like its own entry is that node; cross-entry repeats still refuse) used by all 9 initializers + both hydrations + the composition test; law `a_document_folded_from_the_hub_tail_initializes_again` replays H13's 17-envelope capture (fixture `✳️any/🧫️fixtures/🔁️hub-tail-after-check-in/🔣️.json`, wire fields verbatim; the first Commit has no parent — decoded) → fold → pack → writer initializer → candidate == fold. **Written, not run** (cannot pass without the fix; L1's lanes prove it). The fixture directory is new → R10 taxonomy check |
| 7 | 8010 (H13, declared-batch fix) runs | `c12collab-8010-en` 22:19: **7/15** (1,2,3,4,5,7,12). STEP 6: after the STEP 6 hard reload user1's document never reached the hub (`execution-target/browser-actor` request ERR_ABORTED on the reload, "Check In (1)" stays pending, no check-in status) — same class as STEP 8 (the reload's browser-actor child never came up); STEP 15: the 5 s cut passed; in the 15 s cut user2's browser actor child faulted on a resumed hub frame (`[backbone-worker] malformed hub frame … invocation rejected: invoke reactor/poll: Error: [object Object] (see error.payload)` — the guest error payload is lost in `childRejectionText`, follow-up) and user2's editor stayed empty, so the keystrokes never showed there. The machine rebooted 22:42 (every process died) during `c12collab-8010-de`; re-runs from 22:51 |

"""
if "| 6 | STEP 8 root cause" not in s:
    assert s.count(anchor) == 1
    s = s.replace(anchor, rows + anchor)
    open(p, "w", encoding="utf-8").write(s)
print("ok")
