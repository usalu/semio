//! ⚡️ DIN 4108 artifact — `OpText`/`OpBinary` for `Din4108Mutation`.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifact_schema::mutations::Din4108Mutation;
use crate::artifact_schema::mutations::{
    change_climate_zone,
    change_usage,
    change_t_int_c,
    change_rh_int,
    change_airtightness_n50,
    change_has_mechanical_ventilation,
    change_bb2_details_conform,
    insert_zone,
    remove_zone,
    change_zone_floor_area,
    change_zone_heaviness,
    change_zone_night_ventilation,
    insert_zone_window,
    remove_zone_window,
    change_zone_window_area,
    change_zone_window_g_value,
    change_zone_window_shading_fc,
    insert_element,
    remove_element,
    change_element_area,
    change_element_adjacent,
    change_element_kind,
    insert_layer,
    remove_layer,
    reorder_layers,
    change_layer_thickness,
    change_layer_lambda,
    change_layer_mu,
    change_layer_material_id,
    insert_thermal_bridge,
    remove_thermal_bridge,
    change_thermal_bridge_psi,
    change_thermal_bridge_length,
    change_element_orientation_deg,
    change_element_inclination_deg,
    change_element_delta_u_g,
    change_element_delta_u_f,
    change_element_delta_u_r,
    change_thermal_bridge_bb2_type,
    change_zone_window_orientation,
    change_zone_window_inclination_deg,
    change_layer_application_type,
    change_layer_compressive_class,
};
use protocol::OpText;

//#region 🔖️OpText
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Din4108MutationDsl {
    ChangeClimateZone {
        new_climate_zone: crate::document::ClimateZoneDe,
    },
    ChangeUsage {
        new_usage: String,
    },
    ChangeTIntC {
        new_t_int_c: f64,
    },
    ChangeRhInt {
        new_rh_int: f64,
    },
    ChangeAirtightnessN50 {
        new_airtightness_n50: f64,
    },
    ChangeHasMechanicalVentilation {
        new_has_mechanical_ventilation: bool,
    },
    ChangeBb2DetailsConform {
        new_bb2_details_conform: bool,
    },
    InsertZone {
        index: usize,
        #[dsl(block)]
        zone: crate::ThermalZone,
    },
    RemoveZone {
        index: usize,
    },
    ChangeZoneFloorArea {
        zone_id: String,
        new_floor_area_m2: f64,
    },
    ChangeZoneHeaviness {
        zone_id: String,
        new_heaviness: String,
    },
    ChangeZoneNightVentilation {
        zone_id: String,
        new_night_ventilation: String,
    },
    InsertZoneWindow {
        zone_id: String,
        index: usize,
        #[dsl(block)]
        window: crate::ZoneWindow,
    },
    RemoveZoneWindow {
        zone_id: String,
        index: usize,
    },
    ChangeZoneWindowArea {
        zone_id: String,
        window_id: String,
        new_area_m2: f64,
    },
    ChangeZoneWindowGValue {
        zone_id: String,
        window_id: String,
        new_g_value: f64,
    },
    ChangeZoneWindowShadingFc {
        zone_id: String,
        window_id: String,
        new_shading_fc: f64,
    },
    InsertElement {
        index: usize,
        #[dsl(block)]
        element: crate::EnvelopeElement,
    },
    RemoveElement {
        index: usize,
    },
    ChangeElementArea {
        element_id: String,
        new_area_m2: f64,
    },
    ChangeElementAdjacent {
        element_id: String,
        new_adjacent: String,
    },
    ChangeElementKind {
        element_id: String,
        new_kind: String,
    },
    InsertLayer {
        element_id: String,
        index: usize,
        #[dsl(block)]
        layer: crate::LayerDocument,
    },
    RemoveLayer {
        element_id: String,
        index: usize,
    },
    ReorderLayers {
        element_id: String,
        from: usize,
        to: usize,
    },
    ChangeLayerThickness {
        element_id: String,
        index: usize,
        new_thickness_m: f64,
    },
    ChangeLayerLambda {
        element_id: String,
        index: usize,
        new_lambda: f64,
    },
    ChangeLayerMu {
        element_id: String,
        index: usize,
        new_mu: f64,
    },
    ChangeLayerMaterialId {
        element_id: String,
        index: usize,
        new_material_id: String,
    },
    InsertThermalBridge {
        index: usize,
        #[dsl(block)]
        bridge: crate::ThermalBridge,
    },
    RemoveThermalBridge {
        index: usize,
    },
    ChangeThermalBridgePsi {
        bridge_id: String,
        new_psi: f64,
    },
    ChangeThermalBridgeLength {
        bridge_id: String,
        new_length_m: f64,
    },
    ChangeElementOrientationDeg {
        element_id: String,
        new_orientation_deg: f64,
    },
    ChangeElementInclinationDeg {
        element_id: String,
        new_inclination_deg: f64,
    },
    ChangeElementDeltaUg {
        element_id: String,
        new_delta_u_g: f64,
    },
    ChangeElementDeltaUf {
        element_id: String,
        new_delta_u_f: f64,
    },
    ChangeElementDeltaUr {
        element_id: String,
        new_delta_u_r: f64,
    },
    ChangeThermalBridgeBb2Type {
        bridge_id: String,
        new_bb2_type: String,
    },
    ChangeZoneWindowOrientation {
        zone_id: String,
        window_id: String,
        new_orientation: String,
    },
    ChangeZoneWindowInclinationDeg {
        zone_id: String,
        window_id: String,
        new_inclination_deg: f64,
    },
    ChangeLayerApplicationType {
        element_id: String,
        index: usize,
        new_application_type: String,
    },
    ChangeLayerCompressiveClass {
        element_id: String,
        index: usize,
        new_compressive_class: String,
    },
}

