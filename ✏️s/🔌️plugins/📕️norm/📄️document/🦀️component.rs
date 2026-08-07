//! 📏️ Norm core: shared quantities, clause identity, compliance results, and national annex selection.

use protocol::{OpBinary, OpText, Operation, OperationDiff, ProtocolError};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use store::{DocumentDsl, DocumentPack, TextError, TextSpan};

// #region 🔖️Quantity
/// 📐️ Physical quantity kind for SI-normalized norm computations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuantityKind {
    Dimensionless,
    Length,
    Area,
    Volume,
    Mass,
    Time,
    Temperature,
    Force,
    Pressure,
    Stress,
    Moment,
    Energy,
    Power,
    ThermalConductivity,
    ThermalResistance,
    HeatTransferCoefficient,
    AirPermeability,
    VentilationRate,
    Acceleration,
}

/// 📊️ A scalar value tagged with its physical quantity kind (SI units).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub kind: QuantityKind,
    pub value: f64,
}

impl Quantity {
    pub const fn new(kind: QuantityKind, value: f64) -> Self {
        Self { kind, value }
    }

    pub fn length_m(value: f64) -> Self {
        Self::new(QuantityKind::Length, value)
    }

    pub fn area_m2(value: f64) -> Self {
        Self::new(QuantityKind::Area, value)
    }

    pub fn force_kn(value: f64) -> Self {
        Self::new(QuantityKind::Force, value * 1_000.0)
    }

    pub fn stress_mpa(value: f64) -> Self {
        Self::new(QuantityKind::Stress, value * 1_000_000.0)
    }

    pub fn thermal_resistance_m2k_w(value: f64) -> Self {
        Self::new(QuantityKind::ThermalResistance, value)
    }

    pub fn u_value_w_m2k(value: f64) -> Self {
        Self::new(QuantityKind::HeatTransferCoefficient, value)
    }

    pub fn acceleration_m_s2(value: f64) -> Self {
        Self::new(QuantityKind::Acceleration, value)
    }
}
// #endregion 🔖️Quantity

// #region 🔖️Clause
/// 📑️ Stable clause identifier within a norm family (e.g. `EN 1992-1-1` §6.1).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClauseId {
    pub family: String,
    pub part: String,
    pub section: String,
}

impl ClauseId {
    pub fn new(family: impl Into<String>, part: impl Into<String>, section: impl Into<String>) -> Self {
        Self { family: family.into(), part: part.into(), section: section.into() }
    }
}

impl fmt::Display for ClauseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} §{}", self.family, self.part, self.section)
    }
}
// #endregion 🔖️Clause

// #region 🔖️Check
/// ✅️ Outcome of a single norm compliance check.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    Pass,
    Fail,
    NotApplicable,
}

/// 📋️ One computed check with clause traceability.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckResult {
    pub clause: ClauseId,
    pub status: CheckStatus,
    pub computed: Quantity,
    pub limit: Quantity,
    pub utilization: f64,
    pub message: String,
    pub annex: AnnexChoice,
}

impl CheckResult {
    pub fn pass(clause: ClauseId, computed: Quantity, limit: Quantity, utilization: f64, message: impl Into<String>, annex: AnnexChoice) -> Self {
        Self { clause, status: CheckStatus::Pass, computed, limit, utilization, message: message.into(), annex }
    }

    pub fn fail(clause: ClauseId, computed: Quantity, limit: Quantity, utilization: f64, message: impl Into<String>, annex: AnnexChoice) -> Self {
        Self { clause, status: CheckStatus::Fail, computed, limit, utilization, message: message.into(), annex }
    }

    pub fn from_utilization(clause: ClauseId, computed: Quantity, limit: Quantity, message: impl Into<String>, annex: AnnexChoice) -> Self {
        let utilization = if limit.value.abs() < f64::EPSILON { 0.0 } else { computed.value / limit.value };
        if utilization <= 1.0 {
            Self::pass(clause, computed, limit, utilization, message, annex)
        } else {
            Self::fail(clause, computed, limit, utilization, message, annex)
        }
    }

