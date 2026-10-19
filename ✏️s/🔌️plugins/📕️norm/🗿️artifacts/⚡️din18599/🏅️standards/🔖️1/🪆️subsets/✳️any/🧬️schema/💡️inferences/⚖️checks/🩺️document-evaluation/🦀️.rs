//! 🩺️ DIN V 18599 document evaluation: duplicate ids and the clause-by-clause evaluation of a snapshot.

use super::*;

#[path = "📏️geometry-and-climate-checks/🦀️.rs"]
mod geometry_and_climate_checks;
use geometry_and_climate_checks::{check_heated_volume, check_climate_table, check_net_floor_area, check_element_zone_linkage};

#[path = "🏠️envelope-and-heating-checks/🦀️.rs"]
mod envelope_and_heating_checks;
use envelope_and_heating_checks::{check_transmission_loss, check_monthly_heating_demand, check_ventilation_heat_recovery};

#[path = "💡️lighting-hot-water-and-cooling-checks/🦀️.rs"]
mod lighting_hot_water_and_cooling_checks;
use lighting_hot_water_and_cooling_checks::{check_lighting, check_heating_system_efficiency, check_hot_water, check_cooling_temperature, check_cooling_demand};

#[path = "🤖️automation-and-energy-checks/🦀️.rs"]
mod automation_and_energy_checks;
use automation_and_energy_checks::{check_automation, check_primary_energy, check_tabular_method};

//#region 🔖️Checks

/// 🆔 Fail when zone or envelope element ids are not unique (CORRECTION 14:42).
fn push_duplicate_ids(report: &mut CheckReport, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String, annex: AnnexChoice) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, copy(&format!("Duplicate {table} id"), &format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let free = if options.is_empty() { vec![format!("{id}-unique")] } else { options };
        let builder = CheckResult::assess(
            format!("din18599.integrity.duplicate.{table}.{id}"),
            "DIN V 18599 integrity",
            ClauseId::new("DIN V 18599", "1", "id"),
            subject.clone(),
            copy(&format!("Unique {table} id"), &format!("Eindeutige {table}-Id")),
        )
        .annex(annex)
        .explanation(copy(
            &format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
            &format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
        ))
        .status(CheckStatus::Fail)
        .remedy(Remedy::one_of(
            subject,
            free,
            copy(
                &format!("Rename the duplicated '{id}' entry to a free id."),
                &format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.build());
    }
}

/// ⚖️ Evaluate DIN V 18599 / GEG compliance for the building subject.
pub fn evaluate_document(doc: &Din18599Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    let annex = AnnexChoice::De;
    push_duplicate_ids(
        &mut report,
        "zones",
        &doc.zones.iter().map(|z| z.id.clone()).collect::<Vec<_>>(),
        &|id| format!("zones[id={id}].id"),
        annex,
    );
    push_duplicate_ids(
        &mut report,
        "elements",
        &doc.elements.iter().map(|e| e.id.clone()).collect::<Vec<_>>(),
        &|id| format!("elements[id={id}].id"),
        annex,
    );
    let derived = derive_balance(doc);
    check_heated_volume(doc, &mut report, annex, &derived);
    check_climate_table(doc, &mut report, annex);
    check_net_floor_area(doc, &mut report, annex);
    check_element_zone_linkage(doc, &mut report, annex);
    check_transmission_loss(doc, &mut report, annex, &derived);
    check_monthly_heating_demand(doc, &mut report, annex, &derived);
    check_ventilation_heat_recovery(doc, &mut report, annex, &derived);
    check_lighting(doc, &mut report, annex);
    check_heating_system_efficiency(doc, &mut report, annex, &derived);
    check_hot_water(doc, &mut report, annex, &derived);
    check_cooling_temperature(doc, &mut report, annex, &derived);
    check_cooling_demand(doc, &mut report, annex, &derived);
    check_automation(doc, &mut report, annex);
    check_primary_energy(doc, &mut report, annex, &derived);
    check_tabular_method(doc, &mut report, annex, &derived);
    report
}
//#endregion 🔖️Checks
