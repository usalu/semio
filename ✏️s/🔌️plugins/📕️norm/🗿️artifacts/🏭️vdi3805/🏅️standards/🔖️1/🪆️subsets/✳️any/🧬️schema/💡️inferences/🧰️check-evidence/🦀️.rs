//! 🧰️ VDI 3805 compliance helpers: part labels, localized copy, subject paths, edition profile lookup, per-sheet attribute extraction and the remedies shared by the sheet checks.

use super::*;

//#region 🔖️ComplianceReport
pub(super) fn part_label(sheet: u16) -> String {
    match sheet {
        1 => "VDI 3805 Blatt 1".into(),
        n => format!("VDI 3805 Blatt {n}"),
    }
}

pub(super) fn copy(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub(super) fn product_path(article: &str, field: &str) -> String {
    format!("catalog.products[id={article}].{field}")
}

pub(super) fn curve_point_path(curve_id: &str, index: usize, axis: &str) -> String {
    format!("curves.{curve_id}.points[{index}].{axis}")
}

pub(super) fn nearest_dn(dn: u16) -> u16 {
    VALVE_DN_SERIES.iter().copied().min_by_key(|c| (*c as i32 - dn as i32).unsigned_abs()).unwrap_or(50)
}

pub(super) fn is_operative(status: SchemaStatus) -> bool {
    matches!(status, SchemaStatus::Published | SchemaStatus::Checked)
}

/// 📋 Mandatory Blatt attribute keys carried in native 210 (or typed attributes) per sheet + edition profile.
pub(super) fn sheet_mandatory_keys(sheet: u16, profile: EditionProfileChoice) -> &'static [&'static str] {
    let current = match sheet {
        2 => &["dn", "kvs", "pressure_class", "connection_type", "authority_min", "authority_max"][..],
        3 => &["standard_output_w", "n", "length_m", "height_m", "depth_m", "connection_type"][..],
        4 => &["dn", "pressure_class", "connection_type", "outer_diameter_m", "wall_thickness_m"][..],
        5 => &["dn_suction", "dn_discharge", "nominal_flow_m3_s", "nominal_head_m", "motor_power_w", "hydraulic_efficiency"][..],
        6 => &["nominal_heat_output_w", "fuel_type", "flow_temp_max_c", "return_temp_min_c"][..],
        7 => &["dn", "pressure_class", "connection_type", "volume_m3"][..],
        8 | 10 | 14 | 18 | 33 | 36 | 37 | 40 | 42 | 100 => &["product_group", "type_code", "dn", "pressure_class"][..],
        9 | 11 | 28 | 32 | 34 | 35 | 38 | 41 | 43 | 44 | 45 | 50 | 51 | 52 | 54 | 55 | 99 => {
            &["product_group", "type_code", "dn"][..]
        }
        16 | 17 => &["product_group", "type_code", "airflow_m3_s", "pressure_drop_pa"][..],
        19 => &["product_group", "type_code", "airflow_m3_s", "filter_class"][..],
        20 | 21 | 22 | 23 | 24 | 26 | 27 | 29 => &["product_group", "type_code", "airflow_m3_s"][..],
        53 => &["product_group", "type_code", "dn", "nominal_heat_output_w", "cop"][..],
        60..=66 => &["dn", "pressure_class", "connection_type", "axial_force_n"][..],
        _ => &["product_group", "type_code"][..],
    };
    let legacy = match sheet {
        8 | 10 | 14 | 18 | 33 | 36 | 37 | 40 | 42 | 53 | 100 => &["product_group", "dn"][..],
        4 | 7 | 60..=66 => &["dn", "connection_type"][..],
        16 | 17 | 19 => &["product_group", "type_code"][..],
        _ => current,
    };
    match profile {
        EditionProfileChoice::Legacy => legacy,
        EditionProfileChoice::Current => current,
    }
}

