#!/usr/bin/env python3
"""🎒️ R10 item 2 (row 1.10, oracle conflict `rust:zip`): the kernel's `.sxt` extension packages and the os host's space
collection export/import read and write ZIP through a first-party container (`semio_framework_deflate::zip_archive`,
over the crate's own raw DEFLATE) instead of the third-party `zip` crate, which stays only as that module's
dev-dependency oracle. Also removes the external type `zip::result::ZipError` from the public error enums
`ExtensionPackageError` and `SpaceZipError` (AGENTS.md: no exported API that requires a type outside this codebase).
Guest-linked tree → prepared patch, lands in window 3; idempotent.

New files are copied from `wp-r10/owned-zip/` (same repo-relative paths). Usage:
  python3 owned-zip.py [--apply] [--root <repo or overlay root>]
"""
import shutil
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
STAGED = HERE / "owned-zip"
NEW_FILES = [
    "🧰️framework/🔨️modules/🗜️deflate/🎒️zip/🦀️.rs",
    "🧰️framework/🔨️modules/🗜️deflate/🎒️zip/🧪️tests/🔬️unit/🦀️.rs",
    "🧰️framework/🔨️modules/🗜️deflate/🧫️fixtures/🎒️zip-archive-cases/🔣️.json",
]
DEFLATE = "🧰️framework/🔨️modules/🗜️deflate"
KERNEL = "🧰️framework/🛍️products/💻️os"
EXTENSION = f"{KERNEL}/🔨️modules/🧩️extension/🦀️.rs"
SPACE = f"{KERNEL}/🔨️modules/🪐️space/🦀️.rs"

EXTENSION_ERRORS_OLD = '''use std::collections::BTreeMap;
use std::io::{Cursor, Read, Seek, Write};

use crate::os_semio::{unwrap_binary, wrap_binary, Component, SemioEnvelope, SemioError};

//#region 🔖️Errors
/// ⚠️ Failures packing, unpacking, or verifying an `.sxt` extension package.
#[derive(Debug)]
pub enum ExtensionPackageError {
    Envelope(SemioError),
    UnexpectedEnvelope(String),
    Zip(zip::result::ZipError),
    Io(std::io::Error),
    ManifestJson(String),
    MissingEntry(String),
    InvalidPackageFormat(u16),
    EmptyComponent,
}

impl std::fmt::Display for ExtensionPackageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Envelope(error) => write!(formatter, "semio envelope error: {error}"),
            Self::UnexpectedEnvelope(envelope) => write!(formatter, "unexpected extension package envelope: {envelope}"),
            Self::Zip(error) => write!(formatter, "zip error: {error}"),
            Self::Io(error) => write!(formatter, "io error: {error}"),
            Self::ManifestJson(error) => write!(formatter, "manifest json error: {error}"),
            Self::MissingEntry(entry) => write!(formatter, "missing zip entry: {entry}"),
            Self::InvalidPackageFormat(version) => write!(formatter, "invalid package format version: {version}"),
            Self::EmptyComponent => formatter.write_str("empty component.wasm"),
        }
    }
}

impl std::error::Error for ExtensionPackageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Envelope(error) => Some(error),
            Self::Zip(error) => Some(error),
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SemioError> for ExtensionPackageError {
    fn from(error: SemioError) -> Self {
        Self::Envelope(error)
    }
}

impl From<zip::result::ZipError> for ExtensionPackageError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Zip(error)
    }
}

impl From<std::io::Error> for ExtensionPackageError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
'''
EXTENSION_ERRORS_NEW = '''use std::collections::BTreeMap;

use semio_framework_deflate::zip_archive::{ZipArchive, ZipArchiveError, ZipWriter};

use crate::os_semio::{unwrap_binary, wrap_binary, Component, SemioEnvelope, SemioError};

//#region 🔖️Errors
/// ⚠️ Failures packing, unpacking, or verifying an `.sxt` extension package.
#[derive(Debug)]
pub enum ExtensionPackageError {
    Envelope(SemioError),
    UnexpectedEnvelope(String),
    Zip(ZipArchiveError),
    ManifestJson(String),
    MissingEntry(String),
    InvalidPackageFormat(u16),
    EmptyComponent,
}

impl std::fmt::Display for ExtensionPackageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Envelope(error) => write!(formatter, "semio envelope error: {error}"),
            Self::UnexpectedEnvelope(envelope) => write!(formatter, "unexpected extension package envelope: {envelope}"),
            Self::Zip(error) => write!(formatter, "zip error: {error}"),
            Self::ManifestJson(error) => write!(formatter, "manifest json error: {error}"),
            Self::MissingEntry(entry) => write!(formatter, "missing zip entry: {entry}"),
            Self::InvalidPackageFormat(version) => write!(formatter, "invalid package format version: {version}"),
            Self::EmptyComponent => formatter.write_str("empty component.wasm"),
        }
    }
}

impl std::error::Error for ExtensionPackageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Envelope(error) => Some(error),
            Self::Zip(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SemioError> for ExtensionPackageError {
    fn from(error: SemioError) -> Self {
        Self::Envelope(error)
    }
}

impl From<ZipArchiveError> for ExtensionPackageError {
    fn from(error: ZipArchiveError) -> Self {
        match error {
            ZipArchiveError::MissingEntry(name) => Self::MissingEntry(name),
            other => Self::Zip(other),
        }
    }
}
'''

