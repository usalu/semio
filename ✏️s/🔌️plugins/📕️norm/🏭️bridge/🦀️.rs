//! 🏭️ Production mutation bridge for `✏️s/🔌️plugins/📕️norm`.
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
    descriptors::<semio_s_artifact_norm_din16798::standards::v1::subsets::any::schema::snapshot::Din16798Snapshot, semio_s_artifact_norm_din16798::standards::v1::subsets::any::schema::mutations::Din16798Mutation>,
    descriptors::<semio_s_artifact_norm_din18599::standards::v1::subsets::any::schema::snapshot::Din18599Snapshot, semio_s_artifact_norm_din18599::standards::v1::subsets::any::schema::mutations::Din18599Mutation>,
    descriptors::<semio_s_artifact_norm_din4108::standards::v1::subsets::any::schema::snapshot::Din4108Snapshot, semio_s_artifact_norm_din4108::standards::v1::subsets::any::schema::mutations::Din4108Mutation>,
    descriptors::<semio_s_artifact_norm_en1990::standards::v1::subsets::any::schema::snapshot::En1990Snapshot, semio_s_artifact_norm_en1990::standards::v1::subsets::any::schema::mutations::En1990Mutation>,
    descriptors::<semio_s_artifact_norm_en1991::standards::v1::subsets::any::schema::snapshot::En1991Snapshot, semio_s_artifact_norm_en1991::standards::v1::subsets::any::schema::mutations::En1991Mutation>,
    descriptors::<semio_s_artifact_norm_en1992::standards::v1::subsets::any::schema::snapshot::En1992Snapshot, semio_s_artifact_norm_en1992::standards::v1::subsets::any::schema::mutations::En1992Mutation>,
    descriptors::<semio_s_artifact_norm_en1993::standards::v1::subsets::any::schema::snapshot::En1993Snapshot, semio_s_artifact_norm_en1993::standards::v1::subsets::any::schema::mutations::En1993Mutation>,
    descriptors::<semio_s_artifact_norm_en1994::standards::v1::subsets::any::schema::snapshot::En1994Snapshot, semio_s_artifact_norm_en1994::standards::v1::subsets::any::schema::mutations::En1994Mutation>,
    descriptors::<semio_s_artifact_norm_en1995::standards::v1::subsets::any::schema::snapshot::En1995Snapshot, semio_s_artifact_norm_en1995::standards::v1::subsets::any::schema::mutations::En1995Mutation>,
    descriptors::<semio_s_artifact_norm_en1996::standards::v1::subsets::any::schema::snapshot::En1996Snapshot, semio_s_artifact_norm_en1996::standards::v1::subsets::any::schema::mutations::En1996Mutation>,
    descriptors::<semio_s_artifact_norm_en1997::standards::v1::subsets::any::schema::snapshot::En1997Snapshot, semio_s_artifact_norm_en1997::standards::v1::subsets::any::schema::mutations::En1997Mutation>,
    descriptors::<semio_s_artifact_norm_en1998::standards::v1::subsets::any::schema::snapshot::En1998Snapshot, semio_s_artifact_norm_en1998::standards::v1::subsets::any::schema::mutations::En1998Mutation>,
    descriptors::<semio_s_artifact_norm_en1999::standards::v1::subsets::any::schema::snapshot::En1999Snapshot, semio_s_artifact_norm_en1999::standards::v1::subsets::any::schema::mutations::En1999Mutation>,
    descriptors::<semio_s_artifact_norm_iso16757::standards::v1::subsets::any::schema::snapshot::Iso16757Snapshot, semio_s_artifact_norm_iso16757::standards::v1::subsets::any::schema::mutations::Iso16757Mutation>,
    descriptors::<semio_s_artifact_norm_vdi3805::standards::v1::subsets::any::schema::snapshot::Vdi3805Snapshot, semio_s_artifact_norm_vdi3805::standards::v1::subsets::any::schema::mutations::Vdi3805Mutation>,
];

/// 🗺️ Manifest coordinate → the owner directory whose leaves it measures.
const COORDINATES: &[(&str, &str, &str, &str)] = &[
    ("s.norm.din16798", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.din18599", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.din4108", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1990", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1991", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1992", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1993", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1994", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1995", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1996", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1997", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1998", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.en1999", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.iso16757", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any"),
    ("s.norm.vdi3805", "1", "any", "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any"),
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
        ("producedBy".to_string(), pack::JsonValue::from("semio-norm-mutation-bridge")),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{}", pack::json_to_string(&out));
}
