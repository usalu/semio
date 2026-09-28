# -*- coding: utf-8 -*-
"""📝️ S18 §14c: landing row for the document actor recovery re-open (idempotent)."""
import pathlib

LANDING = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️landing.md")
ROW = "| S18 | 14c — document actor RECOVERY re-open (design approved 00:0x): policy `🏪️store/👷️worker/🚑️actor-recovery/{🔣️.json,🟦️.ts}` (NEW dir → R10 taxonomy) — a lost actor reopens at once, an applied action or a 30 s stable mount starts a fresh count, > 3 consecutive losses end with the typed fault `recovery-exhausted`; worker `requestDocumentActorRecoveryV1` (in-flight batches back to the outbox front in batch order, child retired as `actor-lost`, socket closed with the NEW row `actorLost`, reconnect at once; exhausted → `artifact-bootstrap-failed` code `recovery-exhausted`, no further actor), wired to the unconfirmed action invocation (C12 wires the inbound-frame child fault); mailbox `action-state-unconfirmed` → NEW refusal `not-applied` (input ledger en/de, retryable, never replayed); Shell notices `reopen` / `exhausted` en/de (host-bootstrap fixture + ⚡️quick law); schema-first: renderer schema + worker response union gain `recovery-exhausted`. Laws: `💻️os/🧪️tests/🚑️actor-recovery` (fixture cases + cockatiel `ConsecutiveBreaker` oracle over 2 040 sequences), worker-harness law in `🧪️tests/🧪️space-artifact-creation-owner`. Files: `👷️worker/🟦️.ts`, `📇️directory/🔌️client/🚪️socket-close/🔣️.json`, `🎯️action-handoff/📮️requests/🟦️.ts`, `🎯️input-ledger/🟦️.ts`, `🪪️host-bootstrap/🟦️.tsx` + fixture, `📺️renderer/🧬️schema/{🔣️.json,🟦️.ts}`, `💻️os/🟦️.ts`, `🧪️tests/🎚️config/🟦️.ts`, `🧪️backbone-envelope-io` (mapping law); codemods `wp-s18/s18-14c-actor-recovery{,-law}.py` | TS/JSON only (host) | tsc 0 (`wp-s18/generated/s18-14c-tsc-recovery-1.txt`), boot green (`s18-14c-beacon-5.txt`); laws queued in the native lane (`s18-14c-law-recovery-1.txt`); live trigger pending (no on-demand unconfirmed invocation since the refusal fix) | 29 00:2x |"


def main() -> None:
    text = LANDING.read_text(encoding="utf-8")
    if ROW[:70] not in text:
        LANDING.write_text(text.rstrip("\n") + "\n" + ROW + "\n", encoding="utf-8")
        print("added 1")
    else:
        print("added 0")


main()
