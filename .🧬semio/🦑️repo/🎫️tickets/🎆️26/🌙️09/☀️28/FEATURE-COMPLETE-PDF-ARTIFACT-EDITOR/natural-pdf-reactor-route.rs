//! 🪢️ Drives the reserved natural source family before assembled command ingress.

use super::{fault, reactor_close_fault, with_slot};
use semio_framework::kernel::{
    CommandIngressStatus, CommandPageCursor, Event, FixedCommandPage, NaturalSourceWireAck,
    NaturalSourceWireControl, NaturalSourceWireDeclaration, NaturalSourceWireLoan,
    NaturalSourceWirePhase as Phase, NaturalSourceWireResult as WireResult, NATURAL_SOURCE_COMMAND_KIND,
};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_plugin_decl_media::original_media::{
    natural_source::NaturalSourceStatus,
    natural_source_gate::{NaturalSourceBegin, NaturalSourceDeclaration, NaturalSourceLoan,
        NaturalSourceOffer, NaturalSourceOperation},
};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_reactor_lifecycle::{instance_lifetime::NaturalSourceControl, life_instance_registry::PluginRuntime};

pub(super) fn drive<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, events: &mut Option<Event>,
    page: &mut Option<(CommandPageCursor, FixedCommandPage)>, closing: &[u32],
    cx: &mut StepContext<'_>,
) -> Result<Option<CommandIngressStatus>, OriginalTurnFailure> {
    match events.as_ref() {
        Some(Event::NaturalSourcePrepare(_) | Event::NaturalSourceControl(_)) => {
            if page.is_some() { return Err(reactor_close_fault("natural source control shares a fixed-page turn").into()); }
            return Ok(Some(drive_control(runtime, events, closing, cx)?.unwrap_or(CommandIngressStatus::Idle)));
        }
        _ => {}
    }
    if let Some((cursor, _)) = page.as_ref().filter(|(cursor, _)| cursor.kind == NATURAL_SOURCE_COMMAND_KIND) {
        let cursor = cursor.clone();
        let Some(key) = runtime.live_instance_key(cursor.instance).ok().filter(|_| !closing.contains(&cursor.instance)) else {
            let result = funded_fault(&cursor, b"plugin.natural-source-instance-closed", cx)?;
            return Ok(Some(result.unwrap_or(CommandIngressStatus::Backpressure(cursor))));
        };
        let answer = with_slot(runtime, cursor.instance, cx, |slot, app, cx| {
            let Some(loan) = slot.natural_source.as_ref().map(|source| source.loan()) else {
                return funded_fault(&cursor, b"plugin.natural-source-unissued", cx);
            };
            let fresh = app.natural_source_capability().is_some();
            funded_ack(Some(loan), 0, cx, |cx| {
                let result = slot.receive_natural_source_page(key, fresh, page, cx)?;
                Ok(Some(fields(cursor.owner, cursor.seq, cursor.metadata, Phase::Page, Some(cursor.page_index), result)))
            })
        })?.flatten();
        return Ok(Some(answer.unwrap_or(CommandIngressStatus::Backpressure(cursor))));
    }
    if events.is_some() || page.is_some() { return Ok(None); }
    let instance = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("natural source lifetime is busy"))?
        .next_natural_source_poll(closing);
    let Some(instance) = instance else { return Ok(None); };
    let key = runtime.live_instance_key(instance)?;
    Ok(with_slot(runtime, instance, cx, |slot, app, cx| {
        let Some(source) = slot.natural_source.as_ref() else { return Ok(None); };
        let loan = source.loan();
        let seq = source.sequence();
        let cancel = source.cancellation_pending();
        let complete = source.completion_pending();
        funded_ack(Some(loan), 0, cx, |cx| {
            if cancel || complete {
                let source = slot.natural_source.as_mut().expect("selected natural source");
                if cancel && !source.retire(app.natural_source_format_closed(), cx)? { return Ok(None); }
                Ok(source.terminal_receipt(cx)?.map(|result| fields(u64::from(instance), seq, loan.extent,
                    if cancel { Phase::Cancel } else { Phase::Complete }, None, result)))
            } else {
                let page = slot.poll_natural_source_page(key, app.natural_source_capability().is_some(), cx)?;
                Ok(page.map(|index| fields(u64::from(instance), seq, loan.extent, Phase::Page, Some(index), NaturalSourceStatus::Accepted)))
            }
        })
    })?.flatten())
}

