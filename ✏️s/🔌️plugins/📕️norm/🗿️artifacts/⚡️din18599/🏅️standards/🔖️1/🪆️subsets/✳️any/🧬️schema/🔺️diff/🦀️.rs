//! 🧬️ Din18599 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff. `DiffAlgebra::between` covers the modelled vocabulary only and exists for sync tooling, never for mutation leaves.

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("diff.target-missing", format!("{what} does not exist"))
}

/// 🩹️ Sparse patch of the `zones` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599ZonesPatch {
    pub id: String,
    pub label_en: Option<String>,
    pub label_de: Option<String>,
    pub usage_profile: Option<crate::UsageProfile>,
    pub area_m2: Option<f64>,
    pub volume_m3: Option<f64>,
    pub theta_i_heat_c: Option<f64>,
    pub theta_i_cool_c: Option<f64>,
    pub occupants: Option<u32>,
    pub internal_gains_w_m2: Option<f64>,
    pub lighting_power_w_m2: Option<f64>,
}

impl Din18599ZonesPatch {
    fn apply_to_row(&self, row: &mut crate::ThermalZone) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.label_en {
            row.label_en = value.clone();
        }
        if let Some(value) = &self.label_de {
            row.label_de = value.clone();
        }
        if let Some(value) = &self.usage_profile {
            row.usage_profile = value.clone();
        }
        if let Some(value) = &self.area_m2 {
            row.area_m2 = value.clone();
        }
        if let Some(value) = &self.volume_m3 {
            row.volume_m3 = value.clone();
        }
        if let Some(value) = &self.theta_i_heat_c {
            row.theta_i_heat_c = value.clone();
        }
        if let Some(value) = &self.theta_i_cool_c {
            row.theta_i_cool_c = value.clone();
        }
        if let Some(value) = &self.occupants {
            row.occupants = value.clone();
        }
        if let Some(value) = &self.internal_gains_w_m2 {
            row.internal_gains_w_m2 = value.clone();
        }
        if let Some(value) = &self.lighting_power_w_m2 {
            row.lighting_power_w_m2 = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.label_en.is_some() {
            self.label_en = other.label_en;
        }
        if other.label_de.is_some() {
            self.label_de = other.label_de;
        }
        if other.usage_profile.is_some() {
            self.usage_profile = other.usage_profile;
        }
        if other.area_m2.is_some() {
            self.area_m2 = other.area_m2;
        }
        if other.volume_m3.is_some() {
            self.volume_m3 = other.volume_m3;
        }
        if other.theta_i_heat_c.is_some() {
            self.theta_i_heat_c = other.theta_i_heat_c;
        }
        if other.theta_i_cool_c.is_some() {
            self.theta_i_cool_c = other.theta_i_cool_c;
        }
        if other.occupants.is_some() {
            self.occupants = other.occupants;
        }
        if other.internal_gains_w_m2.is_some() {
            self.internal_gains_w_m2 = other.internal_gains_w_m2;
        }
        if other.lighting_power_w_m2.is_some() {
            self.lighting_power_w_m2 = other.lighting_power_w_m2;
        }
    }

