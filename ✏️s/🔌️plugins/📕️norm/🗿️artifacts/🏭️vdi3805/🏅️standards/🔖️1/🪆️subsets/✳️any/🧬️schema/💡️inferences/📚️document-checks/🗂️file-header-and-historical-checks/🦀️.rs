//! 🗂️ VDI 3805 catalog file header, historical sheet, edition profile and operative sheet coverage checks.

use super::*;

/// 🗂️ Catalog file header checks of [`check_catalog_integrity`] (header version, building-system number, created date, manufacturer, charset), moved out unchanged.
pub(super) fn check_catalog_file_header(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>) {
    // Header version / building-system / created (VDI 3805-1 §4.1–4.3) — single header at catalog.file.
    let file = &document.catalog.file;
    let expected_hv = "3805";
    let hv_ok = file.header_version == expected_hv;
    let mut hv = CheckResult::assess(
        "vdi3805.1.headerVersion",
        part_label(1),
        clause("1", "4.1"),
        subject_dataset(),
        copy("Manufacturer file header version", "Herstellerdatei-Kopfversion"),
    )
    .annex(ANNEX)
    .status(if hv_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if hv_ok {
        hv = hv.explanation(copy(format!("headerVersion `{}` matches the Blatt edition family.", file.header_version), format!("headerVersion `{}` entspricht der Blatt-Ausgabenfamilie.", file.header_version)));
    } else {
        hv = hv
            .explanation(copy(format!("headerVersion `{}` must identify VDI 3805 (e.g. `3805`).", file.header_version), format!("headerVersion `{}` muss VDI 3805 kennzeichnen (z. B. `3805`).", file.header_version)))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "catalog.file.headerVersion", copy("Header version", "Kopfversion")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 3805.0),
                copy("Set catalog.file.headerVersion to `3805` for the declared Blatt edition.", "catalog.file.headerVersion auf `3805` für die deklarierte Blattausgabe setzen."),
            ));
    }
    out.push(hv.build());

    let sheets: BTreeSet<u16> = document.catalog.products.iter().map(|p| p.sheet.0).collect();
    // Building-system (Gewerk) number must be consistent with product sheets' domain (VDI 3805-1 §4.2).
    let bsn = &file.building_system_number;
    let bsn_ok = !bsn.system_code.is_empty()
        && !bsn.subsystem.is_empty()
        && bsn.system_code.chars().all(|c| c.is_ascii_digit())
        && bsn.subsystem.chars().all(|c| c.is_ascii_digit())
        && bsn.sequence >= 1;
    let mut bs = CheckResult::assess(
        "vdi3805.1.buildingSystemNumber",
        part_label(1),
        clause("1", "4.2"),
        subject_dataset(),
        copy("Building system number", "Gewerknummer"),
    )
    .annex(ANNEX)
    .minimum(
        Quantity::new(QuantityKind::Dimensionless, bsn.sequence as f64),
        Quantity::new(QuantityKind::Dimensionless, 1.0),
    )
    .status(if bsn_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if bsn_ok {
        bs = bs.explanation(copy(
            format!("buildingSystemNumber {}.{}.{} present; products on sheets {:?}.", bsn.system_code, bsn.subsystem, bsn.sequence, sheets),
            format!("buildingSystemNumber {}.{}.{} vorhanden; Produkte auf Blättern {:?}.", bsn.system_code, bsn.subsystem, bsn.sequence, sheets),
        ));
    } else {
        bs = bs
            .explanation(copy("buildingSystemNumber is incomplete.", "buildingSystemNumber ist unvollständig."))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "catalog.file.buildingSystemNumber.systemCode", copy("System code", "Systemcode")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 420.0),
                copy("Set catalog.file.buildingSystemNumber to the Gewerk matching the product sheets.", "catalog.file.buildingSystemNumber auf das zu den Produktblättern passende Gewerk setzen."),
            ));
    }
    out.push(bs.build());

    // created vs correctionAsOf / edition dates (VDI 3805-1 §4.3).
    let created_ok = {
        let parts: Vec<_> = file.created.split('-').collect();
        if parts.len() == 3 && parts.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit())) {
            let y: u16 = parts[0].parse().unwrap_or(0);
            let m: u16 = parts[1].parse().unwrap_or(0);
            let d: u16 = parts[2].parse().unwrap_or(0);
            y >= 1990
                && m >= 1
                && m <= 12
                && d >= 1
                && d <= 31
                && (y as i32 * 12 + m as i32) <= (document.correction_as_of.year as i32 * 12 + document.correction_as_of.month as i32 + 12)
        } else {
            false
        }
    };
    let mut cr = CheckResult::assess(
        "vdi3805.1.created",
        part_label(1),
        clause("1", "4.3"),
        subject_dataset(),
        copy("Manufacturer file created date", "Erstellungsdatum der Herstellerdatei"),
    )
    .annex(ANNEX)
    .status(if created_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if created_ok {
        cr = cr.explanation(copy(
            format!("created `{}` is consistent with correctionAsOf {}-{:02}.", file.created, document.correction_as_of.year, document.correction_as_of.month),
            format!("created `{}` ist konsistent mit correctionAsOf {}-{:02}.", file.created, document.correction_as_of.year, document.correction_as_of.month),
        ));
    } else {
        cr = cr
            .explanation(copy(
                format!("created `{}` must be a YYYY-MM[-DD] date not after correctionAsOf.", file.created),
                format!("created `{}` muss ein YYYY-MM[-DD]-Datum nicht nach correctionAsOf sein.", file.created),
            ))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "catalog.file.created", copy("Created date", "Erstellungsdatum")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, document.correction_as_of.year as f64),
                copy("Set catalog.file.created to a date consistent with correctionAsOf / edition dates.", "catalog.file.created auf ein zu correctionAsOf/Ausgabedaten passendes Datum setzen."),
            ));
    }
    out.push(cr.build());

    for (key, geom) in &document.geometry {
        let ok = key == &geom.id;
        let mut gk = CheckResult::assess(
            format!("vdi3805.1.geometry.idKey.{key}"),
            part_label(1),
            clause("1", "5.2"),
            subject_dataset(),
            copy("Geometry map key", "Geometrie-Map-Schlüssel"),
        )
        .annex(ANNEX)
        .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if ok {
            gk = gk.explanation(copy(format!("geometry map key `{key}` matches id.",), format!("Geometrie-Map-Schlüssel `{key}` entspricht id.")));
        } else {
            gk = gk
                .explanation(copy(format!("geometry[`{key}`].id is `{}`.", geom.id), format!("geometry[`{key}`].id ist `{}`.", geom.id)))
                .remedy(Remedy::one_of(
                    SubjectRef::new(key, format!("geometry[id={key}].id"), copy("Geometry id", "Geometrie-ID")),
                    vec![key.clone()],
                    copy("Align geometry.id with its map key.", "geometry.id an den Map-Schlüssel angleichen."),
                ));
        }
        out.push(gk.build());
    }
    for (key, curve) in &document.curves {
        let ok = key == &curve.id;
        let mut ck = CheckResult::assess(
            format!("vdi3805.1.curve.idKey.{key}"),
            part_label(1),
            clause("1", "5.1"),
            subject_dataset(),
            copy("Curve map key", "Kennlinien-Map-Schlüssel"),
        )
        .annex(ANNEX)
        .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if ok {
            ck = ck.explanation(copy(format!("curves map key `{key}` matches id.",), format!("Kennlinien-Map-Schlüssel `{key}` entspricht id.")));
        } else {
            ck = ck
                .explanation(copy(format!("curves[`{key}`].id is `{}`.", curve.id), format!("curves[`{key}`].id ist `{}`.", curve.id)))
                .remedy(Remedy::one_of(
                    SubjectRef::new(key, format!("curves[id={key}].id"), copy("Curve id", "Kennlinien-ID")),
                    vec![key.clone()],
                    copy("Align curve.id with its map key.", "curve.id an den Map-Schlüssel angleichen."),
                ));
        }
        out.push(ck.build());
    }

    let mfr_match = !file.manufacturer.is_empty()
        && document.catalog.products.iter().all(|p| p.identity.manufacturer_code == file.manufacturer);
    let mut mf = CheckResult::assess(
        "vdi3805.1.manufacturer",
        part_label(1),
        clause("1", "4.1"),
        subject_dataset(),
        copy("Manufacturer code", "Herstellercode"),
    )
    .annex(ANNEX)
    .status(if mfr_match { CheckStatus::Pass } else { CheckStatus::Fail });
    if mfr_match {
        mf = mf.explanation(copy(
            format!("manufacturer `{}` matches product identity codes.", file.manufacturer),
            format!("manufacturer `{}` entspricht den Produktidentitätscodes.", file.manufacturer),
        ));
    } else {
        mf = mf
            .explanation(copy(
                format!("manufacturer `{}` must match every product identity.manufacturerCode.", file.manufacturer),
                format!("manufacturer `{}` muss mit jeder product identity.manufacturerCode übereinstimmen.", file.manufacturer),
            ))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "catalog.file.manufacturer", copy("Manufacturer", "Hersteller")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                copy("Set catalog.file.manufacturer to the catalogue manufacturer code.", "catalog.file.manufacturer auf den Katalog-Herstellercode setzen."),
            ));
    }
    out.push(mf.build());

    let charset_ok = matches!(file.charset.as_str(), "UTF-8" | "utf-8" | "ISO-8859-1" | "CP1252");
    let mut cs = CheckResult::assess(
        "vdi3805.1.charset",
        part_label(1),
        clause("1", "4.1"),
        subject_dataset(),
        copy("Manufacturer file charset", "Zeichensatz der Herstellerdatei"),
    )
    .annex(ANNEX)
    .status(if charset_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if charset_ok {
        cs = cs.explanation(copy(format!("charset `{}` is admitted.", file.charset), format!("charset `{}` ist zugelassen.", file.charset)));
    } else {
        cs = cs
            .explanation(copy(format!("charset `{}` is not an admitted Part 1 encoding.", file.charset), format!("charset `{}` ist keine zugelassene Teil-1-Kodierung.", file.charset)))
            .remedy(Remedy::one_of(
                SubjectRef::new("", "catalog.file.charset", copy("Charset", "Zeichensatz")),
                vec!["UTF-8".into(), "ISO-8859-1".into(), "CP1252".into()],
                copy("Set catalog.file.charset to UTF-8 (preferred) or ISO-8859-1/CP1252.", "catalog.file.charset auf UTF-8 (bevorzugt) oder ISO-8859-1/CP1252 setzen."),
            ));
    }
    out.push(cs.build());
}

