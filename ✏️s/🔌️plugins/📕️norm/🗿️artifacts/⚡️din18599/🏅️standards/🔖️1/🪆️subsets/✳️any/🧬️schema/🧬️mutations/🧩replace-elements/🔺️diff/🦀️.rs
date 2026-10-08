//! 🔺️ `replace-elements` sparse diff.

use crate::mutations::replace_elements::ReplaceElements;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ReplaceElements, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if base.elements == payload.new_elements {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "elements already has this value.");
    }
    if let Some((_, row)) = payload.new_elements.iter().enumerate().find(|(at, row)| payload.new_elements[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Element id {} appears twice.", row.id), [row.id.clone()]);
    }
    let removed: Vec<String> = base.elements.iter().filter(|row| !payload.new_elements.iter().any(|next| next.id == row.id)).map(|row| row.id.clone()).collect();
    let added: Vec<_> = payload.new_elements.iter().filter(|next| !base.elements.iter().any(|row| row.id == next.id)).cloned().collect();
    let modified: Vec<Din18599ElementsPatch> = payload.new_elements.iter().filter_map(|next| {
        let row = base.elements.iter().find(|row| row.id == next.id)?;
        (row != next).then(|| Din18599ElementsPatch {
            id: next.id.clone(),
            label_en: (row.label_en != next.label_en).then(|| next.label_en.clone()),
            label_de: (row.label_de != next.label_de).then(|| next.label_de.clone()),
            kind: (row.kind != next.kind).then(|| next.kind.clone()),
            zone_id: (row.zone_id != next.zone_id).then(|| next.zone_id.clone()),
            area_m2: (row.area_m2 != next.area_m2).then(|| next.area_m2.clone()),
            u_value_w_m2k: (row.u_value_w_m2k != next.u_value_w_m2k).then(|| next.u_value_w_m2k.clone()),
            orientation_deg: (row.orientation_deg != next.orientation_deg).then(|| next.orientation_deg.clone()),
            tilt_deg: (row.tilt_deg != next.tilt_deg).then(|| next.tilt_deg.clone()),
            g_value: (row.g_value != next.g_value).then(|| next.g_value.clone()),
            fc: (row.fc != next.fc).then(|| next.fc.clone()),
            adjacency: (row.adjacency != next.adjacency).then(|| next.adjacency.clone()),
        })
    }).collect();
    let mut natural: Vec<String> = base.elements.iter().filter(|row| !removed.contains(&row.id)).map(|row| row.id.clone()).collect();
    natural.extend(added.iter().map(|row| row.id.clone()));
    let wanted: Vec<String> = payload.new_elements.iter().map(|row| row.id.clone()).collect();
    let order = (natural != wanted).then_some(wanted);
    protocol::MutationOutcome::new(Din18599Diff { elements: Some(Din18599ElementsRows { added, removed, modified, order }), ..Default::default() })
}
