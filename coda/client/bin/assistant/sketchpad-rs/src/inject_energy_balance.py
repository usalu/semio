import sys

def main():
    file_path = "/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio/coda/client/bin/assistant/sketchpad-rs/src/lib.rs"
    with open(file_path, "r") as f:
        content = f.read()

    # 1. Update EnergyAnalysisResult struct
    if "pub q_h_nd_kwh: f64," not in content:
        content = content.replace(
            "pub final_heating_demand_kwh: f64,",
            "pub final_heating_demand_kwh: f64,\n    pub q_h_nd_kwh: f64,"
        )

    # 2. Update instantiation
    if "q_h_nd_kwh: q_h_nd," not in content:
        content = content.replace(
            "final_heating_demand_kwh: q_final,",
            "final_heating_demand_kwh: q_final,\n        q_h_nd_kwh: q_h_nd,"
        )

    # 3. Add the final balance nodes at the end of map_state_to_graph
    addition = """
    // Final Energy Balance Calculation
    let calc_heating = graph.add_entity(EntityData::Calculation(CalculationData {
        name: "Heating Demand (Q_{H,nd})".into(),
        formula: "Q_{H,nd} = Q_T + Q_V - \\eta \\cdot (Q_S + Q_I)".into(),
        doc: "Total heat energy required to maintain the setpoint temperature, after subtracting useful solar and internal gains.".into(),
    }));

    let p_qt = graph.add_entity(EntityData::Property(PropertyData {
        name: "Q_T (Transmission)".into(), value: format!("{:.0}", res.transmission_loss_kwh), unit: "kWh/a".into(),
        doc: None
    }));
    let p_qv = graph.add_entity(EntityData::Property(PropertyData {
        name: "Q_V (Ventilation)".into(), value: format!("{:.0}", res.ventilation_loss_kwh), unit: "kWh/a".into(),
        doc: None
    }));
    let p_qs = graph.add_entity(EntityData::Property(PropertyData {
        name: "Q_S (Solar)".into(), value: format!("{:.0}", res.solar_gains_kwh), unit: "kWh/a".into(),
        doc: None
    }));
    let p_qi = graph.add_entity(EntityData::Property(PropertyData {
        name: "Q_I (Internal)".into(), value: format!("{:.0}", res.internal_gains_kwh), unit: "kWh/a".into(),
        doc: None
    }));

    graph.add_relationship(Relationship::InputsTo { parameter: p_qt, calculation: calc_heating });
    graph.add_relationship(Relationship::InputsTo { parameter: p_qv, calculation: calc_heating });
    graph.add_relationship(Relationship::InputsTo { parameter: p_qs, calculation: calc_heating });
    graph.add_relationship(Relationship::InputsTo { parameter: p_qi, calculation: calc_heating });

    let r_heating = graph.add_entity(EntityData::Property(PropertyData {
        name: "Heating Demand Result".into(), value: format!("{:.0}", res.q_h_nd_kwh), unit: "kWh/a".into(),
        doc: Some("Final Heating Demand (Q_{H,nd}).".into())
    }));
    graph.add_relationship(Relationship::OutputsTo { calculation: calc_heating, result: r_heating });

    // Final Delivered Energy Calculation
    let calc_final = graph.add_entity(EntityData::Calculation(CalculationData {
        name: "Delivered Energy (Q_{End})".into(),
        formula: "Q_{End} = (Q_{H,nd} + Q_{d,h} + Q_{s,h}) \\cdot e_{g,h}".into(),
        doc: "Total energy billed by the utility, factoring in the efficiency of the heating system.".into(),
    }));

    graph.add_relationship(Relationship::InputsTo { parameter: r_heating, calculation: calc_final });
    // p_heat is already added earlier, we could link it here but p_heat was created way earlier in scope.
    // We will just add the efficiency factor directly as an input to represent the system.
    let p_eff = graph.add_entity(EntityData::Property(PropertyData {
        name: "e_{g,h} (Efficiency)".into(), value: format!("{:.2}", res.final_heating_demand_kwh / res.q_h_nd_kwh), unit: "-".into(),
        doc: Some("Total system loss factor of the heating generator.".into())
    }));
    graph.add_relationship(Relationship::InputsTo { parameter: p_eff, calculation: calc_final });

    let r_final = graph.add_entity(EntityData::Property(PropertyData {
        name: "Final Energy Result".into(), value: format!("{:.0}", res.final_heating_demand_kwh), unit: "kWh/a".into(),
        doc: Some("Total Delivered Energy (Q_{End}).".into())
    }));
    graph.add_relationship(Relationship::OutputsTo { calculation: calc_final, result: r_final });

"""
    if "calc_heating = graph.add_entity" not in content:
        content = content.replace(
            "    graph\n}",
            addition + "\n    graph\n}"
        )

    with open(file_path, "w") as f:
        f.write(content)

    print("Success")

if __name__ == "__main__":
    main()
