//! 📏️ Geometry and climate checks: the heated volume, the climate table, the net floor area and the element zone linkage of the document evaluation. See [`super`].

use super::*;

/// 📏 Checks the heated gross volume against the sum of the zone net volumes.
pub(super) fn check_heated_volume(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
    {
        let v_e = doc.heated_volume_m3;
        let v_net: f64 = doc.zones.iter().map(|z| z.volume_m3).sum();
        let v_net_min = DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO * v_e;
        let mut b = CheckResult::assess(
            "din18599.1.heated-volume",
            "DIN V 18599-1",
            ClauseId::new("DIN V 18599", "1", "6.1.1"),
            SubjectRef::new("", "heatedVolumeM3", copy("Heated gross volume V_e", "Beheiztes Bruttovolumen V_e")),
            copy("Heated volume V_e vs zone net volumes", "Bruttovolumen V_e gegenüber Nettovolumina"),
        )
        .annex(annex)
        .explanation(copy(
            &format!("V_e = {v_e:.1} m³; ΣV_zone = {v_net:.1} m³; A/V_e = {:.3} m⁻¹; DIN V 18599-1 expects V ≈ 0.80·V_e.", derived.a_over_v_e),
            &format!("V_e = {v_e:.1} m³; ΣV_zone = {v_net:.1} m³; A/V_e = {:.3} m⁻¹; DIN V 18599-1 erwartet V ≈ 0,80·V_e.", derived.a_over_v_e),
        ));
        if v_net > v_e + 1e-6 {
            b = b
                .utilization(Quantity::new(QuantityKind::Volume, v_net), Quantity::new(QuantityKind::Volume, v_e))
                .remedy(Remedy::at_least(
                    SubjectRef::new("", "heatedVolumeM3", copy("Heated gross volume V_e", "Beheiztes Bruttovolumen V_e")),
                    Quantity::new(QuantityKind::Volume, v_e),
                    Quantity::new(QuantityKind::Volume, v_net),
                    copy(
                        &format!("Raise V_e to at least {v_net:.1} m³ so gross volume covers Σ zone net volumes."),
                        &format!("V_e auf mindestens {v_net:.1} m³ anheben, damit das Bruttovolumen Σ Nettovolumina abdeckt."),
                    ),
                ));
        } else if v_net + 1e-6 < v_net_min {
            b = b
                .utilization(Quantity::new(QuantityKind::Volume, v_net), Quantity::new(QuantityKind::Volume, v_net_min))
                .remedy(Remedy::at_most(
                    SubjectRef::new("", "heatedVolumeM3", copy("Heated gross volume V_e", "Beheiztes Bruttovolumen V_e")),
                    Quantity::new(QuantityKind::Volume, v_e),
                    Quantity::new(QuantityKind::Volume, v_net / DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO),
                    copy(
                        &format!("Lower V_e toward {:.1} m³ so ΣV_zone / V_e ≥ 0.80 (DIN V 18599-1).", v_net / DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO),
                        &format!("V_e Richtung {:.1} m³ senken, damit ΣV_zone / V_e ≥ 0,80 (DIN V 18599-1).", v_net / DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO),
                    ),
                ));
        } else {
            b = b.utilization(Quantity::new(QuantityKind::Volume, v_net), Quantity::new(QuantityKind::Volume, v_e));
        }
        report.push(b.build());
    }
}

