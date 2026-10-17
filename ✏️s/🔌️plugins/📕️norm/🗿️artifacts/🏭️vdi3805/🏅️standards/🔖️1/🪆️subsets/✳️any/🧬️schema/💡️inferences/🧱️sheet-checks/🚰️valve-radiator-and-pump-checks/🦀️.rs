//! 🚰️ VDI 3805 valve, radiator and pump sheet checks.

use super::*;

pub(super) fn check_valve(document: &Vdi3805Snapshot, product: &CatalogueProduct, attrs: &ValveHeatingAttributes) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let subject = subject_product(product);
    let part = part_label(2);

    let dn_ok = VALVE_DN_SERIES.contains(&attrs.dn);
    let nearest = nearest_dn(attrs.dn);
    let mut dn_builder = CheckResult::assess(format!("vdi3805.2.dn.{article}"), part.clone(), clause("2", "4.2.1"), subject.clone(), copy("Nominal diameter DN series", "Nennweite DN-Reihe"))
        .annex(ANNEX)
        .utilization(Quantity::new(QuantityKind::Dimensionless, if dn_ok { 1.0 } else { 2.0 }), Quantity::new(QuantityKind::Dimensionless, 1.0));
    if dn_ok {
        dn_builder = dn_builder.explanation(copy(format!("DN {} is in the admitted series.", attrs.dn), format!("DN {} liegt in der zugelassenen Reihe.", attrs.dn)));
    } else {
        dn_builder = dn_builder
            .explanation(copy(format!("DN {} is not in the VDI 3805 Blatt 2 series.", attrs.dn), format!("DN {} ist nicht in der VDI-3805-Blatt-2-Reihe.", attrs.dn)))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, "configuration.attributes.dn"), copy("Nominal diameter", "Nennweite")),
                Quantity::new(QuantityKind::Dimensionless, attrs.dn as f64),
                Quantity::new(QuantityKind::Dimensionless, nearest as f64),
                copy(format!("Change DN from {} to {}.", attrs.dn, nearest), format!("DN von {} auf {} ändern.", attrs.dn, nearest)),
            ));
    }
    out.push(dn_builder.build());

    let min_kvs = valve_min_kvs_m3_h(if dn_ok { attrs.dn } else { nearest });
    let kvs_h = attrs.kvs_m3_h();
    let mut kvs_builder = CheckResult::assess(format!("vdi3805.2.kvs.{article}"), part.clone(), clause("2", "4.2.2"), subject.clone(), copy("Flow coefficient kvs", "Durchflusskoeffizient kvs"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Volume, attrs.kvs_m3_s), Quantity::new(QuantityKind::Volume, min_kvs / 3600.0));
    if kvs_h + 1e-12 >= min_kvs {
        kvs_builder = kvs_builder.explanation(copy(format!("kvs = {kvs_h:.3} m³/h ≥ {min_kvs:.3} m³/h for DN {}.", attrs.dn), format!("kvs = {kvs_h:.3} m³/h ≥ {min_kvs:.3} m³/h für DN {}.", attrs.dn)));
    } else {
        kvs_builder = kvs_builder
            .explanation(copy(format!("kvs = {kvs_h:.3} m³/h below Blatt 2 minimum {min_kvs:.3} m³/h.",), format!("kvs = {kvs_h:.3} m³/h unter Blatt-2-Minimum {min_kvs:.3} m³/h.")))
            .remedy(Remedy::at_least(
                SubjectRef::new(article, product_path(article, "configuration.attributes.kvsM3S"), copy("Flow coefficient kvs", "Durchflusskoeffizient kvs")),
                Quantity::new(QuantityKind::Volume, attrs.kvs_m3_s),
                Quantity::new(QuantityKind::Volume, min_kvs / 3600.0),
                copy(format!("Increase kvs from {kvs_h:.3} m³/h to at least {min_kvs:.3} m³/h."), format!("kvs von {kvs_h:.3} m³/h auf mindestens {min_kvs:.3} m³/h erhöhen.")),
            ));
    }
    out.push(kvs_builder.build());

    let pc_ok = !attrs.pressure_class.is_empty();
    let mut pc = CheckResult::assess(format!("vdi3805.2.pn.{article}"), part.clone(), clause("2", "4.2.3"), subject.clone(), copy("Pressure class", "Druckstufe"))
        .annex(ANNEX)
        .status(if pc_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if pc_ok {
        pc = pc.explanation(copy(format!("Pressure class {} present.", attrs.pressure_class), format!("Druckstufe {} vorhanden.", attrs.pressure_class)));
    } else {
        pc = pc
            .explanation(copy("Pressure class is empty.", "Druckstufe fehlt."))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, "configuration.attributes.pressureClass"), copy("Pressure class", "Druckstufe")),
                vec!["PN6".into(), "PN10".into(), "PN16".into(), "PN25".into()],
                copy("Set pressure class to a PN series value (e.g. PN16).", "Druckstufe auf einen PN-Wert setzen (z. B. PN16)."),
            ));
    }
    out.push(pc.build());

    let ct = attrs.connection_type.as_str();
    let ct_ord = CONNECTION_TYPE_CODES.iter().position(|c| *c == ct).map(|i| (i + 1) as f64).unwrap_or(0.0);
    let ct_ok = ct_ord > 0.0;
    let mut ct_check = CheckResult::assess(format!("vdi3805.2.connectionType.{article}"), part.clone(), clause("2", "4.2.5"), subject.clone(), copy("Connection type", "Anschlussart"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Dimensionless, ct_ord), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .status(if ct_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if ct_ok {
        ct_check = ct_check.explanation(copy(format!("connectionType `{ct}` is in the Blatt 2 code list."), format!("connectionType `{ct}` ist in der Blatt-2-Codeliste.")));
    } else {
        ct_check = ct_check
            .explanation(copy(
                if ct.is_empty() { "Mandatory connectionType omitted from record 210.".into() } else { format!("connectionType `{ct}` is not in the Blatt 2 code list.") },
                if ct.is_empty() { "Pflichtfeld connectionType fehlt im Satz 210.".into() } else { format!("connectionType `{ct}` ist nicht in der Blatt-2-Codeliste.") },
            ))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, "configuration.attributes.connectionType"), copy("Connection type", "Anschlussart")),
                CONNECTION_TYPE_CODES.iter().map(|s| (*s).to_string()).collect(),
                copy("Set connectionType to a Blatt code-list value (e.g. flange).", "connectionType auf einen Blatt-Codelistenwert setzen (z. B. flange)."),
            ));
    }
    out.push(ct_check.build());

    // PN domain (not just presence) — VDI 3805-2 §4.2.3 / DIN EN 1092.
    if !attrs.pressure_class.is_empty() && !PRESSURE_CLASS_CODES.iter().any(|c| *c == attrs.pressure_class.as_str()) {
        out.push(
            CheckResult::assess(format!("vdi3805.2.pn.domain.{article}"), part.clone(), clause("2", "4.2.3"), subject.clone(), copy("Pressure class domain", "Druckstufen-Wertebereich"))
                .annex(ANNEX)
                .status(CheckStatus::Fail)
                .explanation(copy(format!("pressureClass `{}` is not an admitted PN code.", attrs.pressure_class), format!("pressureClass `{}` ist kein zugelassener PN-Code.", attrs.pressure_class)))
                .remedy(Remedy::one_of(
                    SubjectRef::new(article, product_path(article, "configuration.attributes.pressureClass"), copy("Pressure class", "Druckstufe")),
                    PRESSURE_CLASS_CODES.iter().map(|s| (*s).to_string()).collect(),
                    copy("Choose an admitted PN class (PN6…PN40).", "Zugelassene PN-Klasse wählen (PN6…PN40)."),
                ))
                .build(),
        );
    }

    let auth_ok = attrs.authority_min < attrs.authority_max && attrs.authority_min >= 0.0 && attrs.authority_max <= 1.0;
    let mut auth = CheckResult::assess(format!("vdi3805.2.authority.{article}"), part.clone(), clause("2", "4.2.4"), subject.clone(), copy("Valve authority range", "Ventilautoritätsbereich"))
        .annex(ANNEX)
        .status(if auth_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if auth_ok {
        auth = auth.explanation(copy(format!("Authority [{}, {}] is valid.", attrs.authority_min, attrs.authority_max), format!("Autorität [{}, {}] ist gültig.", attrs.authority_min, attrs.authority_max)));
    } else {
        auth = auth
            .explanation(copy(format!("Authority [{}, {}] is invalid.", attrs.authority_min, attrs.authority_max), format!("Autorität [{}, {}] ist ungültig.", attrs.authority_min, attrs.authority_max)))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, "configuration.attributes.authorityMax"), copy("Authority max", "Autorität max")),
                Quantity::new(QuantityKind::Dimensionless, attrs.authority_max),
                Quantity::new(QuantityKind::Dimensionless, (attrs.authority_min + 0.2).min(1.0).max(attrs.authority_min + 0.05)),
                copy("Set authorityMax above authorityMin within [0, 1].", "authorityMax oberhalb von authorityMin im Intervall [0, 1] setzen."),
            ));
    }
    out.push(auth.build());

    for curve_id in &product.configuration.function_refs {
        if let Some(curve) = document.curves.get(curve_id) {
            let mono = curve.points.windows(2).all(|w| w[1].x + 1e-12 >= w[0].x && w[1].y + 1e-12 >= w[0].y);
            let mut c = CheckResult::assess(format!("vdi3805.2.curve.{article}.{curve_id}"), part.clone(), clause("2", "5.1"), subject.clone(), copy("Characteristic curve monotonicity", "Kennlinien-Monotonie"))
                .annex(ANNEX)
                .status(if mono { CheckStatus::Pass } else { CheckStatus::Fail });
            if mono {
                c = c.explanation(copy(format!("Curve {curve_id} is non-decreasing.",), format!("Kennlinie {curve_id} ist monoton steigend.")));
            } else if let Some((idx, current_y, required_y)) = mono_increasing_remedy(curve) {
                let axis = if curve.points.get(idx).is_some_and(|p| curve.points.get(idx - 1).is_some_and(|prev| p.x + 1e-12 < prev.x)) {
                    "x"
                } else {
                    "y"
                };
                let (cur, req) = if axis == "x" {
                    let prev = curve.points[idx - 1].x;
                    (curve.points[idx].x, prev)
                } else {
                    (current_y, required_y)
                };
                c = c
                    .explanation(copy(format!("Curve {curve_id} is not monotonic.",), format!("Kennlinie {curve_id} ist nicht monoton.")))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(curve_id, curve_point_path(curve_id, idx, axis), copy("Curve point", "Kennlinienpunkt")),
                        Quantity::new(QuantityKind::Dimensionless, cur),
                        Quantity::new(QuantityKind::Dimensionless, req),
                        copy(
                            format!("Raise points[{idx}].{axis} so the curve is non-decreasing."),
                            format!("points[{idx}].{axis} anheben, damit die Kennlinie monoton steigt."),
                        ),
                    ));
            }
            out.push(c.build());
            if curve_id.contains("kvs") {
                let ymax = curve.points.iter().map(|p| p.y).fold(0.0_f64, f64::max);
                let span_ok = (ymax - attrs.kvs_m3_h()).abs() <= attrs.kvs_m3_h().max(1.0) * 0.05 + 1e-9;
                let mut ks = CheckResult::assess(format!("vdi3805.2.curve.kvsMatch.{article}.{curve_id}"), part.clone(), clause("2", "5.1"), subject.clone(), copy("kvs curve endpoint", "kvs-Kennlinienendpunkt"))
                    .annex(ANNEX)
                    .minimum(Quantity::new(QuantityKind::Volume, ymax / 3600.0), Quantity::new(QuantityKind::Volume, attrs.kvs_m3_s * 0.95));
                if span_ok {
                    ks = ks.explanation(copy(format!("Curve max y={ymax:.3} matches kvs={:.3} m³/h.", attrs.kvs_m3_h()), format!("Kennlinien-Max y={ymax:.3} entspricht kvs={:.3} m³/h.", attrs.kvs_m3_h())));
                } else {
                    ks = ks
                        .explanation(copy(format!("Curve max y={ymax:.3} disagrees with kvs={:.3} m³/h.", attrs.kvs_m3_h()), format!("Kennlinien-Max y={ymax:.3} weicht von kvs={:.3} m³/h ab.", attrs.kvs_m3_h())))
                        .remedy(Remedy::exactly(
                            SubjectRef::new(curve_id, curve_point_path(curve_id, curve.points.len().saturating_sub(1), "y"), copy("Curve end y", "Kennlinienende y")),
                            Quantity::new(QuantityKind::Dimensionless, ymax),
                            Quantity::new(QuantityKind::Dimensionless, attrs.kvs_m3_h()),
                            copy("Align the kvs curve endpoint with configuration.attributes.kvs.", "kvs-Kennlinienendpunkt an configuration.attributes.kvs angleichen."),
                        ));
                }
                out.push(ks.build());
                for (pi, pt) in curve.points.iter().enumerate() {
                    let upper = attrs.kvs_m3_h() * 1.05 + 1e-9;
                    let in_band = pt.y >= -1e-12 && pt.y <= upper && pt.x >= -1e-12;
                    let mut pb = CheckResult::assess(format!("vdi3805.2.curve.point.{article}.{curve_id}.{pi}"), part.clone(), clause("2", "5.1"), subject.clone(), copy("Curve point envelope", "Kennlinienpunkt-Hüllkurve"))
                        .annex(ANNEX)
                        .utilization(Quantity::new(QuantityKind::Dimensionless, pt.y.max(0.0)), Quantity::new(QuantityKind::Dimensionless, upper.max(1e-9)));
                    if in_band {
                        pb = pb.explanation(copy(format!("Point[{pi}]=({:.3},{:.3}) within kvs envelope.", pt.x, pt.y), format!("Punkt[{pi}]=({:.3},{:.3}) innerhalb der kvs-Hüllkurve.", pt.x, pt.y)));
                    } else {
                        pb = pb
                            .status(CheckStatus::Fail)
                            .explanation(copy(format!("Point[{pi}] y={:.3} outside [0, {:.3}].", pt.y, upper), format!("Punkt[{pi}] y={:.3} außerhalb [0, {:.3}].", pt.y, upper)))
                            .remedy(Remedy::exactly(
                                SubjectRef::new(curve_id, curve_point_path(curve_id, pi, "y"), copy("Curve point y", "Kennlinienpunkt y")),
                                Quantity::new(QuantityKind::Dimensionless, pt.y),
                                Quantity::new(QuantityKind::Dimensionless, attrs.kvs_m3_h().min(pt.y.max(0.0))),
                                copy("Bring the curve point back inside the kvs envelope.", "Kennlinienpunkt zurück in die kvs-Hüllkurve bringen."),
                            ));
                    }
                    out.push(pb.build());
                }
                let symbol_ok = curve.x_unit.symbol == "%"
                    && (curve.y_unit.symbol == "m3/h" || curve.y_unit.symbol == "m³/h");
                let delta_ok = curve.x_unit.delta && !curve.y_unit.delta;
                let factor_ok = curve.x_unit.si_factor > 0.0 && curve.y_unit.si_factor > 0.0;
                let units_ok = symbol_ok && delta_ok && factor_ok;
                let mut xu = CheckResult::assess(format!("vdi3805.2.curve.units.{article}.{curve_id}"), part.clone(), clause("2", "5.1"), subject.clone(), copy("Curve unit metadata", "Kennlinien-Einheitenmetadaten"))
                    .annex(ANNEX)
                    .minimum(
                        Quantity::new(QuantityKind::Dimensionless, if units_ok { curve.x_unit.si_factor.min(curve.y_unit.si_factor) } else { 0.0 }),
                        Quantity::new(QuantityKind::Dimensionless, 1e-12),
                    )
                    .status(if units_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if units_ok {
                    xu = xu.explanation(copy(
                        format!("Units {}/{} (delta/abs) with SI factors {}/{}.", curve.x_unit.symbol, curve.y_unit.symbol, curve.x_unit.si_factor, curve.y_unit.si_factor),
                        format!("Einheiten {}/{} (delta/abs) mit SI-Faktoren {}/{}.", curve.x_unit.symbol, curve.y_unit.symbol, curve.x_unit.si_factor, curve.y_unit.si_factor),
                    ));
                } else {
                    xu = xu
                        .explanation(copy(
                            "kvs curve units must be % (delta) over m3/h (absolute) with positive si_factor.",
                            "kvs-Kennlinieneinheiten müssen % (delta) über m3/h (absolut) mit positivem si_factor sein.",
                        ))
                        .remedy(Remedy::exactly(
                            SubjectRef::new(curve_id, format!("curves[id={curve_id}].xUnit.symbol"), copy("X unit symbol", "X-Einheitensymbol")),
                            Quantity::new(QuantityKind::Dimensionless, 0.0),
                            Quantity::new(QuantityKind::Dimensionless, 1.0),
                            copy("Set xUnit.symbol to `%` (delta) and yUnit.symbol to `m3/h` with positive si_factor.", "xUnit.symbol auf `%` (delta) und yUnit.symbol auf `m3/h` mit positivem si_factor setzen."),
                        ));
                }
                out.push(xu.build());
            }
        }
    }
    out
}

