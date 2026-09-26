//! 🎛️ Norm plugin — the app-surface machinery every one of the fifteen compliance apps shares.
//!
//! 📌️ The fifteen norm apps are structurally identical by construction (one `edit` mode, an
//! inputs/results window pair, the framework document/catalogue/inspection panel trio, the same
//! `model:in`/`report:out` media ports, the same three commands) and differ only in their per-standard
//! `Document` type, ids and labels. Everything that does NOT vary lives here, ONCE; every taxonomy node
//! under each subset's `✏️editor/`/`👁️viewer/` states only what genuinely varies and calls into this module. That is the
//! "shared declarations belong at the shallowest common ancestor" rule taken to its conclusion — the
//! shallowest common ancestor of fifteen sibling apps is the plugin's own `🫀️core`.
//!
//! Nothing here depends on any app or artifact module: every entry point is either a plain constructor
//! or generic over the artifact's `Document`/`NormFamily`, so `🫀️core` stays a leaf of the dependency
//! graph exactly as the artifacts require.

/// 🧵️ Retained norm command output with empty application config and draft lanes.
pub type NormRetainedCommandResult<M> = Result<Emit<M, semio_framework_plugin::NoConfigMutation, semio_framework_plugin::NoDraftMutation>, Fault>;

use crate::document::{
    cached_report_for, document_revision_key, invalidate_cached_report_for, store_cached_report_for, CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, NormFamily, NormHost, Quantity,
    QuantityKind, Remedy, RemedyBound,
};
use semio_framework::ToolExecutionContract;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind};
use semio_framework_ui_contract as ui;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::{
    tree_group, tree_item_desc, tree_item_with_action, tree_window_section_or_placeholder, ui_node_list, ActionFactory, AppIo, ArtifactKindSpec, ArtifactPresentation, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, BuiltNode, ConfigView, Emit, ExampleSource, Fault, FaultCode, FaultOrigin, Locale,
    LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaPortDirection, MediaPortSpec, MediaType, ModeDefinition, NoConfig, NoConfigMutation, OsMediaCapability, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder,
    PluginAssemblyError, PortMultiplicity, SurfaceKind, Terminology, TreeWindows, Trigger, UiAssemblyResult, UiFixedList, UiMapBuilder, UiText, UiValue, WindowConfigOwner, WindowKindDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode,
    WindowLayoutWindowNode, WindowOptions,
};

/// 🧹️ Installs the exact bounded store owners and disposers shared by every Norm editor.
#[macro_export]
macro_rules! norm_exact_store_ownership {
    () => {
        fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
            Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
            Some(semio_framework_plugin::no_config_store_owners())
        }

        fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
            Some(semio_framework_plugin::no_draft_store_owners())
        }

        fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(semio_framework_plugin::no_config_store_disposer())
        }

        fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
            Some(semio_framework_plugin::no_draft_store_disposer())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_framework_plugin::no_presence_store_disposer())
        }

        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin::no_presence_peer_retirement_factory())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_framework_plugin::no_transient_store_disposer())
        }

        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
        }
    };
}

/// 🧹️ The read-only twin of [`norm_exact_store_ownership`]. `ArtifactViewer` defaults every store
/// owner and disposer to `None` ("absent authority fails closed"), so a viewer that omits them
/// cannot be closed — `close_registered_fixture_app` refuses it with
/// `interactive-job.close-owned-disposer-missing`. The viewer trait carries no draft lane and no
/// local-root retirement factories, so this is the editor macro's method set minus those.
#[macro_export]
macro_rules! norm_exact_viewer_store_ownership {
    () => {
        fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
            Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
            Some(semio_framework_plugin::no_config_store_owners())
        }

        fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(semio_framework_plugin::no_config_store_disposer())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_framework_plugin::no_presence_store_disposer())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_framework_plugin::no_transient_store_disposer())
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin::no_presence_peer_retirement_factory())
        }
    };
}

//#region 🔖️Ids
/// 🆔️ The single mode every norm app's editor declares.
pub const MODE_EDIT: &str = "edit";
/// 🆔️ The single mode every norm app's viewer declares (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET).
pub const MODE_VIEW: &str = "view";
/// 🆔️ The report tree every norm results window renders, and its one windowed check section.
pub const NORM_REPORT_TREE_ID: &str = "norm-report";
pub const NORM_REPORT_SECTION_ID: &str = "norm-report.checks";
//#endregion 🔖️Ids

/// 📎️ Norm applications own no config or presence facets; Results-window config is registered separately.
pub fn app_schema_descriptor() -> schema::AppSchemaDescriptor {
    schema::AppSchemaDescriptor {
        id: "s.norm.norm",
        config: schema::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
        presence: schema::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
    }
}

//#region 🔖️ViewerManifest
/// ✏️ The `view` mode definition — identical for all fifteen viewers, the read-only counterpart of
/// `edit_mode_definition`.
pub fn view_mode_definition() -> ModeDefinition {
    ModeDefinition { id: MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane window layout — every norm viewer has exactly one window (the compliance
/// report table), so there is no quadrant/split layout to allocate.
pub fn single_window_layout(window_kind_id: &str, title: &str) -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}

/// 📊️ `TableWindowKit` column headers for a norm `CheckReport` — shared by all fifteen viewers.
pub fn report_table_columns(locale: Locale) -> Vec<String> {
    let terminology = Terminology::Native;
    vec![
        LocalizedLabel::native("Part", "Teil").resolve(terminology, locale).into(),
        LocalizedLabel::native("Clause", "Klausel").resolve(terminology, locale).into(),
        LocalizedLabel::native("Subject", "Gegenstand").resolve(terminology, locale).into(),
        LocalizedLabel::native("Status", "Status").resolve(terminology, locale).into(),
        LocalizedLabel::native("Utilization", "Ausnutzung").resolve(terminology, locale).into(),
        LocalizedLabel::native("Title", "Titel").resolve(terminology, locale).into(),
        LocalizedLabel::native("Remedy", "Abhilfe").resolve(terminology, locale).into(),
    ]
}

/// 📊️ `TableWindowKit` rows for a norm `CheckReport` — columns matching [`report_table_columns`].
pub fn report_table_rows(report: &CheckReport, locale: Locale) -> Vec<Vec<String>> {
    let protocol_locale = protocol_locale(locale);
    report
        .checks
        .iter()
        .map(|check| {
            let remedy = check
                .remedies
                .first()
                .map(|remedy| remedy.action.resolve(&protocol_locale).to_string())
                .unwrap_or_default();
            vec![
                check.part.clone(),
                check.clause.to_string(),
                check.subject.label.resolve(&protocol_locale).to_string(),
                status_label(check.status, locale),
                format!("{:.2}", check.utilization),
                check.title.resolve(&protocol_locale).to_string(),
                remedy,
            ]
        })
        .collect()
}
//#endregion 🔖️ViewerManifest

//#region 🔖️LocaleChrome
/// 🗣️ Maps the shell [`Locale`] onto the protocol axis [`LocalizedCopy`] resolves against.
pub fn protocol_locale(locale: Locale) -> protocol::Locale {
    match locale {
        Locale::En => protocol::Locale::En,
        Locale::De => protocol::Locale::De,
    }
}

/// 🏷️ Localized chrome string from a fixed en/de pair.
fn chrome(en: &str, de: &str, locale: Locale) -> String {
    LocalizedLabel::native(en, de).resolve(Terminology::Native, locale).to_string()
}

/// 🏷️ `MediaPortSpec.label` is a plain `String` — pack both native terms from a [`LocalizedLabel`].
fn media_port_label(label: LocalizedLabel) -> String {
    format!(
        "{} / {}",
        label.resolve(Terminology::Native, Locale::En),
        label.resolve(Terminology::Native, Locale::De),
    )
}

/// ✅️ Localized status chip text.
pub fn status_label(status: CheckStatus, locale: Locale) -> String {
    match status {
        CheckStatus::Pass => chrome("Pass", "Bestanden", locale),
        CheckStatus::Warning => chrome("Warning", "Warnung", locale),
        CheckStatus::Fail => chrome("Fail", "Nicht bestanden", locale),
        CheckStatus::NotApplicable => chrome("Not applicable", "Nicht anwendbar", locale),
    }
}

/// 📐️ Formats a SI [`Quantity`] with a natural display unit for the kind.
pub fn format_quantity(quantity: &Quantity) -> String {
    match quantity.kind {
        QuantityKind::Force => format!("{:.3} kN", quantity.value / 1_000.0),
        QuantityKind::Stress | QuantityKind::Pressure => format!("{:.3} MPa", quantity.value / 1_000_000.0),
        QuantityKind::Length => {
            if quantity.value.abs() > 0.0 && quantity.value.abs() < 1.0 {
                format!("{:.1} mm", quantity.value * 1_000.0)
            } else {
                format!("{:.3} m", quantity.value)
            }
        }
        QuantityKind::Area => format!("{:.3} m²", quantity.value),
        QuantityKind::Volume => format!("{:.3} m³", quantity.value),
        QuantityKind::HeatTransferCoefficient => format!("{:.3} W/(m²K)", quantity.value),
        QuantityKind::ThermalResistance => format!("{:.3} m²K/W", quantity.value),
        QuantityKind::ThermalConductivity => format!("{:.3} W/(m·K)", quantity.value),
        QuantityKind::Energy => format!("{:.3} kWh/(m²a)", quantity.value / 3_600_000.0),
        QuantityKind::Power => format!("{:.3} W", quantity.value),
        QuantityKind::Moment => format!("{:.3} kN·m", quantity.value / 1_000.0),
        QuantityKind::Mass => format!("{:.3} kg", quantity.value),
        QuantityKind::Time => format!("{:.3} s", quantity.value),
        QuantityKind::Temperature => format!("{:.2} K", quantity.value),
        QuantityKind::Acceleration => format!("{:.3} m/s²", quantity.value),
        QuantityKind::AirPermeability => format!("{:.3} m³/(m²h)", quantity.value),
        QuantityKind::VentilationRate => format!("{:.3} 1/h", quantity.value),
        QuantityKind::Dimensionless => format!("{:.3}", quantity.value),
    }
}

/// 🆔️ Editor controller id for a norm variant (`s.norm.<variant>@1/*#editor`).
pub fn editor_controller_id(variant: &str) -> String {
    format!("s.norm.{variant}@1/*#editor")
}
//#endregion 🔖️LocaleChrome

//#region 🔖️ValuePath
/// 🧭 One segment of a camelCase snapshot path (`field`, `[index]`, or `[id=…]`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathSegment {
    Field(String),
    Index(usize),
    Id(String),
}

/// 🧭 Validates a list-element id for `[id=…]` selectors (must not contain `]`, `.`, or `=`).
pub fn validate_path_element_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("list element id must not be empty".into());
    }
    if id.contains([']', '.', '=']) {
        return Err(format!("list element id '{id}' must not contain ']', '.', or '='"));
    }
    Ok(())
}

