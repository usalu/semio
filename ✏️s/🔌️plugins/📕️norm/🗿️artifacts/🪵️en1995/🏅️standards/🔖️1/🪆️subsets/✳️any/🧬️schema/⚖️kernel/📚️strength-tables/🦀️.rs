//! 📚️ Strength-class, kmod, kdef and γM lookups of the EN 1995 timber checks: every number is read from the shared NDP registry (`ndp.en1995.strength.*`, `.kmod.*`, `.kdef.*`, `.gamma-m.*`); this module only keeps the class names and product kinds. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::{ndp_common, ndp_is_verified, ndp_number_opt, ndp_number_or_gap};

/// 🪵 One tabulated strength class in standard units (N/mm², kg/m³); `verified` is true when every property of the row is verified in the registry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrengthRow {
    pub name: &'static str,
    pub product: TimberProduct,
    pub hardwood: bool,
    pub f_m_k: f64,
    pub f_t_0_k: f64,
    pub f_t_90_k: f64,
    pub f_c_0_k: f64,
    pub f_c_90_k: f64,
    pub f_v_k: f64,
    pub e_0_mean: f64,
    pub e_0_05: f64,
    pub e_90_mean: f64,
    pub g_mean: f64,
    pub rho_k: f64,
    pub rho_mean: f64,
    pub source: &'static str,
    pub verified: bool,
}

/// 🏷️ Name, product kind, hardwood flag and source label of one strength class; the numbers live in the registry.
#[derive(Clone, Copy)]
struct ClassMeta {
    name: &'static str,
    product: TimberProduct,
    hardwood: bool,
    source: &'static str,
}

const fn class(name: &'static str, product: TimberProduct, hardwood: bool, source: &'static str) -> ClassMeta {
    ClassMeta { name, product, hardwood, source }
}

const SOLID: TimberProduct = TimberProduct::Solid;
const GLULAM: TimberProduct = TimberProduct::Glulam;
const EN338: &str = "EN 338:2016 Table 1";
const EN338_UNCONFIRMED: &str = "EN 338 Table 1 (unconfirmed: differs from the EN 338:2016 transcription of the oracle library in the fields listed in the worklist)";
const EN14080: &str = "EN 14080:2013 Table 5";

const CLASSES: &[ClassMeta] = &[
    class("C14", SOLID, false, EN338),
    class("C16", SOLID, false, EN338),
    class("C18", SOLID, false, EN338),
    class("C20", SOLID, false, EN338_UNCONFIRMED),
    class("C22", SOLID, false, EN338),
    class("C24", SOLID, false, EN338),
    class("C27", SOLID, false, EN338_UNCONFIRMED),
    class("C30", SOLID, false, EN338),
    class("C35", SOLID, false, EN338_UNCONFIRMED),
    class("C40", SOLID, false, EN338_UNCONFIRMED),
    class("C45", SOLID, false, EN338_UNCONFIRMED),
    class("C50", SOLID, false, EN338_UNCONFIRMED),
    class("D18", SOLID, true, EN338),
    class("D24", SOLID, true, EN338_UNCONFIRMED),
    class("D30", SOLID, true, EN338),
    class("D35", SOLID, true, EN338),
    class("D40", SOLID, true, EN338_UNCONFIRMED),
    class("D50", SOLID, true, EN338),
    class("D60", SOLID, true, EN338),
    class("D70", SOLID, true, EN338_UNCONFIRMED),
    class("GL20h", GLULAM, false, EN14080),
    class("GL22h", GLULAM, false, EN14080),
    class("GL24h", GLULAM, false, EN14080),
    class("GL26h", GLULAM, false, EN14080),
    class("GL28h", GLULAM, false, EN14080),
    class("GL30h", GLULAM, false, EN14080),
    class("GL32h", GLULAM, false, EN14080),
    class("GL20c", GLULAM, false, EN14080),
    class("GL22c", GLULAM, false, EN14080),
    class("GL24c", GLULAM, false, EN14080),
    class("GL26c", GLULAM, false, EN14080),
    class("GL28c", GLULAM, false, EN14080),
    class("GL30c", GLULAM, false, EN14080),
    class("GL32c", GLULAM, false, EN14080),
    class("LVL32", TimberProduct::Lvl, false, "EN 14374 / ETA"),
    class("CLT100", TimberProduct::Clt, false, "EN 16351 lamellae C24"),
];

const PROPERTIES: [&str; 12] = ["fmk", "ft0k", "ft90k", "fc0k", "fc90k", "fvk", "e0mean", "e005", "e90mean", "gmean", "rhok", "rhomean"];

fn normalise(class: &str) -> String {
    class.trim().to_ascii_uppercase().replace([' ', '_'], "")
}

