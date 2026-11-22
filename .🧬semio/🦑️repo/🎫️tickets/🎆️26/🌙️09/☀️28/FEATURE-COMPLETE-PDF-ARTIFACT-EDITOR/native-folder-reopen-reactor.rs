//! 🚪 Authenticates native folder pages before ordinary Shell effect dispatch.
use semio_framework::kernel::{Effect, Event, FolderArchiveLoadEvent, MessageEndpoint, PluginInstanceId};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError};
use semio_os_kernel::channel::archive_ingress::folder_archive_load::{FolderArchiveLoadAction, FolderArchiveLoadReply, FolderArchiveLoadRequest};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;

pub fn is_folder_archive_event(event: &Event) -> bool {
    matches!(event, Event::FolderArchiveLoad(load) if load.is_authentic())
}

pub fn drive_folder_archive<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, input: &mut Option<Event>, effects: &mut Vec<Effect>, maximum_effects: usize, cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    let Some(Event::FolderArchiveLoad(FolderArchiveLoadEvent { instance, request: payload })) = input.as_ref().filter(|event| is_folder_archive_event(event)) else {
        return Ok(());
    };
    let request = FolderArchiveLoadRequest::decode(payload).filter(|request| request.key.instance == *instance)
        .ok_or_else(|| ValueError::invariant("folder archive request changed its authenticated Shell instance"))?;
    let shell = MessageEndpoint::Shell { instance: PluginInstanceId(instance.to_string()) };
    if effects.len() >= maximum_effects { return Ok(()); }

    let response_capacity = FolderArchiveLoadReply::MAXIMUM_BYTES;
    let grow = effects.len() == effects.capacity();
    let prospective_effects = if grow { (effects.len() + 1) * size_of::<Effect>() } else { 0 };
    let released_effects = if grow { effects.capacity() * size_of::<Effect>() } else { 0 };
    let progress = RetainedCloneProgress {
        copied_items: 1, copied_bytes: response_capacity + size_of::<Effect>() + size_of::<Option<Event>>(),
        retained_capacity_bytes: response_capacity + prospective_effects,
        released_bytes: payload.capacity() + released_effects,
        ..Default::default()
    };
    let grant = cx.retained_grant();
    let work = progress.copied_bytes as u64;
    if cx.should_yield() || cx.fuel_remaining() < work || grant.maximum_depth == 0 || grant.maximum_items < 2 || !progress.fits(grant) {
        return Ok(());
    }
    let mut output = Vec::new();
    output.try_reserve_exact(response_capacity).map_err(|_| ValueError::invariant("folder archive response refused its bounded backing"))?;
    effects.try_reserve_exact(1).map_err(|_| ValueError::invariant("folder archive effect could not reserve its recipient"))?;
    let result = cx.with_retained_loan(RetainedCloneGrant {
        maximum_items: grant.maximum_items - progress.copied_items,
        maximum_copy_bytes: grant.maximum_copy_bytes - progress.copied_bytes,
        maximum_capacity_bytes: grant.maximum_capacity_bytes - progress.retained_capacity_bytes,
        maximum_release_bytes: grant.maximum_release_bytes - progress.released_bytes,
        ..grant
    }, |cx| cx.with_original_work_loan(cx.fuel_remaining() - work, u64::MAX, |cx| {
        super::with_slot(runtime, request.key.instance, cx, |slot, app, cx| {
            let target = match request.action {
                FolderArchiveLoadAction::Begin { document, schema, .. } => app.validate_folder_archive_target(document, schema),
                _ => true,
            };
            slot.folder_archive.turn(slot.cell.lifetime(), request, target, |operation, verified, cx| {
                app.begin_paged_document_archive_load(operation, verified, cx)
            }, cx)
        })
    }));
    let reply = match result {
        Ok(Some(reply)) => reply,
        Ok(None) => FolderArchiveLoadReply::Pending,
        Err(error) => {
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: prospective_effects, released_bytes: released_effects, ..Default::default() })?;
            return Err(error);
        }
    };
    if !reply.encode_into(request, &mut output) {
        return Err(ValueError::invariant("folder archive response exceeded its exact backing").into());
    }
    cx.consume_retained(progress)?;
    cx.consume_fuel(work);
    input.take();
    effects.push(Effect::SendMessage { target: shell, payload: output });
    Ok(())
}
