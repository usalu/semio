//! 🧬️ Shared regeneration of committed norm assets: one switch, one rule for every Eurocode artifact. Without the switch the helpers assert byte equality; with `SEMIO_UPDATE_GOLDEN=1` they rewrite the file.
//!
//! @see <https://doc.rust-lang.org/std/env/fn.var.html>

use std::path::{Path, PathBuf};

/// 🚩️ Environment switch shared by every bless pass (scene goldens, assets, packs, SQLite contract numbers).
pub const SWITCH: &str = "SEMIO_UPDATE_GOLDEN";

/// 🎒️ File name of the binary pack that sits next to a `.dsl.semio` asset.
pub const PACK_FILE: &str = "🎒️.pack.semio";

/// 🔦️ Whether the switch is set to `1`.
#[must_use]
pub fn blessing() -> bool {
    std::env::var(SWITCH).ok().as_deref() == Some("1")
}

/// ⚖️ Rewrites `path` with `expected` under the switch, otherwise asserts that the committed bytes are equal.
///
/// # Panics
/// When the file differs and the switch is off, or when it cannot be read or written.
#[track_caller]
pub fn bless_or_assert(path: &Path, expected: &[u8]) {
    let committed = std::fs::read(path).ok();
    if committed.as_deref() == Some(expected) {
        return;
    }
    if blessing() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|error| panic!("{}: {error}", parent.display()));
        }
        std::fs::write(path, expected).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        return;
    }
    let state = committed.map_or_else(|| "missing".to_owned(), |bytes| format!("{} bytes, expected {}", bytes.len(), expected.len()));
    panic!("{} is stale ({state}); regenerate it with {SWITCH}=1 and run the regen tests", path.display());
}

/// 🔎️ Every file below `dir`, depth first and sorted.
fn files_below(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir).unwrap_or_else(|error| panic!("{}: {error}", dir.display())).map(|entry| entry.expect("directory entry").path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            files_below(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// 📦️ Regenerates, for every `.dsl.semio` below `assets`, the canonical DSL text and the pack beside it.
///
/// `codec` maps the committed DSL text to its canonical text and its pack bytes through the artifact's own codecs. A pack is a pure function of its DSL. A pack file that is not named [`PACK_FILE`] always fails, because blessing would otherwise leave two packs next to one DSL. Packs are only rewritten where one is committed; protocol fixtures (`.op`, `.spr`, `.cmd`) are not generated here.
///
/// # Panics
/// When a DSL does not decode, a pack is misnamed, or (switch off) a file is stale.
#[track_caller]
pub fn bless_dsl_and_pack_assets(assets: &Path, codec: &dyn Fn(&str) -> Result<(String, Vec<u8>), String>) -> usize {
    let mut files = Vec::new();
    files_below(assets, &mut files);
    let name_of = |path: &Path| path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
    let misnamed: Vec<String> = files.iter().filter(|path| name_of(path).ends_with("pack.semio") && name_of(path) != PACK_FILE).map(|path| path.display().to_string()).collect();
    assert!(misnamed.is_empty(), "packs must be named {PACK_FILE}: {misnamed:?}");
    let mut checked = 0;
    for dsl_path in files.iter().filter(|path| name_of(path).ends_with("dsl.semio")) {
        let text = std::fs::read_to_string(dsl_path).unwrap_or_else(|error| panic!("{}: {error}", dsl_path.display()));
        let (canonical, pack) = codec(&text).unwrap_or_else(|error| panic!("{}: the DSL does not decode: {error}", dsl_path.display()));
        bless_or_assert(dsl_path, canonical.as_bytes());
        let pack_path = dsl_path.with_file_name(PACK_FILE);
        if pack_path.is_file() {
            bless_or_assert(&pack_path, &pack);
        }
        checked += 1;
    }
    assert!(checked > 0, "no .dsl.semio asset below {}", assets.display());
    checked
}

/// 📏️ What the independent SQLite reader measured on one document: the values behind one `🎛️semantic.json` case.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticExtent {
    pub schema_bytes: u64,
    pub table_widths: Vec<(String, u64)>,
    pub rows: u64,
    pub value_bytes: u64,
}

/// 🧾️ The `🎛️semantic.json` text for the measured cases: schema bytes and table widths (equal in every case) and rows and value bytes per case id.
///
/// # Panics
/// When no case is given or two cases disagree on the schema.
#[must_use]
#[track_caller]
pub fn semantic_contract_text(cases: &[(&str, SemanticExtent)]) -> String {
    let (_, first) = cases.first().expect("at least one case is needed");
    let mut widths = first.table_widths.clone();
    widths.sort();
    for (id, extent) in cases {
        let mut other = extent.table_widths.clone();
        other.sort();
        assert!(extent.schema_bytes == first.schema_bytes && other == widths, "case {id} measured a different schema than the first case");
    }
    let escape = |text: &str| text.replace('\\', "\\\\").replace('"', "\\\"");
    let widths = widths.iter().map(|(table, width)| format!("    \"{}\": {width}", escape(table))).collect::<Vec<_>>().join(",\n");
    let rows = cases.iter().map(|(id, extent)| format!("    {{\n      \"id\": \"{}\",\n      \"rows\": {},\n      \"valueBytes\": {}\n    }}", escape(id), extent.rows, extent.value_bytes)).collect::<Vec<_>>().join(",\n");
    format!("{{\n  \"schemaBytes\": {},\n  \"tableWidths\": {{\n{widths}\n  }},\n  \"cases\": [\n{rows}\n  ]\n}}\n", first.schema_bytes)
}

/// 🎛️ Regenerates (switch) or asserts (default) the SQLite contract numbers from the measured cases.
#[track_caller]
pub fn bless_semantic_contract(path: &Path, cases: &[(&str, SemanticExtent)]) {
    bless_or_assert(path, semantic_contract_text(cases).as_bytes());
}
