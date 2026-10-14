//! 🧰️ Process3d editor kit: identifiers, the dispatch context, ui-value plumbing, the io ports and the installed machine
//! catalogs every command, mode and panel builds on.

extern crate semio_framework_os_kernel as store;

//#region 🧩️Modules
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod process3d {
        #[path = "../🗣️terminology/🦀️.rs"]
        pub mod terminology;
    }
}
//#endregion 🧩️Modules

use semio_s_artifact_process_process3d_model::{snapshot::MachineCatalog, snapshot::MachineCatalogs, snapshot::Process3dSnapshot, snapshot::WorkshopMachine};
use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation};
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText as _;
use semio_framework::kernel::Effect;
use semio_framework_plugin::{ActionDefinition, ActionKind, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MediaClass, MediaForm, MediaType, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
use semio_framework_ui_locale::{Label, LocalizedLabel};
use pack::snapshot::ArtifactPack;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_APP_ID: &str = "process3d-play";
pub const PROCESS_3D_PLAY_CONTROLLER_ID: &str = "process3d-play";
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): the app's sole interaction domain — the
/// stock, every process step, and every installed workshop machine share the "object" granularity
/// (as their raw ids / `"machine:{id}"`), the processed mesh's picked faces use "face" (u32 ids
/// stringified at the `InteractionTarget` boundary). Flat hierarchy: no cross-object parent/child
/// structure to declare a topology for.
pub const PROCESS3D_INTERACTION_DOMAIN: &str = "geometry";
/// 🎯️ The granularity every stock / step / workshop-machine row picks at — the domain's default and
/// the one the workpiece canvas reports for an instance hit, so a tree row and a canvas click land on
/// the same target.
pub const PROCESS3D_GRANULARITY_OBJECT: &str = "object";
/// 🎯️ …and the mesh-face granularity only the canvas produces (u32 face ids, stringified at the
/// `InteractionTarget` boundary). No tree row is a face.
pub const PROCESS3D_GRANULARITY_FACE: &str = "face";
/// 🕹️ Owned snapshot of `InteractionView::selection(PROCESS3D_INTERACTION_DOMAIN).ids`, read once per
/// dispatch by `ArtifactEditor::handle` and threaded through `Process3dDispatchCtx` to the one command
/// handler that needs it (`remove_selected_step`) — mirrors `📐️cad`'s own `CadInteractionSnapshot`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Process3dInteractionSnapshot {
    pub ids: Vec<String>,
}
/// 🕹️ `app_commands!`'s `ctx` payload — every command handler now takes this fourth parameter (most
/// ignore it via `_ctx`), see `🔖️KeyedAndContextualForms` in the SDK's `app_commands!` doc comment.
pub struct Process3dDispatchCtx {
    pub interaction: Process3dInteractionSnapshot,
    pub view_state: Option<semio_framework_plugin::ViewModel>,
}
impl Process3dDispatchCtx {
    pub fn view_state(&self) -> Result<&semio_framework_plugin::ViewModel, Fault> {
        self.view_state.as_ref().ok_or_else(|| Fault::from("process3d command requires host view context"))
    }

