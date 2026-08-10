#!/usr/bin/env python3
"""Force-implement deflate+zip from binary template."""
from __future__ import annotations

import json
import shutil
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]
ART = PLUGIN / "🗿️artifacts"
BIN_DIR = ROSTER["binary"]["dir"]
DEF_DIR = ROSTER["deflate"]["dir"]
ZIP_DIR = ROSTER["zip"]["dir"]
DESER = TOKENS["deserializers"]
SER = TOKENS["serializers"]
SRC = ART / BIN_DIR

TEXT_SUFFIXES = {
    ".rs", ".ts", ".json", ".semio", ".proto", ".graphql",
    ".ebnf", ".g4", ".abnf", ".ksy", ".spicy",
}


def replace_ident(text: str, mid: str, name: str, emoji: str) -> str:
    pairs = [
        (BIN_DIR, emoji),
        ("stdio.binary", f"stdio.{mid}"),
        ("s.stdio.binary", f"s.stdio.{mid}"),
        ("BinaryArtifact", f"{name}Artifact"),
        ("BinarySnapshot", f"{name}Snapshot"),
        ("BinaryDiff", f"{name}Diff"),
        ("BinaryMutation", f"{name}Mutation"),
        ("BinaryEngine", f"{name}Engine"),
        ("BinaryBuilder", f"{name}Builder"),
        ("BinaryDecomposer", f"{name}Decomposer"),
        ("BinaryParts", f"{name}Parts"),
        ("binary_artifact_schema_descriptor", f"{mid}_artifact_schema_descriptor"),
        ("apply_binary_mutation", f"apply_{mid}_mutation"),
        ("empty_binary_snapshot", f"empty_{mid}_snapshot"),
        ("STDIO_BINARY_DOCUMENT_SCHEMA", f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"),
        ("BINARY_ARTIFACT_SCHEMA_ID", f"{mid.upper()}_ARTIFACT_SCHEMA_ID"),
        ("crate::artifacts::binary", f"crate::artifacts::{mid}"),
        ("artifacts::binary", f"artifacts::{mid}"),
        ('name: "Binary".into()', f'name: "{name}".into()'),
        ("Binary", name),
        ("binary", mid),
    ]
    for old, new in pairs:
        text = text.replace(old, new)
    return text


def scaffold_from_binary(mid: str, emoji: str, name: str, example: str) -> Path:
    dst = ART / emoji
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(SRC, dst)
    for path in dst.rglob("*"):
        if path.is_file() and (path.suffix in TEXT_SUFFIXES or ".dsl.semio" in path.name):
            raw = path.read_text(encoding="utf-8")
            path.write_text(replace_ident(raw, mid, name, emoji), encoding="utf-8")
    assets = dst / "📚️examples/🎬️demo/🖼️assets"
    if assets.exists():
        for p in list(assets.iterdir()):
            if p.suffix == ".bin" or p.name.endswith(".bin"):
                target = assets / example
                p.rename(target)
                target.write_bytes(b"")
    return dst


def retarget_io_to_binary(dst: Path, mid: str, name: str) -> None:
    for direction, folder in [("📥️import", DESER), ("📤️export", SER)]:
        base = dst / "🚪️io" / direction / folder / "🗿️artifacts"
        self_leaf = base / dst.name
        bin_leaf = base / BIN_DIR
        if self_leaf.exists():
            if bin_leaf.exists():
                shutil.rmtree(bin_leaf)
            self_leaf.rename(bin_leaf)
        for path in bin_leaf.rglob("*"):
            if path.is_file() and path.suffix in {".rs", ".ts"}:
                text = path.read_text(encoding="utf-8")
                text = text.replace(f"crate::artifacts::{mid}::", "TEMP_PEER::")
                text = text.replace(f"{name}Snapshot", "PEER_SNAP")
                text = text.replace(f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA", "PEER_SCHEMA")
                # restore our own types first where needed — IO files mix both
                path.write_text(text, encoding="utf-8")


def finalize_deflate_io(dst: Path) -> None:
    """deflate IO: binary peer + our DeflateSnapshot."""
    # import: BinarySnapshot -> DeflateSnapshot (compress)
    imp = dst / f"🚪️io/�    # import: BinarySnapshot -> DeflateSnapshot (compress)
    imp = dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{BIN_DIR}/🦀️component.rs"
    exp = dst / f"🚪️io/�<|control37|>export/{SER}/🗿️artifacts/{BIN_DIR}/🦀️component.rs"
    # fix typo path
    exp = dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{BIN_DIR}/🦀️component.rs"
    imp.write_text(
        """//! Deserialize stdio.deflate from stdio.binary (zlib-compress payload).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub fn register() {}

/// 🗜️ Zlib-compress binary payload into a DeflateSnapshot.
pub fn deserialize(from: &BinarySnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let bytes = crate::artifacts::deflate::engine::zlib_compress(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Decode a Binary pack then zlib-compress.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DeflateSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
//#endregion Codec
""",
        encoding="utf-8",
    )
    exp.write_text(
        """//! Serialize stdio.deflate to stdio.binary (zlib-inflate payload).

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::deflate::DeflateSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// 🗜️ Zlib-inflate deflate stream into a BinarySnapshot payload.
pub fn serialize(from: &DeflateSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::deflate::engine::zlib_decompress(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot {
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Inflate then encode as binary pack bytes.
pub fn serialize_bytes(from: &DeflateSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::DocumentPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
""",
        encoding="utf-8",
    )
    # TS stubs
    for side, folder in [("📥️import", DESER), ("📤️export", SER)]:
        ts = dst / f"🚪️io/{side}/{folder}/�import", DESER), ("📤️export", SER)]:
        ts = dst / f"🚪️io/{side}/{folder}/🗿️artifacts/{BIN_DIR}/🟦️component.ts"
        ts.write_text("/** IO bridge stdio.deflate ↔ stdio.binary */\nexport {};\n", encoding="utf-8")