    pub fn from_minimum(clause: ClauseId, computed: Quantity, minimum: Quantity, message: impl Into<String>, annex: AnnexChoice) -> Self {
        let passes = computed.value >= minimum.value;
        let utilization = if passes { minimum.value / computed.value.max(minimum.value) } else { computed.value / minimum.value.max(f64::EPSILON) };
        if passes {
            Self::pass(clause, computed, minimum, utilization, message, annex)
        } else {
            Self::fail(clause, computed, minimum, utilization, message, annex)
        }
    }
}

/// 📑️ Aggregated compliance report for a norm computation run.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CheckReport {
    pub checks: Vec<CheckResult>,
}

impl CheckReport {
    pub fn push(&mut self, check: CheckResult) {
        self.checks.push(check);
    }

    pub fn all_pass(&self) -> bool {
        self.checks.iter().all(|c| c.status != CheckStatus::Fail)
    }

    pub fn worst_utilization(&self) -> f64 {
        self.checks.iter().map(|c| c.utilization).fold(0.0_f64, f64::max)
    }
}
// #endregion 🔖️Check

// #region 🔖️Annex
/// 🇪️🇺️ National annex selection for Eurocode / DIN EN families.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, dsl::DslScalar)]
pub enum AnnexChoice {
    #[dsl(key = "en")]
    En,
    #[dsl(key = "de")]
    De,
}

impl AnnexChoice {
    pub fn label(self) -> &'static str {
        match self {
            Self::En => "EN",
            Self::De => "DE-NA",
        }
    }
}

/// 🗺️ Trait for national annex parameter overrides.
pub trait NationalAnnex {
    fn choice(&self) -> AnnexChoice;
    fn gamma_g(&self) -> f64;
    fn gamma_q(&self) -> f64;
    fn gamma_m(&self, _material: &str) -> f64 {
        1.0
    }
    fn gamma_r(&self) -> f64 {
        1.0
    }
    fn xi(&self, _category: &str) -> f64 {
        1.0
    }
    fn psi_0(&self, category: &str) -> f64;
    fn psi_1(&self, category: &str) -> f64;
    fn psi_2(&self, category: &str) -> f64;
}
// #endregion 🔖️Annex

// #region 🔖️Tables
/// 📊️ One-dimensional table entry for norm lookups.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TableEntry1D {
    pub x: f64,
    pub y: f64,
}

/// 🔍️ Linear interpolation in a sorted 1D table.
pub fn table_lookup_linear(table: &[TableEntry1D], x: f64) -> f64 {
    if table.is_empty() {
        return 0.0;
    }
    if x <= table[0].x {
        return table[0].y;
    }
    if x >= table[table.len() - 1].x {
        return table[table.len() - 1].y;
    }
    for w in table.windows(2) {
        if x >= w[0].x && x <= w[1].x {
            let t = (x - w[0].x) / (w[1].x - w[0].x);
            return w[0].y + t * (w[1].y - w[0].y);
        }
    }
    table[table.len() - 1].y
}

/// 🔍️ Bilinear interpolation on a regular grid.
pub fn table_lookup_bilinear(x: f64, y: f64, x_vals: &[f64], y_vals: &[f64], z: &[f64]) -> f64 {
    let nx = x_vals.len();
    let ny = y_vals.len();
    if nx == 0 || ny == 0 || z.len() < nx * ny {
        return 0.0;
    }
    let xi = x_vals.iter().position(|&v| x <= v).unwrap_or(nx - 1).max(1);
    let yi = y_vals.iter().position(|&v| y <= v).unwrap_or(ny - 1).max(1);
    let x0 = x_vals[xi - 1];
    let x1 = x_vals[xi.min(nx - 1)];
    let y0 = y_vals[yi - 1];
    let y1 = y_vals[yi.min(ny - 1)];
    let tx = if (x1 - x0).abs() < f64::EPSILON { 0.0 } else { ((x - x0) / (x1 - x0)).clamp(0.0, 1.0) };
    let ty = if (y1 - y0).abs() < f64::EPSILON { 0.0 } else { ((y - y0) / (y1 - y0)).clamp(0.0, 1.0) };
    let z00 = z[(yi - 1) * nx + (xi - 1)];
    let z10 = z[(yi - 1) * nx + xi.min(nx - 1)];
    let z01 = z[yi.min(ny - 1) * nx + (xi - 1)];
    let z11 = z[yi.min(ny - 1) * nx + xi.min(nx - 1)];
    let z0 = z00 + tx * (z10 - z00);
    let z1 = z01 + tx * (z11 - z01);
    z0 + ty * (z1 - z0)
}
// #endregion 🔖️Tables

