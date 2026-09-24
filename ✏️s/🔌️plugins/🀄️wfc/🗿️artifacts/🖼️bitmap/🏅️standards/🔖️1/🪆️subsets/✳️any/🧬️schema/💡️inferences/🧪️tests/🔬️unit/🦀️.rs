//! 🧪️ Inference facet — the solve is DETERMINISTIC for a seed, a sample that cannot tile its output
//! is reported as a contradiction rather than a wrong answer, and the roster identity is the one the
//! plugin root advertises.

use super::*;
use crate::schema::snapshot::{decode_base64, encode_base64, BitmapColor, BitmapInput, BitmapOutputSpec, BitmapOverlappingModel, BitmapPinnedPixel};

/// 🧪️ A 4 × 4 vertical-stripe sample: every `2 × 2` window is one of two patterns, so a periodic
/// output of any even width tiles cleanly.
fn stripes() -> BitmapSnapshot {
    let indices: Vec<u8> = (0..16u32).map(|cell| (cell % 2) as u8).collect();
    BitmapSnapshot {
        seed: 11,
        input: BitmapInput { width: 4, height: 4, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: encode_base64(&indices) },
        output: BitmapOutputSpec { width: 6, height: 4, periodic: true },
        model: BitmapOverlappingModel { pattern_size: 2, symmetry: 1, periodic_input: true, ground: None },
        pinned: Vec::new(),
        ..BitmapSnapshot::default()
    }
}

#[test]
fn the_roster_identity_is_the_wfc_owned_solve_route() {
    let metadata = bitmap_inference_metadata();
    assert_eq!(metadata.owner, "wfc");
    assert_eq!(metadata.artifact_kind, crate::WFC_BITMAP_DOCUMENT_SCHEMA);
    assert_eq!(metadata.inference_schema, BITMAP_INFERENCE_TOOL_ID);
    assert_eq!(BITMAP_INFERENCE_TOOL_ID, "s.wfc.bitmap.solve");
    assert_eq!(BITMAP_INFERENCE_JOB_KIND, "semio.infer");
    assert_eq!(BITMAP_INFERENCE_PAYLOAD_SCHEMA, "s.wfc.bitmap.inference.request.v1");
}

#[test]
fn the_descriptor_carries_five_real_leaves() {
    let descriptor = bitmap_artifact_inference_descriptor();
    assert_eq!(descriptor.id, BITMAP_INFERENCE_TOOL_ID);
    for leaf in [descriptor.inference.rust, descriptor.inference.typescript, descriptor.inference.graphql, descriptor.inference.json_schema, descriptor.inference.proto] {
        assert!(!leaf.trim().is_empty());
    }
}

#[test]
fn the_symmetry_count_selects_a_prefix_of_d4() {
    assert_eq!(symmetry_group(1).elements().len(), 1);
    assert_eq!(symmetry_group(4).elements().len(), 4);
    assert_eq!(symmetry_group(8).elements().len(), 8);
    assert_eq!(symmetry_group(99).elements().len(), 8, "the count is clamped, never a panic");
    assert_eq!(symmetry_group(3).elements()[0], semio_s_plugin_wfc_engine::symmetry::Transform2d::Identity, "the identity is always in the group");
    assert_eq!(symmetry_group(2).elements()[1], semio_s_plugin_wfc_engine::symmetry::Transform2d::FlipH, "count 2 is a MIRROR, not a quarter turn");
    let full: std::collections::HashSet<_> = symmetry_group(8).elements().into_iter().collect();
    assert_eq!(full.len(), 8, "count 8 is the whole dihedral group, with no repeats");
}

#[test]
fn the_solve_is_deterministic_for_a_seed() {
    let snapshot = stripes();
    let first = solve_with_job(&snapshot).expect("the stripe sample solves");
    let second = solve_with_job(&snapshot).expect("the stripe sample solves again");
    assert_eq!(first, second, "the same seed and the same spec give the same bitmap");
    assert!(!first.contradiction);
    let pixels = decode_base64(&first.pixels).expect("the committed output decodes");
    assert_eq!(pixels.len(), 24, "one palette index per output cell");
    assert!(pixels.iter().all(|index| *index < 2), "every index names a real palette entry");
    assert_eq!(first.entropy.len(), 24, "one entropy value per output cell");
}