/// 🔎 Row of a class by name, case-insensitive; `CLT` maps to `CLT100`, `LVL-32` to `LVL32`. The twelve values are read from the registry (`ndp.en1995.strength.<class>.<property>`), so the calling check cites them.
pub fn strength_row(class: &str) -> Option<StrengthRow> {
    let key = match normalise(class).as_str() {
        "CLT" | "CLTC24" | "CLT-C24" => "CLT100".to_string(),
        "LVL-32" => "LVL32".to_string(),
        other => other.to_string(),
    };
    let meta = CLASSES.iter().find(|c| normalise(c.name) == key)?;
    let id = |property: &str| format!("ndp.en1995.strength.{}.{property}", meta.name.to_ascii_lowercase());
    let v = |index: usize| ndp_common(&id(PROPERTIES[index]));
    Some(StrengthRow {
        name: meta.name,
        product: meta.product,
        hardwood: meta.hardwood,
        f_m_k: v(0),
        f_t_0_k: v(1),
        f_t_90_k: v(2),
        f_c_0_k: v(3),
        f_c_90_k: v(4),
        f_v_k: v(5),
        e_0_mean: v(6),
        e_0_05: v(7),
        e_90_mean: v(8),
        g_mean: v(9),
        rho_k: v(10),
        rho_mean: v(11),
        source: meta.source,
        verified: PROPERTIES.iter().all(|p| ndp_is_verified(&id(p))),
    })
}

/// 📋 SI conversion of a [`StrengthRow`] into the snapshot-facing [`TimberProperties`].
pub fn row_properties(r: &StrengthRow) -> TimberProperties {
    TimberProperties {
        f_m_k: r.f_m_k * 1e6,
        f_t_0_k: r.f_t_0_k * 1e6,
        f_t_90_k: r.f_t_90_k * 1e6,
        f_c_0_k: r.f_c_0_k * 1e6,
        f_c_90_k: r.f_c_90_k * 1e6,
        f_v_k: r.f_v_k * 1e6,
        e_0_mean: r.e_0_mean * 1e6,
        e_0_05: r.e_0_05 * 1e6,
        e_90_mean: r.e_90_mean * 1e6,
        g_mean: r.g_mean * 1e6,
        rho_k: r.rho_k,
        product: r.product,
    }
}

/// 🧾 Names of every tabulated class, used as the pick list of remedies and field meta.
pub fn strength_class_names() -> Vec<&'static str> {
    CLASSES.iter().map(|c| c.name).collect()
}

/// ⏱ Load-duration columns of Table 3.1 in order permanent, long, medium, short, instantaneous.
pub const DURATIONS: [LoadDuration; 5] = [LoadDuration::Permanent, LoadDuration::Long, LoadDuration::Medium, LoadDuration::Short, LoadDuration::Instantaneous];

/// 🔑 Registry keys of the load-duration columns in the order of [`DURATIONS`].
const DURATION_KEYS: [&str; 5] = ["permanent", "long", "medium", "short", "instant"];

/// 🔎 k_mod of EN 1995-1-1 Table 3.1 by material key, service class and duration, read from `ndp.en1995.kmod.<material>.sc<n>.<duration>`; `None` when the material is not allowed in the service class (registered as 0).
pub fn k_mod_for(material: &str, service_class: u8, duration: LoadDuration) -> Option<f64> {
    let column = DURATIONS.iter().position(|d| *d == duration)?;
    let class = match service_class {
        3 => 3,
        2 => 2,
        _ => 1,
    };
    ndp_number_opt(&format!("ndp.en1995.kmod.{material}.sc{class}.{}", DURATION_KEYS[column]), AnnexChoice::En).filter(|v| *v > 0.0)
}

/// 🔎 k_def of EN 1995-1-1 Table 3.2 by material key and service class, read from `ndp.en1995.kdef.<material>.sc<n>`; `None` when not allowed (registered as 0).
pub fn k_def_for(material: &str, service_class: u8) -> Option<f64> {
    let class = match service_class {
        3 => 3,
        2 => 2,
        _ => 1,
    };
    ndp_number_opt(&format!("ndp.en1995.kdef.{material}.sc{class}"), AnnexChoice::En).filter(|v| *v > 0.0)
}

/// 🔎 γ_M of EN 1995-1-1 Table 2.3 and DIN EN 1995-1-1/NA for a material key under the annex, read from the shared registry (`ndp.en1995.gamma-m.<material>`); accidental situations always yield 1.0; a material without a value logs an `NdpGap` so the check that turns the `None` into NaN is refused.
pub fn gamma_m_for(annex: AnnexChoice, material: &str, accidental: bool) -> Option<f64> {
    if accidental {
        return Some(1.0);
    }
    ndp_number_or_gap(&format!("ndp.en1995.gamma-m.{material}"), annex)
}

/// 🔑 Table key of a [`TimberProduct`] for the k_mod, k_def and γ_M tables.
pub fn product_key(product: TimberProduct) -> &'static str {
    match product {
        TimberProduct::Solid => "solid",
        TimberProduct::Glulam => "glulam",
        TimberProduct::Lvl => "lvl",
        TimberProduct::Clt => "clt",
    }
}
