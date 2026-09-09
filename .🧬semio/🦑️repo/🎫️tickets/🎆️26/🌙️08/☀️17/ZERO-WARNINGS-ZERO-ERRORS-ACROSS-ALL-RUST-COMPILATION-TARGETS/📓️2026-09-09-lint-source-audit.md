# Lint Source Audit

Read-only comparison of completed wasm1044 lint contexts with current source. A changed context is not proof of a fix; fresh strict compilation remains required.

- clippy::vec_init_then_push ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs; unchanged context: false

        let mut mutations = Vec::with_capacity(20);
        mutations.push(En1995Mutation::ChangeAnnex(set_snapshot::ChangeAnnex { new_annex: snapshot.annex }));
        mutations.push(En1995Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: snapshot.m_ed_knm }));
        mutations.push(En1995Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: snapshot.n_ed_kn }));
        mutations.push(En1995Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: snapshot.v_ed_kn }));
        mutations.push(En1995Mutation::ChangeWMm3(change_w_mm3::ChangeWMm3 { new_w_mm3: snapshot.w_mm3 }));
        mutations.push(En1995Mutation::ChangeAMm2(change_a_mm2::ChangeAMm2 { new_a_mm2: snapshot.a_mm2 }));
        mutations.push(En1995Mutation::ChangeBMm(change_b_mm::ChangeBMm { new_b_mm: snapshot.b_mm }));
        mutations.push(En1995Mutation::ChangeHMm(change_h_mm::ChangeHMm { new_h_mm: snapshot.h_mm }));
        mutations.push(En1995Mutation::ChangeFMK(change_f_m_k::ChangeFMK { new_f_m_k: snapshot.f_m_k }));
        mutations.push(En1995Mutation::ChangeFC0K(change_f_c_0_k::ChangeFC0K { new_f_c_0_k: snapshot.f_c_0_k }));
        mutations.push(En1995Mutation::ChangeServiceClass(change_service_class::ChangeServiceClass { new_service_class: snapshot.service_class.clone() }));
        mutations.push(En1995Mutation::ChangeLoadDuration(change_load_duration::ChangeLoadDuration { new_load_duration: snapshot.load_duration.clone() }));
        mutations.push(En1995Mutation::ChangeMCritKnm(change_m_crit_knm::ChangeMCritKnm { new_m_crit_knm: snapshot.m_crit_knm }));
        mutations.push(En1995Mutation::ChangeFEdKn(change_f_ed_kn::ChangeFEdKn { new_f_ed_kn: snapshot.f_ed_kn }));
        mutations.push(En1995Mutation::ChangeAEfMm2(change_a_ef_mm2::ChangeAEfMm2 { new_a_ef_mm2: snapshot.a_ef_mm2 }));
        mutations.push(En1995Mutation::ChangeFVK(change_f_v_k::ChangeFVK { new_f_v_k: snapshot.f_v_k }));
        mutations.push(En1995Mutation::ChangeFireDurationMin(change_fire_duration_min::ChangeFireDurationMin { new_fire_duration_min: snapshot.fire_duration_min }));
        mutations.push(En1995Mutation::ChangeSectionDepthMm(change_section_depth_mm::ChangeSectionDepthMm { new_section_depth_mm: snapshot.section_depth_mm }));
        mutations.push(En1995Mutation::ChangeAVertMS2(change_a_vert_m_s2::ChangeAVertMS2 { new_a_vert_m_s2: snapshot.a_vert_m_s2 }));
        mutations.push(En1995Mutation::ChangeNCyclesBridge(change_n_cycles_bridge::ChangeNCyclesBridge { new_n_cycles_bridge: snapshot.n_cycles_bridge }));

- clippy::large_enum_variant ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs; unchanged context: true

pub enum QueryPreparationStep {
    Pending,
    Complete(QueryExecution),
}

- clippy::field_reassign_with_default ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs; unchanged context: false

            metadata.name = std::mem::take(&mut self.graph.name);

