//! ❄️ Explicit synchronous FEM snapshot values own canonical authoring and reference algebra.
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use crate::standards::v1::subsets::any::schema::structure::FemCombination;
use crate::standards::v1::subsets::any::schema::structure::FemElement;
use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use crate::standards::v1::subsets::any::schema::structure::FemSection;
use crate::standards::v1::subsets::any::schema::structure::FemSolid;
use crate::standards::v1::subsets::any::schema::structure::FemSupport;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields, validate = "Self::validate_interaction_identities", validate_controlled = "Self::validate_interaction_identities_controlled")]
#[dsl(id = "fem.fem3d", layout = "lines")]
#[artifact_schema(id = "s.fem.fem3d")]
pub struct Fem3dSnapshotValue {
    #[dsl(table)]
    #[state(artifact)]
    pub nodes: Vec<FemNodeValue>,
    #[dsl(statements, block)]
    #[state(artifact)]
    pub elements: Vec<FemElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub materials: Vec<FemMaterialValue>,
    #[dsl(table)]
    #[state(artifact)]
    pub sections: Vec<FemSection>,
    #[dsl(table)]
    #[state(artifact)]
    pub solids: Vec<FemSolid>,
    #[dsl(table)]
    #[state(artifact)]
    pub supports: Vec<FemSupport>,
    #[dsl(table)]
    #[state(artifact)]
    pub load_cases: Vec<FemLoadCase>,
    #[dsl(table)]
    #[state(artifact)]
    pub combinations: Vec<FemCombination>,
    #[dsl(block)]
    #[state(artifact)]
    pub analysis: FemAnalysisSettings,
}
impl Fem3dSnapshotValue {
 /// 🛂️ Explicit reference decoding validates plain authoring identities under its original decoder control.
 pub fn validate_interaction_identities_controlled(&self,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_framework_value::ValueError>{super::identity_admission::validate_typed(self,control)}
    pub fn validate_interaction_identities(&self) -> Result<(), semio_framework_value::ValueError> {
        let invalid = || semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "FEM3d identity is empty, repeated in its namespace, or exceeds shared scoped identity bounds");
        let rows = self.nodes.iter().map(|row| ("node", row.id.as_str()))
            .chain(self.elements.iter().map(|row| ("element", crate::standards::v1::subsets::any::schema::structure::element_id(row))))
            .chain(self.solids.iter().map(|row| ("solid", row.id.as_str())))
            .chain(self.materials.iter().map(|row| ("material", row.id.as_str())))
            .chain(self.sections.iter().map(|row| ("section", row.id.as_str())))
            .chain(self.supports.iter().map(|row| ("support", row.id.as_str())))
            .chain(self.load_cases.iter().map(|row| ("loadCase", row.id.as_str())))
            .chain(self.combinations.iter().map(|row| ("combination", row.id.as_str())));
        let mut seen = std::collections::HashSet::new();
        for (kind, id) in rows {
            protocol::wire::validate_scoped_identity_segments(&[kind, id]).map_err(|_| invalid())?;
            if !seen.insert((kind, id)) { return Err(invalid()); }
        }
        let mut loads = std::collections::HashSet::new();
        for case in &self.load_cases {
            for load in &case.loads {
                let id = crate::standards::v1::subsets::any::schema::structure::load_id(load);
                protocol::wire::validate_scoped_identity_segments(&["load", &case.id, id]).map_err(|_| invalid())?;
                if !loads.insert((case.id.as_str(), id)) { return Err(invalid()); }
            }
        }
        Ok(())
    }
}
