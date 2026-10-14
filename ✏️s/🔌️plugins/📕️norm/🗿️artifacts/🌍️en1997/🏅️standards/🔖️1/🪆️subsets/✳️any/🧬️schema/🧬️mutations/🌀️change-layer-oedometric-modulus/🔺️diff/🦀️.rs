use crate::change_layer_oedometric_modulus::ChangeLayerOedometricModulus;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LayersRows, En1997LayersPatch};

pub fn diff(payload: &ChangeLayerOedometricModulus, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_oedometric_modulus.is_finite() || payload.new_oedometric_modulus <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "E_oed must be positive", vec![payload.id.clone()]);
    }
    let Some(_idx) = base.layers.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("layer {} missing", payload.id), vec![payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1997Diff {
        layers: Some(En1997LayersRows::modification(&payload.id, En1997LayersPatch { oedometric_modulus: Some(payload.new_oedometric_modulus), ..Default::default() })),
        ..Default::default()
    })
}
