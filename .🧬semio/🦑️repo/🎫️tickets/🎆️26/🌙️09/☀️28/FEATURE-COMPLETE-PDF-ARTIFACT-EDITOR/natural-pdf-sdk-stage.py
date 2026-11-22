"""Stage SDK registration and the owned paged-input reserved worker."""
from pathlib import Path
from runpy import run_path

ticket = Path(__file__).parent
integration = run_path(str(ticket / "natural-pdf-integration-stage.py"))
staged = integration["staged"]
change = staged["change"]
emit = staged["emit"]
changes = staged["changes"]
output = staged["output"]
plugin = staged["plugin"]
decl = plugin / "📜️declaration"

change(decl / "🎞️media/📥️original-media/📥️natural-source.rs", [
    ("type Input<const N: usize> = ControlledRetirement<PagedBytes<N>>;",
     """type Input<const N: usize> = ControlledRetirement<PagedBytes<N>>;

/// 🧩 The original paid source passed from fixed-page ingress to its natural file decoder.
pub type NaturalFilePagedSource = Input<{ super::natural_source_gate::NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES as usize }>;

/// 🚪 One native natural source receiver under the transport's total extent ceiling.
pub type NaturalFileSourceGuest = NaturalSourceGuest<{ super::natural_source_gate::NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES as usize }>;"""),
    ("    pub fn sequence(&self) -> u64 { self.seq }",
     """    pub fn sequence(&self) -> u64 { self.seq }

    pub fn format_runnable(&self) -> bool { matches!(self.stage, Stage::Sealed | Stage::Lent | Stage::Closing) }"""),
])
facade = plugin / "🦀️.rs"
base = revised = facade.read_text(encoding="utf-8")
changes[facade] = base, revised + "\npub use semio_plugin_decl_media::original_media::natural_source::NaturalFilePagedSource;\n"

change(decl / "👷️job/🧵️reserved-tool-job/🦀️.rs", [
    ("/// 🧭️ Owned input for an app-defined implementation", """/// 🛤️ Internal factory key for the registered, paid paged natural-file route.
pub const NATURAL_SOURCE_TOOL_ID: &str = "import-natural-source";
/// 🗝️ Exact payload schema admitted by the paged natural-file factory.
pub const NATURAL_SOURCE_TOOL_SCHEMA: &str = "framework.reserved.import-natural-source.v1";
/// 📏 Bounds retained decoded output and Pack separately from the paid source extent.
pub const NATURAL_SOURCE_TOOL_CONTRACT: semio_framework::ToolExecutionContract =
    semio_framework::ToolExecutionContract::resumable(1, 8_192, 4_096, 33_554_432, 7_500, 1, 1);

/// ✅ Checks the purpose-bound request metadata before the paged decoder receives its source.
pub fn admits_paged_natural_request(
    tool_id: &str, schema: &str, contract: semio_framework::ToolExecutionContract, raw_wire_bytes: usize,
) -> bool {
    tool_id == NATURAL_SOURCE_TOOL_ID && schema == NATURAL_SOURCE_TOOL_SCHEMA
        && contract == NATURAL_SOURCE_TOOL_CONTRACT && raw_wire_bytes == 0
}

/// 🧭️ Owned input for an app-defined implementation"""),
    ("    OriginalPicker { input: Box<semio_plugin_decl_media::original_media::RawMediaInput> },",
     """    OriginalPicker { input: Box<semio_plugin_decl_media::original_media::RawMediaInput> },
    NaturalPaged { port: String, source: semio_plugin_decl_media::original_media::natural_source::NaturalFilePagedSource },"""),
])
change(decl / "🧩️app/🎛️artifact-app-trait/🦀️.rs", [
    ("    /// 💾️ Encodes genuine natural-format bytes;", """    /// 🧮 Maximum selected natural bytes when the app installs an original paged decoder.
    fn natural_file_paged_maximum_bytes() -> u32 { 0 }

    /// 💾️ Encodes genuine natural-format bytes;"""),
])
change(decl / "🪟️surface/✏️artifact-editor-trait/🦀️.rs", [
    ("    /// 🎟️ Moves the original byte buffer", """    /// 📐 Enables the paged natural route when its decoder admission is implemented.
    const PAGED_NATURAL_SOURCE_MAXIMUM_BYTES: u32 = 0;

    /// 🧳 Receives the paid natural byte owner under the format's granted decoder turn.
    fn admit_paged_natural_file_decode_owner(
        _source: &mut Option<semio_plugin_decl_media::original_media::natural_source::NaturalFilePagedSource>,
        _contract: semio_framework::ToolExecutionContract,
        _receiving: &mut Option<NaturalFileDecodeOwner<Self::Snapshot>>,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,
            "natural-file format has no original paged decoder").into())
    }

    /// 🎟️ Moves the original byte buffer"""),
])
change(decl / "🪟️surface/🖊️editor-app/🦀️.rs", [
    ("        E::build_reserved_tool_job(request,body,construction,cx)", """        if request.as_ref().is_some_and(|request| matches!(request.input.as_ref(), Some(ArtifactReservedToolInput::NaturalPaged { .. }))) {
            use semio_plugin_decl_app::tool_contract::admits_paged_natural_request;
            let original = request.as_ref().expect("paged request was observed");
            if E::PAGED_NATURAL_SOURCE_MAXIMUM_BYTES == 0 || !admits_paged_natural_request(
                &original.tool_id, &original.payload_schema_id, original.contract, original.raw_wire.len(),
            ) {
                return Err(semio_framework_value::ValueError::invariant("paged natural input requires its exact reserved factory contract").into());
            }
            return semio_plugin_decl_app::tool_contract::admit_reserved_work::<Self, crate::natural_file_import::NaturalFileImportWork<E>>(
                request, body, construction, cx,
            );
        }
        if request.as_ref().is_some_and(|request| request.tool_id == semio_plugin_decl_app::tool_contract::NATURAL_SOURCE_TOOL_ID) {
            return Err(semio_framework_value::ValueError::invariant("paged natural factory requires its original source").into());
        }
        E::build_reserved_tool_job(request,body,construction,cx)"""),
    ("    async fn encode_natural_file(snapshot:", """    fn natural_file_paged_maximum_bytes() -> u32 { E::PAGED_NATURAL_SOURCE_MAXIMUM_BYTES }

    async fn encode_natural_file(snapshot:"""),
])