    fn inverse_from_row(&self, row: &crate::ThermalZone) -> Self {
        Self {
            id: self.id.clone(),
            label_en: self.label_en.as_ref().map(|_| row.label_en.clone()),
            label_de: self.label_de.as_ref().map(|_| row.label_de.clone()),
            usage_profile: self.usage_profile.as_ref().map(|_| row.usage_profile.clone()),
            area_m2: self.area_m2.as_ref().map(|_| row.area_m2.clone()),
            volume_m3: self.volume_m3.as_ref().map(|_| row.volume_m3.clone()),
            theta_i_heat_c: self.theta_i_heat_c.as_ref().map(|_| row.theta_i_heat_c.clone()),
            theta_i_cool_c: self.theta_i_cool_c.as_ref().map(|_| row.theta_i_cool_c.clone()),
            occupants: self.occupants.as_ref().map(|_| row.occupants.clone()),
            internal_gains_w_m2: self.internal_gains_w_m2.as_ref().map(|_| row.internal_gains_w_m2.clone()),
            lighting_power_w_m2: self.lighting_power_w_m2.as_ref().map(|_| row.lighting_power_w_m2.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `zones` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599ZonesRows {
    pub added: Vec<crate::ThermalZone>,
    pub removed: Vec<String>,
    pub modified: Vec<Din18599ZonesPatch>,
    pub order: Option<Vec<String>>,
}

impl Din18599ZonesRows {
    fn key(row: &crate::ThermalZone) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::ThermalZone]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::ThermalZone]) -> Result<Vec<crate::ThermalZone>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for id in &self.removed {
            let at = rows.iter().position(|row| Self::key(row) == id).ok_or_else(|| missing_target(format!("removed row \"{id}\"")).at([id.clone()]))?;
            rows.remove(at);
        }
        for patch in &self.modified {
            let row = rows.iter_mut().find(|row| Self::key(row) == patch.id).ok_or_else(|| missing_target(format!("modified row \"{}\"", patch.id)).at([patch.id.clone()]))?;
            patch.apply_to_row(row)?;
        }
        for row in &self.added {
            if rows.iter().any(|existing| Self::key(existing) == Self::key(row)) {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("row \"{}\" already exists", Self::key(row))).at([Self::key(row).to_string()]));
            }
            rows.push(row.clone());
        }
        if let Some(order) = &self.order {
            if order.len() != rows.len() {
                return Err(protocol::MutationApplyError::new("diff.order-mismatch", "order must list every row exactly once"));
            }
            let mut pool = rows;
            let mut ordered = Vec::with_capacity(pool.len());
            for id in order {
                let at = pool.iter().position(|row| Self::key(row) == id).ok_or_else(|| protocol::MutationApplyError::new("diff.order-mismatch", format!("order names unknown row \"{id}\"")).at([id.clone()]))?;
                ordered.push(pool.remove(at));
            }
            rows = ordered;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        let other_removed = other.removed.clone();
        let other_added_ids: Vec<String> = other.added.iter().map(|row| Self::key(row).to_string()).collect();
        for id in other.removed {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == id) {
                self.added.remove(at);
            } else {
                self.modified.retain(|patch| patch.id != id);
                if !self.removed.contains(&id) {
                    self.removed.push(id);
                }
            }
        }
        self.added.extend(other.added);
        for patch in other.modified {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == patch.id) {
                if patch.apply_to_row(&mut self.added[at]).is_err() {
                    self.modified.push(patch);
                }
            } else if let Some(existing) = self.modified.iter_mut().find(|existing| existing.id == patch.id) {
                existing.merge(patch);
            } else {
                self.modified.push(patch);
            }
        }
        self.order = match (other.order, self.order.take()) {
            (Some(order), _) => Some(order),
            (None, Some(mut order)) => {
                order.retain(|id| !other_removed.contains(id));
                order.extend(other_added_ids);
                Some(order)
            }
            (None, None) => None,
        };
    }

    fn inverse_rows(&self, base: &[crate::ThermalZone]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|row| Self::key(row).to_string()).collect();
        inverse.added = base.iter().filter(|row| self.removed.iter().any(|id| id == Self::key(row))).cloned().collect();
        inverse.modified = self.modified.iter().filter_map(|patch| base.iter().find(|row| Self::key(row) == patch.id).map(|row| patch.inverse_from_row(row))).collect();
        let base_ids = Self::ids(base);
        let mut after_ids: Vec<String> = base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().collect();
        after_ids.extend(self.added.iter().map(|row| Self::key(row).to_string()));
        if let Some(order) = &self.order {
            after_ids = order.clone();
        }
        let mut natural: Vec<String> = after_ids.into_iter().filter(|id| !inverse.removed.contains(id)).collect();
        natural.extend(inverse.added.iter().map(|row| Self::key(row).to_string()));
        if natural != base_ids {
            inverse.order = Some(base_ids);
        }
        inverse
    }

    fn between_rows(base: &[crate::ThermalZone], other: &[crate::ThermalZone]) -> Self {
        let mut diff = Self::default();
        diff.removed = base.iter().filter(|row| other.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).map(|row| Self::key(row).to_string()).collect();
        diff.added = other.iter().filter(|row| base.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).cloned().collect();
        let mut natural: Vec<String> = Self::ids(base).into_iter().filter(|id| !diff.removed.contains(id)).collect();
        natural.extend(diff.added.iter().map(|row| Self::key(row).to_string()));
        let wanted = Self::ids(other);
        if natural != wanted {
            diff.order = Some(wanted);
        }
        diff
    }
}

/// 🩹️ Sparse patch of the `elements` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599ElementsPatch {
    pub id: String,
    pub label_en: Option<String>,
    pub label_de: Option<String>,
    pub kind: Option<crate::ElementKind>,
    pub zone_id: Option<String>,
    pub area_m2: Option<f64>,
    pub u_value_w_m2k: Option<f64>,
    pub orientation_deg: Option<f64>,
    pub tilt_deg: Option<f64>,
    pub g_value: Option<f64>,
    pub fc: Option<f64>,
    pub adjacency: Option<crate::Adjacency>,
}

