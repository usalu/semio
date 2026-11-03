//! 🦀️ EN 1990 mutation round trip — the Rust SUBJECT adapter. The committed mutation is applied by the production dispatch and undone by its own computed inverse; the before-snapshot must be restored exactly. The independent Python mutation vocabulary answers the same rows in the oracle role. Step plumbing is the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1990_io::text::mutations::decode_en1990_mutation_json;
    use semio_s_artifact_norm_en1990_io::text::snapshot::decode_en1990_snapshot_json;
    use semio_s_artifact_norm_en1990_mutations::mutation_bridge::{apply_en1990_mutation, inverse_en1990_mutation};

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.round_trip = Box::new(|context, before, mutation| {
            let read = |uri: &str| -> Result<String, String> { String::from_utf8(context.input_bytes(uri)?).map_err(|error| format!("{uri}: {error}")) };
            let base = decode_en1990_snapshot_json(&read(before)?).map_err(|error| format!("{before}: {error}"))?;
            let mutation = decode_en1990_mutation_json(&read(mutation)?).map_err(|error| format!("{mutation}: {error}"))?;
            let (mut current, _) = apply_en1990_mutation(&base, &mutation).map_err(|error| format!("forward mutation: {error}"))?;
            let moved = current != base;
            let steps = inverse_en1990_mutation(&mutation, &base).map_err(|error| format!("computed inverse: {error}"))?;
            for step in &steps {
                current = apply_en1990_mutation(&current, step).map_err(|error| format!("inverse step: {error}"))?.0;
            }
            Ok(moved && !steps.is_empty() && current == base)
        });
        bound
    }
}

/// 🧭️ Registration is by scenario family; the oracle role is the Python adapter beside this file.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = steps::norm_adapter(subject::binding());
    }
    built
}
