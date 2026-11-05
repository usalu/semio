//! ✅️ Checks window of the norm app surface: grouped, sortable, filterable compliance checks with utilisation bars, remedy chips and clause links. See [`super`].

use super::*;

#[path = "📶️utilisation-bar/🦀️.rs"]
mod utilisation_bar;
pub use utilisation_bar::*;

#[path = "🔗️clause-link/🦀️.rs"]
mod clause_link;
pub use clause_link::*;

#[path = "🩹️remedy-chips/🦀️.rs"]
mod remedy_chips;
pub use remedy_chips::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

//#region 🔖️ViewState
/// 🗂️ What the checks list is grouped by.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChecksGrouping {
    #[default]
    Entity,
    Clause,
    Part,
}

/// ↕️ How checks (and groups) are ordered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChecksSort {
    #[default]
    WorstFirst,
    ReportOrder,
    ClauseOrder,
    Title,
}

/// 🔎️ Status toggles and free-text query of the checks list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChecksFilter {
    pub pass: bool,
    pub warning: bool,
    pub fail: bool,
    pub not_applicable: bool,
    pub query: String,
}

impl Default for ChecksFilter {
    fn default() -> Self {
        Self { pass: true, warning: true, fail: true, not_applicable: true, query: String::new() }
    }
}

impl ChecksFilter {
    pub fn admits_status(&self, status: CheckStatus) -> bool {
        match status {
            CheckStatus::Pass => self.pass,
            CheckStatus::Warning | CheckStatus::UnverifiedBasis => self.warning,
            CheckStatus::Fail => self.fail,
            CheckStatus::NotApplicable | CheckStatus::Info => self.not_applicable,
        }
    }

    /// 🔎️ Case-insensitive match of the query against title, subject label and clause reference in the active `locale`, plus entity, path, part, check id and clause id parts.
    pub fn admits(&self, check: &CheckResult, locale: Locale) -> bool {
        if !self.admits_status(check.status) {
            return false;
        }
        let query = self.query.trim().to_lowercase();
        if query.is_empty() {
            return true;
        }
        let protocol = protocol_locale(locale);
        let title = check.title.resolve(&protocol).to_string();
        let subject = check.subject.label.resolve(&protocol).to_string();
        let clause = clause_text(&check.clause, locale);
        [
            title.as_str(),
            subject.as_str(),
            check.subject.entity_id.as_str(),
            check.subject.path.as_str(),
            check.part.as_str(),
            check.id.as_str(),
            check.clause.family.as_str(),
            check.clause.part.as_str(),
            check.clause.section.as_str(),
            clause.as_str(),
        ]
        .iter()
        .any(|haystack| haystack.to_lowercase().contains(&query))
    }
}

/// ⏳️ Progress of a running evaluation shown above the list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChecksPending {
    pub completed: u32,
    pub total: u32,
}

/// 🪟️ Everything the checks list shows besides the report: grouping, sort, filter, selected check, evaluation progress and the entities picked in the viewport (`owners`, base ids, empty shows every check; never part of the wire form).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChecksViewState {
    pub grouping: ChecksGrouping,
    pub sort: ChecksSort,
    pub filter: ChecksFilter,
    pub selected: Option<usize>,
    pub pending: Option<ChecksPending>,
    pub owners: Vec<String>,
}

impl ChecksViewState {
    /// 🧬️ Wire form carried by the `setChecksView` action: `grouping=entity;sort=worst;status=pwfn;query=text`.
    pub fn encode(&self) -> String {
        let grouping = match self.grouping {
            ChecksGrouping::Entity => "entity",
            ChecksGrouping::Clause => "clause",
            ChecksGrouping::Part => "part",
        };
        let sort = match self.sort {
            ChecksSort::WorstFirst => "worst",
            ChecksSort::ReportOrder => "report",
            ChecksSort::ClauseOrder => "clause",
            ChecksSort::Title => "title",
        };
        let status: String = [(self.filter.pass, 'p'), (self.filter.warning, 'w'), (self.filter.fail, 'f'), (self.filter.not_applicable, 'n')].iter().filter(|(on, _)| *on).map(|(_, letter)| *letter).collect();
        let query = self.filter.query.replace([';', '='], " ");
        format!("grouping={grouping};sort={sort};status={status};query={query}")
    }