impl Din18599ElementsPatch {
    fn apply_to_row(&self, row: &mut crate::EnvelopeElement) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.label_en {
            row.label_en = value.clone();
        }
        if let Some(value) = &self.label_de {
            row.label_de = value.clone();
        }
        if let Some(value) = &self.kind {
            row.kind = value.clone();
        }
        if let Some(value) = &self.zone_id {
            row.zone_id = value.clone();
        }
        if let Some(value) = &self.area_m2 {
            row.area_m2 = value.clone();
        }
        if let Some(value) = &self.u_value_w_m2k {
            row.u_value_w_m2k = value.clone();
        }
        if let Some(value) = &self.orientation_deg {
            row.orientation_deg = value.clone();
        }
        if let Some(value) = &self.tilt_deg {
            row.tilt_deg = value.clone();
        }
        if let Some(value) = &self.g_value {
            row.g_value = value.clone();
        }
        if let Some(value) = &self.fc {
            row.fc = value.clone();
        }
        if let Some(value) = &self.adjacency {
            row.adjacency = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.label_en.is_some() {
            self.label_en = other.label_en;
        }
        if other.label_de.is_some() {
            self.label_de = other.label_de;
        }
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if other.zone_id.is_some() {
            self.zone_id = other.zone_id;
        }
        if other.area_m2.is_some() {
            self.area_m2 = other.area_m2;
        }
        if other.u_value_w_m2k.is_some() {
            self.u_value_w_m2k = other.u_value_w_m2k;
        }
        if other.orientation_deg.is_some() {
            self.orientation_deg = other.orientation_deg;
        }
        if other.tilt_deg.is_some() {
            self.tilt_deg = other.tilt_deg;
        }
        if other.g_value.is_some() {
            self.g_value = other.g_value;
        }
        if other.fc.is_some() {
            self.fc = other.fc;
        }
        if other.adjacency.is_some() {
            self.adjacency = other.adjacency;
        }
    }

    fn inverse_from_row(&self, row: &crate::EnvelopeElement) -> Self {
        Self {
            id: self.id.clone(),
            label_en: self.label_en.as_ref().map(|_| row.label_en.clone()),
            label_de: self.label_de.as_ref().map(|_| row.label_de.clone()),
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            zone_id: self.zone_id.as_ref().map(|_| row.zone_id.clone()),
            area_m2: self.area_m2.as_ref().map(|_| row.area_m2.clone()),
            u_value_w_m2k: self.u_value_w_m2k.as_ref().map(|_| row.u_value_w_m2k.clone()),
            orientation_deg: self.orientation_deg.as_ref().map(|_| row.orientation_deg.clone()),
            tilt_deg: self.tilt_deg.as_ref().map(|_| row.tilt_deg.clone()),
            g_value: self.g_value.as_ref().map(|_| row.g_value.clone()),
            fc: self.fc.as_ref().map(|_| row.fc.clone()),
            adjacency: self.adjacency.as_ref().map(|_| row.adjacency.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `elements` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599ElementsRows {
    pub added: Vec<crate::EnvelopeElement>,
    pub removed: Vec<String>,
    pub modified: Vec<Din18599ElementsPatch>,
    pub order: Option<Vec<String>>,
}

impl Din18599ElementsRows {
    fn key(row: &crate::EnvelopeElement) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::EnvelopeElement]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::EnvelopeElement]) -> Result<Vec<crate::EnvelopeElement>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for id in &self.removed {
            let at = rows.iter().position(|row| Self::key(row) == id).ok_or_else(|| missing_target(format!("removed row \"{id}\"")).at([id.clone()]))?;
            rows.remove(at);
        }
        for patch in &self.modified {
            let row = rows.iter_mut().find(|row| Self::key(row) == patch.id).ok_or_else(|| missing_target(format!("modified row \"{}\"", patch.id)).at([patch.id.clone()]))?;
            patch.apply_to_row(row)?;
        }
        for row in &self.added {
            if rows.iter().any(|existing| Self::key(existing) == Self::key(row)) {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("row \"{}\" already exists", Self::key(row))).at([Self::key(row).to_string()]));
            }
            rows.push(row.clone());
        }
        if let Some(order) = &self.order {
            if order.len() != rows.len() {
                return Err(protocol::MutationApplyError::new("diff.order-mismatch", "order must list every row exactly once"));
            }
            let mut pool = rows;
            let mut ordered = Vec::with_capacity(pool.len());
            for id in order {
                let at = pool.iter().position(|row| Self::key(row) == id).ok_or_else(|| protocol::MutationApplyError::new("diff.order-mismatch", format!("order names unknown row \"{id}\"")).at([id.clone()]))?;
                ordered.push(pool.remove(at));
            }
            rows = ordered;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        let other_removed = other.removed.clone();
        let other_added_ids: Vec<String> = other.added.iter().map(|row| Self::key(row).to_string()).collect();
        for id in other.removed {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == id) {
                self.added.remove(at);
            } else {
                self.modified.retain(|patch| patch.id != id);
                if !self.removed.contains(&id) {
                    self.removed.push(id);
                }
            }
        }
        self.added.extend(other.added);
        for patch in other.modified {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == patch.id) {
                if patch.apply_to_row(&mut self.added[at]).is_err() {
                    self.modified.push(patch);
                }
            } else if let Some(existing) = self.modified.iter_mut().find(|existing| existing.id == patch.id) {
                existing.merge(patch);
            } else {
                self.modified.push(patch);
            }
        }
        self.order = match (other.order, self.order.take()) {
            (Some(order), _) => Some(order),
            (None, Some(mut order)) => {
                order.retain(|id| !other_removed.contains(id));
                order.extend(other_added_ids);
                Some(order)
            }
            (None, None) => None,
        };
    }

    fn inverse_rows(&self, base: &[crate::EnvelopeElement]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|row| Self::key(row).to_string()).collect();
        inverse.added = base.iter().filter(|row| self.removed.iter().any(|id| id == Self::key(row))).cloned().collect();
        inverse.modified = self.modified.iter().filter_map(|patch| base.iter().find(|row| Self::key(row) == patch.id).map(|row| patch.inverse_from_row(row))).collect();
        let base_ids = Self::ids(base);
        let mut after_ids: Vec<String> = base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().collect();
        after_ids.extend(self.added.iter().map(|row| Self::key(row).to_string()));
        if let Some(order) = &self.order {
            after_ids = order.clone();
        }
        let mut natural: Vec<String> = after_ids.into_iter().filter(|id| !inverse.removed.contains(id)).collect();
        natural.extend(inverse.added.iter().map(|row| Self::key(row).to_string()));
        if natural != base_ids {
            inverse.order = Some(base_ids);
        }
        inverse
    }

    fn between_rows(base: &[crate::EnvelopeElement], other: &[crate::EnvelopeElement]) -> Self {
        let mut diff = Self::default();
        diff.removed = base.iter().filter(|row| other.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).map(|row| Self::key(row).to_string()).collect();
        diff.added = other.iter().filter(|row| base.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).cloned().collect();
        let mut natural: Vec<String> = Self::ids(base).into_iter().filter(|id| !diff.removed.contains(id)).collect();
        natural.extend(diff.added.iter().map(|row| Self::key(row).to_string()));
        let wanted = Self::ids(other);
        if natural != wanted {
            diff.order = Some(wanted);
        }
        diff
    }
}

