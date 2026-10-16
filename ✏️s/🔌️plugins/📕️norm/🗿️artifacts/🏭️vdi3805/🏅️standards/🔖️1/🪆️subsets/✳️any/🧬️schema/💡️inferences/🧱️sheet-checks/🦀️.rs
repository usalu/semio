//! 🧱️ VDI 3805 sheet checks: attribute and record synchronisation, the part 1 structure check and the per-sheet product checks for valves, radiators, pumps and heat generators.

use super::*;

//#region 🔖️ComplianceReport
fn check_attributes_records_sync(product: &CatalogueProduct) -> Vec<CheckResult> {
    let article = product.id.as_str();
    let derived = attributes_from_records(product.sheet, &product.records);
    let configured = &product.configuration.attributes;
    let ok = match (configured, &derived) {
        (SheetAttributes::ValveHeating(a), SheetAttributes::ValveHeating(b)) => {
            a.dn == b.dn
                && (a.kvs_m3_s - b.kvs_m3_s).abs() < 1e-12
                && a.pressure_class == b.pressure_class
                && a.connection_type == b.connection_type
                && (a.authority_min - b.authority_min).abs() < 1e-12
                && (a.authority_max - b.authority_max).abs() < 1e-12
        }
        (SheetAttributes::Radiator(a), SheetAttributes::Radiator(b)) => {
            (a.standard_output_w - b.standard_output_w).abs() < 1e-9
                && (a.heat_exponent_n - b.heat_exponent_n).abs() < 1e-9
                && (a.length_m - b.length_m).abs() < 1e-9
                && (a.height_m - b.height_m).abs() < 1e-9
                && (a.depth_m - b.depth_m).abs() < 1e-9
                && a.connection_type == b.connection_type
        }
        (SheetAttributes::PumpHeating(a), SheetAttributes::PumpHeating(b)) => {
            a.dn_suction == b.dn_suction
                && a.dn_discharge == b.dn_discharge
                && (a.nominal_flow_m3_s - b.nominal_flow_m3_s).abs() < 1e-12
                && (a.nominal_head_m - b.nominal_head_m).abs() < 1e-9
                && (a.motor_power_w - b.motor_power_w).abs() < 1e-6
                && (a.hydraulic_efficiency - b.hydraulic_efficiency).abs() < 1e-12
                && a.qh_curve_ref == b.qh_curve_ref
        }
        (SheetAttributes::HeatGenerator(a), SheetAttributes::HeatGenerator(b)) => {
            (a.nominal_heat_output_w - b.nominal_heat_output_w).abs() < 1e-9
                && a.fuel_type == b.fuel_type
                && (a.flow_temp_max_c - b.flow_temp_max_c).abs() < 1e-9
                && (a.return_temp_min_c - b.return_temp_min_c).abs() < 1e-9
        }
        (SheetAttributes::Generic(a), SheetAttributes::Generic(b)) => {
            a.entries.iter().all(|e| b.entries.iter().any(|f| f.key.eq_ignore_ascii_case(&e.key) && f.value == e.value))
        }
        _ => false,
    };
    let typed_sheet = matches!(product.sheet.0, 2 | 3 | 5 | 6);
    let configured_typed = matches!(
        configured,
        SheetAttributes::ValveHeating(_) | SheetAttributes::Radiator(_) | SheetAttributes::PumpHeating(_) | SheetAttributes::HeatGenerator(_)
    );
    let derived_typed = matches!(
        derived,
        SheetAttributes::ValveHeating(_) | SheetAttributes::Radiator(_) | SheetAttributes::PumpHeating(_) | SheetAttributes::HeatGenerator(_)
    );
    let mut out = Vec::new();
    if typed_sheet || configured_typed || derived_typed || matches!(configured, SheetAttributes::Generic(_)) {
        let mut b = CheckResult::assess(
            format!("vdi3805.{}.sync.{}", product.sheet.0, article),
            part_label(product.sheet.0),
            clause(&product.sheet.0.to_string(), "4.0"),
            subject_product(product),
            copy("Attributes ↔ records sync", "Attribute ↔ Datensätze Synchronität"),
        )
        .annex(ANNEX);
        if ok && !(typed_sheet && !configured_typed && derived_typed) {
            b = b
                .status(CheckStatus::Pass)
                .explanation(copy(
                    "Typed attributes match native 210 records.",
                    "Typisierte Attribute stimmen mit nativen 210-Sätzen überein.",
                ));
        } else {
            b = b
                .status(CheckStatus::Fail)
                .explanation(copy(
                    "configuration.attributes diverge from native records — align the typed attribute leaf; sync regenerates the 210 record.",
                    "configuration.attributes weichen von nativen Sätzen ab — typisiertes Attributblatt angleichen; Sync erzeugt den 210-Satz neu.",
                ))
                .remedy(sync_divergence_remedy(article, configured, &derived));
        }
        out.push(b.build());
    }
    out
}

