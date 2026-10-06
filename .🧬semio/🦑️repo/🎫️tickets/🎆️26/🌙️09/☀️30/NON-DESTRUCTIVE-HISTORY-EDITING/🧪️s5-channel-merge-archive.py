#!/usr/bin/env python3
"""🧲️ S5-CHANNEL rider of wave B / CHANNEL_VERSION 22 (design §22.22): the codec of the document-archive merge, Rust and
TypeScript twin, with its shared golden vectors.

- `AppCommand::MergeDocumentArchive { seq, archive }`, the last command (tag 43): `seq` varint, then `archive` in exactly the
  `LoadDocumentArchive.archive` encoding (one shared encoder arm, one shared paged decode state, one shared retirement);
- `DocumentArchiveLoadStatus.ahead` (varint after `total`, before `fault`).
`EDITS` are written by `--apply` (explicit files, every anchor exactly its count, else nothing is written), `NEW_FILES` are
created; `HAND` rows are the Edit-tool rows of the hot plugin runtime; `--verify` proves every row is on disk. The guest
answers `app.command.unsupported` (pinned by a guest law) until S5-LOAD lands `PluginApp::begin_document_archive_merge`.
Usage: python3 🧪️s5-channel-merge-archive.py [--apply | --verify]
"""
from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
CHANNEL = f"{OSM}/📡️spr/🧵️channel/🦀️.rs"
CHANNEL_TESTS = f"{OSM}/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs"
RUN = f"{OSM}/🏃️run/🦀️.rs"
RUN_TESTS = f"{OSM}/🏃️run/🧪️tests/🔬️unit/🦀️.rs"
MCP = f"{OSM}/🌉️mcp/🏠️workspace/🦀️.rs"
PLUGIN = f"{OSM}/🔌️plugin/🦀️.rs"
GUEST_TESTS = f"{OSM}/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs"
OS = "🧰️framework/🛍️products/💻️os"
TWIN = f"{OS}/🟦️.ts"
TWIN_TESTS = f"{OS}/🧪️tests/🧪️backbone-envelope-io/🟦️.ts"
HOST_TESTS = f"{OS}/🧪️tests/🧪️document-archive-load-host/🟦️.ts"
RUNTIME_TSX_TESTS = f"{OSM}/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx"

