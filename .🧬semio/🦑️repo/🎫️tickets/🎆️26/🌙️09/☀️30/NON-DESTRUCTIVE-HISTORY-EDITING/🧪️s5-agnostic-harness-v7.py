#!/usr/bin/env python3
"""🪛️ S5-AGNOSTIC harness v7 (found by the graphs family, 19:03 — dag, sequence, wires and flow all: "the seed does not land: the
instance never settled within 60 s … pending typed operations: true"; cad: every case "the seeded document does not survive save and
load before any edit … closure-rejected (Incomplete)"):
1. A plugin ACTION (a child law's seed gesture, an example loaded through `setActiveExample`) is settled with the framework's own
   fixture protocol `artifact_app_laws::settle_registered_typed_operation` (maintenance grant, result pages with their exact ACK,
   effects, events, composed results, completions) — the harness pump only advanced the publication and took UI progress, so a
   composed-lane result page stayed un-ACKed forever. The time-travel pump is unchanged (its sessions pass on five crates).
2. A case on the app's OWN initial document is seeded on the instance as it boots — the base text is loaded only when it differs from
   the instance's document text. Loading a composed parent from its text drops its owned members (design §20.15, report S3.9), so every
   cad case failed its control reload; a document the instance already holds needs no load.
Anchored on the exact post-v6 text, count-asserted, one write; idempotent. Usage: [--apply] [--preview <file>]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"

EDITS = [
    ("""        let mut files = app.document_text().await.map_err(|fault| AcceptanceSeedFault::Base(format!("the document does not print: {fault:?}")))?;
        files.dsl = base.to_string();
        artifact_app_laws::load_document_text(&mut app, &files).await.map_err(|fault| AcceptanceSeedFault::Base(format!("the base document does not load: {fault:?}")))?;""",
     """        let mut files = app.document_text().await.map_err(|fault| AcceptanceSeedFault::Base(format!("the document does not print: {fault:?}")))?;
        if files.dsl != base {
            files.dsl = base.to_string();
            artifact_app_laws::load_document_text(&mut app, &files).await.map_err(|fault| AcceptanceSeedFault::Base(format!("the base document does not load: {fault:?}")))?;
        }"""),
    ("""/// 🪴️ A registered instance whose document is `base` (loaded through the document text path) followed by one clean edit per op""",
     """/// 🪴️ A registered instance whose document is `base` (loaded through the document text path unless the instance boots on exactly
/// that text: its own initial document keeps the members it owns) followed by one clean edit per op"""),
    ("""            acceptance_verb(&mut app, action, args.iter().map(|(key, value)| (key.as_str(), value.clone())).collect()).await?;
            acceptance_pump(&mut app, |app| !app.has_pending_typed_operations()).await?;""",
     """            acceptance_verb(&mut app, action, args.iter().map(|(key, value)| (key.as_str(), value.clone())).collect()).await?;
            artifact_app_laws::settle_registered_typed_operation(&mut app, acceptance_meta().instance_id).await.map_err(|fault| format!("the seed gesture {action} does not publish: {fault:?}"))?;"""),
    ("""                acceptance_pump(&mut saved, |app| !app.has_pending_typed_operations()).await.map_err(|reason| format!("the example never finishes loading: {reason}"))?;""",
     """                artifact_app_laws::settle_registered_typed_operation(&mut saved, acceptance_meta().instance_id).await.map_err(|fault| format!("the example does not finish loading: {fault:?}"))?;"""),
]


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        before = handle.read()
    if "the seed gesture {action} does not publish" in before:
        print("SKIP: already carries harness v7")
        return
    if "the seeded document does not survive save and load before any edit" not in before:
        sys.exit("ORDER: apply harness v6 first")
    for index, (old, _) in enumerate(EDITS):
        if before.count(old) != 1:
            sys.exit(f"ANCHOR #{index}: {before.count(old)} matches: {old[:90]!r}")
    after = before
    for old, new in EDITS:
        after = after.replace(old, new)
    if "--preview" in sys.argv:
        open(sys.argv[sys.argv.index("--preview") + 1], "w", encoding="utf-8").write(after)
    if "--apply" not in sys.argv:
        print(f"WOULD apply harness v7 ({len(EDITS)} hunks, {after.count(chr(10)) - before.count(chr(10)):+d} lines)")
        return
    with open(PATH, encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE: the harness changed while staging")
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(after)
    print("WROTE")


if __name__ == "__main__":
    main()
