//! 👁️ Replaces the app-local solve cache shared by the bitmap editor's output window.

use super::{BitmapTransient, BitmapTransientMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-solve")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSolve {
    pub output_pixels: Option<String>,
    pub contradiction: bool,
    pub output_width: u32,
    pub output_height: u32,
}

impl protocol::MutationKind<BitmapTransient, BitmapTransientMutation> for SetSolve {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "solve", kind: "set-solve", record: "SetSolve" };
    fn diff(&self, _base: &BitmapTransient) -> protocol::MutationOutcome<BitmapTransient> {
        protocol::MutationOutcome::new(BitmapTransient { output_pixels: self.output_pixels.clone(), contradiction: self.contradiction, output_width: self.output_width, output_height: self.output_height })
    }
    fn inverse(&self, base: &BitmapTransient) -> Vec<BitmapTransientMutation> {
        vec![Self { output_pixels: base.output_pixels.clone(), contradiction: base.contradiction, output_width: base.output_width, output_height: base.output_height }.into()]
    }
    fn label(&self) -> String {
        "Set Solve".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["outputPixels".into()]
    }
}