EDITS: dict[str, list[tuple[str, str, int]]] = {
    CHANNEL: [
        ("    pub total: u64,\n    pub fault: Vec<u8>,\n}\n//#endregion 🔖️DocumentArchive",
         "    pub total: u64,\n    /// ⏭️ Events this program holds that the archive lacks: 0 for a replacement, the merge's own count otherwise.\n    pub ahead: u64,\n    pub fault: Vec<u8>,\n}\n//#endregion 🔖️DocumentArchive", 1),
        ("    LoadDocumentArchive { seq: u64, decode: PagedDocumentArchiveDecode },", "    LoadDocumentArchive { seq: u64, merge: bool, decode: PagedDocumentArchiveDecode },", 1),
        ("            if let AppCommand::LoadDocumentArchive { archive, .. } = command {", "            if let AppCommand::LoadDocumentArchive { archive, .. } | AppCommand::MergeDocumentArchive { archive, .. } = command {", 1),
        ("                    32 => PagedAppCommandDecodeState::LoadDocumentArchive { seq, decode: PagedDocumentArchiveDecode::new() },",
         "                    32 | 43 => PagedAppCommandDecodeState::LoadDocumentArchive { seq, merge: tag == 43, decode: PagedDocumentArchiveDecode::new() },", 1),
        ("            PagedAppCommandDecodeState::LoadDocumentArchive { seq, mut decode } => match decode.step(&mut self.reader) {", "            PagedAppCommandDecodeState::LoadDocumentArchive { seq, merge, mut decode } => match decode.step(&mut self.reader) {", 1),
        ("                Ok(Some(archive)) => Some(AppCommand::LoadDocumentArchive { seq, archive }),",
         "                Ok(Some(archive)) => Some(if merge { AppCommand::MergeDocumentArchive { seq, archive } } else { AppCommand::LoadDocumentArchive { seq, archive } }),", 1),
        ("                    self.state = PagedAppCommandDecodeState::LoadDocumentArchive { seq, decode };", "                    self.state = PagedAppCommandDecodeState::LoadDocumentArchive { seq, merge, decode };", 2),
        ("                    AppCommand::LoadDocumentArchive { archive, .. } => document_archive_into_fields(archive),",
         "                    AppCommand::LoadDocumentArchive { archive, .. } | AppCommand::MergeDocumentArchive { archive, .. } => document_archive_into_fields(archive),", 1),
        ("    ReadChildHeads { seq: u64 },\n}\n//#endregion 🔖️AppCommand",
         "    ReadChildHeads { seq: u64 },\n    /// 🧲️ Merges an archive of the document this program already shows: the guest ingests the events its stores lack and\n    /// never replaces them or adopts the archive's viewed head (design §22.22). Same `archive` encoding and the same poll /\n    /// cancel / acknowledge exchange as `LoadDocumentArchive`; an archive of another document is refused with\n    /// `plugin.document-load.other-document`. CHANNEL_VERSION 22 wire addition.\n    MergeDocumentArchive {\n        seq: u64,\n        archive: DocumentArchivePack,\n    },\n}\n//#endregion 🔖️AppCommand", 1),
        ("        AppCommand::LoadDocumentArchive { seq, archive } => {", "        AppCommand::LoadDocumentArchive { seq, archive } | AppCommand::MergeDocumentArchive { seq, archive } => {", 1),
        ("            out.byte(32)?;", "            out.byte(if matches!(command, AppCommand::MergeDocumentArchive { .. }) { 43 } else { 32 })?;", 1),
        ("        42 => AppCommand::ReadChildHeads { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },\n",
         "        42 => AppCommand::ReadChildHeads { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },\n        43 => AppCommand::MergeDocumentArchive { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, archive: read_document_archive(bytes, &mut pos).await? },\n", 1),
        ("            crate::os_spr::write_varint_u64(&mut out, status.total);\n            crate::os_spr::write_bytes(&mut out, &status.fault);",
         "            crate::os_spr::write_varint_u64(&mut out, status.total);\n            crate::os_spr::write_varint_u64(&mut out, status.ahead);\n            crate::os_spr::write_bytes(&mut out, &status.fault);", 1),
        ("            let total = crate::os_spr::read_varint_u64(bytes, &mut pos)?;\n            let fault = crate::os_spr::read_bytes(bytes, &mut pos)?;\n            AppFrame::DocumentArchiveLoad { in_reply_to, status: DocumentArchiveLoadStatus { operation, state, completed, total, fault } }",
         "            let total = crate::os_spr::read_varint_u64(bytes, &mut pos)?;\n            let ahead = crate::os_spr::read_varint_u64(bytes, &mut pos)?;\n            let fault = crate::os_spr::read_bytes(bytes, &mut pos)?;\n            AppFrame::DocumentArchiveLoad { in_reply_to, status: DocumentArchiveLoadStatus { operation, state, completed, total, ahead, fault } }", 1),
    ],
    CHANNEL_TESTS: [
        ("    assert_command_round_trips(&AppCommand::LoadDocumentArchive { seq: 10, archive: sample_document_archive() }).await;\n",
         "    assert_command_round_trips(&AppCommand::LoadDocumentArchive { seq: 10, archive: sample_document_archive() }).await;\n    assert_command_round_trips(&AppCommand::MergeDocumentArchive { seq: 13, archive: sample_document_archive() }).await;\n", 1),
        ("        status: DocumentArchiveLoadStatus { operation: 10, state: DocumentArchiveLoadState::Running, completed: 3, total: 9, fault: Vec::new() },",
         "        status: DocumentArchiveLoadStatus { operation: 10, state: DocumentArchiveLoadState::Running, completed: 3, total: 9, ahead: 4, fault: Vec::new() },", 1),
        ("        (AppCommand::LoadDocumentArchive { seq: 10, archive: sample_document_archive() }, 28),\n",
         "        (AppCommand::LoadDocumentArchive { seq: 10, archive: sample_document_archive() }, 28),\n        (AppCommand::MergeDocumentArchive { seq: 13, archive: sample_document_archive() }, 28),\n", 1),
        ("                        total: answer[\"total\"].as_u64().unwrap(),\n", "                        total: answer[\"total\"].as_u64().unwrap(),\n                        ahead: 0,\n", 1),
        ("#[semio_framework_async_macros::async_test]\nasync fn paged_recursive_archive_crosses_pages_and_decoded_owner_closes_one_field_per_grant() {",
         "/// 🧲️ `MergeDocumentArchive` (tag 43, CHANNEL_VERSION 22, design §22.22) carries its archive in exactly the\n/// `LoadDocumentArchive` encoding: the two commands differ in their tag byte alone, on the flat and on the paged route.\n#[semio_framework_async_macros::async_test]\nasync fn a_merge_archive_command_is_the_load_encoding_under_its_own_tag() {\n    let load = encode_fixture_command(&AppCommand::LoadDocumentArchive { seq: 14, archive: sample_document_archive() }).await;\n    let merge = encode_fixture_command(&AppCommand::MergeDocumentArchive { seq: 14, archive: sample_document_archive() }).await;\n    assert_eq!((load[0], merge[0]), (32, 43));\n    assert_eq!(load[1..], merge[1..]);\n    assert_eq!(decode_app_command(&merge).await.unwrap(), AppCommand::MergeDocumentArchive { seq: 14, archive: sample_document_archive() });\n    let mut cursor = PagedAppCommandDecodeCursor::new(encode_app_command(&AppCommand::MergeDocumentArchive { seq: 14, archive: sample_document_archive() }).await.unwrap());\n    let mut decoded = None;\n    for _ in 0..64 {\n        decoded = cursor.step().unwrap();\n        if decoded.is_some() {\n            break;\n        }\n    }\n    assert_eq!(decoded, Some(AppCommand::MergeDocumentArchive { seq: 14, archive: sample_document_archive() }));\n    assert!(cursor.terminal_is_empty());\n}\n\n/// ⏭️ The shared cross-language vectors of the merge rider (`🧫️fixtures/📡️channel/🧲️document-archive-merge.json`): both\n/// archive commands and the load status with its `ahead` varint after `total`, byte for byte, in both directions.\n#[semio_framework_async_macros::async_test]\nasync fn document_archive_merge_matches_shared_cross_language_json_vectors() {\n    let fixture: serde_json::Value = serde_json::from_str(include_str!(\"../../../../../🧫️fixtures/📡️channel/🧲️document-archive-merge.json\")).expect(\"document archive merge fixture parses\");\n    let bytes = |value: &serde_json::Value| value.as_array().expect(\"bytes\").iter().map(|byte| byte.as_u64().expect(\"byte\") as u8).collect::<Vec<u8>>();\n    let seq = fixture[\"seq\"].as_u64().expect(\"seq\");\n    let archive = || DocumentArchivePack { parent_pack: bytes(&fixture[\"archive\"][\"parent_pack\"]), parent_spr: bytes(&fixture[\"archive\"][\"parent_spr\"]), members: Vec::new() };\n    for (name, command) in [(\"LoadDocumentArchive\", AppCommand::LoadDocumentArchive { seq, archive: archive() }), (\"MergeDocumentArchive\", AppCommand::MergeDocumentArchive { seq, archive: archive() })] {\n        let encoded = encode_fixture_command(&command).await;\n        assert_eq!(hex_encode(&encoded).await, fixture[name].as_str().expect(\"command fixture hex\"), \"{name}\");\n        assert_eq!(decode_app_command(&encoded).await.expect(\"fixture command decodes\"), command, \"{name}\");\n    }\n    let status = &fixture[\"status\"];\n    assert_eq!(status[\"state\"], \"ready\");\n    let frame = AppFrame::DocumentArchiveLoad { in_reply_to: status[\"in_reply_to\"].as_u64().expect(\"in_reply_to\"), status: DocumentArchiveLoadStatus { operation: status[\"operation\"].as_u64().expect(\"operation\"), state: DocumentArchiveLoadState::Ready, completed: status[\"completed\"].as_u64().expect(\"completed\"), total: status[\"total\"].as_u64().expect(\"total\"), ahead: status[\"ahead\"].as_u64().expect(\"ahead\"), fault: bytes(&status[\"fault\"]) } };\n    assert_eq!(hex_encode(&encode_app_frame(&frame).await).await, fixture[\"DocumentArchiveLoad\"].as_str().expect(\"frame fixture hex\"));\n    assert_frame_round_trips(&frame).await;\n}\n\n#[semio_framework_async_macros::async_test]\nasync fn paged_recursive_archive_crosses_pages_and_decoded_owner_closes_one_field_per_grant() {", 1),
    ],
    RUN: [
        ("        | AppCommand::ReadChildHeads { seq }\n        | AppCommand::TakeMediaExportChunk { seq, .. } => *seq,", "        | AppCommand::ReadChildHeads { seq }\n        | AppCommand::MergeDocumentArchive { seq, .. }\n        | AppCommand::TakeMediaExportChunk { seq, .. } => *seq,", 1),
    ],
    RUN_TESTS: [
        ("protocol::DocumentArchiveLoadStatus { operation: self.operation, state: self.state, completed: self.completed, total: self.total, fault: self.fault.clone() }",
         "protocol::DocumentArchiveLoadStatus { operation: self.operation, state: self.state, completed: self.completed, total: self.total, ahead: 0, fault: self.fault.clone() }", 1),
    ],
    MCP: [
        ("        | store::AppCommand::ReadChildHeads { seq }\n        | store::AppCommand::TakeMediaExportChunk { seq, .. } => seq,", "        | store::AppCommand::ReadChildHeads { seq }\n        | store::AppCommand::MergeDocumentArchive { seq, .. }\n        | store::AppCommand::TakeMediaExportChunk { seq, .. } => seq,", 1),
    ],
    GUEST_TESTS: [
        ("async fn query_app() -> VcsArtifactApp<TestApp> {",
         "/// 🧲️ Until the merge handlers land (design §22.22) a program answers `MergeDocumentArchive` with its own typed refusal on\n/// the encoded and on the decoded route: an `Error` frame naming `app.command.unsupported`, never `Done`, never a decode fault.\n#[semio_framework_async_macros::async_test]\nasync fn a_merge_archive_command_is_refused_as_unsupported_until_its_handler_lands() {\n    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();\n    let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await), surface_contexts: Default::default() }));\n    runtime.instances.borrow_mut().insert_admitted(7, cell.clone());\n    let merge = |seq| protocol::AppCommand::MergeDocumentArchive { seq, archive: protocol::DocumentArchivePack { parent_pack: vec![1, 2], parent_spr: vec![3], members: Vec::new() } };\n    for (seq, frames) in [(0, wire_command(&runtime, 0, merge(0)).await), (1, cold_decoded_command(&runtime, 1, merge(1)).await)] {\n        let refusal = frames\n            .iter()\n            .find_map(|frame| match frame {\n                protocol::AppFrame::Error { in_reply_to: Some(answered), fault, .. } if *answered == seq => Some(fault),\n                _ => None,\n            })\n            .unwrap_or_else(|| panic!(\"the merge is refused through its own error frame: {frames:?}\"));\n        let fault: Fault = super::super::decode_wire_serialized(refusal).await.unwrap();\n        assert_eq!(fault.code.0, \"app.command.unsupported\");\n        assert!(!frames.iter().any(|frame| matches!(frame, protocol::AppFrame::Done { .. })), \"a refused merge is not done: {frames:?}\");\n    }\n    drop(cell);\n    crate::plugin_runtime::plugin_destroy_app(&runtime, 7).await.unwrap();\n    for _ in 0..200_000 {\n        crate::plugin_runtime::plugin_step_close_cleanup(&runtime).unwrap();\n        if runtime.close_quarantine.borrow().get(7).is_none() { break; }\n        std::thread::yield_now();\n    }\n    assert!(runtime.close_quarantine.borrow().get(7).is_none());\n}\n\nasync fn query_app() -> VcsArtifactApp<TestApp> {", 1),
    ],
    TWIN: [
        ("readonly completed: number; readonly total: number; readonly fault: readonly number[] };", "readonly completed: number; readonly total: number; readonly ahead: number; readonly fault: readonly number[] };", 1),
        ("  | { readonly LoadDocumentArchive: { readonly seq: number; readonly archive: DocumentArchivePack } }\n",
         "  | { readonly LoadDocumentArchive: { readonly seq: number; readonly archive: DocumentArchivePack } }\n  | { readonly MergeDocumentArchive: { readonly seq: number; readonly archive: DocumentArchivePack } }\n", 1),
        ("ReadDocumentIdentity: 41, ReadChildHeads: 42,\n} as const;", "ReadDocumentIdentity: 41, ReadChildHeads: 42, MergeDocumentArchive: 43,\n} as const;", 1),
        ("    writeDocumentArchive(out, cmd.LoadDocumentArchive.archive);\n",
         "    writeDocumentArchive(out, cmd.LoadDocumentArchive.archive);\n  } else if (\"MergeDocumentArchive\" in cmd) {\n    out.push(APP_COMMAND_TAGS.MergeDocumentArchive);\n    writeVarintU64(out, cmd.MergeDocumentArchive.seq);\n    writeDocumentArchive(out, cmd.MergeDocumentArchive.archive);\n", 1),
        ("      return { LoadDocumentArchive: { seq: readVarintU64(bytes, pos), archive: readDocumentArchive(bytes, pos) } };\n",
         "      return { LoadDocumentArchive: { seq: readVarintU64(bytes, pos), archive: readDocumentArchive(bytes, pos) } };\n    case APP_COMMAND_TAGS.MergeDocumentArchive:\n      return { MergeDocumentArchive: { seq: readVarintU64(bytes, pos), archive: readDocumentArchive(bytes, pos) } };\n", 1),
        ("    writeVarintU64(out, frame.DocumentArchiveLoad.status.total);\n", "    writeVarintU64(out, frame.DocumentArchiveLoad.status.total);\n    writeVarintU64(out, frame.DocumentArchiveLoad.status.ahead);\n", 1),
        ("completed: readVarintU64(bytes, pos), total: readVarintU64(bytes, pos), fault: readBytes(bytes, pos) } } };", "completed: readVarintU64(bytes, pos), total: readVarintU64(bytes, pos), ahead: readVarintU64(bytes, pos), fault: readBytes(bytes, pos) } } };", 1),
    ],
    TWIN_TESTS: [
        ("      { LoadDocumentArchive: { seq: 36, archive: documentArchive } },\n", "      { LoadDocumentArchive: { seq: 36, archive: documentArchive } },\n      { MergeDocumentArchive: { seq: 43, archive: documentArchive } },\n", 1),
        ("      expect(encodeAppCommand({ ReadChildHeads: { seq: 0 } })[0]).toBe(42);\n",
         "      expect(encodeAppCommand({ ReadChildHeads: { seq: 0 } })[0]).toBe(42);\n      expect(encodeAppCommand({ MergeDocumentArchive: { seq: 0, archive: { parent_pack: [], parent_spr: [], members: [] } } })[0]).toBe(43);\n", 1),
        ("total: 3, fault: [] }", "total: 3, ahead: 0, fault: [] }", 4),
        ("total: 2, fault: [] }", "total: 2, ahead: 0, fault: [] }", 1),
        ("total: 1, fault: [] }", "total: 1, ahead: 0, fault: [] }", 1),
        ("    it(\"command() allocates an incrementing seq and returns every frame the batch produced\", async () => {",
         "    it(\"matches the shared cross-language document-archive merge vectors, byte-for-byte\", async () => {\n      const { readFileSync } = await import(\"node:fs\");\n      const { fileURLToPath } = await import(\"node:url\");\n      const { dirname, join } = await import(\"node:path\");\n      const vectors = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), \"🧫️fixtures\", \"📡️channel\", \"🧲️document-archive-merge.json\"), \"utf8\")) as { seq: number; archive: { parent_pack: number[]; parent_spr: number[]; members: [] }; LoadDocumentArchive: string; MergeDocumentArchive: string; status: { in_reply_to: number; operation: number; state: string; completed: number; total: number; ahead: number; fault: number[] }; DocumentArchiveLoad: string };\n      const hex = (bytes: Uint8Array) => Array.from(bytes, (byte) => byte.toString(16).padStart(2, \"0\")).join(\"\");\n      const load = { LoadDocumentArchive: { seq: vectors.seq, archive: vectors.archive } };\n      const merge = { MergeDocumentArchive: { seq: vectors.seq, archive: vectors.archive } };\n      expect(hex(encodeAppCommand(load))).toBe(vectors.LoadDocumentArchive);\n      expect(hex(encodeAppCommand(merge))).toBe(vectors.MergeDocumentArchive);\n      expect(decodeAppCommand(new Uint8Array(Buffer.from(vectors.LoadDocumentArchive, \"hex\")))).toEqual(load);\n      expect(decodeAppCommand(new Uint8Array(Buffer.from(vectors.MergeDocumentArchive, \"hex\")))).toEqual(merge);\n      expect(vectors.status.state).toBe(\"ready\");\n      const frame: AppFrameValue = { DocumentArchiveLoad: { in_reply_to: vectors.status.in_reply_to, status: { operation: vectors.status.operation, state: \"ready\", completed: vectors.status.completed, total: vectors.status.total, ahead: vectors.status.ahead, fault: vectors.status.fault } } };\n      expect(hex(encodeAppFrame(frame))).toBe(vectors.DocumentArchiveLoad);\n      expect(decodeAppFrame(new Uint8Array(Buffer.from(vectors.DocumentArchiveLoad, \"hex\")))).toEqual(frame);\n    });\n\n    it(\"command() allocates an incrementing seq and returns every frame the batch produced\", async () => {", 1),
    ],
    HOST_TESTS: [
        ("completed: answer.completed, total: answer.total, fault: bytes(answer.fault) };", "completed: answer.completed, total: answer.total, ahead: 0, fault: bytes(answer.fault) };", 1),
    ],
    RUNTIME_TSX_TESTS: [
        ("completed: polls, total: 3, fault: [] } } });", "completed: polls, total: 3, ahead: 0, fault: [] } } });", 1),
    ],
}

