//! 🗓️ Correction date and strict mode checks: the correction date, its chronology and the strict mode header of the catalog integrity check. See [`super`].

use super::*;

/// 🗓 Checks the correction date range and its chronology against the file creation date.
pub(super) fn check_correction_date(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>) {
    let year = document.correction_as_of.year;
    let month = document.correction_as_of.month;
    let range_ok = (1990..=2100).contains(&year) && (1..=12).contains(&month);
    let created = document.catalog.file.created.as_str();
    let created_key = {
        let parts: Vec<_> = created.split('-').collect();
        match (parts.first().and_then(|y| y.parse::<u16>().ok()), parts.get(1).and_then(|m| m.parse::<u8>().ok())) {
            (Some(y), Some(m)) => Some((y as u32) * 100 + m as u32),
            _ => None,
        }
    };
    let corr_key = (year as u32) * 100 + month as u32;
    let chronology_ok = created_key.map(|c| c <= corr_key).unwrap_or(false);
    let date_ok = range_ok && chronology_ok;
    let mut corr = CheckResult::assess(
        "vdi3805.1.correctionAsOf",
        part_label(1),
        clause("1", "4.3"),
        subject_dataset(),
        copy("Correction date", "Korrekturdatum"),
    )
    .annex(ANNEX)
    .utilization(
        Quantity::new(QuantityKind::Dimensionless, created_key.unwrap_or(u32::MAX) as f64),
        Quantity::new(QuantityKind::Dimensionless, corr_key as f64),
    );
    if date_ok {
        corr = corr.status(CheckStatus::Pass).explanation(copy(
            format!("created `{created}` is on or before correctionAsOf {year:04}-{month:02}."),
            format!("created `{created}` liegt auf oder vor correctionAsOf {year:04}-{month:02}."),
        ));
    } else {
        let remedy_year = created_key.map(|c| (c / 100) as f64).unwrap_or(2024.0);
        corr = corr
            .status(CheckStatus::Fail)
            .explanation(copy(
                format!("correctionAsOf {year:04}-{month:02} must be a valid stamp on or after created `{created}`."),
                format!("correctionAsOf {year:04}-{month:02} muss ein gültiger Stempel auf oder nach created `{created}` sein."),
            ))
            .remedy(Remedy::at_least(
                SubjectRef::new("", "correctionAsOf.year", copy("Correction year", "Korrekturjahr")),
                Quantity::new(QuantityKind::Dimensionless, year as f64),
                Quantity::new(QuantityKind::Dimensionless, remedy_year),
                copy(
                    "Raise correctionAsOf.year/month so the stamp is on or after catalog.file.created.",
                    "correctionAsOf.year/month so anheben, dass der Stempel auf oder nach catalog.file.created liegt.",
                ),
            ));
    }
    out.push(corr.build());
}

/// 🔒 Checks the charset and the header version of the strict mode.
pub(super) fn check_strict_mode(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>) {
    if document.strict_mode {
        let charset_ok = document.catalog.file.charset.eq_ignore_ascii_case("UTF-8");
        let hv_ok = document.catalog.file.header_version == "3805";
        let strict_ok = charset_ok && hv_ok;
        let mut strict_c = CheckResult::assess(
            "vdi3805.1.strictMode",
            part_label(1),
            clause("1", "4.3"),
            subject_dataset(),
            copy("Strict evaluation mode", "Strenger Auswertungsmodus"),
        )
        .annex(ANNEX)
        .status(if strict_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if strict_ok {
            strict_c = strict_c.explanation(copy(
                "strictMode requires UTF-8 charset and headerVersion 3805 — both present.",
                "strictMode erfordert charset UTF-8 und headerVersion 3805 — beides vorhanden.",
            ));
        } else {
            strict_c = strict_c
                .explanation(copy(
                    "strictMode requires charset UTF-8 and headerVersion 3805.",
                    "strictMode erfordert charset UTF-8 und headerVersion 3805.",
                ))
                .remedy(Remedy::exactly(
                    SubjectRef::new("", "catalog.file.charset", copy("Charset", "Zeichensatz")),
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    copy("Set catalog.file.charset to UTF-8 while strictMode is enabled.", "catalog.file.charset auf UTF-8 setzen, solange strictMode aktiv ist."),
                ));
        }
        out.push(strict_c.build());
    } else {
        out.push(
            CheckResult::assess(
                "vdi3805.1.strictMode",
                part_label(1),
                clause("1", "4.3"),
                subject_dataset(),
                copy("Strict evaluation mode", "Strenger Auswertungsmodus"),
            )
            .annex(ANNEX)
            .not_applicable(copy(
                "strictMode is off — charset/header strict gate does not apply.",
                "strictMode ist aus — strenge charset/header-Prüfung gilt nicht.",
            ))
            .build(),
        );
    }
}