// #region 🔖️DesignSituation
/// 🏗️ Design situation per EN 1990 Table A1.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum DesignSituation {
    #[dsl(key = "persistent")]
    Persistent,
    #[dsl(key = "transient")]
    Transient,
    #[dsl(key = "accidental")]
    Accidental,
    #[dsl(key = "seismic")]
    Seismic,
}

/// 📋️ Consequence class per EN 1990.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceClass {
    Cc1,
    Cc2,
    Cc3,
}

impl ConsequenceClass {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Cc1 => 1,
            Self::Cc2 => 2,
            Self::Cc3 => 3,
        }
    }
}

/// 📊️ Variable action category per EN 1991-1-1 Table 6.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum ImposedCategory {
    #[dsl(key = "a")]
    A,
    #[dsl(key = "b")]
    B,
    #[dsl(key = "c")]
    C,
    #[dsl(key = "d")]
    D,
    #[dsl(key = "e")]
    E,
    #[dsl(key = "f")]
    F,
    #[dsl(key = "g")]
    G,
    #[dsl(key = "h")]
    H,
}

impl ImposedCategory {
    pub fn q_k_kn_m2(self) -> f64 {
        match self {
            Self::A => 2.0,
            Self::B => 2.5,
            Self::C => 3.0,
            Self::D => 4.0,
            Self::E => 5.0,
            Self::F => 3.0,
            Self::G => 5.0,
            Self::H => 20.0,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::A => "residential",
            Self::B => "office",
            Self::C => "congregation",
            Self::D => "retail",
            Self::E => "storage",
            Self::F => "traffic_light",
            Self::G => "traffic_heavy",
            Self::H => "roof",
        }
    }
}
// #endregion 🔖️DesignSituation

// #region 🔖️Shared
/// ⚖️ Limit state per EN 1990.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitState {
    Uls,
    Sls,
    Als,
    Fls,
}

/// ⏱️ Load duration class for timber and similar materials.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadDuration {
    Permanent,
    Long,
    Medium,
    Short,
    Instantaneous,
}

/// 🌡️ Reference climate zone for thermal norms (Germany).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum ClimateZoneDe {
    #[dsl(key = "zone1")]
    Zone1,
    #[dsl(key = "zone2")]
    Zone2,
    #[dsl(key = "zone3")]
    Zone3,
    #[dsl(key = "zone4")]
    Zone4,
}

impl ClimateZoneDe {
    pub fn design_external_temperature_c(self) -> f64 {
        match self {
            Self::Zone1 => -16.0,
            Self::Zone2 => -14.0,
            Self::Zone3 => -12.0,
            Self::Zone4 => -10.0,
        }
    }

    pub fn summer_design_temperature_c(self) -> f64 {
        match self {
            Self::Zone1 => 26.0,
            Self::Zone2 => 28.0,
            Self::Zone3 => 30.0,
            Self::Zone4 => 32.0,
        }
    }

    pub fn heating_degree_days(self) -> f64 {
        match self {
            Self::Zone1 => 3800.0,
            Self::Zone2 => 3200.0,
            Self::Zone3 => 2600.0,
            Self::Zone4 => 2000.0,
        }
    }
}

/// 🏠️ Occupancy type for indoor environment norms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum OccupancyType {
    #[dsl(key = "residential")]
    Residential,
    #[dsl(key = "office")]
    Office,
    #[dsl(key = "classroom")]
    Classroom,
    #[dsl(key = "retail")]
    Retail,
    #[dsl(key = "meeting")]
    Meeting,
    #[dsl(key = "kitchen")]
    Kitchen,
    #[dsl(key = "corridor")]
    Corridor,
}
// #endregion 🔖️Shared

