//! 🧭️ Architect trace window — the document-owned audit event feed.

use crate::standards::v1::subsets::any::schema::inferences::audit_trail;
use crate::ProgramSnapshot;
use dsl::ToValue;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, SurfaceKind, TreeWindows, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_scene::EventFeedScene;

//#region 🔖️Constants
pub const ARCHITECT_WINDOW_TRACE: &str = "architect-trace";
pub const ARCHITECT_BODY_TRACE: &str = "architect.trace";
const ARCHITECT_SURFACE_TRACE: &str = "architect.trace";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_WINDOW_TRACE.into(),
        label: LocalizedLabel::native("Trace", "Nachverfolgung"),
        body_key: ARCHITECT_BODY_TRACE.into(),
        surface_kind: SurfaceKind::EventFeed,
        icon_id: "file-code".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Projection
fn rfc3339_utc_epoch_ms(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    if bytes.len() < 20 || bytes[4] != b'-' || bytes[7] != b'-' || bytes[10] != b'T' || bytes[13] != b':' || bytes[16] != b':' || bytes.last() != Some(&b'Z') {
        return None;
    }
    let digits = |range: std::ops::Range<usize>| bytes.get(range.clone()).filter(|part| part.iter().all(u8::is_ascii_digit)).and_then(|_| value.get(range)?.parse::<i64>().ok());
    let (year, month, day, hour, minute, second) = (digits(0..4)?, digits(5..7)?, digits(8..10)?, digits(11..13)?, digits(14..16)?, digits(17..19)?);
    let fraction = bytes.get(19..bytes.len() - 1)?;
    if !(fraction.is_empty() || (fraction.len() > 1 && fraction[0] == b'.' && fraction[1..].iter().all(u8::is_ascii_digit))) {
        return None;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_days = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if !(1..=12).contains(&month) || day < 1 || day > month_days[(month - 1) as usize] || hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let shifted = if month <= 2 { year - 1 } else { year };
    let era = shifted.div_euclid(400);
    let year_of_era = shifted.rem_euclid(400);
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    let fraction_ms = fraction.get(1..).unwrap_or_default().iter().take(3).fold((0_i64, 100_i64), |(value, place), digit| (value + i64::from(*digit - b'0') * place, place / 10)).0;
    (((days * 24 + hour) * 60 + minute) * 60 + second).checked_mul(1_000)?.checked_add(fraction_ms)
}

fn event_feed_entries(program: &ProgramSnapshot) -> dsl::DslValue {
    dsl::DslValue::Array(
        audit_trail(program, None)
            .events
            .into_iter()
            .map(|event| {
                let mut entry = vec![
                    ("id".into(), event.header.id.0.to_value()),
                    ("timestampMs".into(), rfc3339_utc_epoch_ms(&event.timestamp).unwrap_or(0).to_value()),
                    ("iconId".into(), "bell".to_value()),
                    ("title".into(), event.header.name.to_value()),
                    ("tone".into(), if event.success { "success" } else { "error" }.to_value()),
                ];
                if !event.details.text.is_empty() {
                    entry.push(("detail".into(), event.details.text.to_value()));
                }
                dsl::DslValue::Object(entry)
            })
            .collect(),
    )
}
//#endregion 🔖️Projection

//#region 🔖️Render
/// 📰️ Projects the document-wide audit trail as the neutral EventFeed scene used by every renderer.
pub fn render(program: &ProgramSnapshot, _windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let scene = EventFeedScene { entries_json: dsl::json::to_json_string(&event_feed_entries(program)), follow: Some(true), activate_action: None, domain_id: None };
    scene_surface(ARCHITECT_SURFACE_TRACE, SurfaceKind::EventFeed, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
