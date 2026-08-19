//! ⚡️ DIN 4108 artifact — hand-rolled `OpText`/`OpBinary` for `Din4108Mutation`.
//! `#[derive(dsl_derive::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../🦀️component.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted
//! here, one keyword per semantic verb, grammar `keyword key1=value1 key2=value2 ...`. Every field
//! (scalar or structured) already derives `Serialize`/`Deserialize`, so it round-trips through a
//! quoted JSON atom uniformly — a second handcrafted grammar per Rust primitive type would just
//! duplicate that losslessly (same rationale ISO 16757's sibling facet documents for its
//! structured-only fields, applied uniformly here given this facet's field-count).

pub use crate::artifacts::din4108::schema::mutations::Din4108Mutation;

use crate::artifacts::din4108::schema::mutations::{
    change_airtightness_class::mutation::ChangeAirtightnessClass, change_airtightness_n50::mutation::ChangeAirtightnessN50, change_application_type::mutation::ChangeApplicationType, change_bb2_details_conform::mutation::ChangeBb2DetailsConform,
    change_catalog_id::mutation::ChangeCatalogId, change_category::mutation::ChangeCategory, change_climate::mutation::ChangeClimate, change_declared_application_class::mutation::ChangeDeclaredApplicationClass,
    change_envelope_area_m2::mutation::ChangeEnvelopeAreaM2, change_irradiance_w_m2::mutation::ChangeIrradianceWM2, change_layer_lambda::mutation::ChangeLayerLambda, change_layer_thickness::mutation::ChangeLayerThickness,
    change_material_id::mutation::ChangeMaterialId, change_moisture_mu_exterior::mutation::ChangeMoistureMuExterior, change_moisture_mu_interior::mutation::ChangeMoistureMuInterior, change_psi_times_l_sum::mutation::ChangePsiTimesLSum,
    change_rh_int::mutation::ChangeRhInt, change_solar_absorptance::mutation::ChangeSolarAbsorptance, change_t_int_c::mutation::ChangeTIntC, insert_layer::mutation::InsertLayer, remove_layer::mutation::RemoveLayer,
    reorder_layers::mutation::ReorderLayers,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other token stays space-free and tokenizable by [`tokenize_args`].
async fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
async fn dec_str(s: &str) -> Result<String, String> {
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
/// 🧬️ Every payload field already derives `Serialize`/`Deserialize` — a quoted JSON atom reuses
/// that losslessly instead of a second handcrafted grammar per field type.
async fn enc_json<T: serde::Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("din4108 mutation payload field always serializes"))
}
async fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value —
/// needed because string/JSON payloads may contain spaces.
async fn tokenize_args(rest: &str) -> Vec<String> {
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
async fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️OpText
async fn print_din4108_mutation(mutation: &Din4108Mutation) -> String {
    match mutation {
        Din4108Mutation::ChangeCategory(p) => format!("change-category new-category={}", enc_json(&p.new_category)),
        Din4108Mutation::ChangeClimate(p) => format!("change-climate new-climate={}", enc_json(&p.new_climate)),
        Din4108Mutation::ChangeAirtightnessN50(p) => format!("change-airtightness-n50 new-airtightness-n50={}", enc_json(&p.new_airtightness_n50)),
        Din4108Mutation::ChangePsiTimesLSum(p) => format!("change-psi-times-l-sum new-psi-times-l-sum={}", enc_json(&p.new_psi_times_l_sum)),
        Din4108Mutation::ChangeRhInt(p) => format!("change-rh-int new-rh-int={}", enc_json(&p.new_rh_int)),
        Din4108Mutation::ChangeCatalogId(p) => format!("change-catalog-id new-catalog-id={}", enc_json(&p.new_catalog_id)),
        Din4108Mutation::ChangeMaterialId(p) => format!("change-material-id new-material-id={}", enc_json(&p.new_material_id)),
        Din4108Mutation::ChangeAirtightnessClass(p) => format!("change-airtightness-class new-airtightness-class={}", enc_json(&p.new_airtightness_class)),
        Din4108Mutation::ChangeTIntC(p) => format!("change-t-int-c new-t-int-c={}", enc_json(&p.new_t_int_c)),
        Din4108Mutation::ChangeSolarAbsorptance(p) => format!("change-solar-absorptance new-solar-absorptance={}", enc_json(&p.new_solar_absorptance)),
        Din4108Mutation::ChangeIrradianceWM2(p) => format!("change-irradiance-wm2 new-irradiance-w-m2={}", enc_json(&p.new_irradiance_w_m2)),
        Din4108Mutation::ChangeMoistureMuExterior(p) => format!("change-moisture-mu-exterior new-moisture-mu-exterior={}", enc_json(&p.new_moisture_mu_exterior)),
        Din4108Mutation::ChangeMoistureMuInterior(p) => format!("change-moisture-mu-interior new-moisture-mu-interior={}", enc_json(&p.new_moisture_mu_interior)),
        Din4108Mutation::ChangeEnvelopeAreaM2(p) => format!("change-envelope-area-m2 new-envelope-area-m2={}", enc_json(&p.new_envelope_area_m2)),
        Din4108Mutation::ChangeBb2DetailsConform(p) => format!("change-bb2-details-conform new-bb2-details-conform={}", enc_json(&p.new_bb2_details_conform)),
        Din4108Mutation::ChangeApplicationType(p) => format!("change-application-type new-application-type={}", enc_json(&p.new_application_type)),
        Din4108Mutation::ChangeDeclaredApplicationClass(p) => format!("change-declared-application-class new-declared-application-class={}", enc_json(&p.new_declared_application_class)),
        Din4108Mutation::InsertLayer(p) => format!("insert-layer index={} layer={}", enc_json(&p.index), enc_json(&p.layer)),
        Din4108Mutation::RemoveLayer(p) => format!("remove-layer index={}", enc_json(&p.index)),
        Din4108Mutation::ReorderLayers(p) => format!("reorder-layers from={} to={}", enc_json(&p.from), enc_json(&p.to)),
        Din4108Mutation::ChangeLayerThickness(p) => format!("change-layer-thickness index={} new-thickness-m={}", enc_json(&p.index), enc_json(&p.new_thickness_m)),
        Din4108Mutation::ChangeLayerLambda(p) => format!("change-layer-lambda index={} new-lambda-w-mk={}", enc_json(&p.index), enc_json(&p.new_lambda_w_mk)),
    }
}

async fn parse_din4108_mutation(line: &str) -> Result<Din4108Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("din4108 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-category" => Ok(Din4108Mutation::ChangeCategory(ChangeCategory { new_category: dec_json(&arg("new-category")?)? })),
        "change-climate" => Ok(Din4108Mutation::ChangeClimate(ChangeClimate { new_climate: dec_json(&arg("new-climate")?)? })),
        "change-airtightness-n50" => Ok(Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50: dec_json(&arg("new-airtightness-n50")?)? })),
        "change-psi-times-l-sum" => Ok(Din4108Mutation::ChangePsiTimesLSum(ChangePsiTimesLSum { new_psi_times_l_sum: dec_json(&arg("new-psi-times-l-sum")?)? })),
        "change-rh-int" => Ok(Din4108Mutation::ChangeRhInt(ChangeRhInt { new_rh_int: dec_json(&arg("new-rh-int")?)? })),
        "change-catalog-id" => Ok(Din4108Mutation::ChangeCatalogId(ChangeCatalogId { new_catalog_id: dec_json(&arg("new-catalog-id")?)? })),
        "change-material-id" => Ok(Din4108Mutation::ChangeMaterialId(ChangeMaterialId { new_material_id: dec_json(&arg("new-material-id")?)? })),
        "change-airtightness-class" => Ok(Din4108Mutation::ChangeAirtightnessClass(ChangeAirtightnessClass { new_airtightness_class: dec_json(&arg("new-airtightness-class")?)? })),
        "change-t-int-c" => Ok(Din4108Mutation::ChangeTIntC(ChangeTIntC { new_t_int_c: dec_json(&arg("new-t-int-c")?)? })),
        "change-solar-absorptance" => Ok(Din4108Mutation::ChangeSolarAbsorptance(ChangeSolarAbsorptance { new_solar_absorptance: dec_json(&arg("new-solar-absorptance")?)? })),
        "change-irradiance-wm2" => Ok(Din4108Mutation::ChangeIrradianceWM2(ChangeIrradianceWM2 { new_irradiance_w_m2: dec_json(&arg("new-irradiance-w-m2")?)? })),
        "change-moisture-mu-exterior" => Ok(Din4108Mutation::ChangeMoistureMuExterior(ChangeMoistureMuExterior { new_moisture_mu_exterior: dec_json(&arg("new-moisture-mu-exterior")?)? })),
        "change-moisture-mu-interior" => Ok(Din4108Mutation::ChangeMoistureMuInterior(ChangeMoistureMuInterior { new_moisture_mu_interior: dec_json(&arg("new-moisture-mu-interior")?)? })),
        "change-envelope-area-m2" => Ok(Din4108Mutation::ChangeEnvelopeAreaM2(ChangeEnvelopeAreaM2 { new_envelope_area_m2: dec_json(&arg("new-envelope-area-m2")?)? })),
        "change-bb2-details-conform" => Ok(Din4108Mutation::ChangeBb2DetailsConform(ChangeBb2DetailsConform { new_bb2_details_conform: dec_json(&arg("new-bb2-details-conform")?)? })),
        "change-application-type" => Ok(Din4108Mutation::ChangeApplicationType(ChangeApplicationType { new_application_type: dec_json(&arg("new-application-type")?)? })),
        "change-declared-application-class" => Ok(Din4108Mutation::ChangeDeclaredApplicationClass(ChangeDeclaredApplicationClass { new_declared_application_class: dec_json(&arg("new-declared-application-class")?)? })),
        "insert-layer" => Ok(Din4108Mutation::InsertLayer(InsertLayer { index: dec_json(&arg("index")?)?, layer: dec_json(&arg("layer")?)? })),
        "remove-layer" => Ok(Din4108Mutation::RemoveLayer(RemoveLayer { index: dec_json(&arg("index")?)? })),
        "reorder-layers" => Ok(Din4108Mutation::ReorderLayers(ReorderLayers { from: dec_json(&arg("from")?)?, to: dec_json(&arg("to")?)? })),
        "change-layer-thickness" => Ok(Din4108Mutation::ChangeLayerThickness(ChangeLayerThickness { index: dec_json(&arg("index")?)?, new_thickness_m: dec_json(&arg("new-thickness-m")?)? })),
        "change-layer-lambda" => Ok(Din4108Mutation::ChangeLayerLambda(ChangeLayerLambda { index: dec_json(&arg("index")?)?, new_lambda_w_mk: dec_json(&arg("new-lambda-w-mk")?)? })),
        other => Err(format!("din4108 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for Din4108Mutation {
    async fn print_op(&self) -> String {
        print_din4108_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_din4108_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
/// 🎞️ Every variant's binary form is `tag u8 | json-string-per-field` — the JSON-per-field
/// consolidation used by `OpText` above applies equally here.
async fn write_json_bin<T: serde::Serialize>(out: &mut Vec<u8>, value: &T) {
    let bytes = serde_json::to_string(value).expect("din4108 mutation payload field always serializes");
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes.as_bytes());
}
async fn read_json_bin<T: serde::de::DeserializeOwned>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    serde_json::from_str(text).map_err(|e| e.to_string())
}

impl protocol::OpBinary for Din4108Mutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            Din4108Mutation::ChangeCategory(_) => 0,
            Din4108Mutation::ChangeClimate(_) => 1,
            Din4108Mutation::ChangeAirtightnessN50(_) => 2,
            Din4108Mutation::ChangePsiTimesLSum(_) => 3,
            Din4108Mutation::ChangeRhInt(_) => 4,
            Din4108Mutation::ChangeCatalogId(_) => 5,
            Din4108Mutation::ChangeMaterialId(_) => 6,
            Din4108Mutation::ChangeAirtightnessClass(_) => 7,
            Din4108Mutation::ChangeTIntC(_) => 8,
            Din4108Mutation::ChangeSolarAbsorptance(_) => 9,
            Din4108Mutation::ChangeIrradianceWM2(_) => 10,
            Din4108Mutation::ChangeMoistureMuExterior(_) => 11,
            Din4108Mutation::ChangeMoistureMuInterior(_) => 12,
            Din4108Mutation::ChangeEnvelopeAreaM2(_) => 13,
            Din4108Mutation::ChangeBb2DetailsConform(_) => 14,
            Din4108Mutation::ChangeApplicationType(_) => 15,
            Din4108Mutation::ChangeDeclaredApplicationClass(_) => 16,
            Din4108Mutation::InsertLayer(_) => 17,
            Din4108Mutation::RemoveLayer(_) => 18,
            Din4108Mutation::ReorderLayers(_) => 19,
            Din4108Mutation::ChangeLayerThickness(_) => 20,
            Din4108Mutation::ChangeLayerLambda(_) => 21,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            Din4108Mutation::ChangeCategory(p) => write_json_bin(&mut out, &p.new_category),
            Din4108Mutation::ChangeClimate(p) => write_json_bin(&mut out, &p.new_climate),
            Din4108Mutation::ChangeAirtightnessN50(p) => write_json_bin(&mut out, &p.new_airtightness_n50),
            Din4108Mutation::ChangePsiTimesLSum(p) => write_json_bin(&mut out, &p.new_psi_times_l_sum),
            Din4108Mutation::ChangeRhInt(p) => write_json_bin(&mut out, &p.new_rh_int),
            Din4108Mutation::ChangeCatalogId(p) => write_json_bin(&mut out, &p.new_catalog_id),
            Din4108Mutation::ChangeMaterialId(p) => write_json_bin(&mut out, &p.new_material_id),
            Din4108Mutation::ChangeAirtightnessClass(p) => write_json_bin(&mut out, &p.new_airtightness_class),
            Din4108Mutation::ChangeTIntC(p) => write_json_bin(&mut out, &p.new_t_int_c),
            Din4108Mutation::ChangeSolarAbsorptance(p) => write_json_bin(&mut out, &p.new_solar_absorptance),
            Din4108Mutation::ChangeIrradianceWM2(p) => write_json_bin(&mut out, &p.new_irradiance_w_m2),
            Din4108Mutation::ChangeMoistureMuExterior(p) => write_json_bin(&mut out, &p.new_moisture_mu_exterior),
            Din4108Mutation::ChangeMoistureMuInterior(p) => write_json_bin(&mut out, &p.new_moisture_mu_interior),
            Din4108Mutation::ChangeEnvelopeAreaM2(p) => write_json_bin(&mut out, &p.new_envelope_area_m2),
            Din4108Mutation::ChangeBb2DetailsConform(p) => write_json_bin(&mut out, &p.new_bb2_details_conform),
            Din4108Mutation::ChangeApplicationType(p) => write_json_bin(&mut out, &p.new_application_type),
            Din4108Mutation::ChangeDeclaredApplicationClass(p) => write_json_bin(&mut out, &p.new_declared_application_class),
            Din4108Mutation::InsertLayer(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.layer);
            }
            Din4108Mutation::RemoveLayer(p) => write_json_bin(&mut out, &p.index),
            Din4108Mutation::ReorderLayers(p) => {
                write_json_bin(&mut out, &p.from);
                write_json_bin(&mut out, &p.to);
            }
            Din4108Mutation::ChangeLayerThickness(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.new_thickness_m);
            }
            Din4108Mutation::ChangeLayerLambda(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.new_lambda_w_mk);
            }
        }
        Ok(out)
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => {
                let new_category = read_json_bin(&mut reader).map_err(|e| malformed("new_category", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeCategory(ChangeCategory { new_category }))
            }
            1 => {
                let new_climate = read_json_bin(&mut reader).map_err(|e| malformed("new_climate", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeClimate(ChangeClimate { new_climate }))
            }
            2 => {
                let new_airtightness_n50 = read_json_bin(&mut reader).map_err(|e| malformed("new_airtightness_n50", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50 }))
            }
            3 => {
                let new_psi_times_l_sum = read_json_bin(&mut reader).map_err(|e| malformed("new_psi_times_l_sum", reader.position(), e))?;
                Ok(Din4108Mutation::ChangePsiTimesLSum(ChangePsiTimesLSum { new_psi_times_l_sum }))
            }
            4 => {
                let new_rh_int = read_json_bin(&mut reader).map_err(|e| malformed("new_rh_int", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeRhInt(ChangeRhInt { new_rh_int }))
            }
            5 => {
                let new_catalog_id = read_json_bin(&mut reader).map_err(|e| malformed("new_catalog_id", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeCatalogId(ChangeCatalogId { new_catalog_id }))
            }
            6 => {
                let new_material_id = read_json_bin(&mut reader).map_err(|e| malformed("new_material_id", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeMaterialId(ChangeMaterialId { new_material_id }))
            }
            7 => {
                let new_airtightness_class = read_json_bin(&mut reader).map_err(|e| malformed("new_airtightness_class", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeAirtightnessClass(ChangeAirtightnessClass { new_airtightness_class }))
            }
            8 => {
                let new_t_int_c = read_json_bin(&mut reader).map_err(|e| malformed("new_t_int_c", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeTIntC(ChangeTIntC { new_t_int_c }))
            }
            9 => {
                let new_solar_absorptance = read_json_bin(&mut reader).map_err(|e| malformed("new_solar_absorptance", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeSolarAbsorptance(ChangeSolarAbsorptance { new_solar_absorptance }))
            }
            10 => {
                let new_irradiance_w_m2 = read_json_bin(&mut reader).map_err(|e| malformed("new_irradiance_w_m2", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeIrradianceWM2(ChangeIrradianceWM2 { new_irradiance_w_m2 }))
            }
            11 => {
                let new_moisture_mu_exterior = read_json_bin(&mut reader).map_err(|e| malformed("new_moisture_mu_exterior", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeMoistureMuExterior(ChangeMoistureMuExterior { new_moisture_mu_exterior }))
            }
            12 => {
                let new_moisture_mu_interior = read_json_bin(&mut reader).map_err(|e| malformed("new_moisture_mu_interior", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeMoistureMuInterior(ChangeMoistureMuInterior { new_moisture_mu_interior }))
            }
            13 => {
                let new_envelope_area_m2 = read_json_bin(&mut reader).map_err(|e| malformed("new_envelope_area_m2", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeEnvelopeAreaM2(ChangeEnvelopeAreaM2 { new_envelope_area_m2 }))
            }
            14 => {
                let new_bb2_details_conform = read_json_bin(&mut reader).map_err(|e| malformed("new_bb2_details_conform", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeBb2DetailsConform(ChangeBb2DetailsConform { new_bb2_details_conform }))
            }
            15 => {
                let new_application_type = read_json_bin(&mut reader).map_err(|e| malformed("new_application_type", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeApplicationType(ChangeApplicationType { new_application_type }))
            }
            16 => {
                let new_declared_application_class = read_json_bin(&mut reader).map_err(|e| malformed("new_declared_application_class", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeDeclaredApplicationClass(ChangeDeclaredApplicationClass { new_declared_application_class }))
            }
            17 => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let layer = read_json_bin(&mut reader).map_err(|e| malformed("layer", reader.position(), e))?;
                Ok(Din4108Mutation::InsertLayer(InsertLayer { index, layer }))
            }
            18 => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(Din4108Mutation::RemoveLayer(RemoveLayer { index }))
            }
            19 => {
                let from = read_json_bin(&mut reader).map_err(|e| malformed("from", reader.position(), e))?;
                let to = read_json_bin(&mut reader).map_err(|e| malformed("to", reader.position(), e))?;
                Ok(Din4108Mutation::ReorderLayers(ReorderLayers { from, to }))
            }
            20 => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let new_thickness_m = read_json_bin(&mut reader).map_err(|e| malformed("new_thickness_m", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeLayerThickness(ChangeLayerThickness { index, new_thickness_m }))
            }
            21 => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let new_lambda_w_mk = read_json_bin(&mut reader).map_err(|e| malformed("new_lambda_w_mk", reader.position(), e))?;
                Ok(Din4108Mutation::ChangeLayerLambda(ChangeLayerLambda { index, new_lambda_w_mk }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<Din4108Mutation> {
    use crate::artifacts::din4108::LayerDocument;

    vec![
        Din4108Mutation::ChangeCategory(ChangeCategory { new_category: "nonresidential".into() }),
        Din4108Mutation::ChangeClimate(ChangeClimate { new_climate: crate::document::ClimateZoneDe::Zone3 }),
        Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50: 3.0 }),
        Din4108Mutation::ChangePsiTimesLSum(ChangePsiTimesLSum { new_psi_times_l_sum: 0.03 }),
        Din4108Mutation::ChangeRhInt(ChangeRhInt { new_rh_int: 0.55 }),
        Din4108Mutation::ChangeCatalogId(ChangeCatalogId { new_catalog_id: "AW-02".into() }),
        Din4108Mutation::ChangeMaterialId(ChangeMaterialId { new_material_id: "eps".into() }),
        Din4108Mutation::ChangeAirtightnessClass(ChangeAirtightnessClass { new_airtightness_class: "class1".into() }),
        Din4108Mutation::ChangeTIntC(ChangeTIntC { new_t_int_c: 21.0 }),
        Din4108Mutation::ChangeSolarAbsorptance(ChangeSolarAbsorptance { new_solar_absorptance: 0.7 }),
        Din4108Mutation::ChangeIrradianceWM2(ChangeIrradianceWM2 { new_irradiance_w_m2: 650.0 }),
        Din4108Mutation::ChangeMoistureMuExterior(ChangeMoistureMuExterior { new_moisture_mu_exterior: 18.0 }),
        Din4108Mutation::ChangeMoistureMuInterior(ChangeMoistureMuInterior { new_moisture_mu_interior: 1.5 }),
        Din4108Mutation::ChangeEnvelopeAreaM2(ChangeEnvelopeAreaM2 { new_envelope_area_m2: 120.0 }),
        Din4108Mutation::ChangeBb2DetailsConform(ChangeBb2DetailsConform { new_bb2_details_conform: false }),
        Din4108Mutation::ChangeApplicationType(ChangeApplicationType { new_application_type: "NDEO".into() }),
        Din4108Mutation::ChangeDeclaredApplicationClass(ChangeDeclaredApplicationClass { new_declared_application_class: "kh".into() }),
        Din4108Mutation::InsertLayer(InsertLayer { index: 1, layer: LayerDocument { thickness_m: 0.05, lambda_w_mk: 0.04 } }),
        Din4108Mutation::RemoveLayer(RemoveLayer { index: 1 }),
        Din4108Mutation::ReorderLayers(ReorderLayers { from: 0, to: 1 }),
        Din4108Mutation::ChangeLayerThickness(ChangeLayerThickness { index: 1, new_thickness_m: 0.3 }),
        Din4108Mutation::ChangeLayerLambda(ChangeLayerLambda { index: 1, new_lambda_w_mk: 0.9 }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{OpBinary, OpText};

    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <Din4108Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <Din4108Mutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
