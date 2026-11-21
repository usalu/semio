"""🧩️ Assembles the JPEG cursor and PDF owner candidate without modifying the live source epoch."""
from pathlib import Path

ROOT = Path.cwd()
TICKET = Path(__file__).resolve().parent
STAGE = TICKET / "🗑️generated" / "paged-jpeg-stage"
ARTIFACTS = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
JPG = ARTIFACTS / "📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io"
PDF = ARTIFACTS / "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📦️foreign-artifacts"


def one(text, before, after):
    if text.count(before) != 1:
        raise ValueError(f"Expected one exact anchor: {before}")
    return text.replace(before, after, 1)


def entropy_source():
    text = (JPG / "🔡️huffman-and-bit-io/🦀️.rs").read_text()
    needle = "/// 🏗️ Builds canonical codes"
    trait = """pub(crate) trait HuffLookup {
    fn max_len(&self) -> u8;
    fn symbol(&self, length: u8, code: u16) -> Option<u8>;
}

impl HuffLookup for HuffTable {
    fn max_len(&self) -> u8 { self.max_len }
    fn symbol(&self, length: u8, code: u16) -> Option<u8> { self.decode.get(&(length, code)).copied() }
}

"""
    text = one(text, needle, trait + needle)
    text = one(text, "table: &HuffTable) -> Result<u8, JpgError>", "table: &impl HuffLookup) -> Result<u8, JpgError>")
    text = one(text, "1..=table.max_len", "1..=table.max_len()")
    text = one(text, "if let Some(v) = table.decode.get(&(len, code)) {\n                return Ok(*v);",
               "if let Some(value) = table.symbol(len, code) {\n                return Ok(value);")
    text = one(text, "dc_table: &HuffTable, ac_table: &HuffTable) -> Result<[i32; 64], JpgError>",
               "dc_table: &impl HuffLookup, ac_table: &impl HuffLookup) -> Result<[i32; 64], JpgError>")
    text = one(text, "*dc_pred += extend_sign(bits, sz);",
               """*dc_pred = dc_pred.checked_add(extend_sign(bits, sz))
        .ok_or_else(|| JpgError::Malformed("JPEG DC predictor overflow".into()))?;""")
    text = one(text, "                z += 16;\n                continue;", """                z += 16;
                if z > 64 { return Err(JpgError::Malformed("JPEG zero run overruns the block".into())); }
                continue;""")
    return text


def foreign_source():
    text = (PDF / "🦀️.rs").read_text()
    text = one(text, """        self.hash.update(&data[self.cursor..end]);
        let used = end - self.cursor;
        self.cursor = end;""", """        let used = end - self.cursor;
        self.append(&data[self.cursor..end]);""")
    needle = "    /// ✅️ Indicates that the borrowed encoded stream was read completely."
    text = one(text, needle, """    /// 📍️ Gives the number of original bytes already hashed.
    pub fn position(&self) -> usize { self.cursor }

    /// 🧮️ Hashes the next verified native bytes without requiring a contiguous source.
    pub fn append(&mut self, data: &[u8]) {
        self.hash.update(data);
        self.cursor += data.len();
    }

""" + needle)
    text = one(text, """        let mut hash = self.hash.clone();
        hash.update(self.filter_wire);
        artifact_reference(DCT_KIND, hash.finalize())""", """        Self::reference_for_digest(self.digest())
    }

    /// 🪪️ Materializes the native DCT reference from a completed external digest.
    pub fn reference_for_digest(digest: [u8; 32]) -> ArtifactRef {
        artifact_reference(DCT_KIND, digest)
    }

    /// 🔑️ Computes the fixed-size native identity after hashing admitted input.
    pub fn digest(&self) -> [u8; 32] {
        let mut hash = self.hash.clone();
        hash.update(self.filter_wire);
        hash.finalize()""")
    return text


def stage():
    jpg_entry = (JPG / "🦀️.rs").read_text()
    anchor = "pub use step_decoder::*;"
    jpg_entry = one(jpg_entry, anchor, anchor + '\n\n#[path = "🧮️baseline-proof/🦀️.rs"]\nmod baseline_proof;\npub use baseline_proof::*;')
    files = {
        "jpg-io.rs": jpg_entry,
        "jpg-baseline-proof.rs": (TICKET / "paged-jpg-baseline.rs").read_text(),
        "jpg-entropy.rs": entropy_source(),
        "pdf-foreign-artifacts.rs": foreign_source(),
        "pdf-baseline-resource.rs": (TICKET / "paged-pdf-dct-resource.rs").read_text(),
    }
    document = PDF.parent.parent / "✏️editor/📄️document"
    authoring = (document / "🖼️image-authoring/🦀️.rs").read_text()
    files["pdf-authoring-entry.rs"] = one(authoring, "use super::*;", """use super::*;

#[path = "🧮️baseline-resource.rs"]
pub mod baseline_resource;""")
    cargo = (document / "📦️packages/🦀️rust/Cargo.toml").read_text()
    files["pdf-document.Cargo.toml"] = one(cargo, "[dependencies]\n", """[dependencies]
semio-framework-artifact-reference = { workspace = true }
semio-s-artifact-stdio-pdf-inferences = { path = "../../../../🧬️schema/💡️inferences/📦️packages/🦀️rust" }
semio-s-artifact-stdio-jpg-inferences = { path = "../../../../../../../../../📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/💡️inferences/📦️packages/🦀️rust" }
""")
    source_trait = (JPG / "🔎️header-and-decode-entry/🦀️.rs").read_text()
    foreign = files["pdf-foreign-artifacts.rs"]
    transforms = (JPG / "🧮️transforms-and-tables/🦀️.rs").read_text()
    bridge = transforms.index("/// 🧭️ Carries")
    transforms = transforms[:bridge] + transforms[transforms.index("//#endregion Errors", bridge):]
    files.update({
        "probe.rs": (TICKET / "paged-jpeg-probe.rs").read_text(),
        "live-jpeg-transforms.rs": transforms,
        "live-hash.rs": (ROOT / "🧰️framework/🔨️modules/🔏️hash/🦀️.rs").read_text(),
        "source-trait.rs": source_trait[source_trait.index("pub trait JpgByteSource"):source_trait.index("fn source_range")],
        "identity-core.rs": foreign[foreign.index("const DCT_KIND:"):foreign.index("/// 🛂️ Native admission")],
    })
    STAGE.mkdir(parents=True, exist_ok=True)
    for name, text in files.items():
        (STAGE / name).write_text(text)


if __name__ == "__main__":
    stage()
