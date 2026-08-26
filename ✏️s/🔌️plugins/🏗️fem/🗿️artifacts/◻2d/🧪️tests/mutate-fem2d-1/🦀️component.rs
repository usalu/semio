//! 🏗️ `s.fem.fem2d` exhaustive mutation case — Rust SUBJECT adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` beside this file —
//! a second implementation of the structural model and all twenty-five typed mutations, written in
//! Python from this subset's committed snapshot schema, mutation grammar and specification vectors.
//! This adapter registers the SUBJECT half only: keeping oracle registrations here would put this
//! repository's answer on both sides of the comparison.
//!
//! What the vocabulary edits is the MODEL, not the analysis: nine id-keyed collections and one
//! settings record. No finite-element solver defines this document and none reads `.dsl.semio`.
//!
//! **What the two roles each hold.** The cross-language projection is the whole model; this artifact
//! composes no digest-derived child, so nothing has to be held back. The committed `🔺️diff` — which
//! pins WHICH fields a mutation was allowed to touch — and the committed `🎯️outcome` are Rust-side
//! report shapes rather than parts of the document, so they stay asserted HERE, in role, in
//! [`subject::spec_vector`], exactly as before the conversion. So does the `.dsl.semio` carrier's
//! fixpoint law and its agreement with the binary pack codec.

