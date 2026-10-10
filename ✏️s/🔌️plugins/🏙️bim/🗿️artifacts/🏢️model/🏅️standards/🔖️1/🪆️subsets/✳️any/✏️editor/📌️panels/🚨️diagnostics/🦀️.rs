//! 🚨️ BIM diagnostics panel: the findings of the model as one virtualised tree. Three severity groups (errors, warnings, notes) hold one group per storey (the model-wide findings last), each storey holds one group per kind of finding
//! (clash, reference, opening, ...), and a row is one finding with its message in the locale of the view. Opening or closing a severity group is the severity filter; the host keeps the open state per view, so the filter is customised by
//! the author and survives a repaint. Activating a finding selects the elements it names in the framework `elements` domain, which the plan, the 3D view and the outliner share (`selectFindings`).

use crate::editor::bim::entities::{kind_holding, kind_of};
use crate::editor::bim::kit::{bim_action, ui_capacity_error, ui_label, ui_value_list, ui_value_map, ui_value_text};
use crate::editor::bim::panels::outliner::{item, leaf};
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::{categories, Diagnostic, Severity};
use crate::{ModelInference, ModelSnapshot};
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::BTreeMap;

//#region 🔖️Constants
pub const BODY_KEY: &str = "bim.edit.diagnostics";
pub const ROOT: &str = "bim-diagnostics";
/// ✂️ The longest message or description a row shows, in characters (a fixed UI text holds 512 bytes).
const CLIP: usize = 240;
/// 📂️ A kind of finding with at most this many findings opens on first paint; a longer list waits for the host to open it and then streams its slice.
const OPEN_GROUP_LIMIT: usize = 12;
/// 🚦️ The severities in the order of the groups: the most severe first.
pub const SEVERITIES: [Severity; 3] = [Severity::Error, Severity::Warning, Severity::Info];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: BimLabels::localized(|labels| labels.panel_diagnostics),
        group: PanelGroup::Details,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Grouping
/// 🗂️ The findings of one kind in one storey, as indices into the ordered findings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KindGroup {
    pub category: &'static str,
    pub findings: Vec<usize>,
}

/// 🪜️ The kinds of finding of one storey (`None` is the whole model).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreyGroup {
    pub storey: Option<String>,
    pub kinds: Vec<KindGroup>,
}

impl StoreyGroup {
    /// 🔢️ How many findings the storey holds.
    pub fn total(&self) -> usize {
        self.kinds.iter().map(|kind| kind.findings.len()).sum()
    }
}

/// 🗂️ The findings of one severity grouped by storey (by level, the unknown storeys after the known ones, the whole model last) and then by kind (in the order of the codes).
pub fn groups(snapshot: &ModelSnapshot, found: &[Diagnostic], severity: Severity) -> Vec<StoreyGroup> {
    let mut by_storey: BTreeMap<(u8, i32, Option<String>), BTreeMap<&'static str, Vec<usize>>> = BTreeMap::new();
    for (index, finding) in found.iter().enumerate().filter(|(_, finding)| finding.severity == severity) {
        let key = match &finding.storey {
            Some(storey) => match snapshot.storeys.get(storey) {
                Some(row) => (0, row.level, Some(storey.clone())),
                None => (1, 0, Some(storey.clone())),
            },
            None => (2, 0, None),
        };
        by_storey.entry(key).or_default().entry(finding.code.category()).or_default().push(index);
    }
    let order = categories();
    by_storey
        .into_iter()
        .map(|((_, _, storey), mut kinds)| StoreyGroup { storey, kinds: order.iter().filter_map(|&category| kinds.remove(category).map(|findings| KindGroup { category, findings })).collect() })
        .collect()
}

/// ⚖️ The canonical JSON of the groups of every severity, `{ <severity>: [ { storey, kinds: { <category>: <count> } } ] }` in the order of the panel: the table the third-party oracle recomputes from the exported findings.
pub fn groups_json(snapshot: &ModelSnapshot, found: &[Diagnostic]) -> String {
    use semio_framework_plugin::DslValue;
    let rows = |severity: Severity| {
        DslValue::Array(
            groups(snapshot, found, severity)
                .into_iter()
                .map(|group| {
                    let kinds = DslValue::object(group.kinds.iter().map(|kind| (kind.category.to_string(), DslValue::float(kind.findings.len() as f64))));
                    DslValue::object([("storey".to_string(), group.storey.map_or(DslValue::Null, DslValue::String)), ("kinds".to_string(), kinds)])
                })
                .collect(),
        )
    };
    semio_framework_pack_json::to_json_string(&DslValue::object(SEVERITIES.into_iter().map(|severity| (severity_key(severity).to_string(), rows(severity)))))
}

/// ⚖️ [`groups_json`] of the findings the model graph infers for `model`.
pub fn groups_json_of(model: &ModelSnapshot) -> Result<String, semio_framework_value::ValueError> {
    use protocol::Inference;
    ModelInference::infer(model).map(|inferred| groups_json(model, &inferred.diagnostics))
}
//#endregion 🔖️Grouping

//#region 🔖️Labels
/// 🗣️ The heading of a severity group.
pub fn severity_group_label(labels: &BimLabels, severity: Severity) -> &str {
    match severity {
        Severity::Error => labels.diag_errors.as_str(),
        Severity::Warning => labels.diag_warnings.as_str(),
        Severity::Info => labels.diag_notes.as_str(),
    }
}

/// 🗣️ The word for one finding of a severity, so a reader never depends on the icon.
pub fn severity_label(labels: &BimLabels, severity: Severity) -> &str {
    match severity {
        Severity::Error => labels.diag_error.as_str(),
        Severity::Warning => labels.diag_warning.as_str(),
        Severity::Info => labels.diag_note.as_str(),
    }
}