/// 🩹️ Sparse per-field patch of the `heating` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599HeatingPatch {
    pub generation_efficiency: Option<f64>,
    pub distribution_efficiency: Option<f64>,
    pub storage_efficiency: Option<f64>,
    pub transfer_efficiency: Option<f64>,
    pub energy_carrier: Option<String>,
}

impl Din18599HeatingPatch {
    fn apply_to_row(&self, row: &mut crate::HeatingSystem) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.generation_efficiency {
            row.generation_efficiency = value.clone();
        }
        if let Some(value) = &self.distribution_efficiency {
            row.distribution_efficiency = value.clone();
        }
        if let Some(value) = &self.storage_efficiency {
            row.storage_efficiency = value.clone();
        }
        if let Some(value) = &self.transfer_efficiency {
            row.transfer_efficiency = value.clone();
        }
        if let Some(value) = &self.energy_carrier {
            row.energy_carrier = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.generation_efficiency.is_some() {
            self.generation_efficiency = other.generation_efficiency;
        }
        if other.distribution_efficiency.is_some() {
            self.distribution_efficiency = other.distribution_efficiency;
        }
        if other.storage_efficiency.is_some() {
            self.storage_efficiency = other.storage_efficiency;
        }
        if other.transfer_efficiency.is_some() {
            self.transfer_efficiency = other.transfer_efficiency;
        }
        if other.energy_carrier.is_some() {
            self.energy_carrier = other.energy_carrier;
        }
    }

    fn inverse_from_row(&self, row: &crate::HeatingSystem) -> Self {
        Self {
            generation_efficiency: self.generation_efficiency.as_ref().map(|_| row.generation_efficiency.clone()),
            distribution_efficiency: self.distribution_efficiency.as_ref().map(|_| row.distribution_efficiency.clone()),
            storage_efficiency: self.storage_efficiency.as_ref().map(|_| row.storage_efficiency.clone()),
            transfer_efficiency: self.transfer_efficiency.as_ref().map(|_| row.transfer_efficiency.clone()),
            energy_carrier: self.energy_carrier.as_ref().map(|_| row.energy_carrier.clone()),
        }
    }
}

/// 🩹️ Sparse per-field patch of the `dhw` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599DhwPatch {
    pub specific_demand_kwh_person_a: Option<f64>,
    pub storage_loss_kwh_a: Option<f64>,
    pub distribution_loss_kwh_a: Option<f64>,
    pub energy_carrier: Option<String>,
}

impl Din18599DhwPatch {
    fn apply_to_row(&self, row: &mut crate::DhwSystem) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.specific_demand_kwh_person_a {
            row.specific_demand_kwh_person_a = value.clone();
        }
        if let Some(value) = &self.storage_loss_kwh_a {
            row.storage_loss_kwh_a = value.clone();
        }
        if let Some(value) = &self.distribution_loss_kwh_a {
            row.distribution_loss_kwh_a = value.clone();
        }
        if let Some(value) = &self.energy_carrier {
            row.energy_carrier = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.specific_demand_kwh_person_a.is_some() {
            self.specific_demand_kwh_person_a = other.specific_demand_kwh_person_a;
        }
        if other.storage_loss_kwh_a.is_some() {
            self.storage_loss_kwh_a = other.storage_loss_kwh_a;
        }
        if other.distribution_loss_kwh_a.is_some() {
            self.distribution_loss_kwh_a = other.distribution_loss_kwh_a;
        }
        if other.energy_carrier.is_some() {
            self.energy_carrier = other.energy_carrier;
        }
    }

