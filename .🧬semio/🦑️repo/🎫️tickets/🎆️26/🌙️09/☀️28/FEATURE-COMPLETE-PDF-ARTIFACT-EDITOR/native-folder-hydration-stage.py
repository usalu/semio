"""📥 Connect verified original folder pages to the existing typed Store archive load."""
from pathlib import Path
from difflib import unified_diff
import runpy

ticket = Path(__file__).resolve().parent
base = runpy.run_path(str(ticket / "large-reopen-base.py"))
root, out = base["ROOT"], base["OUT"]
os = base["CHANNEL"].parent.parent
changes = {}


def edit(path, old, new):
    original = path.read_text()
    current = changes.get(path, (original, original))[1]
    assert current.count(old) == 1, (path, old)
    changes[path] = original, current.replace(old, new)


def add(path, filename):
    assert not path.exists(), path
    changes[path] = "", (ticket / filename).read_text()


spr = os / "📡️spr/🦀️.rs"
edit(spr, "pub use protocol::ProtocolError;", "pub use protocol::{ProtocolError, RetainedWireByteSource};")
media = os / "🔌️plugin/🗃️document"
doc = media / "🗄️document-archive-load/🦀️.rs"
edit(media / "🦀️.rs", "mod document_archive_load;", '''mod document_archive_load;
#[path = "🗄️document-archive-load/📥️paged-parent.rs"]
mod paged_parent;''')
add(doc.with_name("📥️paged-parent.rs"), "native-folder-paged-parent.rs")
edit(doc, "    pub archive: Option<Box<protocol::DocumentArchivePack>>,", """    pub archive: Option<Box<protocol::DocumentArchivePack>>,
    pub paged_parent: Option<super::paged_parent::PagedArchiveParent<P>>,""")
edit(doc, "            archive: Some(Box::new(archive)),", """            archive: Some(Box::new(archive)),
            paged_parent: None,""")
edit(doc, "self.archive.is_none() && self.archive_retirement.is_none()", "self.archive.is_none() && self.paged_parent.is_none() && self.archive_retirement.is_none()")
edit(doc, "|| (self.archive.is_none()", "|| (self.archive.is_none() && self.paged_parent.is_none()")
edit(doc, "        if self.history_custody.is_some()||self.decoded_history.is_some(){", '''        if let Some(parent)=self.paged_parent.as_ref(){
            if parent.has_receiver(){return parent.receiver_close_demand(body);}
            if parent.pages.is_some(){
                return Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<protocol::channel::archive_ingress::DocumentArchiveParentPages>(), depth: 2, ..Default::default() });
            }
            return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<super::paged_parent::PagedArchiveParent<P>>(), depth: 1, ..Default::default() });
        }
        if self.history_custody.is_some()||self.decoded_history.is_some(){''')
edit(doc, '''            let source=&self.archive.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"archive history decoder lost its original source"))?.parent_spr;''',
     '''            let source: &dyn protocol::RetainedWireByteSource = match self.paged_parent.as_ref() {
                Some(parent) => parent.spr().ok_or_else(|| ValueError::invariant("archive history lost its paged SPR"))?,
                None => &self.archive.as_ref().ok_or_else(|| ValueError::invariant("archive history lost its original SPR"))?.parent_spr,
            };''')
edit(doc, "        if self.history_custody.is_none(){self.history_custody=self.decoded_history.take();}", '''        if let Some(parent)=self.paged_parent.as_mut(){
            if parent.has_receiver(){
                let step=Exit::resume(parent.close_receiver(effect))?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if parent.pages.is_some(){
                return match semio_framework_value::retirement::admit_owned_retirement(&mut parent.pages,effect).result {
                    Ok(Some((owner,progress)))=>{self.retained=Some(owner);Ok(RetainedCloneStep::Progress(progress))},
                    Ok(None)=>Ok(RetainedCloneStep::Progress(empty)),
                    Err(error)=>Err(error.into())
                };
            }
            if !effect.performed(1)? { return Ok(RetainedCloneStep::Progress(empty)); }
            self.paged_parent.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if self.history_custody.is_none(){self.history_custody=self.decoded_history.take();}''')
anchor = "    pub fn status(&self) -> protocol::DocumentArchiveLoadStatus {"
edit(doc, anchor, '''    pub fn new_paged(
        operation: u64,
        archive: protocol::channel::archive_ingress::VerifiedDocumentArchiveParent,
        history: protocol::RetainedHistoryDecode,
    ) -> Self {
        let mut load = Self::new(operation, protocol::DocumentArchivePack::default(), history);
        load.paged_parent = Some(super::paged_parent::PagedArchiveParent::new(archive.into_pages()));
        load
    }

''' + anchor)

manifest = media / "📦️packages/🦀️rust/Cargo.toml"
edit(manifest, "[dependencies]\n", "[dependencies]\nsemio-framework-hash = { workspace = true }\n")
lock = root / "Cargo.lock"
entry = lock.read_text().split('name = "semio-plugin-document-media"\n')[1].split('\n[[package]]', 1)[0]
assert ' "semio-framework-hash",' not in entry
entry_after = entry.replace(' "semio-framework-job",',' "semio-framework-hash",\n "semio-framework-job",',1)
assert entry_after != entry
edit(lock, 'name = "semio-plugin-document-media"\n' + entry, 'name = "semio-plugin-document-media"\n' + entry_after)

