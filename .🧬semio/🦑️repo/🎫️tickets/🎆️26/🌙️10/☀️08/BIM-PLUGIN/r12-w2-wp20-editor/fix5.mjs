import fs from "node:fs";
let s = fs.readFileSync("entities-energy.rs", "utf8");
s = s.replace("//#endregion 🔖️Conditions", `/// 🌡️ The conditions of a space in one line for the outliner: the set points as a range or a bound in degrees Celsius, then the occupancy type and the schedule profile.
pub fn summary(conditions: &SpaceConditions) -> String {
    let setpoints = match (conditions.heating_setpoint, conditions.cooling_setpoint) {
        (Some(heating), Some(cooling)) => Some(format!("{}–{} °C", number(heating), number(cooling))),
        (Some(heating), None) => Some(format!("≥ {} °C", number(heating))),
        (None, Some(cooling)) => Some(format!("≤ {} °C", number(cooling))),
        (None, None) => None,
    };
    [setpoints, conditions.occupancy.clone(), conditions.schedule.clone()].into_iter().flatten().collect::<Vec<_>>().join(" · ")
}
//#endregion 🔖️Conditions`);
fs.writeFileSync("entities-energy.rs", s);
let t = fs.readFileSync("entities-energy-tests.rs", "utf8");
t += `
#[semio_framework_async_macros::async_test]
async fn the_summary_names_the_set_points_the_occupancy_and_the_schedule_in_one_line() {
    let both = SpaceConditions { heating_setpoint: Some(21.0), cooling_setpoint: Some(26.5), occupancy: Some("Office".into()), schedule: Some("Office 08-18".into()), ..SpaceConditions::empty() };
    assert_eq!(summary(&both), "21–26.5 °C · Office · Office 08-18");
    assert_eq!(summary(&SpaceConditions { heating_setpoint: Some(20.0), ..SpaceConditions::empty() }), "≥ 20 °C");
    assert_eq!(summary(&SpaceConditions { cooling_setpoint: Some(24.0), ..SpaceConditions::empty() }), "≤ 24 °C");
    assert_eq!(summary(&SpaceConditions::empty()), "");
}
`;
fs.writeFileSync("entities-energy-tests.rs", t);
