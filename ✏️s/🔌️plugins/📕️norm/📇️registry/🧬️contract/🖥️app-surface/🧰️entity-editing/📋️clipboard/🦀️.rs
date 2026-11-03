//! 📋️ Copy, paste and duplicate of entities: rows are copied by id, pasted with fresh ids that are unique in the whole document, and references among the copied rows follow their new ids. See [`super`].

use super::*;
use semio_s_plugin_norm_structural_scene::projection_kit::PLACEMENTS_FIELD;
use std::collections::BTreeSet;

/// 📏️ How far (m) a copy sits from its original along `x` and `y`, so repeated duplicates and pastes never stack exactly.
pub const PASTE_STEP: f64 = 0.5;

/// 📋️ What a copy holds: the rows of one collection in document order and the persisted placement rows of those rows, so a paste into another document of the same code keeps their pose.
#[derive(Clone, Debug, PartialEq)]
pub struct Clip {
    pub collection: String,
    pub rows: Vec<DslValue>,
    pub poses: Vec<DslValue>,
}

/// 📥️ The edits of a paste and the ids the pasted rows got, in clip order.
#[derive(Clone, Debug, PartialEq)]
pub struct Pasted {
    pub edits: Vec<NormEdit>,
    pub ids: Vec<String>,
}

/// 🔢️ A numeric top-level field shifted on every pasted row so copies do not sit exactly on their originals.
#[derive(Clone, Debug, PartialEq)]
pub struct PasteOffset {
    pub field: String,
    pub delta: f64,
}

/// 📏️ The offsets every duplicate and paste applies: [`PASTE_STEP`] along the position fields `x` and `y` of the rows and of their placement rows.
pub fn paste_offsets() -> Vec<PasteOffset> {
    ["x", "y"].map(|field| PasteOffset { field: field.to_string(), delta: PASTE_STEP }).to_vec()
}

fn rows_of<'a>(tree: &'a DslValue, collection: &str) -> Result<&'a [DslValue], String> {
    get_value_at_path(tree, collection)?.as_array().ok_or_else(|| format!("{collection} is not a list"))
}

fn collect_ids(value: &DslValue, into: &mut BTreeSet<String>) {
    match value {
        DslValue::Array(items) => items.iter().for_each(|item| collect_ids(item, into)),
        DslValue::Object(entries) => {
            for (key, item) in entries {
                match item {
                    DslValue::String(id) if key == "id" => {
                        into.insert(id.clone());
                    }
                    other => collect_ids(other, into),
                }
            }
        }
        _ => {}
    }
}

fn stem(id: &str) -> &str {
    let base = id.rsplit_once("-copy-").filter(|(_, n)| n.chars().all(|c| c.is_ascii_digit()) && !n.is_empty()).map_or(id, |(base, _)| base);
    base.strip_suffix("-copy").unwrap_or(base)
}

/// 🆔️ The next free id for a copy of `id`: `a` becomes `a-copy`, `a-copy-2`, ... and a copy of a copy stays at that stem; ids in `taken` are never returned.
pub fn fresh_id(taken: &BTreeSet<String>, id: &str) -> String {
    let base = stem(id);
    std::iter::once(format!("{base}-copy")).chain((2..).map(|n| format!("{base}-copy-{n}"))).find(|candidate| !taken.contains(candidate)).unwrap_or_default()
}

/// 📋️ Copies the rows of `collection` with the given ids, in document order; an unknown id or an empty id list is an error.
pub fn copy_rows(tree: &DslValue, collection: &str, ids: &[String]) -> Result<Clip, String> {
    let rows: Vec<DslValue> = rows_of(tree, collection)?.iter().filter(|row| object_id_field(row).is_some_and(|id| ids.iter().any(|wanted| wanted == id))).cloned().collect();
    if rows.is_empty() {
        return Err(format!("nothing to copy in {collection}"));
    }
    if let Some(missing) = ids.iter().find(|id| !rows.iter().any(|row| object_id_field(row) == Some(id.as_str()))) {
        return Err(format!("unknown id '{missing}' in {collection}"));
    }
    let poses: Vec<DslValue> = if collection == PLACEMENTS_FIELD {
        Vec::new()
    } else {
        rows_of(tree, PLACEMENTS_FIELD).map(|poses| poses.iter().filter(|pose| object_id_field(pose).is_some_and(|id| rows.iter().any(|row| object_id_field(row) == Some(id)))).cloned().collect()).unwrap_or_default()
    };
    Ok(Clip { collection: collection.to_string(), rows, poses })
}

fn rewrite(value: &mut DslValue, renamed: &[(String, String)], top: bool) {
    match value {
        DslValue::String(text) => {
            if let Some((_, new)) = renamed.iter().find(|(old, _)| old == text) {
                *text = new.clone();
            }
        }
        DslValue::Array(items) => items.iter_mut().for_each(|item| rewrite(item, renamed, false)),
        DslValue::Object(entries) => {
            for (key, item) in entries.iter_mut() {
                if top && key == "id" {
                    continue;
                }
                rewrite(item, renamed, false);
            }
        }
        _ => {}
    }
}