// #region 🔖️Error
/// ⚠️ Norm computation error.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum NormError {
    #[error("incomplete input: {field}")]
    IncompleteInput { field: String },
    #[error("out of scope: {clause}")]
    OutOfScope { clause: ClauseId },
    #[error("invalid {field}: {reason}")]
    InvalidValue { field: String, reason: String },
}
// #endregion 🔖️Error

// #region 🔖️Family
/// 🏷️ Stable identifier for each norm family crate exposed as a DocumentApp.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NormFamilyId {
    Din4108,
    DinEn16798,
    DinV18599,
    En1990,
    En1991,
    En1992,
    En1993,
    En1994,
    En1995,
    En1996,
    En1997,
    En1998,
    En1999,
    Iso16757,
    Vdi3805,
}

impl NormFamilyId {
    pub fn label(self) -> &'static str {
        match self {
            Self::Din4108 => "DIN 4108",
            Self::DinEn16798 => "DIN EN 16798",
            Self::DinV18599 => "DIN V 18599",
            Self::En1990 => "EN 1990",
            Self::En1991 => "EN 1991",
            Self::En1992 => "EN 1992",
            Self::En1993 => "EN 1993",
            Self::En1994 => "EN 1994",
            Self::En1995 => "EN 1995",
            Self::En1996 => "EN 1996",
            Self::En1997 => "EN 1997",
            Self::En1998 => "EN 1998",
            Self::En1999 => "EN 1999",
            Self::Iso16757 => "ISO 16757",
            Self::Vdi3805 => "VDI 3805",
        }
    }
}

/// 🧩️ Headless norm family contract: typed document, undoable operations, and compliance evaluation.
pub trait NormFamily: Send + Sync + 'static {
    type Document: Clone + Default + PartialEq + Serialize + DeserializeOwned + Send;
    type Operation: Operation<Self::Document> + Clone + PartialEq + Send;

    fn family_id() -> NormFamilyId;
    fn evaluate(document: &Self::Document) -> CheckReport;
}

/// 📤️ Replace the whole family document (VCS undoable).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentDiff<D> {
    #[serde(default)]
    pub document: Option<D>,
}

impl<D: Clone + Default + Serialize + DeserializeOwned> OperationDiff<D> for DocumentDiff<D> {
    fn apply(&self, projection: &D) -> D {
        self.document.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            self.document = other.document;
        }
    }
}

/// 📤️ Whole-document replacement operation shared by norm family sessions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum SetDocumentOperation<D> {
    SetDocument { document: D },
}

impl<D: Clone + Default + PartialEq + Serialize + DeserializeOwned> Operation<D> for SetDocumentOperation<D> {
    type Diff = DocumentDiff<D>;

    fn diff(&self, _projection: &D) -> DocumentDiff<D> {
        match self {
            Self::SetDocument { document } => DocumentDiff { document: Some(document.clone()) },
        }
    }

    fn backwards(&self, projection: &D) -> Vec<Self> {
        vec![Self::SetDocument { document: projection.clone() }]
    }
}

/// 🧠️ Retained headless session: document inputs plus the last computed compliance report.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "F::Document: Serialize", deserialize = "F::Document: DeserializeOwned"))]
pub struct NormHost<F: NormFamily> {
    pub document: F::Document,
    pub report: CheckReport,
}

impl<F: NormFamily> Default for NormHost<F> {
    fn default() -> Self {
        Self::from_document(F::Document::default())
    }
}

impl<F: NormFamily> NormHost<F> {
    pub fn from_document(document: F::Document) -> Self {
        let report = F::evaluate(&document);
        Self { document, report }
    }

    pub fn document(&self) -> &F::Document {
        &self.document
    }

    pub fn report(&self) -> &CheckReport {
        &self.report
    }

    pub fn apply(&mut self, operation: &F::Operation) {
        self.document = vcs::apply_operation(&self.document, operation);
        self.report = F::evaluate(&self.document);
    }

    pub fn replace_document(&mut self, document: F::Document) {
        self.document = document;
        self.report = F::evaluate(&self.document);
    }