//#region 🔖️HandcraftedOpCodecs
impl OpText for Din4108MutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Din4108MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), bytes)
    }
}

impl OpText for Din4108Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(din4108_mutation_from_dsl(Din4108MutationDsl::parse_op(line)?))
    }
    fn print_op(&self) -> String {
        din4108_mutation_to_dsl(self).print_op()
    }
}

impl protocol::OpBinary for Din4108Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        din4108_mutation_to_dsl(self).encode_op()
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(din4108_mutation_from_dsl(Din4108MutationDsl::decode_op(bytes)?))
    }
}

//#endregion 🔖️HandcraftedOpCodecs

fn din4108_mutation_to_dsl(mutation: &Din4108Mutation) -> Din4108MutationDsl {
    match mutation {
        Din4108Mutation::ChangeClimateZone(payload) => Din4108MutationDsl::ChangeClimateZone { new_climate_zone: payload.new_climate_zone },
        Din4108Mutation::ChangeUsage(payload) => Din4108MutationDsl::ChangeUsage { new_usage: payload.new_usage.clone() },
        Din4108Mutation::ChangeTIntC(payload) => Din4108MutationDsl::ChangeTIntC { new_t_int_c: payload.new_t_int_c },
        Din4108Mutation::ChangeRhInt(payload) => Din4108MutationDsl::ChangeRhInt { new_rh_int: payload.new_rh_int },
        Din4108Mutation::ChangeAirtightnessN50(payload) => Din4108MutationDsl::ChangeAirtightnessN50 { new_airtightness_n50: payload.new_airtightness_n50 },
        Din4108Mutation::ChangeHasMechanicalVentilation(payload) => Din4108MutationDsl::ChangeHasMechanicalVentilation { new_has_mechanical_ventilation: payload.new_has_mechanical_ventilation },
        Din4108Mutation::ChangeBb2DetailsConform(payload) => Din4108MutationDsl::ChangeBb2DetailsConform { new_bb2_details_conform: payload.new_bb2_details_conform },
        Din4108Mutation::InsertZone(payload) => Din4108MutationDsl::InsertZone { index: payload.index, zone: payload.zone.clone() },
        Din4108Mutation::RemoveZone(payload) => Din4108MutationDsl::RemoveZone { index: payload.index },
        Din4108Mutation::ChangeZoneFloorArea(payload) => Din4108MutationDsl::ChangeZoneFloorArea { zone_id: payload.zone_id.clone(), new_floor_area_m2: payload.new_floor_area_m2 },
        Din4108Mutation::ChangeZoneHeaviness(payload) => Din4108MutationDsl::ChangeZoneHeaviness { zone_id: payload.zone_id.clone(), new_heaviness: payload.new_heaviness.clone() },
        Din4108Mutation::ChangeZoneNightVentilation(payload) => Din4108MutationDsl::ChangeZoneNightVentilation { zone_id: payload.zone_id.clone(), new_night_ventilation: payload.new_night_ventilation.clone() },
        Din4108Mutation::InsertZoneWindow(payload) => Din4108MutationDsl::InsertZoneWindow { zone_id: payload.zone_id.clone(), index: payload.index, window: payload.window.clone() },
        Din4108Mutation::RemoveZoneWindow(payload) => Din4108MutationDsl::RemoveZoneWindow { zone_id: payload.zone_id.clone(), index: payload.index },
        Din4108Mutation::ChangeZoneWindowArea(payload) => Din4108MutationDsl::ChangeZoneWindowArea { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_area_m2: payload.new_area_m2 },
        Din4108Mutation::ChangeZoneWindowGValue(payload) => Din4108MutationDsl::ChangeZoneWindowGValue { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_g_value: payload.new_g_value },
        Din4108Mutation::ChangeZoneWindowShadingFc(payload) => Din4108MutationDsl::ChangeZoneWindowShadingFc { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_shading_fc: payload.new_shading_fc },
        Din4108Mutation::InsertElement(payload) => Din4108MutationDsl::InsertElement { index: payload.index, element: payload.element.clone() },
        Din4108Mutation::RemoveElement(payload) => Din4108MutationDsl::RemoveElement { index: payload.index },
        Din4108Mutation::ChangeElementArea(payload) => Din4108MutationDsl::ChangeElementArea { element_id: payload.element_id.clone(), new_area_m2: payload.new_area_m2 },
        Din4108Mutation::ChangeElementAdjacent(payload) => Din4108MutationDsl::ChangeElementAdjacent { element_id: payload.element_id.clone(), new_adjacent: payload.new_adjacent.clone() },
        Din4108Mutation::ChangeElementKind(payload) => Din4108MutationDsl::ChangeElementKind { element_id: payload.element_id.clone(), new_kind: payload.new_kind.clone() },
        Din4108Mutation::InsertLayer(payload) => Din4108MutationDsl::InsertLayer { element_id: payload.element_id.clone(), index: payload.index, layer: payload.layer.clone() },
        Din4108Mutation::RemoveLayer(payload) => Din4108MutationDsl::RemoveLayer { element_id: payload.element_id.clone(), index: payload.index },
        Din4108Mutation::ReorderLayers(payload) => Din4108MutationDsl::ReorderLayers { element_id: payload.element_id.clone(), from: payload.from, to: payload.to },
        Din4108Mutation::ChangeLayerThickness(payload) => Din4108MutationDsl::ChangeLayerThickness { element_id: payload.element_id.clone(), index: payload.index, new_thickness_m: payload.new_thickness_m },
        Din4108Mutation::ChangeLayerLambda(payload) => Din4108MutationDsl::ChangeLayerLambda { element_id: payload.element_id.clone(), index: payload.index, new_lambda: payload.new_lambda },
        Din4108Mutation::ChangeLayerMu(payload) => Din4108MutationDsl::ChangeLayerMu { element_id: payload.element_id.clone(), index: payload.index, new_mu: payload.new_mu },
        Din4108Mutation::ChangeLayerMaterialId(payload) => Din4108MutationDsl::ChangeLayerMaterialId { element_id: payload.element_id.clone(), index: payload.index, new_material_id: payload.new_material_id.clone() },
        Din4108Mutation::InsertThermalBridge(payload) => Din4108MutationDsl::InsertThermalBridge { index: payload.index, bridge: payload.bridge.clone() },
        Din4108Mutation::RemoveThermalBridge(payload) => Din4108MutationDsl::RemoveThermalBridge { index: payload.index },
        Din4108Mutation::ChangeThermalBridgePsi(payload) => Din4108MutationDsl::ChangeThermalBridgePsi { bridge_id: payload.bridge_id.clone(), new_psi: payload.new_psi },
        Din4108Mutation::ChangeThermalBridgeLength(payload) => Din4108MutationDsl::ChangeThermalBridgeLength { bridge_id: payload.bridge_id.clone(), new_length_m: payload.new_length_m },

        Din4108Mutation::ChangeElementOrientationDeg(payload) => Din4108MutationDsl::ChangeElementOrientationDeg { element_id: payload.element_id.clone(), new_orientation_deg: payload.new_orientation_deg },
        Din4108Mutation::ChangeElementInclinationDeg(payload) => Din4108MutationDsl::ChangeElementInclinationDeg { element_id: payload.element_id.clone(), new_inclination_deg: payload.new_inclination_deg },
        Din4108Mutation::ChangeElementDeltaUg(payload) => Din4108MutationDsl::ChangeElementDeltaUg { element_id: payload.element_id.clone(), new_delta_u_g: payload.new_delta_u_g },
        Din4108Mutation::ChangeElementDeltaUf(payload) => Din4108MutationDsl::ChangeElementDeltaUf { element_id: payload.element_id.clone(), new_delta_u_f: payload.new_delta_u_f },
        Din4108Mutation::ChangeElementDeltaUr(payload) => Din4108MutationDsl::ChangeElementDeltaUr { element_id: payload.element_id.clone(), new_delta_u_r: payload.new_delta_u_r },
        Din4108Mutation::ChangeThermalBridgeBb2Type(payload) => Din4108MutationDsl::ChangeThermalBridgeBb2Type { bridge_id: payload.bridge_id.clone(), new_bb2_type: payload.new_bb2_type.clone() },
        Din4108Mutation::ChangeZoneWindowOrientation(payload) => Din4108MutationDsl::ChangeZoneWindowOrientation { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_orientation: payload.new_orientation.clone() },
        Din4108Mutation::ChangeZoneWindowInclinationDeg(payload) => Din4108MutationDsl::ChangeZoneWindowInclinationDeg { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_inclination_deg: payload.new_inclination_deg },
        Din4108Mutation::ChangeLayerApplicationType(payload) => Din4108MutationDsl::ChangeLayerApplicationType { element_id: payload.element_id.clone(), index: payload.index, new_application_type: payload.new_application_type.clone() },
        Din4108Mutation::ChangeLayerCompressiveClass(payload) => Din4108MutationDsl::ChangeLayerCompressiveClass { element_id: payload.element_id.clone(), index: payload.index, new_compressive_class: payload.new_compressive_class.clone() },
    }
}