    fn inverse_from_row(&self, row: &crate::DhwSystem) -> Self {
        Self {
            specific_demand_kwh_person_a: self.specific_demand_kwh_person_a.as_ref().map(|_| row.specific_demand_kwh_person_a.clone()),
            storage_loss_kwh_a: self.storage_loss_kwh_a.as_ref().map(|_| row.storage_loss_kwh_a.clone()),
            distribution_loss_kwh_a: self.distribution_loss_kwh_a.as_ref().map(|_| row.distribution_loss_kwh_a.clone()),
            energy_carrier: self.energy_carrier.as_ref().map(|_| row.energy_carrier.clone()),
        }
    }
}

/// 🩹️ Sparse per-field patch of the `ventilation` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599VentilationPatch {
    pub airflow_m3_h: Option<f64>,
    pub heat_recovery_eta: Option<f64>,
    pub fan_power_w: Option<f64>,
}

impl Din18599VentilationPatch {
    fn apply_to_row(&self, row: &mut crate::VentilationSystem) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.airflow_m3_h {
            row.airflow_m3_h = value.clone();
        }
        if let Some(value) = &self.heat_recovery_eta {
            row.heat_recovery_eta = value.clone();
        }
        if let Some(value) = &self.fan_power_w {
            row.fan_power_w = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.airflow_m3_h.is_some() {
            self.airflow_m3_h = other.airflow_m3_h;
        }
        if other.heat_recovery_eta.is_some() {
            self.heat_recovery_eta = other.heat_recovery_eta;
        }
        if other.fan_power_w.is_some() {
            self.fan_power_w = other.fan_power_w;
        }
    }

    fn inverse_from_row(&self, row: &crate::VentilationSystem) -> Self {
        Self {
            airflow_m3_h: self.airflow_m3_h.as_ref().map(|_| row.airflow_m3_h.clone()),
            heat_recovery_eta: self.heat_recovery_eta.as_ref().map(|_| row.heat_recovery_eta.clone()),
            fan_power_w: self.fan_power_w.as_ref().map(|_| row.fan_power_w.clone()),
        }
    }
}

/// 🩹️ Sparse per-field patch of the `cooling` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599CoolingPatch {
    pub plant: Option<Din18599CoolingPatchPlantValue>,
}

impl Din18599CoolingPatch {
    fn apply_to_row(&self, row: &mut crate::CoolingSystem) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.plant {
            row.plant = value.value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.plant.is_some() {
            self.plant = other.plant;
        }
    }

    fn inverse_from_row(&self, row: &crate::CoolingSystem) -> Self {
        Self {
            plant: self.plant.as_ref().map(|_| Din18599CoolingPatchPlantValue { value: row.plant.clone() }),
        }
    }
}

/// 🩹️ Sparse per-field patch of the `lighting` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599LightingPatch {
    pub control_factor: Option<f64>,
}

impl Din18599LightingPatch {
    fn apply_to_row(&self, row: &mut crate::LightingSystem) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.control_factor {
            row.control_factor = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.control_factor.is_some() {
            self.control_factor = other.control_factor;
        }
    }

    fn inverse_from_row(&self, row: &crate::LightingSystem) -> Self {
        Self {
            control_factor: self.control_factor.as_ref().map(|_| row.control_factor.clone()),
        }
    }
}

/// 🩹️ Sparse per-field patch of the `renewables` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599RenewablesPatch {
    pub pv_area_m2: Option<f64>,
    pub pv_efficiency: Option<f64>,
    pub solar_thermal_kwh_a: Option<f64>,
}

impl Din18599RenewablesPatch {
    fn apply_to_row(&self, row: &mut crate::Renewables) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.pv_area_m2 {
            row.pv_area_m2 = value.clone();
        }
        if let Some(value) = &self.pv_efficiency {
            row.pv_efficiency = value.clone();
        }
        if let Some(value) = &self.solar_thermal_kwh_a {
            row.solar_thermal_kwh_a = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.pv_area_m2.is_some() {
            self.pv_area_m2 = other.pv_area_m2;
        }
        if other.pv_efficiency.is_some() {
            self.pv_efficiency = other.pv_efficiency;
        }
        if other.solar_thermal_kwh_a.is_some() {
            self.solar_thermal_kwh_a = other.solar_thermal_kwh_a;
        }
    }

    fn inverse_from_row(&self, row: &crate::Renewables) -> Self {
        Self {
            pv_area_m2: self.pv_area_m2.as_ref().map(|_| row.pv_area_m2.clone()),
            pv_efficiency: self.pv_efficiency.as_ref().map(|_| row.pv_efficiency.clone()),
            solar_thermal_kwh_a: self.solar_thermal_kwh_a.as_ref().map(|_| row.solar_thermal_kwh_a.clone()),
        }
    }
}

/// 🩹️ Sparse per-field patch of the `climate` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599ClimatePatch {
    pub theta_e_c: Option<[f64; 12]>,
    pub g_h_w_m2: Option<[f64; 12]>,
}