instance = os / "🔌️plugin/🧵️instance"
ladder = instance / "📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs"
edit(ladder, "let turn=match active.work.with_loan(&mut (&actual as &dyn semio_plugin_document_media::ArchiveWorkTurn),|loan|decoder.step(&archive.parent_spr,&actual,grant,&mut active.decoded_history,loan))", '''let source: &dyn protocol::RetainedWireByteSource = match active.paged_parent.as_ref() {
                    Some(parent) => parent.spr().ok_or_else(|| missing("document archive has lost its original paged SPR"))?,
                    None => &archive.parent_spr,
                };
                let turn=match active.work.with_loan(&mut (&actual as &dyn semio_plugin_document_media::ArchiveWorkTurn),|loan|decoder.step(source,&actual,grant,&mut active.decoded_history,loan))''')
anchor = "            ActiveDocumentArchiveLoadPhase::HydrateParent => {\n                if active.hydration.is_none() {\n                    let (owners, built)"
edit(ladder, anchor, '''            ActiveDocumentArchiveLoadPhase::HydrateParent => {
''' + '''                if let Some(parent) = active.paged_parent.as_mut() {
                    if active.hydration.is_none() {
                        if !parent.receiver_ready() {
                            return Ok(PluginLifecycleStep::Progress(parent.advance(cx)?));
                        }
                        let history = active.decoded_history.as_ref().ok_or_else(|| missing("paged archive parent history was not decoded"))?;
                        let current = stores.store.envelope();
                        if history.doc_id != current.id || history.schema != A::DOCUMENT_SCHEMA {
                            return Err(missing("paged folder archive history changed its document or schema").into());
                        }
                        let (owners, built) = A::build_document_store_owners(grant).ok_or_else(|| missing("paged archive requires the app's exact Store owner catalog"))?.map_err(owners_admission_fault)?;
                        let mut initial = None;
                        let moved = parent.take_initial(&mut initial, grant)?;
                        let Some(initial) = initial else { return Ok(PluginLifecycleStep::Progress(built)); };
                        let pages = parent.pages.as_mut().ok_or_else(|| missing("verified Pack owner is absent"))?;
                        let pack = std::mem::take(&mut pages.pack);
                        let history = active.decoded_history.take().expect("checked original paged archive History");
                        let expected = store::PersistedDocumentHydrationIdentity { id: history.doc_id.clone(), dialect: Some(A::DIALECT.into()) };
                        active.hydration = Some(store::RetainedPersistedDocumentHydration::from_decoded_pack(
                            initial, pack, parent.digest(), history, expected, current.owner.clone(),
                            A::DOCUMENT_SCHEMA.to_string(), owners, cx.operation(), cx.generation(),
                            u64::MAX, store::PersistedDocumentHydrationTarget::Envelope, stores.store.local_actor_id().clone(),
                        ));
                        return Ok(PluginLifecycleStep::Progress(built.checked_add(moved).map_err(plugin_retirement_fault)?));
                    }
                    if parent.has_receiver() {
                        let Some((closed, work)) = cx.with_original_effect(grant, |_, effect| parent.close_receiver(effect))? else {
                            return Ok(PluginLifecycleStep::Progress(empty));
                        };
                        cx.consume_work(work);
                        return match closed {
                            semio_framework_value::RetainedWorkPoll::Pending { .. } => Ok(PluginLifecycleStep::Progress(empty)),
                            semio_framework_value::RetainedWorkPoll::Complete(turn) => {
                                let progress = turn.result.map_err(|error| archive_retirement_failure(error, None, empty))?.progress();
                                cx.consume_retained(progress)?;
                                Ok(PluginLifecycleStep::Progress(progress))
                            }
                        };
                    }
                }''' + """\n                if active.hydration.is_none() {\n                    let (owners, built)""")
app_impl = instance / "🔌️plugin-app-impl/🦀️.rs"
anchor = "    fn begin_document_archive_load(&mut self,operation:"
edit(app_impl, anchor, '''    fn validate_folder_archive_target(&self, document: &str, schema: &str) -> bool {
        self.stores.store.envelope().id == document && A::DOCUMENT_SCHEMA == schema
    }

    fn begin_paged_document_archive_load(
        &mut self, operation: u64,
        archive: &mut Option<protocol::channel::archive_ingress::VerifiedDocumentArchiveParent>,
        cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        if !self.ingest.document_archive_loads.can_insert(operation) {
            return Err(plugin_sdk_fault("paged archive operation is already live or its registry slot is busy").into());
        }
        let Some(source) = archive.as_ref() else { return Err(plugin_sdk_fault("paged archive admission lost its verified original").into()); };
        let capacity = size_of::<protocol::DocumentArchivePack>();
        let charge = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>>() + capacity,
            retained_capacity_bytes: capacity, ..Default::default()
        };
        if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() == 0 || !charge.fits(cx.retained_grant()) {
            return Ok(false);
        }
        let history = document_archive_history_decoder(source.spr_len()).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let load = ActiveDocumentArchiveLoad::new_paged(operation, archive.take().expect("checked verified archive pages"), history);
        self.ingest.document_archive_loads.insert_admitted(operation, load);
        cx.consume_retained(charge)?;
        cx.consume_fuel(1);
        Ok(true)
    }

''' + anchor)

patch = []
for path, (old, new) in changes.items():
    assert len(str(path).encode("utf-16-le")) // 2 <= 256, path
    assert (path.read_text() if path.is_file() else "") == old, path
    rel = path.relative_to(root)
    patch.extend(unified_diff(old.splitlines(True), new.splitlines(True), f"a/{rel}" if old else "/dev/null", f"b/{rel}"))
(out / "native-folder-hydration.patch").write_text("".join(patch))
(out / "folder_parent.rs").write_text((ticket / "native-folder-paged-parent.rs").read_text())
