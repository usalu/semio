STATUS: GREEN (verified): semio-ui-prepared --lib 2/2 (W-replhist-105) and semio-framework-replication-history --lib passes (W-replhist-110, rc 0).

## ui-prepared shared-border laws (log C/🗑️generated/1105b/W-replhist-105.txt)
- The 392 B was NOT an unreceipted release. The drive turn's border finalization hands backing to the job (DrawList), charged to the drive caller's receipt; the job releases it later through its own close (another caller). observe_prepared_step now asserts full conservation after root closure: total.retained + drive.released == total.released + drive.retained (every retained byte is released by origin or roots, except what the drive turn handed to the job's custody). Not weakened.
- OriginalJobFixtureCaller::retire (job-context fixture) asserts roots_released == spent.retained, which is wrong when origin turns release too; helper no longer calls it (caller terminal already checked, then dropped). job-context untouched.
- Production fix (prepared 🔧️render-job-state-and-close/🦀️.rs): PreparedRenderJob::next_close_demands quoted depth 0 when delegating to the input, while close_granted requires job-level depth/items >= 1 (abandoned close of a job mid border-backing: DepthLimit). Now a wrapper raises items/depth to >= 1 for a non-terminal job (child quote in next_close_child_demands).
- Earlier fix retained: border-finalize push depth helper (testport).

## replication-history
- Errors were all in birth/🎟️original/🦀️.rs rejection_laws test (owner.step/close_original take a RetainedAllocationEffect, polls instead of outcomes, work is u128). Ported with crate-local `authored` + `settled`, u64::try_from on work; observe_backing now runs inside the effect. fold:92 is only a deprecation warning (fetch_update -> try_update), left.
- Blocked: observer's mid-edit in fold/🦀️.rs. Rerun: C/1105b-tools/runtime/tst.sh W-replhist-N 🧰️framework/Cargo.toml native -p semio-framework-replication-history --lib

## FINAL (wrap-up)
- W-replhist-107 (log C/🗑️generated/1105b/W-replhist-107.txt): the only lib test ran and failed at birth/original:240: heap released 103 vs receipt 80 in case "producer pending". The 23 B diff = the fixture's captured cause String (`complete original cause`) freed when the fold drops the never-polled future: fixture-owned heap, not fold production.
- Fix applied (W-replhist-109): the cause now lives in an Arc<Mutex<Option<..>>> the test keeps (`held`/`lent`), the future takes it only when polled. That cleared line 240 but W-replhist-109 (log ...109.txt) now fails earlier at :224 in the step_original loop: observed heap (64,0) vs receipt (0,0). Cause not investigated: likely the Arc/Mutex take or the first poll allocating 64 B inside job.step_original; compare with W-107 where :224 passed. Option: keep the cause as a plain capture and instead make the :240 expectation include the fixture string (account `detail.capacity()` when the future is dropped unpolled), or pre-allocate the Mutex outside the observation.
- Edits this session: prepared 🧪️testing/🦀️.rs (conservation, no retire), prepared 🔧️render-job-state-and-close/🦀️.rs (next_close_demands wrapper), fold birth/🎟️original/🦀️.rs rejection_laws test only (authored/settled port, held/lent cause). No [DEBUG] left (removed the case/step message before 109).
- Rerun: C/1105b-tools/runtime/tst.sh W-replhist-110 🧰️framework/Cargo.toml native -p semio-framework-replication-history --lib
- 04:31 110: Mutex initialised (locked once) before the observed region so its lazy box is outside the heap observation; running W-replhist-110.
- W-replhist-110 rc 0: replication-history lib test green. Rerun: tst.sh W-replhist-N 🧰️framework/Cargo.toml native -p semio-framework-replication-history --lib