/// 🌦 Checks the derived climate table handle.
pub(super) fn check_climate_table(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice) {
    {
        let child_id = doc.climate_table.child_id.as_str();
        let target = &doc.climate_table.target;
        let derived = semio_s_artifact_norm_din18599_model::din18599_climate_table_child(&doc.climate);
        let ok = doc.climate_table == derived;
        let mut b = CheckResult::assess(
            "din18599.1.climate-composition",
            "DIN V 18599-10",
            ClauseId::new("DIN V 18599", "10", "climate"),
            SubjectRef::new("", "climateTable.childId", copy("Climate table", "Klimatabelle")),
            copy("Climate table derived from the monthly climate", "Klimatabelle aus dem Monatsklima abgeleitet"),
        )
        .annex(annex)
        .explanation(copy(
            &format!("Climate table '{child_id}' → {}@{}/{}.", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
            &format!("Klimatabelle '{child_id}' → {}@{}/{}.", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
        ));
        if ok {
            b = b.utilization(dimensionless(1.0), dimensionless(1.0));
        } else {
            b = b
                .utilization(dimensionless(0.0), dimensionless(1.0))
                .remedy(Remedy::one_of(
                    SubjectRef::new("", "climateTable.childId", copy("Climate table id", "Klimatabellen-ID")),
                    vec![derived.child_id.clone()],
                    copy(
                        "Restore the s.stdio.semio@v1/table handle derived from the monthly climate.",
                        "Den aus dem Monatsklima abgeleiteten s.stdio.semio@v1/table-Handle wiederherstellen.",
                    ),
                ));
        }
        report.push(b.build());
    }
}

/// 📐 Checks the net floor area against the sum of the zone areas.
pub(super) fn check_net_floor_area(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice) {
    {
        let a_n = doc.net_floor_area_m2;
        let a_zones: f64 = doc.zones.iter().map(|z| z.area_m2).sum();
        let mut b = CheckResult::assess(
            "din18599.1.net-floor-area",
            "DIN V 18599-1",
            ClauseId::new("DIN V 18599", "1", "A_N"),
            SubjectRef::new("", "netFloorAreaM2", copy("Net floor area A_N", "Nettogrundfläche A_N")),
            copy("Net floor area vs zone areas", "Nettogrundfläche gegenüber Zonenflächen"),
        )
        .utilization(Quantity::new(QuantityKind::Area, a_zones), Quantity::new(QuantityKind::Area, a_n.max(1e-9)))
        .annex(annex)
        .explanation(copy(
            &format!("Net floor area A_N = {a_n:.1} m²; sum of zone areas ΣA_zone = {a_zones:.1} m²."),
            &format!("Nettogrundfläche A_N = {a_n:.1} m²; Summe der Zonenflächen ΣA_zone = {a_zones:.1} m²."),
        ));
        if a_zones > a_n {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new("", "netFloorAreaM2", copy("Net floor area A_N", "Nettogrundfläche A_N")),
                Quantity::new(QuantityKind::Area, a_n),
                Quantity::new(QuantityKind::Area, a_zones),
                copy(
                    &format!("Set A_N to {a_zones:.1} m² to match Σ zone areas."),
                    &format!("A_N auf {a_zones:.1} m² setzen, damit Σ Zonenflächen übereinstimmt."),
                ),
            ));
        }
        report.push(b.build());
    }
}

/// 🔗 Checks that every envelope element links to a declared zone.
pub(super) fn check_element_zone_linkage(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice) {
    {
        let zone_ids: Vec<String> = doc.zones.iter().map(|z| z.id.clone()).collect();
        for el in &doc.elements {
            let path = format!("elements[id={}].zoneId", el.id);
            let ok = zone_ids.iter().any(|z| z == &el.zone_id);
            let mut b = CheckResult::assess(
                format!("din18599.1.element-zone.{}", el.id),
                "DIN V 18599-1",
                ClauseId::new("DIN V 18599", "1", "zone"),
                SubjectRef::new(&el.id, &path, copy("Element zone", "Bauteilzone")),
                copy("Envelope element zone assignment", "Zonenzuordnung Hüllbauteil"),
            )
            .annex(annex)
            .explanation(copy(
                &format!("Element {} assigned to zone '{}'.", el.label_en, el.zone_id),
                &format!("Bauteil {} der Zone '{}' zugeordnet.", el.label_de, el.zone_id),
            ));
            if ok {
                b = b.utilization(dimensionless(1.0), dimensionless(1.0));
            } else {
                b = b
                    .utilization(dimensionless(0.0), dimensionless(1.0))
                    .remedy(Remedy::one_of(
                        SubjectRef::new(&el.id, &path, copy("Element zone", "Bauteilzone")),
                        zone_ids.clone(),
                        copy(
                            &format!("Assign {} to an existing zone id.", el.label_en),
                            &format!("{} einer vorhandenen Zonen-ID zuordnen.", el.label_de),
                        ),
                    ));
            }
            report.push(b.build());
        }
    }
}
