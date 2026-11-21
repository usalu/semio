//! 🔭️ Replays exact MCP Note snapshot pairs through the producer's canonical persisted codec.

use semio_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer;
use semio_os_store_text_io::text_format::{discard_parsed_envelope, parse_document_pack};
use semio_framework_io_base64::base64_standard_decode;
use semio_s_artifact_note_note_mutations::schema::mutations::NoteMutation;
use semio_s_artifact_note_note_schema::NoteSnapshot;
use semio_s_artifact_note_note_schema::standards::v1::subsets::any::io::text::snapshot::encode_note_snapshot_json;
use std::io::{BufRead, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = std::io::stdout().lock();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let (pack, spr) = line.split_once('\t').ok_or("expected Pack and SPR base64 separated by a tab")?;
        let pack = base64_standard_decode(pack).map_err(|error| error.to_string())?;
        let spr = base64_standard_decode(spr).map_err(|error| error.to_string())?;
        let document = semio_framework_async::block_on(
            parse_document_pack::<NoteSnapshot, NoteMutation>(&pack, &spr, &ArtifactPackGenesisProducer),
        ).map_err(|error| error.to_string())?;
        let snapshot_json = encode_note_snapshot_json(&document.snapshot);
        discard_parsed_envelope(document.into_envelope());
        writeln!(output, "{snapshot_json}")?;
        output.flush()?;
    }
    Ok(())
}