fn shifted(row: &mut DslValue, offsets: &[PasteOffset]) {
    let DslValue::Object(entries) = row else { return };
    for offset in offsets {
        if let Some((_, value)) = entries.iter_mut().find(|(key, value)| *key == offset.field && value.as_f64().is_some()) {
            *value = DslValue::float(value.as_f64().unwrap_or_default() + offset.delta);
        }
    }
}

/// 📥️ The paste of a clip at `at` (end of the list when `None`, clamped): every row gets a document-unique fresh id, references among the pasted rows follow, `offsets` shift the named numeric fields. The inserts are listed in reverse and all at `at`, so each one is valid against the unchanged document and applying them in order leaves the rows in clip order.
pub fn paste_edits(tree: &DslValue, clip: &Clip, at: Option<usize>, offsets: &[PasteOffset]) -> Result<Pasted, String> {
    let existing = rows_of(tree, &clip.collection)?;
    let at = at.map_or(existing.len(), |index| index.min(existing.len()));
    let mut taken = BTreeSet::new();
    collect_ids(tree, &mut taken);
    let mut renamed = Vec::new();
    for row in &clip.rows {
        let old = object_id_field(row).ok_or_else(|| format!("a row of {} has no id", clip.collection))?.to_string();
        let new = fresh_id(&taken, &old);
        validate_path_element_id(&new)?;
        taken.insert(new.clone());
        renamed.push((old, new));
    }
    let mut rows: Vec<DslValue> = clip.rows.clone();
    for (row, (_, new)) in rows.iter_mut().zip(&renamed) {
        rewrite(row, &renamed, true);
        if let DslValue::Object(entries) = row {
            entries.iter_mut().filter(|(key, _)| key == "id").for_each(|(_, value)| *value = DslValue::String(new.clone()));
        }
        shifted(row, offsets);
    }
    let mut edits: Vec<NormEdit> = rows.into_iter().rev().map(|row| NormEdit::InsertItem { path: clip.collection.clone(), index: at, value: Some(row) }).collect();
    if let Ok(poses) = rows_of(tree, PLACEMENTS_FIELD) {
        let pasted = clip.poses.iter().filter_map(|pose| {
            let (_, new) = renamed.iter().find(|(old, _)| object_id_field(pose) == Some(old.as_str()))?;
            let mut pose = pose.clone();
            if let DslValue::Object(entries) = &mut pose {
                entries.iter_mut().filter(|(key, _)| key == "id").for_each(|(_, value)| *value = DslValue::String(new.clone()));
            }
            shifted(&mut pose, offsets);
            Some(pose)
        });
        edits.extend(pasted.enumerate().map(|(n, pose)| NormEdit::InsertItem { path: PLACEMENTS_FIELD.to_string(), index: poses.len() + n, value: Some(pose) }));
    }
    Ok(Pasted { edits, ids: renamed.into_iter().map(|(_, new)| new).collect() })
}

/// 📑️ Duplicate: copy and paste in one step, the copies right after the last original.
pub fn duplicate_edits(tree: &DslValue, collection: &str, ids: &[String], offsets: &[PasteOffset]) -> Result<Pasted, String> {
    let clip = copy_rows(tree, collection, ids)?;
    let last = rows_of(tree, collection)?.iter().rposition(|row| object_id_field(row).is_some_and(|id| ids.iter().any(|wanted| wanted == id))).map_or(0, |index| index + 1);
    paste_edits(tree, &clip, Some(last), offsets)
}

/// 🔡️ The text form of a clip, `{collection, rows, placements}`, which the host keeps as its clipboard and hands back to `pasteEntities`.
pub fn clip_to_json(clip: &Clip) -> String {
    semio_framework_pack_json::to_json_string(&DslValue::object(vec![("collection".to_string(), DslValue::String(clip.collection.clone())), ("rows".to_string(), DslValue::Array(clip.rows.clone())), ("placements".to_string(), DslValue::Array(clip.poses.clone()))]))
}

/// 🔡️ Reads a clip text back; a clip needs its collection and a non-empty `rows` array.
pub fn clip_from_json(json: &str) -> Result<Clip, String> {
    let value = semio_framework_pack_json::from_json_str::<DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let collection = value.get("collection").and_then(DslValue::as_str).filter(|collection| !collection.is_empty()).ok_or("a clip needs its 'collection'")?.to_string();
    let rows = value.get("rows").and_then(DslValue::as_array).filter(|rows| !rows.is_empty()).ok_or("a clip needs a non-empty 'rows' array")?.to_vec();
    let poses = value.get("placements").and_then(DslValue::as_array).map(<[DslValue]>::to_vec).unwrap_or_default();
    Ok(Clip { collection, rows, poses })
}

/// 📋️ The clipboard text of a copy: [`copy_rows`] as [`clip_to_json`].
pub fn copy_clip_json(tree: &DslValue, collection: &str, ids: &[String]) -> Result<String, String> {
    copy_rows(tree, collection, ids).map(|clip| clip_to_json(&clip))
}
