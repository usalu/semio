#!/usr/bin/env python3
"""🧪️ S4-BUMP: channel unit tests + goldens for the CHANNEL_VERSION 21 scope addition (`ReadChildHeads` → `ChildHeads`, the
one-lane `PureCommand { seq, command, head }`). Every anchor must match exactly once.
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
UNIT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs"

EDITS = [
    ("AppCommand::PureCommand { seq: 18, command: vec![1], document: vec![2], document_spr: vec![3], config: vec![4], config_spr: vec![5], draft: vec![6], draft_spr: vec![7] }).await;",
     "AppCommand::PureCommand { seq: 18, command: vec![1], head: vec![2] }).await;\n    assert_command_round_trips(&AppCommand::PureCommand { seq: 19, command: vec![1], head: Vec::new() }).await;"),
    ('("PureCommand", AppCommand::PureCommand { seq: 1, command: vec![1], document: vec![2], document_spr: vec![3], config: vec![4], config_spr: vec![5], draft: vec![6], draft_spr: vec![7] }),',
     '("PureCommand", AppCommand::PureCommand { seq: 1, command: vec![1], head: vec![2] }),'),
    ('        "PureCommand" => "0d010101010201030104010501060107",\n', '        "PureCommand" => "0d0101010102",\n'),
    ('        ("ReadChildren", AppCommand::ReadChildren { seq: 1 }),\n',
     '        ("ReadChildren", AppCommand::ReadChildren { seq: 1 }),\n        ("ReadChildHeads", AppCommand::ReadChildHeads { seq: 1 }),\n'),
    ('        "ReadChildren" => "0f01",\n', '        "ReadChildren" => "0f01",\n        "ReadChildHeads" => "2a01",\n'),
    ('        ("OperationCompleted", AppFrame::OperationCompleted { operation: 7, revision: 5, ui_scope: vec![1], history_patch: vec![2] }),\n',
     '        ("OperationCompleted", AppFrame::OperationCompleted { operation: 7, revision: 5, ui_scope: vec![1], history_patch: vec![2] }),\n'
     '        ("ChildHeads", AppFrame::ChildHeads { in_reply_to: 1, entries: vec![ChildHeadPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), head_pack: vec![1] }] }),\n'),
    ('        "OperationCompleted" => "19070501010102",\n', '        "OperationCompleted" => "19070501010102",\n        "ChildHeads" => "2001010173016301640101",\n'),
    ("""    assert_command_round_trips(&AppCommand::ReadChildren { seq: 21 }).await;
    assert_command_round_trips(&AppCommand::ReadHistory { seq: 22 }).await;
}
//#endregion 🔖️Children
""",
     """    assert_command_round_trips(&AppCommand::ReadChildren { seq: 21 }).await;
    assert_command_round_trips(&AppCommand::ReadHistory { seq: 22 }).await;
}

/// 🪆️ `ReadChildHeads` → `ChildHeads` round-trips through both the flat codec and the guest's paged ingress, a head list keeps
/// every entry (an empty head pack included), and a declared count past the member authority is refused before it reserves.
#[semio_framework_async_macros::async_test]
async fn child_head_commands_and_frames_round_trip() {
    assert_command_round_trips(&AppCommand::ReadChildHeads { seq: 23 }).await;
    assert_paged_route_admits(&AppCommand::ReadChildHeads { seq: 24 }).await;
    let entries = vec![
        ChildHeadPackEntry { slot: "mesh".to_string(), child_id: "child-1".to_string(), dialect: "s.stdio.mesh@1/*".to_string(), head_pack: vec![7, 8, 9] },
        ChildHeadPackEntry { slot: "brep".to_string(), child_id: "child-2".to_string(), dialect: "s.stdio.brep@1/*".to_string(), head_pack: Vec::new() },
    ];
    assert_frame_round_trips(&AppFrame::ChildHeads { in_reply_to: 23, entries }).await;
    assert_frame_round_trips(&AppFrame::ChildHeads { in_reply_to: 24, entries: Vec::new() }).await;
    let mut oversized = vec![32, 1];
    crate::os_spr::write_varint_u64(&mut oversized, DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS as u64 + 1);
    assert!(matches!(decode_app_frame(&oversized).await, Err(crate::os_spr::ProtocolError::Malformed { what: "channel child heads", .. })));
}
//#endregion 🔖️Children
"""),
]

text = UNIT.read_text()
for old, new in EDITS:
    found = text.count(old)
    if found != 1:
        sys.exit(f"anchor matched {found}x (expected 1): {old[:140]!r}")
    text = text.replace(old, new)
UNIT.write_text(text)
print(f"channel heads tests: {len(EDITS)} edits applied")