pub(super) fn check_part1(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let issues = validate_structure(document);
    let errors: Vec<_> = issues.iter().filter(|d| d.severity == Severity::Error).collect();
    let subject = subject_dataset();
    if errors.is_empty() {
        out.push(
            CheckResult::assess("vdi3805.1.structure", part_label(1), clause("1", "4.1"), subject.clone(), copy("Part 1 record structure", "Teil 1 Datensatzstruktur"))
                .status(CheckStatus::Pass)
                .annex(ANNEX)
                .explanation(copy("Header, mandatory records, field arities and referential integrity are valid.", "Kopf, Pflichtsätze, Feldanzahlen und Referenzintegrität sind gültig."))
                .build(),
        );
    } else {
        for err in errors.iter() {
            let path = err.field.clone();
            let required_count = if path.contains("recordCount") {
                document.catalog.products.iter().map(|p| p.records.len() as f64).sum::<f64>()
            } else {
                1.0
            };
            let current = if path.contains("recordCount") {
                document.catalog.file.record_count as f64
            } else {
                0.0
            };
            let target = SubjectRef::new("", path.clone(), copy(format!("Structural field `{path}`"), format!("Strukturfeld `{path}`")));
            let slug = path.replace('.', "-").replace('/', "-").replace('[', "-").replace(']', "");
            let mut builder = CheckResult::assess(
                format!("vdi3805.1.structure.{slug}"),
                part_label(1),
                clause("1", "4.1"),
                subject.clone(),
                copy("Part 1 structural rule", "Teil-1-Strukturregel"),
            )
            .status(CheckStatus::Fail)
            .annex(ANNEX)
            .explanation(copy(format!("Part 1 structural error at {}: {}", err.field, err.message), format!("Teil-1-Strukturfehler bei {}: {}", err.field, err.message)));
            if path.contains("recordCount") {
                builder = builder.remedy(Remedy::exactly(
                    target,
                    Quantity::new(QuantityKind::Dimensionless, current),
                    Quantity::new(QuantityKind::Dimensionless, required_count),
                    copy(
                        format!("Set catalog.file.recordCount from {current} to {required_count}."),
                        format!("catalog.file.recordCount von {current} auf {required_count} setzen."),
                    ),
                ));
            } else if path.contains("geometryRef") {
                let article = document.catalog.products.first().map(|p| p.id.as_str()).unwrap_or("");
                let geom_target = SubjectRef::new(article, product_path(article, "configuration.geometryRef"), copy("Geometry reference", "Geometriereferenz"));
                builder = builder.remedy(Remedy::one_of(
                    geom_target,
                    document.geometry.keys().cloned().collect(),
                    copy(
                        "Point geometryRef at an existing geometry id or add the missing geometry block.",
                        "geometryRef auf eine vorhandene Geometrie-ID setzen oder den fehlenden Geometrieblock ergänzen.",
                    ),
                ));
            } else if path.contains("functionRefs") {
                let article = document.catalog.products.first().map(|p| p.id.as_str()).unwrap_or("");
                builder = builder.remedy(Remedy::one_of(
                    SubjectRef::new(article, product_path(article, "configuration.functionRefs"), copy("Curve references", "Kennlinienreferenzen")),
                    document.curves.keys().cloned().collect(),
                    copy("Replace dangling curve refs with existing curve ids.", "Hängende Kennlinienreferenzen durch vorhandene IDs ersetzen."),
                ));
            } else if path.contains("accessoryId") {
                builder = builder.remedy(Remedy::one_of(
                    target,
                    document.catalog.products.iter().map(|p| p.id.clone()).collect(),
                    copy(
                        "Point accessories[].accessoryId at an existing catalogue product id.",
                        "accessories[].accessoryId auf eine vorhandene Katalog-Produkt-ID setzen.",
                    ),
                ));
            } else if path.contains("componentId") {
                builder = builder.remedy(Remedy::one_of(
                    target,
                    document.catalog.products.iter().map(|p| p.id.clone()).collect(),
                    copy(
                        "Point components[].componentId at an existing catalogue product id.",
                        "components[].componentId auf eine vorhandene Katalog-Produkt-ID setzen.",
                    ),
                ));
            } else {
                builder = builder.remedy(Remedy::exactly(
                    target,
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    copy(format!("Fix Part 1 field `{}`: {}.", err.field, err.message), format!("Teil-1-Feld `{}` korrigieren: {}.", err.field, err.message)),
                ));
            }
            out.push(builder.build());
        }
    }
    out
}