EXTENSION_ZIP_START = "//#region 🔖️Zip\nasync fn zip_file_options()"
EXTENSION_ZIP_END = "async fn expect_extension_envelope("
EXTENSION_ZIP_NEW = '''//#region 🔖️Zip
/// 📏️ The largest single entry an `.sxt` package may inflate to (a wasip2 component with its assets stays far below).
const MAX_PACKAGE_ENTRY_BYTES: usize = 256 * 1024 * 1024;

async fn build_zip_payload(manifest: &ExtensionPackageManifest, component_wasm: &[u8], assets: &[(String, Vec<u8>)]) -> Result<Vec<u8>, ExtensionPackageError> {
    if component_wasm.is_empty() {
        return Err(ExtensionPackageError::EmptyComponent);
    }
    if manifest.package_format != EXTENSION_PACKAGE_FORMAT {
        return Err(ExtensionPackageError::InvalidPackageFormat(manifest.package_format));
    }
    let mut writer = ZipWriter::new();
    writer.add(MANIFEST_ENTRY, crate::os_pack::json::to_string(&manifest.to_json()).as_bytes())?;
    writer.add(COMPONENT_ENTRY, component_wasm)?;
    let mut sorted_assets: Vec<&(String, Vec<u8>)> = assets.iter().collect();
    sorted_assets.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, bytes) in sorted_assets {
        let entry = if name.starts_with(ASSETS_PREFIX) { name.clone() } else { format!("{ASSETS_PREFIX}{name}") };
        writer.add(&entry, bytes)?;
    }
    Ok(writer.finish()?)
}

async fn parse_zip_payload(payload: &[u8]) -> Result<ExtensionPackage, ExtensionPackageError> {
    let archive = ZipArchive::parse(payload)?;
    let manifest_bytes = archive.read(MANIFEST_ENTRY, MAX_PACKAGE_ENTRY_BYTES)?;
    let manifest_json = crate::os_pack::json::parse_bytes(&manifest_bytes).map_err(|error| ExtensionPackageError::ManifestJson(error.to_string()))?;
    let manifest = ExtensionPackageManifest::from_json(&manifest_json).map_err(ExtensionPackageError::ManifestJson)?;
    if manifest.package_format != EXTENSION_PACKAGE_FORMAT {
        return Err(ExtensionPackageError::InvalidPackageFormat(manifest.package_format));
    }
    let component_wasm = archive.read(COMPONENT_ENTRY, MAX_PACKAGE_ENTRY_BYTES)?;
    if component_wasm.is_empty() {
        return Err(ExtensionPackageError::EmptyComponent);
    }
    let mut assets = BTreeMap::new();
    for entry in archive.entries() {
        if let Some(relative) = entry.name.strip_prefix(ASSETS_PREFIX).filter(|relative| !relative.is_empty() && !entry.name.ends_with('/')) {
            assets.insert(relative.to_string(), archive.read_entry(entry, MAX_PACKAGE_ENTRY_BYTES)?);
        }
    }
    Ok(ExtensionPackage { manifest, component_wasm, assets })
}

'''