def finalize_zip_io(dst: Path) -> None:
    """zip IO: binary + deflate peers."""
    # binary import: parse zip bytes
    (dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{BIN_DIR}/🦀️component.rs").write_text(
        """//! Deserialize stdio.zip from stdio.binary (parse ZIP bytes).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub fn register() {}

/// 🎒️ Parse ZIP container bytes into a ZipSnapshot.
pub fn deserialize(from: &BinarySnapshot) -> Result<ZipSnapshot, store::PackError> {
    let mut snap = crate::artifacts::zip::engine::decode_zip(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_ZIP_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<ZipSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
//#endregion Codec
""",
        encoding="utf-8",
    )
    (dst / f"🚪️io/�<|control37|>export/{SER}/🗿️artifacts/{BIN_DIR}/🦀️component.rs").write_text("x", encoding="utf-8")
    (dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{BIN_DIR}/🦀️component.rs").write_text(
        """//! Serialize stdio.zip to stdio.binary (encode ZIP bytes).

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::zip::ZipSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// 🎒️ Encode ZipSnapshot as ZIP container bytes.
pub fn serialize(from: &ZipSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::zip::engine::encode_zip(from, true)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot {
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Encode ZIP then wrap as binary pack bytes.
pub fn serialize_bytes(from: &ZipSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::DocumentPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
""",
        encoding="utf-8",
    )
    # deflate leaves
    def_imp = dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{DEF_DIR}"
    def_exp = dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{DEF_DIR}"
    for d in (def_imp, def_exp):
        d.mkdir(parents=True, exist_ok=True)
    (def_imp / "🦀️component.rs").write_text(
        """//! Deserialize stdio.zip from stdio.deflate (inflate then parse ZIP).

use crate::artifacts::deflate::DeflateSnapshot;
use crate::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub fn register() {}

/// 🎒️ Inflate zlib stream then parse ZIP.
pub fn deserialize(from: &DeflateSnapshot) -> Result<ZipSnapshot, store::PackError> {
    let payload = crate::artifacts::deflate::engine::zlib_decompress(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    let mut snap = crate::artifacts::zip::engine::decode_zip(&payload)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_ZIP_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode deflate pack then parse.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<ZipSnapshot, store::PackError> {
    deserialize(&<DeflateSnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
//#endregion Codec
""",
        encoding="utf-8",
    )
    (def_exp / "🦀️component.rs").write_text(
        """//! Serialize stdio.zip to stdio.deflate (encode ZIP then zlib-compress).

use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::artifacts::zip::ZipSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// 🎒️ Encode ZIP bytes then zlib-compress via deflate artifact.
pub fn serialize(from: &ZipSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let zip_bytes = crate::artifacts::zip::engine::encode_zip(from, true)
        .map_err(|e| store::PackError::Schema(e))?;
    let bytes = crate::artifacts::deflate::engine::zlib_compress(&zip_bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Encode as deflate pack bytes.
pub fn serialize_bytes(from: &ZipSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::DocumentPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
""",
        encoding="utf-8",
    )
    for side, folder, peer in [
        ("📥️import", DESER, BIN_DIR),
        ("📤️export", SER, BIN_DIR),
        ("📥️import", DESER, DEF_DIR),
        ("📤️export", SER, DEF_DIR),
    ]:
        ts = dst / f"�import", DESER, DEF_DIR),
        ("📤️export", SER, DEF_DIR),
    ]:
        ts = dst / f"🚪️io/{side}/{folder}/🗿️artifacts/{peer}/🟦️component.ts"
        ts.parent.mkdir(parents=True, exist_ok=True)
        ts.write_text("/** IO bridge stdio.zip */\nexport {};\n", encoding="utf-8")


def write_zip_io_register(dst: Path) -> None:
    (dst / "🚪️io/🦀️component.rs").write_text(
        """//! IO stdio.zip
pub fn register() {
    crate::artifacts::zip::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::zip::io::import::deserializers::artifacts::deflate::register();
    crate::artifacts::zip::io::export::serializers::artifacts::binary::register();
    crate::artifacts::zip::io::export::serializers::artifacts::deflate::register();
}
""",
        encoding="utf-8",
    )


def main() -> None:
    print("scaffold deflate")
    d = scaffold_from_binary("deflate", DEF_DIR, "Deflate", "🗜️example.zz")
    retarget_io_to_binary(d, "deflate", "Deflate")
    finalize_deflate_io(d)
    print("deflate files", sum(1 for p in d.rglob("*") if p.is_file()))

    print("scaffold zip")
    z = scaffold_from_binary("zip", ZIP_DIR, "Zip", "🎒️example.zip")
    retarget_io_to_binary(z, "zip", "Zip")
    finalize_zip_io(z)
    write_zip_io_register(z)
    print("zip files", sum(1 for p in z.rglob("*") if p.is_file()))


if __name__ == "__main__":
    main()
