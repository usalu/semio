/// 🪆️ The capture law owns its literal workingGraph parent declaration and distinct target address.
#[derive(Clone, Default, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct CaptureParentSnapshot {
    working_graph: Vec<store::ArtifactChild<TestSnapshot>>,
}

impl semio_framework_schema_composition::ArtifactCompositionFields for CaptureParentSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        semio_framework_schema_composition::ChildFieldRefs::visit_child_field(&self.working_graph, "workingGraph", visitor)
    }
    fn child_slots() -> &'static [semio_framework_schema_composition::ChildSlotSpec] {
        &[semio_framework_schema_composition::ChildSlotSpec { name: "workingGraph", kind: "s.test.child", many: true }]
    }
}

impl semio_framework_value::retirement::RetireOwned for CaptureParentSnapshot {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.working_graph)
    }
}

impl store::ArtifactDsl for CaptureParentSnapshot {
    const EXTENSION: &'static str = "child-capture-parent";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let value: serde_json::Value = serde_json::from_str(text).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(&value)).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, error.message, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        serde_json::Value::from(&semio_framework_value::ToValue::to_value(self)).to_string()
    }
}

impl store::ArtifactPack for CaptureParentSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <semio_framework_value::DslValue as store::ArtifactPack>::encode_pack_with(&semio_framework_value::ToValue::to_value(self), options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = <semio_framework_value::DslValue as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        <Self as semio_framework_value::FromValue>::from_value(value).map_err(store::PackError::from)
    }
}

impl ::protocol::MutationDiff<CaptureParentSnapshot> for NoConfig {
    fn apply(&self, base: &CaptureParentSnapshot) -> protocol::MutationApplyResult<CaptureParentSnapshot> { Ok(base.clone()) }
    fn absorb(&mut self, _other: Self) {}
}

impl ::protocol::Mutation<CaptureParentSnapshot> for NoConfigMutation {
    type Diff = NoConfig;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = &[];
    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor { match *self {} }
    fn diff(&self, _base: &CaptureParentSnapshot) -> ::protocol::MutationOutcome<NoConfig> { match *self {} }
    fn inverse(&self, _base: &CaptureParentSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> { match *self {} }
}

impl protocol::SemanticMutation<CaptureParentSnapshot> for NoConfigMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] { &[] }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor { match *self {} }
    fn label(&self) -> LocalizedLabel { match *self {} }
    fn target(&self) -> Vec<String> { match *self {} }
}

#[derive(Default)]
struct CaptureParentApp;

impl ArtifactApp for CaptureParentApp {
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.capture", standard: StandardId("native"), subset: SubsetId::ANY };
    const APP_ID: &'static str = "s.test.capture@native/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.capture-parent/v1";
    type Snapshot = CaptureParentSnapshot;
    type Mutation = NoConfigMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = NoConfigMutation;
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_diagnostic::FaultOrigin::Plugin, semio_framework_diagnostic::FaultCode::new("child-capture.parent-projection"), format!("capture parent declaration: {error:?}")))
    }
    async fn initial_snapshot() -> Self::Snapshot {
        let contract: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("closed original capture contract");
        let identity: serde_json::Value = serde_json::from_str(include_str!("../../🪪️identity/🧫️fixtures/🔣️.json")).expect("closed distinct target contract");
        CaptureParentSnapshot { working_graph: vec![store::ArtifactChild::new(contract["childId"].as_str().expect("logical child key").to_string(), ArtifactRef { artifact_id: identity["target"].as_str().expect("original artifact target").to_string(), dialect: test_child_dialect().await })] }
    }
    async fn handle(command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation> { match *command {} }
    async fn render(_body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data("Captured parent"))
    }
}

/// 🌱️ Creates a real loaded parent declaration without changing any existing fixture app or helper.
async fn capture_contract_app_raw() -> VcsArtifactApp<CaptureParentApp, TestMembers> {
    let definition = App::from_builder(App::builder(CaptureParentApp::APP_ID, LocalizedLabel::data("Captured parent")).await.document(["workingGraph"])).await;
    let mut app = VcsArtifactApp::<CaptureParentApp, TestMembers>::with_registry(CaptureParentApp, AppActionRegistry::from_definition(&definition.definition)).await;
    app.bind_instance_id(meta().instance_id).await;
    app
}
