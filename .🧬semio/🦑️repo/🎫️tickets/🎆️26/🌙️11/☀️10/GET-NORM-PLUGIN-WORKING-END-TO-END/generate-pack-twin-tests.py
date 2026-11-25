"""🗜️ Writes the pack/JSON twin equality test into every Norm family examples crate (except en1999, which has its own), mounts it and adds the io dev-dependency where missing."""
import re, sys
from pathlib import Path
A = Path(__file__).resolve().parents[7] / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any"
TEMPLATE = '''//! 🗜️ Every committed pack and JSON twin of an EN example asset equals the document of the DSL of the same example.

use semio_s_artifact_norm_FAM_io::binary::snapshot::{decode_FAM_pack, encode_FAM_pack};
use semio_s_artifact_norm_FAM_io::text::snapshot::{decode_FAM_dsl, decode_FAM_snapshot_json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// 🗂️ Every file below `dir`, depth first and sorted, so failures read in a stable order.
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir).expect("the assets directory is readable").map(|entry| entry.expect("directory entry").path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// 🔎️ The assets whose file name ends with `suffix`, paired with their example id (the directory name without its leading emoji).
fn assets_ending(suffix: &str) -> Vec<(String, PathBuf)> {
    let mut files = Vec::new();
    walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🖼️assets"), &mut files);
    files
        .into_iter()
        .filter(|path| path.file_name().is_some_and(|name| name.to_string_lossy().ends_with(suffix)))
        .map(|path| {
            let id = path.parent().and_then(Path::file_name).expect("an asset sits in an example directory").to_string_lossy().trim_start_matches(|c: char| !c.is_ascii_alphanumeric()).to_owned();
            (id, path)
        })
        .collect()
}

/// 🗜️ The pack and JSON twins decode to the document of their DSL, a pack prints back to the identical bytes, and no example carries two packs or two JSON twins; with `SEMIO_UPDATE_GOLDEN=1` every pack is first rewritten from its DSL document, and every violation is reported, not only the first.
#[test]
fn every_pack_and_json_twin_equals_the_dsl_document_of_its_example() {
    let documents: Vec<_> = assets_ending("dsl.semio")
        .into_iter()
        .map(|(id, path)| {
            let text = std::fs::read_to_string(&path).expect("the DSL asset is utf-8 text");
            let document = decode_FAM_dsl(&text).unwrap_or_else(|error| panic!("{id}: the bundled DSL does not decode: {error}"));
            (id, document)
        })
        .collect();
    assert!(!documents.is_empty());
    let document_of = |id: &str| documents.iter().find(|(known, _)| known == id).map(|(_, document)| document).unwrap_or_else(|| panic!("{id}: a pack or JSON asset without a DSL asset is orphaned"));
    let packs = assets_ending("pack.semio");
    assert_eq!(packs.iter().map(|(id, _)| id).collect::<BTreeSet<_>>().len(), packs.len(), "an example carries more than one pack");
    let blessing = std::env::var("SEMIO_UPDATE_GOLDEN").ok().as_deref() == Some("1");
    let mut findings = Vec::new();
    for (id, path) in packs {
        let canonical = encode_FAM_pack(document_of(&id));
        if blessing {
            std::fs::write(&path, &canonical).expect("the pack asset is writable");
        }
        let bytes = std::fs::read(&path).expect("the pack asset is readable");
        match decode_FAM_pack(&bytes) {
            Ok(decoded) if &decoded == document_of(&id) => {}
            Ok(_) => findings.push(format!("{id}: the pack is stale against its DSL")),
            Err(error) => findings.push(format!("{id}: the pack does not decode: {error}")),
        }
        if canonical != bytes {
            findings.push(format!("{id}: the pack is not the canonical print of its document"));
        }
    }
    let jsons = assets_ending(".json");
    assert_eq!(jsons.iter().map(|(id, _)| id).collect::<BTreeSet<_>>().len(), jsons.len(), "an example carries more than one JSON twin");
    for (id, path) in jsons {
        let text = std::fs::read_to_string(&path).expect("the JSON asset is utf-8 text");
        match decode_FAM_snapshot_json(&text) {
            Ok(decoded) if &decoded == document_of(&id) => {}
            Ok(_) => findings.push(format!("{id}: the JSON twin is stale against its DSL")),
            Err(error) => findings.push(format!("{id}: the JSON twin does not decode: {error}")),
        }
    }
    assert!(findings.is_empty(), "{} twin findings:\n{}", findings.len(), findings.join("\n"));
}
'''
for fam_dir in sorted(A.iterdir()):
    fam = re.sub(r"^\D+?(?=[a-z])", "", fam_dir.name)
    fam = fam_dir.name.lstrip("".join(c for c in fam_dir.name if not c.isascii()))
    if fam == "en1999":
        continue
    ex = fam_dir / ANY / "📚️examples"
    test = ex / "🧪️tests/🗜️pack-twins/🦀️.rs"
    test.parent.mkdir(parents=True, exist_ok=True)
    test.write_text(TEMPLATE.replace("FAM", fam))
    lib = ex / "🦀️.rs"
    text = lib.read_text()
    mount = '#[cfg(test)]\n#[path = "🧪️tests/🗜️pack-twins/🦀️.rs"]\nmod pack_twins;\n'
    if mount not in text:
        text = text.replace("//#endregion 🪢️Mounts", mount + "//#endregion 🪢️Mounts")
        lib.write_text(text)
    cargo = ex / "📦️packages/🦀️rust/Cargo.toml"
    c = cargo.read_text()
    dev = c.split("[dev-dependencies]")[1]
    line = f"semio-s-artifact-norm-{fam}-io = {{ workspace = true }}"
    if line not in dev:
        c = c.rstrip("\n") + "\n" + line + "\n" if c.rstrip().endswith(dev.rstrip()) and "[dev-dependencies]" in c and c.rfind("[dev-dependencies]") > c.rfind("\n[") else c
        if line not in c:
            c = c.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + line + "\n", 1)
        cargo.write_text(c)
    print(fam, "ok")