/// 🧭 Parses `elements[2].layers[id=L1].thicknessM` into segments (`[index]` or `[id=…]`).
pub fn parse_path(path: &str) -> Result<Vec<PathSegment>, String> {
    let mut segments = Vec::new();
    let mut rest = path.trim();
    if rest.is_empty() {
        return Err("path must not be empty".into());
    }
    while !rest.is_empty() {
        if rest.starts_with('.') {
            rest = &rest[1..];
        }
        if rest.starts_with('[') {
            let close = rest.find(']').ok_or_else(|| format!("unclosed selector in path '{path}'"))?;
            let inner = &rest[1..close];
            if let Some(id) = inner.strip_prefix("id=") {
                validate_path_element_id(id).map_err(|error| format!("{error} in path '{path}'"))?;
                segments.push(PathSegment::Id(id.to_string()));
            } else if inner.chars().all(|c| c.is_ascii_digit()) && !inner.is_empty() {
                let index: usize = inner.parse().map_err(|_| format!("invalid index in path '{path}'"))?;
                segments.push(PathSegment::Index(index));
            } else {
                return Err(format!("malformed list selector '[{inner}]' in path '{path}' (expected [index] or [id=…])"));
            }
            rest = &rest[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        let field = &rest[..end];
        if field.is_empty() {
            return Err(format!("empty field in path '{path}'"));
        }
        segments.push(PathSegment::Field(field.to_string()));
        rest = &rest[end..];
    }
    Ok(segments)
}

fn object_id_field(value: &dsl::DslValue) -> Option<&str> {
    let dsl::DslValue::Object(entries) = value else {
        return None;
    };
    entries.iter().find(|(key, _)| key == "id").and_then(|(_, value)| match value {
        dsl::DslValue::String(raw) => Some(raw.as_str()),
        _ => None,
    })
}

/// 🧭 Formats a list-element path segment: `[id=…]` when the element has a string `id`, else `[index]`.
pub fn list_element_selector(item: &dsl::DslValue, index: usize) -> String {
    match object_id_field(item).and_then(|id| validate_path_element_id(id).ok().map(|_| id)) {
        Some(id) => format!("[id={id}]"),
        None => format!("[{index}]"),
    }
}

/// 🧭 Joins an array path with a list-element selector.
pub fn list_element_path(array_path: &str, item: &dsl::DslValue, index: usize) -> String {
    format!("{array_path}{}", list_element_selector(item, index))
}

fn resolve_list_index(items: &[dsl::DslValue], segment: &PathSegment, path_hint: &str) -> Result<usize, String> {
    match segment {
        PathSegment::Index(index) => {
            if *index >= items.len() {
                return Err(format!("index {index} out of range in path '{path_hint}'"));
            }
            Ok(*index)
        }
        PathSegment::Id(id) => {
            let mut found = None;
            for (index, item) in items.iter().enumerate() {
                if object_id_field(item) == Some(id.as_str()) {
                    if found.is_some() {
                        return Err(format!("duplicate list element id '{id}' in path '{path_hint}'"));
                    }
                    found = Some(index);
                }
            }
            found.ok_or_else(|| format!("unknown list element id '{id}' in path '{path_hint}'"))
        }
        PathSegment::Field(name) => Err(format!("expected list selector before continuing past field '{name}' in path '{path_hint}'")),
    }
}

fn value_at_mut<'a>(root: &'a mut dsl::DslValue, segments: &[PathSegment], path_hint: &str) -> Result<&'a mut dsl::DslValue, String> {
    let mut cursor = root;
    for segment in segments {
        match segment {
            PathSegment::Field(name) => {
                let dsl::DslValue::Object(entries) = cursor else {
                    return Err(format!("expected object before field '{name}' in path '{path_hint}'"));
                };
                let Some(index) = entries.iter().position(|(key, _)| key == name) else {
                    return Err(format!("missing field '{name}' in path '{path_hint}'"));
                };
                cursor = &mut entries[index].1;
            }
            PathSegment::Index(_) | PathSegment::Id(_) => {
                let dsl::DslValue::Array(items) = cursor else {
                    return Err(format!("expected array before list selector in path '{path_hint}'"));
                };
                let index = resolve_list_index(items, segment, path_hint)?;
                cursor = &mut items[index];
            }
        }
    }
    Ok(cursor)
}

fn value_at<'a>(root: &'a dsl::DslValue, segments: &[PathSegment], path_hint: &str) -> Result<&'a dsl::DslValue, String> {
    let mut cursor = root;
    for segment in segments {
        match segment {
            PathSegment::Field(name) => {
                let dsl::DslValue::Object(entries) = cursor else {
                    return Err(format!("expected object before field '{name}' in path '{path_hint}'"));
                };
                let Some((_, value)) = entries.iter().find(|(key, _)| key == name) else {
                    return Err(format!("missing field '{name}' in path '{path_hint}'"));
                };
                cursor = value;
            }
            PathSegment::Index(_) | PathSegment::Id(_) => {
                let dsl::DslValue::Array(items) = cursor else {
                    return Err(format!("expected array before list selector in path '{path_hint}'"));
                };
                let index = resolve_list_index(items, segment, path_hint)?;
                cursor = &items[index];
            }
        }
    }
    Ok(cursor)
}

/// 🔎 Reads the value at `path` (supports `[index]` and `[id=…]`).
pub fn get_value_at_path<'a>(root: &'a dsl::DslValue, path: &str) -> Result<&'a dsl::DslValue, String> {
    let segments = parse_path(path)?;
    value_at(root, &segments, path)
}

/// ✏️ Sets the value at `path` inside a camelCase document value tree.
pub fn set_value_at_path(root: &mut dsl::DslValue, path: &str, value: dsl::DslValue) -> Result<(), String> {
    let segments = parse_path(path)?;
    *value_at_mut(root, &segments, path)? = value;
    Ok(())
}

/// ➕ Inserts `value` (or null) into the array at `path` at `index` (append when index ≥ len).
///
/// `path` must address the array itself (not a list element selector).
pub fn insert_value_at_path(root: &mut dsl::DslValue, path: &str, index: usize, value: Option<dsl::DslValue>) -> Result<(), String> {
    let segments = parse_path(path)?;
    if matches!(segments.last(), Some(PathSegment::Index(_) | PathSegment::Id(_))) {
        return Err(format!("insertItem path '{path}' must address an array, not a list element"));
    }
    let slot = value_at_mut(root, &segments, path)?;
    let dsl::DslValue::Array(items) = slot else {
        return Err(format!("insertItem path '{path}' must address an array"));
    };
    let insert_at = index.min(items.len());
    items.insert(insert_at, value.unwrap_or(dsl::DslValue::Null));
    Ok(())
}

/// ➖ Removes a list element.
///
/// - `path` ends with `[index]` / `[id=…]` → remove that element (the `index` arg is ignored).
/// - `path` addresses an array → remove at `index`.
pub fn remove_value_at_path(root: &mut dsl::DslValue, path: &str, index: usize) -> Result<(), String> {
    let segments = parse_path(path)?;
    if let Some(PathSegment::Index(_) | PathSegment::Id(_)) = segments.last() {
        let (parent, last) = segments.split_at(segments.len() - 1);
        let slot = if parent.is_empty() {
            root
        } else {
            value_at_mut(root, parent, path)?
        };
        let dsl::DslValue::Array(items) = slot else {
            return Err(format!("removeItem path '{path}' must address a list element"));
        };
        let remove_at = resolve_list_index(items, &last[0], path)?;
        items.remove(remove_at);
        return Ok(());
    }
    let slot = value_at_mut(root, &segments, path)?;
    let dsl::DslValue::Array(items) = slot else {
        return Err(format!("removeItem path '{path}' must address an array"));
    };
    if index >= items.len() {
        return Err(format!("removeItem index {index} out of range for '{path}'"));
    }
    items.remove(index);
    Ok(())
}

/// 🏷️ One enum/select option — wire `value` plus localized display labels families MUST supply.
#[derive(Clone, Copy, Debug)]
pub struct NormFieldChoice {
    pub value: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
}

/// 🏷️ Optional per-path field metadata families may supply for the structured inputs editor.
#[derive(Clone, Copy, Debug)]
pub struct NormFieldMeta {
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub unit: Option<&'static str>,
    pub choices: Option<&'static [NormFieldChoice]>,
}

/// 🏷️ Lookup hook consumed by [`resolve_field_meta`] (families usually wrap [`lookup_norm_field_meta`]).
pub type NormFieldMetaFn = fn(&str) -> Option<NormFieldMeta>;

/// 🏷️ Empty metadata table hook — wire `Some(empty_field_meta)` until a family fills its table.
pub fn empty_field_meta(_path: &str) -> Option<NormFieldMeta> {
    None
}

/// 🧩 Shared adapter for family metadata tables.
///
/// # Table format
///
/// Each entry is `(key, NormFieldMeta)` where `key` is a camelCase value-tree path template:
/// - Exact keys: `annex`, `site.seismicZone`
/// - List wildcards: replace every `[index]` with `[]`, e.g. `buildings[].storeys[].massKg`
/// - Nested list keys may nest wildcards: `footings[].loadCases[].verticalPermanent`
///
/// `NormFieldMeta.choices` is a slice of [`NormFieldChoice`]: wire `value` plus `label_en`/`label_de`.
/// Families MUST supply localized choice labels — raw codes (or humanized codes) as the select label
/// alone are not acceptable.
///
/// Lookup order for a concrete path such as `buildings[0].storeys[id=S2].massKg`:
/// 1. Exact key match on the concrete path
/// 2. Exact key match on the `[]`-normalized template (`[index]` and `[id=…]` both become `[]`)
/// 3. Longest prefix at a `.` segment boundary (trying template form, then concrete), so a table
///    entry `buildings[].storeys[]` still labels `buildings[0].storeys[id=S2].massKg` when the leaf
///    key is absent
pub fn lookup_norm_field_meta(table: &[(&str, NormFieldMeta)], path: &str) -> Option<NormFieldMeta> {
    if let Some((_, meta)) = table.iter().find(|(key, _)| *key == path) {
        return Some(*meta);
    }
    let templated = index_wildcards(path);
    if let Some((_, meta)) = table.iter().find(|(key, _)| *key == templated.as_str()) {
        return Some(*meta);
    }
    longest_prefix_meta(table, &templated).or_else(|| longest_prefix_meta(table, path))
}

