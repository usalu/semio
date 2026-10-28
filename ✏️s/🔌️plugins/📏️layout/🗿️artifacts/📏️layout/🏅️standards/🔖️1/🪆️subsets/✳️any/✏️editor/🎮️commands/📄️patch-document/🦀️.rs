//! ✏️ Layout play app command — `patch-document`.

use semio_s_artifact_layout_layout_mutations_content::change_data_fields::ChangeDataFields;
use semio_s_artifact_layout_layout_mutations_content::change_print_target::ChangePrintTarget;
use semio_s_artifact_layout_layout_mutations_content::rename_layout::RenameLayout;
use semio_s_artifact_layout_layout_mutations_frames::update_grid::UpdateGrid;
use semio_s_artifact_layout_layout_mutations_content::create_character_style::CreateCharacterStyle;
use semio_s_artifact_layout_layout_mutations_content::delete_character_style::DeleteCharacterStyle;
use semio_s_artifact_layout_layout_mutations_content::update_character_style::UpdateCharacterStyle;
use semio_s_artifact_layout_layout_mutations_content::update_paragraph_style::UpdateParagraphStyle;
use semio_s_artifact_layout_layout_mutations_content::update_parent_page::UpdateParentPage;
use semio_s_artifact_layout_layout_mutations_content::update_spread::UpdateSpread;
use semio_s_artifact_layout_layout_model::text_to_rgba;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "🧩️structure/🦀️.rs"]
mod structure;
#[path = "🧵️text-chunks/🦀️.rs"]
pub mod text_chunks;
#[path = "🧾️data-fields/🦀️.rs"]
mod data_fields;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-document")]
pub struct PatchDocument {
    pub field: String,
    pub value: String,
}

