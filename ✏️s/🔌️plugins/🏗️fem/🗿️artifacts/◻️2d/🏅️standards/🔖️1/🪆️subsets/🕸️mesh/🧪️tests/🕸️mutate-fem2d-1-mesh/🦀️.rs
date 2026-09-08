//! 🏗️ `s.fem.fem2d` mesh mutation case — Rust SUBJECT adapter. Relocated out of the
//! artifact-level `mutate-fem2d-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION` so
//! this subset's own kinds have a subset-owned test.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` beside this file — a
//! second implementation of the structural model and this subset's typed mutations, written in
//! Python from this subset's committed snapshot schema, mutation grammar and specification vectors.
//! This adapter registers the SUBJECT half only: keeping oracle registrations here would put this
//! repository's answer on both sides of the comparison.
//!
//! **What the two roles each hold.** The cross-language projection is the whole model; this artifact
//! composes no digest-derived child, so nothing has to be held back. The committed `🔺️diff` — which
//! pins WHICH fields a mutation was allowed to touch — and the committed `🎯️outcome` are Rust-side
//! report shapes rather than parts of the document, so they stay asserted HERE, in role, in
//! [`subject::spec_vector`].

use semio_repo_test_host::{parse_json, Adapter, Json};

//#region 🔖️Kinds
/// 🏷️ This subset's own slice of `KINDS` in `../../🧬️schema/🧬️mutations/🦀️.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and that file's own
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against both the enum and the manifest.
const KINDS: &[&str] = &["create-node", "delete-node", "create-element", "delete-element", "replace-element", "create-section", "delete-section", "replace-section", "create-region", "delete-region", "replace-region"];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect, so
/// [`law::mutation_is_observable`] must not demand one of them.
/// Empty: every committed vector of this subset is forward and message-free, so every kind's effect
/// is visible.
const UNOBSERVABLE: &[&str] = &[

];

/// 🧫️ The same derived timber-portal-frame model every fem2d mutation subset case shares, as its
/// own local copy — see `../../../🌐️any/🧪️tests/🔄️round-trips-the-committed-document/🥒️.feature`
/// for the full derivation provenance.
#[cfg(feature = "sut")]
const DERIVED_ASSET: &str = "local://🏗️timber-portal-frame.snapshot.json";
//#endregion 🔖️Kinds

//#region 🔖️Scenarios
/// 📇️ Every FORWARD committed vector this subset owns, as `(scenario id, kind)`. Two per kind:
/// the pre-existing `spec-vector-<kind>` and the steel-frame `frame-vector-<kind>`.
const COMMITTED: &[(&str, &str)] = &[
    ("spec-vector-create-element", "create-element"),
    ("frame-vector-create-element", "create-element"),
    ("spec-vector-create-node", "create-node"),
    ("frame-vector-create-node", "create-node"),
    ("spec-vector-create-region", "create-region"),
    ("frame-vector-create-region", "create-region"),
    ("spec-vector-create-section", "create-section"),
    ("frame-vector-create-section", "create-section"),
    ("spec-vector-delete-element", "delete-element"),
    ("frame-vector-delete-element", "delete-element"),
    ("spec-vector-delete-node", "delete-node"),
    ("frame-vector-delete-node", "delete-node"),
    ("spec-vector-delete-region", "delete-region"),
    ("frame-vector-delete-region", "delete-region"),
    ("spec-vector-delete-section", "delete-section"),
    ("frame-vector-delete-section", "delete-section"),
    ("spec-vector-replace-element", "replace-element"),
    ("frame-vector-replace-element", "replace-element"),
    ("spec-vector-replace-region", "replace-region"),
    ("frame-vector-replace-region", "replace-region"),
    ("spec-vector-replace-section", "replace-section"),
    ("frame-vector-replace-section", "replace-section"),
];

