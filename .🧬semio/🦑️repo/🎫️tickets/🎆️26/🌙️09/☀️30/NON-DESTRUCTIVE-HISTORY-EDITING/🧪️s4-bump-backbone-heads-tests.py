#!/usr/bin/env python3
"""🧪️ S4-BUMP: TS channel-twin suite additions for the CHANNEL_VERSION 21 scope addition — `ReadChildHeads` (42) → `ChildHeads`
(32) round trips, tags, goldens and `AppChannelClient.readChildHeads()`, and the one-lane `PureCommand { seq, command, head }`.
Goldens are the Rust corpus' own (`📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs`). Every anchor must match exactly once.
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SUITE = ROOT / "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts"

EDITS = [
    ("      { PureCommand: { seq: 17, command: [1], document: [2], document_spr: [3], config: [4], config_spr: [5], draft: [6], draft_spr: [7] } },\n",
     "      { PureCommand: { seq: 17, command: [1], head: [2] } },\n      { PureCommand: { seq: 41, command: [1], head: [] } },\n"),
    ("      { ReadChildren: { seq: 19 } },\n", "      { ReadChildren: { seq: 19 } },\n      { ReadChildHeads: { seq: 42 } },\n"),
    ('      { Children: { in_reply_to: 13, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } },\n',
     '      { Children: { in_reply_to: 13, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } },\n'
     '      { ChildHeads: { in_reply_to: 42, entries: [{ slot: "mesh", child_id: "child-1", dialect: "s.stdio.mesh@1/*", head_pack: [7, 8, 9] }, { slot: "brep", child_id: "child-2", dialect: "s.stdio.brep@1/*", head_pack: [] }] } },\n'
     "      { ChildHeads: { in_reply_to: 43, entries: [] } },\n"),
    ("      expect(encodeAppCommand({ ReadChildren: { seq: 0 } })[0]).toBe(15);\n",
     "      expect(encodeAppCommand({ ReadChildren: { seq: 0 } })[0]).toBe(15);\n      expect(encodeAppCommand({ ReadChildHeads: { seq: 0 } })[0]).toBe(42);\n"),
    ("      expect(encodeAppFrame({ DocumentArchiveLoad: { in_reply_to: 0, status: { operation: 1, state: \"ready\", completed: 2, total: 2, fault: [] } } })[0]).toBe(27);\n    });\n",
     "      expect(encodeAppFrame({ DocumentArchiveLoad: { in_reply_to: 0, status: { operation: 1, state: \"ready\", completed: 2, total: 2, fault: [] } } })[0]).toBe(27);\n"
     "      expect(encodeAppFrame({ ChildHeads: { in_reply_to: 0, entries: [] } })[0]).toBe(32);\n"
     "      expect(() => decodeAppFrame(new Uint8Array([32, 1, 0x81, 0x08]))).toThrow(\"decodeAppFrame: child head count exceeds the member authority\");\n    });\n"),
    ('        ["PureCommand", { PureCommand: { seq: 1, command: [1], document: [2], document_spr: [3], config: [4], config_spr: [5], draft: [6], draft_spr: [7] } }],\n',
     '        ["PureCommand", { PureCommand: { seq: 1, command: [1], head: [2] } }],\n'),
    ('        ["ReadChildren", { ReadChildren: { seq: 1 } }],\n', '        ["ReadChildren", { ReadChildren: { seq: 1 } }],\n        ["ReadChildHeads", { ReadChildHeads: { seq: 1 } }],\n'),
    ('        PureCommand: "0d010101010201030104010501060107",\n', '        PureCommand: "0d0101010102",\n'),
    ('        ReadChildren: "0f01",\n', '        ReadChildren: "0f01",\n        ReadChildHeads: "2a01",\n'),
    ('        ["Children", { Children: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } }],\n',
     '        ["Children", { Children: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } }],\n'
     '        ["ChildHeads", { ChildHeads: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", head_pack: [1] }] } }],\n'),
    ('        Children: "0c01010173016301640101",\n        Ephemeral:', '        Children: "0c01010173016301640101",\n        ChildHeads: "2001010173016301640101",\n        Ephemeral:'),
    ('''    it("configure()/readDocument() frame the right AppCommand variant", async () => {''',
     '''    it("readChildHeads() sends ReadChildHeads and answers the ChildHeads entries of its own sequence, owning their bytes", async () => {
      const seen: AppCommandValue[] = [];
      const heads = [{ slot: "mesh", child_id: "child-1", dialect: "s.stdio.mesh@1/*", head_pack: [7, 8] }];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        const command = commands[0]!;
        return "ReadChildHeads" in command ? [{ ChildHeads: { in_reply_to: command.ReadChildHeads.seq, entries: heads } }] : [{ Done: { in_reply_to: commandSeq(command) } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      const entries = await client.readChildHeads();
      heads[0]!.head_pack.fill(0);
      expect([seen, entries]).toEqual([[{ ReadChildHeads: { seq: 1 } }], [{ slot: "mesh", child_id: "child-1", dialect: "s.stdio.mesh@1/*", head_pack: [7, 8] }]]);
      const refusing = new AppChannelClient(fakeHandle((_instanceId, commands) => [{ Done: { in_reply_to: commandSeq(commands[0]!) } }]), new AppChannelRequestSequence(), 1, "app.demo");
      await expect(refusing.readChildHeads()).rejects.toThrow("missing ChildHeads frame for seq 1");
    });

    it("configure()/readDocument() frame the right AppCommand variant", async () => {'''),
]

text = SUITE.read_text()
for old, new in EDITS:
    found = text.count(old)
    if found != 1:
        sys.exit(f"anchor matched {found}x (expected 1): {old[:140]!r}")
    text = text.replace(old, new)
SUITE.write_text(text)
print(f"backbone heads: {len(EDITS)} edits applied")