    /// 🧬️ Inverse of [`Self::encode`]; unknown keys and values are rejected, never defaulted.
    pub fn decode(text: &str) -> Option<Self> {
        let mut state = Self::default();
        for part in text.split(';') {
            let (key, value) = part.split_once('=')?;
            match key {
                "grouping" => state.grouping = match value { "entity" => ChecksGrouping::Entity, "clause" => ChecksGrouping::Clause, "part" => ChecksGrouping::Part, _ => return None },
                "sort" => state.sort = match value { "worst" => ChecksSort::WorstFirst, "report" => ChecksSort::ReportOrder, "clause" => ChecksSort::ClauseOrder, "title" => ChecksSort::Title, _ => return None },
                "status" => {
                    if value.chars().any(|letter| !"pwfn".contains(letter)) {
                        return None;
                    }
                    state.filter = ChecksFilter { pass: value.contains('p'), warning: value.contains('w'), fail: value.contains('f'), not_applicable: value.contains('n'), query: state.filter.query.clone() };
                }
                "query" => state.filter.query = value.to_string(),
                _ => return None,
            }
        }
        Some(state)
    }
}

/// 🎛️ Optional actions the window binds once the hosting artifact declares them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChecksActions {
    pub view_controls: bool,
    pub clause_links: bool,
    pub cancel_evaluation: bool,
}

/// 🪟️ How a hosting artifact wires the checks window: its controller, interaction domain and declared optional actions.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChecksWindowSpec {
    pub controller_id: Option<&'static str>,
    pub interaction_domain: Option<&'static str>,
    pub actions: ChecksActions,
}
//#endregion 🔖️ViewState

//#region 🔖️Plan
/// 🗂️ One group of the planned list: its checks (indices into the report) and rollup.
#[derive(Clone, Debug, PartialEq)]
pub struct ChecksGroup {
    pub key: String,
    pub label: String,
    pub entity_id: Option<String>,
    pub rows: Vec<usize>,
    pub worst: f64,
    pub counts: [u32; 4],
}

fn status_slot(status: CheckStatus) -> usize {
    match status {
        CheckStatus::Pass => 0,
        CheckStatus::Warning | CheckStatus::UnverifiedBasis => 1,
        CheckStatus::Fail => 2,
        CheckStatus::NotApplicable | CheckStatus::Info => 3,
    }
}

fn check_order(report: &CheckReport, sort: ChecksSort, locale: Locale) -> impl Fn(&usize, &usize) -> std::cmp::Ordering + '_ {
    let protocol = protocol_locale(locale);
    move |left, right| {
        let (a, b) = (&report.checks[*left], &report.checks[*right]);
        match sort {
            ChecksSort::ReportOrder => left.cmp(right),
            ChecksSort::WorstFirst => (b.status != CheckStatus::NotApplicable).cmp(&(a.status != CheckStatus::NotApplicable)).then(b.utilization.total_cmp(&a.utilization)).then(left.cmp(right)),
            ChecksSort::ClauseOrder => clause_order(&a.clause, &b.clause).then(left.cmp(right)),
            ChecksSort::Title => a.title.resolve(&protocol).to_lowercase().cmp(&b.title.resolve(&protocol).to_lowercase()).then(left.cmp(right)),
        }
    }
}

fn owned_by_picks(check: &CheckResult, owners: &[String]) -> bool {
    owners.is_empty() || (!check.subject.entity_id.is_empty() && owners.iter().any(|owner| owner == semio_s_plugin_norm_structural_scene::projection_kit::base_id(&check.subject.entity_id)))
}

