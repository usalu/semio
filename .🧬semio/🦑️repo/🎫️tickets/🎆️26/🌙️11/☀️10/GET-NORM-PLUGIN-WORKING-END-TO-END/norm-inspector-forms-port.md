# Norm inspector-forms port to the effect-threaded retirement API

Crate: `semio-s-plugin-norm-inspector-forms` under `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪺️inspector-forms`.
Status: source port complete, NOT compiled (build pause was active; no repo run started). Every claim below is from reading the current framework sources. First action on resume: `repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --all-targets -p semio-s-plugin-norm-inspector-forms` from the norm workspace together with `semio-s-plugin-norm-app-surface-viewport3d`, then `test` for both.

## What was wrong
All 40 errors came from `paid/static-source.rs` and `paid/page.rs` (the only retirement state machines in the crate):
- `RetireOwned::retirement` / `RetirementCursor::close_step(grant)` no longer exist (now `type RetirementCursor` + `retirement_seed`, cursor replies through `close_into` / `native_retirement_reply!`).
- ledger identity is `NativeAllocationIdentity`, not `u64`.
- `RetainedCloneSource::close_step`, `RetainedCloneSourceAdmission::{advance,take_output}`, `JsonNativeInputCursor::{advance,take,close_step}` now take `(body, effect)` / an effect and a Source witness, and report a `RetainedWorkPoll`.
- the old `new(cache,(),|c|c.get())` projection closure is gone; the owner authority is now sealed by the admission itself.

No other norm crate uses the retired protocol (grepped `fn retirement(self)`, `close_step(grant`, `ledger_identity`, `retirement_birth_bytes` across `✏️s/🔌️plugins/📕️norm`; the sqlite snapshot files already use `NativeSnapshotDecodeOwner`).

## Design of the port
- `paid/funding.rs` (new): shared `pay`, `run` (one transition inside `NativeDecodeControl::with_allocated_effect`, so every unit of work is charged to the original Work authority and every allocation is quoted), `step` (close poll to `RetirementStep`) and the `derived_cursor_quotes!` macro (all secondary cursor quotes derive from the single authored next-close demand).
- `paid/static-source.rs`:
  - the source type is now `RetainedCloneSource<Vec<T>>` (owner `Vec<T>`, identity projection); `SealedShared<Vec<T>>` cache plus projection closure removed (no `OriginalSourceProjection` impl exists for `SealedShared`, and norm cannot add one).
  - admission is `StagedSourceAdmission::owned`, driven by `advance_owned` / `next_owned_demand` / `next_owned_work_demand` (handles the `projectionStart` phase) and retired through `RetainedCloneClose::begin_granted`.
  - `NormInspectorStaticSourcePreparation::advance(control, body, maximum_work, grant)` returns `RetainedWorkOutcome<RetainedCloneStep, NativeReceivingFailure>`; the caller supplies `maximum_work` (no constant). Quotes: `demands(body, maximum_work)`, `advance_work_demand(maximum_work)`.
  - close ladder `close_step(control, effect) -> OriginalRetirementPoll` with `close_demands` / `close_work_demand`, mirroring `JsonNativeInputCursor::close_step`; the pending `retirement` is always continued first so quote and step select the same owner.
  - `NormInspectorPreparedStaticSource<T>`: `RetireOwned { RetirementCursor = Self, retirement_seed = identity }`, quotes from `RetainedCloneSource::next_close_demand/next_close_work_demand`, effect-shaped `close_step(body, effect)`.
- `paid/page.rs`: `ledger: NativeAllocationIdentity`; each preparation transition (`advance`, `take`) now runs under `run` (one paid work unit, quoted); `close_step(control, effect)`, `close_work_demand`; `NormInspectorPreparedPage` gets `close_poll`, `close_step(body, effect)`, `next_close_work_demand` and the same `RetireOwned` / `RetirementCursor` shape.
- Framework fix needed by this crate (root cause, `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🪪️native-input/🦀️.rs`, the only consumer of `JsonNativeInputCursor` is norm): its `Phase::Source` never handled the `projectionStart` phase of the raw admission (the sibling `OriginalJsonOutputReceiving` does), so every cursor stalled at "original projection must be driven". Fixed: projecting branch (`project_owned`, tag 16), projection demand in `demands`, work quote `advance_work_demand` for the Source phase; added `close_work_demand(body)` and `take_work_demand()` (both required to open exactly one funded effect per action, like `OriginalJsonOutputReceiving`).

## Tests (re-expressed, none deleted)
`paid/static-source-tests.rs` now runs on the shared test infrastructure instead of the retired job fixture caller: `EffectBank` + `session_nested` / `session_driven`, the shared `receiver-driver.rs` (`driver::bank/admit/receive_cut/close/retire`) and `observe_retirement_allocations` for exact heap-versus-receipt equality on every turn. Same cases as before: exact receipts and cancel at every prefix for definitions and seeds, denied frontiers on every demanded axis (zero work, zero progress, zero heap), constructor denials, the 73-byte heap witness, array/choice/variant page tests, the borrowed-choice zero-allocation check, and the two hand-off tests (page survives its first caller and lends a native source to the next caller, via `RetainedOwnedProjectionAdmission` and `driver::receive_cut` on a second bank). Dev-dependencies: removed `semio-framework-job(-native-decode)`, added `semio-framework-value-testing-admission` and `-testing-allocator`; `Cargo.lock` entry of the crate updated by hand.
Known simplification: the old tests used a production `OriginalNativeDecodeSource` decoder for the hand-off cases; the new hand-off cases use two independent effect banks (caller one returns, caller two lends), which is what the shared infrastructure offers.

