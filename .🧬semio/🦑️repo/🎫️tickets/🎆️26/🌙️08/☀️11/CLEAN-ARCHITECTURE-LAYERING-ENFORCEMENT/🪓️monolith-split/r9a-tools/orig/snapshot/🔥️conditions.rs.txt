//! 🔥️ Authored thermal-data rules shared by the energy leaves, the editor, the `energy-envelope` inference and the exports: the limits of the conditions of a space and of the thermal data of a window or door type, and why
//! a record cannot be written. Pure, total and read-only: a refusal names the field below the record and carries the message.
//! 📎 https://www.iso.org/standard/65708.html (ISO 6946, thermal transmittance) and https://gbxml.org/schema_doc/7.03/GreenBuildingXML_Ver7.03.html

use super::{CurtainWallType, DoorType, SpaceConditions, WindowType};

//#region 🔖️Limits
/// 🌡️ The lowest set point a space accepts, in degrees Celsius.
pub const MIN_SETPOINT: f64 = -20.0;

/// 🌡️ The highest set point a space accepts, in degrees Celsius.
pub const MAX_SETPOINT: f64 = 50.0;

/// 🧱️ The largest thermal transmittance a type accepts, in watts per square metre and kelvin (a single pane in a metal frame is about 6).
pub const MAX_U_VALUE: f64 = 10.0;

/// 🔌️ A refusal: the field below the record and the message.
pub type Problem = (&'static str, &'static str);
//#endregion 🔖️Limits

impl SpaceConditions {
    /// 🌡️ Conditions that state nothing: the record a first `set-space-conditions` fills in.
    pub fn empty() -> Self {
        Self { occupancy: None, occupancy_density: None, heating_setpoint: None, cooling_setpoint: None, ventilation_rate: None, lighting_power_density: None, equipment_power_density: None, schedule: None }
    }

    /// 🌡️ Whether the space is heated: it states a heating set point.
    pub fn heated(&self) -> bool {
        self.heating_setpoint.is_some()
    }
}

fn blank(text: &Option<String>) -> bool {
    text.as_ref().is_some_and(|text| text.trim().is_empty())
}

fn non_negative(value: Option<f64>) -> bool {
    value.is_none_or(|value| value.is_finite() && value >= 0.0)
}

fn setpoint(value: Option<f64>) -> bool {
    value.is_none_or(|value| value.is_finite() && (MIN_SETPOINT..=MAX_SETPOINT).contains(&value))
}

/// 🩺️ Why the conditions of a space cannot be written, `None` when they can.
pub fn conditions_problem(conditions: &SpaceConditions) -> Option<Problem> {
    if blank(&conditions.occupancy) {
        return Some(("occupancy", "An occupancy type is not blank."));
    }
    if blank(&conditions.schedule) {
        return Some(("schedule", "A schedule reference is not blank."));
    }
    if !non_negative(conditions.occupancy_density) {
        return Some(("occupancy_density", "An occupancy density is a finite number of persons per square metre, zero or more."));
    }
    if !setpoint(conditions.heating_setpoint) {
        return Some(("heating_setpoint", "A heating set point is a temperature from -20 to 50 degrees Celsius."));
    }
    if !setpoint(conditions.cooling_setpoint) {
        return Some(("cooling_setpoint", "A cooling set point is a temperature from -20 to 50 degrees Celsius."));
    }
    if let (Some(heating), Some(cooling)) = (conditions.heating_setpoint, conditions.cooling_setpoint) {
        if cooling < heating {
            return Some(("cooling_setpoint", "The cooling set point is not below the heating set point."));
        }
    }
    if !non_negative(conditions.ventilation_rate) {
        return Some(("ventilation_rate", "A ventilation rate is a finite flow per square metre, zero or more."));
    }
    if !non_negative(conditions.lighting_power_density) {
        return Some(("lighting_power_density", "A lighting power density is a finite power per square metre, zero or more."));
    }
    if !non_negative(conditions.equipment_power_density) {
        return Some(("equipment_power_density", "An equipment power density is a finite power per square metre, zero or more."));
    }
    None
}

fn u_value_ok(value: Option<f64>) -> bool {
    value.is_none_or(|value| value.is_finite() && value > 0.0 && value <= MAX_U_VALUE)
}

fn glazing_problem(u_value: Option<f64>, g_value: Option<f64>, frame_fraction: Option<f64>) -> Option<Problem> {
    if !u_value_ok(u_value) {
        return Some(("u_value", "A thermal transmittance is a positive number up to 10 watts per square metre and kelvin."));
    }
    if g_value.is_some_and(|value| !(value.is_finite() && (0.0..=1.0).contains(&value))) {
        return Some(("g_value", "A solar energy transmittance is a number from 0 to 1."));
    }
    if frame_fraction.is_some_and(|value| !(value.is_finite() && (0.0..1.0).contains(&value))) {
        return Some(("frame_fraction", "A frame fraction is a number from 0 up to, not including, 1."));
    }
    None
}

/// 🩺️ Why the thermal data of a window type cannot be written, `None` when they can.
pub fn window_thermal_problem(window: &WindowType) -> Option<Problem> {
    glazing_problem(window.u_value, window.g_value, window.frame_fraction)
}

/// 🩺️ Why the thermal data of a curtain wall type cannot be written, `None` when they can.
pub fn curtain_thermal_problem(curtain: &CurtainWallType) -> Option<Problem> {
    glazing_problem(curtain.u_value, curtain.g_value, curtain.frame_fraction)
}

/// 🩺️ Why the thermal data of a door type cannot be written, `None` when they can.
pub fn door_thermal_problem(door: &DoorType) -> Option<Problem> {
    (!u_value_ok(door.u_value)).then_some(("u_value", "A thermal transmittance is a positive number up to 10 watts per square metre and kelvin."))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