    pub fn active_utility(&self) -> Result<&str, Fault> {
        let utility = self.view_state()?.active_utility_id.as_deref().unwrap_or_default();
        (!utility.is_empty()).then_some(utility).ok_or_else(|| Fault::from("process3d command requires host active utility context"))
    }
}
/// 🕹️ The `"geometry"` domain declaration: object granularity (stock/step/machine ids, the domain
/// default) plus face granularity (u32 mesh face ids, stringified at the `InteractionTarget`
/// boundary). Flat hierarchy — no cross-object parent/child structure.
pub fn process3d_interaction_definition() -> InteractionDefinition {
    InteractionDefinition {
        id: PROCESS3D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Geometry", "Geometrie"),
        granularities: vec![
            GranularityDefinition { id: PROCESS3D_GRANULARITY_OBJECT.into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() },
            GranularityDefinition { id: PROCESS3D_GRANULARITY_FACE.into(), label: LocalizedLabel::native("Face", "Fläche"), icon_id: "square".into() },
        ],
        hierarchy: HierarchyProvider::Flat,
        hover: HoverSpec::default(),
        selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
    }
}
pub const PROCESS3D_EXAMPLE_TIMBER: &str = "timber-beam-joinery";
pub const PROCESS3D_EXAMPLE_PLATE: &str = "drilled-plate";
/// 🌲️ Shares its id with `crate::examples::concrete_forest::ID`, the registered example the react
/// shell's picker dispatches back through `setActiveExample`.
pub const PROCESS3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`☑️options/*`, `📌️panels/*`, `🎮️commands/*`) builds its `on_change`/item actions with.
pub fn process3d_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(PROCESS_3D_PLAY_CONTROLLER_ID).action(action, args)
}
/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}
/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}
/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}
/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}
/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}
/// 🏷️ Admits resolved app text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI label admission failed"))
}
/// 📇️ A non-palette action declaration (dispatched by UI wiring/keybindings, never surfaced in the
/// command palette) with the given execution kind.
pub fn internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🎨️ `tree_item_with_action` (SDK)? carries no icon slot, so this app-wide wrapper layers `icon_id` on
/// top via struct-update syntax — shared by the `🛍️catalogue` and `🛠️workshop` panels.
pub fn iconed_tree_item_with_action(
    id: impl AsRef<str>,
    label: impl AsRef<str>,
    icon_id: &str,
    action: semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut node = semio_framework_plugin::tree_item_with_action(id, ui_label(label)?, None, action?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.icon", "fixed tree-item icon admission failed"))?);
    }
    Ok(node)
}
/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "replace the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) every wholesale document-swap
/// command (`🎮️commands/🗿️artifact`, `🎮️commands/🪵️stock`, `🎮️commands/📤️media`, `import_media`'s
/// `geometry:in`) uses instead of the banned whole-snapshot mutation. The spr is a fresh, edit-free
/// op-log (`store::os_store::text_format::empty_document_spr`, the `🏗️fem` shape) — never a live `ArtifactEnvelope` minted
/// just to print it: such an envelope owns a bounded retirement authority, and dropping it at the end
/// of this function trapped the guest (`artifact envelope terminal shell reached Drop before its
/// app-owned bounded retirement authority detached every nested owner`) the moment the react shell
/// dispatched its boot `setActiveExample`.
pub fn reset_process3d_document_effect(document: &Process3dSnapshot) -> Effect {
    let pack = <Process3dSnapshot as ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("process3d", semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA));
    Effect::LoadDocument { pack, spr }
}
/// 🚨 Typed host-action decoding fault with one stable app-specific code.
pub fn process3d_action_fault(action: &str, detail: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("process3d.action.invalid"), format!("action '{action}': {}", detail.into()))
}
//#endregion 🔖️Constants
//#region 🔧️Behavior
/// 🧭️ Axis-angle rotation that maps world-up `[0,0,1]` onto an arbitrary unit `normal`, so a box
/// primitive's local Z axis (its `height` dimension) ends up flush with a picked face's normal. Pure
/// math with no snapshot/io coupling — only `🎮️commands/🌍️world`'s face-drag placement calls it, so
/// it lives here rather than `🧬️schema/💡️inferences`.
pub fn axis_angle_from_up_to(normal: [f64; 3]) -> ([f64; 3], f64) {
    const UP: [f64; 3] = [0.0, 0.0, 1.0];
    let dot = (UP[0] * normal[0] + UP[1] * normal[1] + UP[2] * normal[2]).clamp(-1.0, 1.0);
    if dot > 1.0 - 1e-9 {
        return ([0.0, 0.0, 1.0], 0.0);
    }
    if dot < -1.0 + 1e-9 {
        return ([1.0, 0.0, 0.0], std::f64::consts::PI);
    }
    let cross = [UP[1] * normal[2] - UP[2] * normal[1], UP[2] * normal[0] - UP[0] * normal[2], UP[0] * normal[1] - UP[1] * normal[0]];
    let len = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
    let axis = if len > 1e-9 { [cross[0] / len, cross[1] / len, cross[2] / len] } else { [0.0, 0.0, 1.0] };
    (axis, dot.acos())
}


