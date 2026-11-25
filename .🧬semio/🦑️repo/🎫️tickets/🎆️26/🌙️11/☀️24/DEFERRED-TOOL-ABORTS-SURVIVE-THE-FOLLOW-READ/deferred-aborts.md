# Deferred Tool Aborts Survive the Follow Read

Follow-up of `26/11/22/GET-FEM-PLUGIN-WORKING-END-TO-END`. That ticket made every writer that waits on the tool follow's
retained read drive the follow (`follow_before_writer`). Host aborts did not wait: they were silently dropped.

## Problem

A tool follow (`🔌️plugin/🛠️tool-machine/🔁️follow`) lends a press, gesture or typing slot to a retained read
(`RetainedOriginalSlot::has_read`). While the read was held:

- `GestureLedger::abort` returned `Idle`, so `end_window_gesture` lost a blur, capture loss, utility switch or closing
  window, and `end_every_gesture` lost a freeze or a moved base;
- `ScrubLedger::abort`/`abort_all` and `TypingLedger::abort`/`abort_all` returned `Idle`, so `freeze_tool_machines`
  lost the freeze of the read rows; the scrub and typing ledgers also dropped aborts while their original writer
  (`writing`) held the row.

The ledger could not simply write instead: the follow pins each ledger's `source_revision`, and a revision change under
a held read is refused (`original tool state read requires its unchanged source frontier`).

## Design: deferred abort, applied once the read returns

Each ledger records the abort fact while the slot is lent and applies it once the slot is free. No contract changes:
`ToolStep`, `GestureState` and the gesture-drive law are unchanged, and the deferred abort still answers `Idle`.

| Ledger | Deferred fact | Effect while lent (no revision change) |
| --- | --- | --- |
| `GestureLedger` | `deferred: BTreeMap<window, ToolAbortReason>` | gesture hidden from `open`/`windows`; its press is closed now, so a late release is dropped |
| `ScrubLedger` | `ScrubWindow.deferred: Option<ToolAbortReason>` | scrub hidden from `open`/`windows`; a named cancel closes its press now (not while the original writer holds the row) |
| `TypingLedger` | `TypingWindow.deferred: Option<ToolAbortReason>` | run hidden from `open`/`windows`, so `commit_all`/`retain_windows` skip it |

The first fact wins. `provisional` and `is_empty` stay physical: the overlay keeps the slot until the abort applies,
and the refold that follows removes it.

Each ledger applies the abort through `settle_deferred_aborts()`, which returns the `Aborted` steps with the deferred
reason. Before any write, every writer of the slot applies it too: gesture `drive`/`settle`, scrub `send`, typing
`send`/`commit`/`lapse`, and the original scrub writer and typing edit admissions. Writers that run later find the
slot already ended: a late press tick is silent, and a typing commit publishes nothing.

In the render context (`🧵️instance/🖼️render/📥️staging/🛠️tool-machine`):

- `settle_deferred_tool_aborts()` settles all three ledgers. When a gesture ended it sets `gesture_ended`, so the
  windows repaint.
- It runs after every follow step that may return a read: the advance and close step, and the finished follow's take.
  When anything ended, it requests a refold.
- `follow_tool_machines` settles first, so every writer completion settles too.
- `settle_gesture_slot` settles before it compares the decision with the open gesture, so a decision made before the
  blur never resurrects the gesture.
- `end_window_gesture`, `end_every_gesture` and `freeze_tool_machines` request a refold when an abort was deferred. The
  follow becomes non-current and closes, which returns the read and lets the abort apply.

A lapse during a typing read is not lost. The row is skipped while lent and lapses on the next `lapse` call, which also
reports any deferred aborts it settles.

## Laws

- Ledger law, data-driven: `🔗️read/🧪️tests/⏸️deferred/🔣️.json` with schema `📐️.json`, run by
  `every_host_abort_during_a_retained_read_ends_its_slot_once_the_read_returns`. It holds a real retained read and covers
  14 cases across gesture, scrub and typing. For each case it checks two outcomes: while the read is held (no revision
  move, slot ended for readers, press closed) and after it returns (`Aborted` with the first reason, zero trace).
  - Writers after the return: they find the slot already ended.
  - Named press mismatch: the open press is kept.
  - Retired window: its press is forgotten.
  - Lapse: it waits on the read and then fires.
- Plugin source law next to `every_writer_waiting_on_a_follow_read_drives_the_follow`:
  `a_host_abort_during_a_follow_read_ends_its_slot_once_the_follow_returns_the_read`. It anchors:
  - the three-ledger settlement and the repaint;
  - settle-before-refold;
  - the settlement after the follow step and after the take;
  - the deferred blur, deferred instance-wide fact and freeze refold requests;
  - settle-before-judging in `settle_gesture_slot`.

There is no third-party oracle: retained reads are a native-only Rust concept, and neither the TypeScript ledgers nor
any external library model them. No JSON-schema validator is a workspace dependency, so the schema documents the
fixture but is not machine-checked in the test.

## Verification

See `verification.md`.
