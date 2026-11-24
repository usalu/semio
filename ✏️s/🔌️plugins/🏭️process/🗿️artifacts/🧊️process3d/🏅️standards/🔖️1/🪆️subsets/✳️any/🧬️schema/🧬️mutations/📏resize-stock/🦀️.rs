//! 📏️ Change one authored stock dimension and its composed solid.

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_process_process3d_model::snapshot::{Process3dSnapshot, working_solid_child_handle};
use semio_s_artifact_process_process3d_model_diff::Process3dDiff;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeStock {
    pub field: String,
    pub size: f64,
}

impl<Op> protocol::MutationKind<Process3dSnapshot, Op> for ResizeStock
where
    Op: protocol::Mutation<Process3dSnapshot, Diff = Process3dDiff> + From<Self> + From<crate::replace_stock_solid::ReplaceStockSolid>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "stock", kind: "resize-stock", record: "ResizedStock" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Resize stock", "Rohteilgröße ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Process3dSnapshot, Op> for ResizeStock
where
    Op: protocol::ColdMutation<Process3dSnapshot, ReferenceDiff = Process3dDiff> + From<Self> + From<crate::replace_stock_solid::ReplaceStockSolid>,
{
    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        let mut stock = base.stock_payload.clone();
        let Some(dimension) = stock.solid.dimension_mut(&self.field) else {
            return protocol::MutationOutcome::error("mutation.invalid-field", "Stock does not have the requested dimension.".to_string(), [self.field.clone()]);
        };
        if !self.size.is_finite() || self.size <= 0.0 {
            return protocol::MutationOutcome::error("mutation.invalid-value", "Stock dimensions must be finite and positive.".to_string(), [self.field.clone()]);
        }
        if *dimension == self.size {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "Stock dimension is unchanged.".to_string());
        }
        *dimension = self.size;
        let solid = working_solid_child_handle("stock", &stock.solid);
        protocol::MutationOutcome::new(Process3dDiff { stock_payload: Some(stock), stock_solid: Some(solid), ..Default::default() })
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        let mut solid = base.stock_payload.solid.clone();
        let Some(size) = solid.dimension_mut(&self.field).map(|value| *value) else {
            return Ok(Vec::new());
        };
        let mut inverse = vec![Op::from(Self { field: self.field.clone(), size })];
        if base.stock_solid != working_solid_child_handle("stock", &solid) {
            inverse.push(Op::from(crate::replace_stock_solid::ReplaceStockSolid { new_solid: base.stock_solid.clone() }));
        }
        Ok(inverse)
    }
}
