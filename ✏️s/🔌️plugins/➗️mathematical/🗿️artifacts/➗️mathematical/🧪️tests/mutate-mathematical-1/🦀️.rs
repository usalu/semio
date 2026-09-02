//! ➗️ `s.mathematical.mathematical` exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR. Recorded no-oracle decision
//! `mathematical-mutation-semantics` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`,
//! which also records why `petgraph` and the external CAS candidates were surveyed and DECLINED).
//!
//! This subset's snapshot no longer holds the collections its vocabulary addresses. Ticket
//! UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM replaced the inline `graph`/`geometry` fields with three
//! composed children (`notation`, `results`, `computed`) and added ONE plain field, `equation` — a
//! labelled expression tree whose `EquationNodeLabel` allocator only ever increases so an address
//! survives edits a positional path would not.
//!
//! ⚠️ The consequence is measured rather than described: thirteen of the fifteen kinds have no forward
//! vector. Nine carry REJECTION vectors (`mutation.target-missing`, `mutation.duplicate-id`) and four
//! carry `applied`-but-`mutation.no-op` vectors that restate a value the document already holds. Those
//! vectors are worth asserting — a rejection vector pins the fault code, the offending address AND that
//! the document was left untouched, which is where the frozen outcome contract's law 2 lives — but they
//! are not forward evidence, so all thirteen are listed by name in `UNOBSERVABLE` below.
//!
//! The two that are real: `change-coefficient` raises the leading coefficient of the persisted
//! polynomial to three halves through the never-reused label allocator, writing a `Rational` node with
//! decimal `numer`/`denom` lexemes rather than an `f64`, so an edit routed through a float loses
//! precision and fails; and `insert-point` seeds the empty cloud with its first point, the one
//! index-addressed verb whose inverse is exercised end to end here.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner resolves an
//! oracle implementation from the feature's `@oracle-` tag and this feature has none — so every law this
//! case claims is asserted inside the SUBJECT handlers, through the shared law module
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` that the stdio subsets use. The oracle handlers
//! below still answer with the committed vector read literally, so the reference side exists the moment a
//! second producer ever does. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and that file's own
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against both the enum and the manifest.
const KINDS: &[&str] = &[
    "change-graph-directed",
    "update-graph-algorithm",
    "replace-graph",
    "create-node",
    "delete-node",
    "delete-nodes",
    "change-node-label",
    "move-node",
    "connect-nodes",
    "disconnect-nodes",
    "replace-points",
    "insert-point",
    "remove-point",
    "move-point",
    "change-coefficient",
];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect, so
/// [`law::mutation_is_observable`] must not demand one of them.
/// Thirteen of the fifteen kinds. Nine (`create-node`, `delete-node`, `delete-nodes`, `change-node-label`,
/// `move-node`, `connect-nodes`, `disconnect-nodes`, `remove-point`, `move-point`) carry REJECTION vectors,
/// and four (`change-graph-directed`, `update-graph-algorithm`, `replace-graph`, `replace-points`) carry
/// `applied`-but-`mutation.no-op` vectors, because the graph and the point cloud they address live in
/// composed children no fixture can resolve. `change-coefficient` and `insert-point` are the only forward
/// vectors and are asserted observable.
const UNOBSERVABLE: &[&str] = &[
    "change-graph-directed",
    "update-graph-algorithm",
    "replace-graph",
    "create-node",
    "delete-node",
    "delete-nodes",
    "change-node-label",
    "move-node",
    "connect-nodes",
    "disconnect-nodes",
    "replace-points",
    "remove-point",
    "move-point",
];

/// 🗣️ The real committed document this artifact ships as its own example.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ One kind's committed `(before, mutation, after, diff, outcome)` specification vector, read
/// literally via `include_str!`. This IS the independently handcrafted evidence the no-oracle decision
/// rests on — never recomputed here, never restated as a Rust literal.
struct Vector {
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    /// 🔺️ `None` for the nine REJECTION vectors, which commit no `🔺️diff` file at all — a rejected
    /// mutation produces the default delta by the frozen outcome contract's law 2, so there is nothing
    /// for the vector to pin. Every other subset in this wave commits a diff for every kind; this one
    /// cannot, and the handler below refuses a missing diff on any vector that is not a rejection
    /// rather than treating absence as permission to skip the check.
    diff: Option<&'static str>,
    outcome: &'static str,
}

fn vector(kind: &str) -> Vector {
    match kind {
        "change-graph-directed" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-graph-directed/🧪️tests/keeps-an-already-directed-graph-directed/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-graph-directed/🧪️tests/keeps-an-already-directed-graph-directed/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-graph-directed/🧪️tests/keeps-an-already-directed-graph-directed/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-graph-directed/🧪️tests/keeps-an-already-directed-graph-directed/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-graph-directed/🧪️tests/keeps-an-already-directed-graph-directed/🎯️outcome/🔣️.json"),
        },
        "update-graph-algorithm" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🧪️tests/restates-the-unset-algorithm-and-its-absent-seed/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🧪️tests/restates-the-unset-algorithm-and-its-absent-seed/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🧪️tests/restates-the-unset-algorithm-and-its-absent-seed/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🧪️tests/restates-the-unset-algorithm-and-its-absent-seed/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🧪️tests/restates-the-unset-algorithm-and-its-absent-seed/🎯️outcome/🔣️.json"),
        },
        "replace-graph" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-graph/🧪️tests/replays-the-identical-empty-graph/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-graph/🧪️tests/replays-the-identical-empty-graph/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-graph/🧪️tests/replays-the-identical-empty-graph/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-graph/🧪️tests/replays-the-identical-empty-graph/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-graph/🧪️tests/replays-the-identical-empty-graph/🎯️outcome/🔣️.json"),
        },
        "create-node" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️create-node/🧪️tests/rejects-a-duplicate-node-id/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️create-node/🧪️tests/rejects-a-duplicate-node-id/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️create-node/🧪️tests/rejects-a-duplicate-node-id/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️create-node/🧪️tests/rejects-a-duplicate-node-id/🎯️outcome/🔣️.json"),
        },
        "delete-node" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌️delete-node/🧪️tests/rejects-deleting-a-node-that-is-not-in-the-graph/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌️delete-node/🧪️tests/rejects-deleting-a-node-that-is-not-in-the-graph/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌️delete-node/🧪️tests/rejects-deleting-a-node-that-is-not-in-the-graph/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌️delete-node/🧪️tests/rejects-deleting-a-node-that-is-not-in-the-graph/🎯️outcome/🔣️.json"),
        },
        "delete-nodes" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-nodes/🧪️tests/rejects-a-bulk-delete-where-every-id-is-absent/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-nodes/🧪️tests/rejects-a-bulk-delete-where-every-id-is-absent/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-nodes/🧪️tests/rejects-a-bulk-delete-where-every-id-is-absent/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-nodes/🧪️tests/rejects-a-bulk-delete-where-every-id-is-absent/🎯️outcome/🔣️.json"),
        },
        "change-node-label" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-node-label/🧪️tests/rejects-relabelling-a-node-that-is-not-in-the-graph/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-node-label/🧪️tests/rejects-relabelling-a-node-that-is-not-in-the-graph/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-node-label/🧪️tests/rejects-relabelling-a-node-that-is-not-in-the-graph/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-node-label/🧪️tests/rejects-relabelling-a-node-that-is-not-in-the-graph/🎯️outcome/🔣️.json"),
        },
        "move-node" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️move-node/🧪️tests/rejects-moving-a-node-that-is-not-in-the-graph/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️move-node/🧪️tests/rejects-moving-a-node-that-is-not-in-the-graph/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️move-node/🧪️tests/rejects-moving-a-node-that-is-not-in-the-graph/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️move-node/🧪️tests/rejects-moving-a-node-that-is-not-in-the-graph/🎯️outcome/🔣️.json"),
        },
        "connect-nodes" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-nodes/🧪️tests/rejects-an-edge-between-two-absent-endpoints/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-nodes/🧪️tests/rejects-an-edge-between-two-absent-endpoints/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-nodes/🧪️tests/rejects-an-edge-between-two-absent-endpoints/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-nodes/🧪️tests/rejects-an-edge-between-two-absent-endpoints/🎯️outcome/🔣️.json"),
        },
        "disconnect-nodes" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/rejects-severing-an-edge-that-is-not-in-the-graph/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/rejects-severing-an-edge-that-is-not-in-the-graph/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/rejects-severing-an-edge-that-is-not-in-the-graph/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/rejects-severing-an-edge-that-is-not-in-the-graph/🎯️outcome/🔣️.json"),
        },
        "replace-points" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/🎯️outcome/🔣️.json"),
        },
        "insert-point" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/🎯️outcome/🔣️.json"),
        },
        "remove-point" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/rejects-removing-a-point-from-an-empty-cloud/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/rejects-removing-a-point-from-an-empty-cloud/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/rejects-removing-a-point-from-an-empty-cloud/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/rejects-removing-a-point-from-an-empty-cloud/🎯️outcome/🔣️.json"),
        },
        "move-point" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/rejects-moving-a-point-that-is-not-in-the-cloud/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/rejects-moving-a-point-that-is-not-in-the-cloud/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/rejects-moving-a-point-that-is-not-in-the-cloud/📸️snapshot/➡️after/🔣️.json"),
            diff: None,
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/rejects-moving-a-point-that-is-not-in-the-cloud/🎯️outcome/🔣️.json"),
        },
        "change-coefficient" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-coefficient/🧪️tests/raises-the-leading-coefficient-to-three-halves/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-coefficient/🧪️tests/raises-the-leading-coefficient-to-three-halves/🦠️mutation/🔣️.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-coefficient/🧪️tests/raises-the-leading-coefficient-to-three-halves/📸️snapshot/➡️after/🔣️.json"),
            diff: Some(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-coefficient/🧪️tests/raises-the-leading-coefficient-to-three-halves/🔺️diff/🔣️.json")),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-coefficient/🧪️tests/raises-the-leading-coefficient-to-three-halves/🎯️outcome/🔣️.json"),
        },
        other => panic!("mutate-mathematical-1: no committed specification vector is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-mathematical-1: a committed fixture must be valid JSON: {error}"))
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
    use semio_s_plugin_mathematical::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::mathematical_mutation_report_json;
    use semio_s_plugin_mathematical::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::mathematical_identity_report_json;

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
            let report = parse_json(&mathematical_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("mutate-{kind}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let applied = member(&report, "snapshot")?;
            let expected = member(&report, "expectedSnapshot")?;
            if let Some(first) = law::divergence(applied, expected) {
                return Err(format!("mutate-{kind}: the applied document is not the committed after-snapshot — {first}"));
            }
            match committed.diff {
                Some(diff) => {
                    if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(diff)) {
                        return Err(format!("mutate-{kind}: the produced delta is not the committed 🔺️diff — {first}"));
                    }
                }
                None if canonical(committed.outcome).str("status") == "rejected" => {}
                None => return Err(format!("mutate-{kind}: the vector commits no 🔺️diff, which only a rejection vector may omit")),
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
            let report = parse_json(&mathematical_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("inverse-{kind}: the committed vector did not reach this subset's own codec: {error}"))?)?;
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
        let report = parse_json(&mathematical_identity_report_json(&text).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
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
