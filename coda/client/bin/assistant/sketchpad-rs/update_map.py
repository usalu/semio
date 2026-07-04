import sys

def replace_function(file_path):
    with open(file_path, 'r') as f:
        content = f.read()
        
    start_str = "fn map_state_to_graph("
    start_idx = content.find(start_str)
    
    if start_idx == -1:
        print("Function not found")
        sys.exit(1)
        
    # Find the closing brace of the function
    brace_count = 0
    in_function = False
    end_idx = -1
    
    for i in range(start_idx, len(content)):
        if content[i] == '{':
            brace_count += 1
            in_function = True
        elif content[i] == '}':
            brace_count -= 1
            
        if in_function and brace_count == 0:
            end_idx = i + 1
            break
            
    if end_idx == -1:
        print("Could not find end of function")
        sys.exit(1)
        
    new_func = """fn map_state_to_graph(state: &State, u_wall: f64, u_roof: f64, u_floor: f64, u_window: f64) -> crate::ontology::BuildingKnowledgeGraph {
    use crate::ontology::*;
    let mut graph = BuildingKnowledgeGraph::new();

    let b_idx = graph.add_entity(EntityData::Building(BuildingData {
        name: "Building".to_string(),
        building_type: crate::transmission::BuildingType::Residential,
        building_category: None,
        year_class: state.params.year_class.clone(),
        scenario: state.params.scenario.clone(),
        num_stories: state.params.num_stories,
        heating_system: state.params.heating_system.clone(),
        thermal_bridge_category: crate::transmission::ThermalBridgeCategory::StandardDefault,
        total_conditioned_volume: state.geometry.as_ref().map(|g| g.total_conditioned_volume).unwrap_or(0.0),
        total_floor_area: state.geometry.as_ref().map(|g| g.total_floor_area).unwrap_or(0.0),
        total_roof_area: state.geometry.as_ref().map(|g| g.total_roof_area).unwrap_or(0.0),
        total_ground_area: state.geometry.as_ref().map(|g| g.total_ground_area).unwrap_or(0.0),
        exterior_perimeter: state.geometry.as_ref().map(|g| g.exterior_perimeter).unwrap_or(0.0),
        roof_pitch_deg: None,
        building_rotation_deg: state.params.building_rotation_deg,
        window_to_wall_ratio: state.params.window_to_wall_ratio,
    }));

    let p_vol = graph.add_entity(EntityData::Property(PropertyData { 
        name: "Volume (V_e)".into(), value: format!("{:.1}", state.geometry.as_ref().map(|g| g.total_conditioned_volume).unwrap_or(0.0)), unit: "m³".into(),
        doc: Some("**Conditioned Volume ($V_e$)**\\nThe total heated volume of the building. Used to calculate ventilation air mass flows ($V_e \\\\cdot n$).".into())
    }));
    let p_year = graph.add_entity(EntityData::Property(PropertyData { 
        name: "Year Class".into(), value: state.params.year_class.clone(), unit: "".into(),
        doc: Some("**Year Class**\\nThe construction age bracket. Dictates default U-values, infiltration rates, and system efficiencies if not explicitly overridden.".into())
    }));
    let p_heat = graph.add_entity(EntityData::Property(PropertyData { 
        name: "Heating System".into(), value: state.params.heating_system.clone(), unit: "".into(),
        doc: Some("**Heating System**\\nThe primary thermal generator. Affects the primary energy factor ($f_P$) and conversion efficiency ($e_g$).".into())
    }));
    graph.add_relationship(Relationship::HasProperty { host: b_idx, property: p_vol });
    graph.add_relationship(Relationship::HasProperty { host: b_idx, property: p_year });
    graph.add_relationship(Relationship::HasProperty { host: b_idx, property: p_heat });

    // Global Building Ventilation Calculation
    let calc_vent = graph.add_entity(EntityData::Calculation(CalculationData {
        name: "Ventilation Heat Loss (H_V)".into(),
        formula: "H_V = \\\\rho_{air} \\\\cdot c_{a} \\\\cdot n \\\\cdot V_e".into(),
        doc: "Calculates the heat loss due to air exchange (infiltration and window airing).".into(),
    }));
    graph.add_relationship(Relationship::InputsTo { parameter: p_vol, calculation: calc_vent });

    if let Some(geom) = &state.geometry {
        let a_wall_total = geom.envelope_data.n.gross_wall_area + geom.envelope_data.e.gross_wall_area + geom.envelope_data.s.gross_wall_area + geom.envelope_data.w.gross_wall_area;
        let win_area_total = geom.envelope_data.n.window_area + geom.envelope_data.s.window_area + geom.envelope_data.e.window_area + geom.envelope_data.w.window_area;
        let has_zones = state.ui_state.as_ref().and_then(|ui| ui.get("raw_zones").and_then(|z| z.as_array())).map(|a| !a.is_empty()).unwrap_or(false);

        if has_zones {
            let raw_zones = state.ui_state.as_ref().unwrap().get("raw_zones").unwrap().as_array().unwrap();
            let mut total_zone_area = 0.0;
            for zone in raw_zones {
                let w = zone.get("geometry").and_then(|g| g.get("width")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let l = zone.get("geometry").and_then(|g| g.get("length")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                total_zone_area += w * l;
            }
            if total_zone_area == 0.0 { total_zone_area = 1.0; } 

            for zone in raw_zones {
                let z_name = zone.get("type").and_then(|v| v.as_str()).unwrap_or("Zone").to_string();
                let z_width = zone.get("geometry").and_then(|g| g.get("width")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let z_length = zone.get("geometry").and_then(|g| g.get("length")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let z_area = z_width * z_length;
                let scale_factor = z_area / total_zone_area;

                let z_idx = graph.add_entity(EntityData::Space(SpaceData {
                    name: z_name, volume: z_area * state.params.story_height, net_floor_area: z_area,
                    room_depth: None, ceiling_height: Some(state.params.story_height), is_critical_room: false, unheated_space_type: None
                }));
                graph.add_relationship(Relationship::Aggregates { parent: b_idx, child: z_idx });

                let p_area = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "Area (A_NGF)".into(), value: format!("{:.1}", z_area), unit: "m²".into(),
                    doc: Some("**Net Floor Area ($A_{NGF}$)**\\nThe reference area used to multiply specific internal gains ($q_I$).".into())
                }));
                graph.add_relationship(Relationship::HasProperty { host: z_idx, property: p_area });

                // Calculations
                let calc_internal = graph.add_entity(EntityData::Calculation(CalculationData {
                    name: "Internal Gains (Q_I)".into(),
                    formula: "Q_{I} = q_{I} \\\\cdot A_{NGF} \\\\cdot t".into(),
                    doc: "Heat generated by people, equipment, and lighting.".into(),
                }));
                let calc_trans = graph.add_entity(EntityData::Calculation(CalculationData {
                    name: "Transmission Loss (H_T)".into(),
                    formula: "H_{T,D} = \\\\sum (A_j \\\\cdot U_j \\\\cdot f_{neig,j})".into(),
                    doc: "Direct transmission heat loss through the opaque envelope and windows.".into(),
                }));
                let calc_solar = graph.add_entity(EntityData::Calculation(CalculationData {
                    name: "Solar Gains (Q_S)".into(),
                    formula: "Q_{s,w} = I_s \\\\cdot A_w \\\\cdot (1 - F_F) \\\\cdot F_w \\\\cdot g \\\\cdot F_C \\\\cdot F_S".into(),
                    doc: "Solar energy passing directly through windows, minus frame and shading.".into(),
                }));
                let calc_solar_op = graph.add_entity(EntityData::Calculation(CalculationData {
                    name: "Opaque Solar/Sky (Q_s,op)".into(),
                    formula: "Q_{s,op} = A_{op} \\\\cdot U_{op} \\\\cdot R_{se} \\\\cdot (\\\\alpha \\\\cdot I_s - F_{sky} \\\\cdot h_r \\\\cdot \\\\Delta\\\\theta_{er})".into(),
                    doc: "Solar gains on opaque walls minus radiation lost to the cold night sky.".into(),
                }));

                graph.add_relationship(Relationship::InputsTo { parameter: p_area, calculation: calc_internal });

                // --- WALL ---
                let w_idx = graph.add_entity(EntityData::Wall(WallData {
                    area: a_wall_total * scale_factor, u_value: u_wall, thickness: 0.3,
                    r_si: 0.13, r_se: 0.04, f_neig: 1.0, f_x: 1.0, solar_absorptance: 0.6, is_roof: false
                }));
                graph.add_relationship(Relationship::BoundsSpace { space: z_idx, boundary_element: w_idx });
                
                let wp_u = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "U-Value".into(), value: format!("{:.2}", u_wall), unit: "W/(m²K)".into(),
                    doc: Some("Measures heat transfer rate. Lower values indicate better insulation.".into())
                }));
                let wp_alpha = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "Solar Absorptance (\\\\alpha)".into(), value: "0.6".into(), unit: "-".into(),
                    doc: Some("Solar radiation absorbed based on color.".into())
                }));
                let wp_rse = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "R_se".into(), value: "0.04".into(), unit: "m²K/W".into(),
                    doc: Some("External Surface Resistance. Standard is 0.04 for walls.".into())
                }));
                let wp_area = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "Area".into(), value: format!("{:.1}", a_wall_total * scale_factor), unit: "m²".into(),
                    doc: Some("Total exposed area. Directly proportional to transmission heat losses.".into())
                }));
                graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_u });
                graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_alpha });
                graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_rse });
                graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_area });

                graph.add_relationship(Relationship::InputsTo { parameter: wp_u, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: wp_area, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: wp_alpha, calculation: calc_solar_op });
                graph.add_relationship(Relationship::InputsTo { parameter: wp_rse, calculation: calc_solar_op });
                graph.add_relationship(Relationship::InputsTo { parameter: wp_area, calculation: calc_solar_op });

                // --- WINDOW ---
                let win_idx = graph.add_entity(EntityData::Window(WindowData {
                    area: win_area_total * scale_factor, u_value: u_window, u_w_sh: u_window, f_sh: 0.0, g_value: 0.6, frame_fraction: 0.3, f_neig: 1.0, f_x: 1.0, shading_factor_fc: 1.0, surroundings_shading_fs: 1.0,
                    shutter_control: crate::transmission::ShutterControl::Manual, glazing_type: crate::transmission::WindowGlazingType::Double, inclination_angle: crate::transmission::WindowInclinationAngle::Deg90,
                }));
                graph.add_relationship(Relationship::FillsVoid { host: w_idx, filler: win_idx });

                let win_g = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "g-Value".into(), value: "0.6".into(), unit: "-".into(),
                    doc: Some("Total Solar Energy Transmittance. Fraction of solar radiation passing through glass.".into())
                }));
                let win_ff = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "Frame Fraction (F_F)".into(), value: "0.3".into(), unit: "-".into(),
                    doc: Some("Percentage of window area that is opaque frame.".into())
                }));
                let win_fc = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "Shading Factor (F_C)".into(), value: "1.0".into(), unit: "-".into(),
                    doc: Some("Operable Shading Factor from blinds or curtains.".into())
                }));
                let win_u = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "U-Value".into(), value: format!("{:.2}", u_window), unit: "W/(m²K)".into(),
                    doc: Some("Measures heat transfer through window.".into())
                }));
                let win_area_prop = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "Area".into(), value: format!("{:.2}", win_area_total * scale_factor), unit: "m²".into(),
                    doc: Some("Window size area.".into())
                }));
                graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_g });
                graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_ff });
                graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_fc });
                graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_u });
                graph.add_relationship(Relationship::HasProperty { host: win_idx, property: win_area_prop });

                graph.add_relationship(Relationship::InputsTo { parameter: win_u, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: win_area_prop, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: win_g, calculation: calc_solar });
                graph.add_relationship(Relationship::InputsTo { parameter: win_ff, calculation: calc_solar });
                graph.add_relationship(Relationship::InputsTo { parameter: win_fc, calculation: calc_solar });
                graph.add_relationship(Relationship::InputsTo { parameter: win_area_prop, calculation: calc_solar });

                // --- ROOF ---
                let r_idx = graph.add_entity(EntityData::Roof(RoofData {
                    area: geom.total_roof_area * scale_factor, u_value: u_roof, r_si: 0.1, r_se: 0.04, f_neig: 1.0, f_x: 1.0, solar_absorptance: 0.8
                }));
                graph.add_relationship(Relationship::BoundsSpace { space: z_idx, boundary_element: r_idx });
                
                let rp_fneig = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "f_neig".into(), value: "1.0".into(), unit: "-".into(),
                    doc: Some("Inclination Correction Factor ($f_{neig}$)".into())
                }));
                let rp_u = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "U-Value".into(), value: format!("{:.2}", u_roof), unit: "W/(m²K)".into(),
                    doc: None
                }));
                graph.add_relationship(Relationship::HasProperty { host: r_idx, property: rp_fneig });
                graph.add_relationship(Relationship::HasProperty { host: r_idx, property: rp_u });

                graph.add_relationship(Relationship::InputsTo { parameter: rp_u, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: rp_fneig, calculation: calc_trans });

                // --- SLAB ---
                let s_idx = graph.add_entity(EntityData::Slab(SlabData {
                    area: geom.total_ground_area * scale_factor, u_value: u_floor, r_si: 0.17, r_se: 0.04, f_x: 0.6, ground_contact: None
                }));
                graph.add_relationship(Relationship::BoundsSpace { space: z_idx, boundary_element: s_idx });

                let sp_fx = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "f_x".into(), value: "0.6".into(), unit: "-".into(),
                    doc: Some("Temperature Correction Factor ($f_x$) for ground/unheated spaces.".into())
                }));
                let sp_u = graph.add_entity(EntityData::Property(PropertyData { 
                    name: "U-Value".into(), value: format!("{:.2}", u_floor), unit: "W/(m²K)".into(),
                    doc: None
                }));
                graph.add_relationship(Relationship::HasProperty { host: s_idx, property: sp_fx });
                graph.add_relationship(Relationship::HasProperty { host: s_idx, property: sp_u });

                graph.add_relationship(Relationship::InputsTo { parameter: sp_u, calculation: calc_trans });
                graph.add_relationship(Relationship::InputsTo { parameter: sp_fx, calculation: calc_trans });
            }
        } else {
            let z_idx = graph.add_entity(EntityData::Space(SpaceData {
                name: "Main Zone".into(), volume: geom.total_conditioned_volume, net_floor_area: geom.total_floor_area,
                room_depth: None, ceiling_height: None, is_critical_room: false, unheated_space_type: None
            }));
            graph.add_relationship(Relationship::Aggregates { parent: b_idx, child: z_idx });

            let w_idx = graph.add_entity(EntityData::Wall(WallData {
                area: a_wall_total, u_value: u_wall, thickness: 0.3,
                r_si: 0.13, r_se: 0.04, f_neig: 1.0, f_x: 1.0, solar_absorptance: 0.6, is_roof: false
            }));
            graph.add_relationship(Relationship::BoundsSpace { space: z_idx, boundary_element: w_idx });
            
            let wp_u = graph.add_entity(EntityData::Property(PropertyData { 
                name: "U-Value".into(), value: format!("{:.2}", u_wall), unit: "W/(m²K)".into(),
                doc: Some("Measures the rate of heat transfer through a structure. Lower values indicate better insulation.".into())
            }));
            graph.add_relationship(Relationship::HasProperty { host: w_idx, property: wp_u });
        }
    }
    graph
}"""

    new_content = content[:start_idx] + new_func + content[end_idx:]
    
    with open(file_path, 'w') as f:
        f.write(new_content)
        
replace_function('/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio/coda/client/bin/assistant/sketchpad-rs/src/lib.rs')
