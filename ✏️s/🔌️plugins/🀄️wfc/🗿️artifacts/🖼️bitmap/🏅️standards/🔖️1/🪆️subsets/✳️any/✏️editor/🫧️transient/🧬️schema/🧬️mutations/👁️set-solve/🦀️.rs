//! 👁️ Replaces the app-local solve cache shared by the bitmap editor's output window.

use super::{BitmapTransient, BitmapTransientDiff, BitmapTransientMutation, BitmapTransientText};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    fn diff(&self, base: &BitmapTransient) -> protocol::MutationOutcome<BitmapTransientDiff> {
        let diff = BitmapTransientDiff {
            output_pixels: (base.output_pixels != self.output_pixels).then(|| BitmapTransientText { value: self.output_pixels.clone() }),
            contradiction: (base.contradiction != self.contradiction).then_some(self.contradiction),
            output_width: (base.output_width != self.output_width).then_some(self.output_width),
            output_height: (base.output_height != self.output_height).then_some(self.output_height),
        };
        if protocol::DiffAlgebra::<BitmapTransient>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The solve already holds that result.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &BitmapTransient) -> Result<Vec<BitmapTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { output_pixels: base.output_pixels.clone(), contradiction: base.contradiction, output_width: base.output_width, output_height: base.output_height }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Solve", "Lösung setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["outputPixels".into()]
    }
}
