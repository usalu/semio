//! 🧬️ Vdi3805 artifact schema — every field of the artifact with its state class.

use std::collections::BTreeMap;

use crate::{GenericAttributes, CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use ::framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full Vdi3805 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Artifact {
    #[state(artifact)]
    pub catalog: ManufacturerCatalog,
    #[state(artifact)]
    pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(artifact)]
    pub correction_as_of: EditionId,
    #[state(artifact)]
    pub strict_mode: bool,
    #[state(artifact)]
    pub index: CatalogIndex,
    #[state(artifact)]
    pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(artifact)]
    pub curves: BTreeMap<String, CharacteristicCurve>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Vdi3805Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Vdi3805Snapshot {
        Vdi3805Snapshot {
            catalog: self.catalog.clone(),
            edition_profile: self.edition_profile.clone(),
            correction_as_of: self.correction_as_of,
            strict_mode: self.strict_mode,
            index: self.index.clone(),
            geometry: self.geometry.clone(),
            curves: self.curves.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Vdi3805Snapshot) -> Self {
        Self {
            catalog: snapshot.catalog,
            edition_profile: snapshot.edition_profile,
            correction_as_of: snapshot.correction_as_of,
            strict_mode: snapshot.strict_mode,
            index: snapshot.index,
            geometry: snapshot.geometry,
            curves: snapshot.curves,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: Vdi3805Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.vdi3805` — twenty handcrafted schema leaves.
pub fn vdi3805_artifact_schema_descriptor() -> ::framework_schema::ArtifactSchemaDescriptor {
    ::framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.vdi3805",
        artifact: ::framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: ::framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Vdi3805BuilderConstruction {
        snapshot: Vdi3805Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Vdi3805BuilderConstruction {
        type Snapshot = Vdi3805Snapshot;
        type Mutation = Vdi3805Mutation;
        type Diff = Vdi3805Diff;
        fn empty() -> Self {
            Self { snapshot: Vdi3805Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::Vdi3805Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Vdi3805Parts {
        pub snapshot: Option<Vdi3805Snapshot>,
    }

    pub struct Vdi3805AnalyzerAnalysis;

    impl ArtifactAnalysis for Vdi3805AnalyzerAnalysis {
        type Parts = Vdi3805Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Vdi3805Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Vdi3805BuilderFacets {
        construction: Vdi3805BuilderConstruction,
        analysis: Vdi3805AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Vdi3805ComposerComposition,
    }
    builder: Vdi3805Builder,
    analyzer: Vdi3805Analyzer,
    composer: Vdi3805Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
use crate::document::{AnnexChoice, ClauseId, LocalizedCopy, NormError, SubjectRef};
/// 📐️ Pure VDI 3805 compliance helpers — native-text parsing with typed record families,
/// Part 1 structural validation, and sheet attribute catalogues.
use crate::*;
use ::dsl;

const FAMILY: &str = "VDI 3805";
pub const ANNEX: AnnexChoice = AnnexChoice::De;

pub fn clause(part: &str, section: &str) -> ClauseId {
    ClauseId::new(FAMILY, part, section)
}

pub fn subject_product(product: &CatalogueProduct) -> SubjectRef {
    SubjectRef::new(
        product.id.clone(),
        format!("catalog.products[id={}]", product.id),
        LocalizedCopy::new(format!("Product {}", product.identity.article_number), format!("Produkt {}", product.identity.article_number)),
    )
}

pub fn subject_dataset() -> SubjectRef {
    SubjectRef::whole(LocalizedCopy::new("Manufacturer dataset", "Herstellerdatensatz"))
}

/// 🎛 Nominal diameters admitted by VDI 3805 Blatt 2 / ISO 6708 series.
pub const VALVE_DN_SERIES: &[u16] = &[10, 15, 20, 25, 32, 40, 50, 65, 80, 100, 125, 150, 200, 250, 300];

/// 📊️ Minimum kvs [m³/h] per DN for Blatt 2 control valves (catalogue floor).
pub fn valve_min_kvs_m3_h(dn: u16) -> f64 {
    match dn {
        10 => 0.16,
        15 => 0.25,
        20 => 0.40,
        25 => 0.63,
        32 => 1.00,
        40 => 1.60,
        50 => 2.50,
        65 => 4.00,
        80 => 6.30,
        100 => 10.0,
        125 => 16.0,
        150 => 25.0,
        200 => 40.0,
        250 => 63.0,
        300 => 100.0,
        _ => 0.63,
    }
}

/// 📐️ Minimum radiator heat exponent n (EN 442 / Blatt 3).
pub const RADIATOR_N_MIN: f64 = 1.1;
/// 📐️ Maximum radiator heat exponent n (EN 442 / Blatt 3).
pub const RADIATOR_N_MAX: f64 = 1.5;

/// 📔️ Minimum field arity (including family code) per record family.
pub fn record_min_fields(family: &str) -> Option<usize> {
    match family {
        "010" => Some(6),
        "100" => Some(5),
        "110" => Some(3),
        "200" => Some(5),
        "400" => Some(4),
        "700" => Some(3),
        "900" => Some(2),
        _ => None,
    }
}

fn parse_f64(raw: &str) -> Option<f64> {
    raw.replace(',', ".").parse().ok()
}

pub fn attributes_from_records(sheet: SheetId, records: &[NativeRecord]) -> SheetAttributes {
    let mut kv: BTreeMap<String, String> = BTreeMap::new();
    for record in records {
        if record.family.0.as_str() == "210" {
            let mut it = record.fields.iter().skip(1);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if parse_f64(k).is_none() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        } else if record.family.0.as_str() == "110" && record.fields.len() > 4 {
            let mut it = record.fields.iter().skip(4);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if parse_f64(k).is_none() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        } else if record.family.0.as_str() == "200" && record.fields.len() > 5 {
            let mut it = record.fields.iter().skip(5);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if parse_f64(k).is_none() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        }
        if record.family.0 == RecordFamilyId::R100 {
            if let Some(group) = record.fields.get(2) {
                kv.entry("product_group".into()).or_insert_with(|| group.clone());
            }
        }
    }
    match sheet.0 {
        2 => {
            let dn = kv.get("dn").and_then(|s| s.parse().ok()).unwrap_or(0);
            let kvs_m3_h = kv.get("kvs").and_then(|s| parse_f64(s)).unwrap_or(0.0);
            SheetAttributes::ValveHeating(ValveHeatingAttributes::from_kvs_m3_h(
                dn,
                kvs_m3_h,
                kv.get("pressure_class").cloned().unwrap_or_default(),
                kv.get("connection_type").cloned().unwrap_or_default(),
                kv.get("authority_min").and_then(|s| parse_f64(s)).unwrap_or(0.0),
                kv.get("authority_max").and_then(|s| parse_f64(s)).unwrap_or(0.0),
            ))
        }
        3 => SheetAttributes::Radiator(RadiatorAttributes {
            standard_output_w: kv.get("standard_output_w").or(kv.get("phi")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            heat_exponent_n: kv.get("n").or(kv.get("heat_exponent_n")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            length_m: kv.get("length_m").or(kv.get("l")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            height_m: kv.get("height_m").or(kv.get("h")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            depth_m: kv.get("depth_m").or(kv.get("d")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            connection_type: kv.get("connection_type").cloned().unwrap_or_default(),
        }),
        5 => SheetAttributes::PumpHeating(PumpHeatingAttributes {
            dn_suction: kv.get("dn_suction").and_then(|s| s.parse().ok()).unwrap_or(0),
            dn_discharge: kv.get("dn_discharge").or(kv.get("dn")).and_then(|s| s.parse().ok()).unwrap_or(0),
            nominal_flow_m3_s: kv.get("q").or(kv.get("nominal_flow_m3_s")).and_then(|s| parse_f64(s)).map(|q| if q > 1.0 { q / 3600.0 } else { q }).unwrap_or(0.0),
            nominal_head_m: kv.get("h").or(kv.get("nominal_head_m")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            motor_power_w: kv.get("p").or(kv.get("motor_power_w")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            hydraulic_efficiency: kv.get("eta").or(kv.get("hydraulic_efficiency")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            qh_curve_ref: kv.get("qh_curve_ref").cloned(),
        }),
        6 => SheetAttributes::HeatGenerator(HeatGeneratorAttributes {
            nominal_heat_output_w: kv.get("qn").or(kv.get("nominal_heat_output_w")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            fuel_type: kv.get("fuel_type").cloned().unwrap_or_default(),
            flow_temp_max_c: kv.get("flow_temp_max_c").and_then(|s| parse_f64(s)).unwrap_or(0.0),
            return_temp_min_c: kv.get("return_temp_min_c").and_then(|s| parse_f64(s)).unwrap_or(0.0),
        }),
        _ => SheetAttributes::Generic(GenericAttributes::from_map(&kv)),
    }
}

/// 🔄 Writes typed configuration.attributes into a native 210 record (Part 1 authoritative mirror).
pub fn sync_typed_attributes_into_records(product: &mut CatalogueProduct) {
    let fields = match &product.configuration.attributes {
        SheetAttributes::ValveHeating(a) => vec![
            "210".into(),
            "dn".into(),
            a.dn.to_string(),
            "kvs".into(),
            format!("{}", a.kvs_m3_h()),
            "pressure_class".into(),
            a.pressure_class.clone(),
            "connection_type".into(),
            a.connection_type.clone(),
            "authority_min".into(),
            a.authority_min.to_string(),
            "authority_max".into(),
            a.authority_max.to_string(),
        ],
        SheetAttributes::Radiator(a) => vec![
            "210".into(),
            "standard_output_w".into(),
            a.standard_output_w.to_string(),
            "n".into(),
            a.heat_exponent_n.to_string(),
            "length_m".into(),
            a.length_m.to_string(),
            "height_m".into(),
            a.height_m.to_string(),
            "depth_m".into(),
            a.depth_m.to_string(),
            "connection_type".into(),
            a.connection_type.clone(),
        ],
        SheetAttributes::PumpHeating(a) => vec![
            "210".into(),
            "dn_suction".into(),
            a.dn_suction.to_string(),
            "dn_discharge".into(),
            a.dn_discharge.to_string(),
            "nominal_flow_m3_s".into(),
            a.nominal_flow_m3_s.to_string(),
            "nominal_head_m".into(),
            a.nominal_head_m.to_string(),
            "motor_power_w".into(),
            a.motor_power_w.to_string(),
            "hydraulic_efficiency".into(),
            a.hydraulic_efficiency.to_string(),
        ],
        SheetAttributes::HeatGenerator(a) => vec![
            "210".into(),
            "nominal_heat_output_w".into(),
            a.nominal_heat_output_w.to_string(),
            "fuel_type".into(),
            a.fuel_type.clone(),
            "flow_temp_max_c".into(),
            a.flow_temp_max_c.to_string(),
            "return_temp_min_c".into(),
            a.return_temp_min_c.to_string(),
        ],
        SheetAttributes::Generic(g) => {
            let mut fields = vec!["210".into()];
            for e in &g.entries {
                fields.push(e.key.clone());
                fields.push(e.value.clone());
            }
            fields
        },
    };
    if let Some(r) = product.records.iter_mut().find(|r| r.family.0 == RecordFamilyId::R210) {
        r.fields = fields;
    } else {
        product.records.push(NativeRecord {
            family: RecordFamilyId(RecordFamilyId::R210.to_string()),
            fields,
            extensions: ExtensionBag::default(),
        });
    }
    if product.id.is_empty() {
        product.id = product.identity.article_number.clone();
    }
}


/// 🔤️ Parse semicolon-delimited native VDI 3805 text into a typed catalogue.
pub fn parse_native_text(text: &str, limits: SecurityLimits) -> Result<ManufacturerCatalog, NormError> {
    limits.validate_text(text)?;
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header_line = lines.next().ok_or(NormError::IncompleteInput { field: "header".into() })?;
    let header_fields: Vec<&str> = header_line.split(';').collect();
    let (header_version, manufacturer, bsn_raw, created, charset, record_count_raw) = if header_fields.first() == Some(&"010") {
        if header_fields.len() < 6 {
            return Err(NormError::IncompleteInput { field: "header_fields".into() });
        }
        (header_fields[1], header_fields[2], header_fields[3], header_fields.get(4).copied().unwrap_or(""), header_fields.get(5).copied().unwrap_or("UTF-8"), header_fields.get(6).copied().unwrap_or("0"))
    } else {
        if header_fields.len() < 5 {
            return Err(NormError::IncompleteInput { field: "header_fields".into() });
        }
        (header_fields[0], header_fields[1], header_fields[2], header_fields.get(3).copied().unwrap_or(""), "UTF-8", header_fields[4])
    };
    let bsn = BuildingSystemNumber::parse(bsn_raw)?;
    let record_count: u32 = record_count_raw.parse().map_err(|_| NormError::InvalidValue { field: "record_count".into(), reason: "numeric expected".into() })?;
    let mut products: Vec<CatalogueProduct> = Vec::new();
    let mut orphan_records: Vec<NativeRecord> = Vec::new();
    let mut actual_records: u32 = 0;
    for line in lines {
        if actual_records as usize >= limits.max_records {
            return Err(NormError::InvalidValue { field: "records".into(), reason: "too many records".into() });
        }
        let fields: Vec<String> = line.split(';').map(|s| s.to_string()).collect();
        if fields.is_empty() {
            continue;
        }
        if fields[0].len() > limits.max_field_length {
            return Err(NormError::InvalidValue { field: "field_length".into(), reason: "field exceeds limit".into() });
        }
        actual_records += 1;
        let family = RecordFamilyId(fields[0].clone());
        let record = NativeRecord { family: family.clone(), fields: fields.clone(), extensions: ExtensionBag::default() };
        match fields[0].as_str() {
            "100" if fields.len() >= 4 => {
                let article_number = fields.get(3).cloned().unwrap_or_default();
                let identity = ProductIdentity {
                    manufacturer_code: fields.get(1).cloned().unwrap_or_default(),
                    product_group: fields.get(2).cloned().unwrap_or_default(),
                    article_number: article_number.clone(),
                };
                let sheet_no: u16 = fields.get(4).and_then(|s| s.parse().ok()).unwrap_or(2);
                products.push(CatalogueProduct {
                    id: article_number.clone(),
                    identity,
                    title: bilingual("Produkt", "Product"),
                    sheet: SheetId(sheet_no),
                    records: vec![record],
                    configuration: Configuration {
                        id: format!("cfg.{}", article_number),
                        attributes: SheetAttributes::Generic(GenericAttributes::default()),
                        geometry_ref: None,
                        function_refs: Vec::new(),
                    },
                    accessories: Vec::new(),
                    components: Vec::new(),
                    extensions: ExtensionBag::default(),
                });
            }
            "110" => {
                if let Some(product) = products.last_mut() {
                    if let Some(cfg_id) = fields.get(1) {
                        product.configuration.id = cfg_id.clone();
                    }
                    if let Some(geom) = fields.get(2).filter(|s| !s.is_empty()) {
                        product.configuration.geometry_ref = Some(geom.clone());
                    }
                    product.configuration.function_refs = fields.iter().skip(3).filter(|s| !s.is_empty()).cloned().collect();
                    product.records.push(record);
                } else {
                    orphan_records.push(record);
                }
            }
            "200" | "210" | "400" | "700" | "900" => {
                if let Some(product) = products.last_mut() {
                    product.records.push(record);
                } else {
                    orphan_records.push(record);
                }
            }
            _ => {
                if let Some(product) = products.last_mut() {
                    product.records.push(record);
                } else {
                    orphan_records.push(record);
                }
            }
        }
    }
    for product in &mut products {
        product.configuration.attributes = attributes_from_records(product.sheet, &product.records);
        if let Some(de) = product.records.iter().find(|r| r.family.0 == "700").and_then(|r| r.fields.get(2)) {
            let en = product.records.iter().find(|r| r.family.0 == "700").and_then(|r| r.fields.get(4)).cloned().unwrap_or_else(|| de.clone());
            product.title = bilingual(de.clone(), en);
        }
    }
    if !orphan_records.is_empty() && products.is_empty() {
        return Err(NormError::IncompleteInput { field: "100".into() });
    }
    let _ = actual_records;
    let file = ManufacturerFile {
        header_version: header_version.into(),
        manufacturer: manufacturer.into(),
        building_system_number: bsn,
        created: created.into(),
        charset: charset.into(),
        record_count,
        extensions: ExtensionBag::default(),
    };
    Ok(ManufacturerCatalog { file, products, extensions: ExtensionBag::default() })
}

/// 🔤️ Serialize catalogue to semicolon-delimited native text (010-style header + typed records).
pub fn serialize_native_text(catalog: &ManufacturerCatalog) -> String {
    let f = &catalog.file;
    let mut out = format!("010;{};{};{};{};{};{}\n", f.header_version, f.manufacturer, f.building_system_number.render(), f.created, f.charset, f.record_count);
    for product in &catalog.products {
        let mut wrote_100 = false;
        for record in &product.records {
            out.push_str(&record.fields.join(";"));
            out.push('\n');
            if record.fields.first().is_some_and(|f| f == "100") {
                wrote_100 = true;
            }
        }
        if !wrote_100 {
            out.push_str(&format!("100;{};{};{};{}\n", product.identity.manufacturer_code, product.identity.product_group, product.identity.article_number, product.sheet.0));
        }
        if let Some(line) = attributes_to_native_line(&product.configuration.attributes) {
            out.push_str(&line);
            out.push('\n');
        }
    }
    out
}

fn attributes_to_native_fields(attributes: &SheetAttributes) -> Option<Vec<(String, String)>> {
    match attributes {
        SheetAttributes::ValveHeating(a) => Some(vec![
            ("dn".into(), a.dn.to_string()),
            ("kvs".into(), format!("{:.9}", a.kvs_m3_h())),
            ("pressure_class".into(), a.pressure_class.clone()),
            ("connection_type".into(), a.connection_type.clone()),
            ("authority_min".into(), format!("{}", a.authority_min)),
            ("authority_max".into(), format!("{}", a.authority_max)),
        ]),
        SheetAttributes::Radiator(a) => Some(vec![
            ("standard_output_w".into(), format!("{}", a.standard_output_w)),
            ("n".into(), format!("{}", a.heat_exponent_n)),
            ("length_m".into(), format!("{}", a.length_m)),
            ("height_m".into(), format!("{}", a.height_m)),
            ("depth_m".into(), format!("{}", a.depth_m)),
            ("connection_type".into(), a.connection_type.clone()),
        ]),
        SheetAttributes::PumpHeating(a) => Some(vec![
            ("dn_suction".into(), a.dn_suction.to_string()),
            ("dn_discharge".into(), a.dn_discharge.to_string()),
            ("q".into(), format!("{}", a.nominal_flow_m3_s)),
            ("h".into(), format!("{}", a.nominal_head_m)),
            ("p".into(), format!("{}", a.motor_power_w)),
            ("eta".into(), format!("{}", a.hydraulic_efficiency)),
        ]),
        SheetAttributes::HeatGenerator(a) => Some(vec![
            ("qn".into(), format!("{}", a.nominal_heat_output_w)),
            ("fuel_type".into(), a.fuel_type.clone()),
            ("flow_temp_max_c".into(), format!("{}", a.flow_temp_max_c)),
            ("return_temp_min_c".into(), format!("{}", a.return_temp_min_c)),
        ]),
        SheetAttributes::Generic(_) => None,
    }
}

fn attributes_to_native_line(attributes: &SheetAttributes) -> Option<String> {
    let fields = attributes_to_native_fields(attributes)?;
    let mut line = String::from("210");
    for (k, v) in fields {
        line.push(';');
        line.push_str(&k);
        line.push(';');
        line.push_str(&v);
    }
    Some(line)
}

fn count_native_records(catalog: &ManufacturerCatalog) -> u32 {
    catalog.products.iter().map(|p| p.records.len() as u32).sum()
}

/// ✅️ Part 1 structural validation over the full manufacturer dataset.
pub fn validate_structure(document: &Vdi3805Snapshot) -> Vec<Diagnostic> {
    let catalog = &document.catalog;
    let mut issues = Vec::new();
    if catalog.file.manufacturer.is_empty() {
        issues.push(Diagnostic::error("manufacturerFile.manufacturer", "missing manufacturer code"));
    }
    if catalog.file.charset.is_empty() {
        issues.push(Diagnostic::error("manufacturerFile.charset", "missing charset"));
    }
    if catalog.products.is_empty() {
        issues.push(Diagnostic::error("catalog.products", "empty product list"));
    }
    let actual = count_native_records(catalog);
    if catalog.file.record_count != actual {
        issues.push(Diagnostic::error("manufacturerFile.recordCount", format!("record_count {} != actual {}", catalog.file.record_count, actual)));
    }
    let article_numbers: BTreeSet<String> = catalog.products.iter().map(|p| p.identity.article_number.clone()).collect();
    for (pi, product) in catalog.products.iter().enumerate() {
        let base = format!("catalog.products[{pi}]");
        if product.identity.article_number.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.identity.articleNumber"), "missing article number"));
        }
        if product.identity.manufacturer_code.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.identity.manufacturerCode"), "missing manufacturer code"));
        }
        if product.identity.product_group.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.identity.productGroup"), "missing product group"));
        }
        if product.configuration.id.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.configuration.id"), "missing configuration id"));
        }
        let has_100 = product.records.iter().any(|r| r.family.0 == "100");
        if !has_100 {
            issues.push(Diagnostic::error(format!("{base}.records"), "missing mandatory record 100"));
        }
        for (ri, record) in product.records.iter().enumerate() {
            if let Some(min) = record_min_fields(record.family.0.as_str()) {
                if record.fields.len() < min {
                    issues.push(Diagnostic::error(format!("{base}.records[{ri}]"), format!("record {} needs ≥{min} fields, has {}", record.family.0, record.fields.len())));
                }
            } else {
                let known: BTreeSet<&str> = RecordFamilyId::all_known().iter().copied().collect();
                if !known.contains(record.family.0.as_str()) && !record.family.0.starts_with('9') {
                    issues.push(Diagnostic::error(format!("{base}.records[{ri}].family"), format!("unknown record family {}", record.family.0)));
                }
            }
        }
        if let Some(geom_ref) = &product.configuration.geometry_ref {
            if !document.geometry.contains_key(geom_ref) {
                issues.push(Diagnostic::error(format!("{base}.configuration.geometryRef"), format!("dangling geometry ref {geom_ref}")));
            }
            let has_200 = product.records.iter().any(|r| r.family.0 == "200");
            if !has_200 {
                issues.push(Diagnostic::error(format!("{base}.records"), "geometry_ref set but record 200 missing"));
            }
        }
        for (fi, fun_ref) in product.configuration.function_refs.iter().enumerate() {
            if !document.curves.contains_key(fun_ref) {
                issues.push(Diagnostic::error(format!("{base}.configuration.functionRefs[{fi}]"), format!("dangling curve ref {fun_ref}")));
            }
        }
        for (ai, link) in product.accessories.iter().enumerate() {
            if !article_numbers.contains(&link.accessory_id) {
                issues.push(Diagnostic::error(format!("{base}.accessories[{ai}].accessoryId"), format!("dangling accessory {}", link.accessory_id)));
            }
        }
        for (ci, link) in product.components.iter().enumerate() {
            if !article_numbers.contains(&link.component_id) {
                issues.push(Diagnostic::error(format!("{base}.components[{ci}].componentId"), format!("dangling component {}", link.component_id)));
            }
        }
        let title_locales: BTreeSet<&str> = product.title.iter().map(|t| t.locale.as_str()).collect();
        if !title_locales.contains("de") || !title_locales.contains("en") {
            issues.push(Diagnostic::error(format!("{base}.title"), "multilingual texts require de and en"));
        }
    }
    issues
}

/// 🔢️ Linear map between two scalar domains.
pub fn linear_map(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    if (x1 - x0).abs() < f64::EPSILON {
        return y0;
    }
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}

/// 🧪 Round-trip helpers retained for tests (not emitted into the user-facing report).
pub fn assert_json_round_trip(catalog: &ManufacturerCatalog) -> Result<(), String> {
    let json = crate::standards::v1::subsets::any::io::catalog_to_json(catalog).map_err(|e| e.to_string())?;
    let restored = crate::standards::v1::subsets::any::io::catalog_from_json(&json).map_err(|e| e.to_string())?;
    if restored.products.len() != catalog.products.len() {
        return Err("catalog JSON product count mismatch".into());
    }
    Ok(())
}

pub fn assert_native_round_trip(catalog: &ManufacturerCatalog, limits: SecurityLimits) -> Result<(), String> {
    let native = serialize_native_text(catalog);
    let parsed = parse_native_text(&native, limits).map_err(|e| e.to_string())?;
    if parsed.products.len() != catalog.products.len() {
        return Err("native text product count mismatch".into());
    }
    Ok(())
}
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