fn drive_control<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, input: &mut Option<Event>, closing: &[u32], cx: &mut StepContext<'_>,
) -> Result<Option<CommandIngressStatus>, OriginalTurnFailure> {
    let selected = input.as_ref().expect("selected natural source control");
    match selected {
        Event::NaturalSourcePrepare(offer) => {
            let instance = offer.instance_id;
            let owner = u64::from(instance);
            let release = declaration_capacity(&offer.declaration);
            let request = NaturalSourceOffer { declaration: declaration(&offer.declaration), extent: offer.extent };
            let result = match runtime.live_instance_key(instance).ok().filter(|_| !closing.contains(&instance)) {
                Some(key) => with_slot(runtime, instance, cx, |slot, app, cx| {
                    let Some(shape) = app.natural_source_capability() else {
                        return funded_ack(None, release, cx, |_| Ok(Some(fields(owner, offer.seq, offer.extent, Phase::Prepare, None, NaturalSourceStatus::Refused))));
                    };
                    let reserve = NaturalSourceLoan { operation: NaturalSourceOperation { operation: 1, generation: key.lifetime.guest_lifetime, instance }, declaration: shape.0, extent: offer.extent };
                    funded_ack(Some(reserve), release, cx, |cx| {
                        let (result, loan) = slot.prepare_natural_source(key, offer.seq, request, cx)?;
                        let mut receipt = fields(owner, offer.seq, offer.extent, Phase::Prepare, None, result);
                        receipt.loan = loan.map(wire_loan).transpose()?;
                        Ok(Some(receipt))
                    })
                })?.flatten(),
                None => funded_ack(None, release, cx, |_| Ok(Some(fields(owner, offer.seq, offer.extent, Phase::Prepare, None, NaturalSourceStatus::Stale))))?,
            };
            if result.is_some() { input.take(); }
            Ok(result)
        }
        Event::NaturalSourceControl(event) => {
            let instance = event.loan.instance_id;
            let (control, phase) = match event.control {
                NaturalSourceWireControl::Begin => (NaturalSourceControl::Begin, Phase::Begin),
                NaturalSourceWireControl::Complete => (NaturalSourceControl::Complete, Phase::Complete),
                NaturalSourceWireControl::Cancel => (NaturalSourceControl::Cancel, Phase::Cancel),
            };
            let offer = decode(&event.loan).map(|loan| NaturalSourceBegin { loan, owner: event.owner, seq: event.seq, kind: event.kind, extent: event.extent });
            let emit = |result| Some(fields(event.owner, event.seq, event.extent, phase, None, result));
            let live = runtime.live_instance_key(instance).ok().filter(|_| !closing.contains(&instance));
            let result = if let (Some(key), Some(offer)) = (live, offer) {
                with_slot(runtime, instance, cx, |slot, app, cx| {
                    funded_ack(None, 0, cx, |cx| {
                        let result = slot.control_natural_source(key, app.natural_source_capability().is_some(), app.natural_source_format_closed(), control, offer, cx)?;
                        Ok(emit(result))
                    })
                })?.flatten()
            } else {
                funded_ack(None, 0, cx, |_| Ok(emit(NaturalSourceStatus::Stale)))?
            };
            if let Some(CommandIngressStatus::NaturalSourceAck(mut ack)) = result {
                let Event::NaturalSourceControl(event) = input.take().expect("paid natural source control receipt") else { unreachable!() };
                ack.loan = Some(event.loan);
                return Ok(Some(CommandIngressStatus::NaturalSourceAck(ack)));
            }
            Ok(None)
        }
        _ => unreachable!("prologue selected another event"),
    }
}

fn funded_fault(
    cursor: &CommandPageCursor, message: &[u8], cx: &mut StepContext<'_>,
) -> Result<Option<CommandIngressStatus>, OriginalTurnFailure> {
    let progress = RetainedCloneProgress {
        copied_items: 1, copied_bytes: size_of::<CommandIngressStatus>() + message.len(),
        retained_capacity_bytes: message.len(), ..Default::default()
    };
    if cx.retirement_should_yield() || cx.fuel_remaining() == 0
        || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
        return Ok(None);
    }
    let result = fault(cursor.clone(), message);
    cx.consume_retained(progress)?;
    cx.consume_fuel(1);
    Ok(Some(result))
}