/// 🗂️ Filters, groups and sorts a report into the groups the window draws; pure and deterministic.
pub fn plan_checks(report: &CheckReport, state: &ChecksViewState, locale: Locale) -> Vec<ChecksGroup> {
    let protocol = protocol_locale(locale);
    let owners: &[String] = if report.checks.iter().any(|check| owned_by_picks(check, &state.owners)) { &state.owners } else { &[] };
    let mut groups: Vec<ChecksGroup> = Vec::new();
    for (index, check) in report.checks.iter().enumerate().filter(|(_, check)| state.filter.admits(check, locale) && owned_by_picks(check, owners)) {
        let (key, label, entity_id) = match state.grouping {
            ChecksGrouping::Entity => (check.subject.entity_id.clone(), check.subject.label.resolve(&protocol).to_string(), Some(check.subject.entity_id.clone()).filter(|id| !id.is_empty())),
            ChecksGrouping::Clause => (format!("{} {}", check.clause.family, check.clause.part), format!("{} {}", check.clause.family, check.clause.part), None),
            ChecksGrouping::Part => (check.part.clone(), check.part.clone(), None),
        };
        let position = match groups.iter().position(|group| group.key == key) {
            Some(position) => position,
            None => {
                groups.push(ChecksGroup { key, label, entity_id, rows: Vec::new(), worst: 0.0, counts: [0; 4] });
                groups.len() - 1
            }
        };
        let group = &mut groups[position];
        group.rows.push(index);
        group.counts[status_slot(check.status)] += 1;
        if check.status != CheckStatus::NotApplicable {
            group.worst = group.worst.max(check.utilization);
        }
    }
    let order = check_order(report, state.sort, locale);
    for group in &mut groups {
        group.rows.sort_by(&order);
    }
    match state.sort {
        ChecksSort::WorstFirst => groups.sort_by(|a, b| b.worst.total_cmp(&a.worst)),
        ChecksSort::Title => groups.sort_by_key(|group| group.label.to_lowercase()),
        ChecksSort::ReportOrder | ChecksSort::ClauseOrder => {}
    }
    groups
}

/// 🎯️ The entity and field path a check selects in the viewport and the inspector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckSelection {
    pub entity_id: String,
    pub path: String,
}

/// 🎯️ Selection target of a check; `None` for a check on the whole document.
pub fn check_selection(check: &CheckResult) -> Option<CheckSelection> {
    (!check.subject.entity_id.is_empty()).then(|| CheckSelection { entity_id: check.subject.entity_id.clone(), path: check.subject.path.clone() })
}
//#endregion 🔖️Plan

//#region 🔖️Text
/// 📐️ Quantity in natural display units with the locale's decimal separator.
pub fn quantity_text(quantity: &Quantity, locale: Locale) -> String {
    localize_decimal(&format_quantity(quantity), locale)
}

/// ✅️ Status glyph plus word, so status is never carried by colour alone.
pub fn status_glyph_text(status: CheckStatus, utilization: f64, locale: Locale) -> String {
    format!("{} {}", UtilisationBand::of(status, utilization).glyph(), status_label(status, locale))
}

/// 🗣️ The facts a screen reader speaks for a check row, the one source of [`check_accessible_name`] and of the row node of the screen-reader contract.
pub fn check_row_facts(check: &CheckResult, locale: Locale) -> crate::accessibility::screen_reader::CheckRowFacts {
    use semio_s_plugin_norm_structural_scene::{CheckStatus as SceneStatus, LocalizedCopy as SceneCopy};
    let status = match check.status {
        CheckStatus::Pass => SceneStatus::Pass,
        CheckStatus::Warning | CheckStatus::UnverifiedBasis => SceneStatus::Warning,
        CheckStatus::Fail => SceneStatus::Fail,
        CheckStatus::NotApplicable | CheckStatus::Info => SceneStatus::NotApplicable,
    };
    crate::accessibility::screen_reader::CheckRowFacts {
        title: SceneCopy::new(check.title.en.as_str(), check.title.de.as_str()),
        subject: SceneCopy::new(check.subject.label.en.as_str(), check.subject.label.de.as_str()),
        status,
        utilisation: check.utilization,
        quantities: format!("{} {} {} {}", quantity_text(&check.computed, locale), chrome("of", "von", locale), quantity_text(&check.limit, locale), chrome("limit", "Grenzwert", locale)),
        clause: clause_text(&check.clause, locale),
    }
}

