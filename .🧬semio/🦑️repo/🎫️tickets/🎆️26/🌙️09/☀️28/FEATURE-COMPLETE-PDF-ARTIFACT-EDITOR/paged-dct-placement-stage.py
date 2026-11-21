"""🧾️ Stages the sealed PDF DCT placement and links a narrow public-PDF-history probe."""
import json
import runpy
from pathlib import Path

TICKET = Path(__file__).resolve().parent
INSTALL = runpy.run_path(str(TICKET / "paged-dct-install-stage.py"))
INSTALL["stage"]()
INSTALL["cached_args"]()
BEFORE = INSTALL["OUT"]
OUT = TICKET / "🗑️generated" / "paged-dct-placement"
ONE = INSTALL["GRAPH"]["one"]


def stage():
    OUT.mkdir(parents=True, exist_ok=True)
    for file in ("identity.rs", "📖️dct-view.rs"):
        (OUT / file).write_text((BEFORE / file).read_text())
    source = (BEFORE / "🗃️install-pack.rs").read_text()
    source = source.replace("s.stdio.pdf.dct-install.v1", "s.stdio.pdf.dct-placement.v1")
    source = ONE(source, "chunks:identity:32768", "chunks:identity:32768,placement:typed-pdf-rows-v1")
    source = ONE(source, "const LABEL_BYTES: usize = 42;", "const LABEL_BYTES: usize = 42;\nconst MAX_PLACEMENT: usize = 32 + 4 + 2 * 4096;")
    source = ONE(source, "const MAX_CHUNKS: u64 = 1 +", "const MAX_CHUNKS: u64 = 2 +")
    source = ONE(source, "    view: PdfDctImageView<'_>,\n", "    view: PdfDctImageView<'_>,\n    placement: &[u8],\n")
    source = ONE(source, "    checkpoint(0).await?;", """    if placement.is_empty() || placement.len() > MAX_PLACEMENT {
        return Err(refusal("invalid sealed placement extent").into());
    }
    let mut record = writer.begin_identity_chunk(placement.len()).await?;
    for fragment in placement.chunks(PAGED_BYTES_CHUNK_BYTES) {
        checkpoint(fragment.len()).await?;
        record.write_fragment(fragment).await?;
    }
    record.finish().await?;
    checkpoint(0).await?;""")
    source = ONE(source, "pack.chunk_count() != 1 +", "pack.chunk_count() != 2 +")
    source = ONE(source, "    /// 🪜️ Creates a bounded verifier", """    /// 🧾️ Reads the one bounded canonical placement record after the original chunk directory.
    pub async fn placement_record(&self) -> Result<Vec<u8>, S::Error> {
        let slot = ChunkId((self.pack.chunk_count() - 1) as u32);
        let length = self.pack.chunk_decoded_len(slot)? as usize;
        if length == 0 || length > MAX_PLACEMENT {
            return Err(refusal("invalid sealed placement extent").into());
        }
        let mut record = vec![0; length];
        let mut cursor = self.pack.identity_chunk_cursor(slot, VerificationLevel::Full)?;
        for fragment in record.chunks_mut(PAGED_BYTES_CHUNK_BYTES) {
            if cursor.read_fragment(fragment).await? != fragment.len() {
                return Err(refusal("sealed placement extent changed").into());
            }
        }
        if cursor.read_fragment(&mut [0]).await? != 0 {
            return Err(refusal("sealed placement has trailing bytes").into());
        }
        Ok(record)
    }

    /// 🪜️ Creates a bounded verifier""")
    (OUT / "🗃️install-pack.rs").write_text(source)
    for target, name in [("placement-wire.rs", "paged-dct-placement-wire.rs"), ("operation-hook.rs", "paged-dct-placement-operation-hook.rs")]:
        (OUT / target).write_text((TICKET / name).read_text())
    old_probe = (BEFORE / "probe.rs").read_text()
    harness = old_probe[:old_probe.index("async fn run(")].replace("use install_pack::{NativeDctInstallPack, PagedDctInstallPayload, write_dct_install_pack};", "use install_pack::PagedDctInstallPayload;")
    (OUT / "probe.rs").write_text(harness + (TICKET / "paged-dct-placement-probe.rs").read_text())


def args():
    existing = (BEFORE / "probe.args").read_text()
    root = Path(".cache/b/r/m/u/l/pe73647/prototype/build")
    record = next(root.glob("semio-s-artifact-stdio-pdf-mut-family-8/*/out/libsemio_s_artifact_stdio_pdf_mut_family_8-*.rmeta"))
    deps = {name for name, _ in json.loads(record.with_suffix(".rmeta.semio-deps").read_text())["dependencies"]}
    extra = []
    for name in ("semio_s_artifact_stdio_pdf_mut_family_8", "semio_os_spr", "semio_os_vcs", "semio_framework_dsl_record", "semio_os_store_one_item_publication"):
        matches = sorted(root.glob(f"*/*/out/lib{name}-*.rmeta"))
        compatible = [path for path in matches if path.name[3:-6] in deps]
        found = compatible or matches
        if len(found) != 1:
            raise ValueError((name, found))
        extra += ["--extern", f"{name}={found[0]}", "--extern", f"{name}={found[0].with_suffix('.rlib')}"]
    (OUT / "probe.args").write_text(existing.replace(str(BEFORE), str(OUT)) + "\n".join(extra) + "\n")


if __name__ == "__main__":
    stage()
    args()