fn din4108_mutation_from_dsl(dsl: Din4108MutationDsl) -> Din4108Mutation {
    match dsl {
        Din4108MutationDsl::ChangeClimateZone { new_climate_zone } => Din4108Mutation::ChangeClimateZone(change_climate_zone::ChangeClimateZone { new_climate_zone }),
        Din4108MutationDsl::ChangeUsage { new_usage } => Din4108Mutation::ChangeUsage(change_usage::ChangeUsage { new_usage }),
        Din4108MutationDsl::ChangeTIntC { new_t_int_c } => Din4108Mutation::ChangeTIntC(change_t_int_c::ChangeTIntC { new_t_int_c }),
        Din4108MutationDsl::ChangeRhInt { new_rh_int } => Din4108Mutation::ChangeRhInt(change_rh_int::ChangeRhInt { new_rh_int }),
        Din4108MutationDsl::ChangeAirtightnessN50 { new_airtightness_n50 } => Din4108Mutation::ChangeAirtightnessN50(change_airtightness_n50::ChangeAirtightnessN50 { new_airtightness_n50 }),
        Din4108MutationDsl::ChangeHasMechanicalVentilation { new_has_mechanical_ventilation } => Din4108Mutation::ChangeHasMechanicalVentilation(change_has_mechanical_ventilation::ChangeHasMechanicalVentilation { new_has_mechanical_ventilation }),
        Din4108MutationDsl::ChangeBb2DetailsConform { new_bb2_details_conform } => Din4108Mutation::ChangeBb2DetailsConform(change_bb2_details_conform::ChangeBb2DetailsConform { new_bb2_details_conform }),
        Din4108MutationDsl::InsertZone { index, zone } => Din4108Mutation::InsertZone(insert_zone::InsertZone { index, zone }),
        Din4108MutationDsl::RemoveZone { index } => Din4108Mutation::RemoveZone(remove_zone::RemoveZone { index }),
        Din4108MutationDsl::ChangeZoneFloorArea { zone_id, new_floor_area_m2 } => Din4108Mutation::ChangeZoneFloorArea(change_zone_floor_area::ChangeZoneFloorArea { zone_id, new_floor_area_m2 }),
        Din4108MutationDsl::ChangeZoneHeaviness { zone_id, new_heaviness } => Din4108Mutation::ChangeZoneHeaviness(change_zone_heaviness::ChangeZoneHeaviness { zone_id, new_heaviness }),
        Din4108MutationDsl::ChangeZoneNightVentilation { zone_id, new_night_ventilation } => Din4108Mutation::ChangeZoneNightVentilation(change_zone_night_ventilation::ChangeZoneNightVentilation { zone_id, new_night_ventilation }),
        Din4108MutationDsl::InsertZoneWindow { zone_id, index, window } => Din4108Mutation::InsertZoneWindow(insert_zone_window::InsertZoneWindow { zone_id, index, window }),
        Din4108MutationDsl::RemoveZoneWindow { zone_id, index } => Din4108Mutation::RemoveZoneWindow(remove_zone_window::RemoveZoneWindow { zone_id, index }),
        Din4108MutationDsl::ChangeZoneWindowArea { zone_id, window_id, new_area_m2 } => Din4108Mutation::ChangeZoneWindowArea(change_zone_window_area::ChangeZoneWindowArea { zone_id, window_id, new_area_m2 }),
        Din4108MutationDsl::ChangeZoneWindowGValue { zone_id, window_id, new_g_value } => Din4108Mutation::ChangeZoneWindowGValue(change_zone_window_g_value::ChangeZoneWindowGValue { zone_id, window_id, new_g_value }),
        Din4108MutationDsl::ChangeZoneWindowShadingFc { zone_id, window_id, new_shading_fc } => Din4108Mutation::ChangeZoneWindowShadingFc(change_zone_window_shading_fc::ChangeZoneWindowShadingFc { zone_id, window_id, new_shading_fc }),
        Din4108MutationDsl::InsertElement { index, element } => Din4108Mutation::InsertElement(insert_element::InsertElement { index, element }),
        Din4108MutationDsl::RemoveElement { index } => Din4108Mutation::RemoveElement(remove_element::RemoveElement { index }),
        Din4108MutationDsl::ChangeElementArea { element_id, new_area_m2 } => Din4108Mutation::ChangeElementArea(change_element_area::ChangeElementArea { element_id, new_area_m2 }),
        Din4108MutationDsl::ChangeElementAdjacent { element_id, new_adjacent } => Din4108Mutation::ChangeElementAdjacent(change_element_adjacent::ChangeElementAdjacent { element_id, new_adjacent }),
        Din4108MutationDsl::ChangeElementKind { element_id, new_kind } => Din4108Mutation::ChangeElementKind(change_element_kind::ChangeElementKind { element_id, new_kind }),
        Din4108MutationDsl::InsertLayer { element_id, index, layer } => Din4108Mutation::InsertLayer(insert_layer::InsertLayer { element_id, index, layer }),
        Din4108MutationDsl::RemoveLayer { element_id, index } => Din4108Mutation::RemoveLayer(remove_layer::RemoveLayer { element_id, index }),
        Din4108MutationDsl::ReorderLayers { element_id, from, to } => Din4108Mutation::ReorderLayers(reorder_layers::ReorderLayers { element_id, from, to }),
        Din4108MutationDsl::ChangeLayerThickness { element_id, index, new_thickness_m } => Din4108Mutation::ChangeLayerThickness(change_layer_thickness::ChangeLayerThickness { element_id, index, new_thickness_m }),
        Din4108MutationDsl::ChangeLayerLambda { element_id, index, new_lambda } => Din4108Mutation::ChangeLayerLambda(change_layer_lambda::ChangeLayerLambda { element_id, index, new_lambda }),
        Din4108MutationDsl::ChangeLayerMu { element_id, index, new_mu } => Din4108Mutation::ChangeLayerMu(change_layer_mu::ChangeLayerMu { element_id, index, new_mu }),
        Din4108MutationDsl::ChangeLayerMaterialId { element_id, index, new_material_id } => Din4108Mutation::ChangeLayerMaterialId(change_layer_material_id::ChangeLayerMaterialId { element_id, index, new_material_id }),
        Din4108MutationDsl::InsertThermalBridge { index, bridge } => Din4108Mutation::InsertThermalBridge(insert_thermal_bridge::InsertThermalBridge { index, bridge }),
        Din4108MutationDsl::RemoveThermalBridge { index } => Din4108Mutation::RemoveThermalBridge(remove_thermal_bridge::RemoveThermalBridge { index }),
        Din4108MutationDsl::ChangeThermalBridgePsi { bridge_id, new_psi } => Din4108Mutation::ChangeThermalBridgePsi(change_thermal_bridge_psi::ChangeThermalBridgePsi { bridge_id, new_psi }),
        Din4108MutationDsl::ChangeThermalBridgeLength { bridge_id, new_length_m } => Din4108Mutation::ChangeThermalBridgeLength(change_thermal_bridge_length::ChangeThermalBridgeLength { bridge_id, new_length_m }),

        Din4108MutationDsl::ChangeElementOrientationDeg { element_id, new_orientation_deg } => Din4108Mutation::ChangeElementOrientationDeg(change_element_orientation_deg::ChangeElementOrientationDeg { element_id, new_orientation_deg }),
        Din4108MutationDsl::ChangeElementInclinationDeg { element_id, new_inclination_deg } => Din4108Mutation::ChangeElementInclinationDeg(change_element_inclination_deg::ChangeElementInclinationDeg { element_id, new_inclination_deg }),
        Din4108MutationDsl::ChangeElementDeltaUg { element_id, new_delta_u_g } => Din4108Mutation::ChangeElementDeltaUg(change_element_delta_u_g::ChangeElementDeltaUg { element_id, new_delta_u_g }),
        Din4108MutationDsl::ChangeElementDeltaUf { element_id, new_delta_u_f } => Din4108Mutation::ChangeElementDeltaUf(change_element_delta_u_f::ChangeElementDeltaUf { element_id, new_delta_u_f }),
        Din4108MutationDsl::ChangeElementDeltaUr { element_id, new_delta_u_r } => Din4108Mutation::ChangeElementDeltaUr(change_element_delta_u_r::ChangeElementDeltaUr { element_id, new_delta_u_r }),
        Din4108MutationDsl::ChangeThermalBridgeBb2Type { bridge_id, new_bb2_type } => Din4108Mutation::ChangeThermalBridgeBb2Type(change_thermal_bridge_bb2_type::ChangeThermalBridgeBb2Type { bridge_id, new_bb2_type }),
        Din4108MutationDsl::ChangeZoneWindowOrientation { zone_id, window_id, new_orientation } => Din4108Mutation::ChangeZoneWindowOrientation(change_zone_window_orientation::ChangeZoneWindowOrientation { zone_id, window_id, new_orientation }),
        Din4108MutationDsl::ChangeZoneWindowInclinationDeg { zone_id, window_id, new_inclination_deg } => Din4108Mutation::ChangeZoneWindowInclinationDeg(change_zone_window_inclination_deg::ChangeZoneWindowInclinationDeg { zone_id, window_id, new_inclination_deg }),
        Din4108MutationDsl::ChangeLayerApplicationType { element_id, index, new_application_type } => Din4108Mutation::ChangeLayerApplicationType(change_layer_application_type::ChangeLayerApplicationType { element_id, index, new_application_type }),
        Din4108MutationDsl::ChangeLayerCompressiveClass { element_id, index, new_compressive_class } => Din4108Mutation::ChangeLayerCompressiveClass(change_layer_compressive_class::ChangeLayerCompressiveClass { element_id, index, new_compressive_class }),
    }
}

