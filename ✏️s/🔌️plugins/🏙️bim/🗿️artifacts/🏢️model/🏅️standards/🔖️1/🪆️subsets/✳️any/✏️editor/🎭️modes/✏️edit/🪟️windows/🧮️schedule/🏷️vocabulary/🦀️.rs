//! 🏷️ The words of a schedule in a locale: the heading of a key (with its unit), the name of a category and of a comparison, and the localized text of a cell whose value is a stable token (kind, phase, swing, leaves).

use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::inferences::schedules::ScheduleCell;
use crate::{Phase, ScheduleCategory, ScheduleColumn, ScheduleField, ScheduleKey, ScheduleOp};

/// 🏷️ The name of a built-in field.
pub fn field_label(labels: &BimLabels, field: ScheduleField) -> String {
    use ScheduleField::*;
    let label = match field {
        Id => labels.sf_id,
        Name => labels.sf_name,
        Kind => labels.sf_kind,
        Storey => labels.sf_storey,
        Level => labels.sf_level,
        Type => labels.sf_type,
        Phase => labels.sf_phase,
        Material => labels.sf_material,
        Host => labels.sf_host,
        Number => labels.sf_number,
        Usage => labels.sf_usage,
        Swing => labels.sf_swing,
        Leaves => labels.sf_leaves,
        Panes => labels.sf_panes,
        Count => labels.sf_count,
        Length => labels.sf_length,
        Width => labels.sf_width,
        Height => labels.sf_height,
        Perimeter => labels.sf_perimeter,
        GrossSideArea => labels.sf_gross_side_area,
        OpeningArea => labels.sf_opening_area,
        NetSideArea => labels.sf_net_side_area,
        GrossArea => labels.sf_gross_area,
        NetArea => labels.sf_net_area,
        SurfaceArea => labels.sf_surface_area,
        GrossVolume => labels.sf_gross_volume,
        NetVolume => labels.sf_net_volume,
        Mass => labels.sf_mass,
        Risers => labels.sf_risers,
        Thickness => labels.sf_thickness,
        LayerArea => labels.sf_layer_area,
        LayerVolume => labels.sf_layer_volume,
        LayerMass => labels.sf_layer_mass,
    };
    label.as_str().to_string()
}

/// 🏷️ The name of a key: the field name, or `set · property`.
pub fn key_label(labels: &BimLabels, key: &ScheduleKey) -> String {
    match key {
        ScheduleKey::Field { field } => field_label(labels, *field),
        ScheduleKey::Property { set, name } => format!("{set} · {name}"),
    }
}

/// 🏷️ The heading of a column: its own heading, else the name of its key with the unit of a measure in parentheses.
pub fn heading(labels: &BimLabels, column: &ScheduleColumn) -> String {
    if let Some(heading) = &column.heading {
        return heading.clone();
    }
    match &column.key {
        ScheduleKey::Field { field } if !field.unit().is_empty() => format!("{} ({})", field_label(labels, *field), field.unit()),
        key => key_label(labels, key),
    }
}

/// 🏷️ The name of a category.
pub fn category_label(labels: &BimLabels, category: ScheduleCategory) -> String {
    use ScheduleCategory::*;
    let label = match category {
        Wall => labels.sc_wall,
        CurtainWall => labels.sc_curtain_wall,
        Slab => labels.sc_slab,
        Roof => labels.sc_roof,
        Column => labels.sc_column,
        Beam => labels.sc_beam,
        Window => labels.sc_window,
        Door => labels.sc_door,
        Void => labels.sc_void,
        Stair => labels.sc_stair,
        Railing => labels.sc_railing,
        Space => labels.sc_space,
        Material => labels.sc_material,
    };
    label.as_str().to_string()
}

/// 🏷️ The words of a comparison.
pub fn op_label(labels: &BimLabels, op: ScheduleOp) -> String {
    let label = match op {
        ScheduleOp::Equals => labels.so_equals,
        ScheduleOp::NotEquals => labels.so_not_equals,
        ScheduleOp::Contains => labels.so_contains,
        ScheduleOp::Greater => labels.so_greater,
        ScheduleOp::GreaterOrEqual => labels.so_greater_or_equal,
        ScheduleOp::Less => labels.so_less,
        ScheduleOp::LessOrEqual => labels.so_less_or_equal,
        ScheduleOp::Empty => labels.so_empty,
        ScheduleOp::NotEmpty => labels.so_not_empty,
    };
    label.as_str().to_string()
}

/// 🏷️ The name of a phase.
pub fn phase_label(labels: &BimLabels, phase: Phase) -> String {
    let label = match phase {
        Phase::Existing => labels.sv_existing,
        Phase::New => labels.sv_new,
        Phase::Demolished => labels.sv_demolished,
        Phase::Temporary => labels.sv_temporary,
    };
    label.as_str().to_string()
}

/// 🔤️ The text a cell shows under `key`: the token of an enumeration in the locale, a number as is, any other text unchanged.
pub fn cell_text(labels: &BimLabels, key: &ScheduleKey, cell: &ScheduleCell) -> String {
    let ScheduleCell::Text { value } = cell else { return cell.display() };
    let ScheduleKey::Field { field } = key else { return value.clone() };
    match (field, value.as_str()) {
        (ScheduleField::Kind, token) => crate::editor::bim::entities::kind_of(entity_kind(token)).map_or_else(|| token.to_string(), |row| (row.label)(labels).as_str().to_string()),
        (ScheduleField::Phase, "existing") => labels.sv_existing.as_str().to_string(),
        (ScheduleField::Phase, "new") => labels.sv_new.as_str().to_string(),
        (ScheduleField::Phase, "demolished") => labels.sv_demolished.as_str().to_string(),
        (ScheduleField::Phase, "temporary") => labels.sv_temporary.as_str().to_string(),
        (ScheduleField::Swing, "left") => labels.sv_left.as_str().to_string(),
        (ScheduleField::Swing, "right") => labels.sv_right.as_str().to_string(),
        (ScheduleField::Leaves, "single") => labels.sv_single.as_str().to_string(),
        (ScheduleField::Leaves, "double") => labels.sv_double.as_str().to_string(),
        _ => value.clone(),
    }
}

/// 🏷️ The entity kind a quantity kind token belongs to: windows, doors and voids are all openings.
fn entity_kind(token: &str) -> &str {
    match token {
        "window" | "door" | "void" => "opening",
        other => other,
    }
}
