//! 🔺️ Din18599 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifact_schema::Din18599Artifact;
use crate::Din18599Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl Din18599Diff {
    pub fn apply_to_artifact(&self, artifact: &Din18599Artifact) -> protocol::MutationApplyResult<Din18599Artifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(value) = self.building_category {
                next.building_category = value;
            }
            if let Some(value) = self.attachment {
                next.attachment = value;
            }
            if let Some(value) = self.use_class {
                next.use_class = value;
            }
            if let Some(value) = self.method {
                next.method = value;
            }
            if let Some(value) = self.net_floor_area_m2 {
                next.net_floor_area_m2 = value;
            }
            if let Some(value) = self.heated_volume_m3 {
                next.heated_volume_m3 = value;
            }
            if let Some(value) = self.geg_qp_factor {
                next.geg_qp_factor = value;
            }
            if let Some(value) = self.delta_u_wb_w_m2k {
                next.delta_u_wb_w_m2k = value;
            }
            if let Some(value) = self.automation_class {
                next.automation_class = value;
            }
            if let Some(list) = &self.zones {
                next.zones = list.values.clone();
            }
            if let Some(list) = &self.elements {
                next.elements = list.values.clone();
            }
            if let Some(value) = &self.heating {
                next.heating = value.clone();
            }
            if let Some(value) = &self.dhw {
                next.dhw = value.clone();
            }
            if let Some(value) = &self.ventilation {
                next.ventilation = value.clone();
            }
            if let Some(value) = &self.cooling {
                next.cooling = value.clone();
            }
            if let Some(value) = &self.lighting {
                next.lighting = value.clone();
            }
            if let Some(value) = &self.renewables {
                next.renewables = value.clone();
            }
            if let Some(value) = &self.climate {
                next.climate = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<Din18599Snapshot> for Din18599Diff {
    fn apply(&self, snapshot: &Din18599Snapshot) -> protocol::MutationApplyResult<Din18599Snapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(value) = self.building_category {
                next.building_category = value;
            }
            if let Some(value) = self.attachment {
                next.attachment = value;
            }
            if let Some(value) = self.use_class {
                next.use_class = value;
            }
            if let Some(value) = self.method {
                next.method = value;
            }
            if let Some(value) = self.net_floor_area_m2 {
                next.net_floor_area_m2 = value;
            }
            if let Some(value) = self.heated_volume_m3 {
                next.heated_volume_m3 = value;
            }
            if let Some(value) = self.geg_qp_factor {
                next.geg_qp_factor = value;
            }
            if let Some(value) = self.delta_u_wb_w_m2k {
                next.delta_u_wb_w_m2k = value;
            }
            if let Some(value) = self.automation_class {
                next.automation_class = value;
            }
            if let Some(list) = &self.zones {
                next.zones = list.values.clone();
            }
            if let Some(list) = &self.elements {
                next.elements = list.values.clone();
            }
            if let Some(value) = &self.heating {
                next.heating = value.clone();
            }
            if let Some(value) = &self.dhw {
                next.dhw = value.clone();
            }
            if let Some(value) = &self.ventilation {
                next.ventilation = value.clone();
            }
            if let Some(value) = &self.cooling {
                next.cooling = value.clone();
            }
            if let Some(value) = &self.lighting {
                next.lighting = value.clone();
            }
            if let Some(value) = &self.renewables {
                next.renewables = value.clone();
            }
            if let Some(value) = &self.climate {
                next.climate = value.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(building_category);
        take!(attachment);
        take!(use_class);
        take!(method);
        take!(net_floor_area_m2);
        take!(heated_volume_m3);
        take!(geg_qp_factor);
        take!(delta_u_wb_w_m2k);
        take!(automation_class);
        take!(zones);
        take!(elements);
        take!(heating);
        take!(dhw);
        take!(ventilation);
        take!(cooling);
        take!(lighting);
        take!(renewables);
        take!(climate);
    }
}
//#endregion 🔖️Apply

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Din18599DiffText = String;
//#endregion 🚚️Carrier