/// 📏 Blatt-sourced numeric domains (VDI 3805 Part 1 §5 field ranges + product-sheet tables).
/// Citations: DN series DIN EN ISO 6708 / Blatt tables; airflow/pressure VDI 3805-16/19 §4; COP VDI 3805-53 §4.2.
pub(super) fn sheet_numeric_bounds(sheet: u16) -> &'static [(&'static str, f64, f64)] {
    const BOUNDS_AIR: &[(&str, f64, f64)] = &[
        ("airflow_m3_s", SHEET_NUMERIC_BOUNDS_19_AIRFLOW.0, SHEET_NUMERIC_BOUNDS_19_AIRFLOW.1),
        ("pressure_drop_pa", SHEET_NUMERIC_BOUNDS_16_PRESSURE_DROP_PA.0, SHEET_NUMERIC_BOUNDS_16_PRESSURE_DROP_PA.1),
    ];
    const BOUNDS_53: &[(&str, f64, f64)] = &[
        ("dn", SHEET_NUMERIC_BOUNDS_53_DN.0, SHEET_NUMERIC_BOUNDS_53_DN.1),
        ("nominal_heat_output_w", SHEET_NUMERIC_BOUNDS_53_HEAT_W.0, SHEET_NUMERIC_BOUNDS_53_HEAT_W.1),
        ("cop", SHEET_NUMERIC_BOUNDS_53_COP.0, SHEET_NUMERIC_BOUNDS_53_COP.1),
    ];
    const BOUNDS_DN: &[(&str, f64, f64)] = &[("dn", SHEET_NUMERIC_BOUNDS_DN.0, SHEET_NUMERIC_BOUNDS_DN.1)];
    const BOUNDS_4: &[(&str, f64, f64)] = &[
        ("dn", SHEET_NUMERIC_BOUNDS_DN_PIPE.0, SHEET_NUMERIC_BOUNDS_DN_PIPE.1),
        ("outer_diameter_m", SHEET_NUMERIC_BOUNDS_OUTER_DIAMETER_M.0, SHEET_NUMERIC_BOUNDS_OUTER_DIAMETER_M.1),
        ("wall_thickness_m", SHEET_NUMERIC_BOUNDS_WALL_THICKNESS_M.0, SHEET_NUMERIC_BOUNDS_WALL_THICKNESS_M.1),
    ];
    const BOUNDS_7: &[(&str, f64, f64)] = &[
        ("dn", SHEET_NUMERIC_BOUNDS_DN.0, SHEET_NUMERIC_BOUNDS_DN.1),
        ("volume_m3", SHEET_NUMERIC_BOUNDS_VOLUME_M3.0, SHEET_NUMERIC_BOUNDS_VOLUME_M3.1),
    ];
    const BOUNDS_60: &[(&str, f64, f64)] = &[
        ("dn", SHEET_NUMERIC_BOUNDS_DN.0, SHEET_NUMERIC_BOUNDS_DN.1),
        ("axial_force_n", SHEET_NUMERIC_BOUNDS_AXIAL_FORCE_N.0, SHEET_NUMERIC_BOUNDS_AXIAL_FORCE_N.1),
    ];
    match sheet {
        4 => BOUNDS_4,
        7 => BOUNDS_7,
        8 | 10 | 14 | 18 | 33 | 36 | 37 | 40 | 42 | 100 => BOUNDS_DN,
        9 | 11 | 28 | 32 | 34 | 35 | 38 | 41 | 43 | 44 | 45 | 50 | 51 | 52 | 54 | 55 | 99 => BOUNDS_DN,
        16 | 17 | 19 | 20 | 21 | 22 | 23 | 24 | 26 | 27 | 29 => BOUNDS_AIR,
        53 => BOUNDS_53,
        60..=66 => BOUNDS_60,
        _ => &[],
    }
}

/// 🔌️ VDI 3805-1 §5.2 — common connection port ids on product geometry.
pub(super) const CONNECTION_ID_CODES: &[&str] = &["in", "out", "drain", "vent", "bypass"];

/// 💧 VDI 3805-1 §5.2 — connection medium codes.
pub(super) const CONNECTION_MEDIUM_CODES: &[&str] = &["water", "air", "glycol", "steam", "refrigerant"];

pub(super) fn code_list_for_key(sheet: u16, key: &str) -> Option<&'static [&'static str]> {
    match key {
        "connection_type" => Some(CONNECTION_TYPE_CODES),
        "pressure_class" => Some(PRESSURE_CLASS_CODES),
        "filter_class" if sheet == 19 => Some(FILTER_CLASS_CODES),
        "type_code" => Some(TYPE_CODE_CODES),
        "fuel_type" => Some(&["gas", "oil", "electric", "biomass", "district"]),
        _ => None,
    }
}