impl Din18599ClimatePatch {
    fn apply_to_row(&self, row: &mut crate::MonthlyClimate) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.theta_e_c {
            row.theta_e_c = value.clone();
        }
        if let Some(value) = &self.g_h_w_m2 {
            row.g_h_w_m2 = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.theta_e_c.is_some() {
            self.theta_e_c = other.theta_e_c;
        }
        if other.g_h_w_m2.is_some() {
            self.g_h_w_m2 = other.g_h_w_m2;
        }
    }

    fn inverse_from_row(&self, row: &crate::MonthlyClimate) -> Self {
        Self {
            theta_e_c: self.theta_e_c.as_ref().map(|_| row.theta_e_c.clone()),
            g_h_w_m2: self.g_h_w_m2.as_ref().map(|_| row.g_h_w_m2.clone()),
        }
    }
}

/// 🎁️ Carries the optional `CoolingSystem.plant` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599CoolingPatchPlantValue {
    pub value: Option<crate::CoolingPlant>,
}

/// 🔺️ Keyed sparse diff of the Din18599 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Diff {
    #[state(artifact)]
    pub building_category: Option<crate::BuildingCategory>,
    #[state(artifact)]
    pub attachment: Option<crate::Attachment>,
    #[state(artifact)]
    pub use_class: Option<crate::UseClass>,
    #[state(artifact)]
    pub method: Option<crate::CalculationMethod>,
    #[state(artifact)]
    pub net_floor_area_m2: Option<f64>,
    #[state(artifact)]
    pub heated_volume_m3: Option<f64>,
    #[state(artifact)]
    pub geg_qp_factor: Option<f64>,
    #[state(artifact)]
    pub delta_u_wb_w_m2k: Option<f64>,
    #[state(artifact)]
    pub automation_class: Option<crate::AutomationClass>,
    #[state(artifact)]
    pub zones: Option<Din18599ZonesRows>,
    #[state(artifact)]
    pub elements: Option<Din18599ElementsRows>,
    #[state(artifact)]
    pub heating: Option<Din18599HeatingPatch>,
    #[state(artifact)]
    pub dhw: Option<Din18599DhwPatch>,
    #[state(artifact)]
    pub ventilation: Option<Din18599VentilationPatch>,
    #[state(artifact)]
    pub cooling: Option<Din18599CoolingPatch>,
    #[state(artifact)]
    pub lighting: Option<Din18599LightingPatch>,
    #[state(artifact)]
    pub renewables: Option<Din18599RenewablesPatch>,
    #[state(artifact)]
    pub climate: Option<Din18599ClimatePatch>,
    #[state(artifact)]
    pub climate_table: Option<crate::Din18599ClimateChild>,
}

impl protocol::MutationDiff<Din18599Snapshot> for Din18599Diff {
    fn apply(&self, base: &Din18599Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Din18599Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.building_category {
            next.building_category = value.clone();
        }
        if let Some(value) = &self.attachment {
            next.attachment = value.clone();
        }
        if let Some(value) = &self.use_class {
            next.use_class = value.clone();
        }
        if let Some(value) = &self.method {
            next.method = value.clone();
        }
        if let Some(value) = &self.net_floor_area_m2 {
            next.net_floor_area_m2 = value.clone();
        }
        if let Some(value) = &self.heated_volume_m3 {
            next.heated_volume_m3 = value.clone();
        }
        if let Some(value) = &self.geg_qp_factor {
            next.geg_qp_factor = value.clone();
        }
        if let Some(value) = &self.delta_u_wb_w_m2k {
            next.delta_u_wb_w_m2k = value.clone();
        }
        if let Some(value) = &self.automation_class {
            next.automation_class = value.clone();
        }
        if let Some(rows) = &self.zones {
            next.zones = rows.apply_rows(&base.zones).map_err(|error| error.under(["zones"]))?;
        }
        if let Some(rows) = &self.elements {
            next.elements = rows.apply_rows(&base.elements).map_err(|error| error.under(["elements"]))?;
        }
        if let Some(patch) = &self.heating {
            patch.apply_to_row(&mut next.heating).map_err(|error| error.under(["heating"]))?;
        }
        if let Some(patch) = &self.dhw {
            patch.apply_to_row(&mut next.dhw).map_err(|error| error.under(["dhw"]))?;
        }
        if let Some(patch) = &self.ventilation {
            patch.apply_to_row(&mut next.ventilation).map_err(|error| error.under(["ventilation"]))?;
        }
        if let Some(patch) = &self.cooling {
            patch.apply_to_row(&mut next.cooling).map_err(|error| error.under(["cooling"]))?;
        }
        if let Some(patch) = &self.lighting {
            patch.apply_to_row(&mut next.lighting).map_err(|error| error.under(["lighting"]))?;
        }
        if let Some(patch) = &self.renewables {
            patch.apply_to_row(&mut next.renewables).map_err(|error| error.under(["renewables"]))?;
        }
        if let Some(patch) = &self.climate {
            patch.apply_to_row(&mut next.climate).map_err(|error| error.under(["climate"]))?;
        }
        if let Some(value) = &self.climate_table {
            next.climate_table = value.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.building_category.is_some() {
            self.building_category = other.building_category;
        }
        if other.attachment.is_some() {
            self.attachment = other.attachment;
        }
        if other.use_class.is_some() {
            self.use_class = other.use_class;
        }
        if other.method.is_some() {
            self.method = other.method;
        }
        if other.net_floor_area_m2.is_some() {
            self.net_floor_area_m2 = other.net_floor_area_m2;
        }
        if other.heated_volume_m3.is_some() {
            self.heated_volume_m3 = other.heated_volume_m3;
        }
        if other.geg_qp_factor.is_some() {
            self.geg_qp_factor = other.geg_qp_factor;
        }
        if other.delta_u_wb_w_m2k.is_some() {
            self.delta_u_wb_w_m2k = other.delta_u_wb_w_m2k;
        }
        if other.automation_class.is_some() {
            self.automation_class = other.automation_class;
        }
        if let Some(theirs) = other.zones {
            match self.zones.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.zones = Some(theirs),
            }
            self.zones = self.zones.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.elements {
            match self.elements.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.elements = Some(theirs),
            }
            self.elements = self.elements.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.heating {
            match self.heating.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.heating = Some(theirs),
            }
        }
        if let Some(theirs) = other.dhw {
            match self.dhw.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.dhw = Some(theirs),
            }
        }
        if let Some(theirs) = other.ventilation {
            match self.ventilation.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.ventilation = Some(theirs),
            }
        }
        if let Some(theirs) = other.cooling {
            match self.cooling.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.cooling = Some(theirs),
            }
        }
        if let Some(theirs) = other.lighting {
            match self.lighting.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.lighting = Some(theirs),
            }
        }
        if let Some(theirs) = other.renewables {
            match self.renewables.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.renewables = Some(theirs),
            }
        }
        if let Some(theirs) = other.climate {
            match self.climate.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.climate = Some(theirs),
            }
        }
        if other.climate_table.is_some() {
            self.climate_table = other.climate_table;
        }
    }
}

