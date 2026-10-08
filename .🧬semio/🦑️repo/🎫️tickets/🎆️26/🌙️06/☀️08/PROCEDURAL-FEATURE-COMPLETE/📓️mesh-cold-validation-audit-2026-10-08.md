# Original Cold Validation Frontier

Read-only source audit, 2026-10-08. No production edit, native run, or passing receipt. Existing ticket reused; ticket lifecycle remains with root.

## Exact Remaining Whole-Body Charge

Original Session `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs:1121–1167` cold admission creates the genuine tessellation job, assigns validation exactly one unit for Solid/Shell/Compound, then at1160 calls original `validate_gate_sync`. A zero-budget cold call still admits/creates its job before returning validating. A positive budget1 runs the entire validation gate and subtracts1. Existing locks remain held throughout that call. The mesh job itself is now created with `TessellationJob::new(deflection)` and original entity input is re-presented, so do not reintroduce preparation snapshots.

Engine `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️.rs:1545–1556` computes whole reachable topology, then validates the entire process-wide body, then filters blocking issues by shape reach. Stranger geometry contributes compute even when its findings are filtered. Shell `shell-not-closed` exemption, warning exclusion and unreadable-label blocking are existing verdict semantics to preserve.

Query `…/📐️brep/💡️queries/✅validation/🦀️.rs:388–389` already makes `validate_body` the unbudgeted facade of the original `BodyValidationJob`. This is the natural single implementation to extend; no mirror validator or copied body is necessary. Its constructor529–541 eagerly builds shell face ID vectors and all edge/face ID vectors. Eight checks are each charged one whole-body unit at579–585/632/641: rings, valence, tolerance, missing pcurves, same parameter, shell closure, winding and self-intersection. The one-unit comments explicitly describe that current behavior; they are not a bounded-work guarantee.

Valence additionally invokes `Body::edge_coedges` for every edge, whose original topology198–199 scans all coedges. SameParameter scans all face coedges and up to three refinement passes per coedge. Closure builds a shell-wide edge-use map before verdict iteration. Winding invokes whole loop quadrature. SelfIntersection340+ builds face AABBs/sample arrays per shell and nested face/sample pair probes. Original orientation/degenerate phases already have per-face/per-edge cursors, but volume/area remain whole single-face work; avoid asserting a strict time ceiling from an item budget.

## Narrow Ownership Path

Mesh High owns original validation query, engine scoped-gate seam and Session retained validation state. Extend the existing job with cold arena/reach cursors and actual check frontiers, retain it beside the original tessellation job, and present the original body/entity on each grant. Charge scan/probe work rather than phase completion. Keep reach filtering on original entity IDs and existing diagnostics semantics. Retire validation scratch/cursors with the existing retained owner on cancel, eviction, invalid and Session close. Do not route through synchronous `run_to_completion` during retained preview admission.

Use coedge iteration to accumulate valence incrementally rather than edge-by-all-coedges scans. Ring checks require a retained loop/coedge cursor; tolerance and same-parameter require face/loop/coedge/sample frontier; shell closure requires incremental accumulation and verdict frontier; winding and self-intersection need quadrature/pair cursors if their single-item cost is to be bounded. Existing per-face orientation integration is a separate deeper frontier and should be declared explicitly if left whole.

## Schema-First RED Needed

Original Session schema has no matched validation/preparation declaration; adjacent validation unit tests have no BodyValidationJob/budget/resume matches. Add neutral fixtures in the original Session fixture domain before production repair: cold budget0 performs zero validation probes; repeated budget1 on a sufficiently large original solid yields in validating before tessellation; counters witness scan/sample/pair work bounded by grants; cancellation during each costly phase prevents later probes/publication and retires exact owners; healthy shape with broken stranger keeps scoped verdict unchanged; Shell exemption and warning behavior survive; final diagnostics and mesh equal original unbudgeted output.

Use original validation unit tests and Session mesh-session tests as native consumers of those neutral laws. Existing same implementation unbudgeted facade establishes scheduling equivalence, not independent correctness: third-party reference geometry validation/volume output is still required for at least one neutral feature witness. No test was executed in this read-only audit.