fn index_wildcards(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' {
            out.push_str("[]");
            for x in chars.by_ref() {
                if x == ']' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn longest_prefix_meta(table: &[(&str, NormFieldMeta)], path: &str) -> Option<NormFieldMeta> {
    let mut best: Option<(usize, NormFieldMeta)> = None;
    for (key, meta) in table {
        if path == *key || path.starts_with(&format!("{key}.")) || path.starts_with(&format!("{key}[")) {
            let rank = key.len();
            if best.as_ref().map(|(len, _)| rank > *len).unwrap_or(true) {
                best = Some((rank, *meta));
            }
        }
    }
    best.map(|(_, meta)| meta)
}

/// 🏷️ Resolves metadata for `path` via the family hook, applying `[]` normalization + longest prefix.
pub fn resolve_field_meta(path: &str, meta_fn: Option<NormFieldMetaFn>) -> Option<NormFieldMeta> {
    let lookup = meta_fn?;
    if let Some(meta) = lookup(path) {
        return Some(meta);
    }
    let templated = index_wildcards(path);
    if templated != path {
        if let Some(meta) = lookup(&templated) {
            return Some(meta);
        }
    }
    let mut prefix = templated.as_str();
    while let Some(cut) = prefix.rfind(['.', '[']) {
        prefix = &prefix[..cut];
        if prefix.is_empty() {
            break;
        }
        if let Some(meta) = lookup(prefix) {
            return Some(meta);
        }
    }
    None
}

fn field_label(path: &str, meta: Option<NormFieldMeta>, locale: Locale) -> String {
    if let Some(meta) = meta {
        return chrome(meta.label_en, meta.label_de, locale);
    }
    path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']').to_string()
}

/// 📤️ Projects `document` to its camelCase value tree, runs `edit`, decodes back, and commits `from_snapshot` as one undoable edit.
///
/// 📐️ **SI convention:** snapshot scalar quantity fields MUST store SI (m, N, Pa, …). `applyRemedy` writes
/// `Remedy.required.value` (already SI) straight into `remedy.target.path`. Family agents must not store mm/kN/MPa
/// in snapshot fields — display units are UI-only via [`format_quantity`] / [`NormFieldMeta::unit`].
pub fn commit_value_tree_edit<D, M, F>(document: &D, description: &str, edit: impl FnOnce(&mut dsl::DslValue) -> Result<(), String>, from_snapshot: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    D: Clone + dsl::ToValue + dsl::FromValue,
    F: FnOnce(&D, &D) -> Vec<M>,
{
    let mut tree = dsl::ToValue::to_value(document);
    edit(&mut tree).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-path"), error))?;
    let target = dsl::FromValue::from_value(tree).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-decode"), error.to_string()))?;
    commit_snapshot_fields(from_snapshot(document, &target), description)
}

/// ✏️ Generic `setField` — `{path, value}` over the document value tree.
pub fn handle_set_field<D, M, F>(document: &D, path: &str, value: dsl::DslValue, from_snapshot: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    D: Clone + dsl::ToValue + dsl::FromValue,
    F: FnOnce(&D, &D) -> Vec<M>,
{
    let path = path.to_string();
    commit_value_tree_edit(document, "setField", move |tree| set_value_at_path(tree, &path, value), from_snapshot)
}

/// ➕ Generic `insertItem` — `{path, index, value?}`.
pub fn handle_insert_item<D, M, F>(document: &D, path: &str, index: usize, value: Option<dsl::DslValue>, from_snapshot: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    D: Clone + dsl::ToValue + dsl::FromValue,
    F: FnOnce(&D, &D) -> Vec<M>,
{
    let path = path.to_string();
    commit_value_tree_edit(document, "insertItem", move |tree| insert_value_at_path(tree, &path, index, value), from_snapshot)
}

/// ➖ Generic `removeItem` — `{path, index}`.
pub fn handle_remove_item<D, M, F>(document: &D, path: &str, index: usize, from_snapshot: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    D: Clone + dsl::ToValue + dsl::FromValue,
    F: FnOnce(&D, &D) -> Vec<M>,
{
    let path = path.to_string();
    commit_value_tree_edit(document, "removeItem", move |tree| remove_value_at_path(tree, &path, index), from_snapshot)
}

/// 🩹 Resolves a remedy on `report` into a path + value write (numeric SI or OneOf option string).
pub fn apply_remedy_edit(report: &CheckReport, check_id: &str, remedy_index: usize, option_index: usize, tree: &mut dsl::DslValue) -> Result<(), Fault> {
    let check = report.checks.iter().find(|check| check.id == check_id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-check"), format!("check '{check_id}' not in report")))?;
    let remedy = check.remedies.get(remedy_index).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-remedy"), format!("remedy {remedy_index} missing on '{check_id}'")))?;
    if !remedy.applicable {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-not-applicable"), format!("remedy {remedy_index} on '{check_id}' is not applicable")));
    }
    let path = remedy.target.path.clone();
    if path.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-empty-path"), format!("remedy {remedy_index} on '{check_id}' has empty target path")));
    }
    let value = match remedy.bound {
        RemedyBound::OneOf => {
            let option = remedy
                .options
                .get(option_index)
                .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-option"), format!("option {option_index} missing on OneOf remedy {remedy_index} for '{check_id}'")))?;
            dsl::DslValue::String(option.clone())
        }
        _ => dsl::DslValue::float(remedy.required.value),
    };
    set_value_at_path(tree, &path, value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.value-path"), error))
}

/// 🩹 Generic `applyRemedy` — re-evaluates, finds `checkId`, writes SI `required` (or OneOf option) into `remedy.target.path`.
pub fn handle_apply_remedy<F, Map>(document: &F::Document, check_id: &str, remedy_index: usize, from_snapshot: Map) -> Result<Emit<F::Mutation, NoConfigMutation>, Fault>
where
    F: NormFamily,
    F::Document: Clone + dsl::ToValue + dsl::FromValue,
    Map: FnOnce(&F::Document, &F::Document) -> Vec<F::Mutation>,
{
    handle_apply_remedy_with_option::<F, Map>(document, check_id, remedy_index, 0, from_snapshot)
}

/// 🩹 `applyRemedy` with an explicit OneOf `option_index` (ignored for numeric bounds).
pub fn handle_apply_remedy_with_option<F, Map>(document: &F::Document, check_id: &str, remedy_index: usize, option_index: usize, from_snapshot: Map) -> Result<Emit<F::Mutation, NoConfigMutation>, Fault>
where
    F: NormFamily,
    F::Document: Clone + dsl::ToValue + dsl::FromValue,
    Map: FnOnce(&F::Document, &F::Document) -> Vec<F::Mutation>,
{
    let report = cached_report_for::<F>(document).unwrap_or_else(|| F::evaluate(document));
    commit_value_tree_edit(document, "applyRemedy", move |tree| apply_remedy_edit(&report, check_id, remedy_index, option_index, tree).map_err(|fault| format!("{:?}", fault)), from_snapshot)
}
//#endregion 🔖️ValuePath

//#region 🔖️Render
fn render_text(value: impl Into<String>) -> UiAssemblyResult<BuiltNode> {
    let label = ui::Label::try_from(value.into()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI label admission failed"))?;
    ui::text(label).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text build failed"))
}

fn norm_ui_label(value: impl Into<String>) -> UiAssemblyResult<ui::Label> {
    ui::Label::try_from(value.into()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI label admission failed"))
}

fn render_text_chunks(value: &str) -> UiAssemblyResult<BuiltNode> {
    if value.len() <= ui::UI_TEXT_MAX_BYTES {
        return render_text(value);
    }
    let mut rest = value;
    let mut children = Vec::new();
    while !rest.is_empty() {
        let mut end = rest.len().min(ui::UI_TEXT_MAX_BYTES);
        while !rest.is_char_boundary(end) {
            end -= 1;
        }
        let label = ui::Label::try_from(&rest[..end]).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text chunk admission failed"))?;
        let builder = ui::text(label).try_id(format!("norm-text-chunk-{}", children.len())).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text chunk id admission failed"))?;
        children.push(builder.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text chunk build failed"))?);
        rest = &rest[end..];
    }
    ui::column().try_children(children).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm text chunk admission failed"))?.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm text chunk build failed"))
}

fn ui_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "norm UI admission failed")
}

fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| ui_error("ui.text"))
}

fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    ui_text(value).map(UiValue::Text)
}

fn action_args_map(entries: Vec<(&str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    for (key, value) in entries {
        args.push(key.into(), value).map_err(|_| ui_error("ui.value.map.entry"))?;
    }
    Ok(UiValue::Map(args.finish()))
}

fn norm_action(controller_id: &'static str, action: &str, args: Option<UiValue>) -> UiAssemblyResult<(semio_framework_plugin::ActionId, Option<UiValue>)> {
    ActionFactory::new(controller_id).action(action, args)
}

fn bind_change<B: HasBase>(builder: B, controller_id: &'static str, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    let (action, args) = norm_action(controller_id, action, Some(args))?;
    match args {
        Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| ui_error("ui.control.binding")),
        None => builder.try_on(Trigger::Change, action).map_err(|_| ui_error("ui.control.binding")),
    }
}

fn control_row(row_id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let row = ui::tree_item(norm_ui_label(label)?).try_id(row_id).map_err(|_| ui_error("ui.node.id"))?;
    row.try_child(control).map_err(|_| ui_error("ui.node.child"))?.try_build().map_err(|_| ui_error("ui.node.build"))
}

fn check_row_label(check: &CheckResult, index: usize, locale: Locale) -> String {
    let protocol_locale = protocol_locale(locale);
    let subject = check.subject.label.resolve(&protocol_locale);
    let subject_path = if check.subject.path.is_empty() {
        subject.to_string()
    } else {
        format!("{subject} @ {}", check.subject.path)
    };
    format!(
        "{}. [{}] {} — {} — {} vs {} u={:.2} — {}",
        index + 1,
        status_label(check.status, locale),
        check.title.resolve(&protocol_locale),
        subject_path,
        format_quantity(&check.computed),
        format_quantity(&check.limit),
        check.utilization,
        check.clause
    )
}

fn report_flat_rows<'a>(report: &'a CheckReport) -> Vec<ReportListRow<'a>> {
    let mut rows = Vec::new();
    let mut check_index = 0usize;
    for (part_index, (part, checks)) in report.by_part().into_iter().enumerate() {
        let verdict = report.summary.parts.iter().find(|item| item.part == part);
        rows.push(ReportListRow::Part { part_index, part, verdict });
        for check in checks {
            rows.push(ReportListRow::Check { check_index, check });
            check_index += 1;
        }
    }
    rows
}

enum ReportListRow<'a> {
    Part { part_index: usize, part: &'a str, verdict: Option<&'a crate::document::PartVerdict> },
    Check { check_index: usize, check: &'a CheckResult },
}

fn part_verdict_label(part: &str, verdict: Option<&crate::document::PartVerdict>, locale: Locale) -> String {
    let Some(verdict) = verdict else {
        return part.to_string();
    };
    let status = if verdict.complies {
        chrome("complies", "erfüllt", locale)
    } else {
        chrome("does not comply", "nicht erfüllt", locale)
    };
    format!(
        "{part} — {status} (✓{} ⚠{} ✗{} · u={:.2})",
        verdict.pass, verdict.warning, verdict.fail, verdict.worst_utilization
    )
}