impl protocol::DiffAlgebra<Din18599Snapshot> for Din18599Diff {
    fn inverse(&self, base: &Din18599Snapshot) -> Self {
        Self {
            building_category: self.building_category.as_ref().map(|_| base.building_category.clone()),
            attachment: self.attachment.as_ref().map(|_| base.attachment.clone()),
            use_class: self.use_class.as_ref().map(|_| base.use_class.clone()),
            method: self.method.as_ref().map(|_| base.method.clone()),
            net_floor_area_m2: self.net_floor_area_m2.as_ref().map(|_| base.net_floor_area_m2.clone()),
            heated_volume_m3: self.heated_volume_m3.as_ref().map(|_| base.heated_volume_m3.clone()),
            geg_qp_factor: self.geg_qp_factor.as_ref().map(|_| base.geg_qp_factor.clone()),
            delta_u_wb_w_m2k: self.delta_u_wb_w_m2k.as_ref().map(|_| base.delta_u_wb_w_m2k.clone()),
            automation_class: self.automation_class.as_ref().map(|_| base.automation_class.clone()),
            zones: self.zones.as_ref().map(|rows| rows.inverse_rows(&base.zones)).filter(|rows| !rows.is_empty()),
            elements: self.elements.as_ref().map(|rows| rows.inverse_rows(&base.elements)).filter(|rows| !rows.is_empty()),
            heating: self.heating.as_ref().map(|patch| patch.inverse_from_row(&base.heating)),
            dhw: self.dhw.as_ref().map(|patch| patch.inverse_from_row(&base.dhw)),
            ventilation: self.ventilation.as_ref().map(|patch| patch.inverse_from_row(&base.ventilation)),
            cooling: self.cooling.as_ref().map(|patch| patch.inverse_from_row(&base.cooling)),
            lighting: self.lighting.as_ref().map(|patch| patch.inverse_from_row(&base.lighting)),
            renewables: self.renewables.as_ref().map(|patch| patch.inverse_from_row(&base.renewables)),
            climate: self.climate.as_ref().map(|patch| patch.inverse_from_row(&base.climate)),
            climate_table: self.climate_table.as_ref().map(|_| base.climate_table.clone()),
        }
    }