pub(super) fn check_historical(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let mut out = Vec::new();
    for product in &document.catalog.products {
        let entry = SHEET_ENTRIES.get(product.sheet.0.saturating_sub(1) as usize);
        if entry.is_some_and(|e| e.status == SchemaStatus::HistoricalProposal) {
            let mut b = CheckResult::assess(
                format!("vdi3805.{}.historical.{}", product.sheet.0, product.id),
                part_label(product.sheet.0),
                clause(&product.sheet.0.to_string(), "status"),
                subject_product(product),
                copy("Historical proposal sheet", "Historischer Vorschlagsblatt"),
            )
            .annex(ANNEX);
            if document.strict_mode {
                b = b
                    .status(CheckStatus::Fail)
                    .explanation(copy("Historical proposal sheet not allowed in strict mode.", "Historisches Vorschlagsblatt in Strict Mode nicht zulässig."))
                    .remedy(Remedy::exactly(
                        SubjectRef::new("", "strictMode", copy("Strict mode", "Strenger Modus")),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        copy("Remove the historical-sheet product or set strictMode to false.", "Produkt mit historischem Blatt entfernen oder strictMode auf false setzen."),
                    ));
            } else {
                b = b.status(CheckStatus::Pass).explanation(copy("Historical proposal acknowledged (strict mode off).", "Historischer Vorschlag anerkannt (Strict Mode aus)."));
            }
            out.push(b.build());
        }
    }
    out
}

