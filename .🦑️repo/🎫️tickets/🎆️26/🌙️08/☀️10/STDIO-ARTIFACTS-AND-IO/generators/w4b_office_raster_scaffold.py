#!/usr/bin/env python3
"""W4b office+raster: scaffold png/jpg/gif/tiff/pdf/docx/pptx/xlsx/bcf/glb from zip tree."""
from __future__ import annotations

import json
import shutil
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]
ZIP_DIR = PLUGIN / "🗿️artifacts" / ROSTER["zip"]["dir"]
BINARY_DIR = ROSTER["binary"]["dir"]
DEFLATE_DIR = ROSTER["deflate"]["dir"]
XML_DIR = ROSTER["xml"]["dir"]
JSON_DIR = ROSTER["json"]["dir"]
DESER = TOKENS["deserializers"]
SER = TOKENS["serializers"]

# mid, emoji_dir, Name, example, io: binary | binary_deflate | zip_xml | binary_json
ARTIFACTS = [
    ("png", ROSTER["png"]["dir"], "Png", "example.png", "binary_deflate"),
    ("jpg", ROSTER["jpg"]["dir"], "Jpg", "example.jpg", "binary"),
    ("gif", ROSTER["gif"]["dir"], "Gif", "example.gif", "binary"),
    ("tiff", ROSTER["tiff"]["dir"], "Tiff", "example.tiff", "binary"),
    ("pdf", ROSTER["pdf"]["dir"], "Pdf", "example.pdf", "binary_deflate"),
    ("docx", ROSTER["docx"]["dir"], "Docx", "example.docx", "zip_xml"),
    ("pptx", ROSTER["pptx"]["dir"], "Pptx", "example.pptx", "zip_xml"),
    ("xlsx", ROSTER["xlsx"]["dir"], "Xlsx", "example.xlsx", "zip_xml"),
    ("bcf", ROSTER["bcf"]["dir"], "Bcf", "example.bcf", "zip_xml"),
    ("glb", ROSTER["glb"]["dir"], "Glb", "example.glb", "binary_json"),
]


