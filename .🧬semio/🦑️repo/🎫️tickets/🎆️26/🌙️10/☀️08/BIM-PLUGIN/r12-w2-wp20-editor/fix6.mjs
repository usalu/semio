import fs from "node:fs";
let s = fs.readFileSync("entities-energy.rs", "utf8");
const a = s.indexOf("//#region 🔖️Rows");
const b = s.indexOf("//#endregion 🔖️Rows");
const rows = `//#region 🔖️Rows
/// 🏘️ One total of a zone, \`None\` while the zone does not exist or has no totals.
pub fn zone_total(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str, pick: fn(&EnergyTotals) -> f64) -> Option<String> {
    snapshot.zones.contains_key(id).then(|| scope_total(inference, EnergyScope::Zone(id.into()), pick)).flatten()
}

/// 🏢️ One total of a building, \`None\` while the building does not exist or has no totals.
pub fn building_total(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str, pick: fn(&EnergyTotals) -> f64) -> Option<String> {
    snapshot.buildings.contains_key(id).then(|| scope_total(inference, EnergyScope::Building(id.into()), pick)).flatten()
}

/// 💡️ The thermal totals of a building.
pub static BUILDING_INFERRED: &[InferredRow] = &[
    inferred!("envelope_area", field_envelope_area, |s, inference, id| building_total(s, inference, id, |totals| totals.envelope_area)),
    inferred!("h_t_prime", field_h_t_prime, |s, inference, id| building_total(s, inference, id, |totals| totals.h_t_prime)),
    inferred!("a_over_v", field_a_over_v, |s, inference, id| building_total(s, inference, id, |totals| totals.a_over_v)),
    inferred!("glazing_ratio", field_glazing_ratio, |s, inference, id| building_total(s, inference, id, |totals| totals.glazing_ratio)),
];
`;
s = s.slice(0, a) + rows + s.slice(b);
fs.writeFileSync("entities-energy.rs", s);
