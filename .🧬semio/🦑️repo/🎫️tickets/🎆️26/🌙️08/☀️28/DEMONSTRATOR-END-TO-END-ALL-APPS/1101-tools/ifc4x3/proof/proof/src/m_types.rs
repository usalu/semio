#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub href: &'static str,
    pub universal: bool,
    pub reference: Option<&'static [&'static str]>,
    pub alignment: Option<&'static [&'static str]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdSource {
    pub url: &'static str,
    pub sha256: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdCatalog {
    pub schema: &'static str,
    pub source_commit: &'static str,
    pub sources: &'static [IfcMvdSource],
    pub conformance_evaluated: bool,
    pub entities: &'static [IfcMvdEntry],
    pub concepts: &'static [IfcMvdEntry],
}