fn present(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn flag(value: &str) -> bool {
    matches!(value.trim(), "true" | "1")
}

fn split_field(field: &str) -> Option<(&str, &str)> {
    field.rsplit_once('.')
}

fn detail_mutation(document: &LayoutSnapshot, field: &str, value: &str) -> Option<LayoutMutation> {
    if let Some(mutation) = structure::mutation(document, field, value).or_else(|| data_fields::mutation(document, field, value)) { return Some(mutation) }
    if field == "addCharacterStyle" {
        let mut index = document.character_styles.len() + 1;
        let id = loop {
            let candidate = format!("character-{index}");
            if document.character_styles.iter().all(|style| style.id != candidate) {
                break candidate;
            }
            index += 1;
        };
        let name = value.trim();
        return Some(LayoutMutation::CreateCharacterStyle(CreateCharacterStyle { id, name: (!name.is_empty()).then(|| name.to_string()), index: None }));
    }
    let (id, key) = split_field(field)?;
    if let Some(style) = document.paragraph_styles.iter().find(|style| style.id == id) {
        if !matches!(key, "name" | "fontFamily" | "fontSize" | "fontWeight" | "leading" | "tracking" | "alignment") {
            return None;
        }
        let mut next = style.clone();
        match key {
            "name" => next.name = value.to_string(),
            "fontFamily" => next.font_family = value.to_string(),
            "fontSize" => next.font_size = value.parse().ok()?,
            "fontWeight" => next.font_weight = value.parse::<f64>().ok()?.max(0.0) as u32,
            "leading" => next.leading = value.parse().ok()?,
            "tracking" => next.tracking = value.parse().ok()?,
            "alignment" => next.alignment = value.to_string(),
            _ => return None,
        }
        return Some(LayoutMutation::UpdateParagraphStyle(UpdateParagraphStyle { id: next.id, name: next.name, font_family: next.font_family, font_size: next.font_size, font_weight: next.font_weight, leading: next.leading, tracking: next.tracking, alignment: next.alignment }));
    }
    if key == "delete" && document.character_styles.iter().any(|style| style.id == id) {
        return Some(LayoutMutation::DeleteCharacterStyle(DeleteCharacterStyle { id: id.to_string() }));
    }
    if let Some(style) = document.character_styles.iter().find(|style| style.id == id) {
        if !matches!(key, "name" | "fontFamily" | "fontSize" | "fontWeight" | "italic" | "color" | "colorEnabled" | "colorAlpha" | "tracking") {
            return None;
        }
        let mut next = UpdateCharacterStyle { id: style.id.clone(), name: style.name.clone(), font_family: style.font_family.clone(), font_size: style.font_size, font_weight: style.font_weight, italic: style.italic, color: style.color, tracking: style.tracking };
        match key {
            "name" => next.name = (!value.trim().is_empty()).then(|| value.to_string()),
            "fontFamily" => next.font_family = (!value.trim().is_empty()).then(|| value.to_string()),
            "fontSize" => next.font_size = if value.trim().is_empty() { None } else { Some(value.parse().ok()?) },
            "fontWeight" => next.font_weight = if value.trim().is_empty() { None } else { Some(value.parse::<f64>().ok().map(|weight| weight.max(0.0) as u32)?) },
            "italic" => next.italic = if value.is_empty() { None } else { Some(matches!(value.trim(), "true" | "1")) },
            "color" => {
                let mut color = text_to_rgba(value);
                if let (Some(color), Some(previous)) = (color.as_mut(), style.color) { color[3] = previous[3]; }
                next.color = color;
            }
            "colorEnabled" => next.color = matches!(value, "true" | "1").then(|| style.color.unwrap_or([0.0, 0.0, 0.0, 1.0])),
            "colorAlpha" => {
                let alpha = value.parse::<f32>().ok()?;
                if !alpha.is_finite() { return None }
                let mut color = style.color.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                color[3] = alpha.clamp(0.0, 1.0);
                next.color = Some(color);
            }
            "tracking" => next.tracking = if value.trim().is_empty() { None } else { Some(value.parse().ok()?) },
            _ => return None,
        }
        return Some(LayoutMutation::UpdateCharacterStyle(next));
    }
    if key == "name" {
        if let Some(spread) = document.spreads.iter().find(|spread| spread.id == id) {
            return Some(LayoutMutation::UpdateSpread(UpdateSpread { id: spread.id.clone(), name: value.to_string() }));
        }
    }
    if let Some(parent) = document.parent_pages.iter().find(|parent| parent.id == id) {
        if !matches!(key, "name" | "width" | "height") {
            return None;
        }
        let mut name = parent.name.clone();
        let mut width = parent.width;
        let mut height = parent.height;
        match key {
            "name" => name = value.to_string(),
            "width" => width = value.parse().ok()?,
            "height" => height = value.parse().ok()?,
            _ => return None,
        }
        return Some(LayoutMutation::UpdateParentPage(UpdateParentPage { id: parent.id.clone(), name, width, height }));
    }
    None
}

pub fn handle(payload: &PatchDocument, doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    if payload.field.starts_with("story.") && payload.field.contains(".content.chunk.") { return structure::guarded_text(doc.snapshot, &payload.field, &payload.value) }
    if (payload.field.starts_with("data.") && (payload.field.contains(".value.chunk.") || payload.field.contains(".value.bytes.") || payload.field.contains(".at."))) || payload.field.starts_with("clearDataFields.at.") { return data_fields::guarded(doc.snapshot, &payload.field, &payload.value).map(|mutation| Emit::mutations(vec![mutation])) }
    let grid = &doc.snapshot.grid;
    let mutation = match payload.field.as_str() {
        "name" => LayoutMutation::RenameLayout(RenameLayout { new_name: payload.value.clone() }),
        "printTarget" => LayoutMutation::ChangePrintTarget(ChangePrintTarget { new_print_target: present(&payload.value) }),

        "baselineGrid" => match payload.value.parse::<f64>() {
            Ok(baseline_grid) => LayoutMutation::UpdateGrid(UpdateGrid { baseline_grid, baseline_offset: grid.baseline_offset, snap_to_baseline: grid.snap_to_baseline }),
            Err(_) => return Ok(Emit::default()),
        },
        "baselineOffset" => match payload.value.parse::<f64>() {
            Ok(baseline_offset) => LayoutMutation::UpdateGrid(UpdateGrid { baseline_grid: grid.baseline_grid, baseline_offset, snap_to_baseline: grid.snap_to_baseline }),
            Err(_) => return Ok(Emit::default()),
        },
        "snapToBaseline" => LayoutMutation::UpdateGrid(UpdateGrid { baseline_grid: grid.baseline_grid, baseline_offset: grid.baseline_offset, snap_to_baseline: flag(&payload.value) }),
        _ => match detail_mutation(doc.snapshot, &payload.field, &payload.value) {
            Some(mutation) => mutation,
            None => return Ok(Emit::default()),
        },
    };
    Ok(Emit::mutations(vec![mutation]))
}