job = decl / "📥️natural-file/🦀️.rs"
change(job, [
    ("progress.consumed_input_bytes > self.byte_length || progress.consumed_input_bytes > self.contract.max_raw_wire_bytes",
     "progress.consumed_input_bytes > self.byte_length"),
    ("use semio_plugin_decl_app::NaturalFileDecodeOwner;", "use semio_plugin_decl_app::NaturalFileDecodeOwner;\nuse semio_plugin_decl_media::original_media::natural_source::NaturalFilePagedSource;"),
    ("    pending_bytes: Option<Vec<u8>>,", "    pending_bytes: Option<Vec<u8>>,\n    pending_paged: Option<NaturalFilePagedSource>,"),
    ("pub(crate) struct NaturalFileImportWork<E:ArtifactEditor>", """fn admits_paged_input(port: &str, binary: bool, length: usize, maximum: u32, raw_wire_bytes: usize) -> bool {
    port == semio_plugin_decl_app::NATURAL_FILE_PORT && binary && raw_wire_bytes == 0
        && length != 0 && length <= maximum as usize
}

pub(crate) struct NaturalFileImportWork<E:ArtifactEditor>"""),
    ("            pending_bytes: None,", "            pending_bytes: None,\n            pending_paged: None,"),
    ("&&self.pending_bytes.is_none()&&self.pending_media_value.is_none()", "&&self.pending_bytes.is_none()&&self.pending_paged.is_none()&&self.pending_media_value.is_none()"),
    ("        if self.pending_bytes.is_some(){return Ok(", """        if let Some(source) = self.pending_paged.as_ref() {
            return if source.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<NaturalFilePagedSource>>(), depth: 1, ..Default::default() })
            } else { source.next_demand(body) };
        }
        if self.pending_bytes.is_some(){return Ok("""),
    ("        if self.pending_bytes.is_some(){return admit(", """        if let Some(source) = self.pending_paged.as_mut() {
            if !source.terminal_is_empty() { return source.step(body, effect); }
            let bytes = size_of::<Option<NaturalFilePagedSource>>();
            let work = record_retirement_work(effect, 1);
            if work.is_ok() { self.pending_paged.take(); }
            return settled(work.map(|_| RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }))
                .map_err(|(error, _)| error), effect.performed_work());
        }
        if self.pending_bytes.is_some(){return admit("""),
])
base, revised = changes[job]
start = revised.index("  if self.job.is_none(){let bytes=size_of::<NaturalFileImportJob<E>>()")
end = revised.index("\n }\n}\n", start)
replacement = """        if self.job.is_none() {
            let copied_bytes = size_of::<NaturalFileImportJob<E>>() + size_of::<ArtifactReservedToolInput>()
                + size_of::<ArtifactToolCompletion<EditorApp<E>>>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() };
            if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() == 0
                || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
                return Ok(None);
            }
            let codec = E::natural_file_codec().ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner,
                "natural-file work lost its declared codec"))?;
            let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue,
                "natural-file work source does not match its declared format or byte bound");
            let byte_length = match request.input.as_ref() {
                Some(ArtifactReservedToolInput::Media { port, media }) => {
                    let MediaPayload::Intrinsic { schema, value: DslValue::Bytes(input) } = &media.payload else { return Err(invalid().into()); };
                    if port != semio_plugin_decl_app::NATURAL_FILE_PORT || schema != codec.format_kind
                        || input.len() > request.contract.max_raw_wire_bytes { return Err(invalid().into()); }
                    input.len()
                }
                Some(ArtifactReservedToolInput::NaturalPaged { port, source }) => {
                    let length = source.original().ok_or_else(invalid)?.len();
                    if !semio_plugin_decl_app::tool_contract::admits_paged_natural_request(
                        &request.tool_id, &request.payload_schema_id, request.contract, request.raw_wire.len(),
                    ) { return Err(invalid().into()); }
                    if !admits_paged_input(port, codec.binary, length, E::PAGED_NATURAL_SOURCE_MAXIMUM_BYTES, request.raw_wire.len()) {
                        return Err(invalid().into());
                    }
                    length
                }
                _ => return Err(invalid().into()),
            };
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            let (port, media, pending_bytes, pending_paged) = match request.input.take().expect("verified natural source") {
                ArtifactReservedToolInput::Media { port, mut media } => {
                    let MediaPayload::Intrinsic { value: DslValue::Bytes(source), .. } = &mut media.payload else { unreachable!() };
                    let bytes = std::mem::take(source);
                    (port, Some(media), Some(bytes), None)
                }
                ArtifactReservedToolInput::NaturalPaged { port, source } => (port, None, None, Some(source)),
                _ => unreachable!("verified natural source purpose"),
            };
            self.job = Some(NaturalFileImportJob {
                port, media, decoder: None, completion: Some(request.completion.clone()), emit: None,
                pending_completion_rejection: None, retirement: None, pending_snapshot: None, pack_export: None,
                verified_pack: None, pending_genesis: None, publication_work: 0, pending_bytes, pending_paged,
                pending_media_value: None, pending_media_text: None, pending_port: None, contract: request.contract,
                byte_cursor: 0, byte_length, decode_progress: Default::default(), stage: NaturalFileImportStage::Decode,
                completed: false, state: [0;17], fault: semio_plugin_decl_app::tool_contract::BoundedFaultText::new(),
                pending: None, outcomes: semio_plugin_decl_app::tool_contract::ReservedJobOutcomes::new(), closing: false,
            });
            return Ok(None);
        }
        let job = self.job.as_mut().unwrap();
        if job.decoder.is_none() {
            if job.pending_paged.is_some() {
                let admitted = E::admit_paged_natural_file_decode_owner(&mut job.pending_paged, job.contract, &mut job.decoder, cx)?;
                if admitted != matches!(job.decoder, Some(NaturalFileDecodeOwner::Controlled(_)))
                    || admitted == job.pending_paged.is_some() || matches!(job.decoder, Some(NaturalFileDecodeOwner::Synchronous(_))) {
                    return Err(invariant("paged natural decoder changed original source ownership").into());
                }
            } else if E::admit_natural_file_decode_owner(&mut job.pending_bytes, job.contract, &mut job.decoder, cx)?
                && matches!(job.decoder, Some(NaturalFileDecodeOwner::Synchronous(_))) {
                job.stage = NaturalFileImportStage::Account;
            }
            return Ok(None);
        }
        job.step_original(cx)"""
changes[job] = base, revised[:start] + replacement + revised[end:]
eligible = changes[job][1]
gate = eligible[eligible.index("fn admits_paged_input"):eligible.index("pub(crate) struct NaturalFileImportWork")]
(output / "sdk-admission.rs").write_text(gate, encoding="utf-8")
emit("natural-source-sdk.patch")
