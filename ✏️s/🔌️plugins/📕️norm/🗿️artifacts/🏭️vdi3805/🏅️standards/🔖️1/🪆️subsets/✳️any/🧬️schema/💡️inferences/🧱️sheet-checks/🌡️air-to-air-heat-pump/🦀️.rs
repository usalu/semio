//! 🌡️ Part 22 heat-pump records and published correction fields in catalogue reports.

use super::*;

#[path = "📏️fields.rs"]
mod fields;

#[path = "⚖️published-corrections.rs"]
mod corrections;

fn field_remedy(product: &CatalogueProduct, index: usize) -> Remedy {
    let mut remedy = Remedy::exactly(
        SubjectRef::new(&product.id, format!("catalog.products[id={}].records[{index}]", product.id), copy("Product record", "Produktdatensatz")),
        Quantity::new(QuantityKind::Dimensionless, 0.0),
        Quantity::new(QuantityKind::Dimensionless, 1.0),
        copy("Review the indicated positions against the applicable correction and re-import the supplier record.", "Die angegebenen Felder anhand der gültigen Berichtigung prüfen und den Herstellerdatensatz erneut importieren."),
    );
    remedy.applicable = false;
    remedy
}

fn check_published_fields(product: &CatalogueProduct, family: &str, index: usize, fields: corrections::PublishedFields) -> CheckResult {
    let check_id = fields.check;
    let correction = fields.corrections;
    let check = CheckResult::assess(
        format!("{check_id}.{}.{index}", product.id),
        part_label(22),
        clause("22", fields.clause),
        subject_product(product),
        copy(format!("Published correction fields in record {family}"), format!("Veröffentlichte Berichtigungsfelder in Datensatz {family}")),
    ).annex(ANNEX);
    if fields.invalid.is_empty() {
        return check.status(CheckStatus::Pass).explanation(copy(
            format!("Positions {:?} satisfy the selected edition of the published correction {correction}.", fields.positions),
            format!("Die Felder {:?} erfüllen den gewählten Stand der veröffentlichten Berichtigung {correction}.", fields.positions),
        )).build();
    }
    check.status(CheckStatus::Fail).explanation(copy(
        format!("Positions {:?} are missing or invalid at the selected correction date ({correction}).", fields.invalid),
        format!("Die Felder {:?} fehlen oder sind für den gewählten Berichtigungsstand ungültig ({correction}).", fields.invalid),
    )).remedy(field_remedy(product, index)).build()
}

pub(super) fn check_records(document: &Vdi3805Snapshot, product: &CatalogueProduct) -> Vec<CheckResult> {
    product.records.iter().enumerate().filter_map(|(index, record)| {
        let family = record.family.0.as_str();
        let (check, correction) = match family {
            RecordFamilyId::R710_15 => ("vdi3805.22.record71015", "22-0005"),
            RecordFamilyId::R710_16 => ("vdi3805.22.record71016", "22-0006 / 22-0007"),
            RecordFamilyId::R710_17 => ("vdi3805.22.record71017", "22-0008"),
            _ => return corrections::published_fields(family, &record.fields, document.correction_as_of.key()).map(|fields| check_published_fields(product, family, index, fields)),
        };
        let result = fields::validate_fields(family, &record.fields)?;
        let part = CheckResult::assess(
            format!("{check}.{}.{index}", product.id),
            part_label(22),
            clause("22", "5.2.3"),
            subject_product(product),
            copy(format!("Air-to-air heat pump record {family}"), format!("Datensatz {family} für Luft-Luft-Wärmepumpen")),
        ).annex(ANNEX);
        let part = match (document.correction_as_of.key() >= 202201, result) {
            (false, _) => part.status(CheckStatus::Fail).remedy(field_remedy(product, index)).explanation(copy(
                format!("Correction {correction} took effect on 1 January 2022, after the selected correction date."),
                format!("Berichtigung {correction} gilt ab 1. Januar 2022 und damit nach dem gewählten Berichtigungsstand."),
            )),
            (_, Ok(())) => part.status(CheckStatus::Pass).explanation(copy(
                format!("Field count, numeric formats, codes and conditional fields match correction {correction}."),
                format!("Feldanzahl, Zahlenformate, Kennziffern und bedingte Felder entsprechen Berichtigung {correction}."),
            )),
            (_, Err(fields::RecordError::FieldCount(count))) => part.status(CheckStatus::Fail).remedy(field_remedy(product, index)).explanation(copy(
                format!("Record {family} contains {} fields. Correction {correction} specifies {count}.", record.fields.len()),
                format!("Datensatz {family} enthält {} Felder. Berichtigung {correction} gibt {count} vor.", record.fields.len()),
            )),
            (_, Err(fields::RecordError::Fields(positions))) => part.status(CheckStatus::Fail).remedy(field_remedy(product, index)).explanation(copy(
                format!("Fields {positions:?} do not match correction {correction}."),
                format!("Felder {positions:?} entsprechen Berichtigung {correction} nicht."),
            )),
        };
        Some(part.build())
    }).collect()
}