pub(super) fn check_radiator(product: &CatalogueProduct, attrs: &RadiatorAttributes) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let subject = subject_product(product);
    let part = part_label(3);
    let mut phi = CheckResult::assess(format!("vdi3805.3.phi.{article}"), part.clone(), clause("3", "4.1"), subject.clone(), copy("Standard output at 75/65/20 °C", "Normwärmeleistung bei 75/65/20 °C"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Power, attrs.standard_output_w), Quantity::new(QuantityKind::Power, 1.0));
    if attrs.standard_output_w >= 1.0 {
        phi = phi.explanation(copy(format!("Φ = {:.0} W at 75/65/20 °C.", attrs.standard_output_w), format!("Φ = {:.0} W bei 75/65/20 °C.", attrs.standard_output_w)));
    } else {
        phi = phi
            .explanation(copy("Standard output missing or non-positive.", "Normwärmeleistung fehlt oder ist nicht positiv."))
            .remedy(Remedy::at_least(
                SubjectRef::new(article, product_path(article, "configuration.attributes.standardOutputW"), copy("Standard output", "Normwärmeleistung")),
                Quantity::new(QuantityKind::Power, attrs.standard_output_w),
                Quantity::new(QuantityKind::Power, 500.0),
                copy("Set standardOutputW to the EN 442 / Blatt 3 rated output in watts.", "standardOutputW auf die EN-442-/Blatt-3-Nennleistung in Watt setzen."),
            ));
    }
    out.push(phi.build());

    let n_ok = attrs.heat_exponent_n >= RADIATOR_N_MIN && attrs.heat_exponent_n <= RADIATOR_N_MAX;
    let mut n = CheckResult::assess(format!("vdi3805.3.n.{article}"), part.clone(), clause("3", "4.2"), subject.clone(), copy("Heat exponent n", "Heizexponent n"))
        .annex(ANNEX)
        .utilization(
            Quantity::new(QuantityKind::Dimensionless, attrs.heat_exponent_n),
            Quantity::new(QuantityKind::Dimensionless, RADIATOR_N_MAX),
        )
        .status(if n_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if n_ok {
        n = n.explanation(copy(format!("n = {:.3} within [{RADIATOR_N_MIN}, {RADIATOR_N_MAX}].", attrs.heat_exponent_n), format!("n = {:.3} im Bereich [{RADIATOR_N_MIN}, {RADIATOR_N_MAX}].", attrs.heat_exponent_n)));
    } else {
        let required = attrs.heat_exponent_n.clamp(RADIATOR_N_MIN, RADIATOR_N_MAX);
        n = n
            .explanation(copy(format!("n = {:.3} outside Blatt 3 range.", attrs.heat_exponent_n), format!("n = {:.3} außerhalb des Blatt-3-Bereichs.", attrs.heat_exponent_n)))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, "configuration.attributes.heatExponentN"), copy("Heat exponent", "Heizexponent")),
                Quantity::new(QuantityKind::Dimensionless, attrs.heat_exponent_n),
                Quantity::new(QuantityKind::Dimensionless, if attrs.heat_exponent_n == 0.0 { 1.3 } else { required }),
                copy(format!("Set heatExponentN into [{RADIATOR_N_MIN}, {RADIATOR_N_MAX}] (typical 1.3)."), format!("heatExponentN in [{RADIATOR_N_MIN}, {RADIATOR_N_MAX}] setzen (typisch 1,3).")),
            ));
    }
    out.push(n.build());

    for (leaf, en, de, value, min_v) in [
        ("lengthM", "Radiator length", "Heizkörperlänge", attrs.length_m, 0.1),
        ("heightM", "Radiator height", "Heizkörperhöhe", attrs.height_m, 0.1),
        ("depthM", "Radiator depth", "Heizkörpertiefe", attrs.depth_m, 0.02),
    ] {
        let mut dim = CheckResult::assess(
            format!("vdi3805.3.{leaf}.{article}"),
            part_label(3),
            clause("3", "4.3"),
            subject.clone(),
            copy(en, de),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Length, value), Quantity::new(QuantityKind::Length, min_v));
        if value + 1e-12 >= min_v {
            dim = dim.explanation(copy(format!("{en} = {value:.3} m."), format!("{de} = {value:.3} m.")));
        } else {
            dim = dim
                .explanation(copy(format!("{en} missing or below {min_v} m."), format!("{de} fehlt oder unter {min_v} m.")))
                .remedy(Remedy::at_least(
                    SubjectRef::new(article, attr_leaf_path(article, leaf), copy(en, de)),
                    Quantity::new(QuantityKind::Length, value),
                    Quantity::new(QuantityKind::Length, min_v),
                    copy(format!("Set {leaf} to at least {min_v} m."), format!("{leaf} auf mindestens {min_v} m setzen.")),
                ));
        }
        out.push(dim.build());
    }
    let ct = attrs.connection_type.as_str();
    let ct_ord = CONNECTION_TYPE_CODES.iter().position(|c| *c == ct).map(|i| (i + 1) as f64).unwrap_or(0.0);
    let ct_ok = ct_ord > 0.0;
    let mut ct_check = CheckResult::assess(format!("vdi3805.3.connectionType.{article}"), part_label(3), clause("3", "4.4"), subject.clone(), copy("Connection type", "Anschlussart"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Dimensionless, ct_ord), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .status(if ct_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if ct_ok {
        ct_check = ct_check.explanation(copy(format!("connectionType `{ct}` admitted."), format!("connectionType `{ct}` zugelassen.")));
    } else {
        ct_check = ct_check
            .explanation(copy("connectionType missing or not in Blatt code list.", "connectionType fehlt oder nicht in der Blatt-Codeliste."))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, "configuration.attributes.connectionType"), copy("Connection type", "Anschlussart")),
                CONNECTION_TYPE_CODES.iter().map(|s| (*s).to_string()).collect(),
                copy("Set connectionType to a Blatt code-list value.", "connectionType auf einen Blatt-Codelistenwert setzen."),
            ));
    }
    out.push(ct_check.build());
    out
}

