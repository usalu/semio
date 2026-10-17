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

#[path = "🚰️valve-radiator-and-pump-checks/🦀️.rs"]
mod valve_radiator_and_pump_checks;
use valve_radiator_and_pump_checks::*;

#[path = "🔥️heat-generator-and-operative-sheet-checks/🦀️.rs"]
mod heat_generator_and_operative_sheet_checks;
use heat_generator_and_operative_sheet_checks::*;

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
