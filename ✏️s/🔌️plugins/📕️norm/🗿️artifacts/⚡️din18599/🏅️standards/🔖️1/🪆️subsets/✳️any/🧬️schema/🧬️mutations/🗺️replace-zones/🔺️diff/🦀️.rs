//! 🔺️ `replace-zones` sparse diff.

use crate::mutations::replace_zones::ReplaceZones;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599ZonesRows, Din18599ZonesPatch};

pub fn diff(payload: &ReplaceZones, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if base.zones == payload.new_zones {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "zones already has this value.");
    }
    if let Some((_, row)) = payload.new_zones.iter().enumerate().find(|(at, row)| payload.new_zones[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Zone id {} appears twice.", row.id), [row.id.clone()]);
    }
    let removed: Vec<String> = base.zones.iter().filter(|row| !payload.new_zones.iter().any(|next| next.id == row.id)).map(|row| row.id.clone()).collect();
    let added: Vec<_> = payload.new_zones.iter().filter(|next| !base.zones.iter().any(|row| row.id == next.id)).cloned().collect();
    let modified: Vec<Din18599ZonesPatch> = payload.new_zones.iter().filter_map(|next| {
        let row = base.zones.iter().find(|row| row.id == next.id)?;
        (row != next).then(|| Din18599ZonesPatch {
            id: next.id.clone(),
            label_en: (row.label_en != next.label_en).then(|| next.label_en.clone()),
            label_de: (row.label_de != next.label_de).then(|| next.label_de.clone()),
            usage_profile: (row.usage_profile != next.usage_profile).then(|| next.usage_profile.clone()),
            area_m2: (row.area_m2 != next.area_m2).then(|| next.area_m2.clone()),
            volume_m3: (row.volume_m3 != next.volume_m3).then(|| next.volume_m3.clone()),
            theta_i_heat_c: (row.theta_i_heat_c != next.theta_i_heat_c).then(|| next.theta_i_heat_c.clone()),
            theta_i_cool_c: (row.theta_i_cool_c != next.theta_i_cool_c).then(|| next.theta_i_cool_c.clone()),
            occupants: (row.occupants != next.occupants).then(|| next.occupants.clone()),
            internal_gains_w_m2: (row.internal_gains_w_m2 != next.internal_gains_w_m2).then(|| next.internal_gains_w_m2.clone()),
            lighting_power_w_m2: (row.lighting_power_w_m2 != next.lighting_power_w_m2).then(|| next.lighting_power_w_m2.clone()),
        })
    }).collect();
    let mut natural: Vec<String> = base.zones.iter().filter(|row| !removed.contains(&row.id)).map(|row| row.id.clone()).collect();
    natural.extend(added.iter().map(|row| row.id.clone()));
    let wanted: Vec<String> = payload.new_zones.iter().map(|row| row.id.clone()).collect();
    let order = (natural != wanted).then_some(wanted);
    protocol::MutationOutcome::new(Din18599Diff { zones: Some(Din18599ZonesRows { added, removed, modified, order }), ..Default::default() })
}
