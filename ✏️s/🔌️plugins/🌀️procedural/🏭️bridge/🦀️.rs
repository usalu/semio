//! 🏭️ Production mutation bridge for `✏️s/🔌️plugins/🌀️procedural`.
//!
//! `test inventory` runs this and compares what it prints against each owner manifest and its claimed test
//! catalog. Every row is read out of a production aggregate's `DESCRIPTORS`, which the `dsl::Mutations` derive
//! generates from the mutation leaves themselves; a subset's inventory is the descriptors whose leaf `owner` lies
//! inside that subset's owner, so a verb dispatched in production and absent from the manifest is a breach.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts — the caller.

extern crate semio_framework_os_kernel as protocol;

use protocol::{Mutation, MutationLeafDescriptor, MutationOutcomeClass};

/// 🧬️ One aggregate's descriptors, for the snapshot its `#[mutations(snapshot = …)]` names.
fn descriptors<S, M: Mutation<S>>() -> &'static [MutationLeafDescriptor] {
    M::DESCRIPTORS
}

/// 🧭️ Every mutation aggregate the artifact crates below mount.
const AGGREGATES: &[fn() -> &'static [MutationLeafDescriptor]] = &[
    descriptors::<semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshot, semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::schema::mutations::Generation2dMutation>,
    descriptors::<semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshot, semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::Generation3dMutation>,
];

/// 🗺️ Manifest coordinate → the owner directory whose leaves it measures.
const COORDINATES: &[(&str, &str, &str, &str)] = &[
    ("s.procedural.generation2d", "1", "any", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.procedural.generation3d", "1", "any", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any"),
];

/// 🎯️ Production outcome severities as protocol outcome classes: `Info`/`Warning` ride on an applied outcome.
fn protocol_outcomes(classes: &[MutationOutcomeClass]) -> Vec<&'static str> {
    let mut seen: Vec<&'static str> = Vec::new();
    for outcome in classes {
        let mapped = match outcome {
            MutationOutcomeClass::Applied | MutationOutcomeClass::Info | MutationOutcomeClass::Warning => "applied",
            MutationOutcomeClass::Error | MutationOutcomeClass::Fatal => "rejected",
        };
        if !seen.contains(&mapped) {
            seen.push(mapped);
        }
    }
    if seen.is_empty() {
        seen.push("applied");
    }
    seen
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [command, artifact, standard, subset] = args.as_slice() else {
        eprintln!("usage: list-mutations <artifact> <standard> <subset>");
        std::process::exit(2);
    };
    let Some((_, _, _, owner)) = COORDINATES.iter().find(|(a, s, u, _)| command == "list-mutations" && a == artifact && s == standard && u == subset) else {
        eprintln!("this bridge does not answer {command} {artifact} {standard} {subset}");
        std::process::exit(2);
    };
    let prefix = format!("{owner}/");
    let mut rows: Vec<pack::JsonValue> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for descriptor in AGGREGATES.iter().flat_map(|aggregate| aggregate().iter()) {
        if !descriptor.owner.starts_with(&prefix) || seen.contains(&descriptor.semantic_kind) {
            continue;
        }
        seen.push(descriptor.semantic_kind);
        rows.push(pack::json_object([
            ("id".to_string(), pack::JsonValue::from(descriptor.semantic_kind)),
            ("variant".to_string(), pack::JsonValue::from(descriptor.aggregate_variant)),
            ("outcomes".to_string(), pack::json_array(protocol_outcomes(descriptor.outcome_classes).into_iter().map(pack::JsonValue::from))),
        ]));
    }
    let out = pack::json_object([
        ("schema".to_string(), pack::JsonValue::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), pack::JsonValue::from(artifact.as_str())),
        ("standard".to_string(), pack::JsonValue::from(standard.as_str())),
        ("subset".to_string(), pack::JsonValue::from(subset.as_str())),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("producedBy".to_string(), pack::JsonValue::from("semio-procedural-mutation-bridge")),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{}", pack::json_to_string(&out));
}
