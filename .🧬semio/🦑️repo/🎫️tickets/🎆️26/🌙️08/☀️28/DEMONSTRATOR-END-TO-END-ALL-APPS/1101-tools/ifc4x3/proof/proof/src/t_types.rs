#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfcTemplateEntryKind {
    SingleValue,
    ReferenceValue,
    EnumeratedValue,
    ListValue,
    BoundedValue,
    TableValue,
    Weight,
    Length,
    Area,
    Volume,
    Count,
    Time,
}
impl IfcTemplateEntryKind {
    pub const fn source_key(self) -> &'static str {
        match self {
            Self::SingleValue => "P_SINGLEVALUE",
            Self::ReferenceValue => "P_REFERENCEVALUE",
            Self::EnumeratedValue => "P_ENUMERATEDVALUE",
            Self::ListValue => "P_LISTVALUE",
            Self::BoundedValue => "P_BOUNDEDVALUE",
            Self::TableValue => "P_TABLEVALUE",
            Self::Weight => "Q_WEIGHT",
            Self::Length => "Q_LENGTH",
            Self::Area => "Q_AREA",
            Self::Volume => "Q_VOLUME",
            Self::Count => "Q_COUNT",
            Self::Time => "Q_TIME",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplateEnumeration {
    pub name: &'static str,
    pub literals: &'static [&'static str],
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplateEntry {
    pub name: &'static str,
    pub definition: &'static str,
    pub kind: IfcTemplateEntryKind,
    pub data_type: &'static str,
    pub enumeration: Option<IfcTemplateEnumeration>,
    pub secondary_type: Option<&'static str>,
    pub expression: Option<&'static str>,
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplate {
    pub name: &'static str,
    pub definition: &'static str,
    pub kind: &'static str,
    pub applicable: &'static str,
    pub classes: &'static [&'static str],
    pub entries: &'static [IfcTemplateEntry],
    pub source_file: &'static str,
    pub source_sha256: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplateSource {
    pub url: &'static str,
    pub commit: &'static str,
    pub sha256: &'static str,
    pub language: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplateCatalog {
    pub schema: &'static str,
    pub source: IfcTemplateSource,
    pub templates: &'static [IfcTemplate],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfcTemplateApplicability {
    Applicable,
    Inapplicable,
    RequiresPredefinedType,
    UnknownEntity,
}
impl IfcTemplateApplicability {
    pub const fn source_key(self) -> &'static str {
        match self {
            Self::Applicable => "applicable",
            Self::Inapplicable => "inapplicable",
            Self::RequiresPredefinedType => "requiresPredefinedType",
            Self::UnknownEntity => "unknownEntity",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum IfcTemplateLocale {
    En,
    De,
}
