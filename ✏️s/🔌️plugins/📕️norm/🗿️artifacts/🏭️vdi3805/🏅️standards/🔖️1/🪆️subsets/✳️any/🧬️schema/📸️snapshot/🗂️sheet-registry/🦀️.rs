//! 🗂️ Sheet registry of the VDI 3805 snapshot: the sheet and edition identifiers with their DSL field bindings, the schema status, the domain and the sheet entry table. See [`super`].

/// 📄️ Sheet identifier (1…100).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(transparent)]
pub struct SheetId(pub u16);

impl SheetId {
    pub fn part_str(self) -> String {
        format!("{}", self.0)
    }
}

/// 🔗️ Hand `DslField` bridge for `SheetId`: a tuple ("newtype") struct has no named fields for
/// `#[derive(dsl::DslRecord)]` to enumerate, so it binds directly as `Shape::UInt` instead of
/// changing its public tuple shape (used pervasively as `.0` across this crate).
impl semio_framework_dsl_record::BorrowedDslField for SheetId{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::UInt;}

impl semio_framework_dsl_record::DslField for SheetId {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{<u16 as semio_framework_dsl_record::DslField>::shape_controlled(control)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{<u16 as semio_framework_dsl_record::DslField>::to_value_controlled(&self.0,control)}

    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self,semio_framework_value::ValueError> {
        <u16 as semio_framework_dsl_record::DslField>::from_value_controlled(value, control).map(Self)
    }
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::UInt
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::UInt(self.0 as u64)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::UInt(v) => u16::try_from(*v).map(Self).map_err(|_| format!("sheet id {v} out of range for u16")),
            other => Err(format!("expected UInt, found {other:?}")),
        }
    }
}

/// 📅️ Edition identifier (year + month).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct EditionId {
    pub year: u16,
    pub month: u8,
}

impl EditionId {
    pub const fn new(year: u16, month: u8) -> Self {
        Self { year, month }
    }

    pub fn key(self) -> u32 {
        (self.year as u32) * 100 + self.month as u32
    }
}

/// 📊️ Schema lifecycle status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum SchemaStatus {
    Published,
    Checked,
    Draft,
    Project,
    Withdrawn,
    Superseded,
    HistoricalProposal,
    Reserved,
}

impl SchemaStatus {
    pub fn is_operative(self) -> bool {
        matches!(self, Self::Published | Self::Checked)
    }
}

/// 🏷️ Building-services domain filter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum Domain {
    Heating,
    Ventilation,
    Sanitary,
    BuildingAutomation,
    Electrical,
    Generic,
}

/// 📋️ Sheet registry entry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheetEntry {
    pub id: SheetId,
    pub title_de: &'static str,
    pub title_en: &'static str,
    pub status: SchemaStatus,
    pub domains: &'static [Domain],
    pub part1_edition: EditionId,
    pub current_edition: EditionId,
}