fn severity_icon(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "alert-circle",
        Severity::Warning => "triangle-alert",
        Severity::Info => "info",
    }
}

fn severity_key(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "note",
    }
}

/// 🗣️ The heading of a kind of finding; a category this table does not know yet shows its own name.
pub fn category_label<'a>(labels: &'a BimLabels, category: &'a str) -> &'a str {
    match category {
        "annotation" => labels.diag_cat_annotation.as_str(),
        "ceiling" => labels.diag_cat_ceiling.as_str(),
        "clash" => labels.diag_cat_clash.as_str(),
        "curtain-wall" => labels.diag_cat_curtain_wall.as_str(),
        "degenerate" => labels.diag_cat_degenerate.as_str(),
        "opening" => labels.diag_cat_opening.as_str(),
        "railing" => labels.diag_cat_railing.as_str(),
        "ramp" => labels.diag_cat_ramp.as_str(),
        "reference" => labels.diag_cat_reference.as_str(),
        "roof" => labels.diag_cat_roof.as_str(),
        "space" => labels.diag_cat_space.as_str(),
        "stair" => labels.diag_cat_stair.as_str(),
        "storey" => labels.diag_cat_storey.as_str(),
        "wall" => labels.diag_cat_wall.as_str(),
        "wall-sweep" => labels.diag_cat_wall_sweep.as_str(),
        other => other,
    }
}

fn clip(text: &str) -> String {
    if text.chars().count() <= CLIP {
        return text.to_string();
    }
    let mut clipped: String = text.chars().take(CLIP - 1).collect();
    clipped.push('…');
    clipped
}

fn storey_name<'a>(snapshot: &'a ModelSnapshot, labels: &'a BimLabels, storey: Option<&'a String>) -> &'a str {
    match storey {
        Some(id) => snapshot.storeys.get(id).map_or(id.as_str(), |row| row.name.as_str()),
        None => labels.diag_model_wide.as_str(),
    }
}
//#endregion 🔖️Labels

//#region 🔖️Rows
fn element_names(snapshot: &ModelSnapshot, found: &Diagnostic) -> String {
    found.elements.iter().map(|id| kind_holding(snapshot, id).and_then(|row| (row.name)(snapshot, id)).unwrap_or_else(|| id.clone())).collect::<Vec<_>>().join(", ")
}

fn finding_row(snapshot: &ModelSnapshot, labels: &BimLabels, found: &Diagnostic, index: usize) -> UiAssemblyResult<BuiltNode> {
    let message = found.text(labels.locale().as_str()).unwrap_or_else(|| found.message_key.clone());
    let description = clip(&format!("{} · {}", severity_label(labels, found.severity), element_names(snapshot, found)));
    let args = ui_value_map([("ids", ui_value_list(found.elements.iter().map(|id| ui_value_text(id)).collect::<UiAssemblyResult<Vec<_>>>()?)?)])?;
    let mut node = semio_framework_plugin::tree_item_with_action(format!("{ROOT}.finding.{index}"), clip(&message).as_str(), Some(description), bim_action("selectFindings", Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(severity_icon(found.severity)).ok_or_else(ui_capacity_error)?);
    }
    Ok(node)
}

fn kind_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, labels: &BimLabels, found: &[Diagnostic], group_id: &str, kind: &KindGroup) -> UiAssemblyResult<BuiltNode> {
    let id = format!("{group_id}::{}", kind.category);
    let builder = item(&id, category_label(labels, kind.category), "folder", None, Some(&kind.findings.len().to_string()))?;
    semio_framework_plugin::tree_window_indexed_item(windows, builder, &id, kind.findings.len() <= OPEN_GROUP_LIMIT, kind.findings.len(), |position| {
        let index = kind.findings[position];
        finding_row(snapshot, labels, &found[index], index)
    })
}

fn storey_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, labels: &BimLabels, found: &[Diagnostic], severity: Severity, group: &StoreyGroup) -> UiAssemblyResult<BuiltNode> {
    let id = format!("{ROOT}.{}::{}", severity_key(severity), group.storey.as_deref().unwrap_or("model"));
    let icon = group.storey.as_ref().and_then(|_| kind_of("storey")).map_or("building", |row| row.icon);
    let builder = item(&id, storey_name(snapshot, labels, group.storey.as_ref()), icon, None, Some(&group.total().to_string()))?;
    semio_framework_plugin::tree_window_indexed_item(windows, builder, &id, true, group.kinds.len(), |position| kind_row(windows, snapshot, labels, found, &id, &group.kinds[position]))
}

fn empty_row(labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    leaf(item(&format!("{ROOT}.empty"), labels.diag_empty.as_str(), "info", None, None)?)
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🚨️ Renders the diagnostics panel: a group per severity that has findings, an empty-state row when the model has none.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let found = &inference.diagnostics;
    let mut builder = PanelTreeBuilder::new(ROOT)?;
    if found.is_empty() {
        return builder.section(format!("{ROOT}.none"), Some(ui_label(labels.panel_diagnostics.as_str())?), true, semio_framework_plugin::ui_node_list([empty_row(labels)])?)?.build();
    }
    for severity in SEVERITIES {
        let rows = groups(snapshot, found, severity);
        if rows.is_empty() {
            continue;
        }
        let count: usize = rows.iter().map(StoreyGroup::total).sum();
        let heading = format!("{} ({count})", severity_group_label(labels, severity));
        builder = builder.window_section(windows, &format!("{ROOT}.{}", severity_key(severity)), Some(ui_label(&heading)?), severity != Severity::Info, &rows, |group| storey_row(windows, snapshot, labels, found, severity, group))?;
    }
    builder.build()
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