fn check_valve(document: &Vdi3805Snapshot, product: &CatalogueProduct, attrs: &ValveHeatingAttributes) -> Vec<CheckResult> {
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

fn check_radiator(product: &CatalogueProduct, attrs: &RadiatorAttributes) -> Vec<CheckResult> {
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

fn check_pump(document: &Vdi3805Snapshot, product: &CatalogueProduct, attrs: &PumpHeatingAttributes) -> Vec<CheckResult> {
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

fn check_heat_generator(product: &CatalogueProduct, attrs: &HeatGeneratorAttributes) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let subject = subject_product(product);
    let part = part_label(6);
    let mut qn = CheckResult::assess(format!("vdi3805.6.qn.{article}"), part.clone(), clause("6", "4.1"), subject.clone(), copy("Nominal heat output", "Nennwärmeleistung"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Power, attrs.nominal_heat_output_w), Quantity::new(QuantityKind::Power, 1.0));
    if attrs.nominal_heat_output_w >= 1.0 {
        qn = qn.explanation(copy(format!("Qn = {:.0} W.", attrs.nominal_heat_output_w), format!("Qn = {:.0} W.", attrs.nominal_heat_output_w)));
    } else {
        qn = qn
            .explanation(copy("Nominal heat output missing.", "Nennwärmeleistung fehlt."))
            .remedy(Remedy::at_least(
                SubjectRef::new(article, product_path(article, "configuration.attributes.nominalHeatOutputW"), copy("Nominal heat output", "Nennwärmeleistung")),
                Quantity::new(QuantityKind::Power, attrs.nominal_heat_output_w),
                Quantity::new(QuantityKind::Power, 10_000.0),
                copy("Set nominalHeatOutputW to the rated generator output in watts.", "nominalHeatOutputW auf die Nennleistung in Watt setzen."),
            ));
    }
    out.push(qn.build());

    let fuel_ok = !attrs.fuel_type.is_empty();
    let mut fuel = CheckResult::assess(format!("vdi3805.6.fuel.{article}"), part.clone(), clause("6", "4.2"), subject.clone(), copy("Fuel / energy type", "Brennstoff-/Energieträger"))
        .annex(ANNEX)
        .status(if fuel_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if fuel_ok {
        fuel = fuel.explanation(copy(format!("Fuel type {} present.", attrs.fuel_type), format!("Energieträger {} vorhanden.", attrs.fuel_type)));
    } else {
        fuel = fuel
            .explanation(copy("Fuel type is empty.", "Energieträger fehlt."))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, "configuration.attributes.fuelType"), copy("Fuel type", "Energieträger")),
                vec!["gas".into(), "oil".into(), "electric".into(), "biomass".into(), "district".into()],
                copy("Set fuelType to a Blatt 6 energy carrier code.", "fuelType auf einen Blatt-6-Energieträger setzen."),
            ));
    }
    out.push(fuel.build());

    let flow_ok = attrs.flow_temp_max_c >= 30.0 && attrs.flow_temp_max_c <= 110.0;
    let mut flow = CheckResult::assess(
        format!("vdi3805.6.flowTempMaxC.{article}"),
        part.clone(),
        clause("6", "4.3"),
        subject.clone(),
        copy("Maximum flow temperature", "Maximale Vorlauftemperatur"),
    )
    .annex(ANNEX)
    .status(if flow_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if flow_ok {
        flow = flow.explanation(copy(format!("t_flow,max = {:.1} °C.", attrs.flow_temp_max_c), format!("t_VL,max = {:.1} °C.", attrs.flow_temp_max_c)));
    } else {
        flow = flow
            .explanation(copy("flowTempMaxC must lie in [30, 110] °C.", "flowTempMaxC muss in [30, 110] °C liegen."))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, attr_leaf_path(article, "flowTempMaxC"), copy("Flow temperature max", "Vorlauftemperatur max")),
                Quantity::new(QuantityKind::Temperature, attrs.flow_temp_max_c),
                Quantity::new(QuantityKind::Temperature, 80.0),
                copy("Set flowTempMaxC to a Blatt 6 design flow temperature (e.g. 80 °C).", "flowTempMaxC auf eine Blatt-6-Auslegungsvorlauftemperatur setzen (z. B. 80 °C)."),
            ));
    }
    out.push(flow.build());

    let ret_ok = attrs.return_temp_min_c >= 5.0 && attrs.return_temp_min_c < attrs.flow_temp_max_c.max(30.0);
    let mut ret = CheckResult::assess(
        format!("vdi3805.6.returnTempMinC.{article}"),
        part,
        clause("6", "4.4"),
        subject,
        copy("Minimum return temperature", "Minimale Rücklauftemperatur"),
    )
    .annex(ANNEX)
    .status(if ret_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if ret_ok {
        ret = ret.explanation(copy(format!("t_return,min = {:.1} °C.", attrs.return_temp_min_c), format!("t_RL,min = {:.1} °C.", attrs.return_temp_min_c)));
    } else {
        let required = (attrs.flow_temp_max_c - 20.0).clamp(20.0, 70.0);
        ret = ret
            .explanation(copy("returnTempMinC must be below flowTempMaxC and ≥ 5 °C.", "returnTempMinC muss unter flowTempMaxC und ≥ 5 °C liegen."))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, attr_leaf_path(article, "returnTempMinC"), copy("Return temperature min", "Rücklauftemperatur min")),
                Quantity::new(QuantityKind::Temperature, attrs.return_temp_min_c),
                Quantity::new(QuantityKind::Temperature, required),
                copy(format!("Set returnTempMinC to about {required:.0} °C."), format!("returnTempMinC auf etwa {required:.0} °C setzen.")),
            ));
    }
    out.push(ret.build());
    out
}

