#!/usr/bin/env python3
"""🚪️ S19 one-off codemod (set `gen-archive-load`): generation3d + generation2d load a framework document archive into a
fresh instance (the shell's Import Document door: `createApp` → `loadDocumentArchive` → poll → acknowledge).

Measured live 2026-09-28 (S20 io-matrix `s14-s20-io/s20b-local-en-2/procedural-generation{3d,2d}.console.txt`): generation3d
trapped the guest (`ordered-map root must be explicitly retired before drop` → `wasm unreachable`), generation2d faulted
`plugin.internal.document-archive-replacement.initializer-failed`. Root cause: both editors declare
`REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY` and their `validate_document_store_publication` demands a lease in the
process-wide publication table, but outside tests nothing ever admits one for a replacement the HOST began, so every
whole-document load fails its publication (`*-publication.authority-missing`). process3d has carried the fix since 09-16
(memory `project-process3d-load-document-path`): the initializer grants itself ONE app lease (base = parent = live = the
generation the host started on, the domain's own credits) when no host lease exists, every lookup reads the host table first
and the self-grant second, and the grant is released at validation, cancellation, fault and close. This ports that shape
verbatim into both generation artifacts (the three templated copies stay structurally identical) and adds one law per
artifact driving the real archive door.

Second fault (overlay proof 1, 2026-09-28 14:34, `s14-s19-logs/gen-archive-proof-1.txt`): with the lease in place generation3d
still trapped — the KERNEL's retained hydrations (`🏪️store/🧾️document/📜️history/💧️hydration`, and its config twin
`🏪️store/🎚️config/📥️retained`) replay every persisted forward as `MutationDiff::apply(operation.diff(x).diff(), x)`, which
plain-drops the outcome's diff instead of routing it through `MutationDiff::retire_cold`; a procedural diff owns
`host_snapshot.layout: OrderedMap`, so the FIRST replayed edit aborts the guest. Both seams now use `os_vcs::apply_mutation`
(the per-step replay transform every store-level fold already uses, which retires the diff). Each law exports an EDITED
document so the replay seam is always exercised (generation2d's first law exported an edit-free document and passed by luck).
Idempotent. usage: s19-gen-archive-load.py <root>"""
import os
import sys

root = sys.argv[1]
ARTIFACTS = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts"
VARIANTS = [("generation3d", "Generation3d", "GENERATION3D", "🧊️generation3d"), ("generation2d", "Generation2d", "GENERATION2D", "🌀️generation2d")]


def edit(rel, change):
    path = os.path.join(root, rel)
    before = open(path, encoding="utf-8").read()
    after = change(before)
    if after != before:
        open(path, "w", encoding="utf-8").write(after)
        print(f"edited {rel}")


def once(text, old, new, done_marker):
    if done_marker in text:
        return text
    assert text.count(old) == 1, f"anchor x{text.count(old)}: {old[:100]!r}"
    return text.replace(old, new)