    pub fn evaluate(&mut self) {
        self.report = F::evaluate(&self.document);
    }
}
// #endregion 🔖️Family

// #region 🔖️OpText
/// ✂️ Escapes `\`, `"`, `\n` for embedding arbitrary (possibly multi-line) text inside a single quoted
/// op-text field. Mirrors vcs's own private `escape_text_field`/`unescape_text_field` convention
/// exactly (same three escapes, same order) so escaping behaves identically repo-wide, even though vcs
/// does not expose those helpers for reuse.
fn escape_op_text_field(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            _ => out.push(ch),
        }
    }
    out
}

/// ✂️ Inverts {@link escape_op_text_field}.
fn unescape_op_text_field(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

/// ⚡️ One `OpText` implementation shared by every norm family: `SetDocumentOperation<D>`'s only variant
/// replaces the whole document, so the op line is `set-document "<escaped D::print_dsl() text>"` — the
/// (possibly multi-line) DSL text folded onto one physical line via {@link escape_op_text_field}, and
/// recovered via `D::parse_dsl` after {@link unescape_op_text_field}. Bounded on `store::DocumentDsl` (for
/// `print_dsl`/`parse_dsl`) plus the same bounds `SetDocumentOperation<D>`'s own `Operation<D>` impl
/// already requires, so every one of the 15 family `Document` types gets this for free the moment it
/// implements `DocumentDsl`.
impl<D> OpText for SetDocumentOperation<D>
where
    D: DocumentDsl + Clone + Default + PartialEq + Serialize + DeserializeOwned,
{
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let trimmed = line.trim();
        let rest = trimmed.strip_prefix("set-document ").ok_or_else(|| TextError::new("expected 'set-document \"<document>\"'", TextSpan::at(1, 1)))?.trim();
        let quoted = rest.strip_prefix('"').and_then(|value| value.strip_suffix('"')).ok_or_else(|| TextError::new("expected a double-quoted document text field", TextSpan::at(1, 15)))?;
        let document_text = unescape_op_text_field(quoted);
        let document = D::parse_dsl(&document_text)?;
        Ok(SetDocumentOperation::SetDocument { document })
    }

    fn print_op(&self) -> String {
        match self {
            SetDocumentOperation::SetDocument { document } => {
                format!("set-document \"{}\"", escape_op_text_field(&document.print_dsl()))
            }
        }
    }
}