## Risks to verify at first compile
1. `with_retirement` is called inside `with_allocated_effect` / `with_effect` in the test `Owner::close`; if the retirement loan refuses a live work action, split into `session_driven` over `ControlledRetirement` for the preparations.
2. `ManuallyDrop` field auto-deref for `begin_granted(&mut self.admission, ..)` and `advance_owned(source, ..)` (same pattern as `JsonNativeInputCursor`).
3. Page demand `copy_bytes` doubles as the effect grant; the receipts already equal the demand, so heap equality should hold.
4. maxSourceFiles (32) and maxActiveLines (4000) of the crate: 17 sources, well inside.

## Not done / blockers
- Nothing compiled or run (pause). `norm-app-surface-viewport3d` QuantityKind fix was not touched by this task (already in source, unbuilt).
- No `[DEBUG]` lines were added; no temp files in the repo (scratch copies of the old test file are in the session scratchpad only).

---

# normforms2 verification pass (post-merge)

Logs: `🗑️generated/normforms2-check.log`, `normforms2-check2.log`.
- Check 1 (`check --all-targets`, norm workspace, both crates): inspector-forms lib compiled with zero errors (the port holds). One test error: `NativeTreeReceiver::terminal_is_empty` is a `NativeReceiver` trait method, trait not imported in `paid/static-source-tests.rs`. Fixed by importing `NativeReceiver` (line 6).
- Check 2: inspector-forms lib + lib test compile clean. viewport3d lib test failed with 2 errors: `tests/unit` line 386 built `Pattern::Cage { stirrup_spacing, count }`, but the scene schema is now `Cage { outline, zones, reference }` (count/spacing live in `LinkZone`). Fixed in source with a closed `Ring2` outline, one `LinkZone { start 0, spacing 0.15, count 5 }`, reference `Vec3(0,0,1)` (same shape as structural-scene tests). Unverified by compile.
- NOT RUN because of BUILD PAUSE #5: re-check of viewport3d, `test` for both crates (watch `with_retirement` inside `with_allocated_effect` in `Owner::close`), json native-input crate tests (framework fix: projectionStart, close/take work demands), en1992/en1993 editor/viewer artifact checks.
- No `[DEBUG]` lines added.

Resume order: (1) check --all-targets both crates, (2) test both, (3) json native-input crate tests from the framework workspace, (4) en1992/en1993 artifacts.

---

# normforms3 pass (post-merge, logs `🗑️generated/normforms3-*`)

1. `check --all-targets -p inspector-forms -p viewport3d`: 0 errors (28 min, mostly queue time).
2. `test --lib` results, last run `normforms3-test6.log`:
   - viewport3d: 44/44 pass after four stale-test fixes (tests/unit): Plan preset compares `position[0]` to `target[0]` (centre is x=2, not 0); vertex-field resample expects the parametric value `1.5 + 10*0.25` (4 values over 7 vertices, mid vertex); `hatch_ribbons` turned by pi/2 must equal the Vertical pattern (`along_ratio` helper removed as inverted); laminate and hatched-box entities used layer `"main"` which the fixture does not define, now `"members"` (0 instances before).
   - inspector-forms: 11/19 pass. `insertion_seed_contains_only_one_declared_variant` fixed at the root in the framework: `time_travel_default_value` (`⏪️time-travel/🪟️panel/🔢️pointer/🦀️.rs`) now seeds an object with only the variant its selector starts as (it previously seeded every required field of every variant). Docstring updated.
   - 8 `paid::static_source::tests::*` still red. Progress so far in `static-source-tests.rs`: (a) denied close now opens the retirement loan with the full session grant and denies only the effect (`with_retirement(grant())`, the loan refuses zero-item grants at `begin`); (b) `receive` unwraps the result before posting, so errors are not masked by the heap-vs-receipt assert; (c) advance turns use the exact quote grant (`exact(demand)`, denials `denied_from(funded, axis)` one below the quote) like `receiver-driver.rs`; this removed `native decoding ownership exceeds caller limit` (a generous 1 MiB grant against `receiving_capacity`). Risk 1 (`with_retirement` inside `with_allocated_effect`) did not fire.
   - Current failure for 7 tests: at the first funded turn of the cursor's Receive phase, `original work requires its original issuer and resolved quote` (`🎟️work-action/⚙️operation`), i.e. the json cursor's `Phase::Receive` (`NativeReceivingOriginal::advance`) and `local()` work actions need the driven-producer authority (`driven::<OriginalReceivingPrimary>` / `Local{work}` selection in `receiver-driver.rs`), not `session_nested`. Next step: drive the preparation's Read/Receive turns through `driven`/`bank_run_with_source` (or expose `select_advance`-style selection on the cursor) instead of `session_nested`. The seeds test additionally fails in close with `original projection frame and answer must return before its inputs` (close order of the cursor admission), not yet analysed.
3. Json native-input crate tests (framework): NOT run (build pause #6). en1992/en1993 check and test: NOT run; they depend on inspector-forms lib tests only for `test`, the lib already compiles.
4. No `[DEBUG]` lines left by this task. BUILD PAUSE #6: stopped after test6.
