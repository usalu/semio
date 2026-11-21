//! 📥️ Diverts registered original-media pages before the generic command byte reader.

use semio_framework::kernel::{CommandIngressStatus, CommandPageCursor, Event, FixedCommandPage, OriginalMediaControlEvent, OriginalMediaWireControl, ORIGINAL_MEDIA_COMMAND_KIND};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

pub fn drive_raw_media<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, events: &mut Option<Event>, command_page: &mut Option<(CommandPageCursor, FixedCommandPage)>,
    closing: &[u32], identity: &mut EntityIdentityAuthority<'_>, cx: &mut StepContext<'_>,
) -> Result<Option<CommandIngressStatus>, OriginalTurnFailure> {
    if let Some(Event::OriginalMediaControl(event)) = events.as_ref() {
        if command_page.is_some() { return Err(reactor_close_fault("original media control shares a fixed-page turn").into()); }
        let grant = cx.retained_grant();
        let close = raw_control_close(event);
        if grant.maximum_depth == 0 || cx.fuel_remaining() == 0 || !close.fits(grant) {
            return Ok(Some(CommandIngressStatus::Idle));
        }
        let mut ack = None;
        cx.with_retained_loan(semio_framework_value::RetainedCloneGrant {
            maximum_items: grant.maximum_items - close.copied_items,
            maximum_copy_bytes: grant.maximum_copy_bytes - close.copied_bytes,
            maximum_release_bytes: grant.maximum_release_bytes - close.released_bytes,
            ..grant
        }, |cx| {
            cx.with_original_work_loan(cx.fuel_remaining() - 1, u64::MAX, |cx| {
                ack = control(runtime, event, closing, cx)?;
                Ok(())
            })
        })?;
        if ack.is_some() {
            cx.consume_retained(close)?;
            cx.consume_fuel(1);
            events.take();
        }
        return Ok(Some(ack.unwrap_or(CommandIngressStatus::Idle)));
    }
    if let Some((cursor, _)) = command_page.as_ref().filter(|(cursor, _)| cursor.kind == ORIGINAL_MEDIA_COMMAND_KIND) {
        let cursor = cursor.clone();
        if closing.contains(&cursor.instance) {
            return Ok(Some(fault(cursor, b"plugin.original-media-instance-closing")));
        }
        let routed = with_slot(runtime, cursor.instance, cx, |slot, app, cx| {
            if slot.raw_media.terminal_is_empty() { return Ok(Some(fault(cursor.clone(), b"plugin.original-media-unissued"))); }
            let live = app.original_media_live_document();
            Ok(slot.raw_media.page(live.as_ref(), command_page, cx)?.map(CommandIngressStatus::OriginalMediaAck))
        })?;
        return Ok(Some(routed.flatten().unwrap_or_else(|| CommandIngressStatus::Backpressure(cursor))));
    }
    if command_page.is_some() { return Ok(None); }
    let next = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("original media lifetime is busy"))?
        .next_raw_media_poll(closing);
    let Some(instance) = next else { return Ok(None); };
    Ok(with_slot(runtime, instance, cx, |slot, app, cx| {
        let live = app.original_media_live_document();
        if let Some(ack) = slot.raw_media.poll(live.as_ref(), app.original_media_format_closed(), cx)? {
            return Ok(Some(CommandIngressStatus::OriginalMediaAck(ack)));
        }
        app.advance_original_media_format(&mut slot.raw_media.picker, identity, cx)?;
        Ok(None)
    })?.flatten())
}

fn raw_control_close(event: &OriginalMediaControlEvent) -> RetainedCloneProgress {
    let loan = &event.loan;
    let release = loan.operation_id.capacity() + loan.generation.capacity() +
        loan.document_id.capacity() + loan.base_revision.capacity() + match &event.control {
            OriginalMediaWireControl::Begin { name, media_type } => name.capacity() + media_type.capacity(),
            OriginalMediaWireControl::Complete | OriginalMediaWireControl::Cancel => 0,
        };
    RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Option<Event>>(),
        released_bytes: release, ..Default::default() }
}

fn control<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, event: &OriginalMediaControlEvent, closing: &[u32], cx: &mut StepContext<'_>,
) -> Result<Option<CommandIngressStatus>, OriginalTurnFailure> {
    let instance = event.loan.instance_id;
    if closing.contains(&instance) {
        return Ok(None);
    }
    Ok(with_slot(runtime, instance, cx, |slot, app, cx| {
        let live = app.original_media_live_document();
        Ok(slot.raw_media.control(event, live.as_ref(), cx)?.map(CommandIngressStatus::OriginalMediaAck))
    })?.flatten())
}

fn with_slot<PA: PluginApp, R>(
    runtime: &PluginRuntime<PA>, instance: u32, cx: &mut StepContext<'_>,
    step: impl FnOnce(&mut semio_plugin_reactor_lifecycle::instance_lifetime::NativeLifetimeSlot<PA>, &mut PA, &mut StepContext<'_>) -> Result<R, OriginalTurnFailure>,
) -> Result<Option<R>, OriginalTurnFailure> {
    let key = runtime.live_instance_key(instance)?;
    let instances = runtime.instances.try_borrow().map_err(|_| reactor_close_fault("original media instance registry is busy"))?;
    let cell = instances.get(instance).ok_or_else(|| reactor_close_fault("original media instance is missing"))?;
    let mut active = match cell.instance.try_lock() {
        Ok(value) => value,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(None),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err(reactor_close_fault("original media app is poisoned").into()),
    };
    let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("original media lifetime is busy"))?;
    let slot = lifetimes.get_mut(instance).ok_or_else(|| reactor_close_fault("original media lifetime is missing"))?;
    if !slot.admit_command_origin(key, cx)? { return Ok(None); }
    step(slot, &mut active.app, cx).map(Some)
}

fn fault(cursor: CommandPageCursor, fault: &[u8]) -> CommandIngressStatus {
    CommandIngressStatus::Fault { cursor, fault: fault.to_vec() }
}

fn reactor_close_fault(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, message)
}