#[test]
fn a_different_seed_is_free_to_answer_differently_but_stays_valid() {
    let mut other = stripes();
    other.seed = 4_242;
    let solved = solve_with_job(&other).expect("the stripe sample solves under another seed");
    assert!(!solved.contradiction);
    assert_eq!(decode_base64(&solved.pixels).expect("output decodes").len(), 24);
}

/// 🩺 The sample `0 1 0` learned as 2 × 2 windows over a periodic input yields three patterns
/// whose only horizontal rule is "a pattern's right column must be the next one's left column".
/// Colour 1 is the anchor of exactly ONE pattern, whose right column is `1`, and no pattern's
/// left column is `1` except that same one — which its own rule forbids following itself. Pinning
/// two ADJACENT output cells to colour 1 therefore asks for an adjacency the sample never
/// contains, and no assignment exists.
#[test]
fn a_sample_that_cannot_tile_the_output_reports_a_contradiction() {
    let snapshot = BitmapSnapshot {
        seed: 5,
        input: BitmapInput { width: 3, height: 1, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: encode_base64(&[0, 1, 0]) },
        output: BitmapOutputSpec { width: 3, height: 1, periodic: false },
        model: BitmapOverlappingModel { pattern_size: 2, symmetry: 1, periodic_input: true, ground: None },
        pinned: vec![BitmapPinnedPixel { x: 0, y: 0, color: 1 }, BitmapPinnedPixel { x: 1, y: 0, color: 1 }],
        ..BitmapSnapshot::default()
    };
    let solved = solve_with_job(&snapshot).expect("the job completes even when the spec is unsatisfiable");
    assert!(solved.contradiction, "a spec with no consistent assignment is a verdict, never a wrong bitmap");
    assert!(solved.pixels.is_empty(), "a contradiction carries no output pixels");
    assert_eq!(solved.entropy.len(), 3, "the entropy map still describes the output it could not fill");
    assert!(<BitmapContradiction as store::InferredField<BitmapSnapshot>>::compute(&snapshot, &"bitmap".to_string(), &[]));
    assert_eq!(<BitmapSolve as store::InferredField<BitmapSnapshot>>::compute(&snapshot, &"bitmap".to_string(), &[]), BitmapSolveResult::Unsolved);
}

/// 🩺 With `N = 2` over a non-periodic `0 1 2`, only `0` and `1` are ever a window's top-left
/// cell, so colour `2` is not expressible as a pin at all. Silently ignoring the pin would hand
/// back a bitmap that quietly disobeys the document.
#[test]
fn a_pin_on_a_colour_no_pattern_anchors_is_refused_rather_than_dropped() {
    let snapshot = BitmapSnapshot {
        seed: 5,
        input: BitmapInput {
            width: 3,
            height: 2,
            palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(120, 120, 120), BitmapColor::opaque(255, 255, 255)],
            pixels: encode_base64(&[0, 1, 2, 0, 1, 2]),
        },
        output: BitmapOutputSpec { width: 2, height: 2, periodic: false },
        model: BitmapOverlappingModel { pattern_size: 2, symmetry: 1, periodic_input: false, ground: None },
        pinned: vec![BitmapPinnedPixel { x: 0, y: 0, color: 2 }],
        ..BitmapSnapshot::default()
    };
    assert_eq!(solve_with_job(&snapshot).unwrap_err(), "bitmap-inference-unreachable-pin");
}

#[test]
fn an_oversized_output_is_refused_at_admission() {
    let mut snapshot = stripes();
    snapshot.output = BitmapOutputSpec { width: 512, height: 512, periodic: false };
    assert!(solve_with_job(&snapshot).is_err(), "an output past the admission ceiling is refused rather than attempted");
}

