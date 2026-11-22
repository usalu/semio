//! 🧪 First-history reference for cache lanes with only the production format crate.
use protocol::{format::{SprWriter, WriteOptions}, wire::DictBuilder, ProtocolError};
use semio_os_pack::{ByteWriter, CodecId};

pub mod channel {
    pub const DOCUMENT_ARCHIVE_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;
    pub const DOCUMENT_ARCHIVE_VERSION: u8 = 1;
}

pub struct AppDocumentIdentity {
    pub app_instance_id: u32,
    pub parent_document_id: Option<String>,
    pub base_revision: u64,
}

pub const REC_COMPOSITION: u8 = 0x41;

pub async fn canonical(document: &str, schema: &str, dialect: Option<[&str; 3]>) -> Result<Vec<u8>, ProtocolError> {
    let mut writer = SprWriter::begin(Vec::new(), &WriteOptions {
        required_flags: protocol::REQUIRED_HASH_CHAIN, optional_flags: protocol::OPTIONAL_CANONICAL,
    }).await?;
    let mut dict = DictBuilder::new();
    let mut base = 0;
    let doc = payload(&[1], &[document, schema], &mut dict)?;
    delta(&mut writer, &dict, &mut base).await?;
    writer.write_record(protocol::REC_DOC, true, &doc, CodecId(0)).await?;
    if let Some(fields) = dialect {
        let composition = payload(&[1, 2], &fields, &mut dict)?;
        delta(&mut writer, &dict, &mut base).await?;
        writer.write_record(REC_COMPOSITION, false, &composition, CodecId(0)).await?;
    }
    writer.commit().await?;
    Ok(writer.into_sink().await)
}

fn payload(prefix: &[u8], fields: &[&str], dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let mut output = ByteWriter::new();
    output.write_bytes(prefix);
    for field in fields {
        protocol::scalar::write_id(&mut output, field, |token| dict.intern(token), |_| None)?;
    }
    Ok(output.into_bytes())
}

async fn delta(writer: &mut SprWriter<Vec<u8>>, dict: &DictBuilder, base: &mut u32) -> Result<(), ProtocolError> {
    if *base < dict.len() {
        let mut output = ByteWriter::new();
        output.write_u8(1);
        output.write_varint_u64(u64::from(*base));
        let entries = dict.entries_since(*base);
        output.write_varint_u64(entries.len() as u64);
        for entry in entries {
            output.write_varint_u64(entry.len() as u64);
            output.write_bytes(entry.as_bytes());
        }
        writer.write_record(protocol::REC_STR_DICT, true, &output.into_bytes(), CodecId(0)).await?;
        *base = dict.len();
    }
    Ok(())
}

pub fn archive(pack: &[u8], spr: &[u8]) -> Vec<u8> {
    let mut bytes = ByteWriter::new();
    bytes.write_u8(channel::DOCUMENT_ARCHIVE_VERSION);
    bytes.write_varint_u64(pack.len() as u64);
    bytes.write_bytes(pack);
    bytes.write_varint_u64(spr.len() as u64);
    bytes.write_bytes(spr);
    bytes.write_u8(0);
    bytes.into_bytes()
}