/// ♿️ The accessibility tree of the checks window: a table node and one row node per listed check in list order (grouped, sorted and filtered like the window), each with the spoken title, subject, status, utilisation, quantities, the clause as description, its position among the listed rows and whether it is the selected check.
pub fn check_window_nodes(report: &CheckReport, state: &ChecksViewState, locale: Locale) -> Vec<crate::accessibility::published_nodes::PublishedNode> {
    use crate::accessibility::published_nodes::PublishedNode;
    use crate::accessibility::screen_reader::{check_row_node, AriaNode};
    let lang = crate::viewport3d::config::Lang::of(locale);
    let listed: Vec<usize> = plan_checks(report, state, locale).into_iter().flat_map(|group| group.rows).collect();
    let table = AriaNode { role: "table", name: chrome("Checks", "Nachweise", locale), description: Some(format!("{} {}", listed.len(), chrome("checks", "Nachweise", locale))), ..AriaNode::default() };
    let rows = listed.iter().enumerate().map(|(position, index)| PublishedNode {
        key: format!("check:{}", report.checks[*index].id),
        node: AriaNode { selected: Some(state.selected == Some(*index)), ..check_row_node(&check_row_facts(&report.checks[*index], locale), lang, position + 1, listed.len()) },
        depth: 1,
    });
    std::iter::once(PublishedNode { key: "checks:table".to_owned(), node: table, depth: 0 }).chain(rows).collect()
}

/// 🏷️ Accessible name of a check row: the name of its screen-reader row node followed by the clause it describes.
pub fn check_accessible_name(check: &CheckResult, locale: Locale) -> String {
    let node = crate::accessibility::screen_reader::check_row_node(&check_row_facts(check, locale), crate::viewport3d::config::Lang::of(locale), 1, 1);
    match node.description {
        Some(clause) => format!("{}, {clause}", node.name),
        None => node.name,
    }
}

fn check_row_label(check: &CheckResult, locale: Locale) -> String {
    let protocol = protocol_locale(locale);
    let band = UtilisationBand::of(check.status, check.utilization);
    let percent = if band == UtilisationBand::NotApplicable { "—".to_string() } else { utilisation_percent_text(check.utilization, locale) };
    format!("{} {:>6} {}  {}", band.glyph(), percent, utilisation_bar_text(check.status, check.utilization), check.title.resolve(&protocol))
}

fn check_row_description(check: &CheckResult, locale: Locale) -> String {
    let protocol = protocol_locale(locale);
    format!("{} · {} · {} / {}", check.subject.label.resolve(&protocol), clause_text(&check.clause, locale), quantity_text(&check.computed, locale), quantity_text(&check.limit, locale))
}
//#endregion 🔖️Text

//#region 🔖️Rows
fn row_error(code: &'static str) -> PluginAssemblyError {
    ui_error(code)
}