for v, T, C, d in VARIANTS:
    subset = f"{ARTIFACTS}/{d}/🏅️standards/🔖️1/🪆️subsets/✳️any"
    binary = f"{subset}/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"
    editor = f"{subset}/✏️editor/🦀️.rs"
    law_file = f"{subset}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

    APP_LEASE = f'''
/// 🔐️ The ONE lease the app grants ITSELF for a replacement the host began (`loadDocumentArchive`, a whole-document load
/// → `build_document_store_initialization_job`): base, parent and live revision are all the generation the host started
/// the replacement on — the only publication that commit can accept — with the domain's own credits. A holder that
/// admitted a HOST lease for the same operation first keeps it (`Err`). The twin of `process3d_app_publication_lease`.
///
/// A self-grant is NOT a host publication and does not live in the host's fixed direct-mapped table, where it would refuse
/// an unrelated host publication mapping to the same slot. The host drives one replacement per instance at a time, so this
/// authority holds exactly one lease and a new self-grant supersedes the load the host already abandoned; it is released
/// through `{v}_release_app_publication_authority` at validation, cancellation, fault and initializer close.
fn {v}_app_publication_lease() -> &'static std::sync::Mutex<Option<(semio_framework_job::FixedOperationKey, {T}PublicationLease)>> {{
    static LEASE: std::sync::OnceLock<std::sync::Mutex<Option<(semio_framework_job::FixedOperationKey, {T}PublicationLease)>>> = std::sync::OnceLock::new();
    LEASE.get_or_init(|| std::sync::Mutex::new(None))
}}

/// 🔎️ One lease by its exact key, from the host table first and the app self-grant second.
fn {v}_publication_lease_by_key(key: semio_framework_job::FixedOperationKey) -> Result<Option<{T}PublicationLease>, &'static str> {{
    let leases = {v}_publication_leases().try_lock().map_err(|_| "{v}-publication.contended")?;
    if let Some(lease) = leases.get(key) {{
        return Ok(Some(*lease));
    }}
    drop(leases);
    let app = {v}_app_publication_lease().try_lock().map_err(|_| "{v}-publication.contended")?;
    Ok(app.as_ref().filter(|(held, _)| *held == key).map(|(_, lease)| *lease))
}}

/// 🔎️ One lease by operation, from the host table first and the app self-grant second.
fn {v}_publication_lease_by_operation(operation: semio_framework_job::OperationId) -> Result<Option<{T}PublicationLease>, &'static str> {{
    let leases = {v}_publication_leases().try_lock().map_err(|_| "{v}-publication.contended")?;
    if let Some((_, lease)) = leases.get_operation(operation) {{
        return Ok(Some(*lease));
    }}
    drop(leases);
    let app = {v}_app_publication_lease().try_lock().map_err(|_| "{v}-publication.contended")?;
    Ok(app.as_ref().filter(|(_, lease)| lease.operation == operation.0).map(|(_, lease)| *lease))
}}

/// 🔐️ Grants the app's own lease for `operation` — see [`{v}_app_publication_lease`].
pub fn {v}_admit_app_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(), &'static str> {{
    let leases = {v}_publication_leases().try_lock().map_err(|_| "{v}-publication.contended")?;
    if leases.get_operation(operation).is_some() {{
        return Err("{v}-publication.operation-duplicate");
    }}
    drop(leases);
    let mut app = {v}_app_publication_lease().try_lock().map_err(|_| "{v}-publication.contended")?;
    if app.as_ref().is_some_and(|(_, lease)| lease.operation == operation.0) {{
        return Err("{v}-publication.operation-duplicate");
    }}
    *app = Some((
        {v}_publication_key(operation, generation),
        {T}PublicationLease {{
            operation: operation.0,
            generation: generation.0,
            base_revision: generation.0,
            parent_revision: generation.0,
            live_revision: generation.0,
            maximum_items: {C}_MAXIMUM_DOMAIN_ITEMS,
            maximum_output_pages: {C}_MOUNTED_OUTPUT_CHANNELS,
            maximum_controls: {C}_MOUNTED_CONTROL_CREDITS,
            closing: false,
            terminal: false,
        }},
    ));
    Ok(())
}}

/// 🔐️ Releases the app-admitted lease of `operation` (a host-admitted one is the host's to release).
pub fn {v}_release_app_publication_authority(operation: semio_framework_job::OperationId) -> bool {{
    let Ok(mut app) = {v}_app_publication_lease().try_lock() else {{ return false }};
    if app.as_ref().is_none_or(|(_, lease)| lease.operation != operation.0) {{
        return false;
    }}
    app.take().is_some()
}}
'''

    def binary_change(text):
        text = once(text, f'''        .map_err(|_| "{v}-publication.saturated")
}}

pub fn {v}_refresh_publication_authority''', f'''        .map_err(|_| "{v}-publication.saturated")
}}
{APP_LEASE}
pub fn {v}_refresh_publication_authority''', f"fn {v}_app_publication_lease()")
        text = once(text, f'''    let mut leases = {v}_publication_leases().try_lock().map_err(|_| "{v}-publication.contended")?;
    let lease = leases.get_mut({v}_publication_key(operation, generation)).ok_or("{v}-publication.stale-authority")?;
    lease.live_revision = live_revision;
    Ok(())''', f'''    let key = {v}_publication_key(operation, generation);
    let mut leases = {v}_publication_leases().try_lock().map_err(|_| "{v}-publication.contended")?;
    if let Some(lease) = leases.get_mut(key) {{
        lease.live_revision = live_revision;
        return Ok(());
    }}
    drop(leases);
    let mut app = {v}_app_publication_lease().try_lock().map_err(|_| "{v}-publication.contended")?;
    let Some((_, lease)) = app.as_mut().filter(|(held, _)| *held == key) else {{ return Err("{v}-publication.stale-authority") }};
    lease.live_revision = live_revision;
    Ok(())''', "let Some((_, lease)) = app.as_mut().filter(|(held, _)| *held == key)")
        text = once(text, f'''pub fn {v}_validate_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(u64, u64), &'static str> {{
    let leases = {v}_publication_leases().try_lock().map_err(|_| "{v}-publication.contended")?;
    let lease = leases.get({v}_publication_key(operation, generation)).ok_or("{v}-publication.stale-authority")?;''', f'''pub fn {v}_validate_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(u64, u64), &'static str> {{
    let lease = {v}_publication_lease_by_key({v}_publication_key(operation, generation))?.ok_or("{v}-publication.stale-authority")?;''', f'''-> Result<(u64, u64), &'static str> {{
    let lease = {v}_publication_lease_by_key(''')
        text = once(text, f'''    let leases = {v}_publication_leases().try_lock().map_err(|_| "{v}-publication.contended")?;
    let lease = leases.get_operation(operation).map(|(_, lease)| *lease).ok_or("{v}-publication.authority-missing")?;''', f'''    let lease = {v}_publication_lease_by_operation(operation)?.ok_or("{v}-publication.authority-missing")?;''', f"let lease = {v}_publication_lease_by_operation(operation)?")
        text = once(text, f'''pub fn {v}_publication_item_credit(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<usize, &'static str> {{
    let leases = {v}_publication_leases().try_lock().map_err(|_| "{v}-publication.contended")?;
    let lease = leases.get({v}_publication_key(operation, generation)).ok_or("{v}-publication.stale-authority")?;''', f'''pub fn {v}_publication_item_credit(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<usize, &'static str> {{
    let lease = {v}_publication_lease_by_key({v}_publication_key(operation, generation))?.ok_or("{v}-publication.stale-authority")?;''', f'''-> Result<usize, &'static str> {{
    let lease = {v}_publication_lease_by_key(''')
        text = once(text, f'''    fn new(envelope: store::ArtifactEnvelope<{T}Snapshot, {T}Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {{
        let (base_revision, parent_revision) = {v}_validate_publication_authority(operation, generation).unwrap_or((u64::MAX, u64::MAX));''', f'''    fn new(envelope: store::ArtifactEnvelope<{T}Snapshot, {T}Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {{
        if {v}_validate_publication_authority(operation, generation).is_err() {{
            let _ = {v}_admit_app_publication_authority(operation, generation);
        }}
        let (base_revision, parent_revision) = {v}_validate_publication_authority(operation, generation).unwrap_or((u64::MAX, u64::MAX));''', f"let _ = {v}_admit_app_publication_authority(operation, generation);")
        text = once(text, f'''            {T}StoreInitializationPhase::RetireCancelled | {T}StoreInitializationPhase::RetireFault => match self.pump_retirement({C}_OWNER_BYTES) {{
                Ok(false) => return semio_framework_job::StepOutcome::Yield,
                Ok(true) => {{
                    *self.initial_digest = None;''', f'''            {T}StoreInitializationPhase::RetireCancelled | {T}StoreInitializationPhase::RetireFault => match self.pump_retirement({C}_OWNER_BYTES) {{
                Ok(false) => return semio_framework_job::StepOutcome::Yield,
                Ok(true) => {{
                    {v}_release_app_publication_authority(self.operation);
                    *self.initial_digest = None;''', f'''                Ok(true) => {{
                    {v}_release_app_publication_authority(self.operation);
                    *self.initial_digest = None;''')
        text = once(text, f'''        match self.pump_retirement(maximum_bytes.min({C}_OWNER_BYTES)) {{
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending {{ released_items: 1, released_bytes: 0 }}),
            Ok(true) => {{
                *self.initial_digest = None;''', f'''        match self.pump_retirement(maximum_bytes.min({C}_OWNER_BYTES)) {{
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending {{ released_items: 1, released_bytes: 0 }}),
            Ok(true) => {{
                {v}_release_app_publication_authority(self.operation);
                *self.initial_digest = None;''', f'''            Ok(true) => {{
                {v}_release_app_publication_authority(self.operation);
                *self.initial_digest = None;
                *self.edit_digest = None;
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)''')
        return text

    edit(binary, binary_change)

    def editor_change(text):
        return once(text, f'''        crate::standards::v1::subsets::any::schema::mutations::binary::{v}_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "{T} atomic publication authority is absent or stale"))
    }}''', f'''        crate::standards::v1::subsets::any::schema::mutations::binary::{v}_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "{T} atomic publication authority is absent or stale"))?;
        crate::standards::v1::subsets::any::schema::mutations::binary::{v}_release_app_publication_authority(operation);
        Ok(())
    }}''', f"binary::{v}_release_app_publication_authority(operation);\n        Ok(())")

    edit(editor, editor_change)

    if v == "generation3d":
        seat = f'''    context::dispatch(&mut source, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample {{ example_id: crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }})).await;
    context::settle(&mut source).await;
'''
        read = "context::snapshot"
        close = "semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *{app});"
        deref = "&mut *"
        ref = "&*"
    else:
        seat = '''    context::dispatch(&mut source, Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, format: None, action: None, x: None, y: None })).await;
'''
        read = "context::snapshot_read"
        close = "context::close({app});"
        deref = "&mut "
        ref = "&"
    LAW = f'''
//#region 🚪️ArchiveImportDoor
/// 🚪️ A framework document archive of this editor loads into a FRESH instance through the shell's Import Document door
/// (`createApp` → `loadDocumentArchive` → poll → acknowledge). Measured live 2026-09-28 (S20 io-matrix,
/// `s14-s20-io/s20b-local-en-2`): generation3d trapped the guest, generation2d faulted
/// `document-archive-replacement.initializer-failed`. Two faults: the initializer held no publication lease outside tests
/// (`{v}-publication.authority-missing`) — it now grants itself the lease the host's replacement needs — and the kernel's
/// retained hydration plain-dropped every replayed diff, whose `host_snapshot.layout` `OrderedMap` aborts on drop — it now
/// retires them through `os_vcs::apply_mutation`. The source document is EDITED first so the archive carries history to
/// replay (ticket 26/09/23 slice S19).
#[semio_framework_async_macros::async_test]
async fn a_document_archive_loads_into_a_fresh_instance_through_the_import_door() {{
    let _serial = crate::publication_authority::lock();
    let mut source = app_with_registry().await;
{seat}    let archive = PluginApp::document_archive({ref}source).await.expect("the open document exports as a framework archive");
    let expected: Vec<String> = {read}({ref}source).host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    assert!(!expected.is_empty(), "the exported document carries widgets to compare");
    let mut target = app_with_registry().await;
    PluginApp::begin_document_archive_load({deref}target, 91, archive).expect("archive admission");
    let mut status = None;
    for _ in 0..200_000 {{
        let polled = PluginApp::poll_document_archive_load({deref}target, 91).await.expect("archive status");
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {{
            PluginApp::acknowledge_document_archive_load({deref}target, 91).expect("archive acknowledgement");
            status = Some(polled);
            break;
        }}
        PluginApp::maintenance_step({deref}target, 1, 4_096).expect("archive maintenance step");
    }}
    let status = status.expect("the archive load reaches a terminal state");
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{{}}", String::from_utf8_lossy(&status.fault));
    let loaded: Vec<String> = {read}({ref}target).host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    assert_eq!(loaded, expected, "the fresh instance holds exactly the archived document");
    {close.format(app="target")}
    {close.format(app="source")}
}}
//#endregion 🚪️ArchiveImportDoor
'''

    def law_change(text):
        start, end = "\n//#region 🚪️ArchiveImportDoor\n", "//#endregion 🚪️ArchiveImportDoor\n"
        if start in text:
            head, rest = text.split(start, 1)
            return head + LAW + rest.split(end, 1)[1]
        return text.rstrip("\n") + "\n" + LAW

    edit(law_file, law_change)


