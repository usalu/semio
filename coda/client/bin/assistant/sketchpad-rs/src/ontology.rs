use generational_arena::{Arena, Index};
use serde::{Deserialize, Serialize};

use crate::transmission::{
    UsageProfile, AutomationClass, ClimateRegion, GroundContactType,
    UnheatedSpaceType, ShutterControl, WindowGlazingType, 
    WindowInclinationAngle, ThermalBridgeCategory, BuildingType
};
use crate::overheating::{SummerClimateRegion, NightVentilation, BuildingCategory};
use crate::ventilation::{TightnessCategory};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityData {
    Site(SiteData),
    Building(BuildingData),
    Story(StoryData),
    Space(SpaceData),
    
    // Physical Envelope Elements
    Wall(WallData),
    Window(WindowData),
    Slab(SlabData),
    Roof(RoofData),
    ThermalBridge(ThermalBridgeData),
    
    // Abstract Properties & Materials
    Material(MaterialData),
    Layer(LayerData),

    // Physics & Semantic Entities
    Climate(ClimateData),
    UsageProfile(UsageProfileData),
    Automation(AutomationData),
    VentilationSystem(VentilationSystemData),
    LightingSystem(LightingSystemData),
    InternalHeatSource(InternalHeatSourceData),

    // Abstract Semantic Property Nodes
    Property(PropertyData),
    
    // Abstract Calculation Processes
    Calculation(CalculationData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationData {
    pub name: String,
    pub formula: String,
    pub doc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyData {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SiteData { pub name: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingData { 
    pub name: String,
    pub building_type: BuildingType,
    pub building_category: Option<BuildingCategory>,
    pub year_class: String,
    pub scenario: String,
    pub num_stories: i32,
    pub heating_system: String,
    pub thermal_bridge_category: ThermalBridgeCategory,
    
    pub total_conditioned_volume: f64,
    pub total_floor_area: f64,
    pub total_roof_area: f64,
    pub total_ground_area: f64,
    pub exterior_perimeter: f64,
    pub roof_pitch_deg: Option<f64>,
    pub building_rotation_deg: f64,
    pub window_to_wall_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoryData { pub name: String, pub elevation: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceData { 
    pub name: String, 
    pub volume: f64, 
    pub net_floor_area: f64,
    pub room_depth: Option<f64>,
    pub ceiling_height: Option<f64>,
    pub is_critical_room: bool,
    pub unheated_space_type: Option<UnheatedSpaceType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallData { 
    pub area: f64, 
    pub u_value: f64, 
    pub thickness: f64,
    pub r_si: f64,
    pub r_se: f64,
    pub f_neig: f64,
    pub f_x: f64,
    pub solar_absorptance: f64,
    pub is_roof: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowData { 
    pub area: f64, 
    pub u_value: f64, 
    pub u_w_sh: f64,
    pub f_sh: f64,
    pub g_value: f64,
    pub frame_fraction: f64,
    pub f_neig: f64,
    pub f_x: f64,
    pub shading_factor_fc: f64,
    pub surroundings_shading_fs: f64,
    pub shutter_control: ShutterControl,
    pub glazing_type: WindowGlazingType,
    pub inclination_angle: WindowInclinationAngle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlabData { 
    pub area: f64, 
    pub u_value: f64,
    pub r_si: f64,
    pub r_se: f64,
    pub f_x: f64,
    pub ground_contact: Option<GroundContactType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoofData { 
    pub area: f64, 
    pub u_value: f64,
    pub r_si: f64,
    pub r_se: f64,
    pub f_neig: f64,
    pub f_x: f64,
    pub solar_absorptance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalBridgeData {
    pub length: f64,
    pub psi_value: f64,
    pub f_x: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MaterialData { 
    pub name: String, 
    pub lambda: f64, 
    pub specific_heat_capacity: f64 
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayerData { 
    pub thickness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateData {
    pub region: ClimateRegion,
    pub summer_region: SummerClimateRegion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageProfileData {
    pub profile: UsageProfile,
    pub q_i_p: f64,
    pub q_i_app: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationData {
    pub class: AutomationClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentilationSystemData {
    pub tightness: TightnessCategory,
    pub mechanical_ventilation: bool,
    pub night_ventilation: NightVentilation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LightingSystemData {
    pub q_l_f_daily: f64,
    pub exhaust_type_mu_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InternalHeatSourceData {
    pub description: String,
    pub sensible_heat_watts: f64,
    pub daily_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Relationship {
    Aggregates { parent: Index, child: Index },
    BoundsSpace { space: Index, boundary_element: Index },
    FillsVoid { host: Index, filler: Index },
    
    ContainsLayer { host: Index, layer: Index, order: i32 },
    ComposedOfMaterial { layer: Index, material: Index },
    HasThermalBridge { host: Index, bridge: Index },

    OperatesUnderProfile { space: Index, profile: Index },
    ControlledByAutomation { space: Index, automation: Index },
    LocatedInClimate { building: Index, climate: Index },
    ServicedByVentilation { space: Index, system: Index },
    IlluminatedBy { space: Index, lighting_system: Index },
    ContainsHeatSource { space: Index, source: Index },
    
    HasProperty { host: Index, property: Index },
    InputsTo { parameter: Index, calculation: Index },
    OutputsTo { calculation: Index, result: Index },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingKnowledgeGraph {
    pub entities: Arena<EntityData>,
    pub relationships: Vec<Relationship>,
}

impl Default for BuildingKnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildingKnowledgeGraph {
    pub fn new() -> Self {
        Self {
            entities: Arena::new(),
            relationships: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, data: EntityData) -> Index {
        self.entities.insert(data)
    }

    pub fn remove_entity(&mut self, id: Index) -> Option<EntityData> {
        self.entities.remove(id)
    }

    pub fn get_entity(&self, id: Index) -> Option<&EntityData> {
        self.entities.get(id)
    }

    pub fn add_relationship(&mut self, rel: Relationship) {
        self.relationships.push(rel);
    }

    pub fn to_vis_network_json(&self) -> serde_json::Value {
        #[derive(Serialize)]
        struct VisNode {
            id: String,
            label: String,
            group: String,
            title: Option<String>,
            doc: Option<String>,
        }

        #[derive(Serialize)]
        struct VisEdge {
            from: String,
            to: String,
            label: String,
        }

        let mut nodes = Vec::new();
        for (index, entity) in &self.entities {
            let (idx, gen) = index.into_raw_parts();
            let id = format!("{}-{}", idx, gen);
            
            let mut doc_text = None;
            
            let (label, group) = match entity {
                EntityData::Site(d) => (format!("Site: {}", d.name), "Site"),
                EntityData::Building(d) => (format!("Building: {}", d.name), "Building"),
                EntityData::Story(d) => (format!("Story: {}", d.name), "Story"),
                EntityData::Space(d) => (format!("Space: {}", d.name), "Space"),
                EntityData::Wall(d) => (format!("Wall (U={:.2})", d.u_value), "Wall"),
                EntityData::Window(d) => (format!("Window (U={:.2})", d.u_value), "Window"),
                EntityData::Slab(d) => (format!("Slab (U={:.2})", d.u_value), "Slab"),
                EntityData::Roof(d) => (format!("Roof (U={:.2})", d.u_value), "Roof"),
                EntityData::ThermalBridge(_) => (format!("ThermalBridge"), "ThermalBridge"),
                EntityData::Material(d) => (format!("Material: {}", d.name), "Material"),
                EntityData::Layer(d) => (format!("Layer ({:.2}m)", d.thickness), "Layer"),
                EntityData::Climate(_) => (format!("Climate"), "Climate"),
                EntityData::UsageProfile(_) => (format!("Usage"), "UsageProfile"),
                EntityData::Automation(_) => (format!("Automation"), "Automation"),
                EntityData::VentilationSystem(_) => (format!("Ventilation"), "VentilationSystem"),
                EntityData::LightingSystem(_) => (format!("Lighting"), "LightingSystem"),
                EntityData::InternalHeatSource(d) => (format!("Heat Source: {}", d.description), "InternalHeatSource"),
                EntityData::Property(d) => {
                    doc_text = d.doc.clone();
                    (format!("{}: {} {}", d.name, d.value, d.unit), "Property")
                },
                EntityData::Calculation(d) => {
                    doc_text = Some(format!("**Formula:**\n$${}$$\n\n{}", d.formula, d.doc));
                    (format!("Calculation: {}", d.name), "Calculation")
                },
            };
            nodes.push(VisNode { id, label, group: group.to_string(), title: doc_text.clone(), doc: doc_text });
        }

        let mut edges = Vec::new();
        for rel in &self.relationships {
            let (from, to, label) = match rel {
                Relationship::Aggregates { parent, child } => (*parent, *child, "Aggregates"),
                Relationship::BoundsSpace { space, boundary_element } => (*space, *boundary_element, "BoundsSpace"),
                Relationship::FillsVoid { host, filler } => (*host, *filler, "FillsVoid"),
                Relationship::ContainsLayer { host, layer, order: _ } => (*host, *layer, "ContainsLayer"),
                Relationship::ComposedOfMaterial { layer, material } => (*layer, *material, "ComposedOfMaterial"),
                Relationship::HasThermalBridge { host, bridge } => (*host, *bridge, "HasThermalBridge"),
                Relationship::OperatesUnderProfile { space, profile } => (*space, *profile, "OperatesUnderProfile"),
                Relationship::ControlledByAutomation { space, automation } => (*space, *automation, "ControlledByAutomation"),
                Relationship::LocatedInClimate { building, climate } => (*building, *climate, "LocatedInClimate"),
                Relationship::ServicedByVentilation { space, system } => (*space, *system, "ServicedByVentilation"),
                Relationship::IlluminatedBy { space, lighting_system } => (*space, *lighting_system, "IlluminatedBy"),
                Relationship::ContainsHeatSource { space, source } => (*space, *source, "ContainsHeatSource"),
                Relationship::HasProperty { host, property } => (*host, *property, "HasProperty"),
                Relationship::InputsTo { parameter, calculation } => (*parameter, *calculation, "InputsTo"),
                Relationship::OutputsTo { calculation, result } => (*calculation, *result, "OutputsTo"),
            };
            let (f_idx, f_gen) = from.into_raw_parts();
            let (t_idx, t_gen) = to.into_raw_parts();
            edges.push(VisEdge { 
                from: format!("{}-{}", f_idx, f_gen), 
                to: format!("{}-{}", t_idx, t_gen), 
                label: label.to_string() 
            });
        }

        serde_json::json!({
            "nodes": nodes,
            "edges": edges
        })
    }
}