/// 🧾️ Detail rows of one check shared by the window's expanded rows and the inspection panel: explanation, quantities, utilisation, clause and remedy chips.
pub fn check_detail_rows(id: &str, check: &CheckResult, locale: Locale, spec: &ChecksWindowSpec) -> UiAssemblyResult<Vec<BuiltNode>> {
    let protocol = protocol_locale(locale);
    let mut rows = vec![
        tree_item_desc(format!("{id}.status"), norm_ui_label(chrome("Status", "Status", locale))?, Some(status_glyph_text(check.status, check.utilization, locale)))?,
        tree_item_desc(format!("{id}.explanation"), norm_ui_label(chrome("Explanation", "Erläuterung", locale))?, Some(check.explanation.resolve(&protocol).to_string()))?,
        tree_item_desc(format!("{id}.computed"), norm_ui_label(chrome("Computed", "Berechnet", locale))?, Some(quantity_text(&check.computed, locale)))?,
        tree_item_desc(format!("{id}.limit"), norm_ui_label(chrome("Limit", "Grenzwert", locale))?, Some(quantity_text(&check.limit, locale)))?,
        tree_item_desc(format!("{id}.utilization"), norm_ui_label(chrome("Utilization", "Ausnutzung", locale))?, Some(format!("{} {}", utilisation_percent_text(check.utilization, locale), utilisation_bar_text(check.status, check.utilization))))?,
    ];
    if spec.actions.clause_links {
        rows.push(clause_link_row(format!("{id}.clause"), &check.clause, locale, spec.controller_id)?);
    } else {
        rows.push(tree_item_desc(format!("{id}.clause"), norm_ui_label(chrome("Clause", "Klausel", locale))?, Some(clause_text(&check.clause, locale)))?);
    }
    rows.extend(remedy_chip_rows(id, check, locale, spec.controller_id.filter(|controller| !controller.ends_with("#viewer")))?);
    Ok(rows)
}

fn check_row(check: &CheckResult, index: usize, state: &ChecksViewState, locale: Locale, spec: &ChecksWindowSpec) -> UiAssemblyResult<BuiltNode> {
    let selection_only = spec.controller_id.is_some_and(|controller| controller.ends_with("#viewer")) && spec.interaction_domain.is_some();
    let id = if selection_only { crate::check_address(&check.id).ok_or_else(|| row_error("ui.node.id"))? } else { format!("norm-checks-check-{index}") };
    let band = UtilisationBand::of(check.status, check.utilization);
    let mut builder = ui::tree_item(norm_ui_label(check_row_label(check, locale))?)
        .try_id(&id)
        .map_err(|_| row_error("ui.node.id"))?
        .description(ui_text(check_row_description(check, locale))?)
        .tone(band.tone())
        .selected(state.selected == Some(index))
        .dimmed(state.pending.is_some())
        .try_label(check_accessible_name(check, locale))
        .map_err(|_| row_error("ui.node.label"))?;
    if selection_only {
        builder = builder.granularity(ui_text(crate::viewport3d::picking::GRANULARITY_CHECK)?);
    } else if let Some(controller_id) = spec.controller_id {
        let args = action_args_map(vec![("index", UiValue::Number(index as f64))])?;
        builder = builder.target(semio_framework_plugin::activation_target(norm_action(controller_id, "setSelectedCheckIndex", Some(args))?)?);
    }
    if matches!(check.status, CheckStatus::Fail | CheckStatus::Warning) {
        let mut children: UiFixedList<_> = UiFixedList::default();
        for row in check_detail_rows(&id, check, locale, spec)? {
            children.try_push(row).map_err(|_| row_error("ui.fixed-capacity"))?;
        }
        builder = builder.default_open(true).try_children(children).map_err(|_| row_error("ui.node.children"))?;
    }
    builder.try_build().map_err(|_| row_error("ui.node.build"))
}

fn group_row(group: &ChecksGroup, group_index: usize, state: &ChecksViewState, locale: Locale, spec: &ChecksWindowSpec) -> UiAssemblyResult<BuiltNode> {
    let worst = if group.counts[3] as usize == group.rows.len() { chrome("—", "—", locale) } else { utilisation_percent_text(group.worst, locale) };
    let verdict = if group.counts[2] > 0 { '✗' } else if group.counts[1] > 0 { '⚠' } else { '✓' };
    let label = format!("{verdict} {} — {} {} · u {worst}", group.label, group.rows.len(), chrome("checks", "Nachweise", locale));
    let description = format!("✓{} ⚠{} ✗{} ∅{}", group.counts[0], group.counts[1], group.counts[2], group.counts[3]);
    let selectable = spec.interaction_domain.is_some() && spec.controller_id.is_some_and(|controller| !controller.ends_with("#viewer")) && state.grouping == ChecksGrouping::Entity;
    let id = match (&group.entity_id, selectable) {
        (Some(entity_id), true) => entity_id.clone(),
        _ => format!("norm-checks-group-{group_index}"),
    };
    let mut builder = ui::tree_item(norm_ui_label(label)?).try_id(&id).map_err(|_| row_error("ui.node.id"))?.description(ui_text(description)?);
    if selectable && group.entity_id.is_some() {
        builder = builder.granularity(ui_text("entity")?);
    }
    builder.try_build().map_err(|_| row_error("ui.node.build"))
}