fn build_check_tree_item(check: &CheckResult, check_index: usize, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<BuiltNode> {
    let protocol_locale = protocol_locale(locale);
    let label = check_row_label(check, check_index, locale);
    let id = format!("norm-report-check-{check_index}");
    let expand = matches!(check.status, CheckStatus::Fail | CheckStatus::Warning);
    if !expand {
        return tree_item_desc(id, norm_ui_label(label)?, None);
    }
    let mut children = UiFixedList::default();
    children
        .try_push(tree_item_desc(
            format!("{id}.explanation"),
            norm_ui_label(chrome("Explanation", "Erläuterung", locale))?,
            Some(check.explanation.resolve(&protocol_locale).to_string()),
        )?)
        .map_err(|_| ui_error("ui.fixed-capacity"))?;
    for (remedy_index, remedy) in check.remedies.iter().enumerate() {
        let remedy_label = format!("{}: {}", chrome("Remedy", "Abhilfe", locale), remedy.action.resolve(&protocol_locale));
        if matches!(remedy.bound, RemedyBound::OneOf) {
            if remedy.options.is_empty() {
                children
                    .try_push(tree_item_desc(format!("{id}.remedy-{remedy_index}"), norm_ui_label(remedy_label)?, Some(chrome("No options", "Keine Optionen", locale)))?)
                    .map_err(|_| ui_error("ui.fixed-capacity"))?;
                continue;
            }
            for (option_index, option) in remedy.options.iter().enumerate() {
                let child = if let Some(controller_id) = controller_id {
                    let args = action_args_map(vec![("path", ui_value_text(&remedy.target.path)?), ("value", ui_value_text(option)?)])?;
                    tree_item_with_action(
                        format!("{id}.remedy-{remedy_index}.option-{option_index}"),
                        norm_ui_label(format!("{} — {option}", chrome("Choose", "Wählen", locale)))?,
                        Some(remedy_label.clone()),
                        norm_action(controller_id, "setField", Some(args))?,
                    )?
                } else {
                    tree_item_desc(format!("{id}.remedy-{remedy_index}.option-{option_index}"), norm_ui_label(format!("{remedy_label}: {option}"))?, None)?
                };
                children.try_push(child).map_err(|_| ui_error("ui.fixed-capacity"))?;
            }
            continue;
        }
        let child = if remedy.applicable {
            if let Some(controller_id) = controller_id {
                let args = action_args_map(vec![
                    ("checkId", ui_value_text(&check.id)?),
                    ("remedyIndex", UiValue::Number(remedy_index as f64)),
                ])?;
                tree_item_with_action(
                    format!("{id}.remedy-{remedy_index}"),
                    norm_ui_label(format!("{} — {}", chrome("Apply", "Anwenden", locale), remedy_label))?,
                    Some(format!("{} → {}", format_quantity(&remedy.current), format_quantity(&remedy.required))),
                    norm_action(controller_id, "applyRemedy", Some(args))?,
                )?
            } else {
                tree_item_desc(format!("{id}.remedy-{remedy_index}"), norm_ui_label(remedy_label)?, Some(format!("{} → {}", format_quantity(&remedy.current), format_quantity(&remedy.required))))?
            }
        } else {
            tree_item_desc(format!("{id}.remedy-{remedy_index}"), norm_ui_label(remedy_label)?, Some(chrome("Not auto-applicable", "Nicht automatisch anwendbar", locale)))?
        };
        children.try_push(child).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    tree_group(id, norm_ui_label(label)?, true, children)
}

/// 📑️ Renders a whole `CheckReport` grouped by `part`, virtualised as one flat windowed section.
pub fn render_report(report: &CheckReport, windows: &TreeWindows<'_>, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<BuiltNode> {
    let rows = report_flat_rows(report);
    PanelTreeBuilder::new(NORM_REPORT_TREE_ID)?
        .window_section_or_placeholder(
            windows,
            NORM_REPORT_SECTION_ID,
            Some(norm_ui_label(chrome("Checks", "Nachweise", locale))?),
            true,
            &rows,
            |row| match row {
                ReportListRow::Part { part_index, part, verdict } => {
                    tree_item_desc(format!("norm-report-part-{part_index}"), norm_ui_label(part_verdict_label(part, *verdict, locale))?, None)
                }
                ReportListRow::Check { check_index, check } => build_check_tree_item(check, *check_index, locale, controller_id),
            },
            norm_ui_label(chrome("No checks computed.", "Keine Nachweise berechnet.", locale))?,
        )?
        .build()
}

/// 📜 Arrays and object maps at or above this length rely on host windowing (same path for both).
pub const NORM_LIST_VIRTUALIZE_THRESHOLD: usize = 64;

/// 🪪 Stable `TreeWindows` node key for a document path (`root` when empty).
pub fn inputs_section_id(path: &str) -> String {
    if path.is_empty() {
        "norm-inputs-root".into()
    } else {
        format!("norm-inputs-{path}")
    }
}

fn collection_header(label: &str, count: usize) -> String {
    format!("{label} ({count})")
}

fn render_object_editor(
    path: &str,
    entries: &[(String, dsl::DslValue)],
    locale: Locale,
    controller_id: &'static str,
    meta_fn: Option<NormFieldMetaFn>,
    windows: &TreeWindows<'_>,
    depth: usize,
    label: String,
) -> UiAssemblyResult<BuiltNode> {
    let section_id = inputs_section_id(path);
    let default_open = path.is_empty();
    let section_label = if path.is_empty() { chrome("Document", "Dokument", locale) } else { label };
    let pairs: Vec<(&String, &dsl::DslValue)> = entries.iter().map(|(key, child)| (key, child)).collect();
    tree_window_section_or_placeholder(
        windows,
        &section_id,
        norm_ui_label(collection_header(&section_label, pairs.len()))?,
        default_open,
        &pairs,
        |(key, child)| {
            let child_path = if path.is_empty() { (*key).clone() } else { format!("{path}.{}", *key) };
            render_value_editor(&child_path, child, locale, controller_id, meta_fn, windows, depth + 1)
        },
        norm_ui_label(chrome("Empty object", "Leeres Objekt", locale))?,
    )
}

fn render_array_editor(
    path: &str,
    items: &[dsl::DslValue],
    locale: Locale,
    controller_id: &'static str,
    meta_fn: Option<NormFieldMetaFn>,
    windows: &TreeWindows<'_>,
    depth: usize,
    label: String,
) -> UiAssemblyResult<BuiltNode> {
    let section_id = inputs_section_id(path);
    let default_open = false;
    let indexed: Vec<(usize, &dsl::DslValue)> = items.iter().enumerate().collect();
    let list = tree_window_section_or_placeholder(
        windows,
        &section_id,
        norm_ui_label(collection_header(&label, indexed.len()))?,
        default_open,
        &indexed,
        |(index, child)| {
            let child_path = list_element_path(path, child, *index);
            let mut row_children = UiFixedList::default();
            row_children
                .try_push(render_value_editor(&child_path, child, locale, controller_id, meta_fn, windows, depth + 1)?)
                .map_err(|_| ui_error("ui.fixed-capacity"))?;
            let remove_args = action_args_map(vec![("path", ui_value_text(&child_path)?), ("index", UiValue::Number(0.0))])?;
            row_children
                .try_push(tree_item_with_action(
                    format!("norm-inputs.{path}.remove-{index}"),
                    norm_ui_label(chrome("Remove", "Entfernen", locale))?,
                    None,
                    norm_action(controller_id, "removeItem", Some(remove_args))?,
                )?)
                .map_err(|_| ui_error("ui.fixed-capacity"))?;
            tree_group(
                format!("norm-inputs.{path}.row-{index}"),
                norm_ui_label(list_element_selector(child, *index))?,
                false,
                row_children,
            )
        },
        norm_ui_label(chrome("Empty list", "Leere Liste", locale))?,
    )?;
    if !windows.is_open(&section_id, default_open) {
        return Ok(list);
    }
    let insert_args = action_args_map(vec![("path", ui_value_text(path)?), ("index", UiValue::Number(items.len() as f64))])?;
    let add = tree_item_with_action(
        format!("norm-inputs.{path}.add"),
        norm_ui_label(chrome("Add item", "Eintrag hinzufügen", locale))?,
        None,
        norm_action(controller_id, "insertItem", Some(insert_args))?,
    )?;
    let mut children = UiFixedList::default();
    children.try_push(list).map_err(|_| ui_error("ui.fixed-capacity"))?;
    children.try_push(add).map_err(|_| ui_error("ui.fixed-capacity"))?;
    tree_group(format!("norm-inputs.{path}.editor"), norm_ui_label(label)?, true, children)
}

fn render_value_editor(
    path: &str,
    value: &dsl::DslValue,
    locale: Locale,
    controller_id: &'static str,
    meta_fn: Option<NormFieldMetaFn>,
    windows: &TreeWindows<'_>,
    depth: usize,
) -> UiAssemblyResult<BuiltNode> {
    if depth > 24 {
        return tree_item_desc(format!("norm-inputs.{path}"), norm_ui_label(path)?, Some("…".into()));
    }
    let meta = resolve_field_meta(path, meta_fn);
    let label = field_label(path, meta, locale);
    match value {
        dsl::DslValue::Object(entries) => render_object_editor(path, entries, locale, controller_id, meta_fn, windows, depth, label),
        dsl::DslValue::Array(items) => render_array_editor(path, items, locale, controller_id, meta_fn, windows, depth, label),
        dsl::DslValue::Bool(flag) => {
            let row_id = format!("norm-inputs.{path}");
            let options: Vec<(String, String)> = vec![("true".into(), chrome("True", "Wahr", locale)), ("false".into(), chrome("False", "Falsch", locale))];
            let mut control = ui::select(ui_text(if *flag { "true" } else { "false" })?).try_id(format!("{row_id}.select")).map_err(|_| ui_error("ui.node.id"))?;
            for (option, option_label) in options {
                control = control.try_item(ui_text(option)?, norm_ui_label(option_label)?).map_err(|_| ui_error("ui.select.item"))?;
            }
            let args = action_args_map(vec![("path", ui_value_text(path)?)])?;
            control_row(&row_id, &label, bind_change(control, controller_id, "setField", args)?.try_build().map_err(|_| ui_error("ui.node.build"))?)
        }
        dsl::DslValue::Number(number) => {
            let row_id = format!("norm-inputs.{path}");
            let display = if let Some(unit) = meta.and_then(|meta| meta.unit) {
                format!("{} {unit}", number.as_f64())
            } else {
                format!("{}", number.as_f64())
            };
            let control = ui::input(InputKind::Number).value(ui_text(display)?).try_id(format!("{row_id}.input")).map_err(|_| ui_error("ui.node.id"))?;
            let args = action_args_map(vec![("path", ui_value_text(path)?)])?;
            control_row(&row_id, &label, bind_change(control, controller_id, "setField", args)?.try_build().map_err(|_| ui_error("ui.node.build"))?)
        }
        dsl::DslValue::String(raw) => {
            let row_id = format!("norm-inputs.{path}");
            if let Some(choices) = meta.and_then(|meta| meta.choices) {
                let mut control = ui::select(ui_text(raw)?).try_id(format!("{row_id}.select")).map_err(|_| ui_error("ui.node.id"))?;
                for choice in choices.iter().take(32) {
                    let choice_label = chrome(choice.label_en, choice.label_de, locale);
                    control = control.try_item(ui_text(choice.value)?, norm_ui_label(choice_label)?).map_err(|_| ui_error("ui.select.item"))?;
                }
                let args = action_args_map(vec![("path", ui_value_text(path)?)])?;
                control_row(&row_id, &label, bind_change(control, controller_id, "setField", args)?.try_build().map_err(|_| ui_error("ui.node.build"))?)
            } else {
                let control = ui::input(InputKind::Text).value(ui_text(raw)?).try_id(format!("{row_id}.input")).map_err(|_| ui_error("ui.node.id"))?;
                let args = action_args_map(vec![("path", ui_value_text(path)?)])?;
                control_row(&row_id, &label, bind_change(control, controller_id, "setField", args)?.try_build().map_err(|_| ui_error("ui.node.build"))?)
            }
        }
        dsl::DslValue::Null => tree_item_desc(format!("norm-inputs.{path}"), norm_ui_label(label)?, Some(chrome("empty", "leer", locale))),
    }
}

/// 📄️ Structured, schema-driven, localized property editor over the document's camelCase value tree.
pub fn render_document_editor<D: dsl::ToValue>(
    document: &D,
    locale: Locale,
    controller_id: &'static str,
    meta_fn: Option<NormFieldMetaFn>,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode> {
    let tree = dsl::ToValue::to_value(document);
    render_value_editor("", &tree, locale, controller_id, meta_fn, windows, 0)
}

/// 📄️ Legacy pretty JSON — kept for tests that still project raw text chunks.
pub fn render_document_json<D: dsl::ToValue>(document: &D) -> UiAssemblyResult<BuiltNode> {
    let json = pack::json::to_string_pretty(&pack::json::from_dsl_value(&dsl::ToValue::to_value(document)));
    render_text_chunks(&json)
}

/// 🧾️ Localized summary headline from the report rollup.
pub fn render_summary<F: NormFamily>(host: &NormHost<F>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = host.report();
    let summary = &report.summary;
    let verdict = if summary.complies {
        chrome("complies", "erfüllt", locale)
    } else {
        chrome("does not comply", "nicht erfüllt", locale)
    };
    let parts = summary
        .parts
        .iter()
        .map(|part| {
            let mark = if part.complies { "✓" } else { "✗" };
            format!("{mark}{}", part.part)
        })
        .collect::<Vec<_>>()
        .join(", ");
    render_text(format!(
        "{} — {} {}: {verdict}: {} {}, {} {}, {} {} · {}={:.2} · {parts}",
        F::family_id().label(),
        summary.total,
        chrome("checks", "Nachweise", locale),
        summary.fail,
        chrome("failing", "nicht bestanden", locale),
        summary.warning,
        chrome("warnings", "Warnungen", locale),
        summary.pass,
        chrome("passing", "bestanden", locale),
        chrome("worst u", "max. Ausnutzung", locale),
        summary.worst_utilization
    ))
}

/// 📚️ One column in a normative reference [`CatalogueTable`].
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogueColumn {
    pub id: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub unit: Option<&'static str>,
}

/// 📚️ Typed cell value for a catalogue table row.
#[derive(Clone, Debug, PartialEq)]
pub enum CatalogueCell {
    Text(String),
    Number { value: f64, decimals: u8 },
    Empty,
}

impl CatalogueCell {
    /// 🏷️ Localized display string for a cell (numbers use fixed decimals; empty → em dash).
    pub fn display(&self, locale: Locale) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Number { value, decimals } => format!("{value:.prec$}", prec = *decimals as usize),
            Self::Empty => chrome("—", "—", locale),
        }
    }

    /// 🔤 Text cell from a static label.
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    /// 🔢 Number cell with fixed decimal places.
    pub fn number(value: f64, decimals: u8) -> Self {
        Self::Number { value, decimals }
    }
}

/// 📚️ One data row in a normative reference table.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogueRow {
    pub id: String,
    pub cells: Vec<CatalogueCell>,
}

