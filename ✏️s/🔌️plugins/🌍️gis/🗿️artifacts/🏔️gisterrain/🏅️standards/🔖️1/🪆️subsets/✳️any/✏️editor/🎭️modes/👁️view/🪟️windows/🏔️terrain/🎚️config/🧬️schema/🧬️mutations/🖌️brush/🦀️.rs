//! 🖌️ Reversible mutation for the addressed terrain window's brush settings.

use super::super::{GisTerrainWindowConfig, GisTerrainWindowConfigDelta, GisTerrainWindowConfigDiff, GisTerrainWindowConfigMutation, TerrainBrushChange, TerrainBrushSettings};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-terrain-brush")]
pub struct SetTerrainBrush {
    #[dsl(block)]
    #[value(default)]
    pub brush: Option<TerrainBrushSettings>,
}

impl MutationKind<GisTerrainWindowConfig, GisTerrainWindowConfigMutation> for SetTerrainBrush {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "brush", kind: "set-terrain-brush", record: "SetTerrainBrush" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Set terrain brush", "Geländepinsel einstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["brush".into()]
    }
}

impl protocol::ColdMutationKind<GisTerrainWindowConfig, GisTerrainWindowConfigMutation> for SetTerrainBrush {
    fn diff(&self, base: &GisTerrainWindowConfig) -> MutationOutcome<GisTerrainWindowConfigDiff> {
        if self.brush.map(TerrainBrushSettings::validate).transpose().is_err() {
            return MutationOutcome::error("mutation.invalid", "Terrain brush settings exceed their ranges.", ["brush"]);
        }
        if self.brush == base.brush {
            return MutationOutcome::empty().warning("mutation.no-op", "The terrain brush already uses these settings.");
        }
        MutationOutcome::new(GisTerrainWindowConfigDelta {
            brush: Some(TerrainBrushChange { brush: self.brush }),
            ..Default::default()
        }.into())
    }

    fn inverse(&self, base: &GisTerrainWindowConfig) -> Result<Vec<GisTerrainWindowConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![Self { brush: base.brush }.into()])
    }
}
