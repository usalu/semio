//! 🏗️ `fem.fem2d` exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `fem2d-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, which also records why
//! `frame3dd`, `calculix` and `code_aster` were surveyed and DECLINED: they are solvers with no
//! editing vocabulary at all).
//!
//! This is NOT the 3D artifact's vocabulary with a coordinate removed. The planar noun here is a
//! REGION — outline plus holes, a thickness, a material and a mesh size — where the 3D artifact has an
//! extruded SOLID; nodes are `(x, y)`; materials carry no shear modulus; sections carry `area`/`iy`
//! and neither `iz` nor a torsion constant; member loads are a `(wx, wy)` pair; combinations are a
//! `terms:LIST`; and the committed grammar writes quantities WITH UNITS where the 3D grammar writes
//! bare numbers.
//!
//! The 25 committed vectors are authored against cascade hazards, not easy cases: deleting a load case
//! takes its loads with it while deleting a node must NOT take its support, `replace-element` converts
//! a beam into a bar IN PLACE so a remove-then-append lands with the right set and the wrong order, and
//! `replace-region` punches a stair opening through the slab without touching the outline.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner resolves an
//! oracle implementation from the feature's `@oracle-` tag and this feature has none — so every law this
//! case claims is asserted inside the SUBJECT handlers, through the shared law module
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` that the stdio subsets use. The oracle handlers
//! below still answer with the committed vector read literally, so the reference side exists the moment a
//! second producer ever does. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

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
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-fem2d-1: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed after-snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let after = vector(kind).after;
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed before-snapshot — undoing a mutation must land back
/// exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let before = vector(kind).before;
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, DSL_ASSET, UNOBSERVABLE};
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

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to its committed before-snapshot and asserts THREE things the vector commits
    /// to: the resulting document is the committed after-snapshot, the produced delta is the committed
    /// `🔺️diff` (which pins WHICH fields the mutation was allowed to touch, not merely where it ended
    /// up), and the diagnostics are the ones the committed `🎯️outcome` declares. A kind the vector shows
    /// moving is additionally held to the observability law, so a mutation that quietly did nothing
    /// cannot pass by agreeing with an unchanged document.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(kind);
            let report = parse_json(&fem2d_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("mutate-{kind}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let applied = member(&report, "snapshot")?;
            let expected = member(&report, "expectedSnapshot")?;
            if let Some(first) = law::divergence(applied, expected) {
                return Err(format!("mutate-{kind}: the applied document is not the committed after-snapshot — {first}"));
            }
            if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(committed.diff)) {
                return Err(format!("mutate-{kind}: the produced delta is not the committed 🔺️diff — {first}"));
            }
            declared_outcome_holds(kind, &members(&report, "messages")?, &canonical(committed.outcome))?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must restore
    /// the committed before-snapshot exactly. Asserted in role through `law::inverse_restores`, so a
    /// divergence is reported by JSON path rather than as a bare inequality, and an inverse step that
    /// was itself rejected fails here rather than silently leaving the document where it was.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(kind);
            let report = parse_json(&fem2d_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("inverse-{kind}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let faults: Vec<String> = members(&report, "inverseMessages")?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect();
            if !faults.is_empty() {
                return Err(format!("inverse-{kind}: an inverse step was rejected with {faults:?}, so the document never got the chance to return"));
            }
            let restored = member(&report, "inverseSnapshot")?;
            law::inverse_restores(kind, restored, member(&report, "base")?)?;
            Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored.clone()))
        }
    }

    /// 🔁️ The real committed document through this subset's own two codecs. The semantic half is
    /// `law::round_trip_preserves`: parsing, printing back and parsing again must not move the
    /// projection. The byte half is `law::carrier_is_exact` rather than the wave's usual
    /// no-pass-through tripwire, and deliberately so — `store::ArtifactDsl`'s own documented LAW is that
    /// canonical `print_dsl` output is a `parse_dsl` fixpoint, so the correct answer for a second
    /// printing IS byte identity and anything else is the defect. Neither printing is compared against
    /// the committed file, which the same law explicitly allows to normalize on the way in. The pack
    /// decoding is a separate binary codec, so agreeing on one snapshot cannot be reached by carrying
    /// text bytes across.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let report = parse_json(&fem2d_identity_report_json(&text).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
        let parsed = member(&report, "parsed")?;
        law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
        law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
        if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
            return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
        }
        Ok(Outcome::with_raw(parsed.to_string().into_bytes(), parsed.clone()))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario id,
/// so the loop mirrors the feature's `Examples` tables exactly; `identity-round-trip` is subject-only
/// because turning the committed example's DSL bytes into a document needs this subset's own codec,
/// which the oracle-only build must not link.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
