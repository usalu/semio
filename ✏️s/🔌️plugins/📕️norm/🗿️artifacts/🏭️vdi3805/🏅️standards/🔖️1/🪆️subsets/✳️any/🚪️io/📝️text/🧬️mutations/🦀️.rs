//! ⚡️ VDI 3805 artifact — hand-rolled `OpText`/`OpBinary` for `Vdi3805Mutation`.
//! `#[derive(dsl_derive::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../../../🧬️schema/🧬️mutations/🦀️.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted
//! here, one keyword per semantic verb, grammar `keyword key1=value1 key2=value2 ...`. Structured
//! payload fields (manufacturer file header, security limits, catalogue products, geometry,
//! curves, ...) round-trip through a quoted JSON string — every one of them already derives
//! `Serialize`/`Deserialize`, so a second handcrafted grammar per structured type would just
//! duplicate that losslessly.

use crate::artifact_schema::mutations::Vdi3805Mutation;

use crate::artifact_schema::mutations::{
    add_geometry_connection::AddGeometryConnection, change_correction_as_of::ChangeCorrectionAsOf, change_edition_profile::ChangeEditionProfile, change_strict_mode::ChangeStrictMode, add_curve::AddCurve, add_geometry::AddGeometry,
    add_product::AddProduct, remove_curve::RemoveCurve, remove_geometry::RemoveGeometry, remove_product::RemoveProduct, remove_edition_profile::RemoveEditionProfile, remove_geometry_connection::RemoveGeometryConnection,
    rename_product::RenameProduct, change_curve_points::ChangeCurvePoints, change_geometry_parameters::ChangeGeometryParameters, change_product_configuration::ChangeProductConfiguration, resize_geometry::ResizeGeometry,
    change_manufacturer_file::ChangeManufacturerFile,
    change_limits::ChangeLimits,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other scalar's text form stays space-free and tokenizable by [`tokenize_args`].
fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
fn dec_str(s: &str) -> Result<String, String> {
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {s:?}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(out)
}
fn enc_bool(v: bool) -> String {
    v.to_string()
}
fn dec_bool(s: &str) -> Result<bool, String> {
    s.parse().map_err(|e: std::str::ParseBoolError| e.to_string())
}
fn enc_opt_usize(v: &Option<usize>) -> String {
    match v {
        Some(index) => index.to_string(),
        None => "-".to_string(),
    }
}
fn dec_opt_usize(s: &str) -> Result<Option<usize>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(s.parse().map_err(|e: std::num::ParseIntError| e.to_string())?))
    }
}
/// 🧬️ Every structured payload field (manufacturer file header, security limits, catalogue
/// products, geometry, curves, ...) already derives `ToValue`/`FromValue` — a quoted JSON
/// string reuses that losslessly instead of a second handcrafted grammar per type.
fn enc_json<T: semio_framework_value::ToValue>(value: &T) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(value))
}
fn dec_json<T: semio_framework_value::FromValue>(s: &str) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value —
/// needed because names/JSON payloads may contain spaces.
fn tokenize_args(rest: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                current.push(c);
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️OpText
fn print_vdi3805_mutation(mutation: &Vdi3805Mutation) -> String {
    match mutation {
        Vdi3805Mutation::ChangeManufacturerFile(p) => format!("change-manufacturer-file new-manufacturer-file={}", enc_json(&p.new_manufacturer_file)),
        Vdi3805Mutation::ChangeLimits(p) => format!("change-limits new-limits={}", enc_json(&p.new_limits)),
        Vdi3805Mutation::ChangeCorrectionAsOf(p) => format!("change-correction-as-of new-correction-as-of={}", enc_json(&p.new_correction_as_of)),
        Vdi3805Mutation::ChangeStrictMode(p) => format!("change-strict-mode new-strict-mode={}", enc_bool(p.new_strict_mode)),
        Vdi3805Mutation::ChangeEditionProfile(p) => format!("change-edition-profile sheet={} new-choice={}", enc_str(&p.sheet), enc_json(&p.new_choice)),
        Vdi3805Mutation::RemoveEditionProfile(p) => format!("remove-edition-profile sheet={}", enc_str(&p.sheet)),
        Vdi3805Mutation::AddProduct(p) => format!("add-product product={} index={}", enc_json(&p.product), enc_opt_usize(&p.index)),
        Vdi3805Mutation::RemoveProduct(p) => format!("remove-product id={}", enc_str(&p.id)),
        Vdi3805Mutation::RenameProduct(p) => format!("rename-product id={} new-title={}", enc_str(&p.id), enc_json(&p.new_title)),
        Vdi3805Mutation::ChangeProductConfiguration(p) => format!("change-product-configuration id={} new-configuration={}", enc_str(&p.id), enc_json(&p.new_configuration)),
        Vdi3805Mutation::AddGeometry(p) => format!("add-geometry geometry={}", enc_json(&p.geometry)),
        Vdi3805Mutation::RemoveGeometry(p) => format!("remove-geometry id={}", enc_str(&p.id)),
        Vdi3805Mutation::ResizeGeometry(p) => format!("resize-geometry id={} new-bbox={}", enc_str(&p.id), enc_json(&p.new_bbox)),
        Vdi3805Mutation::AddGeometryConnection(p) => format!("add-geometry-connection id={} connection={} index={}", enc_str(&p.id), enc_json(&p.connection), enc_opt_usize(&p.index)),
        Vdi3805Mutation::RemoveGeometryConnection(p) => format!("remove-geometry-connection id={} connection-id={}", enc_str(&p.id), enc_str(&p.connection_id)),
        Vdi3805Mutation::ChangeGeometryParameters(p) => format!("change-geometry-parameters id={} new-parameters={}", enc_str(&p.id), enc_json(&p.new_parameters)),
        Vdi3805Mutation::AddCurve(p) => format!("add-curve curve={}", enc_json(&p.curve)),
        Vdi3805Mutation::RemoveCurve(p) => format!("remove-curve id={}", enc_str(&p.id)),
        Vdi3805Mutation::ChangeCurvePoints(p) => format!("change-curve-points id={} new-points={}", enc_str(&p.id), enc_json(&p.new_points)),
    }
}

fn parse_vdi3805_mutation(line: &str) -> Result<Vdi3805Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("vdi3805 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-manufacturer-file" => Ok(Vdi3805Mutation::ChangeManufacturerFile(ChangeManufacturerFile { new_manufacturer_file: dec_json(&arg("new-manufacturer-file")?)? })),
        "change-correction-as-of" => Ok(Vdi3805Mutation::ChangeCorrectionAsOf(ChangeCorrectionAsOf { new_correction_as_of: dec_json(&arg("new-correction-as-of")?)? })),
        "change-strict-mode" => Ok(Vdi3805Mutation::ChangeStrictMode(ChangeStrictMode { new_strict_mode: dec_bool(&arg("new-strict-mode")?)? })),
        "change-edition-profile" => Ok(Vdi3805Mutation::ChangeEditionProfile(ChangeEditionProfile { sheet: dec_str(&arg("sheet")?)?, new_choice: dec_json(&arg("new-choice")?)? })),
        "remove-edition-profile" => Ok(Vdi3805Mutation::RemoveEditionProfile(RemoveEditionProfile { sheet: dec_str(&arg("sheet")?)? })),
        "add-product" => Ok(Vdi3805Mutation::AddProduct(AddProduct { product: dec_json(&arg("product")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "remove-product" => Ok(Vdi3805Mutation::RemoveProduct(RemoveProduct { id: dec_str(&arg("id")?)? })),
        "rename-product" => Ok(Vdi3805Mutation::RenameProduct(RenameProduct { id: dec_str(&arg("id")?)?, new_title: dec_json(&arg("new-title")?)? })),
        "change-product-configuration" => Ok(Vdi3805Mutation::ChangeProductConfiguration(ChangeProductConfiguration { id: dec_str(&arg("id")?)?, new_configuration: dec_json(&arg("new-configuration")?)? })),
        "add-geometry" => Ok(Vdi3805Mutation::AddGeometry(AddGeometry { geometry: dec_json(&arg("geometry")?)? })),
        "remove-geometry" => Ok(Vdi3805Mutation::RemoveGeometry(RemoveGeometry { id: dec_str(&arg("id")?)? })),
        "resize-geometry" => Ok(Vdi3805Mutation::ResizeGeometry(ResizeGeometry { id: dec_str(&arg("id")?)?, new_bbox: dec_json(&arg("new-bbox")?)? })),
        "add-geometry-connection" => Ok(Vdi3805Mutation::AddGeometryConnection(AddGeometryConnection { id: dec_str(&arg("id")?)?, connection: dec_json(&arg("connection")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "remove-geometry-connection" => Ok(Vdi3805Mutation::RemoveGeometryConnection(RemoveGeometryConnection { id: dec_str(&arg("id")?)?, connection_id: dec_str(&arg("connection-id")?)? })),
        "change-geometry-parameters" => Ok(Vdi3805Mutation::ChangeGeometryParameters(ChangeGeometryParameters { id: dec_str(&arg("id")?)?, new_parameters: dec_json(&arg("new-parameters")?)? })),
        "add-curve" => Ok(Vdi3805Mutation::AddCurve(AddCurve { curve: dec_json(&arg("curve")?)? })),
        "remove-curve" => Ok(Vdi3805Mutation::RemoveCurve(RemoveCurve { id: dec_str(&arg("id")?)? })),
        "change-curve-points" => Ok(Vdi3805Mutation::ChangeCurvePoints(ChangeCurvePoints { id: dec_str(&arg("id")?)?, new_points: dec_json(&arg("new-points")?)? })),
        other => Err(format!("vdi3805 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for Vdi3805Mutation {
    fn print_op(&self) -> String {
        print_vdi3805_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_vdi3805_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
/// 🎞️ Every variant's binary form is `tag u8 | json-string-per-field`; the JSON-per-field
/// consolidation used by `OpText` above applies equally here — one `write_str_bin` per field
/// regardless of that field's own structural complexity.









//#region 🏷️WireTags
/// 🏷️ Op tags of `Vdi3805Mutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.




















//#endregion 🏷️WireTags


//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<Vdi3805Mutation> {
    use crate::{GenericAttributes, 
        BoundingBox, CatalogueProduct, CharacteristicCurve, Configuration, ConnectionPoint, CurvePoint, EditionId, EditionProfileChoice, ExtensionBag, ParametricGeometry, ProductIdentity, SheetAttributes, SheetId, ValveHeatingAttributes,
        VdiQuantityKind, VdiUnit,
    };

    let product = CatalogueProduct {
        id: "VLV-NEW".into(),
        identity: ProductIdentity { manufacturer_code: "DEMO".into(), product_group: "HV".into(), article_number: "VLV-NEW".into() },
        title: crate::bilingual("Neu", "New"),
        sheet: SheetId(3),
        records: Vec::new(),
        configuration: Configuration { id: "cfg.new".into(), attributes: SheetAttributes::Generic(GenericAttributes::default()), geometry_ref: None, function_refs: Vec::new() },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    };
    let geometry = ParametricGeometry { id: "geom.new".into(), bbox: BoundingBox::from_size(1.0, 1.0, 1.0), connections: Vec::new(), parameters: std::collections::BTreeMap::new() };
    let curve = CharacteristicCurve { id: "curve.new".into(), x_unit: VdiUnit::delta("%", VdiQuantityKind::Dimensionless, 0.01), y_unit: VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0), points: vec![CurvePoint { x: 0.0, y: 0.0 }] };

    vec![
        Vdi3805Mutation::ChangeManufacturerFile(ChangeManufacturerFile { new_manufacturer_file: crate::reference_fixture().catalog.file }),
        Vdi3805Mutation::ChangeCorrectionAsOf(ChangeCorrectionAsOf { new_correction_as_of: EditionId::new(2025, 3) }),
        Vdi3805Mutation::ChangeStrictMode(ChangeStrictMode { new_strict_mode: true }),
        Vdi3805Mutation::ChangeEditionProfile(ChangeEditionProfile { sheet: "8".into(), new_choice: EditionProfileChoice::Legacy }),
        Vdi3805Mutation::RemoveEditionProfile(RemoveEditionProfile { sheet: "8".into() }),
        Vdi3805Mutation::AddProduct(AddProduct { product: product.clone(), index: Some(0) }),
        Vdi3805Mutation::RemoveProduct(RemoveProduct { id: "VLV-50-001".into() }),
        Vdi3805Mutation::RenameProduct(RenameProduct { id: "VLV-50-001".into(), new_title: crate::bilingual("Umbenannt", "Renamed") }),
        Vdi3805Mutation::ChangeProductConfiguration(ChangeProductConfiguration { id: "VLV-50-001".into(), new_configuration: product.configuration.clone() }),
        Vdi3805Mutation::AddGeometry(AddGeometry { geometry: geometry.clone() }),
        Vdi3805Mutation::RemoveGeometry(RemoveGeometry { id: "geom-valve-50".into() }),
        Vdi3805Mutation::ResizeGeometry(ResizeGeometry { id: "geom-valve-50".into(), new_bbox: BoundingBox::from_size(2.0, 2.0, 2.0) }),
        Vdi3805Mutation::AddGeometryConnection(AddGeometryConnection {
            id: "geom-valve-50".into(),
            connection: ConnectionPoint { id: "mid".into(), medium: "water".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], diameter_mm: Some(25.0) },
            index: None,
        }),
        Vdi3805Mutation::RemoveGeometryConnection(RemoveGeometryConnection { id: "geom-valve-50".into(), connection_id: "in".into() }),
        Vdi3805Mutation::ChangeGeometryParameters(ChangeGeometryParameters { id: "geom-valve-50".into(), new_parameters: std::collections::BTreeMap::from([("scale".to_string(), 2.0)]) }),
        Vdi3805Mutation::AddCurve(AddCurve { curve: curve.clone() }),
        Vdi3805Mutation::RemoveCurve(RemoveCurve { id: "curve-kvs".into() }),
        Vdi3805Mutation::ChangeCurvePoints(ChangeCurvePoints { id: "curve-kvs".into(), new_points: vec![CurvePoint { x: 0.0, y: 0.0 }, CurvePoint { x: 100.0, y: 9.0 }] }),
        Vdi3805Mutation::AddProduct(AddProduct {
            product: CatalogueProduct {
                configuration: Configuration {
                    id: "cfg.dn".into(),
                    attributes: SheetAttributes::ValveHeating(ValveHeatingAttributes::from_kvs_m3_h(80, 8.0, "PN16", "flange", 0.3, 0.7)),
                    geometry_ref: None,
                    function_refs: Vec::new(),
                },
                ..product
            },
            index: None,
        }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{CatalogIndexEntry, CatalogueProduct, SheetAttributes, Vdi3805Diff, Vdi3805Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::add_geometry_connection;
use crate::standards::v1::subsets::any::schema::mutations::change_correction_as_of;
use crate::standards::v1::subsets::any::schema::mutations::change_edition_profile;
use crate::standards::v1::subsets::any::schema::mutations::change_strict_mode;
use crate::standards::v1::subsets::any::schema::mutations::add_curve;
use crate::standards::v1::subsets::any::schema::mutations::add_geometry;
use crate::standards::v1::subsets::any::schema::mutations::add_product;
use crate::standards::v1::subsets::any::schema::mutations::remove_curve;
use crate::standards::v1::subsets::any::schema::mutations::remove_geometry;
use crate::standards::v1::subsets::any::schema::mutations::remove_product;
use crate::standards::v1::subsets::any::schema::mutations::remove_edition_profile;
use crate::standards::v1::subsets::any::schema::mutations::remove_geometry_connection;
use crate::standards::v1::subsets::any::schema::mutations::rename_product;
use crate::standards::v1::subsets::any::schema::mutations::change_curve_points;
use crate::standards::v1::subsets::any::schema::mutations::change_geometry_parameters;
use crate::standards::v1::subsets::any::schema::mutations::change_product_configuration;
use crate::standards::v1::subsets::any::schema::mutations::resize_geometry;
use crate::standards::v1::subsets::any::schema::mutations::change_manufacturer_file;
use crate::standards::v1::subsets::any::schema::mutations::change_limits;

/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json`
/// specification vectors carry — into a real [`Vdi3805Mutation`]. The generated test host of
/// `../../../../../🧪️tests/🏭️mutate-vdi3805-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_vdi3805_mutation_json(text: &str) -> Result<Vdi3805Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