/// 📇️ Every REFUSAL or no-op vector this subset owns, numbered in the catalog's own order.
const REFUSED: &[(&str, &str)] = &[
    ("reject-create-element-1", "create-element"),
    ("reject-create-node-1", "create-node"),
    ("reject-create-region-1", "create-region"),
    ("reject-create-region-2", "create-region"),
    ("reject-create-region-3", "create-region"),
    ("reject-create-section-1", "create-section"),
    ("reject-create-section-2", "create-section"),
    ("reject-delete-element-1", "delete-element"),
    ("reject-delete-element-2", "delete-element"),
    ("reject-delete-node-1", "delete-node"),
    ("reject-delete-region-1", "delete-region"),
    ("reject-delete-region-2", "delete-region"),
    ("reject-delete-section-1", "delete-section"),
    ("reject-delete-section-2", "delete-section"),
    ("reject-replace-element-1", "replace-element"),
    ("reject-replace-element-2", "replace-element"),
    ("reject-replace-element-3", "replace-element"),
    ("reject-replace-region-1", "replace-region"),
    ("reject-replace-region-2", "replace-region"),
    ("reject-replace-region-3", "replace-region"),
    ("reject-replace-region-4", "replace-region"),
    ("reject-replace-section-1", "replace-section"),
    ("reject-replace-section-2", "replace-section"),
    ("reject-replace-section-3", "replace-section"),
];
//#endregion 🔖️Scenarios

//#region 🔖️Fixtures
/// 🧫️ One kind's committed `(before, mutation, after, diff, outcome)` specification vector, read
/// literally via `include_str!`. This IS the independently handcrafted evidence this case rests on —
/// never recomputed here, never restated as a Rust literal.
struct Vector {
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    diff: Option<&'static str>,
    outcome: &'static str,
}