//#endregion 🔧️Behavior
//#region 🔧️Behavior
//#endregion 🔧️Behavior
//#region 🔧️Behavior
const PROCESS_CONTRIBUTION_MAX_BYTES: usize = 256 * 1024;
const PROCESS_CONTRIBUTION_MAX_DEPTH: usize = 32;
const PROCESS_CONTRIBUTION_MAX_ITEMS: usize = 4 * 1024;
/// 📏️ Longest JSON string one contribution payload may carry. The REAL `machinesJson` of the four
/// shipped `process-extension-*` bundles is 3 311 / 4 010 / 4 011 / 4 705 bytes (measured 2026-09-16,
/// ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS), so the old 4 KiB bound silently refused the
/// wood catalog — `contributed_machine_catalogs` returned nothing and the workshop showed built-ins
/// only, whatever the host pushed.
const PROCESS_CONTRIBUTION_MAX_STRING_BYTES: usize = 16 * 1024;
fn process_json_envelope_is_bounded(input: &str) -> bool {
    if input.len() > PROCESS_CONTRIBUTION_MAX_BYTES {
        return false;
    }
    let mut depth = 0usize;
    let mut items = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut string_bytes = 0usize;
    let mut in_scalar = false;
    for byte in input.bytes() {
        if in_string {
            if escaped {
                string_bytes = string_bytes.saturating_add(1);
                escaped = false;
            } else if byte == b'\\' {
                string_bytes = string_bytes.saturating_add(1);
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            } else {
                string_bytes = string_bytes.saturating_add(1);
            }
            if string_bytes > PROCESS_CONTRIBUTION_MAX_STRING_BYTES {
                return false;
            }
            continue;
        }
        if in_scalar {
            if byte.is_ascii_whitespace() || matches!(byte, b',' | b']' | b'}') {
                in_scalar = false;
            } else {
                continue;
            }
        }
        match byte {
            b'"' => {
                items = items.saturating_add(1);
                in_string = true;
                string_bytes = 0;
            }
            b'{' | b'[' => {
                items = items.saturating_add(1);
                depth = depth.saturating_add(1);
                if depth > PROCESS_CONTRIBUTION_MAX_DEPTH {
                    return false;
                }
            }
            b'}' | b']' => {
                let Some(next) = depth.checked_sub(1) else {
                    return false;
                };
                depth = next;
            }
            b':' | b',' => {}
            byte if byte.is_ascii_whitespace() => {}
            _ => {
                items = items.saturating_add(1);
                in_scalar = true;
            }
        }
        if items > PROCESS_CONTRIBUTION_MAX_ITEMS {
            return false;
        }
    }
    !in_string && !escaped && depth == 0
}
// 🚫️async: E1 pure — bounded JSON decode into operation-owned values, zero suspension points.
// `TopicContribution::decode` itself is still `fn` in
// `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (out of this packet's path_scope); bridged via
// `semio_framework_os_kernel::io::resolve_ready` (see `imperative_extension_sdk`'s identical bridge and this
// packet's lease-request for the SDK owner to revert `decode` to sync directly).
/// 🧩️ Distils a host pack down to what THIS app can install: the `process.machines` entries
/// addressed to it, in host order, while the re-encoded roster still fits `maximum_bytes`.
///
/// ⚖️ The app retains the roster it can act on, never the host's whole pack. A pack is cut from the
/// receiver's whole `consumes` closure — for the demonstrator that is `cad.computer` and
/// `sourcing.module` as well — and a retained config lane is a fixed envelope; distilling at the
/// app boundary is what makes the two compatible (the `sourcing.module` lane's own precedent).
pub fn installable_contributions(contributions_json: &str, maximum_bytes: usize) -> String {
    let empty = "[]".to_string();
    if !process_json_envelope_is_bounded(contributions_json) {
        return empty;
    }
    let mut kept: Vec<semio_framework::ProgramContributionEntry> = Vec::new();
    for entry in semio_framework::parse_contributions(contributions_json) {
        let addressed = entry
            .topic_contribution
            .as_ref()
            .filter(|topic| topic.topic == "process.machines")
            .and_then(|topic| topic.decode::<ProcessMachinesTopicPayload>().ok())
            .is_some_and(|payload| payload.app_id == PROCESS_3D_PLAY_APP_ID);
        if !addressed {
            continue;
        }
        kept.push(entry);
        if semio_framework_pack_json::to_json_string(&kept).len() > maximum_bytes {
            kept.pop();
        }
    }
    if kept.is_empty() { empty } else { semio_framework_pack_json::to_json_string(&kept) }
}
fn contributed_machine_catalogs(contributions_json: &str) -> Vec<ContributedMachineCatalog> {
    if !process_json_envelope_is_bounded(contributions_json) {
        return Vec::new();
    }
    let mut catalogs = Vec::new();
    for entry in semio_framework::parse_contributions(contributions_json) {
        let Some(payload) = entry.topic_contribution.as_ref().filter(|topic| topic.topic == "process.machines").and_then(|topic| topic.decode::<ProcessMachinesTopicPayload>().ok()) else {
            continue;
        };
        let (app_id, module_id, label, icon_id, machines_json) = (payload.app_id, payload.module_id, payload.label, payload.icon_id, payload.machines_json);
        if app_id != PROCESS_3D_PLAY_APP_ID {
            continue;
        }
        if !process_json_envelope_is_bounded(&machines_json) {
            continue;
        }
        let Ok(machines) = semio_framework_pack_json::from_json_str::<Vec<WorkshopMachine>>(&machines_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            continue;
        };
        if machines.len() > PROCESS_CONTRIBUTION_MAX_ITEMS {
            continue;
        }
        catalogs.push(ContributedMachineCatalog { catalog_id: module_id, label, icon_id, machines });
    }
    catalogs
}
pub fn builtin_installed_catalogs() -> Vec<MachineCatalogs> {
    vec![semio_s_artifact_process_process3d_model::GenericCatalog.into(), semio_s_artifact_process_process3d_model::wood_catalog().into(), semio_s_artifact_process_process3d_model::concrete_catalog().into(), semio_s_artifact_process_process3d_model::metal_catalog().into(), semio_s_artifact_process_process3d_model::robotic_catalog().into()]
}
/// 🧩️ Every machine catalog installed in this build, in stable display order — the built-in generic
/// catalog first (so it renders as the default-open section), then every `process.machines` contribution
/// parsed from the invoking configuration's runtime-installable extensions under
/// `🏭️process/🧩️extensions/`.
///
/// ⚖️ A `catalog_id` names exactly one catalog here. The four shipped `process-extension-*` packs
/// contribute the SAME ids this crate already compiles in (`🧩️extensions/🪵️wood/🦀️.rs` sends
/// `moduleId = catalog.catalog_id() = "wood"`), so a bare concatenation renders Wood/Metal/Concrete/
/// Robotic twice. The installed build wins and the contribution is dropped; only a genuinely new id
/// appends, in host order — the same app-boundary law `sourcing`'s `schema::installable_contributions`
/// applies to `sourcing.module`.
pub fn installed_catalogs(contributions_json: &str) -> Vec<MachineCatalogs> {
    let mut catalogs = builtin_installed_catalogs();
    for contributed in contributed_machine_catalogs(contributions_json) {
        if catalogs.iter().any(|catalog| catalog.catalog_id() == contributed.catalog_id) {
            continue;
        }
        catalogs.push(MachineCatalogs::from(contributed));
    }
    catalogs
}
/// 🔎️ One machine, by catalog + machine id, with `catalog_id` stamped onto the snapshot — the
/// "install into workshop" lookup for the workshop configurator's add-machine action.
pub fn catalog_machine(contributions_json: &str, catalog_id: &str, machine_id: &str) -> Option<WorkshopMachine> {
    let catalog = installed_catalogs(contributions_json).into_iter().find(|catalog| catalog.catalog_id() == catalog_id)?;
    let mut machine = catalog.machines().into_iter().find(|machine| machine.id == machine_id)?;
    machine.catalog_id = Some(catalog_id.to_string());
    Some(machine)
}
//#endregion 🔧️Behavior

//#region 🔖️Process3dPlayApp
//#region 🧵️RetainedCommands
/// 📏️ The retained `contributions_json` lane, sized from the REAL pack: the four shipped
/// `process-extension-*` bundles are 19 823 bytes of `process.machines` entries once the host has
/// scoped them by this receiver's `consumes` row (measured 2026-09-16, ticket
/// 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS). `installable_contributions` distils the host pack to
/// exactly those entries before anything is retained, so the app never stores the whole closure.
pub const PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES: usize = 24_576;
//#endregion 🧵️RetainedCommands
//#endregion 🔖️Process3dPlayApp
