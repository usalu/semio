//! 🌦️ `change-climate-zone`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeClimateZone {
    pub new_climate_zone: semio_s_plugin_norm_compliance::ClimateZoneDe,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeClimateZone where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_climate_zone::ChangeClimateZone> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "climate-zone",
        kind: "change-climate-zone",
        record: "ChangedClimateZone",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change summer climate region", "Sommerklimaregion ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeClimateZone where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_climate_zone::ChangeClimateZone> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> {
        crate::change_climate_zone::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_climate_zone::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    //! 🔺️ `change-climate-zone` diff.

    use crate::change_climate_zone::ChangeClimateZone;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn diff(payload: &ChangeClimateZone, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        if base.climate_zone == payload.new_climate_zone {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "climate_zone already has this value.");
        }
        protocol::MutationOutcome::new(Din4108Diff { climate_zone: Some(payload.new_climate_zone), ..Default::default() })
    }
}
pub mod inverse {
    //! ↩️ `change-climate-zone` inverse.

    use crate::change_climate_zone::ChangeClimateZone;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn inverse<Op>(_payload: &ChangeClimateZone, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_climate_zone::ChangeClimateZone> {
        Ok((|| {
        vec![Op::from(ChangeClimateZone { new_climate_zone: base.climate_zone })]

        })())
    }
}
//#endregion 🪢️Mounts
