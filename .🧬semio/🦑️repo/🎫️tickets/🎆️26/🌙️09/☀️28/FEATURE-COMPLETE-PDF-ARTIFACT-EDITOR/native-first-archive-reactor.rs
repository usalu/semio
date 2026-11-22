//! 📤 Routes a bounded first-genesis request before ordinary Shell message dispatch.
use semio_framework::kernel::{Effect, Event, MessageEndpoint, PluginInstanceId};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_os_kernel::channel::{FirstArchiveKey, FirstArchiveReply, FirstArchiveRequest};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;

pub fn is_first_archive_event(event: &Event) -> bool {
    matches!(event, Event::Message { source: MessageEndpoint::Shell { .. }, payload } if FirstArchiveRequest::is_wire(payload))
}

pub fn drive_first_archive<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, input: &mut Option<Event>, effects: &mut Vec<Effect>,
    maximum_effects: usize, cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let Some(Event::Message { source: MessageEndpoint::Shell { instance }, payload }) = input.as_ref()
        .filter(|event| is_first_archive_event(event))
    else { return Ok(false); };
    let Some(request) = FirstArchiveRequest::decode(payload).filter(|request|
        instance.0.parse::<u32>().ok() == Some(request.key.instance) && instance.0.len() <= 10
        && !instance.0.starts_with('0')
    ) else { return Err(invalid("first archive request changed its Shell instance").into()); };
    if effects.len() >= maximum_effects { return Ok(true); }
    let response_capacity = 44 + 4096;
    let effect_growth = if effects.len() == effects.capacity() { (effects.len() + 1) * size_of::<Effect>() } else { 0 };
    let old_effect_capacity = if effect_growth != 0 { effects.capacity() * size_of::<Effect>() } else { 0 };
    let mut progress = RetainedCloneProgress {
        copied_items: 1,
        copied_bytes: response_capacity + size_of::<Effect>() + size_of::<Option<Event>>(),
        retained_capacity_bytes: response_capacity + effect_growth,
        released_bytes: payload.capacity() + old_effect_capacity,
        ..Default::default()
    };
    let grant = cx.retained_grant();
    if cx.should_yield() || cx.fuel_remaining() < progress.copied_bytes as u64
        || grant.maximum_depth == 0 || grant.maximum_items < 2 || !progress.fits(grant) { return Ok(true); }
    effects.try_reserve_exact(1).map_err(|_| invalid("first archive effect recipient allocation refused"))?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(response_capacity).map_err(|_| invalid("first archive response allocation refused"))?;
    let mut responded = None;
    let response_work = progress.copied_bytes as u64;
    cx.with_retained_loan(RetainedCloneGrant {
        maximum_items: grant.maximum_items - progress.copied_items,
        maximum_copy_bytes: grant.maximum_copy_bytes - progress.copied_bytes,
        maximum_capacity_bytes: grant.maximum_capacity_bytes - progress.retained_capacity_bytes,
        maximum_release_bytes: grant.maximum_release_bytes - progress.released_bytes,
        ..grant
    }, |cx| cx.with_original_work_loan(cx.fuel_remaining() - response_work, u64::MAX, |cx| {
        responded = super::with_slot(runtime, request.key.instance, cx, |slot, app, cx| {
            let source = slot.natural_source.as_ref().filter(|source| source.first_archive_frontier().is_some());
            if let Some(source) = source.filter(|source| {
                let operation = source.loan().operation;
                request.key == (FirstArchiveKey { instance: operation.instance, operation: operation.operation,
                    generation: operation.generation, sequence: source.sequence() })
            }) {
                app.first_genesis_archive_turn(source, request, &mut bytes, cx)?;
            } else {
                FirstArchiveReply::Refused.encode_into(request, &mut bytes);
            }
            Ok(())
        })?;
        Ok(())
    }))?;
    if responded.is_none() || bytes.is_empty() { FirstArchiveReply::Pending.encode_into(request, &mut bytes); }
    progress.copied_bytes = bytes.len() + size_of::<Effect>() + size_of::<Option<Event>>();
    progress.retained_capacity_bytes = bytes.capacity() + effect_growth;
    cx.consume_retained(progress)?;
    cx.consume_fuel(progress.copied_bytes as u64);
    let Some(Event::Message { source: MessageEndpoint::Shell { instance: PluginInstanceId(instance) }, .. }) = input.take()
    else { unreachable!("first archive turn retained its selected Shell request"); };
    effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: PluginInstanceId(instance) }, payload: bytes });
    Ok(true)
}

fn invalid(detail: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, detail) }