pub(super) fn check_pump(document: &Vdi3805Snapshot, product: &CatalogueProduct, attrs: &PumpHeatingAttributes) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let subject = subject_product(product);
    let part = part_label(5);
    let mut flow = CheckResult::assess(format!("vdi3805.5.q.{article}"), part.clone(), clause("5", "4.1"), subject.clone(), copy("Nominal volume flow", "Nennvolumenstrom"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Volume, attrs.nominal_flow_m3_s), Quantity::new(QuantityKind::Volume, 1e-6));
    if attrs.nominal_flow_m3_s >= 1e-6 {
        flow = flow.explanation(copy(format!("Q = {:.4} m³/s.", attrs.nominal_flow_m3_s), format!("Q = {:.4} m³/s.", attrs.nominal_flow_m3_s)));
    } else {
        flow = flow
            .explanation(copy("Nominal flow missing.", "Nennvolumenstrom fehlt."))
            .remedy(Remedy::at_least(
                SubjectRef::new(article, product_path(article, "configuration.attributes.nominalFlowM3S"), copy("Nominal flow", "Nennvolumenstrom")),
                Quantity::new(QuantityKind::Volume, attrs.nominal_flow_m3_s),
                Quantity::new(QuantityKind::Volume, 0.0025),
                copy("Set nominalFlowM3S to the rated pump flow in m³/s.", "nominalFlowM3S auf den Nennvolumenstrom in m³/s setzen."),
            ));
    }
    out.push(flow.build());

    let eta_ok = attrs.hydraulic_efficiency > 0.0 && attrs.hydraulic_efficiency <= 1.0;
    let mut eta = CheckResult::assess(format!("vdi3805.5.eta.{article}"), part.clone(), clause("5", "4.2"), subject.clone(), copy("Hydraulic efficiency", "Hydraulischer Wirkungsgrad"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Dimensionless, attrs.hydraulic_efficiency), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .status(if eta_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if eta_ok {
        eta = eta.explanation(copy(format!("η = {:.3}.", attrs.hydraulic_efficiency), format!("η = {:.3}.", attrs.hydraulic_efficiency)));
    } else {
        eta = eta
            .explanation(copy("Hydraulic efficiency must be in (0, 1].", "Hydraulischer Wirkungsgrad muss in (0, 1] liegen."))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, "configuration.attributes.hydraulicEfficiency"), copy("Efficiency", "Wirkungsgrad")),
                Quantity::new(QuantityKind::Dimensionless, attrs.hydraulic_efficiency),
                Quantity::new(QuantityKind::Dimensionless, 0.45),
                copy("Set hydraulicEfficiency to a value in (0, 1], e.g. 0.45.", "hydraulicEfficiency auf einen Wert in (0, 1] setzen, z. B. 0,45."),
            ));
    }
    out.push(eta.build());

    for (leaf, en, de, value, min_v, kind) in [
        ("dnSuction", "Suction DN", "Saugseitige DN", attrs.dn_suction as f64, 10.0, QuantityKind::Dimensionless),
        ("dnDischarge", "Discharge DN", "Druckseitige DN", attrs.dn_discharge as f64, 10.0, QuantityKind::Dimensionless),
        ("nominalHeadM", "Nominal head", "Nennförderhöhe", attrs.nominal_head_m, 0.5, QuantityKind::Length),
        ("motorPowerW", "Motor power", "Motorleistung", attrs.motor_power_w, 10.0, QuantityKind::Power),
    ] {
        let mut c = CheckResult::assess(
            format!("vdi3805.5.{leaf}.{article}"),
            part.clone(),
            clause("5", "4.3"),
            subject.clone(),
            copy(en, de),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(kind, value), Quantity::new(kind, min_v));
        if value + 1e-12 >= min_v {
            c = c.explanation(copy(format!("{en} = {value}."), format!("{de} = {value}.")));
        } else {
            c = c
                .explanation(copy(format!("{en} must be ≥ {min_v}."), format!("{de} muss ≥ {min_v} sein.")))
                .remedy(Remedy::at_least(
                    SubjectRef::new(article, attr_leaf_path(article, leaf), copy(en, de)),
                    Quantity::new(kind, value),
                    Quantity::new(kind, min_v),
                    copy(format!("Set {leaf} to at least {min_v}."), format!("{leaf} auf mindestens {min_v} setzen.")),
                ));
        }
        out.push(c.build());
    }

    let qh_ref = attrs.qh_curve_ref.clone().or_else(|| product.configuration.function_refs.first().cloned());
    let mut qhref = CheckResult::assess(format!("vdi3805.5.qhCurveRef.{article}"), part.clone(), clause("5", "5.1"), subject.clone(), copy("Q-H curve reference", "Q-H-Kennlinienreferenz"))
        .annex(ANNEX);
    match qh_ref.as_deref() {
        Some(curve_id) if document.curves.contains_key(curve_id) => {
            qhref = qhref
                .minimum(Quantity::new(QuantityKind::Dimensionless, 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .status(CheckStatus::Pass)
                .explanation(copy(format!("qhCurveRef `{curve_id}` resolves."), format!("qhCurveRef `{curve_id}` ist auflösbar.")));
        }
        Some(curve_id) => {
            qhref = qhref
                .minimum(Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .status(CheckStatus::Fail)
                .explanation(copy(format!("qhCurveRef `{curve_id}` is dangling."), format!("qhCurveRef `{curve_id}` ist hängend.")))
                .remedy(Remedy::one_of(
                    SubjectRef::new(article, product_path(article, "configuration.attributes.qhCurveRef"), copy("Q-H curve reference", "Q-H-Kennlinienreferenz")),
                    document.curves.keys().cloned().collect(),
                    copy("Point qhCurveRef at an existing curve id.", "qhCurveRef auf eine vorhandene Kennlinien-ID setzen."),
                ));
        }
        None => {
            qhref = qhref
                .minimum(Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .status(CheckStatus::Fail)
                .explanation(copy("qhCurveRef omitted.", "qhCurveRef fehlt."))
                .remedy(Remedy::one_of(
                    SubjectRef::new(article, product_path(article, "configuration.attributes.qhCurveRef"), copy("Q-H curve reference", "Q-H-Kennlinienreferenz")),
                    document.curves.keys().cloned().collect(),
                    copy("Set qhCurveRef to an existing curve id.", "qhCurveRef auf eine vorhandene Kennlinien-ID setzen."),
                ));
        }
    }
    out.push(qhref.build());
    if let Some(curve_id) = qh_ref.as_ref() {
        if let Some(curve) = document.curves.get(curve_id) {
            let mono = curve.points.windows(2).all(|w| w[1].x >= w[0].x && w[1].y <= w[0].y);
            let rising = curve.points.windows(2).filter(|w| w[1].y > w[0].y + 1e-12).count() as f64;
            let mut c = CheckResult::assess(format!("vdi3805.5.qh.{article}"), part.clone(), clause("5", "5.1"), subject.clone(), copy("Q-H curve shape", "Q-H-Kennlinie"))
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Dimensionless, if mono { 0.0 } else { rising }), Quantity::new(QuantityKind::Dimensionless, 0.0))
                .status(if mono { CheckStatus::Pass } else { CheckStatus::Fail });
            if mono {
                c = c.explanation(copy("Q-H head is non-increasing with flow.", "Förderhöhe fällt mit dem Volumenstrom nicht an."));
            } else {
                c = c.explanation(copy("Q-H curve must not rise with flow.", "Q-H-Kennlinie darf mit dem Volumenstrom nicht steigen."));
                if let Some((idx, current_y, required_y)) = mono_decreasing_y_remedy(curve) {
                    c = c.remedy(Remedy::exactly(
                        SubjectRef::new(curve_id, curve_point_path(curve_id, idx, "y"), copy("Q-H point y", "Q-H-Punkt y")),
                        Quantity::new(QuantityKind::Dimensionless, current_y),
                        Quantity::new(QuantityKind::Dimensionless, required_y),
                        copy("Lower the rising head point so Q-H is non-increasing.", "Ansteigenden Förderhöhenpunkt absenken, damit Q-H nicht steigt."),
                    ));
                } else if let Some(pt) = curve.points.first() {
                    c = c.remedy(Remedy::exactly(
                        SubjectRef::new(curve_id, curve_point_path(curve_id, 0, "y"), copy("Q-H point y", "Q-H-Punkt y")),
                        Quantity::new(QuantityKind::Dimensionless, pt.y),
                        Quantity::new(QuantityKind::Dimensionless, pt.y),
                        copy("Resample the Q-H curve so head is non-increasing with flow.", "Q-H-Kennlinie so neu abtasten, dass die Förderhöhe mit dem Volumenstrom nicht steigt."),
                    ));
                }
            }
            out.push(c.build());

            let x_symbol_ok = curve.x_unit.symbol == "%" || curve.x_unit.symbol == "m3/h" || curve.x_unit.symbol == "m³/h";
            let y_symbol_ok = curve.y_unit.symbol == "m";
            let delta_ok = curve.x_unit.delta == (curve.x_unit.symbol == "%") && !curve.y_unit.delta;
            let factor_ok = curve.x_unit.si_factor > 0.0 && curve.y_unit.si_factor > 0.0;
            let units_ok = x_symbol_ok && y_symbol_ok && delta_ok && factor_ok;
            let mut u = CheckResult::assess(
                format!("vdi3805.5.qh.units.{article}.{curve_id}"),
                part.clone(),
                clause("5", "5.1"),
                subject.clone(),
                copy("Q-H curve units", "Q-H-Kennlinieneinheiten"),
            )
            .annex(ANNEX)
            .minimum(
                Quantity::new(QuantityKind::Dimensionless, if units_ok { curve.x_unit.si_factor.min(curve.y_unit.si_factor) } else { 0.0 }),
                Quantity::new(QuantityKind::Dimensionless, 1e-12),
            )
            .status(if units_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if units_ok {
                u = u.explanation(copy(
                    format!("Q-H units {}/{} (delta={} / abs) with SI factors {}/{}.", curve.x_unit.symbol, curve.y_unit.symbol, curve.x_unit.delta, curve.x_unit.si_factor, curve.y_unit.si_factor),
                    format!("Q-H-Einheiten {}/{} (delta={} / abs) mit SI-Faktoren {}/{}.", curve.x_unit.symbol, curve.y_unit.symbol, curve.x_unit.delta, curve.x_unit.si_factor, curve.y_unit.si_factor),
                ));
            } else {
                u = u
                    .explanation(copy(
                        "Q-H curve units must be % (delta) or m3/h over head in m (absolute) with positive si_factor.",
                        "Q-H-Kennlinieneinheiten müssen % (delta) oder m3/h über Förderhöhe in m (absolut) mit positivem si_factor sein.",
                    ))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(curve_id, format!("curves[id={curve_id}].yUnit.symbol"), copy("Y unit symbol", "Y-Einheitensymbol")),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy("Set yUnit.symbol to `m` (absolute) and xUnit to `%` (delta) or `m3/h`.", "yUnit.symbol auf `m` (absolut) und xUnit auf `%` (delta) oder `m3/h` setzen."),
                    ));
            }
            out.push(u.build());
            for (axis, pt) in curve.points.iter().enumerate() {
                let mut pb = CheckResult::assess(
                    format!("vdi3805.5.qh.point.{article}.{curve_id}.{axis}"),
                    part.clone(),
                    clause("5", "5.1"),
                    subject.clone(),
                    copy("Q-H curve point", "Q-H-Kennlinienpunkt"),
                )
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Dimensionless, pt.x), Quantity::new(QuantityKind::Dimensionless, 0.0))
                .utilization(
                    Quantity::new(QuantityKind::Dimensionless, pt.y.max(0.0)),
                    Quantity::new(QuantityKind::Dimensionless, curve.points.first().map(|p| p.y.max(1e-9)).unwrap_or(1.0)),
                )
                .status(if pt.x >= -1e-12 && pt.y >= -1e-12 { CheckStatus::Pass } else { CheckStatus::Fail });
                if pt.x < -1e-12 || pt.y < -1e-12 {
                    pb = pb
                        .explanation(copy("Q-H points must have non-negative flow and head.", "Q-H-Punkte müssen nicht-negativen Volumenstrom und Förderhöhe haben."))
                        .remedy(Remedy::at_least(
                            SubjectRef::new(curve_id, curve_point_path(curve_id, axis, "y"), copy("Q-H point y", "Q-H-Punkt y")),
                            Quantity::new(QuantityKind::Dimensionless, pt.y),
                            Quantity::new(QuantityKind::Dimensionless, 0.0),
                            copy("Set Q-H point coordinates to non-negative values.", "Q-H-Punktkoordinaten auf nicht-negative Werte setzen."),
                        ));
                } else {
                    pb = pb.explanation(copy(
                        format!("Q-H point {axis} at Q={:.4}, H={:.4}.", pt.x, pt.y),
                        format!("Q-H-Punkt {axis} bei Q={:.4}, H={:.4}.", pt.x, pt.y),
                    ));
                }
                out.push(pb.build());
            }
        }
    }
    out
}