    fn between(base: &Din18599Snapshot, other: &Din18599Snapshot) -> Self {
        Self {
            building_category: (base.building_category != other.building_category).then(|| other.building_category.clone()),
            attachment: (base.attachment != other.attachment).then(|| other.attachment.clone()),
            use_class: (base.use_class != other.use_class).then(|| other.use_class.clone()),
            method: (base.method != other.method).then(|| other.method.clone()),
            net_floor_area_m2: (base.net_floor_area_m2 != other.net_floor_area_m2).then(|| other.net_floor_area_m2.clone()),
            heated_volume_m3: (base.heated_volume_m3 != other.heated_volume_m3).then(|| other.heated_volume_m3.clone()),
            geg_qp_factor: (base.geg_qp_factor != other.geg_qp_factor).then(|| other.geg_qp_factor.clone()),
            delta_u_wb_w_m2k: (base.delta_u_wb_w_m2k != other.delta_u_wb_w_m2k).then(|| other.delta_u_wb_w_m2k.clone()),
            automation_class: (base.automation_class != other.automation_class).then(|| other.automation_class.clone()),
            zones: Some(Din18599ZonesRows::between_rows(&base.zones, &other.zones)).filter(|rows| !rows.is_empty()),
            elements: Some(Din18599ElementsRows::between_rows(&base.elements, &other.elements)).filter(|rows| !rows.is_empty()),
            heating: Some(Din18599HeatingPatch {
                generation_efficiency: (base.heating.generation_efficiency != other.heating.generation_efficiency).then(|| other.heating.generation_efficiency.clone()),
                distribution_efficiency: (base.heating.distribution_efficiency != other.heating.distribution_efficiency).then(|| other.heating.distribution_efficiency.clone()),
                storage_efficiency: (base.heating.storage_efficiency != other.heating.storage_efficiency).then(|| other.heating.storage_efficiency.clone()),
                transfer_efficiency: (base.heating.transfer_efficiency != other.heating.transfer_efficiency).then(|| other.heating.transfer_efficiency.clone()),
                energy_carrier: (base.heating.energy_carrier != other.heating.energy_carrier).then(|| other.heating.energy_carrier.clone()),
            }).filter(|patch| *patch != Din18599HeatingPatch::default()),
            dhw: Some(Din18599DhwPatch {
                specific_demand_kwh_person_a: (base.dhw.specific_demand_kwh_person_a != other.dhw.specific_demand_kwh_person_a).then(|| other.dhw.specific_demand_kwh_person_a.clone()),
                storage_loss_kwh_a: (base.dhw.storage_loss_kwh_a != other.dhw.storage_loss_kwh_a).then(|| other.dhw.storage_loss_kwh_a.clone()),
                distribution_loss_kwh_a: (base.dhw.distribution_loss_kwh_a != other.dhw.distribution_loss_kwh_a).then(|| other.dhw.distribution_loss_kwh_a.clone()),
                energy_carrier: (base.dhw.energy_carrier != other.dhw.energy_carrier).then(|| other.dhw.energy_carrier.clone()),
            }).filter(|patch| *patch != Din18599DhwPatch::default()),
            ventilation: Some(Din18599VentilationPatch {
                airflow_m3_h: (base.ventilation.airflow_m3_h != other.ventilation.airflow_m3_h).then(|| other.ventilation.airflow_m3_h.clone()),
                heat_recovery_eta: (base.ventilation.heat_recovery_eta != other.ventilation.heat_recovery_eta).then(|| other.ventilation.heat_recovery_eta.clone()),
                fan_power_w: (base.ventilation.fan_power_w != other.ventilation.fan_power_w).then(|| other.ventilation.fan_power_w.clone()),
            }).filter(|patch| *patch != Din18599VentilationPatch::default()),
            cooling: Some(Din18599CoolingPatch {
                plant: (base.cooling.plant != other.cooling.plant).then(|| Din18599CoolingPatchPlantValue { value: other.cooling.plant.clone() }),
            }).filter(|patch| *patch != Din18599CoolingPatch::default()),
            lighting: Some(Din18599LightingPatch {
                control_factor: (base.lighting.control_factor != other.lighting.control_factor).then(|| other.lighting.control_factor.clone()),
            }).filter(|patch| *patch != Din18599LightingPatch::default()),
            renewables: Some(Din18599RenewablesPatch {
                pv_area_m2: (base.renewables.pv_area_m2 != other.renewables.pv_area_m2).then(|| other.renewables.pv_area_m2.clone()),
                pv_efficiency: (base.renewables.pv_efficiency != other.renewables.pv_efficiency).then(|| other.renewables.pv_efficiency.clone()),
                solar_thermal_kwh_a: (base.renewables.solar_thermal_kwh_a != other.renewables.solar_thermal_kwh_a).then(|| other.renewables.solar_thermal_kwh_a.clone()),
            }).filter(|patch| *patch != Din18599RenewablesPatch::default()),
            climate: Some(Din18599ClimatePatch {
                theta_e_c: (base.climate.theta_e_c != other.climate.theta_e_c).then(|| other.climate.theta_e_c.clone()),
                g_h_w_m2: (base.climate.g_h_w_m2 != other.climate.g_h_w_m2).then(|| other.climate.g_h_w_m2.clone()),
            }).filter(|patch| *patch != Din18599ClimatePatch::default()),
            climate_table: (base.climate_table != other.climate_table).then(|| other.climate_table.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.building_category.is_none() && self.attachment.is_none() && self.use_class.is_none() && self.method.is_none() && self.net_floor_area_m2.is_none() && self.heated_volume_m3.is_none() && self.geg_qp_factor.is_none() && self.delta_u_wb_w_m2k.is_none() && self.automation_class.is_none() && self.zones.as_ref().map_or(true, |rows| rows.is_empty()) && self.elements.as_ref().map_or(true, |rows| rows.is_empty()) && self.heating.is_none() && self.dhw.is_none() && self.ventilation.is_none() && self.cooling.is_none() && self.lighting.is_none() && self.renewables.is_none() && self.climate.is_none() && self.climate_table.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
