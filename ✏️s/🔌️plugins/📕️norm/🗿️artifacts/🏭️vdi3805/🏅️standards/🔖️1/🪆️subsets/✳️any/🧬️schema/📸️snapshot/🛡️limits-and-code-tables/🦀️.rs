//! 🛡️ Limits and code tables of the VDI 3805 snapshot: the security limits with their defaults and the connection, pressure, filter and type code tables with the sheet numeric bounds. See [`super`].

use semio_s_plugin_norm_compliance::NormError;

/// 🔒️ Security limits for untrusted manufacturer files.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SecurityLimits {
    pub max_file_bytes: usize,
    pub max_records: usize,
    pub max_field_length: usize,
    pub max_nesting_depth: usize,
}

impl Default for SecurityLimits {
    fn default() -> Self {
        Self { max_file_bytes: 16 * 1024 * 1024, max_records: 100_000, max_field_length: 8_192, max_nesting_depth: 32 }
    }
}

impl SecurityLimits {
    pub fn validate_text(&self, text: &str) -> Result<(), NormError> {
        if text.len() > self.max_file_bytes {
            return Err(NormError::InvalidValue { field: "file".into(), reason: format!("exceeds {} bytes", self.max_file_bytes) });
        }
        Ok(())
    }
}

/// 🏷️ Connection-type codes shared by catalogue tables, field-meta, and evaluate (VDI 3805-1 §6.2).
pub const CONNECTION_TYPE_CODES: &[&str] = &["flange", "thread", "weld", "press"];

/// 🏷️ Connection-type rows: code, en, de.
pub const CONNECTION_TYPE_ROWS: &[(&str, &str, &str)] = &[
    ("flange", "Flanged", "Flansch"),
    ("thread", "Threaded", "Gewinde"),
    ("weld", "Welded", "Schweißanschluss"),
    ("press", "Press fit", "Pressverbindung"),
];

/// 🏷️ PN pressure-class codes (DIN EN 1092 / VDI 3805 product sheets §4).
pub const PRESSURE_CLASS_CODES: &[&str] = &["PN6", "PN10", "PN16", "PN25", "PN40"];

/// 🏷️ PN rows with nominal bar pressure for catalogue cells.
pub const PRESSURE_CLASS_ROWS: &[(&str, f64)] = &[("PN6", 6.0), ("PN10", 10.0), ("PN16", 16.0), ("PN25", 25.0), ("PN40", 40.0)];

/// 🏷️ Filter classes Blatt 19 (ISO 16890 / EN 779 legacy — VDI 3805-19 Table 1).
pub const FILTER_CLASS_CODES: &[&str] = &["G4", "M5", "M6", "F7", "F8", "F9", "ePM1", "ePM2_5", "ePM10"];

/// 🏷️ Filter-class rows: code, en, de.
pub const FILTER_CLASS_ROWS: &[(&str, &str, &str)] = &[
    ("G4", "Coarse G4", "Grob G4"),
    ("M5", "Medium M5", "Mittel M5"),
    ("M6", "Medium M6", "Mittel M6"),
    ("F7", "Fine F7", "Fein F7"),
    ("F8", "Fine F8", "Fein F8"),
    ("F9", "Fine F9", "Fein F9"),
    ("ePM1", "ePM1 85%", "ePM1 85 %"),
    ("ePM2_5", "ePM2.5 70%", "ePM2,5 70 %"),
    ("ePM10", "ePM10 50%", "ePM10 50 %"),
];

/// 🏷️ Generic type_code tokens (VDI 3805 multi-profile sheets §4.1).
pub const TYPE_CODE_CODES: &[&str] = &["STD", "STD-A", "STD-B", "TYPE-A", "TYPE-B", "TYPE-C", "DEFAULT"];

/// 📏 Blatt-sourced numeric domains used by evaluate and catalogue (min, max).
pub const SHEET_NUMERIC_BOUNDS_19_AIRFLOW: (f64, f64) = (1e-5, 50.0);

pub const SHEET_NUMERIC_BOUNDS_16_PRESSURE_DROP_PA: (f64, f64) = (0.0, 5000.0);

pub const SHEET_NUMERIC_BOUNDS_53_COP: (f64, f64) = (1.0, 10.0);

pub const SHEET_NUMERIC_BOUNDS_DN: (f64, f64) = (6.0, 600.0);

pub const SHEET_NUMERIC_BOUNDS_DN_PIPE: (f64, f64) = (6.0, 1200.0);

pub const SHEET_NUMERIC_BOUNDS_OUTER_DIAMETER_M: (f64, f64) = (0.006, 2.0);

pub const SHEET_NUMERIC_BOUNDS_WALL_THICKNESS_M: (f64, f64) = (0.0005, 0.1);

pub const SHEET_NUMERIC_BOUNDS_VOLUME_M3: (f64, f64) = (0.001, 500.0);

pub const SHEET_NUMERIC_BOUNDS_AXIAL_FORCE_N: (f64, f64) = (0.0, 1.0e7);

pub const SHEET_NUMERIC_BOUNDS_53_HEAT_W: (f64, f64) = (100.0, 5.0e6);

pub const SHEET_NUMERIC_BOUNDS_53_DN: (f64, f64) = (6.0, 300.0);
