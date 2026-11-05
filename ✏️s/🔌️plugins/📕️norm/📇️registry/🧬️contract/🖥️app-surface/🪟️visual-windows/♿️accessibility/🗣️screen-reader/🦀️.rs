//! 🗣️ Screen-reader contract: accessible roles, names, values and set positions for viewport entities, handles, check rows and diagrams (through their table twins), in English and German.
//!
//! 🔗️ Roles and required attributes follow WAI-ARIA 1.2; the scene list is the keyboard-accessible model of the 3D viewport, see `visual-editor-architecture.md` §8.2.

use crate::diagrams2d::{accessible_description, table_twin};
use crate::viewport3d::config::Lang;
use crate::viewport3d::description::{entity_spoken_name, pick, spoken_utilisation};
use crate::viewport3d::overlays::number;
use semio_ui_locale::Locale;
use semio_s_plugin_norm_structural_scene::{CheckStatus, Diagram2d, Handle, LocalizedCopy, SceneEntity};

/// 🎚️ Current, minimum and maximum of a range widget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AriaRange {
    pub now: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// 🏷️ The accessibility properties one widget exposes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AriaNode {
    pub role: &'static str,
    pub name: String,
    pub role_description: Option<String>,
    pub description: Option<String>,
    pub value_text: Option<String>,
    pub range: Option<AriaRange>,
    pub position: Option<(usize, usize)>,
    pub selected: Option<bool>,
    pub described_by: Option<String>,
}

/// ⚖️ Attributes a role requires (WAI-ARIA 1.2, "Required States and Properties" of the roles used here).
pub fn required_attributes(role: &str) -> &'static [&'static str] {
    match role {
        "slider" => &["aria-valuenow"],
        "option" => &["aria-selected"],
        _ => &[],
    }
}

/// 📋️ The roles this contract emits, all defined by WAI-ARIA 1.2.
pub const ROLES: [&str; 9] = ["application", "listbox", "option", "slider", "row", "img", "table", "menu", "menuitem"];

impl AriaNode {
    fn new(role: &'static str, name: String) -> Self {
        Self { role, name, ..Self::default() }
    }

    /// 🧾️ The `role` and `aria-*` attributes as the host writes them.
    pub fn attributes(&self) -> Vec<(&'static str, String)> {
        let mut out = vec![("role", self.role.to_owned()), ("aria-label", self.name.clone())];
        let mut put = |key: &'static str, value: Option<String>| if let Some(v) = value { out.push((key, v)) };
        put("aria-roledescription", self.role_description.clone());
        put("aria-description", self.description.clone());
        put("aria-valuetext", self.value_text.clone());
        put("aria-valuenow", self.range.map(|r| r.now.to_string()));
        put("aria-valuemin", self.range.and_then(|r| r.min).map(|v| v.to_string()));
        put("aria-valuemax", self.range.and_then(|r| r.max).map(|v| v.to_string()));
        put("aria-posinset", self.position.map(|(p, _)| p.to_string()));
        put("aria-setsize", self.position.map(|(_, n)| n.to_string()));
        put("aria-selected", self.selected.map(|s| s.to_string()));
        put("aria-describedby", self.described_by.clone());
        out
    }

    /// ⚖️ Required attributes of the role that are missing.
    pub fn missing_required(&self) -> Vec<&'static str> {
        let present = self.attributes();
        required_attributes(self.role).iter().copied().filter(|r| !present.iter().any(|(k, _)| k == r)).collect()
    }
}

fn is_german(lang: Lang) -> bool {
    matches!(lang, Lang::De)
}

fn t(lang: Lang, en: &str, de: &str) -> String {
    if is_german(lang) { de.to_owned() } else { en.to_owned() }
}

/// 🧊️ The 3D viewport: an application region described by the scene summary, with its keyboard model in the scene list.
pub fn viewport_node(summary: &str, lang: Lang) -> AriaNode {
    AriaNode { role_description: Some(t(lang, "3D model", "3D-Modell")), description: Some(summary.to_owned()), ..AriaNode::new("application", t(lang, "3D model of the structure", "3D-Modell des Tragwerks")) }
}

/// 📋️ The scene as a list, the pointer-free twin of the viewport.
pub fn scene_list_node(lang: Lang) -> AriaNode {
    AriaNode::new("listbox", t(lang, "Scene as list", "Szene als Liste"))
}