fn funded_ack(
    reserve: Option<NaturalSourceLoan<'static>>, released_bytes: usize, cx: &mut StepContext<'_>,
    step: impl FnOnce(&mut StepContext<'_>) -> Result<Option<NaturalSourceWireAck>, OriginalTurnFailure>,
) -> Result<Option<CommandIngressStatus>, OriginalTurnFailure> {
    let strings = reserve.map_or(0, loan_bytes);
    let progress = RetainedCloneProgress {
        copied_items: 1, copied_bytes: size_of::<Option<Event>>() + size_of::<NaturalSourceWireAck>() + strings,
        retained_capacity_bytes: strings, released_bytes,
        ..Default::default()
    };
    let grant = cx.retained_grant();
    if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || grant.maximum_depth == 0 || !progress.fits(grant) {
        return Ok(None);
    }
    let mut ack = None;
    cx.with_retained_loan(RetainedCloneGrant {
        maximum_items: grant.maximum_items - progress.copied_items,
        maximum_copy_bytes: grant.maximum_copy_bytes - progress.copied_bytes,
        maximum_capacity_bytes: grant.maximum_capacity_bytes - progress.retained_capacity_bytes,
        maximum_release_bytes: grant.maximum_release_bytes - progress.released_bytes,
        ..grant
    }, |cx| cx.with_original_work_loan(cx.fuel_remaining() - 1, u64::MAX, |cx| {
        ack = step(cx)?;
        Ok(())
    }))?;
    let Some(mut ack) = ack else { return Ok(None); };
    if ack.loan.is_none() && ack.phase != Phase::Prepare {
        ack.loan = reserve.map(wire_loan).transpose()?;
    }
    cx.consume_retained(RetainedCloneProgress {
        retained_capacity_bytes: ack.loan.as_ref().map_or(0, |loan| loan.operation_id.capacity() + loan.generation.capacity() + declaration_capacity(&loan.declaration)),
        ..progress
    })?;
    cx.consume_fuel(1);
    Ok(Some(CommandIngressStatus::NaturalSourceAck(ack)))
}

fn fields(owner: u64, seq: u64, extent: u32, phase: Phase, page_index: Option<u32>, result: NaturalSourceStatus) -> NaturalSourceWireAck {
    let result = match result {
        NaturalSourceStatus::Retry => WireResult::Retry,
        NaturalSourceStatus::Pending => WireResult::Pending,
        NaturalSourceStatus::Accepted => WireResult::Accepted,
        NaturalSourceStatus::Stale => WireResult::Stale,
        NaturalSourceStatus::Refused => WireResult::Refused,
    };
    NaturalSourceWireAck { loan: None, owner, seq, kind: NATURAL_SOURCE_COMMAND_KIND, extent, phase, page_index, result }
}

fn declaration(value: &NaturalSourceWireDeclaration) -> NaturalSourceDeclaration<'_> {
    NaturalSourceDeclaration { format_kind: &value.format_kind, extension: &value.extension, media_type: &value.media_type, binary: value.binary }
}

fn declaration_capacity(value: &NaturalSourceWireDeclaration) -> usize {
    value.format_kind.capacity() + value.extension.capacity() + value.media_type.capacity()
}

fn loan_bytes(loan: NaturalSourceLoan<'_>) -> usize {
    32 + loan.declaration.format_kind.len() + loan.declaration.extension.len() + loan.declaration.media_type.len()
}

fn decode(loan: &NaturalSourceWireLoan) -> Option<NaturalSourceLoan<'_>> {
    fn field(value: &str) -> Option<u64> {
        if value.len() != 16 || !value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) { return None; }
        u64::from_str_radix(value, 16).ok().filter(|value| *value != 0)
    }
    Some(NaturalSourceLoan {
        operation: NaturalSourceOperation { operation: field(&loan.operation_id)?, generation: field(&loan.generation)?, instance: loan.instance_id },
        declaration: declaration(&loan.declaration), extent: loan.extent,
    })
}

fn wire_loan(loan: NaturalSourceLoan<'_>) -> Result<NaturalSourceWireLoan, ValueError> {
    fn text(value: &str) -> Result<String, ValueError> {
        let mut out = String::new();
        out.try_reserve_exact(value.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "natural source receipt allocation refused"))?;
        out.push_str(value);
        Ok(out)
    }
    fn hex(n: u64) -> Result<String, ValueError> {
        let digits: [u8; 16] = std::array::from_fn(|index| b"0123456789abcdef"[((n >> ((15 - index) * 4)) & 15) as usize]);
        text(std::str::from_utf8(&digits).expect("hexadecimal natural source receipt"))
    }
    Ok(NaturalSourceWireLoan {
        operation_id: hex(loan.operation.operation)?, generation: hex(loan.operation.generation)?, instance_id: loan.operation.instance,
        declaration: NaturalSourceWireDeclaration {
            format_kind: text(loan.declaration.format_kind)?, extension: text(loan.declaration.extension)?,
            media_type: text(loan.declaration.media_type)?, binary: loan.declaration.binary,
        },
        extent: loan.extent,
    })
}