#[test]
fn the_prior_entropy_is_zero_for_a_pinned_cell_and_positive_elsewhere() {
    let mut snapshot = stripes();
    snapshot.pinned = vec![BitmapPinnedPixel { x: 0, y: 0, color: 0 }];
    let universe = pattern_universe_entropy(&snapshot);
    assert!(universe > 0.0, "a two-pattern universe has real entropy");
    assert_eq!(<BitmapEntropy as store::InferredField<BitmapSnapshot>>::compute(&snapshot, &"0".to_string(), &[]), 0.0);
    assert_eq!(<BitmapEntropy as store::InferredField<BitmapSnapshot>>::compute(&snapshot, &"5".to_string(), &[]), universe);
    assert_eq!(<BitmapEntropy as store::InferredField<BitmapSnapshot>>::plan(&snapshot).len(), 24);
}

#[test]
fn the_solve_field_answers_solved_for_a_tileable_sample() {
    let snapshot = stripes();
    match <BitmapSolve as store::InferredField<BitmapSnapshot>>::compute(&snapshot, &"bitmap".to_string(), &[]) {
        BitmapSolveResult::Solved { pixels } => assert_eq!(decode_base64(&pixels).expect("pixels decode").len(), 24),
        BitmapSolveResult::Unsolved => panic!("the stripe sample is tileable"),
    }
    assert!(!<BitmapContradiction as store::InferredField<BitmapSnapshot>>::compute(&snapshot, &"bitmap".to_string(), &[]));
}

//#region 🧩️LocalSimilarity
/// 🧩️ The SYMMETRY-EXPANDED window set the extractor learns from this document's own input — the
/// exact enumeration `extract_2d` performs: every window position (wrapped when `periodicInput`,
/// otherwise only fully in-bounds), each expanded under the same `symmetry_group(n)` transforms.
fn expanded_input_windows(snapshot: &BitmapSnapshot) -> std::collections::HashSet<Vec<u8>> {
    let indices = snapshot.input.indices().expect("the sample decodes");
    let (width, height) = (snapshot.input.width as usize, snapshot.input.height as usize);
    let window = snapshot.model.pattern_size as usize;
    let periodic = snapshot.model.periodic_input;
    let (x_positions, y_positions) = if periodic { (width, height) } else { (width.saturating_sub(window - 1), height.saturating_sub(window - 1)) };
    let transforms = symmetry_group(snapshot.model.symmetry).elements();
    let mut expanded = std::collections::HashSet::new();
    for origin_y in 0..y_positions {
        for origin_x in 0..x_positions {
            let mut base = Vec::with_capacity(window * window);
            for row in 0..window {
                for column in 0..window {
                    let x = if periodic { (origin_x + column) % width } else { origin_x + column };
                    let y = if periodic { (origin_y + row) % height } else { origin_y + row };
                    base.push(semio_s_plugin_wfc_engine::ids::TileId(u32::from(indices[y * width + x])));
                }
            }
            for transform in &transforms {
                let (_, _, tiles) = transform.apply_window(window, window, &base);
                expanded.insert(tiles.into_iter().map(|tile| tile.get() as u8).collect::<Vec<u8>>());
            }
        }
    }
    expanded
}

/// 🧩️ Every window of the SOLVED output, with wrap-around when the output is periodic and only the
/// fully in-bounds positions when it is not — the positions the four-neighbour stencil actually
/// constrained.
fn output_windows(snapshot: &BitmapSnapshot, pixels: &[u8]) -> Vec<((usize, usize), Vec<u8>)> {
    let (width, height) = (snapshot.output.width as usize, snapshot.output.height as usize);
    let window = snapshot.model.pattern_size as usize;
    let periodic = snapshot.output.periodic;
    let (x_positions, y_positions) = if periodic { (width, height) } else { (width.saturating_sub(window - 1), height.saturating_sub(window - 1)) };
    let mut windows = Vec::new();
    for origin_y in 0..y_positions {
        for origin_x in 0..x_positions {
            let mut cells = Vec::with_capacity(window * window);
            for row in 0..window {
                for column in 0..window {
                    let x = if periodic { (origin_x + column) % width } else { origin_x + column };
                    let y = if periodic { (origin_y + row) % height } else { origin_y + row };
                    cells.push(pixels[y * width + x]);
                }
            }
            windows.push(((origin_x, origin_y), cells));
        }
    }
    windows
}