NEW_FILES: dict[str, str] = {
    f"{OS}/🧫️fixtures/📡️channel/🧲️document-archive-merge.json": """{
  "_comment": "🧲️ Cross-language golden wire vectors of the document-archive merge (CHANNEL_VERSION 22, design §22.22). `MergeDocumentArchive` (tag 43) carries `seq` and the archive in exactly the `LoadDocumentArchive` (tag 32) encoding, so the two commands differ in their tag byte alone; the load status frame (tag 27) states `ahead` as one varint after `total` and before `fault`. Rust proves them in protocol_channel's document_archive_merge_matches_shared_cross_language_json_vectors; TypeScript encodes and decodes the same bytes in 🧪️backbone-envelope-io.",
  "seq": 7,
  "archive": { "parent_pack": [1, 2], "parent_spr": [3], "members": [] },
  "LoadDocumentArchive": "2007020102010300",
  "MergeDocumentArchive": "2b07020102010300",
  "status": { "in_reply_to": 9, "operation": 7, "state": "ready", "completed": 5, "total": 5, "ahead": 3, "fault": [] },
  "DocumentArchiveLoad": "1b09070205050300"
}
""",
}

HAND: dict[str, list[tuple[str, str, int]]] = {
    PLUGIN: [
        ("completed: self.completed.saturating_add(self.fold.0), total: self.total.saturating_add(self.fold.1), fault: self.fault.clone() }",
         "completed: self.completed.saturating_add(self.fold.0), total: self.total.saturating_add(self.fold.1), ahead: 0, fault: self.fault.clone() }", 1),
        ("state: protocol::DocumentArchiveLoadState::Pending, completed: 0, total: 0, fault: Vec::new() }", "state: protocol::DocumentArchiveLoadState::Pending, completed: 0, total: 0, ahead: 0, fault: Vec::new() }", 1),
        ("                protocol::AppCommand::PollDocumentArchiveLoad { seq, operation } => {\n                    let status = with_instances_mut(runtime, |list| {",
         "                protocol::AppCommand::MergeDocumentArchive { seq, .. } => {\n                    push_app_fault(&mut frames, Some(seq), Fault::new(FaultOrigin::Framework, FaultCode::new(\"app.command.unsupported\"), \"this program does not merge a document archive yet\")).await;\n                }\n                protocol::AppCommand::PollDocumentArchiveLoad { seq, operation } => {\n                    let status = with_instances_mut(runtime, |list| {", 1),
    ],
}


