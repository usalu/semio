
/// 🧊️ Accepts the inbound cold document pair page, begins its document load and steps the pending load; returns the ingress status this turn answers.
pub(super) async fn drive_cold_pair_ingress<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework::kernel::ColdPairIngressStatus {
    let mut cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Idle;
    let cold_pair_page_lifetime = cold_pair_page.as_ref().map(|page| page.header.lifetime);
    if let Some(page) = cold_pair_page {
        let lifetime = page.header.lifetime;
        let transfer_generation = page.header.transfer_generation;
        supersede_cold_pair_document_load(runtime, lifetime, transfer_generation).await;
        let terminal_cursor = page.header.cursor(page.header.page_count.saturating_sub(1));
        let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
        let another_loads = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow().as_ref().is_some_and(|pending| pending.cursor.lifetime != lifetime));
        cold_pair_ingress = if another_loads && page.page_index.saturating_add(1) == page.header.page_count {
            semio_framework::kernel::ColdPairIngressStatus::Backpressure(page.header.cursor(page.page_index))
        } else {
            COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().accept_page(&page, live))
        };
        if matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_)) {
            let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
            let load = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().begin_load(lifetime, transfer_generation, live));
            match load.map(|load| cold_pair::ColdPairDocumentLoad::new(load, transfer_generation, terminal_cursor)) {
                Some(pending) => match semio_framework_plugin_runtime_ops::ops_document::plugin_begin_document_archive_load(runtime, lifetime.instance_id, pending.operation, pending.archive()).await {
                    Ok(()) => COLD_PAIR_DOCUMENT_LOAD.with(|slot| *slot.borrow_mut() = Some(pending)),
                    Err(fault) => cold_pair_ingress = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().finish_load(pending.load, Err(semio_framework_diagnostic::encode_fault_bytes(&fault)), live)),
                },
                None => cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Fault { cursor: terminal_cursor, fault: b"cold-pair.load-admission".to_vec() },
            }
        }
    }
    if let Some(pending) = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow_mut().take()) {
        let answers_this_turn = !pending.superseded && cold_pair_page_lifetime.is_none_or(|lifetime| lifetime == pending.cursor.lifetime);
        let status = step_cold_pair_document_load(runtime, pending, identity, cx).await;
        if answers_this_turn {
            cold_pair_ingress = status;
        }
    }
    cold_pair_ingress
}
