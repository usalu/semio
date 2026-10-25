//! 🧵️ Retained commands of the shooting editor: the bounded tool ids and limits and the bounded command job factory. See [`super`].

use super::*;

//#region 🔖️ShootingPlayApp
//#region 🧵️RetainedCommands
/// 🧵️ Every shell-reachable verb is a bounded first-step tool (ticket 26/09/16/SHOOTING-PLUGIN-END-TO-END):
/// the framework refuses UI dispatch of any command not classified `Migrated`, so a partial roster left
/// every panel/measure/viewport gesture dead. Order mirrors the `ShootingCommand` rows.
pub const SHOOTING_BOUNDED_TOOL_IDS: &[&str] = &[
    "importSnapshotJson",
    "setActiveExample",
    "setActiveShot",
    "setActiveAsset",
    "setShotCamera",
    "saveCamera",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setAmbientIntensity",
    "setMaterialRoughness",
    "setShadowEnabled",
    "toggleSun",
    "setActiveShotLabel",
    "setActiveShotFormat",
    "setActiveShotShape",
    "patchShots",
    "patchAssets",
    "addShot",
    "addAsset",
    "importAsset",
    "resetSnapshot",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "setCamera",
    "loadSavedCamera",
    "setCenterModel",
    "setShotSelection",
    "worldPointerDown",
    "worldPointerMove",
    "saveDownload",
    "importAssetRequest",
    "exportActiveShot",
    "exportAllShots",
];

const SHOOTING_RETAINED_PAYLOAD_SCHEMA: &str = "shooting.shooting.tool-command.v1";

pub const SHOOTING_BOUNDED_RAW_BYTES: usize = 65_536;

pub const SHOOTING_BOUNDED_WORK_ITEMS: usize = 1;

fn shooting_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SHOOTING_BOUNDED_RAW_BYTES, 64, SHOOTING_BOUNDED_WORK_ITEMS as u64, 262_144, 7_500)
}

pub fn shooting_bounded_extent(command: &ShootingCommand, _snapshot: &ShootingSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    SHOOTING_BOUNDED_TOOL_IDS.contains(&command.command_id()).then_some(SHOOTING_BOUNDED_WORK_ITEMS)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub fn shooting_bounded_reduce(
    command: &ShootingCommand,
    snapshot: &ShootingSnapshot,
    config: &ShootingConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<ShootingPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, Fault> {
    if !SHOOTING_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("shooting.retained.route"), "the bounded Shooting reducer rejects resumable routes"));
    }
    let mut ctx = ShootingDispatchCtx { selected_asset_ids: interaction.selection.get(SHOOTING_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default() };
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None }, &mut ctx)
}

pub struct ShootingCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ShootingCommandJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: SHOOTING_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for ShootingCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<ShootingPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<ShootingPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SHOOTING_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        shooting_bounded_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: SHOOTING_BOUNDED_RAW_BYTES, checkpoint_resume: false);

}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ShootingCommandJobFactory {
    type Owner = EditorApp<ShootingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = SHOOTING_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "importSnapshotJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setActiveShot", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveAsset", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setShotCamera", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "saveCamera", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setAmbientIntensity", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setMaterialRoughness", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setShadowEnabled", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveShotLabel", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveShotFormat", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveShotShape", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchShots", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchAssets", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addShot", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "addAsset", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "importAsset", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "resetSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "loadSavedCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setCenterModel", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setShotSelection", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "worldPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "saveDownload", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importAssetRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportActiveShot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportAllShots", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️ShootingPlayApp