- clippy::too_many_arguments ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs; unchanged context: false

    pub fn with_content(schema: String, name: String, manifest_id: Option<String>, manifest: Manifest, camera: Camera, nodes: Vec<Node>, edges: Vec<Edge>, root_node_id: Option<String>) -> Self {

- clippy::large_enum_variant ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs; unchanged context: true

enum JackSnapshotDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

- clippy::large_enum_variant ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs; unchanged context: true

enum JackMutationDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

- clippy::new_without_default ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs; unchanged context: true

    pub fn new() -> Self {
        Self::with_local_owner(true)
    }

- clippy::too_many_arguments ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn jack_retained_window_config_reduce(
    command: &TrinityJackCommand,
    _snapshot: &JackSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TrinityJackPlayApp>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<TrinityGraphMutation, NoConfigMutation, NoDraftMutation>, Fault> {

- clippy::needless_update ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs; unchanged context: false

        Self { exaggeration: snapshot.exaggeration, imported_features_json: snapshot.imported_features_json, mesh: snapshot.mesh, ..Self::default() }

- clippy::too_many_arguments ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn gis3d_retained_reduce(
    command: &Gis3dCommand,
    snapshot: &GisTerrainSnapshot,
    config: &Gis3dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gis3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation, NoDraftMutation>, Fault> {

- clippy::needless_update ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🦀️.rs; unchanged context: false

            MutationOutcome::new(Gis3dConfigDelta { camera_json: Some(self.camera_json.clone()), ..Default::default() }.into())

- clippy::empty_line_after_outer_attr ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs; unchanged context: true

#[artifact_schema(id = "s.procedural.generation2d")]


- clippy::empty_line_after_outer_attr ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs; unchanged context: true

#[artifact_schema(id = "s.procedural.generation2d")]


- clippy::empty_line_after_outer_attr ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs; unchanged context: true

#[artifact_schema(id = "s.procedural.generation2d")]


- clippy::empty_line_after_doc_comments ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs; unchanged context: false

/// would align them; not editable here — glue.rs is shared with the sibling `generation3d` artifact).


- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs; unchanged context: false

                    let numbers = value.as_array().map(<[DslValue]>::to_vec).unwrap_or_else(|| question.fields.as_deref().unwrap_or_default().iter().map(|field| DslValue::float(field.value.unwrap_or(0.0))).collect());

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs; unchanged context: false

                    let labels: Vec<String> = question
                        .fields
                        .as_deref()
                        .map(|fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect())
                        .unwrap_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect());

- clippy::derivable_impls ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs; unchanged context: false

impl Default for Generation2dArtifact {
    fn default() -> Self {
        Self { fixture: FlowFixture::default(), generation: GenerationPlayRoot::default() }
    }
}

- clippy::needless_update ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs; unchanged context: false

        Self { fixture: snapshot.fixture, generation: snapshot.generation, ..Self::default() }

- clippy::derivable_impls ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs; unchanged context: false

impl Default for Generation2dSnapshot {
    fn default() -> Self {
        Self { fixture: FlowFixture::default(), generation: GenerationPlayRoot::default() }
    }
}

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs; unchanged context: false

    fn begin_dsl(&mut self) -> Result<bool, &'static str> {

- clippy::single_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs; unchanged context: false

            Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
                Some(
                    Generation2dMountedContainerOwner::Synapses { field, present, next, .. } | Generation2dMountedContainerOwner::Generations { field, present, next, .. } | Generation2dMountedContainerOwner::Dictionary { field, present, next, .. },
                ) => {
                    *field = Some(u16::try_from(value).map_err(|_| "generation2d-mounted.table-field")?);
                    present.fill(false);
                    *next = 0;
                }
                _ => {}
            },

- clippy::collapsible_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs; unchanged context: false

                    if value == 0 {
                        present.fill(true);
                    }

- clippy::single_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs; unchanged context: false

            Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
                Some(Generation2dMountedContainerOwner::Synapses { present, .. } | Generation2dMountedContainerOwner::Generations { present, .. } | Generation2dMountedContainerOwner::Dictionary { present, .. }) => {
                    for bit in 0..8 {
                        let row = first_row as usize + bit;
                        if row < present.len() {
                            present[row] = value & (1 << bit) != 0;
                        }
                    }
                }
                _ => {}
            },

- clippy::needless_pass_by_value ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs; unchanged context: false

pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation2dDiff {

- clippy::needless_pass_by_value ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs; unchanged context: false

pub fn diff_generation_from_ops(base: &Generation2dSnapshot, ops: Vec<GenerationMutation>) -> Generation2dDiff {

- clippy::too_many_arguments ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

pub fn generation2d_admit_publication_authority(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    live_revision: u64,
    maximum_items: usize,
    maximum_output_pages: usize,
    maximum_controls: usize,
) -> Result<(), &'static str> {

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                Generation2dReplayDisplaced::Camera(value) => drop(value),

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

fn generation2d_retire_displaced(value: Generation2dReplayDisplaced) -> Option<Box<dyn store::ErasedSnapshotRetirement>> {

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

    fn begin_dsl(&mut self) -> Result<bool, &'static str> {

- clippy::single_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

            Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::Dictionary { field, present, next, .. }) => {
                    *field = Some(u16::try_from(value).map_err(|_| "generation2d-mutation.dictionary-field")?);
                    present.fill(false);
                    *next = 0;
                }
                _ => {}
            },

- clippy::collapsible_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    if value == 0 {
                        present.fill(true);
                    }

- clippy::single_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

            Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::Dictionary { present, .. }) => {
                    for bit in 0..8 {
                        let row = first_row as usize + bit;
                        if row < present.len() {
                            present[row] = value & (1 << bit) != 0;
                        }
                    }
                }
                _ => {}
            },

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        drop(self.layout.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        drop(self.camera.take());

- clippy::manual_range_contains ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        if expected_bytes < 3 || expected_bytes > GENERATION2D_OWNER_BYTES || maximum_items == 0 {

- clippy::collapsible_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    if let store::mounted_pack_rt::RetainedRecordBodyToken::Value(token) = event {
                        let complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
                        let body = self.body.as_ref().expect("P2 retained mutation body");
                        self.owner.as_mut().expect("P2 retained mutation owner").accept(token, body)?;
                        if complete {
                            self.phase = Generation2dMutationSessionPhase::Ready;
                            return Ok(true);
                        }
                    }

- clippy::clone_on_copy ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        dsl::DslValue::Number(value) => dsl::DslValue::Number(value.clone()),

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    drop(self.candidate_disposer.take());

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                        let id = entry
                            .mutation_meta
                            .get(index)
                            .and_then(|meta| meta.mutation_id.as_ref())
                            .map(|id| protocol::MutationId(generation2d_copy_string(&id.0).unwrap_or_default()))
                            .unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    drop(self.initial_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    drop(self.edit_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        drop(self.initial_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        drop(self.edit_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                drop(self.initial_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                drop(self.edit_digest.take());

- clippy::explicit_auto_deref ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),

- clippy::explicit_auto_deref ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn generation2d_bounded_extent(_command: &Generation2dCommand, _snapshot: &Generation2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {

- clippy::too_many_arguments ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn generation2d_retained_reduce(
    command: &Generation2dCommand,
    snapshot: &Generation2dSnapshot,
    config: &Generation2dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation2dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation, NoDraftMutation>, Fault> {

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs; unchanged context: false

        let body = store::semio_format::split_text_preamble(text).map(|(_, rest)| rest).unwrap_or(text);

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️generation/🦀️.rs; unchanged context: false

                ("generationId".into(), payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null)),

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️generation/🦀️.rs; unchanged context: false

            let args = dsl::DslValue::object([("id".into(), payload.id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null))]);

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️select-generation/🦀️.rs; unchanged context: false

    let args = dsl::DslValue::object([("id".into(), payload.id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null))]);

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️update-generation-values/🦀️.rs; unchanged context: false

        ("generationId".into(), payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null)),

- clippy::unnecessary_to_owned ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs; unchanged context: false

    let widget_items = crate::ui_node_list(document.fixture.widgets.iter().map(|widget| tree_item(widget_id(widget).to_string(), widget_id(widget).to_string())))?;

- clippy::empty_line_after_outer_attr ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs; unchanged context: true

#[artifact_schema(id = "s.procedural.generation3d")]


- clippy::empty_line_after_outer_attr ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs; unchanged context: true

#[artifact_schema(id = "s.procedural.generation3d")]


- clippy::empty_line_after_outer_attr ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs; unchanged context: true

#[artifact_schema(id = "s.procedural.generation3d")]


- clippy::needless_update ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs; unchanged context: false

        Self { fixture: snapshot.fixture, generation: snapshot.generation, ..Self::default() }

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs; unchanged context: false

    fn begin_dsl(&mut self) -> Result<bool, &'static str> {

- clippy::single_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs; unchanged context: false

            Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
                Some(
                    Generation3dMountedContainerOwner::Synapses { field, present, next, .. } | Generation3dMountedContainerOwner::Generations { field, present, next, .. } | Generation3dMountedContainerOwner::Dictionary { field, present, next, .. },
                ) => {
                    *field = Some(u16::try_from(value).map_err(|_| "generation3d-mounted.table-field")?);
                    present.fill(false);
                    *next = 0;
                }
                _ => {}
            },

- clippy::collapsible_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs; unchanged context: false

                    if value == 0 {
                        present.fill(true);
                    }

- clippy::single_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs; unchanged context: false

            Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
                Some(Generation3dMountedContainerOwner::Synapses { present, .. } | Generation3dMountedContainerOwner::Generations { present, .. } | Generation3dMountedContainerOwner::Dictionary { present, .. }) => {
                    for bit in 0..8 {
                        let row = first_row as usize + bit;
                        if row < present.len() {
                            present[row] = value & (1 << bit) != 0;
                        }
                    }
                }
                _ => {}
            },

- clippy::needless_pass_by_value ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs; unchanged context: false

pub fn diff_fixture_from_helpers(base: &Generation3dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation3dDiff {

- clippy::needless_pass_by_value ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs; unchanged context: false

pub fn diff_generation_from_ops(base: &Generation3dSnapshot, ops: Vec<GenerationMutation>) -> Generation3dDiff {

- clippy::too_many_arguments ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

pub fn generation3d_admit_publication_authority(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    live_revision: u64,
    maximum_items: usize,
    maximum_output_pages: usize,
    maximum_controls: usize,
) -> Result<(), &'static str> {

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                Generation3dReplayDisplaced::Camera(value) => drop(value),

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

fn generation3d_retire_displaced(value: Generation3dReplayDisplaced) -> Option<Box<dyn ErasedSnapshotRetirement>> {

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

    fn begin_dsl(&mut self) -> Result<bool, &'static str> {

- clippy::single_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

            Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
                Some(Generation3dMutationFrame::Dictionary { field, present, next, .. }) => {
                    *field = Some(u16::try_from(value).map_err(|_| "generation3d-mutation.dictionary-field")?);
                    present.fill(false);
                    *next = 0;
                }
                _ => {}
            },

- clippy::collapsible_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    if value == 0 {
                        present.fill(true);
                    }

- clippy::single_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

            Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
                Some(Generation3dMutationFrame::Dictionary { present, .. }) => {
                    for bit in 0..8 {
                        let row = first_row as usize + bit;
                        if row < present.len() {
                            present[row] = value & (1 << bit) != 0;
                        }
                    }
                }
                _ => {}
            },

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        drop(self.layout.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        drop(self.camera.take());

- clippy::manual_range_contains ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        if expected_bytes < 3 || expected_bytes > GENERATION3D_OWNER_BYTES || maximum_items == 0 {

- clippy::collapsible_match ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    if let store::mounted_pack_rt::RetainedRecordBodyToken::Value(token) = event {
                        let complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
                        let body = self.body.as_ref().expect("P3 retained mutation body");
                        self.owner.as_mut().expect("P3 retained mutation owner").accept(token, body)?;
                        if complete {
                            self.phase = Generation3dMutationSessionPhase::Ready;
                            return Ok(true);
                        }
                    }

- clippy::clone_on_copy ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        dsl::DslValue::Number(value) => dsl::DslValue::Number(value.clone()),

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    drop(self.candidate_disposer.take());

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                        let id = entry
                            .mutation_meta
                            .get(index)
                            .and_then(|meta| meta.mutation_id.as_ref())
                            .map(|id| protocol::MutationId(generation3d_copy_string(&id.0).unwrap_or_default()))
                            .unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    drop(self.initial_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                    drop(self.edit_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        drop(self.initial_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

        drop(self.edit_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                drop(self.initial_digest.take());

- clippy::drop_non_drop ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs; unchanged context: false

                drop(self.edit_digest.take());

- clippy::explicit_auto_deref ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),

- clippy::explicit_auto_deref ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {

- clippy::too_many_arguments ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn generation3d_retained_reduce(
    command: &Generation3dCommand,
    snapshot: &Generation3dSnapshot,
    config: &Generation3dConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>, Fault> {

- clippy::redundant_closure ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: false

                operations_json: str_arg(&["operationsJson", "operations_json"]).or_else(|| args.get("operations").map(|value| dsl::json::to_json_string(value))).unwrap_or_else(|| "[]".into()),

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️generation/🦀️.rs; unchanged context: false

            let generation_id = payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null);

- clippy::map_unwrap_or ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️update-generation-values/🦀️.rs; unchanged context: false

    let generation_id = payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null);

- clippy::too_many_arguments ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs; unchanged context: false

    fn push_item(&mut self, page: usize, strings: [Option<&str>; 4], numbers: [f64; 16], number_len: u8, indexes: [u32; 8], index_len: u8, flags: u16) -> Result<(), Vec<u8>> {

- clippy::derivable_impls ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs; unchanged context: false

impl Default for Fem3dArtifact {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            elements: Default::default(),
            materials: Default::default(),
            sections: Default::default(),
            solids: Default::default(),
            supports: Default::default(),
            load_cases: Default::default(),
            combinations: Default::default(),
            analysis: Default::default(),
        }
    }
}

- clippy::needless_update ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs; unchanged context: false

            ..Self::default()

- clippy::too_many_arguments ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs; unchanged context: true

fn fem3d_retained_reduce(
    command: &Fem3dCommand,
    snapshot: &Fem3dSnapshot,
    config: &Fem3dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Fem3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, NoDraftMutation>, Fault> {

- clippy::map_unwrap_or ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs; unchanged context: false

            let blocks_json = node.params.iter().find(|param| param.key == "blocksJson").map(|param| param.value.as_str()).unwrap_or("[]");

- clippy::type_complexity ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs; unchanged context: false

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[

- clippy::needless_update ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs; unchanged context: false

        Self { schema: snapshot.schema, id: snapshot.id, version: snapshot.version, title: snapshot.title, document: snapshot.document, flow: snapshot.flow, ..Self::default() }

- clippy::manual_is_multiple_of ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs; unchanged context: false

    if s.len() % 2 != 0 {

- clippy::field_reassign_with_default ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs; unchanged context: false

    snapshot.schema = read_str_lp(&mut reader)?;

- clippy::needless_pass_by_value ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs; unchanged context: false

fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> PlaybookTopology {

- clippy::explicit_auto_deref ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),

- clippy::explicit_auto_deref ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),

- clippy::redundant_clone ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

            return crate::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });

- clippy::redundant_clone ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

            let bytes = crate::io::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;

- clippy::redundant_clone ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

            let bytes = crate::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;

- clippy::redundant_clone ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

            let bytes = crate::io::export::serializers::artifacts::docx::v_ecma_376::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;

- clippy::redundant_clone ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

            let bytes = crate::io::export::serializers::artifacts::md::v_commonmark::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;

- clippy::redundant_clone ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs; unchanged context: false

            let bytes = crate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;

- clippy::useless_format ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs; unchanged context: false

    snap.title = Some(format!("Imported pdf"));

- clippy::useless_format ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs; unchanged context: false

    snap.title = Some(format!("Imported docx"));

- clippy::too_many_arguments ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn playbook_retained_reduce(
    command: &PlaybookCommand,
    snapshot: &PlaybookSnapshot,
    config: &PlaybookConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PlaybookPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation, NoDraftMutation>, Fault> {

- clippy::redundant_clone ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️steps/🦀️.rs; unchanged context: false

            TreeNodeView { id: step.id.clone(), label, children }

- clippy::empty_line_after_doc_comments ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs; unchanged context: false

/// pair of regenerated content-addressed child handles, exactly like every other composed plugin.


- clippy::map_unwrap_or ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs; unchanged context: false

        "const" => Some(FormExpr::Const { value: semio_value_map_get(value, "value").map(dsl_from_semio_value).unwrap_or(dsl::DslValue::Null) }),

- clippy::needless_pass_by_value ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs; unchanged context: false

pub fn forms_snapshot_with_state(schema: String, id: String, version: String, title: Option<String>, steps: Vec<FormStep>) -> FormsSnapshot {

- clippy::type_complexity ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs; unchanged context: false

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[

- clippy::needless_update ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs; unchanged context: false

        Self { schema: snapshot.schema, id: snapshot.id, version: snapshot.version, title: snapshot.title, structure: snapshot.structure, results: snapshot.results, ..Self::default() }

- clippy::needless_pass_by_value ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs; unchanged context: false

fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> FormsTopology {

- clippy::manual_is_multiple_of ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs; unchanged context: false

    if s.len() % 2 != 0 {

- clippy::needless_pass_by_value ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs; unchanged context: false

pub fn forms_diff_from_delta(delta: FormsStepsDelta, base: &FormsSnapshot) -> FormsDiff {

- clippy::needless_borrow ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: false

                raw.push_str(&chunk);

- clippy::unnecessary_lazy_evaluations ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: false

    (payload.question_kind == kind).then(|| QuestionKindRoute { app_id: payload.app_id, params_body_key: payload.params_body_key, preview_body_key: payload.preview_body_key })

- clippy::unnecessary_wraps ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn forms_bounded_extent(_command: &FormsCommand, _snapshot: &FormsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {

- clippy::too_many_arguments ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn forms_retained_reduce(
    command: &FormsCommand,
    snapshot: &FormsSnapshot,
    config: &FormsConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<FormsPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<FormMutation, FormsConfigMutation, NoDraftMutation>, Fault> {

- clippy::len_without_is_empty ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs; unchanged context: true

    pub fn len(&self) -> usize {

- clippy::type_complexity ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs; unchanged context: false

static ACTIVE_TRY_VALUE_GENERATIONS: OnceLock<Mutex<BTreeMap<(String, String, String), u64>>> = OnceLock::new();

- clippy::result_large_err ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs; unchanged context: true

fn put_session(key: FormsJobKey, session: TryValueSession) -> Result<(), TryValueSession> {

- clippy::map_unwrap_or ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs; unchanged context: false

                local_end += chunk[local_start..].chars().next().map(char::len_utf8).unwrap_or(1);

- clippy::map_unwrap_or ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs; unchanged context: false

        next = end.min(start + source[start..].chars().next().map(char::len_utf8).unwrap_or(1));

- clippy::needless_pass_by_value ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs; unchanged context: false

fn continuation_emit(generation: u64, next: SetTryValueStep) -> Emit<FormMutation, FormsConfigMutation> {

- clippy::needless_pass_by_value ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs; unchanged context: false

fn finish_try_value(generation: u64, session: TryValueSession, mutations: Vec<FormsConfigMutation>) -> Emit<FormMutation, FormsConfigMutation> {

- clippy::obfuscated_if_else ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs; unchanged context: false

                source: current_content_id
                    .is_none()
                    .then(|| {
                        let fallback = match &rewrite {
                            TryValueRewrite::Vector(_) => "null",
                            TryValueRewrite::Container(ContainerRewrite { edit: ContainerEdit::Option { .. }, .. }) => "[]",
                            TryValueRewrite::Container(ContainerRewrite { edit: ContainerEdit::Object { .. }, .. }) => "{}",
                        };
                        ChunkedSource::from_text(fallback.into())
                    })
                    .unwrap_or_default(),

- clippy::map_unwrap_or ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs; unchanged context: false

    let chunk = payload.value_json.as_ref().map(ChunkAddressableJson::owner).unwrap_or_else(|| std::sync::Arc::from("false"));

- clippy::type_complexity ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-try-values/🦀️.rs; unchanged context: false

static ACTIVE_BULK_GENERATIONS: OnceLock<Mutex<BTreeMap<(String, String, String), (u64, String)>>> = OnceLock::new();

- clippy::type_complexity ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-try-values/🦀️.rs; unchanged context: false

fn active_bulk_generations() -> &'static Mutex<BTreeMap<(String, String, String), (u64, String)>> {

- clippy::needless_pass_by_value ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/▶️try/🦀️.rs; unchanged context: false

fn read_only_field(question: &FormQuestion, value_text: String) -> UiAssemblyResult<ui::BuiltNode> {

- clippy::needless_pass_by_value ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🦀️.rs; unchanged context: false

pub fn set_vortex_kinds_parts(catalog: &mut store::ArtifactChild<SemioKitSnapshot>, extra: &mut Vec<Block3dVortexKindExtra>, kinds: Vec<Block3dVortexKind>) {

- clippy::too_many_arguments ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: true

fn block3d_retained_reduce(
    command: &Block3dCommand,
    snapshot: &Block3dSnapshot,
    config: &Block3dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Block3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Block3dMutation, Block3dConfigMutation, NoDraftMutation>, Fault> {

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🦀️.rs; unchanged context: true

fn preview_footprint(_: &Block3dWorldWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {

- clippy::derivable_impls ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs; unchanged context: false

impl Default for NoteCompositeWindowConfig {
    fn default() -> Self {
        Self { camera: NoteCamera::default() }
    }
}

- clippy::manual_range_patterns ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs; unchanged context: false

            [2, edge, 0 | 1 | 2] => {

- clippy::manual_range_patterns ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs; unchanged context: false

            [2, edge, slot @ (0 | 1 | 2), 0] => {

- clippy::manual_range_patterns ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs; unchanged context: false

            [1, _, 0] | [2, _, 0 | 1 | 2] => &["value"],

- clippy::unnecessary_wraps ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs; unchanged context: false

    fn close_json_cursor(cursor: &mut Option<JsonValidationCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage) -> Result<PluginCloseStep, Fault> {

- clippy::unnecessary_wraps ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs; unchanged context: false

    fn close_typed_cursor(cursor: &mut Option<TypedJsonCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {

- clippy::unnecessary_wraps ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs; unchanged context: false

    fn close_optional_string(value: &mut Option<String>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {

- clippy::unnecessary_wraps ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs; unchanged context: false

    fn close_required_string(value: &mut String, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {

- clippy::unusual_byte_groupings ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🦀️.rs; unchanged context: false

    SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_nanos() as u64).unwrap_or(0xC0FF_EE00_D15E_A5E)

- clippy::redundant_clone ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🦀️.rs; unchanged context: false

        operator_info("math.add", "Add", "Add", "Adds numbers, points, or vectors", scalar.clone(), sum_output.clone()),

- clippy::redundant_clone ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🦀️.rs; unchanged context: false

        operator_info("math.subtract", "Subtract", "Sub", "Subtracts numbers, points, or vectors", subtract_scalar.clone(), vec![difference_out()]),

- clippy::redundant_clone ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🦀️.rs; unchanged context: false

    register_simple(registry, operator_info("math.sum", "Sum", "Sum", "Sums numbers in a list dictionary", vec![ChannelSpec::list("list", &["math.sum"])], sum_output.clone()), Sum, vec!["list"], &["number"]);

- clippy::needless_borrow ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🦀️.rs; unchanged context: true

        let indices = list_indices(&list);

- clippy::needless_borrow ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🦀️.rs; unchanged context: true

        let next = list_indices(&list).len();

- clippy::needless_borrow ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🦀️.rs; unchanged context: true

        Ok(channel_output("count", number_dictionary(list_indices(&list).len() as f64)))

- clippy::needless_borrow ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🦀️.rs; unchanged context: true

        Ok(channel_output("list", remove_list_index(&list, index)))

- clippy::needless_pass_by_value ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs; unchanged context: false

fn map_kernel_error(error: DrawingError) -> EvalError {

- clippy::unnecessary_wraps ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs; unchanged context: false

fn read_rgba(input: &Dictionary, key: &str) -> Result<[f64; 4], EvalError> {

- clippy::too_many_arguments ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs; unchanged context: true

fn space_bounded_reduce(
    command: &SpaceCommand,
    snapshot: &WorkflowSnapshot,
    config: &SpaceConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<SpaceApp>>,
    operation: &AppOperationContext,
) -> Result<Emit<WorkflowMutation, SpaceConfigMutation, NoDraftMutation>, Fault> {
