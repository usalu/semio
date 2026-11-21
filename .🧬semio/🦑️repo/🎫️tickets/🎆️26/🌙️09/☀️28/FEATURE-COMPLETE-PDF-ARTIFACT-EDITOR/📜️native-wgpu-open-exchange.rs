//! 📥 Submits a complete, bounded natural file to the fresh native program instance.
use super::driver::MAX_TOTAL;
use super::NativeMediaDeclaration;
use super::super::{KernelClient, wasm_program_exchange::{app_frame_error_message, exchange, next_seq}};
use protocol::{AppCommand, AppFrame};
use semio_framework::NATURAL_FILE_PORT_ID;
use semio_framework_value::ToValue;
use semio_plugin::app::{MediaArtifactDescriptor, MediaWireFormat};

pub const NATIVE_DOCUMENT_FILE_MAXIMUM: usize = MAX_TOTAL;

pub(crate) async fn import_media(
    client: &KernelClient, instance: u32, declaration: &NativeMediaDeclaration, bytes: Vec<u8>,
) -> Result<(), String> {
    let sequence = next_seq();
    let command = import_command(sequence, declaration, bytes)?;
    let outcome = exchange(client, instance, vec![command]).await.map_err(|error| error.to_string())?;
    match super::driver::one_save_reply(outcome.frames, sequence, app_frame_error_message)? {
        AppFrame::Done { in_reply_to } if in_reply_to == sequence => Ok(()),
        _ => Err("Open File did not receive the exact import completion".into()),
    }
}

/// 🧾 Addresses a bounded natural file using its manifest-owned binary descriptor.
fn import_command(sequence: u64, declaration: &NativeMediaDeclaration, bytes: Vec<u8>) -> Result<AppCommand, String> {
    if !declaration.reference || bytes.is_empty() || bytes.len() > NATIVE_DOCUMENT_FILE_MAXIMUM {
        return Err("Open File requires the declared natural binary format within its byte limit".into());
    }
    let port = NATURAL_FILE_PORT_ID;
    let descriptor = MediaArtifactDescriptor {
        edge_id: None, port_id: Some(port.into()), kind_id: Some(declaration.format_kind.clone()),
        media_type: None, wire: MediaWireFormat::Binary { format_kind: declaration.format_kind.clone() }, blob_hash: None,
    };
    let descriptor = protocol::pack_rt::encode_wire_value(&descriptor.to_value());
    Ok(AppCommand::MediaIn { seq: sequence, port: port.into(), descriptor, data: bytes })
}