/// ⚡️ Binary twin of the `OpText` impl above, shared the same way: `format u8 (=1) | pack bytes`
/// (no length prefix needed — the single field consumes every remaining byte), using
/// `D::encode_pack`/`decode_pack` instead of the DSL-text quoting trick (unnecessary for binary,
/// which has no line-boundary to protect). Bounded on `store::DocumentPack` in addition to the
/// `OpText` impl's bounds — every family `Document` type gets both for free from the same
/// `#[derive(dsl::DslDocument)]` that already granted `DocumentDsl`.
impl<D> OpBinary for SetDocumentOperation<D>
where
    D: DocumentPack + Clone + Default + PartialEq + Serialize + DeserializeOwned,
{
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        match self {
            SetDocumentOperation::SetDocument { document } => {
                let mut out = vec![1u8];
                out.extend(document.encode_pack());
                Ok(out)
            }
        }
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let format = *bytes.first().ok_or(ProtocolError::Malformed { what: "set-document operation", offset: 0, detail: "empty payload".to_string() })?;
        if format != 1 {
            return Err(ProtocolError::Malformed { what: "set-document operation", offset: 0, detail: format!("unsupported format {format}") });
        }
        let document = D::decode_pack(&bytes[1..]).map_err(|error| ProtocolError::Malformed { what: "set-document operation", offset: 1, detail: error.to_string() })?;
        Ok(SetDocumentOperation::SetDocument { document })
    }
}
// #endregion 🔖️OpText

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result_passes_when_utilization_below_one() {
        let clause = ClauseId::new("EN 1990", "§6.4", "6.10");
        let result = CheckResult::from_utilization(clause, Quantity::stress_mpa(250.0), Quantity::stress_mpa(300.0), "ULS stress check", AnnexChoice::De);
        assert_eq!(result.status, CheckStatus::Pass);
        assert!(result.utilization < 1.0);
    }

    #[test]
    fn table_lookup_linear_interpolates() {
        let table = [TableEntry1D { x: 0.0, y: 1.0 }, TableEntry1D { x: 10.0, y: 2.0 }];
        assert!((table_lookup_linear(&table, 5.0) - 1.5).abs() < 1e-9);
    }

    #[test]
    fn check_minimum_passes_when_above_threshold() {
        let result = CheckResult::from_minimum(ClauseId::new("DIN 4108-3", "§6", "6.1"), Quantity::new(QuantityKind::Dimensionless, 0.8), Quantity::new(QuantityKind::Dimensionless, 0.25), "f_Rsi", AnnexChoice::De);
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
    #[dsl(extension = "demo-norm", layout = "lines")]
    struct DemoDocument {
        value: f64,
    }

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for DemoDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for DemoDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️DocumentCodec


    struct DemoFamily;

    impl NormFamily for DemoFamily {
        type Document = DemoDocument;
        type Operation = SetDocumentOperation<DemoDocument>;

        fn family_id() -> NormFamilyId {
            NormFamilyId::En1990
        }

        fn evaluate(document: &DemoDocument) -> CheckReport {
            let mut report = CheckReport::default();
            report.push(CheckResult::from_utilization(ClauseId::new("demo", "§1", "1.1"), Quantity::new(QuantityKind::Dimensionless, document.value), Quantity::new(QuantityKind::Dimensionless, 1.0), "demo check", AnnexChoice::De));
            report
        }
    }

    #[test]
    fn norm_host_recomputes_report_after_apply() {
        let mut host = NormHost::<DemoFamily>::default();
        assert!(host.report().checks[0].utilization < 1.0);
        host.apply(&SetDocumentOperation::SetDocument { document: DemoDocument { value: 2.0 } });
        assert!(host.report().checks[0].utilization > 1.0);
    }

    #[test]
    fn demo_document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&DemoDocument { value: 4.5 });
        store::test_support::assert_dsl_pack_equivalence(&DemoDocument { value: 4.5 });
    }

    #[test]
    fn set_document_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&SetDocumentOperation::SetDocument { document: DemoDocument { value: 4.5 } });
    }

    #[test]
    fn set_document_operation_op_text_escapes_multiline_dsl_text() {
        // ⚡️ The op-text field wraps the newline `print_dsl` always emits, so this exercises the
        // `\n` escape (not just the general round-trip law already covered above).
        let printed = SetDocumentOperation::SetDocument { document: DemoDocument { value: 7.0 } }.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
        assert_eq!(printed, "set-document \"value=7\\n\"");
        let parsed = <SetDocumentOperation<DemoDocument> as OpText>::parse_op(&printed).expect("parse_op");
        assert_eq!(parsed, SetDocumentOperation::SetDocument { document: DemoDocument { value: 7.0 } });
    }

    #[test]
    fn document_text_round_trips_for_a_norm_family_document() {
        let envelope = store::create_document_envelope("norm.demo/v1", "demo", DemoDocument { value: 1.0 }, None);
        let mut store = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { operations: vec![SetDocumentOperation::SetDocument { document: DemoDocument { value: 3.0 } }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `SetDocumentOperation<DemoDocument>`'s `Edit` round-trips through
    /// `protocol::OperationEnvelope`s beside this file's existing pack round-trip law (same pattern
    /// as `dag`'s own `command_envelope_round_trip_holds_for_an_applied_operation`) — proves the law
    /// once for the shared generic `SetDocumentOperation<D>` bridge (`POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST`'s
    /// `"SetDocumentOperation"` entry), covering every norm family that reuses it.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let envelope = store::create_document_envelope("norm.demo/v1", "demo", DemoDocument { value: 1.0 }, None);
        let mut store = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { operations: vec![SetDocumentOperation::SetDocument { document: DemoDocument { value: 3.0 } }], description: None }).expect("apply");
        let edit: &Edit<SetDocumentOperation<DemoDocument>> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<DemoDocument, SetDocumentOperation<DemoDocument>>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
