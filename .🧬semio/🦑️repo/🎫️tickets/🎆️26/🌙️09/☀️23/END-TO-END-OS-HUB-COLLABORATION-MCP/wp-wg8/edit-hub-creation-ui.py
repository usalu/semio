import pathlib
p = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs")
t = p.read_text()
def swap(old, new):
    global t
    assert t.count(old) == 1, old[:90]
    t = t.replace(old, new)

swap('''use semio_framework_os_kernel::os_directory::DirectorySpaceRole;
''', '''use semio_framework_os_kernel::os_directory::schema::space_artifact_creation::{SpaceArtifactCreateV1, SpaceArtifactCreationCatalogV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationReadyV1};
use semio_framework_os_kernel::os_directory::DirectorySpaceRole;
''')
swap('''pub const HUB_INVITE_INPUT_ID: &str = "framework.hub.invite";
''', '''pub const HUB_INVITE_INPUT_ID: &str = "framework.hub.invite";
pub const HUB_ARTIFACT_CREATION_ID: &str = "framework.hub.artifact-creation";
pub const HUB_ARTIFACT_NAME_INPUT_ID: &str = "framework.hub.artifact-name";
''')
swap('''    pub const REDEEM_INVITE: &str = "hubRedeemInvite";
}''', '''    pub const REDEEM_INVITE: &str = "hubRedeemInvite";
    pub const SELECT_ARTIFACT_KIND: &str = "hubSelectArtifactKind";
    pub const SET_ARTIFACT_NAME: &str = "hubSetArtifactName";
    pub const CREATE_ARTIFACT: &str = "hubCreateArtifact";
    pub const CANCEL_ARTIFACT_CREATION: &str = "hubCancelArtifactCreation";
    pub const OPEN_CREATED_ARTIFACT: &str = "hubOpenCreatedArtifact";
}''')
swap('''    pub clipboard_available: bool,
}

impl HubWorkspaceState {''', '''    pub clipboard_available: bool,
    /// 🌱️ The open space's artifact-creation door (catalog, choice, name, the creation in flight).
    pub creation: HubArtifactCreationState,
}

impl HubWorkspaceState {''')
swap('''            clipboard_available: false,
        }
    }''', '''            clipboard_available: false,
            creation: HubArtifactCreationState::default(),
        }
    }''')