fn vector(scenario: &str) -> Vector {
    match scenario {
        "spec-vector-create-element" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends-bar-e2-between-fc1c09/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends-bar-e2-between-fc1c09/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends-bar-e2-between-fc1c09/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends-bar-e2-between-fc1c09/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends-bar-e2-between-fc1c09/🎯️outcome/🔣️.json"),
        },
        "frame-vector-create-element" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/📐️braces-the-upper-d96634/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/📐️braces-the-upper-d96634/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/📐️braces-the-upper-d96634/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/📐️braces-the-upper-d96634/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/📐️braces-the-upper-d96634/🎯️outcome/🔣️.json"),
        },
        "reject-create-element-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚫️rejects-a-dangling-b9e64c/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚫️rejects-a-dangling-b9e64c/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚫️rejects-a-dangling-b9e64c/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚫️rejects-a-dangling-b9e64c/🎯️outcome/🔣️.json"),
        },
        "spec-vector-create-node" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-node-n3/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-node-n3/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-node-n3/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-node-n3/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-node-n3/🎯️outcome/🔣️.json"),
        },
        "frame-vector-create-node" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏢️appends-the-canopy-226a20/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏢️appends-the-canopy-226a20/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏢️appends-the-canopy-226a20/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏢️appends-the-canopy-226a20/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏢️appends-the-canopy-226a20/🎯️outcome/🔣️.json"),
        },
        "reject-create-node-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚫️rejects-a-duplicate-eb0df0/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚫️rejects-a-duplicate-eb0df0/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚫️rejects-a-duplicate-eb0df0/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚫️rejects-a-duplicate-eb0df0/🎯️outcome/🔣️.json"),
        },
        "spec-vector-create-region" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🧱️appends-a-solid-d78275/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🧱️appends-a-solid-d78275/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🧱️appends-a-solid-d78275/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🧱️appends-a-solid-d78275/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🧱️appends-a-solid-d78275/🎯️outcome/🔣️.json"),
        },
        "frame-vector-create-region" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🏢️infills-the-upper-6cc520/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🏢️infills-the-upper-6cc520/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🏢️infills-the-upper-6cc520/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🏢️infills-the-upper-6cc520/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🏢️infills-the-upper-6cc520/🎯️outcome/🔣️.json"),
        },
        "reject-create-region-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🚫️rejects-a-duplicate-11ca0d/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🚫️rejects-a-duplicate-11ca0d/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🚫️rejects-a-duplicate-11ca0d/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🚫️rejects-a-duplicate-11ca0d/🎯️outcome/🔣️.json"),
        },
        "reject-create-region-2" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/📐️denies-two-point-99954a/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/📐️denies-two-point-99954a/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/📐️denies-two-point-99954a/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/📐️denies-two-point-99954a/🎯️outcome/🔣️.json"),
        },
        "reject-create-region-3" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🕳️denies-loose-hole-d9efa1/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🕳️denies-loose-hole-d9efa1/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🕳️denies-loose-hole-d9efa1/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🕳️denies-loose-hole-d9efa1/🎯️outcome/🔣️.json"),
        },
        "spec-vector-create-section" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/📐️appends-the-ipe300-profile/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/📐️appends-the-ipe300-profile/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/📐️appends-the-ipe300-profile/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/📐️appends-the-ipe300-profile/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/📐️appends-the-ipe300-profile/🎯️outcome/🔣️.json"),
        },
        "frame-vector-create-section" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/➕️adds-the-hea220-dfdf34/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/➕️adds-the-hea220-dfdf34/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/➕️adds-the-hea220-dfdf34/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/➕️adds-the-hea220-dfdf34/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/➕️adds-the-hea220-dfdf34/🎯️outcome/🔣️.json"),
        },
        "reject-create-section-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚫️rejects-a-duplicate-e91bc7/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚫️rejects-a-duplicate-e91bc7/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚫️rejects-a-duplicate-e91bc7/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚫️rejects-a-duplicate-e91bc7/🎯️outcome/🔣️.json"),
        },
        "reject-create-section-2" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/⚗️denies-zero-area-58b5ca/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/⚗️denies-zero-area-58b5ca/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/⚗️denies-zero-area-58b5ca/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/⚗️denies-zero-area-58b5ca/🎯️outcome/🔣️.json"),
        },
        "spec-vector-delete-element" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes-bar-e2-and-3c0260/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes-bar-e2-and-3c0260/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes-bar-e2-and-3c0260/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes-bar-e2-and-3c0260/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes-bar-e2-and-3c0260/🎯️outcome/🔣️.json"),
        },
        "frame-vector-delete-element" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/✂️cuts-the-lower-e4a250/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/✂️cuts-the-lower-e4a250/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/✂️cuts-the-lower-e4a250/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/✂️cuts-the-lower-e4a250/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/✂️cuts-the-lower-e4a250/🎯️outcome/🔣️.json"),
        },
        "reject-delete-element-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛔️rejects-a-missing-611215/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛔️rejects-a-missing-611215/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛔️rejects-a-missing-611215/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛔️rejects-a-missing-611215/🎯️outcome/🔣️.json"),
        },
        "reject-delete-element-2" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🔗️blocks-udl-a1df8e/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🔗️blocks-udl-a1df8e/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🔗️blocks-udl-a1df8e/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🔗️blocks-udl-a1df8e/🎯️outcome/🔣️.json"),
        },
        "spec-vector-delete-node" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-6eab3f/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-6eab3f/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-6eab3f/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-6eab3f/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-6eab3f/🎯️outcome/🔣️.json"),
        },
        "frame-vector-delete-node" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🗑️drops-the-spare-6c285d/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🗑️drops-the-spare-6c285d/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🗑️drops-the-spare-6c285d/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🗑️drops-the-spare-6c285d/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🗑️drops-the-spare-6c285d/🎯️outcome/🔣️.json"),
        },
        "reject-delete-node-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/⛔️rejects-a-missing-429801/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/⛔️rejects-a-missing-429801/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/⛔️rejects-a-missing-429801/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/⛔️rejects-a-missing-429801/🎯️outcome/🔣️.json"),
        },
        "spec-vector-delete-region" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🚫️removes-the-slab-and-5b301a/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🚫️removes-the-slab-and-5b301a/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🚫️removes-the-slab-and-5b301a/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🚫️removes-the-slab-and-5b301a/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🚫️removes-the-slab-and-5b301a/🎯️outcome/🔣️.json"),
        },
        "frame-vector-delete-region" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🧹️drops-the-spare-460714/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🧹️drops-the-spare-460714/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🧹️drops-the-spare-460714/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🧹️drops-the-spare-460714/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🧹️drops-the-spare-460714/🎯️outcome/🔣️.json"),
        },
        "reject-delete-region-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/⛔️rejects-a-missing-a83a6d/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/⛔️rejects-a-missing-a83a6d/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/⛔️rejects-a-missing-a83a6d/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/⛔️rejects-a-missing-a83a6d/🎯️outcome/🔣️.json"),
        },
        "reject-delete-region-2" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🔗️blocks-in-use-7c7862/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🔗️blocks-in-use-7c7862/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🔗️blocks-in-use-7c7862/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🔗️blocks-in-use-7c7862/🎯️outcome/🔣️.json"),
        },
        "spec-vector-delete-section" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-1c235a/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-1c235a/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-1c235a/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-1c235a/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-1c235a/🎯️outcome/🔣️.json"),
        },
        "frame-vector-delete-section" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/✂️drops-the-spare-dcf609/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/✂️drops-the-spare-dcf609/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/✂️drops-the-spare-dcf609/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/✂️drops-the-spare-dcf609/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/✂️drops-the-spare-dcf609/🎯️outcome/🔣️.json"),
        },
        "reject-delete-section-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛔️rejects-a-missing-dbd0a4/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛔️rejects-a-missing-dbd0a4/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛔️rejects-a-missing-dbd0a4/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛔️rejects-a-missing-dbd0a4/🎯️outcome/🔣️.json"),
        },
        "reject-delete-section-2" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🔗️blocks-in-use-0a6a3c/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🔗️blocks-in-use-0a6a3c/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🔗️blocks-in-use-0a6a3c/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🔗️blocks-in-use-0a6a3c/🎯️outcome/🔣️.json"),
        },
        "spec-vector-replace-element" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/♻️converts-beam-e1-into-a-5d21f5/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/♻️converts-beam-e1-into-a-5d21f5/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/♻️converts-beam-e1-into-a-5d21f5/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/♻️converts-beam-e1-into-a-5d21f5/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/♻️converts-beam-e1-into-a-5d21f5/🎯️outcome/🔣️.json"),
        },
        "frame-vector-replace-element" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔧️regrades-the-roof-fb20eb/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔧️regrades-the-roof-fb20eb/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔧️regrades-the-roof-fb20eb/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔧️regrades-the-roof-fb20eb/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔧️regrades-the-roof-fb20eb/🎯️outcome/🔣️.json"),
        },
        "reject-replace-element-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⛔️rejects-a-missing-bd448c/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⛔️rejects-a-missing-bd448c/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⛔️rejects-a-missing-bd448c/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⛔️rejects-a-missing-bd448c/🎯️outcome/🔣️.json"),
        },
        "reject-replace-element-2" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️denies-rename-0d46d8/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️denies-rename-0d46d8/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️denies-rename-0d46d8/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️denies-rename-0d46d8/🎯️outcome/🔣️.json"),
        },
        "reject-replace-element-3" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚫️dangling-start-cda887/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚫️dangling-start-cda887/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚫️dangling-start-cda887/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚫️dangling-start-cda887/🎯️outcome/🔣️.json"),
        },
        "spec-vector-replace-region" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪜️punches-a-stair-f7b3b1/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪜️punches-a-stair-f7b3b1/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪜️punches-a-stair-f7b3b1/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪜️punches-a-stair-f7b3b1/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪜️punches-a-stair-f7b3b1/🎯️outcome/🔣️.json"),
        },
        "frame-vector-replace-region" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪟️widens-the-window-09a8ec/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪟️widens-the-window-09a8ec/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪟️widens-the-window-09a8ec/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪟️widens-the-window-09a8ec/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪟️widens-the-window-09a8ec/🎯️outcome/🔣️.json"),
        },
        "reject-replace-region-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/⛔️rejects-a-missing-6e0d70/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/⛔️rejects-a-missing-6e0d70/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/⛔️rejects-a-missing-6e0d70/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/⛔️rejects-a-missing-6e0d70/🎯️outcome/🔣️.json"),
        },
        "reject-replace-region-2" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪪️denies-rename-574c91/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪪️denies-rename-574c91/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪪️denies-rename-574c91/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪪️denies-rename-574c91/🎯️outcome/🔣️.json"),
        },
        "reject-replace-region-3" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/👻️dangling-mat-7ef81b/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/👻️dangling-mat-7ef81b/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/👻️dangling-mat-7ef81b/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/👻️dangling-mat-7ef81b/🎯️outcome/🔣️.json"),
        },
        "reject-replace-region-4" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/📐️denies-zero-thick-7d805e/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/📐️denies-zero-thick-7d805e/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/📐️denies-zero-thick-7d805e/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/📐️denies-zero-thick-7d805e/🎯️outcome/🔣️.json"),
        },
        "spec-vector-replace-section" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/💪️stiffens-ipe200-with-5e9c08/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/💪️stiffens-ipe200-with-5e9c08/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/💪️stiffens-ipe200-with-5e9c08/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/💪️stiffens-ipe200-with-5e9c08/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/💪️stiffens-ipe200-with-5e9c08/🎯️outcome/🔣️.json"),
        },
        "frame-vector-replace-section" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🛠️thickens-the-chs-e235a5/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🛠️thickens-the-chs-e235a5/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🛠️thickens-the-chs-e235a5/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🛠️thickens-the-chs-e235a5/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🛠️thickens-the-chs-e235a5/🎯️outcome/🔣️.json"),
        },
        "reject-replace-section-1" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⛔️rejects-a-missing-b468f4/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⛔️rejects-a-missing-b468f4/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⛔️rejects-a-missing-b468f4/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⛔️rejects-a-missing-b468f4/🎯️outcome/🔣️.json"),
        },
        "reject-replace-section-2" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️denies-rename-1d02dd/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️denies-rename-1d02dd/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️denies-rename-1d02dd/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️denies-rename-1d02dd/🎯️outcome/🔣️.json"),
        },
        "reject-replace-section-3" => Vector {
            before: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⚗️denies-zero-iy-404e31/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⚗️denies-zero-iy-404e31/🦠️mutation/🔣️.json"),
            after: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⚗️denies-zero-iy-404e31/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⚗️denies-zero-iy-404e31/🎯️outcome/🔣️.json"),
        },
        other => panic!("🕸️mutate-fem2d-1-mesh: no committed specification vector is registered for scenario {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-fem2d-1-mesh: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, DERIVED_ASSET, UNOBSERVABLE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use crate::standards::v1::subsets::any::schema::mutations::fem2d_mutation_report_json;

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
    /// The projection carries BOTH models: projecting only the restored one would make every row
    /// project the same value and the differential would be vacuous.
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

    /// 📐️ Replays one committed handcrafted specification vector, addressed by SCENARIO id rather
    /// than by kind — every kind now carries several. This is where the evidence the case carried
    /// before the relocation still lives, undiminished: the applied model is held to the committed
    /// after-snapshot, the produced delta to the committed `🔺️diff`, and the diagnostics to the
    /// committed `🎯️outcome`.
    pub fn committed_vector(kind: &'static str, scenario: &str) -> impl Fn(&Context) -> Result<Outcome, String> {
        let scenario = scenario.to_string();
        move |_ctx: &Context| {
            let committed = vector(&scenario);
            let report = report_of(&scenario, committed.before, committed.mutation, committed.after)?;
            let applied = member(&report, "snapshot")?;
            if let Some(first) = law::divergence(applied, member(&report, "expectedSnapshot")?) {
                return Err(format!("{scenario}: the applied model is not the committed after-snapshot — {first}"));
            }
            match committed.diff {
                Some(text) => {
                    if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(text)) {
                        return Err(format!("{scenario}: the produced delta is not the committed 🔺️diff — {first}"));
                    }
                }
                None => return Err(format!("{scenario}: a forward vector must carry a committed 🔺️diff")),
            }
            declared_outcome_holds(kind, &members(&report, "messages")?, &canonical(committed.outcome))?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            touches_one(&scenario, kind, member(&report, "base")?, applied)?;
            law::inverse_restores(kind, member(&report, "inverseSnapshot")?, member(&report, "base")?)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// 🚫️ Replays one committed REFUSAL or no-op vector. The projection carries the diagnostic
    /// beside the model on purpose: two implementations that merely both decline to move a document
    /// agree vacuously, and what these rows are evidence for is that they refuse for the SAME
    /// reason — same code, same level, same address.
    pub fn reject(kind: &'static str, scenario: &str) -> impl Fn(&Context) -> Result<Outcome, String> {
        let scenario = scenario.to_string();
        move |_ctx: &Context| {
            let committed = vector(&scenario);
            if committed.diff.is_some() {
                return Err(format!("{scenario}: a refusal vector carries 🔺️diff/🚫️.absent, never a committed delta"));
            }
            let report = report_of(&scenario, committed.before, committed.mutation, committed.after)?;
            let applied = member(&report, "snapshot")?;
            if let Some(first) = law::divergence(applied, member(&report, "base")?) {
                return Err(format!("{scenario}: a refused or no-op mutation must leave the model exactly where it was — {first}"));
            }
            let raised = members(&report, "messages")?;
            let first = raised.first().ok_or_else(|| format!("{scenario}: the vector declares a refusal, the implementation raised nothing"))?;
            declared_outcome_holds(kind, &raised, &canonical(committed.outcome))?;
            let refusal = Json::Object(vec![
                ("code".to_string(), Json::String(first.str("code"))),
                ("level".to_string(), Json::String(level_of(&first.str("level")))),
                ("target".to_string(), Json::Array(strings(first, "target").into_iter().map(Json::String).collect())),
            ]);
            let projection = Json::Object(vec![("model".to_string(), applied.clone()), ("refusal".to_string(), refusal)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }
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
        }
        for &(scenario, kind) in COMMITTED {
            built = built.subject(scenario, subject::committed_vector(kind, scenario));
        }
        for &(scenario, kind) in REFUSED {
            built = built.subject(scenario, subject::reject(kind, scenario));
        }
        return built;
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = (KINDS, UNOBSERVABLE, COMMITTED, REFUSED, vector as fn(&str) -> Vector);
        built
    }
}
//#endregion 🔖️Registration
