#!/usr/bin/env python3
"""🧪️ S4-BUMP: re-seals the Rust channel unit tests for CHANNEL_VERSION 21 (design §20.7).

Drops `AppCommand::LoadDocument` (tag 6 stays unassigned), `AppCommand::TransactionPrepare.label` and
`AppFrame::TransactionProposal.{description, coalesce_key}` from every literal and golden; every anchor is
asserted to match exactly the expected number of times so a drifted file fails loudly instead of half-editing.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
UNIT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs"


def replace(text: str, old: str, new: str, count: int) -> str:
    found = text.count(old)
    if found != count:
        sys.exit(f"anchor matched {found}x (expected {count}): {old[:120]!r}")
    return text.replace(old, new)


def sub(text: str, pattern: str, new: str, count: int) -> str:
    result, found = re.subn(pattern, new, text)
    if found != count:
        sys.exit(f"pattern matched {found}x (expected {count}): {pattern!r}")
    return result


text = UNIT.read_text()
text = replace(
    text,
    """#[semio_framework_async_macros::async_test]
async fn app_command_load_document_round_trips() {
    assert_command_round_trips(&AppCommand::LoadDocument { seq: 8, pack: vec![1], spr: vec![2] }).await;
}
""",
    """/// 🕳️ Tag 6 (the retired whole-document `LoadDocument`) stays unassigned: a frame that still carries it is refused as an
/// unknown tag by both decoders, never misread as another command (design §20.7, `📓️api-stepped-document-load.md` §5).
#[semio_framework_async_macros::async_test]
async fn app_command_tag_six_stays_unassigned() {
    let err = decode_app_command(&[6, 8, 1, 1, 1, 2]).await.unwrap_err();
    assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-command tag", .. }));
    let mut pages = CommandPageSet::try_new(1).expect("one fixed page");
    pages.try_push(FixedCommandPage::try_copy_from(&[6, 8, 1, 1, 1, 2]).expect("retired tag page")).unwrap_or_else(|(fault, _page)| panic!("admit retired tag page: {fault:?}"));
    let command = PagedCommand::try_from_pages(pages).unwrap_or_else(|(fault, _pages)| panic!("admit retired tag command: {fault:?}"));
    let fault = PagedAppCommandDecodeCursor::new(command).step().expect_err("the retired tag is refused");
    assert_eq!(fault.code.0, "plugin.command-route-state-machine-required");
}
""",
    1,
)
text = sub(text, r"label: (?:String::new\(\)|\"l\"\.to_string\(\)), (origin: )", r"\1", 10)
text = replace(text, 'description: "d".to_string(), coalesce_key: "k".to_string(), ', "", 2)
text = replace(text, "        (AppCommand::LoadDocument { seq: 1, pack: vec![1, 2], spr: vec![3] }, 3),\n", "", 1)
text = replace(text, '        ("LoadDocument", AppCommand::LoadDocument { seq: 1, pack: vec![1], spr: vec![2] }),\n', "", 1)
text = replace(text, '        "LoadDocument" => "060101010102",\n', "", 1)
text = replace(text, '"TransactionPrepareOwner" => "11010174016d010900000000"', '"TransactionPrepareOwner" => "11010174016d0109000000"', 1)
text = replace(text, '"TransactionPreparePrePlanned" => "110201740000020101020202016c010900"', '"TransactionPreparePrePlanned" => "110201740000020101020202010900"', 1)
text = replace(text, '"TransactionPreparePrePlannedChildren" => "110701740000010101016c0109020506"', '"TransactionPreparePrePlannedChildren" => "1107017400000101010109020506"', 1)
text = replace(text, '"TransactionProposal" => "0f0101700101010164016b00"', '"TransactionProposal" => "0f01017001010100"', 1)
text = replace(text, "sequence, and no other command (never `LoadDocument`).", "sequence, and no other command.", 1)
UNIT.write_text(text)
print("channel unit tests re-sealed")
