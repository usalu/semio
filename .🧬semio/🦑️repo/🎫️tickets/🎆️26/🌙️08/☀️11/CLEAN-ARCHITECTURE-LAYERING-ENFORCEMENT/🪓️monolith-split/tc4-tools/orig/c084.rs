/// ⏱️ Calibration: the deterministic machine-load calibration that scales the budget ceilings.

//#region ⏱️Calibration
const BUDGET_CALIBRATION_FIXTURE_JSON: &str = include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧫️fixtures/⏱️budget-calibration/🔣️.json");
const CALIBRATION_SCHEMA: &str = "s.procedural.generation3d.example-budget-calibration/v1";

/// ⚖️ The committed statement of how fast the machine that SET this lane's ceilings executes a
/// fixed, kernel-independent workload.
///
/// Every ceiling in every `budget` row, and `maxRoundTrips` too, is a wall-clock quantity measured
/// on one machine under one load. A wall clock on a shared build host measures the code plus
/// whatever else was resident — this repository's fleet routinely holds the load average above 50
/// while peers compile — so the same correct code reads two to five times over its ceiling and the
/// law convicts the machine instead of the algorithm. That is not a strict law, it is a noisy one,
/// and a noisy law gets ignored.
///
/// 🧭 The honest correction is a SCALE, not a looser ceiling: run a workload whose cost this
/// repository never changes, and divide out how much slower it is right now than when the ceilings
/// were set. A genuine algorithmic regression still fails, because it inflates the phase without
/// inflating the calibration; contention inflates both, and cancels
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️kernel-performance-2026-09-13.md` §6.1).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BudgetCalibration {
    schema: String,
    rounds: u32,
    checksum: String,
    reference_micros: u64,
    maximum_load_factor: f64,
}

/// ⏱️ How many timed passes a calibration reading takes, and which of them is believed.
///
/// 🐛️ The MINIMUM is the wrong statistic here, and measuring it proved so: with a pass costing
/// 1.8 ms the min of five passes read 1811 us under 48 spinners on a 10-core machine — byte for byte
/// its idle reading — while `FlowHost::evaluate` on the same machine at the same moment took an
/// order of magnitude longer than its ceiling. A workload short enough to fit inside one scheduling
/// quantum is never preempted, so its minimum measures a machine that is not the one the phase ran
/// on. The pass is therefore sized to ~100 ms — long enough to be preempted the way a real phase is
/// — and the MEDIAN of three is believed, which discards one lucky and one unlucky reading without
/// believing either.
const CALIBRATION_PASSES: usize = 3;

fn budget_calibration() -> BudgetCalibration {
    let calibration: BudgetCalibration = serde_json::from_str(BUDGET_CALIBRATION_FIXTURE_JSON).expect("budget calibration fixture parses");
    assert_eq!(calibration.schema, CALIBRATION_SCHEMA);
    calibration
}

/// ✅️ Every invariant the calibration row states about ITSELF.
fn assert_calibration_contract() {
    let calibration = budget_calibration();
    assert!(calibration.rounds > 0, "a calibration of zero rounds measures nothing");
    assert!(calibration.reference_micros > 0, "a reference of zero microseconds would scale every ceiling to infinity");
    assert!(calibration.maximum_load_factor >= 1.0, "the load factor may only ever widen a ceiling, never narrow one");
    assert_eq!(calibration.checksum.len(), 18, "the checksum is a 64-bit hex literal");
    assert!(calibration.checksum.starts_with("0x"));
}

/// 🎲 ONE deterministic calibration pass: a fixed LCG feeding a `orient2d`-shaped cross-product sum
/// and one owned `Vec` per round, so the pass exercises the same mix the geometry kernel's own cost
/// is made of — floating-point multiply/add chains plus allocator traffic — without touching a line
/// of kernel code that any lane in this repository may legitimately make faster.
///
/// 🪪️ Only the integer state is a checksum. The floating-point accumulator is returned so the work
/// cannot be elided, but never asserted: `a * b + c` may be contracted into a single fused
/// multiply-add on one target and not on another, and a calibration must not fail on that.
fn calibration_pass(rounds: u32) -> (u64, f64) {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut acc = 0.0f64;
    let mut samples: Vec<f64> = Vec::with_capacity(1024);
    for _ in 0..rounds {
        samples.clear();
        for _ in 0..1024 {
            state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
            samples.push(((state >> 11) as f64) * (1.0 / 9_007_199_254_740_992.0));
        }
        for window in samples.windows(4) {
            acc += window[0] * window[3] - window[1] * window[2];
        }
        let owned: Vec<f64> = samples.iter().map(|sample| sample * 1.000_000_001).collect();
        acc += owned.iter().sum::<f64>();
        state ^= owned.len() as u64;
    }
    (state, acc)
}

/// ⏱️ The machine's CURRENT cost for that workload, in microseconds — the median of
/// [`CALIBRATION_PASSES`] timed passes, each one checked to have produced the committed result.
fn calibration_micros(calibration: &BudgetCalibration) -> u64 {
    let mut readings = Vec::with_capacity(CALIBRATION_PASSES);
    for _ in 0..CALIBRATION_PASSES {
        let started = std::time::Instant::now();
        let (state, acc) = calibration_pass(calibration.rounds);
        let elapsed = started.elapsed().as_micros() as u64;
        std::hint::black_box(acc);
        assert_eq!(format!("{state:#018x}"), calibration.checksum, "the budget calibration workload must be bit-identical wherever it is measured, or its reading is not comparable to the committed reference");
        readings.push(elapsed);
    }
    readings.sort_unstable();
    readings[readings.len() / 2].max(1)
}

/// ⚖️ How much slower this machine is RIGHT NOW than the one that set the ceilings. Never below
/// `1.0` — a faster machine tightens nothing, because the ceilings are regression guards with
/// deliberate headroom, not targets — and never above the row's own `maximumLoadFactor`, past which
/// the reading is contention noise rather than a scale anyone should trust.
fn machine_load_factor() -> f64 {
    let calibration = budget_calibration();
    let measured = calibration_micros(&calibration);
    let raw = measured as f64 / calibration.reference_micros as f64;
    let factor = raw.clamp(1.0, calibration.maximum_load_factor);
    println!("[CALIBRATION] micros={measured} reference={} raw={raw:.2} factor={factor:.2}", calibration.reference_micros);
    factor
}

/// ✅️ Holds one measured phase to its committed ceiling — and, only when the raw reading overruns,
/// to that ceiling scaled by the machine's calibrated speed at that moment. An idle lane pays for no
/// calibration at all; a contended one pays five short passes and is judged on what the code cost
/// rather than on what the machine was doing.
fn assert_phase_budget(example: &str, phase: &str, measured: u64, ceiling: u64, regression: &str) {
    if measured <= ceiling {
        return;
    }
    let factor = machine_load_factor();
    let scaled = (ceiling as f64 * factor) as u64;
    assert!(
        measured <= scaled,
        "{example}: {phase} took {measured} us (best of {TIMING_ATTEMPTS}) against a {ceiling} us ceiling — {scaled} us after this machine's measured {factor:.2}x load calibration — {regression}, see 📓️kernel-performance-2026-09-13.md"
    );
    println!("[BUDGET] {example}: {phase} {measured} us is over its {ceiling} us ceiling but inside the calibrated {scaled} us — this machine is {factor:.2}x slower right now than the one that set it");
}

/// ⚖️ LAW: the calibration itself. Its workload is bit-identical everywhere — otherwise its reading
/// is not comparable to the committed reference and every scaled ceiling is a guess — and the factor
/// it derives never NARROWS a ceiling, so a fast machine can only ever judge the committed numbers
/// as they stand.
///
/// 🪪️ This law is what makes the scaled ceilings auditable: it prints the machine's raw reading, so
/// any lane that reports a budget failure can say in one line whether the machine was the cause.
#[test]
fn the_budget_calibration_is_deterministic_and_never_narrows_a_ceiling() {
    assert_calibration_contract();
    let calibration = budget_calibration();
    let (first, _) = calibration_pass(calibration.rounds);
    let (second, _) = calibration_pass(calibration.rounds);
    assert_eq!(first, second, "the calibration workload must be deterministic within one process");
    assert_eq!(format!("{first:#018x}"), calibration.checksum, "the calibration workload no longer produces the committed result — its reading cannot be compared to the committed reference");
    let factor = machine_load_factor();
    assert!(factor >= 1.0 && factor <= calibration.maximum_load_factor, "the load factor {factor} left its declared range");
}
//#endregion ⏱️Calibration
