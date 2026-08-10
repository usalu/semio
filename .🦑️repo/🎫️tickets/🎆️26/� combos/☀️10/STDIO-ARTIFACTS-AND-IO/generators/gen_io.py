#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from pathlib import Path
import json

TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = Path("✏️s/🔌️plugins") / TOKENS["stdio_plugin"]
DESER = TOKENS["deserializers"]
SER = TOKENS["serializers"]
schema_dir = PLUGIN / "🗿️artifacts" / ROSTER["binary"]["dir"] / "🧬️schema"
RS = next(p.name for p in schema_dir.iterdir() if p.name.endswith("component.rs"))
TS = next(p.name for p in schema_dir.iterdir() if p.name.endswith("component.ts"))

IO_DEPS = {
    "binary": ["binary"],
    "txt": ["binary"],
    "json": ["txt"],
}

for mid, name in [("binary", "Binary"), ("txt", "Txt"), ("json", "Json")]:
    snap = name + "Snapshot"
    kind = "stdio." + mid
    base = PLUGIN / "🗿️artifacts" / ROSTER[mid]["dir"] / "🚪️io"
    base.mkdir(parents=True, exist_ok=True)

    dep_regs = []
    for dep in IO_DEPS[mid]:
        dep_regs.append(
            "        crate::artifacts::"
            + mid
            + "::io::import::deserializers::artifacts::"
            + dep
            + "::register();"
        )
        dep_regs.append(
            "        crate::artifacts::"
            + mid
            + "::io::export::serializers::artifacts::"
            + dep
            + "::register();"
        )
    text = "\n".join(
        [
            "//! 🚪️ `" + kind + "` IO facet — artifact-to-artifact serializers/deserializers.",
            "",
            "//#region 🔖️Register",
            "/// 🗂️ Registers import deserializers and export serializers.",
            "pub fn register() {",
        ]
        + dep_regs
        + [
            "}",
            "//#endregion 🔖️Register",
            "",
        ]
    )
    (base / RS).write_text(text)
    (base / TS).write_text("/** 🚪️ `" + kind + "` IO facet. */\nexport {};\n")

    for dep in IO_DEPS[mid]:
        dep_dir = ROSTER[dep]["dir"]
        dpath = base / "📥️import" / DESER / "🗿️artifacts" / dep_dir
        dpath.mkdir(parents=True, exist_ok=True)
        spath = base / "�ER / "🗿️artifacts" / dep_dir
        dpath.mkdir(parents=True, exist_ok=True)
        spath = base / "📤️export" / SER / "🗿️artifacts" / dep_dir
        spath.mkdir(parents=True, exist_ok=True)

        if mid == "binary" and dep == "binary":
            dbody = "\n".join(
                [
                    "//! 📥️ Deserialize `" + kind + "` from stdio.binary.",
                    "",
                    "use crate::artifacts::" + mid + "::{" + snap + "};",
                    "",
                    "//#region 🔖️Codec",
                    "/// 🗂️ Register deserializer hooks (identity for terminal binary).",
                    "pub fn register() {}",
                    "",
                    "/// 📥 Decode opaque bytes into a BinarySnapshot.",
                    "pub fn deserialize(bytes: &[u8]) -> Result<" + snap + ", store::PackError> {",
                    "    <" + snap + " as store::DocumentPack>::decode_pack(bytes)",
                    "}",
                    "//#endregion 🔖️Codec",
                    "",
                ]
            )
            sbody = "\n".join(
                [
                    "//! � crit Serialize `" + kind + "` to stdio.binary.",
                    "",
                    "use crate::artifacts::" + mid + "::{" + snap + "};",
                    "",
                    "//#region 🔖️Codec",
                    "/// 🗂️ Register serializer hooks (identity for terminal binary).",
                    "pub fn register() {}",
                    "",
                    "/// � crit Encode a BinarySnapshot to pack bytes.",
                    "pub fn serialize(snapshot: &" + snap + ") -> Result<Vec<u8>, store::PackError> {",
                    "    snapshot.encode_pack_with(&store::PackEncodeOptions::default())",
                    "}",
                    "//#endregion 🔖️Codec",
                    "",
                ]
            )
        elif mid == "txt" and dep == "binary":
            dbody = "\n".join(
                [
                    "//! 📥️ Deserialize `" + kind + "` from stdio.binary.",
                    "",
                    "use crate::artifacts::binary::BinarySnapshot;",
                    "use crate::artifacts::" + mid + "::{" + snap + ", STDIO_TXT_DOCUMENT_SCHEMA};",
                    "",
                    "//#region 🔖️Codec",
                    "/// 🗂️ Register deserializer hooks.",
                    "pub fn register() {}",
                    "",
                    "/// 📥 UTF-8 decode binary bytes into a TxtSnapshot.",
                    "pub fn deserialize(from: &BinarySnapshot) -> Result<" + snap + ", store::PackError> {",
                    "    let text = String::from_utf8(from.bytes.clone())",
                    "        .map_err(|e| store::PackError::Schema(e.to_string()))?;",
                    "    Ok(" + snap + " { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })",
                    "}",
                    "",
                    "/// 📥 Decode pack bytes via binary then UTF-8.",
                    "pub fn deserialize_bytes(bytes: &[u8]) -> Result<" + snap + ", store::PackError> {",
                    "    let binary = <BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?;",
                    "    deserialize(&binary)",
                    "}",
                    "//#endregion 🔖️Codec",
                    "",
                ]
            )
            sbody = "\n".join(
                [
                    "//! 📤️ Serialize `" + kind + "` to stdio.binary.",
                    "",
                    "use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};",
                    "use crate::artifacts::" + mid + "::{" + snap + "};",
                    "",
                    "//#region 🔖️Codec",
                    "/// 🗂️ Register serializer hooks.",
                    "pub fn register() {}",
                    "",
                    "/// � crit UTF-8 encode text into a BinarySnapshot.",
                    "pub fn serialize(from: &" + snap + ") -> BinarySnapshot {",
                    "    BinarySnapshot {",
                    "        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),",
                    "        bytes: from.text.as_bytes().to_vec(),",
                    "    }",
                    "}",
                    "",
                    "/// � crit Encode as binary pack bytes.",
                    "pub fn serialize_bytes(from: &" + snap + ") -> Result<Vec<u8>, store::PackError> {",
                    "    let binary = serialize(from);",
                    "    binary.encode_pack_with(&store::PackEncodeOptions::default())",
                    "}",
                    "//#endregion 🔖️Codec",
                    "",
                ]
            )
        else:
            dbody = "\n".join(
                [
                    "//! 📥️ Deserialize `" + kind + "` from stdio.txt.",
                    "",
                    "use crate::artifacts::txt::TxtSnapshot;",
                    "use crate::artifacts::" + mid + "::{" + snap + ", STDIO_JSON_DOCUMENT_SCHEMA};",
                    "",
                    "//#region 🔖️Codec",
                    "/// 🗂️ Register deserializer hooks.",
                    "pub fn register() {}",
                    "",
                    "/// 📥 Parse JSON text into a JsonSnapshot.",
                    "pub fn deserialize(from: &TxtSnapshot) -> Result<" + snap + ", store::TextError> {",
                    "    let value = serde_json::from_str(from.text.trim()).map_err(|e| {",
                    '        store::TextError::new(format!("json parse: {e}"), dsl::TextSpan::at(1, 1))',
                    "    })?;",
                    "    Ok(" + snap + " { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })",
                    "}",
                    "",
                    "/// 📥 Parse DSL/text via txt then JSON.",
                    "pub fn deserialize_text(text: &str) -> Result<" + snap + ", store::TextError> {",
                    "    let txt = <TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?;",
                    "    deserialize(&txt)",
                    "}",
                    "//#endregion 🔖️Codec",
                    "",
                ]
            )
            sbody = "\n".join(
                [
                    "//! � crit Serialize `" + kind + "` to stdio.txt.",
                    "",
                    "use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};",
                    "use crate::artifacts::" + mid + "::{" + snap + "};",
                    "",
                    "//#region 🔖️Codec",
                    "/// 🗂️ Register serializer hooks.",
                    "pub fn register() {}",
                    "",
                    "/// � crit Pretty-print JSON into a TxtSnapshot.",
                    "pub fn serialize(from: &" + snap + ") -> Result<TxtSnapshot, store::PackError> {",
                    "    let text = serde_json::to_string_pretty(&from.value)",
                    "        .map_err(|e| store::PackError::Schema(e.to_string()))?;",
                    "    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })",
                    "}",
                    "",
                    "/// � crit Encode as txt DSL text.",
                    "pub fn serialize_text(from: &" + snap + ") -> Result<String, store::PackError> {",
                    "    let txt = serialize(from)?;",
                    "    Ok(store::DocumentDsl::print_dsl(&txt))",
                    "}",
                    "//#endregion 🔖️Codec",
                    "",
                ]
            )

        assert "\ufffd" not in dbody
        assert "\ufffd" not in sbody
        (dpath / RS).write_text(dbody)
        (dpath / TS).write_text(
            "/** 📥️ deserialize " + kind + " via " + dep + ". */\nexport {};\n"
        )
        (spath / RS).write_text(sbody)
        (spath / TS).write_text(
            "/** 📤️ serialize " + kind + " via " + dep + ". */\nexport {};\n"
        )
    print("io", mid)