/// 🖼️ THE DEFINING PROPERTY OF THIS ARTIFACT, asserted through its OWN pipeline: a solved output
/// "looks locally similar to the input" means every `N × N` window of the output is a window the
/// input actually contains, under the same symmetry expansion the extractor applied. Driven through
/// `BitmapSnapshot` → `s.wfc.bitmap.solve`, so the base64 carrier, the palette, the ground row and
/// the pin anchoring are all in the path — the engine's own generic test exercises none of those and
/// only ever runs `SymmetryGroup2d::None`.
#[test]
fn every_output_window_of_a_solved_example_occurs_in_the_symmetry_expanded_input() {
    for (label, snapshot) in [("rooms-16", crate::examples::rooms_16::snapshot()), ("flowers-24", crate::examples::flowers_24::snapshot())] {
        assert!(snapshot.model.symmetry > 1, "{label}: this law is only worth asserting with a real symmetry expansion");
        let commit = solve_with_job(&snapshot).unwrap_or_else(|error| panic!("{label}: the job completes: {error}"));
        assert!(!commit.contradiction, "{label}: a bundled example must tile its own declared output");
        let pixels = decode_base64(&commit.pixels).unwrap_or_else(|| panic!("{label}: the output decodes"));
        assert_eq!(pixels.len(), (snapshot.output.width as usize) * (snapshot.output.height as usize), "{label}");
        let allowed = expanded_input_windows(&snapshot);
        assert!(!allowed.is_empty(), "{label}: an empty pattern universe proves nothing");
        let windows = output_windows(&snapshot, &pixels);
        assert!(!windows.is_empty(), "{label}: the output is smaller than one pattern window");
        for ((x, y), cells) in windows {
            assert!(allowed.contains(&cells), "{label}: the output window at ({x}, {y}) does not occur in the symmetry-expanded input");
        }
    }
}

/// 🎲 The same law, re-checked on a second seed, so a pass cannot be an accident of one sampler
/// trajectory — and the two seeds must really produce different bitmaps, or the check is vacuous.
#[test]
fn local_similarity_holds_under_a_different_seed_and_a_non_periodic_output() {
    let mut snapshot = crate::examples::flowers_24::snapshot();
    assert!(!snapshot.output.periodic, "this case exists to cover the non-periodic boundary");
    let first = solve_with_job(&snapshot).expect("the flowers sample solves");
    snapshot.seed = 20_260_918;
    let second = solve_with_job(&snapshot).expect("the flowers sample solves under another seed");
    assert!(!second.contradiction);
    let allowed = expanded_input_windows(&snapshot);
    let pixels = decode_base64(&second.pixels).expect("the output decodes");
    for ((x, y), cells) in output_windows(&snapshot, &pixels) {
        assert!(allowed.contains(&cells), "the reseeded output window at ({x}, {y}) does not occur in the symmetry-expanded input");
    }
    assert_ne!(first.pixels, second.pixels, "two seeds that agree byte for byte would make the law vacuous");
}
//#endregion 🧩️LocalSimilarity


#[test]
fn an_oversized_checkpoint_request_is_refused_at_admission() {
    let snapshot = stripes();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let checkpoint = vec![0u8; semio_s_plugin_wfc_engine::job::MAX_CHECKPOINT_BYTES.saturating_add(1)];
    let rejected = BitmapInferenceJob::new(operation, BitmapInferenceRequest { snapshot: Some(snapshot), document: None, checkpoint: Some(checkpoint) });
    assert!(rejected.is_err(), "a checkpoint past the engine ceiling is an admission refusal");
}