/// 📚️ Schema-first normative reference table shown beside examples in the catalogue panel.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogueTable {
    pub id: &'static str,
    pub title_en: &'static str,
    pub title_de: &'static str,
    pub clause: ClauseId,
    pub columns: Vec<CatalogueColumn>,
    pub rows: Vec<CatalogueRow>,
}

fn catalogue_column_header(column: &CatalogueColumn, locale: Locale) -> String {
    let label = chrome(column.label_en, column.label_de, locale);
    match column.unit {
        Some(unit) => format!("{label} [{unit}]"),
        None => label,
    }
}

fn catalogue_row_description(table: &CatalogueTable, row: &CatalogueRow, locale: Locale) -> String {
    table
        .columns
        .iter()
        .enumerate()
        .map(|(index, column)| {
            let cell = row.cells.get(index).unwrap_or(&CatalogueCell::Empty);
            format!("{}={}", catalogue_column_header(column, locale), cell.display(locale))
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

fn render_catalogue_examples(examples: &[ExampleSource], locale: Locale, controller_id: &'static str) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut items = UiFixedList::default();
    if examples.is_empty() {
        items
            .try_push(tree_item_desc("norm-catalogue.empty", norm_ui_label(chrome("Examples", "Beispiele", locale))?, Some(chrome("No examples", "Keine Beispiele", locale)))?)
            .map_err(|_| ui_error("ui.fixed-capacity"))?;
    } else {
        for example in examples {
            let args = action_args_map(vec![("exampleId", ui_value_text(example.id())?)])?;
            items
                .try_push(tree_item_with_action(
                    format!("norm-catalogue.{}", example.id()),
                    norm_ui_label(example.label().resolve(Terminology::Native, locale))?,
                    None,
                    norm_action(controller_id, "setActiveExample", Some(args))?,
                )?)
                .map_err(|_| ui_error("ui.fixed-capacity"))?;
        }
    }
    Ok(items)
}

/// 📚️ Catalogue panel: example picker plus optional normative reference tables (windowed).
///
/// ```ignore
/// render_catalogue(&examples, &tables, locale, CONTROLLER_ID, &TreeWindows::for_body(view, BODY_CATALOGUE))
/// ```
pub fn render_catalogue(
    examples: &[ExampleSource],
    tables: &[CatalogueTable],
    locale: Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode> {
    let example_items = render_catalogue_examples(examples, locale, controller_id)?;
    let mut builder = PanelTreeBuilder::new("norm-catalogue")?.section(
        "norm-catalogue.examples",
        Some(norm_ui_label(chrome("Examples", "Beispiele", locale))?),
        true,
        example_items,
    )?;
    for table in tables {
        let section_id = format!("norm-catalogue.table-{}", table.id);
        let title = chrome(table.title_en, table.title_de, locale);
        let header = format!("{title} · {} · ({})", table.clause, table.rows.len());
        let column_band = table.columns.iter().map(|column| catalogue_column_header(column, locale)).collect::<Vec<_>>().join(" | ");
        let rows: Vec<&CatalogueRow> = table.rows.iter().collect();
        builder = builder.window_section_or_placeholder(
            windows,
            &section_id,
            Some(norm_ui_label(header)?),
            false,
            &rows,
            |row| {
                tree_item_desc(
                    format!("norm-catalogue.table-{}.{}", table.id, row.id),
                    norm_ui_label(catalogue_row_description(table, row, locale))?,
                    Some(column_band.clone()),
                )
            },
            norm_ui_label(chrome("Empty table", "Leere Tabelle", locale))?,
        )?;
    }
    builder.build()
}

/// 🔍️ Full check card for the inspection panel.
pub fn render_inspection(report: &CheckReport, selected_check_index: Option<u32>, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<BuiltNode> {
    let checks = &report.checks;
    let index = selected_check_index.map(|value| value as usize).filter(|index| *index < checks.len()).unwrap_or(0);
    let Some(check) = checks.get(index) else {
        let items = ui_node_list([tree_item_desc("norm-inspection.empty", norm_ui_label(chrome("Check", "Nachweis", locale))?, Some(chrome("No checks", "Keine Nachweise", locale)))])?;
        return PanelTreeBuilder::new("norm-inspection")?.section("norm-inspection.summary", Some(norm_ui_label(chrome("Inspection", "Inspektion", locale))?), true, items)?.build();
    };
    let protocol_locale = protocol_locale(locale);
    let mut rows = Vec::new();
    rows.push(tree_item_desc("norm-inspection.check.id", norm_ui_label(chrome("Id", "Kennung", locale))?, Some(check.id.clone()))?);
    rows.push(tree_item_desc("norm-inspection.check.part", norm_ui_label(chrome("Part", "Teil", locale))?, Some(check.part.clone()))?);
    rows.push(tree_item_desc("norm-inspection.check.clause", norm_ui_label(chrome("Clause", "Klausel", locale))?, Some(check.clause.to_string()))?);
    rows.push(tree_item_desc(
        "norm-inspection.check.subject",
        norm_ui_label(chrome("Subject", "Gegenstand", locale))?,
        Some(check.subject.label.resolve(&protocol_locale).to_string()),
    )?);
    if !check.subject.path.is_empty() {
        rows.push(tree_item_desc(
            "norm-inspection.check.subject-path",
            norm_ui_label(chrome("Subject path", "Gegenstandspfad", locale))?,
            Some(check.subject.path.clone()),
        )?);
    }
    rows.push(tree_item_desc("norm-inspection.check.status", norm_ui_label(chrome("Status", "Status", locale))?, Some(status_label(check.status, locale)))?);
    rows.push(tree_item_desc("norm-inspection.check.title", norm_ui_label(chrome("Title", "Titel", locale))?, Some(check.title.resolve(&protocol_locale).to_string()))?);
    rows.push(tree_item_desc(
        "norm-inspection.check.explanation",
        norm_ui_label(chrome("Explanation", "Erläuterung", locale))?,
        Some(check.explanation.resolve(&protocol_locale).to_string()),
    )?);
    rows.push(tree_item_desc("norm-inspection.check.computed", norm_ui_label(chrome("Computed", "Berechnet", locale))?, Some(format_quantity(&check.computed)))?);
    rows.push(tree_item_desc("norm-inspection.check.limit", norm_ui_label(chrome("Limit", "Grenzwert", locale))?, Some(format_quantity(&check.limit)))?);
    rows.push(tree_item_desc("norm-inspection.check.utilization", norm_ui_label(chrome("Utilization", "Ausnutzung", locale))?, Some(format!("{:.2}", check.utilization)))?);
    rows.push(tree_item_desc("norm-inspection.check.annex", norm_ui_label(chrome("Annex", "Anhang", locale))?, Some(check.annex.label().into()))?);
    for (remedy_index, remedy) in check.remedies.iter().enumerate() {
        let detail = format!(
            "{} · {} → {} · {}",
            remedy.action.resolve(&protocol_locale),
            format_quantity(&remedy.current),
            format_quantity(&remedy.required),
            remedy.target.path
        );
        let row = if remedy.applicable {
            if let Some(controller_id) = controller_id {
                let args = action_args_map(vec![("checkId", ui_value_text(&check.id)?), ("remedyIndex", UiValue::Number(remedy_index as f64))])?;
                tree_item_with_action(
                    format!("norm-inspection.check.remedy-{remedy_index}"),
                    norm_ui_label(format!("{} {}", chrome("Apply", "Anwenden", locale), remedy_index + 1))?,
                    Some(detail),
                    norm_action(controller_id, "applyRemedy", Some(args))?,
                )?
            } else {
                tree_item_desc(format!("norm-inspection.check.remedy-{remedy_index}"), norm_ui_label(chrome("Remedy", "Abhilfe", locale))?, Some(detail))?
            }
        } else {
            tree_item_desc(format!("norm-inspection.check.remedy-{remedy_index}"), norm_ui_label(chrome("Remedy", "Abhilfe", locale))?, Some(detail))?
        };
        rows.push(row);
    }
    let items = ui_node_list(rows.into_iter().map(Ok))?;
    PanelTreeBuilder::new("norm-inspection")?
        .section("norm-inspection.check", Some(norm_ui_label(format!("{} {}", chrome("Check", "Nachweis", locale), index + 1))?), true, items)?
        .build()
}

/// ❓️ The unknown-body-key fallback every norm app's `render` ends with.
pub fn render_unknown_body(body_key: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    render_text(format!("{}: {body_key}", chrome("Unknown body", "Unbekannter Inhalt", locale)))
}
//#endregion 🔖️Render

//#region 🔖️Manifest
/// ✏️ The `edit` mode definition — identical for all fifteen apps.
pub fn edit_mode_definition() -> ModeDefinition {
    ModeDefinition { id: MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A norm window kind — both windows of every app are plain `Canvas2d` surfaces with no measures,
/// engagement, actions or utilities, so only id/label/body/icon vary.
pub fn window_definition(id: &str, label: LocalizedLabel, body_key: &str, icon_id: &str) -> WindowKindDefinition {
    WindowKindDefinition {
        id: id.into(),
        label,
        body_key: body_key.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: icon_id.into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}

/// 📌️ A norm panel tab — every one is a framework-predefined leaf id bound to this app's body key.
/// Byte-identical to the `AppBuilder::panel_tab(id, label, group, body_key)` scalar call it replaces
/// (`PanelTabSpec::leaf` builds exactly this shape).
pub fn panel_definition(id: &str, label: LocalizedLabel, group: PanelGroup, body_key: &str) -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(id.into()), label, group, body_key: Some(body_key.into()), children: Vec::new() }
}

/// 🗿️ A norm artifact kind — Data × Value document per owner-table (IO coverage lattice).
pub fn artifact_kind_spec(variant: &str, label: &str) -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: artifact_kind_id(variant),
        name: label.into(),
        source_format: format!("norm.{variant}.document"),
        component_kind: "norm".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: format!("norm.{variant}.document"),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}

/// 🆔️ `computation.norm.<variant>` — the artifact kind id `report:out` pins itself to.
pub fn artifact_kind_id(variant: &str) -> String {
    format!("computation.norm.{variant}")
}

/// 🔌️ Every norm family's typed media I/O surface — the implicit `document:in`/`document:out` pair
/// (auto-injected by `AppIo::all_ports`) plus the two extra workflow ports every norm app gets:
/// `model:in` (a generic upstream-model input — an honest pass-through, no family `Document` shape has
/// a generic "raw model" field to receive one into yet) and `report:out` (the computed `CheckReport`,
/// pinned to this family's own already-declared `computation.norm.{variant}` artifact kind via
/// `kind_id`). One function serves both the builder's `.io(...)` declaration and each app's
/// `ArtifactApp::io` override, so the two never drift apart.
pub fn norm_io(variant: &str, artifact_schema: &str) -> AppIo {
    let artifact_kind_id = artifact_kind_id(variant);
    AppIo {
        artifact_schema: artifact_schema.into(),
        artifact_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        ports: vec![
            MediaPortSpec {
                id: "model:in".into(),
                label: media_port_label(LocalizedLabel::native("Model", "Modell")),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                kind_id: None,
                required: false,
                multiplicity: PortMultiplicity::One,
            },
            MediaPortSpec {
                id: "report:out".into(),
                label: media_port_label(LocalizedLabel::native("Report", "Bericht")),
                direction: MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
                kind_id: Some(artifact_kind_id.clone()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: ArtifactPresentation { id: artifact_kind_id, name: variant.into(), dimension: "data".into(), component_kind: "norm".into() },
    }
}
//#endregion 🔖️Manifest

//#region 🔖️MediaPorts
/// 🎞️ `"report:out"` dumps the currently computed `CheckReport`, pinned to this family's declared
/// artifact kind; `"artifact:out"` replicates the SDK default (whole-document pack) since overriding
/// `export_media` shadows it entirely. Any other port is `NotImplemented`.
pub fn export_media<F>(port: &str, variant: &str, artifact_schema: &str, document: &F::Document) -> Result<Media, MediaError>
where
    F: NormFamily,
    F::Document: store::ArtifactPack,
{
    if port == "report:out" {
        let report = cached_report_for::<F>(document).unwrap_or_else(|| {
            let report = F::evaluate(document);
            store_cached_report_for::<F>(document, report.clone());
            report
        });
        let json = pack::json::to_json_string(&report);
        return Ok(Media { media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: artifact_kind_id(variant), json } });
    }
    if port != "artifact:out" {
        return Err(MediaError::NotImplemented);
    }
    let bytes = store::ArtifactPack::encode_pack(document);
    Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: artifact_schema.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
}

/// 🎞️ `"model:in"` is an honest generic pass-through: a payload that happens to decode as this family's
/// own `Document` shape becomes a bundle of targeted `change-<field>` mutations (one per persistent
/// field, via each migrated facet's `XMutation::from_snapshot`) rather than a single whole-document
/// replace mutation — the banned whole-document-replace escape hatch has no 1:1 replacement, so `wrap` now
/// decomposes the imported document into the closed semantic vocabulary instead. Bundling them into one
/// `Emit::mutations` call keeps the import atomic (one edit, one undo entry), matching the old
/// single-mutation commit's history shape. Anything that doesn't decode is accepted but inert (no norm
/// family document has a generic "raw model" field to stash a foreign shape into yet). `"artifact:in"`
/// replicates the SDK default (decodes the base64 pack).
pub fn import_media<D, M, F>(port: &str, media: &Media, wrap: F) -> Result<Emit<M, NoConfigMutation>, MediaError>
where
    D: Clone + Default + PartialEq + dsl::ToValue + dsl::FromValue + store::ArtifactPack,
    F: Fn(D) -> Vec<M>,
{
    if port == "model:in" {
        if let MediaPayload::Structured { json, .. } = &media.payload {
            if let Ok(document) = pack::json::from_json_str::<D>(json) {
                return Ok(Emit::mutations(wrap(document)));
            }
        }
        return Ok(Emit::default());
    }
    if port != "artifact:in" {
        return Err(MediaError::NotImplemented);
    }
    let MediaPayload::Structured { json, .. } = &media.payload else {
        return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
    };
    let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
    let document = <D as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
    Ok(Emit::mutations(wrap(document)))
}
//#endregion 🔖️MediaPorts

//#region 🔖️Commands
/// 📤️ Commits a typed document mutation under its manifest action description.
pub fn commit_snapshot<M>(mutation: M, description: &str) -> Result<Emit<M, NoConfigMutation>, Fault> {
    Ok(Emit::commit(vec![mutation], description))
}

/// 📤️ Commit a bundle of targeted semantic mutations as one described edit — the migrated facets'
/// replacement for `commit_snapshot`'s old single whole-document-replace commit: a `set-snapshot`
/// command payload (or a re-evaluation re-commit) decomposes into one `change-<field>` mutation per
/// persistent field via `XMutation::from_snapshot`, bundled here into a single undo entry.
pub fn commit_snapshot_fields<M>(mutations: Vec<M>, description: &str) -> Result<Emit<M, NoConfigMutation>, Fault> {
    Ok(Emit::commit(mutations, description))
}

/// 🎯️ Builds the args-side of an app's `command_from_action` bridge for `selected-check` — the shells
/// still speak `{action,args}` for chrome actions.
pub fn selected_check_index_arg(args: Option<&dsl::DslValue>) -> Option<u32> {
    args.and_then(|value| value.get("index")).and_then(dsl::DslValue::as_u64).map(|value| value as u32)
}

/// 🎯️ Path argument used by `setField` / `insertItem` / `removeItem`.
pub fn path_arg(args: Option<&dsl::DslValue>) -> String {
    args.and_then(|value| value.get("path"))
        .and_then(|value| if let dsl::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(dsl::json::to_json_string(value)) })
        .unwrap_or_default()
}

/// 🎯️ Index argument used by `insertItem` / `removeItem` / `applyRemedy.remedyIndex`.
pub fn index_arg(args: Option<&dsl::DslValue>, key: &str) -> usize {
    args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_u64).map(|value| value as usize).unwrap_or(0)
}

/// 🎯️ Optional value argument — host control merges under `value`; absent ⇒ JSON null.
pub fn value_arg_json(args: Option<&dsl::DslValue>) -> String {
    match args.and_then(|value| value.get("value")) {
        Some(value) => dsl::json::to_json_string(value),
        None => "null".into(),
    }
}

/// 🎯️ `applyRemedy.checkId` (also accepts `check_id`).
pub fn check_id_arg(args: Option<&dsl::DslValue>) -> String {
    args.and_then(|value| value.get("checkId").or_else(|| value.get("check_id")))
        .and_then(|value| if let dsl::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(dsl::json::to_json_string(value)) })
        .unwrap_or_default()
}

fn dsl_value_from_json(json: &str) -> Result<dsl::DslValue, String> {
    pack::json::from_json_str::<dsl::DslValue>(json).map_err(|error| error.to_string())
}

/// ✏️ Shared `setField` handler body used by every family's command leaf.
pub fn dispatch_set_field<D, M, F>(document: &D, path: &str, value_json: &str, from_snapshot: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    D: Clone + dsl::ToValue + dsl::FromValue,
    F: FnOnce(&D, &D) -> Vec<M>,
{
    let value = dsl_value_from_json(value_json).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.set-field-value"), error))?;
    handle_set_field(document, path, value, from_snapshot)
}

/// ➕ Shared `insertItem` handler body.
pub fn dispatch_insert_item<D, M, F>(document: &D, path: &str, index: usize, value_json: Option<&str>, from_snapshot: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    D: Clone + dsl::ToValue + dsl::FromValue,
    F: FnOnce(&D, &D) -> Vec<M>,
{
    let value = match value_json {
        Some(json) if json != "null" => Some(dsl_value_from_json(json).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.insert-item-value"), error))?),
        _ => None,
    };
    handle_insert_item(document, path, index, value, from_snapshot)
}

/// ➖ Shared `removeItem` handler body.
pub fn dispatch_remove_item<D, M, F>(document: &D, path: &str, index: usize, from_snapshot: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    D: Clone + dsl::ToValue + dsl::FromValue,
    F: FnOnce(&D, &D) -> Vec<M>,
{
    handle_remove_item(document, path, index, from_snapshot)
}

/// 🩹 Shared `applyRemedy` handler body.
pub fn dispatch_apply_remedy<Fam, Map>(document: &Fam::Document, check_id: &str, remedy_index: usize, from_snapshot: Map) -> Result<Emit<Fam::Mutation, NoConfigMutation>, Fault>
where
    Fam: NormFamily,
    Fam::Document: Clone + dsl::ToValue + dsl::FromValue,
    Map: FnOnce(&Fam::Document, &Fam::Document) -> Vec<Fam::Mutation>,
{
    handle_apply_remedy::<Fam, Map>(document, check_id, remedy_index, from_snapshot)
}


/// 🌉️ Installs the `{action, args}` → typed-`Command` bridge every norm editor needs, for the eight
/// verbs all fifteen declare (`setSnapshot`/`evaluate`/`setSelectedCheckIndex`/`setActiveExample`/`setField`/`insertItem`/`removeItem`/`applyRemedy`).
///
/// 🩹️ `ArtifactEditor::command_from_action`'s default refuses EVERY id — `app.command.unsupported:
/// action '…' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand)`
/// — so an editor that does not override it has an Actions rail whose every row is inert, however
/// green its `--lib` tests are. Exactly ONE of the fifteen norm editors (🧱️din4108) carried the
/// bridge, hand-written; the other fourteen were dispatch-dead in every shell. Measured inside the
/// real `s` host on a spawned `norm-din16798` (ticket 26/09/18, S7 §4 / S8 §4.3): `setSnapshot
/// refused: dispatch-failed (user window=norm-10::norm-din16798-inputs)`, which S8 attributed to the
/// HOST's routing. It is not the host: the refusal text is this trait default's own, raised by the
/// guest, and the fix is the bridge each editor owes.
///
/// `$command` is the editor's aggregated command enum; `$decode` is that artifact's own
/// `decode_<variant>_snapshot_json`. The three payload modules (`evaluate`, `selected_check`,
/// `set_snapshot`) resolve at the CALL site, which every norm editor already imports.
#[macro_export]
macro_rules! norm_command_from_action {
    // 📝️ The `text` shape. Thirteen norm editors declare `ReplaceSnapshot { snapshot: XSnapshot }`
    // and decode the shell's camelCase JSON into it; `⚖️en1990` and `⚡️din18599` declare
    // `ReplaceSnapshot { text: String }` instead, because their snapshot types stopped implementing
    // `dsl::DslField` when `q_k`/`climate` became composed `ArtifactChild<S>` slots, so their payload
    // carries the artifact's own `.en1990`/`.din18599` DSL text on one op-text line. The ARGUMENT is
    // the same for all fifteen: `snapshot`, the document's camelCase JSON the manifest declares, decoded
    // here with `$decode` and re-printed as the document's own escaped DSL text. This arm MUST precede
    // the `$decode:path` arm — `text` would otherwise match `$decode:path` and expand into the wrong body.
    ($command:ident, text, $decode:path) => {
        /// 🌉️ Resolves the React/wgpu shells' `{action, args}` pair into this editor's typed command,
        /// for the two editors whose `setSnapshot` payload carries DSL TEXT rather than a decoded
        /// snapshot struct.
        fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<$command, semio_framework_plugin::Fault> {
            match action {
                "evaluate" => Ok($command::Evaluate(evaluate::Evaluate {})),
                "setSelectedCheckIndex" => Ok($command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: $crate::app_surface::selected_check_index_arg(args) })),
                "setActiveExample" => {
                    let example_id = args
                        .and_then(|value| value.get("exampleId").or_else(|| value.get("example_id")).or_else(|| value.get("value")))
                        .and_then(|value| if let dsl::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(dsl::json::to_json_string(value)) })
                        .unwrap_or_default();
                    Ok($command::SetActiveExample(set_active_example::SetActiveExample { example_id }))
                }
                "setField" => Ok($command::SetField(set_field::SetField {
                    path: $crate::app_surface::path_arg(args),
                    value_json: $crate::app_surface::value_arg_json(args),
                })),
                "insertItem" => Ok($command::InsertItem(insert_item::InsertItem {
                    path: $crate::app_surface::path_arg(args),
                    index: $crate::app_surface::index_arg(args, "index") as u32,
                    value_json: {
                        let raw = $crate::app_surface::value_arg_json(args);
                        if raw == "null" { None } else { Some(raw) }
                    },
                })),
                "removeItem" => Ok($command::RemoveItem(remove_item::RemoveItem {
                    path: $crate::app_surface::path_arg(args),
                    index: $crate::app_surface::index_arg(args, "index") as u32,
                })),
                "applyRemedy" => Ok($command::ApplyRemedy(apply_remedy::ApplyRemedy {
                    check_id: $crate::app_surface::check_id_arg(args),
                    remedy_index: $crate::app_surface::index_arg(args, "remedyIndex") as u32,
                })),
                "setSnapshot" => {
                    let json = args
                        .and_then(|value| value.get("snapshot"))
                        .and_then(|value| if let dsl::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(dsl::json::to_json_string(value)) })
                        .ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.set-snapshot-arg-missing"), "setSnapshot needs a 'snapshot' argument carrying the document's camelCase JSON"))?;
                    let snapshot = $decode(&json).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.set-snapshot-arg-invalid"), error))?;
                    Ok($command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { text: $crate::document::escape_op_text_field(&store::ArtifactDsl::print_dsl(&snapshot)) }))
                }
                other => Err(semio_framework_plugin::Fault::new(
                    semio_framework_plugin::FaultOrigin::App,
                    semio_framework_plugin::FaultCode::new("norm.unhandled-action"),
                    format!("action '{other}' is not one of this app's declared verbs (setSnapshot/evaluate/setSelectedCheckIndex/setActiveExample/setField/insertItem/removeItem/applyRemedy)"),
                )),
            }
        }
    };
    ($command:ident, $decode:path) => {
        /// 🌉️ Resolves the React/wgpu shells' `{action, args}` pair into this editor's typed command.
        /// `setSnapshot` carries the whole compliance document, so its argument is that document's own
        /// camelCase JSON projection (exactly what the Inputs window renders).
        fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<$command, semio_framework_plugin::Fault> {
            match action {
                "evaluate" => Ok($command::Evaluate(evaluate::Evaluate {})),
                "setSelectedCheckIndex" => Ok($command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: $crate::app_surface::selected_check_index_arg(args) })),
                "setActiveExample" => {
                    let example_id = args
                        .and_then(|value| value.get("exampleId").or_else(|| value.get("example_id")).or_else(|| value.get("value")))
                        .and_then(|value| if let dsl::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(dsl::json::to_json_string(value)) })
                        .unwrap_or_default();
                    Ok($command::SetActiveExample(set_active_example::SetActiveExample { example_id }))
                }
                "setField" => Ok($command::SetField(set_field::SetField {
                    path: $crate::app_surface::path_arg(args),
                    value_json: $crate::app_surface::value_arg_json(args),
                })),
                "insertItem" => Ok($command::InsertItem(insert_item::InsertItem {
                    path: $crate::app_surface::path_arg(args),
                    index: $crate::app_surface::index_arg(args, "index") as u32,
                    value_json: {
                        let raw = $crate::app_surface::value_arg_json(args);
                        if raw == "null" { None } else { Some(raw) }
                    },
                })),
                "removeItem" => Ok($command::RemoveItem(remove_item::RemoveItem {
                    path: $crate::app_surface::path_arg(args),
                    index: $crate::app_surface::index_arg(args, "index") as u32,
                })),
                "applyRemedy" => Ok($command::ApplyRemedy(apply_remedy::ApplyRemedy {
                    check_id: $crate::app_surface::check_id_arg(args),
                    remedy_index: $crate::app_surface::index_arg(args, "remedyIndex") as u32,
                })),
                "setSnapshot" => {
                    let text = args
                        .and_then(|value| value.get("snapshot"))
                        .and_then(|value| if let dsl::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(dsl::json::to_json_string(value)) })
                        .ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.set-snapshot-arg-missing"), "setSnapshot needs a 'snapshot' argument carrying the document's camelCase JSON"))?;
                    let snapshot = $decode(&text).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.set-snapshot-arg-invalid"), error))?;
                    Ok($command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot }))
                }
                other => Err(semio_framework_plugin::Fault::new(
                    semio_framework_plugin::FaultOrigin::App,
                    semio_framework_plugin::FaultCode::new("norm.unhandled-action"),
                    format!("action '{other}' is not one of this app's declared verbs (setSnapshot/evaluate/setSelectedCheckIndex/setActiveExample/setField/insertItem/removeItem/applyRemedy)"),
                )),
            }
        }
    };
}
//#endregion 🔖️Commands