pub(super) fn check_edition_profiles(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let mut out = Vec::new();
    for (key, profile) in &document.edition_profile {
        let Ok(sheet) = key.parse::<u16>() else { continue };
        let product = document.catalog.products.iter().find(|p| p.sheet.0 == sheet);
        let mut b = CheckResult::assess(
            format!("vdi3805.{sheet}.edition_profile"),
            part_label(sheet),
            clause(&sheet.to_string(), "edition"),
            subject_dataset(),
            copy("Edition profile selection", "Editionsprofilwahl"),
        )
        .annex(ANNEX);
        let Some(product) = product else {
            out.push(b.not_applicable(copy(format!("No product for multi-profile sheet {sheet}."), format!("Kein Produkt für Mehrprofil-Blatt {sheet}."))).build());
            continue;
        };
        let keys = sheet_mandatory_keys(sheet, *profile);
        let kv = record_kv(&product.records);
        let missing: Vec<_> = keys
            .iter()
            .copied()
            .filter(|key| match *key {
                "product_group" => product.identity.product_group.is_empty() && !kv.contains_key("product_group"),
                other => !kv.get(other).is_some_and(|v| !v.is_empty()),
            })
            .collect();
        if missing.is_empty() {
            b = b
                .status(CheckStatus::Pass)
                .explanation(copy(format!("Sheet {sheet} profile {profile:?} mandatory set satisfied."), format!("Blatt {sheet} Profil {profile:?}: Pflichtsatz erfüllt.")));
        } else {
            b = b
                .status(CheckStatus::Fail)
                .explanation(copy(
                    format!("Profile {profile:?} missing mandatory keys {missing:?}."),
                    format!("Profil {profile:?} fehlt Pflichtfelder {missing:?}."),
                ))
                .remedy({
                    let first = missing.first().copied().unwrap_or("product_group");
                    let path = if first == "product_group" {
                        product_path(&product.id, "identity.productGroup")
                    } else {
                        let entries = match &product.configuration.attributes {
                            SheetAttributes::Generic(g) => g.entries.clone(),
                            _ => Vec::new(),
                        };
                        generic_entry_value_path(&product.id, &entries, first)
                    };
                    Remedy::exactly(
                        SubjectRef::new(&product.id, path, copy("Edition mandatory attribute", "Editions-Pflichtattribut")),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy(
                            format!("Set `{first}` for {profile:?} on sheet {sheet}; sync regenerates the 210 record."),
                            format!("`{first}` für {profile:?} auf Blatt {sheet} setzen; Sync erzeugt den 210-Satz neu."),
                        ),
                    )
                });
        }
        out.push(b.build());
    }
    out
}

pub(super) fn check_operative_sheet_coverage(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let missing: Vec<u16> = SHEET_ENTRIES
        .iter()
        .filter(|e| is_operative(e.status) && e.id.0 != 1)
        .filter(|e| !document.catalog.products.iter().any(|p| p.sheet == e.id))
        .map(|e| e.id.0)
        .collect();
    if missing.is_empty() {
        return Vec::new();
    }
    vec![CheckResult::assess(
        "vdi3805.coverage.operative",
        part_label(1),
        clause("1", "scope"),
        subject_dataset(),
        copy("Operative sheet coverage", "Abdeckung operativer Blätter"),
    )
    .annex(ANNEX)
    .not_applicable(copy(
        format!("{} operative sheets have no catalogue product (e.g. {:?}).", missing.len(), &missing[..missing.len().min(8)]),
        format!("{} operative Blätter ohne Katalogprodukt (z. B. {:?}).", missing.len(), &missing[..missing.len().min(8)]),
    ))
    .build()]
}