#[test]
fn a_within_budget_checkpoint_request_is_admitted_on_the_inference_job() {
    let snapshot = stripes();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let admitted = BitmapInferenceJob::new(operation, BitmapInferenceRequest { snapshot: Some(snapshot), document: None, checkpoint: Some(vec![0u8; 16]) });
    assert!(admitted.is_ok(), "a within-budget checkpoint request is a real inference input");
    let mut job = admitted.expect("admitted");
    semio_framework_job::InteractiveJob::begin_close(&mut job);
    while !semio_framework_job::InteractiveJob::terminal_is_empty(&job) {
        let _ = semio_framework_job::InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

//#region 📜️PublishedContract
/// 📜️ The contract this inference PUBLISHES is the contract its request actually decodes: the
/// binding names `document`, the request carries a `document`, and the schema id is the one the
/// factory already answered to `payload_schema_id()` and nothing published
/// (`📓️ce3-four-mcp-gates-green.md` §3.3: "no payload contract is published anywhere an agent can
/// read it").
#[test]
fn the_published_contract_binds_the_field_the_request_decodes() {
    let contract = bitmap_inference_metadata().payload.expect("the bitmap solve publishes a payload contract");
    assert_eq!(contract.payload_schema_id, BITMAP_INFERENCE_PAYLOAD_SCHEMA);
    assert_eq!(contract.progress_unit, "cells");
    assert!(contract.input_schema.contains("\"document\"") && contract.input_schema.contains("\"snapshot\""), "the request schema names both carriers");
    assert_eq!(contract.output_schema, include_str!("../../🔣️.json"), "the result schema is the facet leaf, carried verbatim");
    let binding = contract.artifact_binding.expect("this solve runs ON an artifact");
    assert_eq!(binding.field, "document");
    assert_eq!(binding.encoding, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    assert!(binding.required, "a bitmap solve cannot be stated by hand, so the binding is required");

    let wire = format!("{{\"{}\":{{\"pack\":\"AAAA\",\"spr\":\"AAAA\"}}}}", binding.field);
    let request: BitmapInferenceRequest = protocol::json::from_json_str(&wire).expect("the field the contract names is the field the request decodes");
    assert!(request.document.is_some() && request.snapshot.is_none());
}

/// 🚧️ A request that states neither carrier is refused by NAME, and the refusal says how to fix it
/// — never a default snapshot and never a panic.
#[test]
fn a_request_with_neither_carrier_is_refused_by_name() {
    let error = BitmapInferenceRequest { snapshot: None, document: None, checkpoint: None }.resolve_snapshot().expect_err("neither carrier is present");
    assert!(error.contains("inference-no-snapshot") && error.contains("artifactId"), "{error}");
}

/// 📸️ A stated snapshot still wins and is carried through untouched — the in-shell solve path,
/// which never had an artifact binding, is unchanged by this route.
#[test]
fn a_stated_snapshot_is_resolved_unchanged() {
    let snapshot = stripes();
    let resolved = BitmapInferenceRequest { snapshot: Some(snapshot.clone()), document: None, checkpoint: None }.resolve_snapshot().expect("a stated snapshot resolves");
    assert_eq!(resolved.seed, snapshot.seed);
    assert_eq!(resolved.output.width, snapshot.output.width);
}
//#endregion 📜️PublishedContract

//#region 🌱️ArtifactBoundGenesis
/// 🌱️ The canonical pair `artifact_create` of an `s.wfc.bitmap` answers over the semio MCP
/// (captured live, G4 session 10): the artifact-bound carrier must decode to the genesis snapshot and
/// that snapshot must solve, exactly like the stated-snapshot carrier.
const GENESIS_PACK_BASE64: &str = "iVNFTQ0KGgoSAAAAd2ZjLmJpdG1hcC5wYWNrIHYxiVNQSw0KGgoBAAAAAQAAAAEAAADvZe+IAAAAAAAAAAADAxAOY+Qp1itPS9ZLyizJTSwAAClm/p8EA6ABqgPjZWBjYGRhZ2IRYGYRYGG/weTo6OgMxEDgCaHSwaSTY6Crk6MjEAe6OoLZgUB1YNoTIpcOlgPqRVED1YdVP7odUHux6UezA2IvmhqoPuz2o9kBtRebflQ7oPbicD9W+7GFn60tK4sEG4sEOxMHCzMnCwcXE48IMwsDA7PaI8ZjjIxA+h5jCyMTA7PeJUYPZgbm/4wgyCvCwAxUA5QGygAA6gTz6AEDMjBj+PH28NHH/f2nXdQftkRY3Wg5UdRX+sW6Wen6vDyVf5dlHlusYmRUkGAAglXMvAyMAOkhXcwAAAB6o2RgU1BLRk9PVDEBAAAAAQAAAOIAAAAAAAAAOgAAAAAAAAB3AQAAAAAAAIpDqEbUaRgDDs1P6htRYTSodwW++npI2e6EP3NjDg6FAAAAAAAAAAA9mtYA";
const GENESIS_SPR_BASE64: &str = "iVNQUg0KGgoBAAAAAQAAAAEAAABv3uXPAAAAAAAAAAAqAwIBAAIXcy53ZmMuYml0bWFwQDEvKiNlZGl0b3IMcy53ZmMuYml0bWFwwYNpoTMAAAAHAQIBAQABAQY4LAwQAAAACQMCAQICATEBKl5NgEISAAAACkEAAQIBAQECAQNojUfJEwAAAEIMAgEAAAAAAAAAAAAAAAAAAABoAAAAAAAAAAQAAAAAAAAAT2/GOvLREraRMQ76Y5Hc2YPnnZpw7/JgKceH3PJJ3c5CncDjSwAAAA==";

#[test]
fn the_artifact_bound_genesis_document_resolves_and_solves() {
    let document = semio_framework_plugin::ArtifactDocumentPayload { pack: GENESIS_PACK_BASE64.to_string(), spr: GENESIS_SPR_BASE64.to_string() };
    let snapshot = BitmapInferenceRequest { snapshot: None, document: Some(document), checkpoint: None }.resolve_snapshot().expect("the genesis pair decodes");
    assert_eq!((snapshot.input.width, snapshot.input.height, snapshot.input.palette.len()), (16, 16, 3));
    assert_eq!((snapshot.output.width, snapshot.output.height, snapshot.output.periodic), (24, 24, true));
    assert_eq!((snapshot.model.pattern_size, snapshot.model.symmetry), (3, 8));
    let commit = solve_with_job(&snapshot).expect("the genesis snapshot solves");
    assert!(!commit.contradiction);
    assert_eq!(decode_base64(&commit.pixels).expect("pixels decode").len(), 24 * 24);
}
//#endregion 🌱️ArtifactBoundGenesis

//#region ⏱️RelayCrossingGrant
/// ⏱️ The language-agnostic law this region replays: `semio.infer`'s `Pump` must spend the host's
/// whole crossing grant on the mounted session instead of ending the crossing at the first outcome.
const RELAY_CROSSING_LAW: &str = include_str!("../../🧫️fixtures/⏱️relay-crossing-grant-law.json");

/// 🔁️ The reactor's job futures settle inside the state action that polls them, so one poll with a
/// no-op waker is the whole drive; a pending future here is itself a law violation.
fn settle<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(output) => output,
        std::task::Poll::Pending => panic!("a reactor job future must settle inside its own state action"),
    }
}

/// 🌱️ The exact wire request the semio MCP gateway sends for `inference_run` on an `artifact_create`d
/// bitmap: the genesis pair bound into the request body, the gateway's default work units.
fn genesis_wire_request(law: &serde_json::Value, cancellation_id: &str) -> Vec<u8> {
    let metadata = bitmap_inference_metadata();
    let workload = &law["workload"];
    let payload = BitmapInferenceRequest { snapshot: None, document: Some(semio_framework_plugin::ArtifactDocumentPayload { pack: GENESIS_PACK_BASE64.to_string(), spr: GENESIS_SPR_BASE64.to_string() }), checkpoint: None };
    let request = semio_framework_plugin::app::WireArtifactInferenceRequest {
        wire_version: semio_framework_plugin::app::ARTIFACT_INFERENCE_WIRE_VERSION,
        owner: metadata.owner.into(),
        artifact_kind: workload["artifactKind"].as_str().expect("law workload kind").into(),
        artifact_schema: metadata.artifact_schema.into(),
        artifact_schema_version: metadata.artifact_schema_version,
        inference_schema: workload["inferenceSchema"].as_str().expect("law workload schema").into(),
        inference_schema_version: metadata.inference_schema_version,
        algorithm_version: metadata.algorithm_version,
        policy_version: metadata.policy_version,
        revision: 1,
        generation: 1,
        source_dialect: "s.wfc.bitmap.standard.v1.dialect.canonical".into(),
        policy: Vec::new(),
        budgets: semio_framework_plugin::app::WireArtifactInferenceBudget { allocation_bytes: workload["allocationBytes"].as_u64().expect("law allocation"), work_units: workload["workUnits"].as_u64().expect("law work units"), recursion_depth: 4 },
        cancellation_id: cancellation_id.into(),
        previous_state: None,
        requested_cache_mode: semio_framework_plugin::app::WireArtifactInferenceCacheMode::Cold,
        canonical_payload: protocol::json::to_json_string(&payload).into_bytes(),
        dependencies: Vec::new(),
    };
    protocol::json::to_json_string(&request).into_bytes()
}

/// 🏁 Drives one `semio.infer` job through the guest job protocol under `grant` exactly as the host
/// relay does, and reports how many `step-job` crossings it took and how long.
fn run_genesis_crossings(law: &serde_json::Value, job: u64, grant: &serde_json::Value) -> (usize, std::time::Duration) {
    use semio_framework_plugin::reactor::jobs::{start_job, step_job, JobBudget, JobStep, JOB_KIND_INFER};
    register_bitmap_inference_factory(&semio_framework::ActionBus::production()).ok();
    let budget = JobBudget { fuel: grant["fuel"].as_u64().expect("law fuel"), deadline_ms: u32::try_from(grant["deadlineMs"].as_u64().expect("law deadline")).expect("law deadline fits") };
    let started = std::time::Instant::now();
    settle(start_job(job, JOB_KIND_INFER, &genesis_wire_request(law, &format!("wfc-relay-crossing-law-{job}"))));
    for crossing in 1..=5_000_000usize {
        match settle(step_job(job, budget)) {
            JobStep::Running(_) => {}
            JobStep::Done(bytes) => {
                let result: serde_json::Value = serde_json::from_slice(&bytes).expect("the inference result is json");
                assert_eq!(result["complete"], true, "{result}");
                return (crossing, started.elapsed());
            }
            JobStep::Failed(error) => panic!("the genesis solve failed after {crossing} crossings: {}", String::from_utf8_lossy(&error)),
        }
    }
    panic!("the genesis solve did not settle within five million crossings");
}

/// ⏱️ With a grant that covers the whole solve, the machine needs only its fixed state walk —
/// dispatch, the pump, the commit's outcome pages and the session's retirement — never a crossing
/// per preview, and it costs no more than the law's wall ratio over the headless native driver
/// solving the same snapshot in the same process. The crossing count is load-independent: a crossing
/// ends on the grant, a lossless outcome or the terminal, never on time.
#[test]
fn a_whole_solve_grant_settles_the_genesis_inference_in_a_bounded_state_walk() {
    let law: serde_json::Value = serde_json::from_str(RELAY_CROSSING_LAW).expect("relay crossing law parses");
    let grant = &law["wholeSolveGrant"];
    let snapshot = BitmapInferenceRequest { snapshot: None, document: Some(semio_framework_plugin::ArtifactDocumentPayload { pack: GENESIS_PACK_BASE64.to_string(), spr: GENESIS_SPR_BASE64.to_string() }), checkpoint: None }.resolve_snapshot().expect("the genesis pair decodes");
    let native_started = std::time::Instant::now();
    solve_with_job(&snapshot).expect("the genesis snapshot solves natively");
    let native = native_started.elapsed();
    let (crossings, relayed) = run_genesis_crossings(&law, 9_101, grant);
    let ceiling = grant["maxCrossings"].as_u64().expect("law crossing ceiling") as usize;
    assert!(crossings <= ceiling, "the genesis solve took {crossings} step-job crossings under a whole-solve grant ({relayed:?}); the law allows {ceiling}");
    let ratio = relayed.as_secs_f64() / native.as_secs_f64();
    let wall_ceiling = grant["maxWallRatioToHeadlessNative"].as_f64().expect("law wall ratio");
    assert!(ratio <= wall_ceiling, "the job protocol took {relayed:?} against {native:?} for the headless native driver ({ratio:.2}x); the law allows {wall_ceiling}x");
}

//#endregion ⏱️RelayCrossingGrant
