/// 📇️ Fixture: the committed expected-geometry statement types and the budget contract.

//#region 🔖️Fixture
/// 📇️ One example's committed expected-geometry statement — the single source both this lane and
/// its TypeScript twin (`🧩️example/🟦️.ts`) read.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExampleGeometryFixture {
    schema: String,
    example: String,
    op_chain: Vec<String>,
    preview: PreviewTarget,
    tessellation_tolerance: f64,
    expect: Expectation,
    delivery: DeliveryExpectation,
    budget: BudgetExpectation,
    kernel_status: String,
}

/// ⏱️ What this example's chain is allowed to COST, in wall microseconds measured on this lane's own
/// native `test` (unoptimized) profile — the third half of "the example works": geometry that is
/// right and delivered but takes 78 s to appear is an example the user never sees converge
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️kernel-performance-2026-09-13.md`).
///
/// ⚖️ These are CEILINGS with deliberate headroom over the measured value, not targets: the point is
/// to convict an algorithmic regression of the class this ticket removed (the boolean kernel's
/// face-stitch spent 99% of `🍩️sphere-cut-with-torus`'s 61 s in arbitrary-precision rational
/// predicates — a 14-35x penalty), never to police a few percent of machine-to-machine variance.
/// A number here is only ever lowered after a measured improvement, never raised to admit a
/// regression.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BudgetExpectation {
    max_evaluate_micros: u64,
    max_tessellate_micros: u64,
    max_preview_tessellate_micros: u64,
}

/// 🚚️ What this example's preview must DELIVER across the extension boundary, at the LOD the live
/// surface asks for — the second half of "the geometry is right": a correct mesh nobody receives is
/// a blank viewport. `max_round_trips` is the load-bearing number, because one round trip is one
/// whole `flowEvalTick` (evaluate → invoke → answer → refresh), seconds apiece in a served build
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️preview-mesh-delivery-2026-09-12.md`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeliveryExpectation {
    lod_mode: String,
    /// 🏷️ How many of those meshes carry each ROLE, hand-written per example. `meshes` alone cannot
    /// separate "the solid is on screen" from "the solid vanished and its two companion channels
    /// remain": every published mesh stamps a `role` (`preview_eval::preview_mesh_role`) and a
    /// preview that loses its only solid still publishes the wire it was extruded from and the
    /// vector that drove it. The runtime oracles compare against THIS row, per role, which is what
    /// makes a browser mesh census comparable to the native one at all
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-mesh-oracle-2026-09-14.md`).
    mesh_roles: BTreeMap<String, usize>,
    /// 🕸️ The EXACT number of meshes this example's preview publishes. A floor is not enough: a
    /// preview that loses one of three meshes still reports every node `ok`, and the surface simply
    /// paints less — which is how a renamed operator port that stopped feeding `extrude` its axis
    /// reached a browser with a green suite behind it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    meshes: usize,
    min_triangles: usize,
    min_edge_segments: usize,
    /// 📦️ The extent the DELIVERED payload occupies, hand-written per example from `run_delivery`'s own
    /// `preview_payload`. Deliberately its own row beside `expect.boundingBox*`: `expect` measures the
    /// FINE tessellation this lane computes, the delivery paints the preview LOD, and an inscribed
    /// coarse mesh is legitimately smaller. Without both committed, a browser oracle can only grade the
    /// delivered extent against a stated envelope rather than a number — which is exactly what
    /// `📓️react-oracle-hardening-2026-09-14.md` §1.1 had to do (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎯️ It is also what the preview FRAMES: the render hosts read these bounds off the `fit` lane, so a
    /// wrong number here is a camera that cuts the example off, not merely a weak assertion.
    bounding_box_min: [f64; 3],
    bounding_box_max: [f64; 3],
    bounding_box_tolerance: f64,
    max_round_trips: usize,
    max_chunks: u32,
}

/// 🎯️ Which evaluated node channel carries the geometry the preview renders.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreviewTarget {
    node: String,
    channel: String,
    kind: String,
}

/// 📐️ The measured quantities a run must reproduce, each with its own stated tolerance.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Expectation {
    min_triangles: usize,
    closed: bool,
    volume: Option<f64>,
    volume_tolerance: f64,
    volume_source: String,
    edge_perimeter: Option<f64>,
    edge_perimeter_tolerance: f64,
    bounding_box_min: [f64; 3],
    bounding_box_max: [f64; 3],
    bounding_box_tolerance: f64,
    kernel_volume_node: Option<String>,
    kernel_volume_channel: Option<String>,
    kernel_volume_tolerance: Option<f64>,
}

const FIXTURE_SCHEMA: &str = "s.procedural.generation3d.example-geometry/v1";

/// ⏱️ The hard bound the two USER-FACING phases sit under — mirrors the TypeScript twin's
/// `EXAMPLE_BUDGET_INTERACTIVE_CEILING_MICROS`. Evaluating the op chain and tessellating at the LOD
/// the live preview asks for are what the playground waits on, and neither may claim more than two
/// seconds of this lane's native `test`-profile wall time: the shipped wasm guest runs the identical code at
/// the `wasm-release` profile (`opt-level = "s"`, thin LTO) inside a budgeted
/// resumable job, so a phase over this bound cannot converge in the playground within the
/// user-facing target ticket 26/09/09/PROCEDURAL-3D-END-TO-END set.
const BUDGET_INTERACTIVE_CEILING_MICROS: u64 = 2_000_000;

/// ⏱️ The bound on the FIDELITY phase — `tessellate_geometry` at the fixture's own
/// `tessellationTolerance`, which is far finer than any LOD the live preview requests and exists so
/// the committed geometry statement is measured on a converged mesh. Nobody waits on this number in
/// the app, so its ceiling is a regression guard rather than a user-facing promise and is set
/// looser; it still convicts the class of defect this ticket removed, which cost 14-35x.
const BUDGET_FIDELITY_CEILING_MICROS: u64 = 8_000_000;

/// ✅️ Every invariant the `budget` row states about ITSELF, checked before any timing is compared
/// against it — a ceiling nobody bounded is the absence of a budget, and the preview LOD is by
/// construction coarser than the fixture's own tessellation tolerance.
fn assert_budget_contract(fixture: &ExampleGeometryFixture) {
    let budget = &fixture.budget;
    for micros in [budget.max_evaluate_micros, budget.max_preview_tessellate_micros] {
        assert!(micros > 0, "{}: a budget of zero is not a budget", fixture.example);
        assert!(micros <= BUDGET_INTERACTIVE_CEILING_MICROS, "{}: a {micros} us ceiling exceeds the {BUDGET_INTERACTIVE_CEILING_MICROS} us bound every user-facing phase sits under", fixture.example);
    }
    assert!(budget.max_tessellate_micros > 0, "{}: a budget of zero is not a budget", fixture.example);
    assert!(budget.max_tessellate_micros <= BUDGET_FIDELITY_CEILING_MICROS, "{}: a {} us ceiling exceeds the {BUDGET_FIDELITY_CEILING_MICROS} us bound the fidelity phase sits under", fixture.example, budget.max_tessellate_micros);
    assert!(budget.max_preview_tessellate_micros <= budget.max_tessellate_micros, "{}: the preview LOD is coarser than the fixture tolerance, so its ceiling may never exceed the full-tolerance one", fixture.example);
    assert_calibration_contract();
}

/// 🚧️ The machine-readable kernel standings a fixture may declare. Each `blocked-*` value names one
/// SPECIFIC located kernel defect that this lane's run reproduced, never a licence to relax an
/// expectation: the numbers stay exactly what the geometry must be, and the run keeps failing until
/// the named defect is fixed. A defect that gets fixed takes its value out of this list with it —
/// a standing nothing declares is dead vocabulary.
///
/// - `green` — the example's whole chain is exact today.
///
/// `blocked-on-fillet-kernel` was the last such standing and is gone with the defect it named:
/// `diff::blend`'s analytic rewrite (ticket 26/09/09/PROCEDURAL-3D-END-TO-END) mints the corner
/// patches `📐️box-fillet-preview` was missing, so every bundled example is `green`.
const KERNEL_STATUSES: [&str; 1] = ["green"];
//#endregion 🔖️Fixture
