//! 🧷️ Carries the typed original picker between the retained job and reactor-only effect publisher.

use super::{OriginalMediaLoan, RawMediaIntent, RawMediaTarget, ORIGINAL_MEDIA_COMMAND_KIND};
use semio_framework::kernel::Effect;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{DslValue, RetainedCloneProgress, ValueError, ValueRefusalKind};

const KEY: &str = "semio:original-media-issue";
const TAG: &[u8; 4] = b"OMI1";
const HEADER: usize = 127;

pub fn pending_picker_effect(effect: &Effect) -> bool {
    matches!(effect, Effect::RequestFileOpen { args: Some(args), .. } if args.get(KEY).is_some())
}

/// ✉️ Projects only the trusted typed intent into the retained command's internal custody lane.
pub fn issue_args(intent: &RawMediaIntent, cx: &mut StepContext<'_>) -> Result<Option<DslValue>, OriginalTurnFailure> {
    let loan = intent.loan();
    let bytes = HEADER + loan.document_id.len();
    let progress = RetainedCloneProgress {
        copied_items: 1,
        copied_bytes: bytes + KEY.len() + size_of::<(String, DslValue)>(),
        retained_capacity_bytes: bytes + KEY.len() + size_of::<(String, DslValue)>(),
        ..Default::default()
    };
    if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() == 0 ||
        cx.retained_grant().maximum_depth < 2 || !progress.fits(cx.retained_grant())
    {
        return Ok(None);
    }
    let document_length = u16::try_from(loan.document_id.len()).map_err(|_| invalid())?;
    let maximum = u32::try_from(intent.maximum()).map_err(|_| invalid())?;
    let mut payload = Vec::new();
    payload.try_reserve_exact(bytes).map_err(|_| invalid())?;
    payload.extend_from_slice(TAG);
    payload.extend_from_slice(&loan.operation_id.to_le_bytes());
    payload.extend_from_slice(&loan.generation.to_le_bytes());
    payload.extend_from_slice(&loan.instance_id.to_le_bytes());
    payload.push(intent.kind());
    payload.extend_from_slice(&maximum.to_le_bytes());
    payload.extend_from_slice(loan.base_revision);
    for word in intent.target() {
        payload.extend_from_slice(&word.to_le_bytes());
    }
    payload.extend_from_slice(&document_length.to_le_bytes());
    payload.extend_from_slice(loan.document_id.as_bytes());
    let marker = DslValue::object([(KEY.to_owned(), DslValue::Bytes(payload))]);
    let capacity = picker_args_capacity(&marker);
    cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: capacity, ..progress })?;
    cx.consume_fuel(1);
    Ok(Some(marker))
}

pub struct DecodedIssue<'a> {
    operation: u64,
    generation: u64,
    instance: u32,
    document: &'a str,
    revision: [u8; 32],
    target: RawMediaTarget,
    maximum: usize,
}

impl<'a> DecodedIssue<'a> {
    pub fn loan(&self) -> OriginalMediaLoan<'_> {
        OriginalMediaLoan { operation_id: self.operation, generation: self.generation, instance_id: self.instance,
            document_id: self.document, base_revision: &self.revision }
    }

    /// 🎫️ Places the typed issue in the actor’s Source, before it can be admitted to the runtime slot.
    pub fn prepare(&self, target: &mut Option<RawMediaIntent>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        RawMediaIntent::issue(self.loan(), self.target, ORIGINAL_MEDIA_COMMAND_KIND, self.maximum, target, cx)
    }
}

pub fn decode_picker_effect(effect: &Effect) -> Result<DecodedIssue<'_>, ValueError> {
    let Effect::RequestFileOpen { args: Some(DslValue::Object(rows)), multiple: false, .. } = effect else { return Err(invalid()); };
    let [(key, DslValue::Bytes(payload))] = rows.as_slice() else { return Err(invalid()); };
    if key != KEY || payload.len() < HEADER || payload.len() > HEADER + 1024 { return Err(invalid()); }
    let mut wire = payload.as_slice();
    if take::<4>(&mut wire)? != *TAG { return Err(invalid()); }
    let operation = u64::from_le_bytes(take(&mut wire)?);
    let generation = u64::from_le_bytes(take(&mut wire)?);
    let instance = u32::from_le_bytes(take(&mut wire)?);
    let [kind] = take(&mut wire)?;
    let maximum = u32::from_le_bytes(take(&mut wire)?) as usize;
    let revision = take(&mut wire)?;
    let mut target = [0; 8];
    for word in &mut target { *word = u64::from_le_bytes(take(&mut wire)?); }
    let length = usize::from(u16::from_le_bytes(take(&mut wire)?));
    if kind != ORIGINAL_MEDIA_COMMAND_KIND || operation == 0 || instance == 0 || maximum == 0 ||
        maximum > super::RAW_MEDIA_BYTES || length != wire.len()
    {
        return Err(invalid());
    }
    let document = std::str::from_utf8(wire).map_err(|_| invalid())?;
    if document.is_empty() || document.chars().take(257).count() > 256 { return Err(invalid()); }
    Ok(DecodedIssue { operation, generation, instance, document, revision, target, maximum })
}

pub fn picker_args_capacity(args: &DslValue) -> usize {
    match args {
        DslValue::Object(rows) => rows.capacity() * size_of::<(String, DslValue)>() +
            rows.iter().map(|(key, value)| key.capacity() + picker_args_capacity(value)).sum::<usize>(),
        DslValue::Bytes(bytes) => bytes.capacity(),
        DslValue::String(value) => value.capacity(),
        _ => 0,
    }
}

fn take<const N: usize>(source: &mut &[u8]) -> Result<[u8; N], ValueError> {
    let (prefix, suffix) = source.split_at_checked(N).ok_or_else(invalid)?;
    *source = suffix;
    prefix.try_into().map_err(|_| invalid())
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "retained original picker issue is outside its exact shape")
}