SPACE_OLD_ERRORS = '''pub enum SpaceZipError {
    Zip(zip::result::ZipError),
    Io(std::io::Error),
    Pack(String),
    MissingPath(String),
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::fmt::Display for SpaceZipError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zip(error) => write!(formatter, "zip error: {error}"),
            Self::Io(error) => write!(formatter, "io error: {error}"),
            Self::Pack(detail) => write!(formatter, "pack error: {detail}"),
            Self::MissingPath(path) => write!(formatter, "missing path for entry {path}"),
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::error::Error for SpaceZipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Zip(error) => Some(error),
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl From<zip::result::ZipError> for SpaceZipError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Zip(error)
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl From<std::io::Error> for SpaceZipError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
'''
SPACE_NEW_ERRORS = '''pub enum SpaceZipError {
    Zip(semio_framework_deflate::zip_archive::ZipArchiveError),
    Pack(String),
    MissingPath(String),
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::fmt::Display for SpaceZipError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zip(error) => write!(formatter, "zip error: {error}"),
            Self::Pack(detail) => write!(formatter, "pack error: {detail}"),
            Self::MissingPath(path) => write!(formatter, "missing path for entry {path}"),
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::error::Error for SpaceZipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Zip(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl From<semio_framework_deflate::zip_archive::ZipArchiveError> for SpaceZipError {
    fn from(error: semio_framework_deflate::zip_archive::ZipArchiveError) -> Self {
        Self::Zip(error)
    }
}
'''
SPACE_OLD_HELPERS = '''#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn zip_file_options() -> zip::write::SimpleFileOptions {
    // 🕰️ Fixed (epoch) timestamp on every entry — the export→import→export byte-stability law
    // depends on nothing time-varying leaking into the zip's central directory.
    zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).last_modified_time(zip::DateTime::default())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn write_zip_file<W: std::io::Write + Seek>(writer: &mut zip::ZipWriter<W>, name: &str, bytes: &[u8], options: zip::write::SimpleFileOptions) -> Result<(), SpaceZipError> {
    writer.start_file(name, options)?;
    writer.write_all(bytes)?;
    Ok(())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn read_zip_entry<R: std::io::Read + Seek>(archive: &mut zip::ZipArchive<R>, name: &str) -> Result<Vec<u8>, SpaceZipError> {
    let mut file = archive.by_name(name)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}
'''
SPACE_NEW_HELPERS = '''/// 📏️ The largest single entry a collection archive may inflate to.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
const MAX_COLLECTION_ENTRY_BYTES: usize = 1024 * 1024 * 1024;
'''
SPACE_EDITS = [
    (SPACE_OLD_ERRORS, SPACE_NEW_ERRORS),
    (SPACE_OLD_HELPERS, SPACE_NEW_HELPERS),
    ("""    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip_file_options();

    write_zip_file(&mut writer, "collection.collection.pack", &store::ArtifactPack::encode_pack(collection), options)?;
    write_zip_file(&mut writer, "collection.collection.spr", collection_spr, options)?;
""", """    let mut writer = semio_framework_deflate::zip_archive::ZipWriter::new();
    writer.add("collection.collection.pack", &store::ArtifactPack::encode_pack(collection))?;
    writer.add("collection.collection.spr", collection_spr)?;
"""),
    ("""                write_zip_file(&mut writer, &format!("{path}.pack"), &pack_bytes, options)?;
                write_zip_file(&mut writer, &format!("{path}.spr"), &spr_bytes, options)?;""", """                writer.add(&format!("{path}.pack"), &pack_bytes)?;
                writer.add(&format!("{path}.spr"), &spr_bytes)?;"""),
    ("""                write_zip_file(&mut writer, &path, &bytes, options)?;""", """                writer.add(&path, &bytes)?;"""),
    ("""    let cursor = writer.finish()?;
    Ok(cursor.into_inner())
}""", """    Ok(writer.finish()?)
}"""),
    ("""    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
    let collection_pack = read_zip_entry(&mut archive, "collection.collection.pack")?;
    let collection_spr = read_zip_entry(&mut archive, "collection.collection.spr")?;""", """    let archive = semio_framework_deflate::zip_archive::ZipArchive::parse(bytes)?;
    let collection_pack = archive.read("collection.collection.pack", MAX_COLLECTION_ENTRY_BYTES)?;
    let collection_spr = archive.read("collection.collection.spr", MAX_COLLECTION_ENTRY_BYTES)?;"""),
    ("""                let pack_bytes = read_zip_entry(&mut archive, &format!("{path}.pack"))?;
                let spr_bytes = read_zip_entry(&mut archive, &format!("{path}.spr"))?;""", """                let pack_bytes = archive.read(&format!("{path}.pack"), MAX_COLLECTION_ENTRY_BYTES)?;
                let spr_bytes = archive.read(&format!("{path}.spr"), MAX_COLLECTION_ENTRY_BYTES)?;"""),
    ("""                let raw = read_zip_entry(&mut archive, &path)?;""", """                let raw = archive.read(&path, MAX_COLLECTION_ENTRY_BYTES)?;"""),
]

