import sys

def main():
    file_path = "/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio/coda/client/bin/assistant/sketchpad-rs/src/lib.rs"
    with open(file_path, "r") as f:
        content = f.read()

    struct_def = """
pub struct EnergyAnalysisResult {
    pub transmission_loss_kwh: f64,
    pub ventilation_loss_kwh: f64,
    pub solar_gains_kwh: f64,
    pub internal_gains_kwh: f64,
    pub final_heating_demand_kwh: f64,
    pub h_t_wb: f64,
    pub f_x: f64,
    pub n50: f64,
    pub r_se: f64,
    pub r_si: f64,
    pub h_t_d: f64,
    pub h_t_iu: f64,
    pub h_v_inf: f64,
    pub h_v_win: f64,
    pub h_v_mech: f64,
    pub h_tr: f64,
    pub h_ve: f64,
}

"""

    # Add struct before map_state_to_graph
    if "pub struct EnergyAnalysisResult" not in content:
        content = content.replace(
            "fn map_state_to_graph(state: &State, u_wall: f64, u_roof: f64, u_floor: f64, u_window: f64) -> crate::ontology::BuildingKnowledgeGraph {",
            struct_def + "fn map_state_to_graph(state: &State, u_wall: f64, u_roof: f64, u_floor: f64, u_window: f64, res: &EnergyAnalysisResult) -> crate::ontology::BuildingKnowledgeGraph {"
        )
    
    # Instantiate results and call map_state_to_graph
    instantiation = """
    let results = EnergyAnalysisResult {
        transmission_loss_kwh: q_ht_tr_total,
        ventilation_loss_kwh: q_ht_ve,
        solar_gains_kwh: q_sol,
        internal_gains_kwh: q_int,
        final_heating_demand_kwh: q_final,
        h_t_wb,
        f_x: f_x_ground,
        n50,
        r_se: 0.04,
        r_si: 0.13,
        h_t_d,
        h_t_iu,
        h_v_inf,
        h_v_win,
        h_v_mech,
        h_tr,
        h_ve,
    };
    let knowledge_graph = map_state_to_graph(state, u_wall, u_roof, u_floor, u_window, &results);
"""
    content = content.replace(
        "let knowledge_graph = map_state_to_graph(state, u_wall, u_roof, u_floor, u_window);",
        instantiation
    )

    # Inject the mapping logic inside map_state_to_graph
    # We will find the end of `graph.add_relationship(Relationship::InputsTo { parameter: p_vol, calculation: calc_vent });`
    # and add ventilation missing parameters
    vent_addition = """
    let p_n50 = graph.add_entity(EntityData::Property(PropertyData {
        name: "n50 (Blower Door)".into(), value: format!("{:.2}", res.n50), unit: "1/h".into(),
        doc: Some("Air change rate at 50 Pa pressure difference. Measures building envelope airtightness.".into())
    }));
    let p_hv_inf = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{V,inf}".into(), value: format!("{:.1}", res.h_v_inf), unit: "W/K".into(),
        doc: Some("Ventilation heat transfer coefficient for infiltration through envelope leaks.".into())
    }));
    let p_hv_win = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{V,win}".into(), value: format!("{:.1}", res.h_v_win), unit: "W/K".into(),
        doc: Some("Ventilation heat transfer coefficient for window airing.".into())
    }));
    let p_hv_mech = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{V,mech}".into(), value: format!("{:.1}", res.h_v_mech), unit: "W/K".into(),
        doc: Some("Ventilation heat transfer coefficient for mechanical ventilation (considering heat recovery).".into())
    }));
    let p_h_ve = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_V (Total)".into(), value: format!("{:.1}", res.h_ve), unit: "W/K".into(),
        doc: Some("Total ventilation heat transfer coefficient.".into())
    }));
    
    graph.add_relationship(Relationship::InputsTo { parameter: p_n50, calculation: calc_vent });
    graph.add_relationship(Relationship::InputsTo { parameter: p_hv_inf, calculation: calc_vent });
    graph.add_relationship(Relationship::InputsTo { parameter: p_hv_win, calculation: calc_vent });
    graph.add_relationship(Relationship::InputsTo { parameter: p_hv_mech, calculation: calc_vent });
    graph.add_relationship(Relationship::InputsTo { parameter: p_h_ve, calculation: calc_vent });

    let r_vent = graph.add_entity(EntityData::Property(PropertyData {
        name: "Ventilation Loss Result".into(), value: format!("{:.0}", res.ventilation_loss_kwh), unit: "kWh/a".into(),
        doc: Some("Final calculated annual heat loss due to ventilation (Q_v).".into())
    }));
    graph.add_relationship(Relationship::OutputsTo { calculation: calc_vent, result: r_vent });
"""
    if "p_n50 = graph.add_entity" not in content:
        content = content.replace(
            "graph.add_relationship(Relationship::InputsTo { parameter: p_vol, calculation: calc_vent });",
            "graph.add_relationship(Relationship::InputsTo { parameter: p_vol, calculation: calc_vent });\n" + vent_addition
        )

    # Now for transmission and solar/internal
    # Find `doc: "Direct transmission heat loss through the opaque envelope and windows.".into(),`
    trans_addition = """
    let p_ht_d = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{T,D}".into(), value: format!("{:.1}", res.h_t_d), unit: "W/K".into(),
        doc: Some("Direct transmission heat transfer coefficient to the exterior environment.".into())
    }));
    let p_ht_iu = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{T,iu}".into(), value: format!("{:.1}", res.h_t_iu), unit: "W/K".into(),
        doc: Some("Transmission heat transfer coefficient to unheated spaces.".into())
    }));
    let p_ht_wb = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_{T,wb}".into(), value: format!("{:.1}", res.h_t_wb), unit: "W/K".into(),
        doc: Some("Transmission heat transfer coefficient for thermal bridges.".into())
    }));
    let p_fx = graph.add_entity(EntityData::Property(PropertyData {
        name: "f_x (Ground)".into(), value: format!("{:.2}", res.f_x), unit: "-".into(),
        doc: Some("Temperature weighting factor for ground-coupled components.".into())
    }));
    let p_rse = graph.add_entity(EntityData::Property(PropertyData {
        name: "R_se".into(), value: format!("{:.2}", res.r_se), unit: "m²K/W".into(),
        doc: Some("External surface thermal resistance.".into())
    }));
    let p_rsi = graph.add_entity(EntityData::Property(PropertyData {
        name: "R_si".into(), value: format!("{:.2}", res.r_si), unit: "m²K/W".into(),
        doc: Some("Internal surface thermal resistance.".into())
    }));
    let p_h_tr = graph.add_entity(EntityData::Property(PropertyData {
        name: "H_T (Total)".into(), value: format!("{:.1}", res.h_tr), unit: "W/K".into(),
        doc: Some("Total transmission heat transfer coefficient.".into())
    }));

    graph.add_relationship(Relationship::InputsTo { parameter: p_ht_d, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_ht_iu, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_ht_wb, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_fx, calculation: calc_trans });
    graph.add_relationship(Relationship::InputsTo { parameter: p_h_tr, calculation: calc_trans });

    let r_trans = graph.add_entity(EntityData::Property(PropertyData {
        name: "Transmission Loss Result".into(), value: format!("{:.0}", res.transmission_loss_kwh), unit: "kWh/a".into(),
        doc: Some("Final calculated annual heat loss due to transmission (Q_T).".into())
    }));
    graph.add_relationship(Relationship::OutputsTo { calculation: calc_trans, result: r_trans });

    // Assuming calc_internal and calc_solar exist in scope here or we will link them directly after they are created.
"""
    if "p_ht_d = graph.add_entity" not in content:
        content = content.replace(
            "doc: \"Direct transmission heat loss through the opaque envelope and windows.\".into(),\n                }));",
            "doc: \"Direct transmission heat loss through the opaque envelope and windows.\".into(),\n                }));\n" + trans_addition
        )

    # Link results for internal gains and solar gains
    # For internal gains:
    int_addition = """
                let r_int = graph.add_entity(EntityData::Property(PropertyData {
                    name: "Internal Gains Result".into(), value: format!("{:.0}", res.internal_gains_kwh), unit: "kWh/a".into(),
                    doc: Some("Final calculated annual heat gains from people and equipment (Q_I).".into())
                }));
                graph.add_relationship(Relationship::OutputsTo { calculation: calc_internal, result: r_int });
"""
    if "r_int = graph.add_entity" not in content:
        content = content.replace(
            "graph.add_relationship(Relationship::InputsTo { parameter: p_area, calculation: calc_internal });",
            "graph.add_relationship(Relationship::InputsTo { parameter: p_area, calculation: calc_internal });\n" + int_addition
        )

    # For solar gains:
    sol_addition = """
                    let r_sol = graph.add_entity(EntityData::Property(PropertyData {
                        name: "Solar Gains Result".into(), value: format!("{:.0}", res.solar_gains_kwh), unit: "kWh/a".into(),
                        doc: Some("Final calculated annual solar heat gains through windows (Q_S).".into())
                    }));
                    graph.add_relationship(Relationship::OutputsTo { calculation: calc_solar, result: r_sol });
"""
    if "r_sol = graph.add_entity" not in content:
        content = content.replace(
            "graph.add_relationship(Relationship::InputsTo { parameter: win_area_prop, calculation: calc_solar });",
            "graph.add_relationship(Relationship::InputsTo { parameter: win_area_prop, calculation: calc_solar });\n" + sol_addition
        )

    with open(file_path, "w") as f:
        f.write(content)

    print("Success")

if __name__ == "__main__":
    main()