fn check_operative_sheet(document: &Vdi3805Snapshot, product: &CatalogueProduct) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let sheet = product.sheet.0;
    let entry = SHEET_ENTRIES.get(sheet.saturating_sub(1) as usize);
    let status = entry.map(|e| e.status).unwrap_or(SchemaStatus::Reserved);
    if status == SchemaStatus::Reserved {
        return vec![CheckResult::assess(
            format!("vdi3805.{sheet}.reserved.{article}"),
            part_label(sheet),
            clause(&sheet.to_string(), "scope"),
            subject_product(product),
            copy("Reserved sheet", "Reserviertes Blatt"),
        )
        .annex(ANNEX)
        .not_applicable(copy("Sheet is reserved in the registry.", "Blatt ist im Register reserviert."))
        .build()];
    }
    if status == SchemaStatus::HistoricalProposal {
        return Vec::new();
    }
    let profile = profile_for_sheet(document, sheet);
    let keys = sheet_mandatory_keys(sheet, profile);
    let kv = record_kv(&product.records);
    let generic_entries = match &product.configuration.attributes {
        SheetAttributes::Generic(g) => g.entries.clone(),
        _ => GenericAttributes::from_map(&kv).entries,
    };
    let mut missing = Vec::new();
    for key in keys {
        let present = match *key {
            "product_group" => !product.identity.product_group.is_empty() || kv.contains_key("product_group") || generic_entries.iter().any(|e| e.key.eq_ignore_ascii_case("product_group") && !e.value.is_empty()),
            other => {
                kv.get(other).is_some_and(|v| !v.is_empty())
                    || generic_entries.iter().any(|e| e.key.eq_ignore_ascii_case(other) && !e.value.is_empty())
            }
        };
        if !present {
            missing.push(*key);
        }
    }
    let title_ok = !product.title.is_empty() && product.title.iter().any(|t| t.locale == "en" || t.locale == "de");
    let identity_ok = !product.identity.article_number.is_empty() && product.id == product.identity.article_number;
    let mut b = CheckResult::assess(
        format!("vdi3805.{sheet}.mandatory.{article}"),
        part_label(sheet),
        clause(&sheet.to_string(), "4.1"),
        subject_product(product),
        copy("Mandatory Blatt attributes", "Pflichtattribute des Blatts"),
    )
    .annex(ANNEX);
    if identity_ok && title_ok && missing.is_empty() {
        b = b.status(CheckStatus::Pass).explanation(copy(
            format!("Sheet {sheet} ({profile:?}) mandatory keys present in native records."),
            format!("Blatt {sheet} ({profile:?}): Pflichtattribute in nativen Sätzen vorhanden."),
        ));
    } else {
        let first = missing.first().copied().unwrap_or("product_group");
        let path = if first == "product_group" {
            product_path(article, "identity.productGroup")
        } else {
            generic_entry_value_path(article, &generic_entries, first)
        };
        let required = match first {
            "dn" | "dn_suction" | "dn_discharge" => 50.0,
            "airflow_m3_s" => 0.1,
            "cop" => 3.0,
            "volume_m3" => 0.1,
            "axial_force_n" => 1000.0,
            "outer_diameter_m" => 0.05,
            "wall_thickness_m" => 0.002,
            "pressure_drop_pa" => 50.0,
            "nominal_heat_output_w" => 5000.0,
            _ => 1.0,
        };
        b = b
            .status(CheckStatus::Fail)
            .explanation(copy(
                format!("Sheet {sheet} ({profile:?}) missing mandatory keys {missing:?}; identity_ok={identity_ok}; title_ok={title_ok}."),
                format!("Blatt {sheet} ({profile:?}) fehlt Pflichtfelder {missing:?}; identity_ok={identity_ok}; title_ok={title_ok}."),
            ))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, path, copy("Mandatory attribute", "Pflichtattribut")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, required),
                copy(
                    format!("Provide Blatt {sheet} mandatory attribute `{first}` for {profile:?}; sync regenerates the 210 record."),
                    format!("Blatt-{sheet}-Pflichtattribut `{first}` für {profile:?} setzen; Sync erzeugt den 210-Satz neu."),
                ),
            ));
    }
    out.push(b.build());

    // Every generic attribute entry must carry a non-empty key; numeric Blatt keys must parse.
    for (ei, entry) in generic_entries.iter().enumerate() {
        let key_ok = !entry.key.is_empty();
        let mut kc = CheckResult::assess(
            format!("vdi3805.{sheet}.generic.key.{article}.{ei}"),
            part_label(sheet),
            clause(&sheet.to_string(), "4.1"),
            subject_product(product),
            copy("Generic attribute key", "Generisches Attribut-Schlüssel"),
        )
        .annex(ANNEX)
        .status(if key_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if !key_ok {
            kc = kc.remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, &format!("configuration.attributes.entries[{ei}].key")), copy("Attribute key", "Attributschlüssel")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                copy("Set a non-empty generic attribute key.", "Nicht-leeren generischen Attributschlüssel setzen."),
            ));
        }
        out.push(kc.build());
        if sheet_numeric_bounds(sheet).iter().any(|(k, _, _)| *k == entry.key) || keys.iter().any(|k| *k == entry.key) {
            let (value_ok, value_metric) = if let Some(codes) = code_list_for_key(sheet, &entry.key) {
                let ok = codes.iter().any(|c| *c == entry.value.as_str());
                (ok, if ok { 1.0 } else { 0.0 })
            } else if let Ok(v) = entry.value.parse::<f64>() {
                (true, v.abs().max(1e-12))
            } else {
                (!entry.value.is_empty(), if entry.value.is_empty() { 0.0 } else { 1.0 })
            };
            let mut vc = CheckResult::assess(
                format!("vdi3805.{sheet}.generic.value.{article}.{ei}"),
                part_label(sheet),
                clause(&sheet.to_string(), "4.1"),
                subject_product(product),
                copy("Generic attribute value", "Generisches Attribut-Wert"),
            )
            .annex(ANNEX)
            .minimum(Quantity::new(QuantityKind::Dimensionless, value_metric), Quantity::new(QuantityKind::Dimensionless, 1e-12))
            .status(if value_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if value_ok {
                vc = vc.explanation(copy(
                    format!("entries[{ei}].value `{}` is admissible for key `{}`.", entry.value, entry.key),
                    format!("entries[{ei}].value `{}` ist für Schlüssel `{}` zulässig.", entry.value, entry.key),
                ));
            } else {
                vc = vc
                    .explanation(copy(
                        format!("entries[{ei}].value `{}` is not admissible for key `{}`.", entry.value, entry.key),
                        format!("entries[{ei}].value `{}` ist für Schlüssel `{}` unzulässig.", entry.value, entry.key),
                    ))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(article, product_path(article, &format!("configuration.attributes.entries[{ei}].value")), copy("Attribute value", "Attributwert")),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy("Set a numeric or code-list value for this Blatt attribute.", "Numerischen oder Codelisten-Wert für dieses Blatt-Attribut setzen."),
                    ));
            }
            out.push(vc.build());
        }
    }

    // Domain validation for mandatory enum keys (not just presence) — Blatt code lists.
    for key in keys {
        let Some(codes) = code_list_for_key(sheet, key) else { continue };
        let raw = kv.get(*key).cloned().or_else(|| generic_entries.iter().find(|e| e.key.eq_ignore_ascii_case(key)).map(|e| e.value.clone())).unwrap_or_default();
        if raw.is_empty() {
            continue; // presence Fail already emitted
        }
        let ord = codes.iter().position(|c| *c == raw.as_str()).map(|i| (i + 1) as f64).unwrap_or(0.0);
        let ok = ord > 0.0;
        let path = generic_entry_value_path(article, &generic_entries, key);
        let mut c = CheckResult::assess(
            format!("vdi3805.{sheet}.domain.{key}.{article}"),
            part_label(sheet),
            clause(&sheet.to_string(), "4.1"),
            subject_product(product),
            copy(format!("Blatt {sheet} `{key}` domain"), format!("Blatt-{sheet}-`{key}`-Wertebereich")),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Dimensionless, ord), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if ok {
            c = c.explanation(copy(format!("`{key}` = `{raw}` is in the Blatt code list."), format!("`{key}` = `{raw}` ist in der Blatt-Codeliste.")));
        } else {
            c = c
                .explanation(copy(format!("`{key}` = `{raw}` is not an admitted code."), format!("`{key}` = `{raw}` ist kein zugelassener Code.")))
                .remedy(Remedy::one_of(
                    SubjectRef::new(article, path, copy(format!("Attribute {key}"), format!("Attribut {key}"))),
                    codes.iter().map(|s| (*s).to_string()).collect(),
                    copy(format!("Set `{key}` to an admitted Blatt code."), format!("`{key}` auf einen zugelassenen Blatt-Code setzen.")),
                ));
        }
        out.push(c.build());
    }

    for (key, min_v, max_v) in sheet_numeric_bounds(sheet) {
        let raw = kv.get(*key).cloned().or_else(|| generic_entries.iter().find(|e| e.key.eq_ignore_ascii_case(key)).map(|e| e.value.clone()));
        let Some(raw) = raw.filter(|s| !s.is_empty()) else { continue };
        let Ok(value) = raw.parse::<f64>() else { continue };
        let ok = value >= *min_v && value <= *max_v;
        let path = generic_entry_value_path(article, &generic_entries, key);
        let mut c = CheckResult::assess(
            format!("vdi3805.{sheet}.range.{key}.{article}"),
            part_label(sheet),
            clause(&sheet.to_string(), "4.2"),
            subject_product(product),
            copy(format!("Blatt {sheet} attribute `{key}` range"), format!("Blatt-{sheet}-Attribut `{key}` Bereich")),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Dimensionless, value), Quantity::new(QuantityKind::Dimensionless, *max_v))
        .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if ok {
            c = c.explanation(copy(format!("{key} = {value} within [{min_v}, {max_v}]."), format!("{key} = {value} in [{min_v}, {max_v}].")));
        } else {
            let required = value.clamp(*min_v, *max_v);
            c = c
                .explanation(copy(format!("{key} = {value} outside [{min_v}, {max_v}]."), format!("{key} = {value} außerhalb [{min_v}, {max_v}].")))
                .remedy(Remedy::exactly(
                    SubjectRef::new(article, path, copy(format!("Attribute {key}"), format!("Attribut {key}"))),
                    Quantity::new(QuantityKind::Dimensionless, value),
                    Quantity::new(QuantityKind::Dimensionless, required),
                    copy(format!("Clamp `{key}` into [{min_v}, {max_v}]."), format!("`{key}` in [{min_v}, {max_v}] begrenzen.")),
                ));
        }
        out.push(c.build());
    }

    for curve_id in &product.configuration.function_refs {
        if let Some(curve) = document.curves.get(curve_id) {
            let mono = curve.points.windows(2).all(|w| w[1].x + 1e-12 >= w[0].x);
            let mut c = CheckResult::assess(
                format!("vdi3805.{sheet}.curve.{article}.{curve_id}"),
                part_label(sheet),
                clause(&sheet.to_string(), "5.1"),
                subject_product(product),
                copy("Characteristic data monotonicity", "Kennlinien-Monotonie"),
            )
            .annex(ANNEX)
            .status(if mono { CheckStatus::Pass } else { CheckStatus::Fail });
            if mono {
                c = c.explanation(copy(format!("Curve {curve_id} x-order is non-decreasing."), format!("Kennlinie {curve_id}: x-Folge ist monoton.")));
            } else if let Some((idx, cur, req)) = mono_increasing_remedy(curve) {
                c = c
                    .explanation(copy(format!("Curve {curve_id} is not ordered."), format!("Kennlinie {curve_id} ist nicht geordnet.")))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(curve_id, curve_point_path(curve_id, idx, "x"), copy("Curve point x", "Kennlinienpunkt x")),
                        Quantity::new(QuantityKind::Dimensionless, cur),
                        Quantity::new(QuantityKind::Dimensionless, req),
                        copy("Repair characteristic point so x is non-decreasing.", "Kennlinienpunkt so korrigieren, dass x monoton steigt."),
                    ));
            }
            out.push(c.build());
        }
    }
    out
}

pub(super) fn check_sheet_product(document: &Vdi3805Snapshot, product: &CatalogueProduct) -> Vec<CheckResult> {
    let mut out = check_attributes_records_sync(product);
    match product.sheet.0 {
        2 => {
            let attrs = valve_attrs_from_product(product);
            out.extend(check_valve(document, product, &attrs));
        }
        3 => {
            let attrs = radiator_attrs_from_product(product);
            out.extend(check_radiator(product, &attrs));
        }
        5 => {
            let attrs = pump_attrs_from_product(product);
            out.extend(check_pump(document, product, &attrs));
        }
        6 => {
            let attrs = heat_attrs_from_product(product);
            out.extend(check_heat_generator(product, &attrs));
        }
        _ => out.extend(check_operative_sheet(document, product)),
    }
    out
}
//#endregion 🔖️ComplianceReport