//#region 🔖️Views
/// 📄️ Reads the document out of a `ArtifactView` — spelled once so every app's `render`/`handle` reads
/// it the same way.
pub fn snapshot<'a, D>(doc: &'a ArtifactView<'_, D>) -> &'a D {
    doc.snapshot
}
//#endregion 🔖️Views

//#region 🧵️RetainedCommands
/// 🧾️ Every norm tool id, in `app_commands!` row order. All fifteen apps declare exactly this set, so
/// the list, [`NORM_PUBLICATION_CONTRACTS`], every factory key set and every `bounded_first_step_tool_proofs!`
/// block are driven from this one constant (including `setActiveExample`).
pub const NORM_RETAINED_TOOL_IDS: &[&str] = &["setSnapshot", "evaluate", "setSelectedCheckIndex", "setActiveExample", "setField", "insertItem", "removeItem", "applyRemedy"];
/// 🧬️ The payload schema id every norm retained command job is admitted under.
pub const NORM_RETAINED_PAYLOAD_SCHEMA: &str = "norm.tool-command.v1";
/// 🎒️ Wire ceiling for one norm tool dispatch: the largest payload is `setSnapshot`'s whole compliance
/// document, a few dozen scalar quantities plus an ordered layer list — kilobytes, never megabytes.
pub const NORM_RETAINED_RAW_BYTES: usize = 524_288;
/// 🎒️ Real bound for one Artifact-lane edit: a single `change-<field>`/`insert-layer`/`remove-layer`
/// leaf, the only artifact mutations any norm command emits.
pub const NORM_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 2_097_152;
/// 🚦️ Per-tool publication lanes, read straight off the three command bodies: `set-snapshot` commits
/// artifact mutations, `evaluate` emits nothing at all (the report is derived on every read), and
/// `selected-check` writes persisted-local state through the exact Results-window config lane.
pub const NORM_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setSnapshot", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "evaluate", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setSelectedCheckIndex", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setField", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "insertItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "applyRemedy", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