enum ChecksListRow<'a> {
    Group { index: usize, group: &'a ChecksGroup },
    Check { index: usize },
}

fn summary_rows(report: &CheckReport, state: &ChecksViewState, locale: Locale, spec: &ChecksWindowSpec) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let summary = &report.summary;
    let verdict = if summary.complies { chrome("✓ Complies", "✓ Erfüllt", locale) } else { chrome("✗ Does not comply", "✗ Nicht erfüllt", locale) };
    let description = format!("✓{} ⚠{} ✗{} ∅{} · u {}", summary.pass, summary.warning, summary.fail, summary.not_applicable, utilisation_percent_text(summary.worst_utilization, locale));
    let mut rows = vec![tree_item_desc("norm-checks.summary.verdict", norm_ui_label(verdict)?, Some(description))?];
    for (part_index, part) in summary.parts.iter().enumerate() {
        let state_glyph = if part.complies { '✓' } else { '✗' };
        rows.push(tree_item_desc(
            format!("norm-checks.summary.part-{part_index}"),
            norm_ui_label(format!("{state_glyph} {}", part.part))?,
            Some(format!("✓{} ⚠{} ✗{} ∅{} · u {}", part.pass, part.warning, part.fail, part.not_applicable, utilisation_percent_text(part.worst_utilization, locale))),
        )?);
    }
    if let Some(pending) = state.pending {
        rows.push(tree_item_desc("norm-checks.summary.pending", norm_ui_label(chrome("Evaluating…", "Berechnung läuft…", locale))?, Some(format!("{} / {}", pending.completed, pending.total)))?);
        if let (Some(controller_id), true) = (spec.controller_id, spec.actions.cancel_evaluation) {
            rows.push(tree_item_with_action("norm-checks.summary.cancel", norm_ui_label(chrome("Cancel evaluation", "Berechnung abbrechen", locale))?, None, norm_action(controller_id, "cancelEvaluation", None)?)?);
        }
    }
    if let (Some(controller_id), true) = (spec.controller_id, spec.actions.view_controls) {
        rows.extend(view_control_rows(state, locale, controller_id)?);
    }
    ui_node_list(rows.into_iter().map(Ok))
}