swap('''//#region 🌲️RetainedTree
fn descriptor(''', '''//#region 🌱️ArtifactCreation
/// 📚️ Where the open space's selected current creation catalog is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HubArtifactCatalogPhase {
    #[default]
    Idle,
    Loading,
    Ready,
    Unavailable,
}

impl HubArtifactCatalogPhase {
    /// 🏷️ The phase's own wire spelling, so a probe reads it without parsing a translation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Loading => "loading",
            Self::Ready => "ready",
            Self::Unavailable => "unavailable",
        }
    }
}

/// 🚪️ What happened to a ready artifact's opening.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HubArtifactOpening {
    #[default]
    Idle,
    Opening,
    Opened,
    Failed,
}

/// 🌱️ The one creation this workspace drives: the sealed intent, the hub's latest receipt, whether a
/// cancel was asked and sent, and the opening of its result. `deadline_at_ms` bounds the whole
/// operation; past it the outcome is `Indeterminate`, never silently `Failed`.
#[derive(Clone, Debug, PartialEq)]
pub struct HubArtifactCreation {
    pub intent: SpaceArtifactCreateV1,
    pub space_id: String,
    pub phase: SpaceArtifactCreationPhaseV1,
    pub submitted: bool,
    pub cancel_requested: bool,
    pub cancel_sent: bool,
    pub ready: Option<SpaceArtifactCreationReadyV1>,
    pub opening: HubArtifactOpening,
    pub deadline_at_ms: u64,
    pub next_poll_at_ms: u64,
}

/// 🌱️ The open space's artifact-creation door: the hub's selected current catalog, the chosen kind,
/// the name draft and at most one creation in flight — the wgpu twin of React's Space-index create
/// dialog plus `🏛️ShellHost/🌱️artifact-creation`'s progress surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HubArtifactCreationState {
    pub catalog_phase: HubArtifactCatalogPhase,
    pub catalog: Option<SpaceArtifactCreationCatalogV1>,
    pub kind_id: Option<String>,
    pub name_draft: String,
    pub operation: Option<HubArtifactCreation>,
}

/// ⏱️ The whole-operation bound and the poll cadence — the browser worker's
/// `SPACE_ARTIFACT_CREATION_DEADLINE_MS` / `SPACE_ARTIFACT_CREATION_POLL_MS` (`🏪️store/👷️worker/🟦️.ts`).
pub const HUB_ARTIFACT_CREATION_DEADLINE_MS: u64 = 120_000;
pub const HUB_ARTIFACT_CREATION_POLL_MS: u64 = 100;

/// 🏁️ A phase after which the hub will not change the receipt again.
pub fn hub_artifact_creation_terminal(phase: SpaceArtifactCreationPhaseV1) -> bool {
    matches!(phase, SpaceArtifactCreationPhaseV1::Ready | SpaceArtifactCreationPhaseV1::Indeterminate | SpaceArtifactCreationPhaseV1::Failed | SpaceArtifactCreationPhaseV1::Cancelled)
}

/// 🏷️ A phase's wire spelling.
pub fn hub_artifact_creation_phase_str(phase: SpaceArtifactCreationPhaseV1) -> &'static str {
    match phase {
        SpaceArtifactCreationPhaseV1::Accepted => "accepted",
        SpaceArtifactCreationPhaseV1::Preparing => "preparing",
        SpaceArtifactCreationPhaseV1::Ready => "ready",
        SpaceArtifactCreationPhaseV1::Indeterminate => "indeterminate",
        SpaceArtifactCreationPhaseV1::Failed => "failed",
        SpaceArtifactCreationPhaseV1::Cancelled => "cancelled",
    }
}

/// 📥️ The sealed intent a Create click issues, or `None` while the door may not create: the catalog
/// must be ready, the chosen kind one of its rows, the name valid, and no creation still in flight.
pub fn hub_artifact_creation_intent(state: &HubWorkspaceState, request_id: &str) -> Option<SpaceArtifactCreateV1> {
    let creation = &state.creation;
    if state.session.phase != HubSessionPhase::SignedIn || creation.catalog_phase != HubArtifactCatalogPhase::Ready || creation.operation.as_ref().is_some_and(|operation| !hub_artifact_creation_terminal(operation.phase)) {
        return None;
    }
    let catalog = creation.catalog.as_ref()?;
    let kind = catalog.kinds.iter().find(|kind| Some(kind.kind_id.as_str()) == creation.kind_id.as_deref())?;
    let intent = SpaceArtifactCreateV1 {
        schema: "semio.hub.space-artifact-create/v1".into(),
        request_id: request_id.to_string(),
        expected_catalog_generation_id: catalog.catalog_generation_id.clone(),
        kind_id: kind.kind_id.clone(),
        name: creation.name_draft.trim_matches(' ').to_string(),
    };
    intent.validate().then_some(intent)
}

/// 🏷️ The closed set of texts the creation door names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubArtifactCreationLabel {
    Title,
    Name,
    Create,
    Heading,
    Cancel,
    Cancelling,
    Opening,
    OpeningFailed,
    OpenRetry,
    CatalogLoading,
    CatalogReady,
    CatalogUnavailable,
}

/// 🏷️ One door text in both languages this product owns, byte-identical to React's
/// `ARTIFACT_CREATION_PROGRESS_TEXT_V1` (`🏛️ShellHost/🌱️artifact-creation/🟦️.tsx`) where it has one.
pub fn hub_artifact_creation_label(key: HubArtifactCreationLabel, locale: Locale) -> &'static str {
    match (key, locale) {
        (HubArtifactCreationLabel::Title, Locale::En) => "Create an artifact",
        (HubArtifactCreationLabel::Title, Locale::De) => "Artefakt erstellen",
        (HubArtifactCreationLabel::Name, Locale::En) => "Artifact name",
        (HubArtifactCreationLabel::Name, Locale::De) => "Artefaktname",
        (HubArtifactCreationLabel::Create, Locale::En) => "Create artifact",
        (HubArtifactCreationLabel::Create, Locale::De) => "Artefakt erstellen",
        (HubArtifactCreationLabel::Heading, Locale::En) => "Creating artifact",
        (HubArtifactCreationLabel::Heading, Locale::De) => "Artefakt wird erstellt",
        (HubArtifactCreationLabel::Cancel, Locale::En) => "Cancel creation",
        (HubArtifactCreationLabel::Cancel, Locale::De) => "Erstellung abbrechen",
        (HubArtifactCreationLabel::Cancelling, Locale::En) => "Cancellation requested…",
        (HubArtifactCreationLabel::Cancelling, Locale::De) => "Abbruch angefordert…",
        (HubArtifactCreationLabel::Opening, Locale::En) => "Opening the artifact…",
        (HubArtifactCreationLabel::Opening, Locale::De) => "Artefakt wird geöffnet…",
        (HubArtifactCreationLabel::OpeningFailed, Locale::En) => "The artifact was created, but it could not be opened. You can safely try opening it again.",
        (HubArtifactCreationLabel::OpeningFailed, Locale::De) => "Das Artefakt wurde erstellt, konnte aber nicht geöffnet werden. Du kannst das Öffnen sicher erneut versuchen.",
        (HubArtifactCreationLabel::OpenRetry, Locale::En) => "Open artifact",
        (HubArtifactCreationLabel::OpenRetry, Locale::De) => "Artefakt öffnen",
        (HubArtifactCreationLabel::CatalogLoading, Locale::En) => "Loading the available artifact kinds…",
        (HubArtifactCreationLabel::CatalogLoading, Locale::De) => "Verfügbare Artefaktarten werden geladen…",
        (HubArtifactCreationLabel::CatalogReady, Locale::En) => "The available artifact kinds are current.",
        (HubArtifactCreationLabel::CatalogReady, Locale::De) => "Die verfügbaren Artefaktarten sind aktuell.",
        (HubArtifactCreationLabel::CatalogUnavailable, Locale::En) => "Artifact kinds are unavailable. Reopen the space before creating an artifact.",
        (HubArtifactCreationLabel::CatalogUnavailable, Locale::De) => "Artefaktarten sind nicht verfügbar. Öffne den Space erneut, bevor du ein Artefakt erstellst.",
    }
}

/// 🚦️ One creation phase's live text in both languages, byte-identical to React's.
pub fn hub_artifact_creation_phase_text(phase: SpaceArtifactCreationPhaseV1, locale: Locale) -> &'static str {
    match (phase, locale) {
        (SpaceArtifactCreationPhaseV1::Accepted, Locale::En) => "The creation request was accepted.",
        (SpaceArtifactCreationPhaseV1::Accepted, Locale::De) => "Die Erstellungsanfrage wurde angenommen.",
        (SpaceArtifactCreationPhaseV1::Preparing, Locale::En) => "The artifact is being created…",
        (SpaceArtifactCreationPhaseV1::Preparing, Locale::De) => "Das Artefakt wird erstellt…",
        (SpaceArtifactCreationPhaseV1::Ready, Locale::En) => "The artifact is ready.",
        (SpaceArtifactCreationPhaseV1::Ready, Locale::De) => "Das Artefakt ist bereit.",
        (SpaceArtifactCreationPhaseV1::Indeterminate, Locale::En) => "The creation outcome is unknown. Refresh the space before trying again.",
        (SpaceArtifactCreationPhaseV1::Indeterminate, Locale::De) => "Das Ergebnis der Erstellung ist unbekannt. Aktualisiere den Space vor einem neuen Versuch.",
        (SpaceArtifactCreationPhaseV1::Failed, Locale::En) => "Artifact creation failed.",
        (SpaceArtifactCreationPhaseV1::Failed, Locale::De) => "Die Artefakterstellung ist fehlgeschlagen.",
        (SpaceArtifactCreationPhaseV1::Cancelled, Locale::En) => "Artifact creation was cancelled.",
        (SpaceArtifactCreationPhaseV1::Cancelled, Locale::De) => "Die Artefakterstellung wurde abgebrochen.",
    }
}
//#endregion 🌱️ArtifactCreation

//#region 🌲️RetainedTree
fn descriptor(''')
swap('''/// 🏛️ The whole hub workspace as one retained document''', '''/// 🌱️ The open space's creation door: the catalog line, one choice per creatable kind (its own en +
/// de label from the hub), the name, Create, then the one creation's live phase with its Cancel
/// while it runs and its opening once it is ready.
fn hub_artifact_creation_section(state: &HubWorkspaceState, locale: Locale) -> UiNode {
    let creation = &state.creation;
    let busy = creation.operation.as_ref().is_some_and(|operation| !hub_artifact_creation_terminal(operation.phase));
    let mut children = vec![tagged_row(hub_artifact_creation_label(HubArtifactCreationLabel::Title, locale), &[("data-semio-hub-artifact-catalog", creation.catalog_phase.as_str())])];
    match creation.catalog_phase {
        HubArtifactCatalogPhase::Idle => {}
        HubArtifactCatalogPhase::Loading => children.push(text_row(hub_artifact_creation_label(HubArtifactCreationLabel::CatalogLoading, locale))),
        HubArtifactCatalogPhase::Ready => children.push(text_row(hub_artifact_creation_label(HubArtifactCreationLabel::CatalogReady, locale))),
        HubArtifactCatalogPhase::Unavailable => children.push(text_row(hub_artifact_creation_label(HubArtifactCreationLabel::CatalogUnavailable, locale))),
    }
    for kind in creation.catalog.iter().flat_map(|catalog| catalog.kinds.iter()) {
        let selected = creation.kind_id.as_deref() == Some(kind.kind_id.as_str());
        let label = match locale {
            Locale::En => kind.label.en.as_str(),
            Locale::De => kind.label.de.as_str(),
        };
        children.push(button(&format!("{HUB_ARTIFACT_CREATION_ID}.kind.{}", kind.kind_id), if selected { IconName::Check } else { IconName::CircleDot }, label, descriptor(action::SELECT_ARTIFACT_KIND, one_arg("kindId", &kind.kind_id)), !busy));
    }
    children.push(input(HUB_ARTIFACT_NAME_INPUT_ID, &creation.name_draft, hub_artifact_creation_label(HubArtifactCreationLabel::Name, locale), action::SET_ARTIFACT_NAME, Some(action::CREATE_ARTIFACT)));
    children.push(button(
        &format!("{HUB_ARTIFACT_CREATION_ID}.create"),
        IconName::Plus,
        hub_artifact_creation_label(HubArtifactCreationLabel::Create, locale),
        descriptor(action::CREATE_ARTIFACT, None),
        hub_artifact_creation_intent(state, "11111111111111111111111111111111").is_some(),
    ));
    if let Some(operation) = &creation.operation {
        let phase = hub_artifact_creation_phase_str(operation.phase);
        children.push(tagged_row(hub_artifact_creation_label(HubArtifactCreationLabel::Heading, locale), &[("data-semio-hub-artifact-creation", operation.intent.request_id.as_str()), ("data-semio-hub-artifact-creation-phase", phase)]));
        children.push(text_row(hub_artifact_creation_phase_text(operation.phase, locale)));
        if !hub_artifact_creation_terminal(operation.phase) {
            if operation.cancel_requested {
                children.push(text_row(hub_artifact_creation_label(HubArtifactCreationLabel::Cancelling, locale)));
            } else {
                children.push(button(&format!("{HUB_ARTIFACT_CREATION_ID}.cancel"), IconName::X, hub_artifact_creation_label(HubArtifactCreationLabel::Cancel, locale), descriptor(action::CANCEL_ARTIFACT_CREATION, None), true));
            }
        }
        match (operation.ready.as_ref(), operation.opening) {
            (Some(_), HubArtifactOpening::Opening) => children.push(text_row(hub_artifact_creation_label(HubArtifactCreationLabel::Opening, locale))),
            (Some(ready), HubArtifactOpening::Failed) => {
                children.push(text_row(hub_artifact_creation_label(HubArtifactCreationLabel::OpeningFailed, locale)));
                children.push(button(&format!("{HUB_ARTIFACT_CREATION_ID}.open"), IconName::ArrowRight, hub_artifact_creation_label(HubArtifactCreationLabel::OpenRetry, locale), descriptor(action::OPEN_CREATED_ARTIFACT, None), true));
                children.push(tagged_row(&ready.artifact_id, &[("data-semio-hub-artifact-created", ready.artifact_id.as_str())]));
            }
            (Some(ready), _) => children.push(tagged_row(&ready.artifact_id, &[("data-semio-hub-artifact-created", ready.artifact_id.as_str())])),
            (None, _) => {}
        }
    }
    stack(HUB_ARTIFACT_CREATION_ID, children)
}

/// 🏛️ The whole hub workspace as one retained document''')
swap('''    if state.open_space_id.is_some() {
        children.push(hub_members_section(state, locale));
    }''', '''    if state.open_space_id.is_some() {
        children.push(hub_members_section(state, locale));
        children.push(hub_artifact_creation_section(state, locale));
    }''')
p.write_text(t)
print("hub creation UI written")