/// 🛣️ Stable language-neutral identifier for one live publication lane.
pub const fn publication_lane_id(lane: ArtifactToolPublicationLane) -> &'static str {
    match lane {
        ArtifactToolPublicationLane::HostOnly => "host-only",
        ArtifactToolPublicationLane::Artifact => "artifact",
        ArtifactToolPublicationLane::Config => "config",
        ArtifactToolPublicationLane::Draft => "draft",
        ArtifactToolPublicationLane::Presence => "presence",
        ArtifactToolPublicationLane::Transient => "transient",
        ArtifactToolPublicationLane::WindowConfig => "window-config",
        ArtifactToolPublicationLane::WindowTransient => "window-transient",
        ArtifactToolPublicationLane::Child => "child",
        ArtifactToolPublicationLane::Interaction => "interaction",
    }
}

/// ⏱️ The one bounded-first-step contract all forty-five norm tool identities share.
pub fn norm_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(NORM_RETAINED_RAW_BYTES, 64, 64, 65_536, 7_999)
}

/// 🧵️ The per-app half of the shared factory: an editor states only how its own aggregated command enum
/// dispatches, and inherits every retained-command constant, reducer, factory and store preparation
/// below. `dispatch_retained` MUST route into the app's `🎮️commands/*` bodies, which stay the sole
/// authority for what a norm command does.
pub trait NormRetainedEditor: semio_framework_plugin::ArtifactEditor<Config = NoConfig, ConfigMutation = NoConfigMutation, DraftMutation = semio_framework_plugin::NoDraftMutation> {
    type Family: NormFamily<Document = Self::Snapshot, Mutation = Self::Mutation>;
    type ResultsWindowConfigOwner: WindowConfigOwner<State = crate::results_window_config::NormResultsWindowConfig, Mutation = crate::results_window_config::NormResultsWindowConfigMutation>;

    fn selected_check_window_mutation(command: &Self::Command) -> Option<crate::results_window_config::NormResultsWindowConfigMutation>;

    fn dispatch_retained(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> NormRetainedCommandResult<Self::Mutation>;
}

/// 🧮️ Cooperative evaluate-job state (progress phases + cancel + checkpoint bytes).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NormEvaluateWorkState {
    pub phase: u8,
    pub cancelled: bool,
}