fn view_control_rows(state: &ChecksViewState, locale: Locale, controller_id: &'static str) -> UiAssemblyResult<Vec<BuiltNode>> {
    let mark = |on: bool| if on { "●" } else { "○" };
    let mut rows = Vec::new();
    let mut push = |id: String, label: String, next: ChecksViewState| -> UiAssemblyResult<()> {
        let args = action_args_map(vec![("view", ui_value_text(next.encode())?)])?;
        rows.push(tree_item_with_action(id, norm_ui_label(label)?, None, norm_action(controller_id, "setChecksView", Some(args))?)?);
        Ok(())
    };
    for (name_en, name_de, grouping) in [("entity", "Gegenstand", ChecksGrouping::Entity), ("clause", "Klausel", ChecksGrouping::Clause), ("part", "Teil", ChecksGrouping::Part)] {
        push(format!("norm-checks.control.group-{name_en}"), format!("{} {}: {}", mark(state.grouping == grouping), chrome("Group by", "Gruppieren nach", locale), chrome(name_en, name_de, locale)), ChecksViewState { grouping, ..state.clone() })?;
    }
    for (name_en, name_de, sort) in [("worst first", "Schlechteste zuerst", ChecksSort::WorstFirst), ("report order", "Berichtsreihenfolge", ChecksSort::ReportOrder), ("clause", "Klausel", ChecksSort::ClauseOrder), ("title", "Titel", ChecksSort::Title)] {
        push(format!("norm-checks.control.sort-{}", name_en.replace(' ', "-")), format!("{} {}: {}", mark(state.sort == sort), chrome("Sort", "Sortieren", locale), chrome(name_en, name_de, locale)), ChecksViewState { sort, ..state.clone() })?;
    }
    let toggles: [(&str, &str, &str, bool); 4] = [("pass", "Bestanden", "pass", state.filter.pass), ("warning", "Warnung", "warning", state.filter.warning), ("fail", "Nicht bestanden", "fail", state.filter.fail), ("not applicable", "Nicht anwendbar", "na", state.filter.not_applicable)];
    for (slot, (name_en, name_de, id, on)) in toggles.into_iter().enumerate() {
        let mut next = state.clone();
        match slot {
            0 => next.filter.pass = !on,
            1 => next.filter.warning = !on,
            2 => next.filter.fail = !on,
            _ => next.filter.not_applicable = !on,
        }
        push(format!("norm-checks.control.show-{id}"), format!("{} {}: {}", mark(on), chrome("Show", "Anzeigen", locale), chrome(name_en, name_de, locale)), next)?;
    }
    Ok(rows)
}

/// ✅️ Renders the checks window with default view state; the minimal replacement of the retired report tree.
pub fn render_checks(report: &CheckReport, windows: &TreeWindows<'_>, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<BuiltNode> {
    render_checks_with(report, &ChecksViewState::default(), &ChecksWindowSpec { controller_id, ..ChecksWindowSpec::default() }, windows, locale)
}

/// ✅️ Renders the checks window: summary with the determinate worst-utilisation bar, then the grouped, sorted, filtered and windowed list.
pub fn render_checks_with(report: &CheckReport, state: &ChecksViewState, spec: &ChecksWindowSpec, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    publish_checks_access(spec.interaction_domain.unwrap_or_default(), check_window_nodes(report, state, locale));
    let groups = plan_checks(report, state, locale);
    let mut rows = Vec::new();
    for (index, group) in groups.iter().enumerate() {
        rows.push(ChecksListRow::Group { index, group });
        rows.extend(group.rows.iter().map(|check_index| ChecksListRow::Check { index: *check_index }));
    }
    let mut tree = PanelTreeBuilder::new(NORM_CHECKS_TREE_ID)?.section(
        NORM_CHECKS_SUMMARY_ID,
        Some(norm_ui_label(chrome("Summary", "Zusammenfassung", locale))?),
        true,
        summary_rows(report, state, locale, spec)?,
    )?;
    if let (Some(controller_id), Some(domain)) = (spec.controller_id, spec.interaction_domain) {
        tree = tree.interaction_domain(controller_id, domain.to_string())?;
    }
    let tree = tree
        .window_section_or_placeholder(
            windows,
            NORM_CHECKS_SECTION_ID,
            Some(norm_ui_label(chrome("Checks", "Nachweise", locale))?),
            true,
            &rows,
            |row| match row {
                ChecksListRow::Group { index, group } => group_row(group, *index, state, locale, spec),
                ChecksListRow::Check { index } => check_row(&report.checks[*index], *index, state, locale, spec),
            },
            norm_ui_label(if report.checks.is_empty() { chrome("No checks computed.", "Keine Nachweise berechnet.", locale) } else { chrome("No checks match the filter.", "Keine Nachweise passen zum Filter.", locale) })?,
        )?
        .build()?;
    if report.checks.is_empty() {
        return Ok(tree);
    }
    let overall = if report.summary.complies { CheckStatus::Pass } else { CheckStatus::Fail };
    ui::column().try_children([utilisation_progress_node(overall, report.summary.worst_utilization, locale)?, tree]).map_err(|_| row_error("ui.node.children"))?.try_build().map_err(|_| row_error("ui.node.build"))
}
//#endregion 🔖️Rows