MANIFEST_EDITS = {
    f"{DEFLATE}/📦️packages/🦀️rust/Cargo.toml": [
        ('[dev-dependencies]\nminiz_oxide = "0.8"\n', '[dev-dependencies]\nminiz_oxide = "0.8"\nzip = { version = "2.4", default-features = false, features = ["deflate"] }\n'),
    ],
    f"{KERNEL}/📦️packages/🦀️rust/Cargo.toml": [
        ("`zip` backs\n# `🧩️extension`'s `.sxt` package pack/unpack/verify", "`semio-framework-deflate` (its first-party\n# `zip_archive`) backs `🧩️extension`'s `.sxt` package pack/unpack/verify"),
        ('zip = { version = "2.4", default-features = false, features = ["deflate"] }\n', 'semio-framework-deflate = { path = "../../../../🔨️modules/🗜️deflate/📦️packages/🦀️rust" }\n'),
    ],
    f"{KERNEL}/🖥️host/📦️packages/🦀️rust/Cargo.toml": [
        ("# 🗜️ `zip` moved from an unconditional optional dependency: its only production consumer is", "# 🗜️ `semio-framework-deflate` (its first-party `zip_archive`) is optional: its only production consumer is"),
        ('zip = { version = "2.4", default-features = false, features = ["deflate"], optional = true }\n', 'semio-framework-deflate = { path = "../../../../../🔨️modules/🗜️deflate/📦️packages/🦀️rust", optional = true }\n'),
        ('"dep:zip"', '"dep:semio-framework-deflate"'),
    ],
}


def edited(text, edits, label):
    for old, new in edits:
        if new in text and old not in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"{label}: anchor found {text.count(old)} times: {old[:70]!r}")
        text = text.replace(old, new)
    return text


def main():
    apply = "--apply" in sys.argv
    writes = {}
    deflate_lib = ROOT / DEFLATE / "🦀️.rs"
    lib = deflate_lib.read_text(encoding="utf-8")
    anchor = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;'
    module = '#[path = "🎒️zip/🦀️.rs"]\npub mod zip_archive;\n\n'
    if module not in lib:
        if lib.count(anchor) != 1:
            raise SystemExit("deflate lib test anchor not unique")
        lib = lib.replace(anchor, module + anchor)
    writes[deflate_lib] = lib
    extension = (ROOT / EXTENSION).read_text(encoding="utf-8")
    extension = edited(extension, [(EXTENSION_ERRORS_OLD, EXTENSION_ERRORS_NEW)], "extension errors")
    if EXTENSION_ZIP_NEW not in extension:
        start, end = extension.index(EXTENSION_ZIP_START), extension.index(EXTENSION_ZIP_END)
        extension = extension[:start] + EXTENSION_ZIP_NEW + extension[end:]
    writes[ROOT / EXTENSION] = extension
    space = edited((ROOT / SPACE).read_text(encoding="utf-8"), SPACE_EDITS, "space")
    writes[ROOT / SPACE] = space
    for relative, edits in MANIFEST_EDITS.items():
        writes[ROOT / relative] = edited((ROOT / relative).read_text(encoding="utf-8"), edits, relative)
    survivors = [str(path.relative_to(ROOT)) for path, text in writes.items() if path.suffix == ".rs" and ("zip::" in text.replace("zip_archive::", "") or "ZipWriter<" in text)]
    if survivors:
        raise SystemExit(f"third-party zip use survives in {survivors}")
    changed = [str(path.relative_to(ROOT)) for path, text in writes.items() if path.read_text(encoding="utf-8") != text]
    missing = [relative for relative in NEW_FILES if not (ROOT / relative).exists() or (ROOT / relative).read_bytes() != (STAGED / relative).read_bytes()]
    print(f"edits: {len(changed)} files {changed}; new files to write: {len(missing)}")
    if not apply:
        print("dry run clean")
        return
    for path, text in writes.items():
        path.write_text(text, encoding="utf-8")
    for relative in missing:
        (ROOT / relative).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(STAGED / relative, ROOT / relative)
    print("applied")


main()
