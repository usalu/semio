"""🧾️ C12 14c: rows 15–16 (catch-up gate, STEP 6 harness minute) + landing rows (idempotent)."""
p = "/Users/ueli/Documents/semio/.tmp-ticket/📓️wp-c12.md"
s = open(p, encoding="utf-8").read()
anchor = "**Session 14c log**"
rows = """| 15 | Catch-up gate (coordinator): no input applies to a document that has not caught up | LANDED host TS (`wp-c12/c12-catchup-gate-edit.py`): the actor's action gate splits identity mismatches (`action-owner-mismatch`) from catch-up (`documentCatchingUpV1`: unbound actor, retained tail not drained, bootstrap/rebuild in flight, `Welcome` tail frontier not reached, cold pair not applied/transferring) → typed `action-catching-up` → ledger reason `catching-up` (retryable, notified; en "Not applied — the document is still catching up with the hub. Try again in a moment." / de "Nicht angewendet — das Dokument gleicht sich noch mit dem Hub ab. Bitte gleich erneut versuchen."), mailbox + shell map it; the TextEditor host drops the refused keystroke and resyncs to the guest's text (never applied to stale state). Laws: predicate (each fact alone gates) + mailbox mapping. tsc os **0** (`s14-c12-captures/tsc-os-14c-6.txt`); worker laws **137/137**, os root laws **245/245**, input-ledger + text-input laws **75/75** (captures `vitest-worker-14c-6`, `vitest-os-root-14c-1`, `vitest-input-ledger-14c-1`); rule-20 boot + splice scratch proof chained behind idle lanes (`c12-idle-sequence.sh`, `s14-c12-captures/sequence.txt`). OPEN: (a) a visible catching-up indicator with progress + cancel needs a new execution-target status code whose vocabulary has Rust twins (sync crate, wgpu shell, fixtures) → a train set; (b) the stale-host-view data loss (dd1: an editor painted before the tail, first whole-text SET wiped it) is not proven gone by this gate — it needs a live re-run after the chain (the splice set removes the loss itself) |
| 16 | STEP 6 harness minute resolution | `wp-c12/c12-harness-step6-minute-edit.py`: each user's row must show the check-in's minute or later (`collabRowUpdatedMinute`, en + de formats, checked on real rows), instead of "row text changed"; tsc os 0 (`tsc-os-14c-5.txt`) |

"""
if "| 15 | Catch-up gate" not in s:
    assert s.count(anchor) == 1
    s = s.replace(anchor, rows + anchor)
    open(p, "w", encoding="utf-8").write(s)
p2 = "/Users/ueli/Documents/semio/.tmp-ticket/📓️landing.md"
s2 = open(p2, encoding="utf-8").read()
land = [
    "| C12 | 14c — typed catch-up refusal: actor gate splits `action-catching-up` (`documentCatchingUpV1`) from `action-owner-mismatch`; ledger reason `catching-up` (en/de, retryable, notified); mailbox + ShellHost mapping; laws: `🏪️store/👷️worker/🟦️.ts`, `🏛️ShellHost/🎯️input-ledger/🟦️.ts`, `🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests/🟦️.ts`, `🏛️ShellHost/🟦️.tsx`, `🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts`, `🧪️tests/🧪️backbone-envelope-io/🟦️.ts` (`wp-c12/c12-catchup-gate-edit.py`) | host TS; tsc os 0 (`.🧬semio/🌐hub/s14-c12-captures/tsc-os-14c-6.txt`) | worker 137/137, os root 245/245, input-ledger + text-input 75/75; boot chained behind idle lanes | 05:1x |\n",
    "| C12 | 14c — collab-e2e STEP 6 judges the table's updated column at minute resolution (`collabRowUpdatedMinute`, `collabAwaitRowUpdatedMinute`): `🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` (`wp-c12/c12-harness-step6-minute-edit.py`) | tsc os 0 (`tsc-os-14c-5.txt`) | live after the chain | 05:0x |\n",
]
for row in land:
    if row[:60] not in s2:
        s2 = s2.rstrip("\n") + "\n" + row
open(p2, "w", encoding="utf-8").write(s2)
print("ok")