/// 🧱️ A viewport entity as an option of the scene list: name, spoken utilisation and status, detail text and position in the set.
pub fn entity_node(entity: &SceneEntity, lang: Lang, position: usize, size: usize, selected: bool) -> AriaNode {
    let spoken = entity_spoken_name(entity, lang);
    let detail = pick(&entity.accessible_detail, lang);
    AriaNode { description: (!detail.is_empty()).then(|| detail.to_owned()), position: Some((position, size)), selected: Some(selected), ..AriaNode::new("option", spoken) }
}

/// 🏷️ The status word source of a utilisation: the governing check of the entity a grip or marker is bound to, as `(ratio, status)`.
pub type BoundUtilisation = (f64, CheckStatus);

/// 🔢️ The bound field value shown for the drag parameter `s` as text with its unit (or the label of the chosen token), in the language.
pub fn handle_value_text(handle: &Handle, s: f64, lang: Lang) -> String {
    let first = handle.writes.first();
    match first.and_then(|w| w.quantise.as_ref()?.token_at(w.map.apply(s))) {
        Some(token) => pick(&token.label, lang).to_owned(),
        None => format!("{} {}", number(first.map_or(s, |w| w.value_at(s)), is_german(lang), 3), first.map_or("", |w| w.unit.as_str())).trim_end().to_owned(),
    }
}

/// 🕹️ A handle as a slider: its name, the current field value with unit and the live utilisation of the bound check as text, the field range of its limits as range, and how to nudge, type and cancel.
pub fn handle_node(handle: &Handle, value: Option<f64>, utilisation: Option<BoundUtilisation>, lang: Lang) -> AriaNode {
    let name = pick(&handle.accessible_name, lang).to_owned();
    let field = |s: f64| handle.writes.first().map_or(s, |w| w.value_at(s));
    let bounds = handle.limits.as_ref().map(|l| (field(l.min).min(field(l.max)), field(l.min).max(field(l.max))));
    let text = value.map(|v| match utilisation {
        Some((ratio, status)) => format!("{}, {}", handle_value_text(handle, v, lang), spoken_utilisation(ratio, status, lang)),
        None => handle_value_text(handle, v, lang),
    });
    AriaNode {
        role_description: Some(t(lang, "handle", "Griff")),
        description: Some(t(lang, "Arrow keys nudge by the field step, Shift for a large step, Alt for a fine step, Enter opens a value field and applies it, Escape cancels, Control plus arrow key switches to another handle of the same entity", "Pfeiltasten verschieben um den Feldschritt, Umschalt für einen großen Schritt, Alt für einen feinen Schritt, Eingabe öffnet ein Wertfeld und übernimmt es, Escape bricht ab, Strg plus Pfeiltaste wechselt zu einem anderen Griff desselben Bauteils")),
        value_text: text,
        range: value.map(|now| AriaRange { now: field(now), min: bounds.map(|b| b.0), max: bounds.map(|b| b.1) }),
        ..AriaNode::new("slider", name)
    }
}

/// ✅️ What a check row says besides its title; `quantities` is the computed value against its limit as text, empty when there is none.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckRowFacts {
    pub title: LocalizedCopy,
    pub subject: LocalizedCopy,
    pub status: CheckStatus,
    pub utilisation: f64,
    pub quantities: String,
    pub clause: String,
}

/// ✅️ A check row: title, subject, status, utilisation and quantities spoken in one name, the clause as description, position in the list.
pub fn check_row_node(row: &CheckRowFacts, lang: Lang, position: usize, size: usize) -> AriaNode {
    let mut name = format!("{}, {}, {}", pick(&row.title, lang), pick(&row.subject, lang), spoken_utilisation(row.utilisation, row.status, lang));
    if !row.quantities.is_empty() {
        name = format!("{name}, {}", row.quantities);
    }
    AriaNode { description: (!row.clause.is_empty()).then(|| row.clause.clone()), position: Some((position, size)), ..AriaNode::new("row", name) }
}

/// 📈️ A diagram as an image described by its text and by its table twin (`table_id`), and that table as a data table of its own.
pub fn diagram_nodes(diagram: &Diagram2d, lang: Lang, table_id: &str) -> (AriaNode, AriaNode) {
    let locale = if is_german(lang) { Locale::De } else { Locale::En };
    let twin = table_twin(diagram, locale);
    let image = AriaNode { description: Some(accessible_description(diagram, locale)), described_by: Some(table_id.to_owned()), ..AriaNode::new("img", pick(&diagram.title, lang).to_owned()) };
    let rows = twin.rows.len() + twin.marker_rows.len();
    let table = AriaNode { description: Some(format!("{rows} {}, {} {}", t(lang, "rows", "Zeilen"), twin.headers.len(), t(lang, "columns", "Spalten"))), ..AriaNode::new("table", twin.caption) };
    (image, table)
}