HYDRATIONS = [
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs", "MemberOpenDiagnostic",
     "use crate::{CompositionPin, Edit, FromValue, Mutation, MutationDiff, OpBinary, OpText, ToValue};",
     "use crate::{CompositionPin, Edit, FromValue, Mutation, OpBinary, OpText, ToValue};"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs", "ConfigStoreHydrationDiagnostic",
     "ErasedSnapshotRetirement, FromValue, Mutation, MutationDiff, OpBinary,",
     "ErasedSnapshotRetirement, FromValue, Mutation, OpBinary,"),
]


def hydration_change(diagnostic, old_use, new_use):
    def change(text):
        for projection in ("validation", "current"):
            old = f"""                        let next = match MutationDiff::apply(operation.diff({projection}).diff(), {projection}) {{
                            Ok(next) => next,
                            Err(_) => return self.reject({diagnostic}::Replay),"""
            new = f"""                        let next = match crate::os_vcs::apply_mutation({projection}, operation) {{
                            Ok((next, _)) => next,
                            Err(_) => return self.reject({diagnostic}::Replay),"""
            text = once(text, old, new, new)
        return once(text, old_use, new_use, new_use)
    return change


for rel, diagnostic, old_use, new_use in HYDRATIONS:
    edit(rel, hydration_change(diagnostic, old_use, new_use))
