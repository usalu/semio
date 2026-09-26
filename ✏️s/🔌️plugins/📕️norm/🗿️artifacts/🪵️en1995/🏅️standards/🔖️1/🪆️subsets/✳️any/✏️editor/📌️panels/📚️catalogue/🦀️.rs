//! 📚️ EN 1995 play app panel — examples plus EN 338 / EN 14080 strength classes, k_mod, fastener types and member roles.

use semio_framework_plugin::{LocalizedLabel, Locale, PanelGroup, PanelTabDefinition, Terminology, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

use crate::app_surface::{CatalogueCell, CatalogueColumn, CatalogueRow, CatalogueTable, NormFieldChoice};
use crate::artifact_schema::{k_mod, parse_service_class, properties_for_class, spacing_minima, strength_class_options, LoadDuration};
use crate::document::ClauseId;
use crate::{TimberConnection, TimberProduct};

//#region 🔖️Constants
pub const BODY_CATALOGUE: &str = "norm.en1995.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    crate::app_surface::panel_definition(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
}
//#endregion 🔖️Definition

//#region 📚️Tables
fn pick(locale: Locale, en: &str, de: &str) -> String {
    LocalizedLabel::native(en, de).resolve(Terminology::Native, locale).to_string()
}

fn choices(path: &str) -> &'static [NormFieldChoice] {
    crate::field_meta::en1995_field_meta(path).and_then(|meta| meta.choices).unwrap_or(&[])
}

fn choice_label(path: &str, value: &str, locale: Locale) -> String {
    choices(path).iter().find(|c| c.value == value).map(|c| pick(locale, c.label_en, c.label_de)).unwrap_or_else(|| value.to_string())
}

fn product_label(product: TimberProduct, locale: Locale) -> String {
    match product {
        TimberProduct::Solid => pick(locale, "Solid softwood (EN 338)", "Nadelvollholz (EN 338)"),
        TimberProduct::Glulam => pick(locale, "Glulam (EN 14080)", "Brettschichtholz (EN 14080)"),
        TimberProduct::Lvl => pick(locale, "LVL (EN 14374)", "Furnierschichtholz (EN 14374)"),
        TimberProduct::Clt => pick(locale, "CLT (EN 16351)", "Brettsperrholz (EN 16351)"),
    }
}

fn column(id: &'static str, label_en: &'static str, label_de: &'static str, unit: Option<&'static str>) -> CatalogueColumn {
    CatalogueColumn { id, label_en, label_de, unit }
}

/// 🪵️ EN 338 / EN 14080 characteristic values for every class the evaluator tabulates.
fn strength_class_table(locale: Locale) -> CatalogueTable {
    let rows = strength_class_options()
        .iter()
        .filter_map(|class| properties_for_class(class).map(|p| (class, p)))
        .map(|(class, p)| CatalogueRow {
            id: class.to_ascii_lowercase(),
            cells: vec![
                CatalogueCell::text(*class),
                CatalogueCell::text(product_label(p.product, locale)),
                CatalogueCell::number(p.f_m_k / 1e6, 1),
                CatalogueCell::number(p.f_t_0_k / 1e6, 1),
                CatalogueCell::number(p.f_c_0_k / 1e6, 1),
                CatalogueCell::number(p.f_c_90_k / 1e6, 1),
                CatalogueCell::number(p.f_v_k / 1e6, 1),
                CatalogueCell::number(p.e_0_mean / 1e9, 1),
                CatalogueCell::number(p.rho_k, 0),
            ],
        })
        .collect();
    CatalogueTable {
        id: "strength-classes",
        title_en: "Strength classes — EN 338 / EN 14080",
        title_de: "Festigkeitsklassen — EN 338 / EN 14080",
        clause: ClauseId::new("EN 1995-1-1", "§3.2", "3.2"),
        columns: vec![
            column("class", "Class", "Klasse", None),
            column("product", "Product", "Produkt", None),
            column("fmk", "f_m,k", "f_m,k", Some("MPa")),
            column("ft0k", "f_t,0,k", "f_t,0,k", Some("MPa")),
            column("fc0k", "f_c,0,k", "f_c,0,k", Some("MPa")),
            column("fc90k", "f_c,90,k", "f_c,90,k", Some("MPa")),
            column("fvk", "f_v,k", "f_v,k", Some("MPa")),
            column("e0mean", "E_0,mean", "E_0,mean", Some("GPa")),
            column("rhok", "ρ_k", "ρ_k", Some("kg/m³")),
        ],
        rows,
    }
}

/// ⏱️ EN 1995-1-1 Table 3.1 k_mod per service class and load-duration class.
fn k_mod_table(locale: Locale) -> CatalogueTable {
    let durations = [LoadDuration::Permanent, LoadDuration::Long, LoadDuration::Medium, LoadDuration::Short, LoadDuration::Instantaneous];
    let rows = choices("members[].serviceClass")
        .iter()
        .map(|sc| {
            let service = parse_service_class(sc.value.parse().unwrap_or(1));
            let mut cells = vec![CatalogueCell::text(pick(locale, sc.label_en, sc.label_de))];
            cells.extend(durations.iter().map(|d| CatalogueCell::number(k_mod(service, *d), 2)));
            CatalogueRow { id: format!("sc{}", sc.value), cells }
        })
        .collect();
    let mut columns = vec![column("service-class", "Service class", "Nutzungsklasse", None)];
    columns.extend(choices("members[].actions[].loadDuration").iter().map(|d| column(d.value, d.label_en, d.label_de, None)));
    CatalogueTable {
        id: "k-mod",
        title_en: "Modification factor k_mod",
        title_de: "Modifikationsbeiwert k_mod",
        clause: ClauseId::new("EN 1995-1-1", "§3.1.3", "Table 3.1"),
        columns,
        rows,
    }
}