pub(super) fn record_kv(records: &[NativeRecord]) -> BTreeMap<String, String> {
    let mut kv = BTreeMap::new();
    for record in records {
        if record.family.0.as_str() == "210" {
            let mut it = record.fields.iter().skip(1);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if k.parse::<f64>().is_err() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        } else if record.family.0.as_str() == "110" && record.fields.len() > 4 {
            let mut it = record.fields.iter().skip(4);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if k.parse::<f64>().is_err() {
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
    kv
}

pub(super) fn profile_for_sheet(document: &Vdi3805Snapshot, sheet: u16) -> EditionProfileChoice {
    document
        .edition_profile
        .get(&sheet.to_string())
        .copied()
        .unwrap_or(EditionProfileChoice::Current)
}

pub(super) fn valve_attrs_from_product(product: &CatalogueProduct) -> ValveHeatingAttributes {
    match &product.configuration.attributes {
        SheetAttributes::ValveHeating(a) => a.clone(),
        _ => match attributes_from_records(product.sheet, &product.records) {
            SheetAttributes::ValveHeating(a) => a,
            _ => ValveHeatingAttributes::from_kvs_m3_h(0, 0.0, "", "", 0.0, 0.0),
        },
    }
}

pub(super) fn radiator_attrs_from_product(product: &CatalogueProduct) -> RadiatorAttributes {
    match &product.configuration.attributes {
        SheetAttributes::Radiator(a) => a.clone(),
        _ => match attributes_from_records(product.sheet, &product.records) {
            SheetAttributes::Radiator(a) => a,
            _ => RadiatorAttributes {
                standard_output_w: 0.0,
                heat_exponent_n: 0.0,
                length_m: 0.0,
                height_m: 0.0,
                depth_m: 0.0,
                connection_type: String::new(),
            },
        },
    }
}

pub(super) fn pump_attrs_from_product(product: &CatalogueProduct) -> PumpHeatingAttributes {
    match &product.configuration.attributes {
        SheetAttributes::PumpHeating(a) => a.clone(),
        _ => match attributes_from_records(product.sheet, &product.records) {
            SheetAttributes::PumpHeating(a) => a,
            _ => PumpHeatingAttributes {
                dn_suction: 0,
                dn_discharge: 0,
                nominal_flow_m3_s: 0.0,
                nominal_head_m: 0.0,
                motor_power_w: 0.0,
                hydraulic_efficiency: 0.0,
                qh_curve_ref: None,
            },
        },
    }
}

pub(super) fn heat_attrs_from_product(product: &CatalogueProduct) -> HeatGeneratorAttributes {
    match &product.configuration.attributes {
        SheetAttributes::HeatGenerator(a) => a.clone(),
        _ => match attributes_from_records(product.sheet, &product.records) {
            SheetAttributes::HeatGenerator(a) => a,
            _ => HeatGeneratorAttributes {
                nominal_heat_output_w: 0.0,
                fuel_type: String::new(),
                flow_temp_max_c: 0.0,
                return_temp_min_c: 0.0,
            },
        },
    }
}

pub(super) fn attr_leaf_path(article: &str, leaf: &str) -> String {
    product_path(article, &format!("configuration.attributes.{leaf}"))
}

pub(super) fn generic_entry_value_path(article: &str, entries: &[GenericAttribute], key: &str) -> String {
    if let Some((i, _)) = entries.iter().enumerate().find(|(_, e)| e.key.eq_ignore_ascii_case(key)) {
        attr_leaf_path(article, &format!("entries[{i}].value"))
    } else {
        product_path(article, "identity.productGroup")
    }
}

pub(super) fn sync_divergence_remedy(article: &str, configured: &SheetAttributes, derived: &SheetAttributes) -> Remedy {
    match (configured, derived) {
        (SheetAttributes::ValveHeating(a), SheetAttributes::ValveHeating(b)) => {
            if a.dn != b.dn {
                return Remedy::exactly(
                    SubjectRef::new(article, attr_leaf_path(article, "dn"), copy("Nominal diameter DN", "Nennweite DN")),
                    Quantity::new(QuantityKind::Dimensionless, a.dn as f64),
                    Quantity::new(QuantityKind::Dimensionless, b.dn as f64),
                    copy(format!("Set attributes.dn to {0} from native 210 (records authoritative).", b.dn), format!("attributes.dn auf {0} aus nativem 210 setzen (Sätze maßgeblich).", b.dn)),
                );
            }
            if (a.kvs_m3_s - b.kvs_m3_s).abs() >= 1e-12 {
                return Remedy::exactly(
                    SubjectRef::new(article, attr_leaf_path(article, "kvsM3S"), copy("Flow coefficient kvs", "Durchflusskoeffizient kvs")),
                    Quantity::new(QuantityKind::Volume, a.kvs_m3_s),
                    Quantity::new(QuantityKind::Volume, b.kvs_m3_s),
                    copy("Align attributes.kvsM3S with the native 210 kvs value.", "attributes.kvsM3S an den nativen 210-kvs-Wert angleichen."),
                );
            }
            Remedy::exactly(
                SubjectRef::new(article, attr_leaf_path(article, "pressureClass"), copy("Pressure class", "Druckstufe")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                copy("Align pressureClass with the native 210 pressure_class.", "pressureClass an natives 210-pressure_class angleichen."),
            )
        }
        (SheetAttributes::Radiator(a), SheetAttributes::Radiator(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "standardOutputW"), copy("Standard output", "Normwärmeleistung")),
            Quantity::new(QuantityKind::Power, a.standard_output_w),
            Quantity::new(QuantityKind::Power, b.standard_output_w),
            copy("Align standardOutputW with native 210 Φ.", "standardOutputW an natives 210-Φ angleichen."),
        ),
        (SheetAttributes::PumpHeating(a), SheetAttributes::PumpHeating(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "nominalFlowM3S"), copy("Nominal flow", "Nennvolumenstrom")),
            Quantity::new(QuantityKind::Volume, a.nominal_flow_m3_s),
            Quantity::new(QuantityKind::Volume, b.nominal_flow_m3_s),
            copy("Align nominalFlowM3S with native 210 Q.", "nominalFlowM3S an natives 210-Q angleichen."),
        ),
        (SheetAttributes::HeatGenerator(a), SheetAttributes::HeatGenerator(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "nominalHeatOutputW"), copy("Nominal heat output", "Nennwärmeleistung")),
            Quantity::new(QuantityKind::Power, a.nominal_heat_output_w),
            Quantity::new(QuantityKind::Power, b.nominal_heat_output_w),
            copy("Align nominalHeatOutputW with native 210 Qn.", "nominalHeatOutputW an natives 210-Qn angleichen."),
        ),
        (SheetAttributes::Generic(g), SheetAttributes::Generic(d)) => {
            let key = d.entries.first().map(|e| e.key.as_str()).unwrap_or("product_group");
            let req = d.get(key).unwrap_or("").parse::<f64>().unwrap_or(1.0);
            let cur = g.get(key).unwrap_or("").parse::<f64>().unwrap_or(0.0);
            Remedy::exactly(
                SubjectRef::new(article, generic_entry_value_path(article, &g.entries, key), copy("Generic attribute", "Generisches Attribut")),
                Quantity::new(QuantityKind::Dimensionless, cur),
                Quantity::new(QuantityKind::Dimensionless, req),
                copy(format!("Align generic attribute `{key}` with native 210."), format!("Generisches Attribut `{key}` an natives 210 angleichen.")),
            )
        }
        (_, SheetAttributes::ValveHeating(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "dn"), copy("Nominal diameter DN", "Nennweite DN")),
            Quantity::new(QuantityKind::Dimensionless, 0.0),
            Quantity::new(QuantityKind::Dimensionless, b.dn as f64),
            copy(
                "configuration.attributes must be valveHeating matching native 210; set dn from records then re-sync.",
                "configuration.attributes muss valveHeating gemäß 210 sein; dn aus Sätzen setzen und neu synchronisieren.",
            ),
        ),
        (_, SheetAttributes::Radiator(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "standardOutputW"), copy("Standard output", "Normwärmeleistung")),
            Quantity::new(QuantityKind::Power, 0.0),
            Quantity::new(QuantityKind::Power, b.standard_output_w.max(500.0)),
            copy("configuration.attributes must be radiator matching native 210 Φ.", "configuration.attributes muss radiator gemäß 210-Φ sein."),
        ),
        (_, SheetAttributes::PumpHeating(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "nominalFlowM3S"), copy("Nominal flow", "Nennvolumenstrom")),
            Quantity::new(QuantityKind::Volume, 0.0),
            Quantity::new(QuantityKind::Volume, b.nominal_flow_m3_s.max(0.0025)),
            copy("configuration.attributes must be pumpHeating matching native 210 Q.", "configuration.attributes muss pumpHeating gemäß 210-Q sein."),
        ),
        (_, SheetAttributes::HeatGenerator(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "nominalHeatOutputW"), copy("Nominal heat output", "Nennwärmeleistung")),
            Quantity::new(QuantityKind::Power, 0.0),
            Quantity::new(QuantityKind::Power, b.nominal_heat_output_w.max(10_000.0)),
            copy("configuration.attributes must be heatGenerator matching native 210 Qn.", "configuration.attributes muss heatGenerator gemäß 210-Qn sein."),
        ),
        _ => Remedy::exactly(
            SubjectRef::new(article, product_path(article, "identity.productGroup"), copy("Product group", "Produktgruppe")),
            Quantity::new(QuantityKind::Dimensionless, 0.0),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            copy("Align typed attributes with native 210 records.", "Typisierte Attribute an native 210-Sätze angleichen."),
        ),
    }
}

pub(super) fn mono_increasing_remedy(curve: &CharacteristicCurve) -> Option<(usize, f64, f64)> {
    for (i, w) in curve.points.windows(2).enumerate() {
        if w[1].y + 1e-12 < w[0].y {
            return Some((i + 1, w[1].y, w[0].y));
        }
        if w[1].x + 1e-12 < w[0].x {
            return Some((i + 1, w[1].x, w[0].x));
        }
    }
    None
}

pub(super) fn mono_decreasing_y_remedy(curve: &CharacteristicCurve) -> Option<(usize, f64, f64)> {
    for (i, w) in curve.points.windows(2).enumerate() {
        if w[1].y > w[0].y + 1e-12 {
            return Some((i + 1, w[1].y, w[0].y));
        }
    }
    None
}
//#endregion 🔖️ComplianceReport
