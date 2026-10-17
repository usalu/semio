//! 🔥️ VDI 3805 heat generator and operative sheet checks.

use super::*;

pub(super) fn check_heat_generator(product: &CatalogueProduct, attrs: &HeatGeneratorAttributes) -> Vec<CheckResult> {
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

pub(super) fn check_operative_sheet(document: &Vdi3805Snapshot, product: &CatalogueProduct) -> Vec<CheckResult> {
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
