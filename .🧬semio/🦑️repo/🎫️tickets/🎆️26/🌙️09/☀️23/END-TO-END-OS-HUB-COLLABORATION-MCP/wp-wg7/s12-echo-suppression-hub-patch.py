#!/usr/bin/env python3
"""🔁️ WG7 session 12 — late joiners see no history: the hub half (coordinator decision 17:1x, H9 informed before touching the file).

The hub's hello catch-up tail carries a DECLARED hub origin (`HUB_CATCH_UP_ORIGIN`, pinned to the fixture's `hubCatchUpOrigin`),
never the receiving socket's actor: until now `state.db.hello(…, actor.clone(), …)` stamped the joiner's own actor on the tail, which
every replica read as its own echo (ticket 26/09/23 session 12, run s12i). One hunk in `🌎️hub/🏗️bootstrap/🦀️.rs` (the hello call and
its constant) plus the existing joiner/reconnect catch-up law in `🧪️tests/🔬️bin-unit/🦀️.rs`, which now also pins the origin.

Apply in the landing window (hub rebuild + restart needed; rule 24 forbids builds now). Dry run by default; `--apply` writes.
"""

import difflib
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
BOOTSTRAP = ROOT / "🌎️hub/🏗️bootstrap/🦀️.rs"
LAWS = ROOT / "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"

BOOTSTRAP_EDITS = [
    (
        "    let mut hello_session = match state.db.hello(db_id.clone(), frontier, session_id, actor.clone(), 64 * 1024).await {",
        "    let mut hello_session = match state.db.hello(db_id.clone(), frontier, session_id, ActorId(HUB_CATCH_UP_ORIGIN.into()), 64 * 1024).await {",
    ),
    (
        "/// @emoji 📨️ Submits one batch and returns its `Ack` plus the `Commands` relay for peers.",
        """/// @emoji 🔁️ The origin this hub stamps on a hello catch-up tail: a declared hub identity, never the receiving socket's actor — a
/// tail carries anyone's edits, and a replica that read its own actor there discarded the whole history as its own echo (ticket
/// 26/09/23 session 12, run s12i). Replicas suppress echoes by operation identity; this names the tail. Pinned to
/// `🏪️store/🧫️fixtures/document-echo-suppression-v1` `hubCatchUpOrigin`.
const HUB_CATCH_UP_ORIGIN: &str = "hub.catch-up";

/// @emoji 📨️ Submits one batch and returns its `Ack` plus the `Commands` relay for peers.""",
    ),
]

LAW_EDITS = [
    (
        """        assert!(matches!(next_server_frame(&mut resend).await, ServerFrame::Commands { envelopes, .. } if envelopes[0].mutation_id == committed.mutation_id), "the reconnect is caught up with the committed edit");""",
        """        assert!(matches!(next_server_frame(&mut resend).await, ServerFrame::Commands { envelopes, origin, .. } if envelopes[0].mutation_id == committed.mutation_id && origin.0 == HUB_CATCH_UP_ORIGIN), "the reconnect is caught up with the committed edit, under the hub's declared catch-up origin");""",
    ),
    (
        """        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Commands { .. }), "a joiner behind the head is caught up before Session");""",
        """        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Commands { origin, .. } if origin.0 == HUB_CATCH_UP_ORIGIN && origin.0 != legacy_receipt.actor_id), "a joiner behind the head is caught up before Session, never under its own actor");
        let echo_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/document-echo-suppression-v1/🔣️.json")).expect("echo suppression fixture");
        assert_eq!(echo_fixture["hubCatchUpOrigin"].as_str(), Some(HUB_CATCH_UP_ORIGIN), "the hub's catch-up origin is the fixture's declared one");""",
    ),
]


def replaced(path, source, edits):
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.name}: {old[:90]!r}")
        source = source.replace(old, new)
    return source


def main():
    apply = "--apply" in sys.argv
    for path, edits in ((BOOTSTRAP, BOOTSTRAP_EDITS), (LAWS, LAW_EDITS)):
        before = path.read_text(encoding="utf-8")
        after = replaced(path, before, edits)
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), path.name, path.name + " (patched)", n=1))
        if apply:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'APPLIED' if apply else 'DRY RUN'}: 2 files")


if __name__ == "__main__":
    main()
