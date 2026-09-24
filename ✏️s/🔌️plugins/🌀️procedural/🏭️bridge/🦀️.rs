//! 🏭️ Production mutation bridge for `✏️s/🔌️plugins/🌀️procedural`.
//!
//! `test inventory` runs this and compares what it prints against each owner manifest and its claimed test
//! catalog. Every row is read out of a production aggregate's `DESCRIPTORS`, which the `dsl::Mutations` derive
//! generates from the mutation leaves themselves; a subset's inventory is the descriptors whose leaf `owner` lies
//! inside that subset's owner, so a verb dispatched in production and absent from the manifest is a breach.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts — the caller.

extern crate semio_framework_os_kernel as protocol;

use protocol::{Mutation, MutationLeafDescriptor};

/// 🧬️ One aggregate's descriptors, for the snapshot its `#[mutations(snapshot = …)]` names.
fn descriptors<S, M: Mutation<S>>() -> &'static [MutationLeafDescriptor] {
    M::DESCRIPTORS
}

/// 🧭️ Every mutation aggregate the artifact crates below mount, with its type name.
const AGGREGATES: &[(&str, fn() -> &'static [MutationLeafDescriptor])] = &[
    ("Generation2dTransientMutation", descriptors::<semio_s_artifact_procedural_generation2d::editor::generation2d::transient::Generation2dTransient, semio_s_artifact_procedural_generation2d::editor::generation2d::transient::Generation2dTransientMutation>),
    ("Generation2dMutation", descriptors::<semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshot, semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::schema::mutations::Generation2dMutation>),
    ("Generation3dConfigMutation", descriptors::<semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig, semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfigMutation>),
    ("Generation3dTransientMutation", descriptors::<semio_s_artifact_procedural_generation3d::editor::generation3d::transient::Generation3dTransient, semio_s_artifact_procedural_generation3d::editor::generation3d::transient::Generation3dTransientMutation>),
    ("Generation3dMutation", descriptors::<semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshot, semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::Generation3dMutation>),
    ("Generation3dViewConfigMutation", descriptors::<semio_s_artifact_procedural_generation3d::viewer::generation3d::config::Generation3dViewConfig, semio_s_artifact_procedural_generation3d::viewer::generation3d::config::Generation3dViewConfigMutation>),
    ("Generation3dViewPresenceMutation", descriptors::<semio_s_artifact_procedural_generation3d::viewer::generation3d::presence::Generation3dViewPresence, semio_s_artifact_procedural_generation3d::viewer::generation3d::presence::Generation3dViewPresenceMutation>),
    ("Generation3dViewTransientMutation", descriptors::<semio_s_artifact_procedural_generation3d::viewer::generation3d::transient::Generation3dViewTransient, semio_s_artifact_procedural_generation3d::viewer::generation3d::transient::Generation3dViewTransientMutation>),
];

/// 🗺️ Manifest coordinate (artifact, standard, subset, state-lane surface or `""` for the document) → the owner
/// directory whose leaves it measures, and, where several subsets share one owner directory, the aggregate type-name
/// prefix that is that subset's dispatch.
const COORDINATES: &[(&str, &str, &str, &str, &str, &str)] = &[
    ("s.procedural.generation2d", "1", "any", "", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.procedural.generation2d", "1", "any", "✏️editor/🫧️transient", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient", ""),
    ("s.procedural.generation3d", "1", "any", "", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.procedural.generation3d", "1", "any", "✏️editor/🎚️config", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", ""),
    ("s.procedural.generation3d", "1", "any", "✏️editor/🫧️transient", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient", ""),
    ("s.procedural.generation3d", "1", "any", "👁️viewer/🎚️config", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config", ""),
    ("s.procedural.generation3d", "1", "any", "👁️viewer/👥️presence", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence", ""),
    ("s.procedural.generation3d", "1", "any", "👁️viewer/🫧️transient", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🫧️transient", ""),
];

/// 🎚️ The subset surface directories whose state lanes (config, presence, transient) are never document dispatch.
const SURFACE_DIRS: &[&str] = &["👁️viewer", "✏️editor"];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (command, artifact, standard, subset, surface) = match args.as_slice() {
        [command, artifact, standard, subset] => (command, artifact, standard, subset, ""),
        [command, artifact, standard, subset, surface] => (command, artifact, standard, subset, surface.as_str()),
        _ => {
            eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");
            std::process::exit(2);
        }
    };
    let Some((_, _, _, _, owner, prefix)) = COORDINATES.iter().find(|(a, s, u, f, _, _)| command == "list-mutations" && a == artifact && s == standard && u == subset && *f == surface) else {
        eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");
        std::process::exit(2);
    };
    let owner_prefix = format!("{owner}/");
    let in_state_lane = |leaf: &str| surface.is_empty() && leaf.split('/').any(|segment| SURFACE_DIRS.contains(&segment));
    let mut rows: Vec<pack::JsonValue> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for descriptor in AGGREGATES.iter().filter(|(name, _)| name.starts_with(prefix)).flat_map(|(_, aggregate)| aggregate().iter()) {
        if !descriptor.owner.starts_with(&owner_prefix) || in_state_lane(&descriptor.owner[owner_prefix.len()..]) || seen.contains(&descriptor.semantic_kind) {
            continue;
        }
        seen.push(descriptor.semantic_kind);
        rows.push(pack::json_object([
            ("id".to_string(), pack::JsonValue::from(descriptor.semantic_kind)),
            ("variant".to_string(), pack::JsonValue::from(descriptor.aggregate_variant)),
            ("outcomes".to_string(), pack::json_array(descriptor.outcome_classes.iter().map(|class| pack::JsonValue::from(class.as_str())))),
        ]));
    }
    let mut fields = vec![
        ("schema".to_string(), pack::JsonValue::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), pack::JsonValue::from(artifact.as_str())),
        ("standard".to_string(), pack::JsonValue::from(standard.as_str())),
        ("subset".to_string(), pack::JsonValue::from(subset.as_str())),
    ];
    if !surface.is_empty() {
        fields.push(("surface".to_string(), pack::JsonValue::from(surface)));
    }
    fields.extend([
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("producedBy".to_string(), pack::JsonValue::from("semio-procedural-mutation-bridge")),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    let out = pack::json_object(fields);
    println!("{}", pack::json_to_string(&out));
}