use semio_repo_test_host::{parse_json, Adapter, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and that file's own
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against both the enum and the manifest.
const KINDS: &[&str] = &[
    "create-node",
    "delete-node",
    "create-element",
    "delete-element",
    "replace-element",
    "create-material",
    "delete-material",
    "replace-material",
    "create-section",
    "delete-section",
    "replace-section",
    "create-support",
    "delete-support",
    "replace-support",
    "create-region",
    "delete-region",
    "replace-region",
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect, so
/// [`law::mutation_is_observable`] must not demand one of them.
/// Empty: all 25 committed vectors are forward and message-free, so every kind's effect is visible.
const UNOBSERVABLE: &[&str] = &[

];

/// 🗣️ The real committed document this artifact ships as its own example.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";

/// 🧫️ That same frame as a snapshot, with seven unreferenced spares appended so every `delete-` and
/// `replace-` verb has an unambiguous trailing target. Derived once, provenance recorded in the
/// feature description; every spare comes from a committed specification vector of this subset.
#[cfg(feature = "sut")]
const DERIVED_ASSET: &str = "local://🏗️timber-portal-frame.snapshot.json";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ One kind's committed `(before, mutation, after, diff, outcome)` specification vector, read
/// literally via `include_str!`. This IS the independently handcrafted evidence the no-oracle decision
/// rests on — never recomputed here, never restated as a Rust literal.
struct Vector {
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    diff: &'static str,
    outcome: &'static str,
}

fn vector(kind: &str) -> Vector {
    match kind {
        "create-node" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱⚪️create-node/🧪️tests/appends-node-n3/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱⚪️create-node/🧪️tests/appends-node-n3/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱⚪️create-node/🧪️tests/appends-node-n3/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱⚪️create-node/🧪️tests/appends-node-n3/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱⚪️create-node/🧪️tests/appends-node-n3/🎯️outcome/🔣️component.json"),
        },
        "delete-node" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑⚪️delete-node/🧪️tests/removes-node-n3-without-cascading-to-its-support/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑⚪️delete-node/🧪️tests/removes-node-n3-without-cascading-to-its-support/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑⚪️delete-node/🧪️tests/removes-node-n3-without-cascading-to-its-support/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑⚪️delete-node/🧪️tests/removes-node-n3-without-cascading-to-its-support/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑⚪️delete-node/🧪️tests/removes-node-n3-without-cascading-to-its-support/🎯️outcome/🔣️component.json"),
        },
        "create-element" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧩️create-element/🧪️tests/appends-bar-e2-between-n2-and-n3/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧩️create-element/🧪️tests/appends-bar-e2-between-n2-and-n3/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧩️create-element/🧪️tests/appends-bar-e2-between-n2-and-n3/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧩️create-element/🧪️tests/appends-bar-e2-between-n2-and-n3/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧩️create-element/🧪️tests/appends-bar-e2-between-n2-and-n3/🎯️outcome/🔣️component.json"),
        },
        "delete-element" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧩️delete-element/🧪️tests/removes-bar-e2-and-keeps-its-end-nodes/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧩️delete-element/🧪️tests/removes-bar-e2-and-keeps-its-end-nodes/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧩️delete-element/🧪️tests/removes-bar-e2-and-keeps-its-end-nodes/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧩️delete-element/🧪️tests/removes-bar-e2-and-keeps-its-end-nodes/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧩️delete-element/🧪️tests/removes-bar-e2-and-keeps-its-end-nodes/🎯️outcome/🔣️component.json"),
        },
        "replace-element" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧩️replace-element/🧪️tests/converts-beam-e1-into-a-bar-in-place/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧩️replace-element/🧪️tests/converts-beam-e1-into-a-bar-in-place/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧩️replace-element/🧪️tests/converts-beam-e1-into-a-bar-in-place/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧩️replace-element/🧪️tests/converts-beam-e1-into-a-bar-in-place/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧩️replace-element/🧪️tests/converts-beam-e1-into-a-bar-in-place/🎯️outcome/🔣️component.json"),
        },
        "create-material" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧱️create-material/🧪️tests/appends-concrete-c30/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧱️create-material/🧪️tests/appends-concrete-c30/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧱️create-material/🧪️tests/appends-concrete-c30/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧱️create-material/🧪️tests/appends-concrete-c30/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🧱️create-material/🧪️tests/appends-concrete-c30/🎯️outcome/🔣️component.json"),
        },
        "delete-material" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧱️delete-material/🧪️tests/removes-the-unreferenced-timber-material/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧱️delete-material/🧪️tests/removes-the-unreferenced-timber-material/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧱️delete-material/🧪️tests/removes-the-unreferenced-timber-material/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧱️delete-material/🧪️tests/removes-the-unreferenced-timber-material/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🧱️delete-material/🧪️tests/removes-the-unreferenced-timber-material/🎯️outcome/🔣️component.json"),
        },
        "replace-material" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧱️replace-material/🧪️tests/restates-steel-as-s355-in-its-original-slot/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧱️replace-material/🧪️tests/restates-steel-as-s355-in-its-original-slot/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧱️replace-material/🧪️tests/restates-steel-as-s355-in-its-original-slot/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧱️replace-material/🧪️tests/restates-steel-as-s355-in-its-original-slot/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🧱️replace-material/🧪️tests/restates-steel-as-s355-in-its-original-slot/🎯️outcome/🔣️component.json"),
        },
        "create-section" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-section/🧪️tests/appends-the-ipe300-profile/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-section/🧪️tests/appends-the-ipe300-profile/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-section/🧪️tests/appends-the-ipe300-profile/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-section/🧪️tests/appends-the-ipe300-profile/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-section/🧪️tests/appends-the-ipe300-profile/🎯️outcome/🔣️component.json"),
        },
        "delete-section" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📐️delete-section/🧪️tests/removes-the-spare-hollow-section/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📐️delete-section/🧪️tests/removes-the-spare-hollow-section/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📐️delete-section/🧪️tests/removes-the-spare-hollow-section/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📐️delete-section/🧪️tests/removes-the-spare-hollow-section/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📐️delete-section/🧪️tests/removes-the-spare-hollow-section/🎯️outcome/🔣️component.json"),
        },
        "replace-section" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁📐️replace-section/🧪️tests/stiffens-ipe200-with-a-reinforced-profile/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁📐️replace-section/🧪️tests/stiffens-ipe200-with-a-reinforced-profile/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁📐️replace-section/🧪️tests/stiffens-ipe200-with-a-reinforced-profile/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁📐️replace-section/🧪️tests/stiffens-ipe200-with-a-reinforced-profile/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁📐️replace-section/🧪️tests/stiffens-ipe200-with-a-reinforced-profile/🎯️outcome/🔣️component.json"),
        },
        "create-support" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🛡️create-support/🧪️tests/adds-a-vertical-roller-at-node-n2/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🛡️create-support/🧪️tests/adds-a-vertical-roller-at-node-n2/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🛡️create-support/🧪️tests/adds-a-vertical-roller-at-node-n2/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🛡️create-support/🧪️tests/adds-a-vertical-roller-at-node-n2/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🛡️create-support/🧪️tests/adds-a-vertical-roller-at-node-n2/🎯️outcome/🔣️component.json"),
        },
        "delete-support" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-support/🧪️tests/releases-the-roller-at-node-n2/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-support/🧪️tests/releases-the-roller-at-node-n2/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-support/🧪️tests/releases-the-roller-at-node-n2/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-support/🧪️tests/releases-the-roller-at-node-n2/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-support/🧪️tests/releases-the-roller-at-node-n2/🎯️outcome/🔣️component.json"),
        },
        "replace-support" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-support/🧪️tests/upgrades-the-roller-at-n2-to-a-full-fixity/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-support/🧪️tests/upgrades-the-roller-at-n2-to-a-full-fixity/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-support/🧪️tests/upgrades-the-roller-at-n2-to-a-full-fixity/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-support/🧪️tests/upgrades-the-roller-at-n2-to-a-full-fixity/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-support/🧪️tests/upgrades-the-roller-at-n2-to-a-full-fixity/🎯️outcome/🔣️component.json"),
        },
        "create-region" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🗺️create-region/🧪️tests/appends-a-solid-rectangular-slab/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🗺️create-region/🧪️tests/appends-a-solid-rectangular-slab/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🗺️create-region/🧪️tests/appends-a-solid-rectangular-slab/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🗺️create-region/🧪️tests/appends-a-solid-rectangular-slab/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🗺️create-region/🧪️tests/appends-a-solid-rectangular-slab/🎯️outcome/🔣️component.json"),
        },
        "delete-region" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🗺️delete-region/🧪️tests/removes-the-slab-and-keeps-its-material/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🗺️delete-region/🧪️tests/removes-the-slab-and-keeps-its-material/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🗺️delete-region/🧪️tests/removes-the-slab-and-keeps-its-material/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🗺️delete-region/🧪️tests/removes-the-slab-and-keeps-its-material/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🗺️delete-region/🧪️tests/removes-the-slab-and-keeps-its-material/🎯️outcome/🔣️component.json"),
        },
        "replace-region" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🗺️replace-region/🧪️tests/punches-a-stair-opening-through-the-slab/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🗺️replace-region/🧪️tests/punches-a-stair-opening-through-the-slab/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🗺️replace-region/🧪️tests/punches-a-stair-opening-through-the-slab/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🗺️replace-region/🧪️tests/punches-a-stair-opening-through-the-slab/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁🗺️replace-region/🧪️tests/punches-a-stair-opening-through-the-slab/🎯️outcome/🔣️component.json"),
        },
        "create-load-case" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱📋️create-load-case/🧪️tests/appends-a-live-case-carrying-one-nodal-load/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱📋️create-load-case/🧪️tests/appends-a-live-case-carrying-one-nodal-load/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱📋️create-load-case/🧪️tests/appends-a-live-case-carrying-one-nodal-load/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱📋️create-load-case/🧪️tests/appends-a-live-case-carrying-one-nodal-load/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱📋️create-load-case/🧪️tests/appends-a-live-case-carrying-one-nodal-load/🎯️outcome/🔣️component.json"),
        },
        "delete-load-case" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📋️delete-load-case/🧪️tests/removes-the-live-case-together-with-its-loads/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📋️delete-load-case/🧪️tests/removes-the-live-case-together-with-its-loads/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📋️delete-load-case/🧪️tests/removes-the-live-case-together-with-its-loads/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📋️delete-load-case/🧪️tests/removes-the-live-case-together-with-its-loads/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑📋️delete-load-case/🧪️tests/removes-the-live-case-together-with-its-loads/🎯️outcome/🔣️component.json"),
        },
        "add-load" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-load/🧪️tests/appends-a-member-udl-to-the-dead-case/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-load/🧪️tests/appends-a-member-udl-to-the-dead-case/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-load/🧪️tests/appends-a-member-udl-to-the-dead-case/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-load/🧪️tests/appends-a-member-udl-to-the-dead-case/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-load/🧪️tests/appends-a-member-udl-to-the-dead-case/🎯️outcome/🔣️component.json"),
        },
        "remove-load" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-load/🧪️tests/strips-the-trailing-member-udl-from-the-dead-case/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-load/🧪️tests/strips-the-trailing-member-udl-from-the-dead-case/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-load/🧪️tests/strips-the-trailing-member-udl-from-the-dead-case/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-load/🧪️tests/strips-the-trailing-member-udl-from-the-dead-case/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-load/🧪️tests/strips-the-trailing-member-udl-from-the-dead-case/🎯️outcome/🔣️component.json"),
        },
        "change-load-case-self-weight" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖change-load-case-self-weight/🧪️tests/switches-self-weight-on-for-the-dead-case/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖change-load-case-self-weight/🧪️tests/switches-self-weight-on-for-the-dead-case/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖change-load-case-self-weight/🧪️tests/switches-self-weight-on-for-the-dead-case/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖change-load-case-self-weight/🧪️tests/switches-self-weight-on-for-the-dead-case/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖change-load-case-self-weight/🧪️tests/switches-self-weight-on-for-the-dead-case/🎯️outcome/🔣️component.json"),
        },
        "create-combination" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🔗️create-combination/🧪️tests/appends-an-uls-combination-over-both-cases/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🔗️create-combination/🧪️tests/appends-an-uls-combination-over-both-cases/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🔗️create-combination/🧪️tests/appends-an-uls-combination-over-both-cases/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🔗️create-combination/🧪️tests/appends-an-uls-combination-over-both-cases/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱🔗️create-combination/🧪️tests/appends-an-uls-combination-over-both-cases/🎯️outcome/🔣️component.json"),
        },
        "delete-combination" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🔗️delete-combination/🧪️tests/removes-the-uls-combination-and-keeps-both-cases/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🔗️delete-combination/🧪️tests/removes-the-uls-combination-and-keeps-both-cases/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🔗️delete-combination/🧪️tests/removes-the-uls-combination-and-keeps-both-cases/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🔗️delete-combination/🧪️tests/removes-the-uls-combination-and-keeps-both-cases/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑🔗️delete-combination/🧪️tests/removes-the-uls-combination-and-keeps-both-cases/🎯️outcome/🔣️component.json"),
        },
        "update-analysis-settings" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛update-analysis-settings/🧪️tests/doubles-the-modal-count-and-halves-the-deformation-scale/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛update-analysis-settings/🧪️tests/doubles-the-modal-count-and-halves-the-deformation-scale/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛update-analysis-settings/🧪️tests/doubles-the-modal-count-and-halves-the-deformation-scale/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛update-analysis-settings/🧪️tests/doubles-the-modal-count-and-halves-the-deformation-scale/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛update-analysis-settings/🧪️tests/doubles-the-modal-count-and-halves-the-deformation-scale/🎯️outcome/🔣️component.json"),
        },
        other => panic!("mutate-fem2d-1: no committed specification vector is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-fem2d-1: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, DERIVED_ASSET, DSL_ASSET, UNOBSERVABLE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_fem::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::fem2d_mutation_report_json;
    use semio_s_plugin_fem::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::fem2d_identity_report_json;

    //#region 🔖️Report
    /// 📋️ One member of the production bridge's report, named in the error when it is absent — never
    /// defaulted, because a silently missing member would turn every comparison below into a comparison
    /// of two empty values.
    fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
        report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
    }

    /// 📋️ An array member of the report, rejecting a present-but-wrong-shaped value rather than
    /// treating it as empty.
    fn members(report: &Json, key: &str) -> Result<Vec<Json>, String> {
        match member(report, key)? {
            Json::Array(items) => Ok(items.clone()),
            other => Err(format!("the report's {key:?} member is {}, not an array", other.to_string())),
        }
    }

    /// 📋️ A string member of the report, rejecting a present-but-wrong-shaped value.
    fn text(report: &Json, key: &str) -> Result<String, String> {
        match member(report, key)? {
            Json::String(value) => Ok(value.clone()),
            other => Err(format!("the report's {key:?} member is {}, not a string", other.to_string())),
        }
    }

    /// 📋️ A string array read as owned `String`s — an address list, either declared by a committed
    /// outcome or reported by a diagnostic.
    fn strings(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .iter()
            .map(|segment| match segment {
                Json::String(text) => text.clone(),
                other => other.to_string(),
            })
            .collect()
    }

    /// 🚦️ Normalizes a declared severity word. The committed outcome vectors are not consistent — some
    /// write `warn` where the serialized `Severity` writes `warning` — so the level is normalized before
    /// comparison while the `code`, which is a frozen closed-set identifier, is compared verbatim.
    fn level_of(word: &str) -> String {
        if word == "warn" {
            "warning".to_string()
        } else {
            word.to_string()
        }
    }

    /// 🎯️ Checks the produced diagnostics against the ones the committed `🎯️outcome` vector declares.
    /// A `rejected` vector declares one fault code and the offending address; an `applied` vector
    /// declares an ordered (possibly empty) message list and forbids anything at error level or worse.
    fn declared_outcome_holds(kind: &str, produced: &[Json], outcome: &Json) -> Result<(), String> {
        let codes: Vec<String> = produced.iter().map(|message| message.str("code")).collect();
        let levels: Vec<String> = produced.iter().map(|message| level_of(&message.str("level"))).collect();
        if outcome.str("status") == "rejected" {
            let expected = outcome.str("code");
            if codes != vec![expected.clone()] {
                return Err(format!("mutate-{kind}: the vector declares a rejection with code {expected:?}, the implementation raised {codes:?}"));
            }
            if !levels.iter().any(|level| level == "error" || level == "fatal") {
                return Err(format!("mutate-{kind}: the vector declares a rejection, but the implementation raised it at {levels:?} — a rejection is at least an error"));
            }
            let path = strings(outcome, "path");
            let target = strings(&produced[0], "target");
            if !path.is_empty() && target != path {
                return Err(format!("mutate-{kind}: the vector declares the offending address {path:?}, the implementation reported {target:?}"));
            }
            return Ok(());
        }
        let expected: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).collect();
        if codes != expected {
            return Err(format!("mutate-{kind}: the vector declares the diagnostics {expected:?}, the implementation raised {codes:?}"));
        }
        match levels.iter().find(|level| level.as_str() == "error" || level.as_str() == "fatal") {
            Some(level) => Err(format!("mutate-{kind}: the vector declares an applied outcome, but the implementation raised a {level}")),
            None => Ok(()),
        }
    }
    //#endregion 🔖️Report

    //#region 🔖️Plan
    /// 🧫️ The one declared fixture URI of this scenario's steps containing `needle`.
    fn uri_in(ctx: &Context, needle: &str) -> Result<String, String> {
        ctx.scenario
            .steps
            .iter()
            .flat_map(|(_, step)| step.split_whitespace())
            .find(|token| (token.starts_with("asset://") || token.starts_with("local://") || token.starts_with("shared://")) && token.contains(needle))
            .map(|token| token.to_string())
            .ok_or_else(|| format!("scenario {} declares no fixture URI containing {needle:?}", ctx.scenario.id))
    }

    /// 🧫️ The declared fixture's bytes as UTF-8 text.
    fn fixture_text(ctx: &Context, needle: &str) -> Result<String, String> {
        let uri = uri_in(ctx, needle)?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the declared fixture {uri} is not UTF-8: {error}"))
    }

    /// 🔀️ Each verb writes exactly ONE of the nine members. That is the check an after-snapshot
    /// comparison cannot make on its own: an implementation that re-derived a sibling collection on
    /// every edit — renumbering ids, re-sorting sections — would still land on the right value for
    /// the member it meant to write.
    fn touches_one(scenario: &str, kind: &str, before: &Json, after: &Json) -> Result<(), String> {
        let written = match kind {
            "update-analysis-settings" => "analysis",
            "add-load" | "remove-load" | "change-load-case-self-weight" | "create-load-case" | "delete-load-case" => "loadCases",
            _ => match kind.split_once('-').map(|(_, noun)| noun).unwrap_or_default() {
                "node" => "nodes",
                "element" => "elements",
                "region" => "regions",
                "material" => "materials",
                "section" => "sections",
                "support" => "supports",
                "combination" => "combinations",
                other => return Err(format!("{scenario}: no collection is declared for the noun {other:?}")),
            },
        };
        let moved: Vec<String> = ["nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis"]
            .iter()
            .filter(|name| before.get(name) != after.get(name))
            .map(|name| (*name).to_string())
            .collect();
        if moved != vec![written.to_string()] {
            return Err(format!("{scenario}: this verb writes {written} and nothing else, but {moved:?} moved"));
        }
        Ok(())
    }

    /// 🧭️ The one report the production bridge produces for a `(base, mutation)` pair. The bridge's
    /// third argument only feeds its `expectedSnapshot` member, which the real-model scenarios do not
    /// consult, so they pass the base for it.
    fn report_of(scenario: &str, base: &str, mutation: &str, after: &str) -> Result<Json, String> {
        parse_json(&fem2d_mutation_report_json(base, mutation, after).map_err(|error| format!("{scenario}: the input did not reach this subset's own codec: {error}"))?)
    }
    //#endregion 🔖️Plan

    //#region 🔖️Handlers
    /// 🎯️ Applies one kind to the REAL derived timber portal frame with the parameters the feature
    /// states, and asserts in role that it moved the model and wrote exactly one member.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = fixture_text(ctx, "timber-portal-frame")?;
            let report = report_of(&format!("mutate-{kind}"), &base, ctx.doc_string()?, &base)?;
            let applied = member(&report, "snapshot")?;
            let faults: Vec<String> = members(&report, "messages")?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect();
            if !faults.is_empty() {
                return Err(format!("mutate-{kind}: the feature's parameters were rejected with {faults:?}"));
            }
            law::mutation_is_observable(kind, applied, member(&report, "base")?, &[])?;
            touches_one(&format!("mutate-{kind}"), kind, member(&report, "base")?, applied)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// ↩️ Applies one kind to the REAL derived frame and then EVERY step of its OWN computed inverse.
    /// The projection carries BOTH models: projecting only the restored one would make all
    /// twenty-five rows project the same value and the differential would be vacuous.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = fixture_text(ctx, "timber-portal-frame")?;
            let report = report_of(&format!("inverse-{kind}"), &base, ctx.doc_string()?, &base)?;
            let faults: Vec<String> = members(&report, "inverseMessages")?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect();
            if !faults.is_empty() {
                return Err(format!("inverse-{kind}: an inverse step was rejected with {faults:?}, so the model never got the chance to return"));
            }
            let applied = member(&report, "snapshot")?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, &[])?;
            let restored = member(&report, "inverseSnapshot")?;
            law::inverse_restores(kind, restored, member(&report, "base")?)?;
            let projection = Json::Object(vec![("mutated".to_string(), applied.clone()), ("restored".to_string(), restored.clone())]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 📐️ Replays one committed handcrafted specification vector. This is where the evidence the case
    /// carried before the conversion still lives, undiminished: the applied model is held to the
    /// committed after-snapshot, the produced delta to the committed `🔺️diff`, and the diagnostics to
    /// the committed `🎯️outcome`.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(kind);
            let report = report_of(&format!("spec-vector-{kind}"), committed.before, committed.mutation, committed.after)?;
            let applied = member(&report, "snapshot")?;
            if let Some(first) = law::divergence(applied, member(&report, "expectedSnapshot")?) {
                return Err(format!("spec-vector-{kind}: the applied model is not the committed after-snapshot — {first}"));
            }
            if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(committed.diff)) {
                return Err(format!("spec-vector-{kind}: the produced delta is not the committed 🔺️diff — {first}"));
            }
            declared_outcome_holds(kind, &members(&report, "messages")?, &canonical(committed.outcome))?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            touches_one(&format!("spec-vector-{kind}"), kind, member(&report, "base")?, applied)?;
            law::inverse_restores(kind, member(&report, "inverseSnapshot")?, member(&report, "base")?)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// 🔁️ Two identities in one scenario, because they can only be asserted in two different places.
    ///
    /// The CARRIER identity is Rust-only and asserted here in role, on the artifact's own committed
    /// example: `law::round_trip_preserves` for the semantic half, `law::carrier_is_exact` for the
    /// byte half — deliberately the fixpoint law rather than the wave's no-pass-through tripwire,
    /// because `store::ArtifactDsl` documents canonical `print_dsl` output as a `parse_dsl` fixpoint.
    /// The pack decoding is a separate binary codec, so agreeing on one model cannot be reached by
    /// carrying text bytes across.
    ///
    /// The MODEL identity is what the Python reference can also produce: the nine members this
    /// subset's own JSON codec reads out of the derived real frame. The feature's doc string is
    /// absent here, so the derived model is read through the bridge's `base` member with the
    /// committed `update-analysis-settings` vector's own payload, which this scenario then requires
    /// to have left the model where it was.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = fixture_text(ctx, "📚️examples")?;
        let report = parse_json(&fem2d_identity_report_json(&committed).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
        let parsed = member(&report, "parsed")?;
        law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
        law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
        if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
            return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
        }
        let derived = fixture_text(ctx, "timber-portal-frame")?;
        let probe = report_of("identity-round-trip", &derived, IDENTITY_PROBE, &derived)?;
        let base = member(&probe, "base")?;
        Ok(Outcome::with_raw(base.to_string().into_bytes(), base.clone()))
    }

    /// 🧭️ A payload that reaches the bridge's decode without applying an edit: it names the analysis
    /// settings the derived model already holds, so `base` and `snapshot` agree and only the decode
    /// is exercised.
    const IDENTITY_PROBE: &str = "{\"mutation\":\"updateAnalysisSettings\",\"settings\":{\"modalCount\":3,\"bucklingCount\":3,\"deformationScale\":300.0}}";
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id. SUBJECT only:
/// the reference for every scenario here is the Python implementation beside this file, and
/// registering an oracle handler as well would put this repository's answer on both sides.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        let mut built = built;
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind));
            built = built.subject(&format!("inverse-{kind}"), subject::inverse(kind));
            built = built.subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
        }
        return built.subject("identity-round-trip", subject::round_trip);
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = (KINDS, UNOBSERVABLE, vector as fn(&str) -> Vector);
        built
    }
}
//#endregion 🔖️Registration