/// 📚️ The full 100-sheet VDI 3805 registry (Part 1 + sheets 2–100), 1:1 from the norm's sheet index.
pub const SHEET_ENTRIES: &[SheetEntry] = &[
    SheetEntry { id: SheetId(1), title_de: "Grundlagen", title_en: "Fundamentals", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(2), title_de: "Stellventile Heizung", title_en: "Control valves heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(3), title_de: "Heizkörper", title_en: "Radiators", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(4), title_de: "Rohrleitungen Heizung", title_en: "Pipes heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(5), title_de: "Pumpen Heizung", title_en: "Pumps heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(6), title_de: "Wärmeerzeuger", title_en: "Heat generators", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(7), title_de: "Speicher", title_en: "Storage tanks", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(8), title_de: "Armaturen Heizung", title_en: "Fittings heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(9), title_de: "Regelung Heizung", title_en: "Controls heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(10), title_de: "Verteiler Heizung", title_en: "Manifolds heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(11), title_de: "Messgeräte Heizung", title_en: "Meters heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry {
        id: SheetId(12),
        title_de: "Historischer Vorschlag A",
        title_en: "Historical proposal A",
        status: SchemaStatus::HistoricalProposal,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2018, 6),
    },
    SheetEntry {
        id: SheetId(13),
        title_de: "Historischer Vorschlag B",
        title_en: "Historical proposal B",
        status: SchemaStatus::HistoricalProposal,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2018, 6),
    },
    SheetEntry { id: SheetId(14), title_de: "Ventile Lüftung", title_en: "Valves ventilation", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(15), title_de: "Blatt 15", title_en: "Sheet 15", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(16), title_de: "Luftdurchlässe", title_en: "Air terminals", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(17), title_de: "Kanäle Lüftung", title_en: "Ducts ventilation", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(18), title_de: "Blatt 18", title_en: "Sheet 18", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(19), title_de: "Filter Lüftung", title_en: "Filters ventilation", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(20), title_de: "Wärmerückgewinnung", title_en: "Heat recovery", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(21), title_de: "Sanitärarmaturen", title_en: "Sanitary fittings", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(22), title_de: "Rohrleitungen Sanitär", title_en: "Pipes sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(23), title_de: "Pumpen Sanitär", title_en: "Pumps sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(24), title_de: "Speicher Sanitär", title_en: "Storage sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry {
        id: SheetId(25),
        title_de: "Historischer Vorschlag Sanitär",
        title_en: "Historical proposal sanitary",
        status: SchemaStatus::HistoricalProposal,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2018, 6),
    },
    SheetEntry { id: SheetId(26), title_de: "Regelung Sanitär", title_en: "Controls sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(27), title_de: "Messgeräte Sanitär", title_en: "Meters sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry {
        id: SheetId(28),
        title_de: "Gebäudeautomation",
        title_en: "Building automation",
        status: SchemaStatus::Published,
        domains: &[Domain::BuildingAutomation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry { id: SheetId(29), title_de: "Elektro Komponenten", title_en: "Electrical components", status: SchemaStatus::Published, domains: &[Domain::Electrical], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(30), title_de: "Blatt 30", title_en: "Sheet 30", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(31), title_de: "Blatt 31", title_en: "Sheet 31", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(32), title_de: "Kältemaschinen", title_en: "Chillers", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(33), title_de: "Kühldecken", title_en: "Chilled ceilings", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(34), title_de: "Konvektoren", title_en: "Convectors", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(35), title_de: "Fußbodenheizung", title_en: "Underfloor heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(36), title_de: "Blatt 36", title_en: "Sheet 36", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(37), title_de: "Blatt 37", title_en: "Sheet 37", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(38), title_de: "Schalldämpfer", title_en: "Silencers", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(39), title_de: "Blatt 39", title_en: "Sheet 39", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(40), title_de: "Klappen Lüftung", title_en: "Dampers ventilation", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(41), title_de: "Ventilatoren", title_en: "Fans", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(42), title_de: "VAV-Regler", title_en: "VAV controllers", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(43), title_de: "Wärmetauscher", title_en: "Heat exchangers", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(44), title_de: "Druckerhöhung", title_en: "Pressure boosting", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(45), title_de: "Entgasung", title_en: "Degassing", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(46), title_de: "Blatt 46", title_en: "Sheet 46", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(47), title_de: "Blatt 47", title_en: "Sheet 47", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(48), title_de: "Blatt 48", title_en: "Sheet 48", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(49), title_de: "Blatt 49", title_en: "Sheet 49", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(50), title_de: "Brandschutzklappen", title_en: "Fire dampers", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(51), title_de: "Rohrbegleitheizung", title_en: "Trace heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(52), title_de: "Solarthermie", title_en: "Solar thermal", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(53), title_de: "Wärmepumpen", title_en: "Heat pumps", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(54), title_de: "Befeuchtung", title_en: "Humidification", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(55), title_de: "Entfeuchtung", title_en: "Dehumidification", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(56), title_de: "Blatt 56", title_en: "Sheet 56", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(57), title_de: "Blatt 57", title_en: "Sheet 57", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(58), title_de: "Blatt 58", title_en: "Sheet 58", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(59), title_de: "Blatt 59", title_en: "Sheet 59", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(60), title_de: "Kompensatoren", title_en: "Compensators", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(61), title_de: "Trennstellen", title_en: "Separation points", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(62), title_de: "Schmutzfänger", title_en: "Strainers", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(63), title_de: "Rückschlagventile", title_en: "Check valves", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(64), title_de: "Sicherheitsventile", title_en: "Safety valves", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(65), title_de: "Absperrarmaturen", title_en: "Shut-off fittings", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(66), title_de: "Mischventile", title_en: "Mixing valves", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(67), title_de: "Blatt 67", title_en: "Sheet 67", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(68), title_de: "Blatt 68", title_en: "Sheet 68", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(69), title_de: "Blatt 69", title_en: "Sheet 69", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(70), title_de: "Blatt 70", title_en: "Sheet 70", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(71), title_de: "Blatt 71", title_en: "Sheet 71", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(72), title_de: "Blatt 72", title_en: "Sheet 72", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(73), title_de: "Blatt 73", title_en: "Sheet 73", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(74), title_de: "Blatt 74", title_en: "Sheet 74", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(75), title_de: "Blatt 75", title_en: "Sheet 75", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(76), title_de: "Blatt 76", title_en: "Sheet 76", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(77), title_de: "Blatt 77", title_en: "Sheet 77", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(78), title_de: "Blatt 78", title_en: "Sheet 78", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(79), title_de: "Blatt 79", title_en: "Sheet 79", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(80), title_de: "Blatt 80", title_en: "Sheet 80", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(81), title_de: "Blatt 81", title_en: "Sheet 81", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(82), title_de: "Blatt 82", title_en: "Sheet 82", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(83), title_de: "Blatt 83", title_en: "Sheet 83", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(84), title_de: "Blatt 84", title_en: "Sheet 84", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(85), title_de: "Blatt 85", title_en: "Sheet 85", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(86), title_de: "Blatt 86", title_en: "Sheet 86", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(87), title_de: "Blatt 87", title_en: "Sheet 87", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(88), title_de: "Blatt 88", title_en: "Sheet 88", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(89), title_de: "Blatt 89", title_en: "Sheet 89", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(90), title_de: "Blatt 90", title_en: "Sheet 90", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(91), title_de: "Blatt 91", title_en: "Sheet 91", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(92), title_de: "Blatt 92", title_en: "Sheet 92", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(93), title_de: "Blatt 93", title_en: "Sheet 93", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(94), title_de: "Blatt 94", title_en: "Sheet 94", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(95), title_de: "Blatt 95", title_en: "Sheet 95", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(96), title_de: "Blatt 96", title_en: "Sheet 96", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(97), title_de: "Blatt 97", title_en: "Sheet 97", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(98), title_de: "Blatt 98", title_en: "Sheet 98", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(99), title_de: "Erweiterungen", title_en: "Extensions", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(100), title_de: "Profilübergreifend", title_en: "Cross-profile", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
];