/// 🔩️ Dowel-type fastener types with their embedment rule, rope effect and Table 8.2–8.5 minimum spacings in multiples of d.
fn fastener_table(locale: Locale) -> CatalogueTable {
    let rows = choices("connections[].fastenerType")
        .iter()
        .map(|f| {
            let probe = TimberConnection {
                id: String::new(),
                label_en: String::new(),
                label_de: String::new(),
                fastener_type: f.value.to_string(),
                strength_class: String::new(),
                service_class: 1,
                diameter_m: 1.0,
                number: 1,
                rows: 1,
                spacing_m: 1.0,
                edge_distance_m: 1.0,
                end_distance_m: 1.0,
                t1_m: 0.0,
                t2_m: 0.0,
                steel_plate: false,
                steel_plate_thickness_m: 0.0,
                shear_planes: 1,
                f_u_k: 0.0,
                actions: Vec::new(),
            };
            let (a1, a3, a4) = spacing_minima(&probe);
            let embedment = if f.value == "nail" { "0.082·ρ_k·d^-0.3 (d < 8 mm)" } else { "0.082·(1 − 0.01·d)·ρ_k" };
            let rope = if f.value == "screw" { pick(locale, "yes — F_ax,Rk/4", "ja — F_ax,Rk/4") } else { pick(locale, "no", "nein") };
            CatalogueRow {
                id: f.value.to_string(),
                cells: vec![
                    CatalogueCell::text(pick(locale, f.label_en, f.label_de)),
                    CatalogueCell::text(embedment),
                    CatalogueCell::text(rope),
                    CatalogueCell::number(a1, 0),
                    CatalogueCell::number(a3, 0),
                    CatalogueCell::number(a4, 0),
                ],
            }
        })
        .collect();
    CatalogueTable {
        id: "fastener-types",
        title_en: "Dowel-type fasteners",
        title_de: "Stiftförmige Verbindungsmittel",
        clause: ClauseId::new("EN 1995-1-1", "§8", "8.3–8.6"),
        columns: vec![
            column("fastener", "Fastener", "Verbindungsmittel", None),
            column("fhk", "f_h,k", "f_h,k", Some("MPa")),
            column("rope", "Rope effect", "Einhängeeffekt", None),
            column("a1", "a₁,min", "a₁,min", Some("d")),
            column("a3t", "a₃,t,min", "a₃,t,min", Some("d")),
            column("a4t", "a₄,t,min", "a₄,t,min", Some("d")),
        ],
        rows,
    }
}

/// 🎯️ Member roles and the clauses each one is verified against.
fn role_table(locale: Locale) -> CatalogueTable {
    let clauses = |role: &str| match role {
        "column" => ("EN 1995-1-1 §6.1.6, §6.1.7, §6.3.2, §6.2.4, §6.1.5, §7.2", "EN 1995-1-1 §6.1.6, §6.1.7, §6.3.2, §6.2.4, §6.1.5, §7.2"),
        "floor" => ("EN 1995-1-1 §6.1.6, §6.1.7, §6.1.5, §7.2, §7.3", "EN 1995-1-1 §6.1.6, §6.1.7, §6.1.5, §7.2, §7.3"),
        "bridge" => ("EN 1995-2 Annex A (fatigue), Annex B (a_vert, a_hor), §5 ULS, §7 SLS", "EN 1995-2 Anhang A (Ermüdung), Anhang B (a_vert, a_hor), §5 GZT, §7 GZG"),
        _ => ("EN 1995-1-1 §6.1.2, §6.1.6, §6.1.7, §6.1.5, §7.2", "EN 1995-1-1 §6.1.2, §6.1.6, §6.1.7, §6.1.5, §7.2"),
    };
    let rows = choices("members[].role")
        .iter()
        .map(|r| {
            let (en, de) = clauses(r.value);
            CatalogueRow {
                id: r.value.to_string(),
                cells: vec![
                    CatalogueCell::text(choice_label("members[].role", r.value, locale)),
                    CatalogueCell::text(pick(locale, en, de)),
                    CatalogueCell::text(pick(locale, "EN 1995-1-2 §4.2 when fire duration > 0", "EN 1995-1-2 §4.2 bei Branddauer > 0")),
                ],
            }
        })
        .collect();
    CatalogueTable {
        id: "member-roles",
        title_en: "Member roles",
        title_de: "Bauteilrollen",
        clause: ClauseId::new("EN 1995-1-1", "§6", "6–7"),
        columns: vec![column("role", "Role", "Rolle", None), column("checks", "Checks", "Nachweise", None), column("fire", "Fire", "Brand", None)],
        rows,
    }
}

/// 📚️ Normative reference tables for the timber subject, localized to `locale`.
pub fn reference_tables(locale: Locale) -> Vec<CatalogueTable> {
    vec![strength_class_table(locale), k_mod_table(locale), fastener_table(locale), role_table(locale)]
}
//#endregion 📚️Tables

//#region 🔖️Render
pub fn render(examples: Vec<semio_framework_plugin::ExampleSource>, locale: Locale, controller_id: &'static str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_catalogue(&examples, &reference_tables(locale), locale, controller_id, &semio_framework_plugin::TreeWindows::unhosted())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
