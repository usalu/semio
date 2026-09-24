//! 🏭️ Production mutation bridge for `✏️s/🔌️plugins/📕️norm`.
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
    ("Din16798Mutation", descriptors::<semio_s_artifact_norm_din16798::standards::v1::subsets::any::schema::snapshot::Din16798Snapshot, semio_s_artifact_norm_din16798::standards::v1::subsets::any::schema::mutations::Din16798Mutation>),
    ("Din18599Mutation", descriptors::<semio_s_artifact_norm_din18599::standards::v1::subsets::any::schema::snapshot::Din18599Snapshot, semio_s_artifact_norm_din18599::standards::v1::subsets::any::schema::mutations::Din18599Mutation>),
    ("Din4108Mutation", descriptors::<semio_s_artifact_norm_din4108::standards::v1::subsets::any::schema::snapshot::Din4108Snapshot, semio_s_artifact_norm_din4108::standards::v1::subsets::any::schema::mutations::Din4108Mutation>),
    ("En1990Mutation", descriptors::<semio_s_artifact_norm_en1990::standards::v1::subsets::any::schema::snapshot::En1990Snapshot, semio_s_artifact_norm_en1990::standards::v1::subsets::any::schema::mutations::En1990Mutation>),
    ("En1991Mutation", descriptors::<semio_s_artifact_norm_en1991::standards::v1::subsets::any::schema::snapshot::En1991Snapshot, semio_s_artifact_norm_en1991::standards::v1::subsets::any::schema::mutations::En1991Mutation>),
    ("En1992Mutation", descriptors::<semio_s_artifact_norm_en1992::standards::v1::subsets::any::schema::snapshot::En1992Snapshot, semio_s_artifact_norm_en1992::standards::v1::subsets::any::schema::mutations::En1992Mutation>),
    ("En1993Mutation", descriptors::<semio_s_artifact_norm_en1993::standards::v1::subsets::any::schema::snapshot::En1993Snapshot, semio_s_artifact_norm_en1993::standards::v1::subsets::any::schema::mutations::En1993Mutation>),
    ("En1994Mutation", descriptors::<semio_s_artifact_norm_en1994::standards::v1::subsets::any::schema::snapshot::En1994Snapshot, semio_s_artifact_norm_en1994::standards::v1::subsets::any::schema::mutations::En1994Mutation>),
    ("En1995Mutation", descriptors::<semio_s_artifact_norm_en1995::standards::v1::subsets::any::schema::snapshot::En1995Snapshot, semio_s_artifact_norm_en1995::standards::v1::subsets::any::schema::mutations::En1995Mutation>),
    ("En1996Mutation", descriptors::<semio_s_artifact_norm_en1996::standards::v1::subsets::any::schema::snapshot::En1996Snapshot, semio_s_artifact_norm_en1996::standards::v1::subsets::any::schema::mutations::En1996Mutation>),
    ("En1997Mutation", descriptors::<semio_s_artifact_norm_en1997::standards::v1::subsets::any::schema::snapshot::En1997Snapshot, semio_s_artifact_norm_en1997::standards::v1::subsets::any::schema::mutations::En1997Mutation>),
    ("En1998Mutation", descriptors::<semio_s_artifact_norm_en1998::standards::v1::subsets::any::schema::snapshot::En1998Snapshot, semio_s_artifact_norm_en1998::standards::v1::subsets::any::schema::mutations::En1998Mutation>),
    ("En1999Mutation", descriptors::<semio_s_artifact_norm_en1999::standards::v1::subsets::any::schema::snapshot::En1999Snapshot, semio_s_artifact_norm_en1999::standards::v1::subsets::any::schema::mutations::En1999Mutation>),
    ("Iso16757Mutation", descriptors::<semio_s_artifact_norm_iso16757::standards::v1::subsets::any::schema::snapshot::Iso16757Snapshot, semio_s_artifact_norm_iso16757::standards::v1::subsets::any::schema::mutations::Iso16757Mutation>),
    ("Vdi3805Mutation", descriptors::<semio_s_artifact_norm_vdi3805::standards::v1::subsets::any::schema::snapshot::Vdi3805Snapshot, semio_s_artifact_norm_vdi3805::standards::v1::subsets::any::schema::mutations::Vdi3805Mutation>),
];

/// 🗺️ Manifest coordinate (artifact, standard, subset, state-lane surface or `""` for the document) → the owner
/// directory whose leaves it measures, and, where several subsets share one owner directory, the aggregate type-name
/// prefix that is that subset's dispatch.
const COORDINATES: &[(&str, &str, &str, &str, &str, &str)] = &[
    ("s.norm.din16798", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.din18599", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.din4108", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1990", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1991", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1992", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1993", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1994", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1995", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1996", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1997", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1998", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.en1999", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.iso16757", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
    ("s.norm.vdi3805", "1", "any", "", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any", ""),
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
        ("producedBy".to_string(), pack::JsonValue::from("semio-norm-mutation-bridge")),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    let out = pack::json_object(fields);
    println!("{}", pack::json_to_string(&out));
}
