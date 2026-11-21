//! 📸 Binds the Base image action's trusted placement to Original media ingress.

use crate::{
    command_owner::OriginalMediaFormatInput, guest::{OriginalGuestMediaInput, OriginalMediaGuest, OriginalMediaPickerIntent},
    ingress::OriginalMediaLoan, raw_picker::RawPicker, raw_picker_pack::picker_args,
    standards::v1_7::subsets::base::schema::{mutations::PdfMutation, snapshot::PdfSnapshot},
};
use semio_plugin::{AppOperationContext, Effect, Emit, Fault, RequestId, __job::{OriginalTurnFailure, StepContext}};
use semio_s_artifact_stdio_pdf_editor_document::{
    check_image_placement, revision_bytes, ImagePlacement, IMPORT, INSERT_OPERATOR, MAX_FILE_BYTES, PngTarget,
};

const PNG_BYTES: usize = MAX_FILE_BYTES as usize;
pub type PdfImagePicker = OriginalMediaPickerIntent<ImagePlacement>;
pub type PdfImageIngress = OriginalMediaGuest<ImagePlacement, PNG_BYTES>;
pub type PdfImageFormatInput = OriginalGuestMediaInput<ImagePlacement, PNG_BYTES>;
pub type PdfRawImagePicker = RawPicker<ImagePlacement, PNG_BYTES>;

/// 🔖 Pairs the editor's saved command with the canonical document data observed by the instance.
pub fn picker_loan<'a>(
    operation: &AppOperationContext, live_instance: u32, live_document: &'a str, live_revision: &'a [u8; 32],
) -> OriginalMediaLoan<'a> {
    OriginalMediaLoan {
        operation_id: operation.operation_id, generation: operation.generation,
        instance_id: live_instance, document_id: live_document, base_revision: live_revision,
    }
}

/// 📂 Captures geometry before the existing image action opens a local picker.
pub fn admit_image_picker(
    operation: &AppOperationContext, original: &PdfSnapshot, target: &PngTarget, port_kind: u8,
    intent: &mut Option<PdfImagePicker>, cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let placement = target.placement();
    check_image_placement(original, placement)?;
    let issued = picker_loan(operation, operation.app_instance_id, &operation.parent_document_id, &operation.canonical_base_revision);
    PdfImagePicker::issue(issued, placement, port_kind, PNG_BYTES, intent, cx)
}

/// 🌱 Admits the Base editor's raw picker from its trusted typed target and live store identity.
pub fn admit_raw_image_picker(
    operation: &AppOperationContext, current: OriginalMediaLoan<'_>, original: &PdfSnapshot,
    target: &PngTarget, guard: &str, kind: u8, slot: &mut PdfRawImagePicker, cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let issued = picker_loan(operation, operation.app_instance_id, &operation.parent_document_id, &operation.canonical_base_revision);
    if current != issued || (target.operator != INSERT_OPERATOR && guard.as_bytes() != &revision_bytes(operation)[..16]) {
        return Err(Fault::from("Selected PDF image changed before the picker opened").into());
    }
    let placement = target.placement();
    check_image_placement(original, placement)?;
    slot.issue(issued, placement, kind, PNG_BYTES, cx)
}

/// 🧾 Emits the existing Pack-backed effect after the original intent has been retained.
pub fn raw_image_picker_effect(slot: &PdfRawImagePicker) -> Option<Emit<PdfMutation>> {
    let args = picker_args(slot)?;
    Some(Emit::effect(Effect::RequestFileOpen {
        req: RequestId(0x5044494d), accept: "image/png,.png".into(), read_as: None,
        import_action: IMPORT.into(), multiple: false, args: Some(args), max_file_bytes: Some(MAX_FILE_BYTES),
    }))
}

/// 🎞️ Lends the original PNG bytes to the placement job under the actual current PDF snapshot.
pub fn image_format_input<'a>(
    input: &'a mut PdfImageFormatInput, live: OriginalMediaLoan<'_>, original: &PdfSnapshot,
    cx: &StepContext<'_>,
) -> Result<Option<&'a mut OriginalMediaFormatInput<ImagePlacement, PNG_BYTES>>, OriginalTurnFailure> {
    let Some(received) = input.current_input(live, cx)? else { return Ok(None); };
    check_image_placement(original, received.target)?;
    Ok(Some(received))
}