#[cfg(test)]
fn demo_mutation_cases() -> Vec<Din4108Mutation> {
    use crate::document::ClimateZoneDe;
    vec![
        Din4108Mutation::ChangeClimateZone(change_climate_zone::ChangeClimateZone { new_climate_zone: ClimateZoneDe::Zone3 }),
        Din4108Mutation::ChangeUsage(change_usage::ChangeUsage { new_usage: "nonresidential".into() }),
        Din4108Mutation::ChangeTIntC(change_t_int_c::ChangeTIntC { new_t_int_c: 21.5 }),
        Din4108Mutation::ChangeRhInt(change_rh_int::ChangeRhInt { new_rh_int: 0.55 }),
        Din4108Mutation::ChangeAirtightnessN50(change_airtightness_n50::ChangeAirtightnessN50 { new_airtightness_n50: 1.5 }),
        Din4108Mutation::ChangeHasMechanicalVentilation(change_has_mechanical_ventilation::ChangeHasMechanicalVentilation { new_has_mechanical_ventilation: true }),
        Din4108Mutation::ChangeBb2DetailsConform(change_bb2_details_conform::ChangeBb2DetailsConform { new_bb2_details_conform: false }),
        Din4108Mutation::RemoveZone(remove_zone::RemoveZone { index: 0 }),
        Din4108Mutation::RemoveElement(remove_element::RemoveElement { index: 0 }),
        Din4108Mutation::RemoveThermalBridge(remove_thermal_bridge::RemoveThermalBridge { index: 0 }),
        Din4108Mutation::ChangeZoneFloorArea(change_zone_floor_area::ChangeZoneFloorArea { zone_id: "z0".into(), new_floor_area_m2: 40.0 }),
        Din4108Mutation::ChangeElementArea(change_element_area::ChangeElementArea { element_id: "e0".into(), new_area_m2: 12.0 }),
        Din4108Mutation::ChangeLayerThickness(change_layer_thickness::ChangeLayerThickness { element_id: "e0".into(), index: 0, new_thickness_m: 0.2 }),
        Din4108Mutation::ChangeLayerLambda(change_layer_lambda::ChangeLayerLambda { element_id: "e0".into(), index: 0, new_lambda: 0.035 }),
        Din4108Mutation::ChangeThermalBridgePsi(change_thermal_bridge_psi::ChangeThermalBridgePsi { bridge_id: "b0".into(), new_psi: 0.05 }),
        Din4108Mutation::ChangeThermalBridgeLength(change_thermal_bridge_length::ChangeThermalBridgeLength { bridge_id: "b0".into(), new_length_m: 8.0 }),
        Din4108Mutation::ChangeElementOrientationDeg(change_element_orientation_deg::ChangeElementOrientationDeg { element_id: "e0".into(), new_orientation_deg: 180.0 }),
        Din4108Mutation::ChangeThermalBridgeBb2Type(change_thermal_bridge_bb2_type::ChangeThermalBridgeBb2Type { bridge_id: "b0".into(), new_bb2_type: "categoryB".into() }),
        Din4108Mutation::ChangeLayerApplicationType(change_layer_application_type::ChangeLayerApplicationType { element_id: "e0".into(), index: 0, new_application_type: "WAP".into() }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