def judge(index: int, text: str, old: str, new: str, count: int, verify: bool) -> bool:
    """⚖️ Dry run: `old` occurs exactly `count` times. Verify: `new` is on disk as often and no bare `old` is left over."""
    found = text.count(new) if verify else text.count(old)
    leftover = verify and text.count(old) != (text.count(new) if old in new else 0)
    wrong = found != count or leftover
    print(f"   {index:2} {'MISMATCH' if wrong else 'ok'} x{found}/{count}: {old.strip().splitlines()[0][:100]}")
    return wrong


def main() -> None:
    apply, verify = "--apply" in sys.argv, "--verify" in sys.argv
    if not (ROOT / ".git").exists():
        raise SystemExit("[DEBUG] not the repo root: refusing")
    results: dict[str, str] = {}
    failed = False
    for path, rows in EDITS.items():
        target = ROOT / path
        if not target.is_file():
            raise SystemExit(f"[DEBUG] missing file: {path}")
        text = target.read_text()
        print(f"== {path}")
        for index, (old, new, count) in enumerate(rows, 1):
            failed |= judge(index, text, old, new, count, verify)
            text = text.replace(old, new)
        results[path] = text
    for path, rows in HAND.items():
        text = (ROOT / path).read_text()
        print(f"== HAND (Edit tool) {path}")
        for index, (old, new, count) in enumerate(rows, 1):
            failed |= judge(index, text, old, new, count, verify)
    for path, content in NEW_FILES.items():
        present = (ROOT / path).is_file()
        wrong = present != verify or (verify and (ROOT / path).read_text() != content)
        failed |= wrong
        print(f"== NEW {path}: {'MISMATCH' if wrong else 'ok'} ({'present' if present else 'absent'})")
    if failed:
        raise SystemExit("[DEBUG] anchors do not match: nothing written")
    if apply:
        for path, text in results.items():
            (ROOT / path).write_text(text)
        for path, content in NEW_FILES.items():
            (ROOT / path).write_text(content)
    print(f"[DEBUG] {len(results)} files + {len(NEW_FILES)} new {'written' if apply else ('verified' if verify else 'would change (dry run)')}; {sum(map(len, HAND.values()))} hand rows")


if __name__ == "__main__":
    main()