def replace_zip(text: str, mid: str, emoji_dir: str, name: str) -> str:
    pairs = [
        ("🎒️zip", emoji_dir),
        ("stdio.zip", f"stdio.{mid}"),
        ("s.stdio.zip", f"s.stdio.{mid}"),
        ("s.stdio.zip.diff", f"s.stdio.{mid}.diff"),
        ("ZipArtifact", f"{name}Artifact"),
        ("ZipSnapshot", f"{name}Snapshot"),
        ("ZipDiff", f"{name}Diff"),
        ("ZipMutation", f"{name}Mutation"),
        ("ZipEngine", f"{name}Engine"),
        ("ZipBuilder", f"{name}Builder"),
        ("ZipDecomposer", f"{name}Decomposer"),
        ("ZipParts", f"{name}Parts"),
        ("ZipEntry", f"{name}Entry"),
        ("zip_artifact_schema_descriptor", f"{mid}_artifact_schema_descriptor"),
        ("apply_zip_mutation", f"apply_{mid}_mutation"),
        ("empty_zip_snapshot", f"empty_{mid}_snapshot"),
        ("STDIO_ZIP_DOCUMENT_SCHEMA", f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"),
        ("ZIP_ARTIFACT_SCHEMA_ID", f"{mid.upper()}_ARTIFACT_SCHEMA_ID"),
        ("crate::artifacts::zip", f"crate::artifacts::{mid}"),
        ("artifacts::zip", f"artifacts::{mid}"),
        ("pub mod zip", f"pub mod {mid}"),
        ("Zip", name),
        ("zip", mid),
    ]
    for old, new in pairs:
        text = text.replace(old, new)
    return text


def fix_io_register(dst: Path, mid: str, name: str, io_mode: str) -> None:
    io_rs = dst / "🚪️io/🦀️component.rs"
    if io_mode == "binary":
        io_rs.write_text(
            f"""//! IO stdio.{mid}
//#region Register
pub fn register() {{
    crate::artifacts::{mid}::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::{mid}::io::export::serializers::artifacts::binary::register();
}}
//#endregion Register
""",
            encoding="utf-8",
        )
        for sub in ("import", "export"):
            d = DEFLATE_DIR
            path = dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{d}" if sub == "import" else dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{d}"
            if path.exists():
                shutil.rmtree(path)
    elif io_mode == "binary_deflate":
        io_rs.write_text(
            f"""//! IO stdio.{mid}
//#region Register
pub fn register() {{
    crate::artifacts::{mid}::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::{mid}::io::import::deserializers::artifacts::deflate::register();
    crate::artifacts::{mid}::io::export::serializers::artifacts::binary::register();
    crate::artifacts::{mid}::io::export::serializers::artifacts::deflate::register();
}}
//#endregion Register
""",
            encoding="utf-8",
        )
    elif io_mode == "zip_xml":
        zd = ROSTER["zip"]["dir"]
        bin_imp = dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{BINARY_DIR}"
        bin_ser = dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{BINARY_DIR}"
        zip_imp = dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{zd}"
        zip_ser = dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{zd}"
        xml_imp = dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{XML_DIR}"
        xml_ser = dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{XML_DIR}"
        for d in (zip_imp, zip_ser, xml_imp, xml_ser):
            if d.exists():
                shutil.rmtree(d)
        shutil.copytree(bin_imp, zip_imp)
        shutil.copytree(bin_ser, zip_ser)
        xml_ref_imp = PLUGIN / "🗿️artifacts" / ROSTER["xml"]["dir"] / f"🚪️io/📥️import/{DESER}/🗿️artifacts/📄txt"
        xml_ref_ser = PLUGIN / "🗿️artifacts" / ROSTER["xml"]["dir"] / f"🚪️io/📤️export/{SER}/🗿️artifacts/📄txt"
        shutil.copytree(xml_ref_imp, xml_imp)
        shutil.copytree(xml_ref_ser, xml_ser)
        for path in list(zip_imp.rglob("🦀️component.rs")) + list(zip_ser.rglob("🦀️component.rs")):
            t = path.read_text(encoding="utf-8")
            t = t.replace(f"crate::artifacts::{mid}::engine::decode_zip", "crate::artifacts::zip::engine::decode_zip")
            t = t.replace(f"crate::artifacts::{mid}::engine::encode_zip", "crate::artifacts::zip::engine::encode_zip")
            path.write_text(t, encoding="utf-8")
        for path in list(xml_imp.rglob("🦀️component.rs")) + list(xml_ser.rglob("🦀️component.rs")):
            t = path.read_text(encoding="utf-8")
            t = t.replace("crate::artifacts::xml", f"crate::artifacts::{mid}")
            t = t.replace("XmlSnapshot", f"{name}Snapshot")
            t = t.replace("STDIO_XML_DOCUMENT_SCHEMA", f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA")
            path.write_text(t, encoding="utf-8")
        for d in (
            dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{DEFLATE_DIR}",
            dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{DEFLATE_DIR}",
            bin_imp,
            bin_ser,
        ):
            if d.exists():
                shutil.rmtree(d)
        io_rs.write_text(
            f"""//! IO stdio.{mid}
//#region Register
pub fn register() {{
    crate::artifacts::{mid}::io::import::deserializers::artifacts::zip::register();
    crate::artifacts::{mid}::io::import::deserializers::artifacts::xml::register();
    crate::artifacts::{mid}::io::export::serializers::artifacts::zip::register();
    crate::artifacts::{mid}::io::export::serializers::artifacts::xml::register();
}}
//#endregion Register
""",
            encoding="utf-8",
        )
    elif io_mode == "binary_json":
        json_imp = dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{JSON_DIR}"
        json_ser = dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{JSON_DIR}"
        bin_imp = dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{BINARY_DIR}"
        for d in (json_imp, json_ser):
            if d.exists():
                shutil.rmtree(d)
        shutil.copytree(bin_imp, json_imp)
        shutil.copytree(dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{BINARY_DIR}", json_ser)
        for path in list(json_imp.rglob("*")) + list(json_ser.rglob("*")):
            if path.suffix in (".rs", ".ts"):
                t = path.read_text(encoding="utf-8")
                t = t.replace(BINARY_DIR, JSON_DIR)
                t = t.replace("::binary::", "::json::")
                t = t.replace("BinarySnapshot", "JsonSnapshot")
                t = t.replace("STDIO_BINARY_DOCUMENT_SCHEMA", "STDIO_JSON_DOCUMENT_SCHEMA")
                t = t.replace("artifacts::binary", "artifacts::json")
                path.write_text(t, encoding="utf-8")
        for d in (dst / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{DEFLATE_DIR}", dst / f"🚪️io/📤️export/{SER}/🗿️artifacts/{DEFLATE_DIR}"):
            if d.exists():
                shutil.rmtree(d)
        io_rs.write_text(
            f"""//! IO stdio.{mid}
//#region Register
pub fn register() {{
    crate::artifacts::{mid}::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::{mid}::io::import::deserializers::artifacts::json::register();
    crate::artifacts::{mid}::io::export::serializers::artifacts::binary::register();
    crate::artifacts::{mid}::io::export::serializers::artifacts::json::register();
}}
//#endregion Register
""",
            encoding="utf-8",
        )


def scaffold_one(mid: str, emoji_dir: str, name: str, example: str, io_mode: str) -> None:
    dst = PLUGIN / "🗿️artifacts" / emoji_dir
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(ZIP_DIR, dst)
    for path in dst.rglob("*"):
        if not path.is_file():
            continue
        if path.suffix in (
            ".rs", ".ts", ".json", ".semio", ".proto", ".graphql",
            ".ebnf", ".g4", ".abnf", ".ksy", ".spicy",
        ) or path.name.endswith(".dsl.semio"):
            raw = path.read_text(encoding="utf-8")
            path.write_text(replace_zip(raw, mid, emoji_dir, name), encoding="utf-8")
    old = dst / "📚️examples/🎬️demo/🖼️assets/🎒️example.zip"
    if old.exists():
        old.rename(old.parent / example)
    fix_io_register(dst, mid, name, io_mode)
    print("scaffolded", emoji_dir, io_mode)


for row in ARTIFACTS:
    scaffold_one(*row)