impl NormEvaluateWorkState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.is_empty() {
            return Err(Fault::from("norm-evaluate-checkpoint-capacity"));
        }
        target[0] = self.phase;
        Ok(1)
    }

    pub fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        self.phase = checkpoint.first().copied().unwrap_or(0);
        self.cancelled = false;
        Ok(())
    }

    pub fn begin_close(&mut self) {
        self.cancelled = true;
    }

    pub fn emit_progress_stage(&mut self) -> Option<&'static str> {
        match self.phase {
            0 => {
                self.phase = 1;
                Some("norm-evaluate-prepare")
            }
            1 => {
                self.phase = 2;
                Some("norm-evaluate-run")
            }
            _ => None,
        }
    }

    pub fn ready_to_evaluate(&self) -> bool {
        self.phase >= 2 && !self.cancelled
    }
}

/// 🧮️ Multi-step retained `evaluate` work: progress events, cooperative cancel, checkpoint/restore, revision-keyed report cache.
pub struct NormEvaluateCommandWork<A: NormRetainedEditor> {
    tool_id: &'static str,
    state: NormEvaluateWorkState,
    _owner: std::marker::PhantomData<fn() -> A>,
}

impl<A: NormRetainedEditor> NormEvaluateCommandWork<A> {
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id, state: NormEvaluateWorkState::new(), _owner: std::marker::PhantomData }
    }
}

impl<A: NormRetainedEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<A>> for NormEvaluateCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        _command: &A::Command,
        _snapshot: &A::Snapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<A>>>,
    ) -> Option<usize> {
        Some(3)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<A>>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<A>>, Fault> {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.state.cancelled {
            return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
        }
        if let Some(stage) = self.state.emit_progress_stage() {
            let preview: &[u8] = match stage {
                "norm-evaluate-prepare" => br#"{"en":"Preparing evaluation","de":"Auswertung wird vorbereitet"}"#.as_slice(),
                _ => br#"{"en":"Evaluating checks","de":"Nachweise werden ausgewertet"}"#.as_slice(),
            };
            return Ok(ArtifactCommandWorkStep::Progress { stage, preview });
        }
        invalidate_cached_report_for::<A::Family>(input.snapshot);
        let report = <A::Family as NormFamily>::evaluate(input.snapshot);
        store_cached_report_for::<A::Family>(input.snapshot, report);
        let _ = document_revision_key(input.snapshot);
        let emit = norm_retained_reduce::<A>(
            input.command,
            input.snapshot,
            input.config,
            input.history,
            input.interaction,
            input.hover,
            input.context,
            input.operation,
        )?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        self.state.checkpoint(target)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        self.state.restore(checkpoint)
    }

    fn begin_close(&mut self) {
        self.state.begin_close();
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        InteractiveJobCloseStep::Complete
    }
}


/// 🎯️ Routes ordinary and retained commands through the same exact Results-window address.
pub fn dispatch_norm_command<A: NormRetainedEditor>(
    command: &A::Command,
    doc: &ArtifactView<'_, A::Snapshot>,
    cfg: &ConfigView<'_, NoConfig>,
    view: Option<&semio_framework_plugin::ViewModel>,
) -> NormRetainedCommandResult<A::Mutation> {
    let Some(mutation) = A::selected_check_window_mutation(command) else {
        return A::dispatch_retained(command, doc, cfg);
    };
    let view = view.ok_or_else(|| Fault::from("norm-results-window-view-required"))?;
    Ok(Emit {
        window_config_mutations: vec![crate::results_window_config::addressed::<A::ResultsWindowConfigOwner>(view, mutation)?],
        ..Default::default()
    })
}

/// 🧵️ The retained reducer shared by all fifteen apps — no norm command reads selection or hover, so the
/// interaction owners are unused and the reduction is exactly the ordinary `handle` path.
///
/// 🔁️ `artifact_mutations` is handed back in authored order: the retained publication lane stages the
/// whole bundle front-to-back into ONE batched edit (`store::begin_apply_batch`), exactly as
/// `Emit::commit`'s ordinary dispatch applies it. `XMutation::from_snapshot` emits ordered
/// `remove-layer`/`insert-layer` runs, so the published document is identical either way — the LIFO
/// compensation the old one-mutation-per-turn drain needed is gone with that drain.
#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub fn norm_retained_reduce<A: NormRetainedEditor>(
    command: &A::Command,
    snapshot: &A::Snapshot,
    config: &A::Config,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<A>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> NormRetainedCommandResult<A::Mutation> {
    if !NORM_RETAINED_TOOL_IDS.contains(&A::command_id(command)) {
        return Err(Fault::from("norm-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let window = context.and_then(|context| context.window_config.as_ref());
    let emit = dispatch_norm_command::<A>(command, &doc, &ConfigView { snapshot: config, window }, context.and_then(|context| context.view_state.as_ref()))?;
    Ok(emit)
}

/// 📏️ Every norm command is one bounded step; none of the three walks a collection incrementally.
pub fn norm_bounded_extent<A: NormRetainedEditor>(_command: &A::Command, _snapshot: &A::Snapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 🏭️ The one owned bounded tool-job factory serving all fifteen norm editors — generic over the app, so
/// the per-standard `Snapshot`/`Mutation`/`Command` types are the only thing that varies and the owner
/// witness stays each app's own concrete `EditorApp<A>`.
pub struct NormBoundedCommandJobFactory<A: NormRetainedEditor> {
    keys: Vec<semio_framework::ToolFactoryKey>,
    owner: std::marker::PhantomData<fn() -> A>,
}

impl<A: NormRetainedEditor> NormBoundedCommandJobFactory<A> {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: NORM_RETAINED_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect(), owner: std::marker::PhantomData }
    }
}

impl<A: NormRetainedEditor> semio_framework::ToolJobFactory for NormBoundedCommandJobFactory<A> {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<A>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<A>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        NORM_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework_plugin::InteractiveJobClassification {
        semio_framework_plugin::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        norm_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > NORM_RETAINED_RAW_BYTES
            || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES)
        {
            return Err((semio_framework::ToolJobFactoryError::new("norm retained command rejects oversized wire or checkpoint"), input, checkpoint));
        }
        match checkpoint {
            Some(checkpoint) => Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint)),
            None => Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input)),
        }
    }
}

impl<A: NormRetainedEditor> semio_framework_plugin::ArtifactOwnedToolJobFactory for NormBoundedCommandJobFactory<A> {
    type Owner = semio_framework_plugin::EditorApp<A>;
    const TOOL_IDS: &'static [&'static str] = NORM_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = <A as semio_framework_plugin::ArtifactEditor>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = NORM_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 🔌️EditorOverrides
/// 📬️ `ArtifactEditor::build_artifact_store_one_item_preparation_factory` for every norm editor —
/// the framework's own bounded one-item publication authority, the same one trinity, dag and
/// reasoning bind. Norm used to carry a hand-copied twin of it whose `preflight` declared
/// `ArtifactStoreOneItemFootprint { work_items: 1, .. }`; a point-invertible item folds TWO staged
/// rows (`store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS`), so every `setSnapshot` was
/// rejected by `ArtifactStore::fold_batch_item`'s fixed fold contract and the document never
/// published (ticket 26/09/18 slice B2c).
pub fn norm_artifact_store_preparation<A: NormRetainedEditor>() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>>
where
    A::Mutation: Clone + Sync,
{
    Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<A::Snapshot, A::Mutation>("norm-artifact-retained", NORM_ARTIFACT_STORE_MAXIMUM_BYTES))
}

/// 🏭️ Declares one norm app's concrete owned factory as a newtype over the shared generic
/// [`NormBoundedCommandJobFactory`], plus its `register_tool_job_factories` entry point. The newtype is
/// required, not decorative: `ArtifactBoundedFirstStepProof` joins the `factory:` literal against the
/// last `::` segment of `std::any::type_name`, which for a generic instantiation is the owner app's own
/// name followed by `>`. Every behavioural line still lives once, in the generic base this delegates to.
#[macro_export]
macro_rules! norm_owned_tool_job_factory {
    ($factory:ident, $app:ty) => {
        pub struct $factory($crate::app_surface::NormBoundedCommandJobFactory<$app>);

        impl $factory {
            pub fn new(controller_id: &str) -> Self {
                Self($crate::app_surface::NormBoundedCommandJobFactory::<$app>::new(controller_id))
            }

            pub fn register(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<$app>>) -> Result<(), semio_framework_plugin::Fault> {
                let controller = registry.controller_id().to_string();
                registry.register(Self::new(&controller))
            }
        }

        impl semio_framework::ToolJobFactory for $factory {
            type Payload = <$crate::app_surface::NormBoundedCommandJobFactory<$app> as semio_framework::ToolJobFactory>::Payload;
            type Job = <$crate::app_surface::NormBoundedCommandJobFactory<$app> as semio_framework::ToolJobFactory>::Job;

            fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
                semio_framework::ToolJobFactory::keys(&self.0)
            }

            fn payload_schema_id(&self) -> &str {
                semio_framework::ToolJobFactory::payload_schema_id(&self.0)
            }

            fn classification(&self) -> semio_framework_plugin::InteractiveJobClassification {
                semio_framework::ToolJobFactory::classification(&self.0)
            }

            fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
                semio_framework::ToolJobFactory::execution_contract(&self.0)
            }

            fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
                semio_framework::ToolJobFactory::create_job(&mut self.0, operation, payload)
            }

            fn create_job_from_wire_pages_with_payload(
                &mut self,
                operation: semio_framework_job::Operation,
                payload: Self::Payload,
                input: semio_framework::action_bus::RetainedToolWireInput,
                checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
            ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
                semio_framework::ToolJobFactory::create_job_from_wire_pages_with_payload(&mut self.0, operation, payload, input, checkpoint)
            }
        }

        impl semio_framework_plugin::ArtifactOwnedToolJobFactory for $factory {
            type Owner = semio_framework_plugin::EditorApp<$app>;
            const TOOL_IDS: &'static [&'static str] = $crate::app_surface::NORM_RETAINED_TOOL_IDS;
            const DOCUMENT_SCHEMA: &'static str = <$app as semio_framework_plugin::ArtifactEditor>::DOCUMENT_SCHEMA;
            const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = $crate::app_surface::NORM_PUBLICATION_CONTRACTS;
        }
    };
}

/// 🧵️ `ArtifactEditor::build_tool_job` for every norm editor.
pub fn build_norm_tool_job<A: NormRetainedEditor>(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<A>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
    if !NORM_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
        return Ok(None);
    }
    let tool_id = A::command_id(&request.command);
    if tool_id != request.tool_id {
        return Err(Fault::from("norm-command-tool-mismatch"));
    }
    let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<A>>> = if tool_id == "evaluate" {
        Box::new(NormEvaluateCommandWork::<A>::new(tool_id))
    } else {
        Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, norm_retained_reduce::<A>, norm_bounded_extent::<A>))
    };
    let maximum_work_items = if tool_id == "evaluate" { 3 } else { 1 };
    let operation = semio_framework_plugin::AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
    };
    let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
        semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation,
            completion: request.completion,
        },
        A::command_id,
        NORM_RETAINED_RAW_BYTES,
        maximum_work_items,
        work,
    )?;
    Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}
//#endregion 🔌️EditorOverrides

#[cfg(any(test, feature = "compliance-testing"))]
//#region 🧵️RetainedDispositionOracle
#[path = "🧪️tests/🔬️retained-disposition-oracle/🦀️.rs"]
pub mod retained_disposition_oracle;
//#endregion 🧵️RetainedDispositionOracle

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
