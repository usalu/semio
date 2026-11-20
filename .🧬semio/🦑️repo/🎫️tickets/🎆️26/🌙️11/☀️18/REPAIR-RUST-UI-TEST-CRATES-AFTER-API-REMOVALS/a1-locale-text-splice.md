# 🧪️ Repair UI test crates after the value API removals (locale, text-splice, contract-values)

## Results per crate

| Crate | Before | After | Run |
|---|---|---|---|
| `semio-framework-text-splice-native` | did not compile | **4 passed, 0 failed** | `a1-splice-14` |
| `semio-framework-value-native-receiving-original` (reference law) | 0 passed, 7 failed (`a1-dep1-2`) | **7 passed, 0 failed** | `a1-original-10` |
| `semio-ui-locale` | did not compile (9 errors) | 16 passed, 1 failed | `a1-locale-31` |
| `semio-ui-contract-values` | `[[test]] native-bounded-ui-receiving` did not compile | the `[[test]]` is gone; the lib test `text_chunks::tests::utf8_text_chunk_transport_matches_shared_vectors` fails (see below) | `a1-values-1` |
| `semio-ui-contract-values-laws` | — | did not compile: 10 errors, all in the arena laws (see below) | `a1-laws-1` |

## Root causes fixed

1. **Splice expiry (job).** The documented contract is "🧹️ Physical close spends the same finite source even when its semantic deadline expired" (`🧵️job/⏱️context/README.md`; `StepContext::retirement_should_yield` exempts original retirement). Three places broke it:
   - `🧵️job/⏱️context/🦀️.rs`: `OriginalRetirementTurn::with_effect` hard-coded `retirement=false`. The pump is the only issuer of owner-less retirement turns, so it now stays admitted after the deadline. The production headless caller already relies on this: it pumps parent close while `retiring` past its deadline.
   - `🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs`: the fixture caller closed the return birth outside `with_original_retirement`. The production headless caller wraps it, so the fixture now does too.
   - The splice law's decoder teardown now runs inside `with_original_retirement`.
2. **Composer construction (production).** `✂️text-splice/🧬️native/🦀️.rs` `construct` never reported its actual allocation. As a result every admission ended `Failed`, and a 10192-byte box was left without a receipt.
3. **Receiving progress re-posted earlier turns (value).** `🧳️original/🦀️.rs` `NativeReceivingOriginal::advance` covered each turn with the construction recipient's *lifetime* `submitted()`. It now covers only what was accepted since the turn's baseline (`since_progress`). A recipient lent across turns (the fixture session authorities, the reference law's local authorities) is cumulative by design. This fixed the reference law's denied-turn and over-grant failures.
4. **Nested grant accounting (value).** Changes in `🧳️original/🦀️.rs`:
   - Nested actions are lent the turn's *remaining* grant.
   - A nested action left `Pending` resumes under its recorded grant (`nested` field), so schema key requests keep their locked grant.
   - A nested action starts after the first action of a turn only while the remaining grant still covers a schema metadata step.
5. **Schema metadata items (value).** `🧩️record/🔎️schema/🎛️drive.rs` `advance_metadata` posts three construction items (the action, its Source Post, the completion release), but clamped its report to 1. Each post's progress is fixed by the contract (`publish_completion` requires `payout.progress == completion.progress`). So the step now:
   - reports its true item count;
   - declares it as `NATIVE_SCHEMA_METADATA_ITEMS = 3`, re-exported as `semio_framework_value_native_owned::owned::NATIVE_SCHEMA_METADATA_ITEMS`;
   - stays idle under a grant with fewer items.

   The laws fund turns with that item count: the locale law and the reference law's `quoted`. The stage-by-stage shell law keeps one item per turn.
6. **Locale law.** Work ceiling per turn is now constant (`TURN_WORK`): a resident action is refused if its `maximum` changes between turns. Suspended turns settle as zero progress carrying their work, as the original law did.

The shared admission fixture (`🌱️value/🧪️testing/📥️admission/driven.rs`) is back to its original content, byte for byte (16272 B). The root fix is item 3, so the fixture change is no longer needed and nothing that depends on it changes.

## Contract-values port

- The test `🗺️fixed-map-and-bytes/🛬️receive/🧪️tests/🦀️.rs` is rewritten onto the shared `📥️admission/receiver-driver.rs`, which `🎠️kernel/📥️events` and the value receiver laws also use.
- For every case and cancel prefix, the case is received or cancelled, then the receiver, the output and the genuine Source are closed. Every demanded axis is denied first.
- The serde oracle check is kept, and refused rows must fail.
- The test is mounted in the laws crate as `bounded_receiving_tests`. The laws crate gained dev-deps `semio-framework-value-testing-admission` and `semio-framework-value-native-owned`.
- The values crate's `[[test]]` entry is removed, so its sources did not grow.

## Still failing / not verified

- **`semio-ui-locale`, label case.** Not stable across runs while others edit `🌱️value` at the same time:
  - `a1-locale-31`: `original locale receiving action did not settle` (settle loop idle under a 3-item quote, demand copy 128 / depth 4).
  - `a1-locale-32` (same sources, one concurrent edit later): `posted progress … heap (432, 368) vs (0, 0)` from `Wallet::post`.

  The locale and terminology cases pass every prefix, denial and close. What remains is how a nested schema key or metadata step settles inside `NativeLabelTerminologies` under exact one-turn quotes.
- **`semio-ui-contract-values-laws`.** Compile errors are only in `🏟️collection-arena/🌱️opening/🧪️tests/🦀️.rs` (mounted by the coordinator's `🏟️arena` laws, edited at 19:08): `post` work type `u128`→`u64`, and a private `pointer()`. My module compiles.
- **`semio-ui-contract-values`** lib test `utf8_text_chunk_transport_matches_shared_vectors` fails: `UI arena has no accepting lifetime registration` (`🏟️collection-arena/🦀️.rs:138`). This is arena registration work, not touched by me.
- **Dependent crates of the changed shared code were not re-run** after the final changes. Round 1 (`a1-deps.sh 1`) was stopped when the fixture change was reverted. Its only result that is still current: the admission crate's own tests `marker.rs` / `original-close-marker.rs` do not compile (they use the removed `capture` / `close_owned`), which predates this ticket. To re-run: `a1-deps.sh 2`.

## Follow-up: regression check of the changes outside the UI tree (runs `a1-reg*`, `a1-admission-3`, `a1-reg5-job-baseline`)

Changed files outside 🖱️ui:
- `🧵️job/⏱️context/🦀️.rs` (retirement pump `retirement=true`)
- `🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs` (birth close under `with_original_retirement`)
- `🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs` and `🪞️projection/🦀️.rs` (per-turn construction cover; remaining nested grant; recorded grant for a started pending nested action; mid-turn nested only while the grant covers a metadata step)
- `🌱️value/🛬️decode/🧳️owned/🧩️record/🔎️schema/🎛️drive.rs` + `🦀️.rs` (`NATIVE_SCHEMA_METADATA_ITEMS`, metadata gate). The constant is **not** re-exported from native-owned any more: that crate is already over its source boundary, so dependents use `semio-framework-value-native-schema` directly (dependency added to the original receiving crate; dev-dependency added to locale).

| Package(s) | Result | Attribution |
|---|---|---|
| job batch: job, job-capture, job-context, job-failure, job-native-decode, job-native-encoding, job-native-turn, job-park, job-receiving, job-return, job-testing-caller, job-worker | 50 suites ok, 20 failed, 74 failing tests (`a1-reg4-job`) | **None caused by this ticket.** The same packages with my two job edits temporarily reverted (`a1-reg5-job-baseline`) gave the same 74 failures plus one more (`original_float_field_uses_same_200_call_fuel_stock_and_cancel_deadline_authority`, flaky). job-worker's ~31 `component::` failures appear in both runs. My edits match HEAD apart from these two lines (`git show HEAD:` had `retirement=false` and the unwrapped birth close). Examples from outside this ticket: `birth-chain.rs:34` "table never born" (before any retirement); `🎟️source/🏠️native/🧪️tests/🦀️.rs:84` `with_native_retirement_receiving`. |
| semio-framework-value-native-receiving-original | **7/7 pass** (`a1-reg10-own`) | — |
| semio-framework-value-native-schema | **4/4 pass** | — |
| semio-framework-value-testing-admission | **5/5 pass** (`a1-admission-3`) | Pre-existing stale tests now ported: `marker.rs` (staged `admit_owned` admission, work debited once, Source closed to terminal; the per-stage `actions` list in `marker.json` is no longer observable in Rust and stays checked by `marker.ts`), `original-close-marker.rs` (close through `FixtureRetirementProducer` with reply and producer transfers; zero-work denial, birth markers, release conservation), `decode-effect-driver.rs` (fixture `Authorities::effect`). |
| value-derive, value-edit, value-native-list-receiver, value-native-reservation, value-native-shared-slots, value-ordered | 26 suites ok, 4 failed (`a1-reg11-rest`) | **Not from this ticket.** None of these tests touches `NativeReceivingOriginal`, schema metadata or the pump. The reservation `parent_tests` failure already failed in round 1 (`a1-dep1-4`, `left 0 right 5`). Others: derive syn-oracle tests; `🔑️named/📇️index/🧪️tests/helpers.rs:40` heap (537,239); `🏭️construction/🧳️slots/🧪️tests/owner.rs:189/144`; `📦️handoff/🦀️.rs:19`. |
| Not testable, preflight or compile fails in tests that predate this ticket | — | dsl-record (`🗣️dsl/🧬️schema/🛬️receive/🧪️tests/🦀️.rs`: `mod funding` file missing; blocks every batch containing dsl crates), value-native-owned (source boundary: `receiver-driver.rs` 16635 B, 13528 B left), value (`📦️paged` append/record tests), value-dictionary (`🟰️equal/🧪️tests.rs`), value-snapshot-read, value-snapshot-projection, value-paged, value-native-shared-engine, value-shared-issuance, value-native-default (`🎟️work/🧪️tests`), value-native-owned-try-map, value-native-tree-engine. |

## Follow-up: locale label case — still failing

`semio-ui-locale`: 16 passed, 1 failed (`a1-locale-40`).

- The earlier `heap (432,368) vs receipt (0,0)` failure came from my own temporary `[DEBUG]` traces: their `format!`/first `eprintln!` allocated inside the observed region. It is gone.
- The remaining failure is the label case: `original locale receiving action did not settle` (`axis-value/🦀️.rs:99`, `settle_pending`).

What the allocation-free traces showed (`a1-locale-37/38`, since removed):
- A funded receiving turn makes the nested `NativeLabelTerminologies` schema key action resident: `recvpending=true`.
- The first nested poll returns `Pending{work:0}`.
- Every later turn under the same quote returns `Pending{work:0}` or idle zero progress, so the action never settles.
- The resumed poll goes through the Source recover path. `OriginalLabels::recover_scalar` returns `reply.pending()`, as every in-tree source does (including `DslValue`).

Why the first poll suspends with zero work, and therefore which owner is at fault, is not resolved. Candidates are the matcher key frame (`🧩️record/🌳️source/🗝️key-frame.rs`) and the `NativeReceivingOriginal` nested scheduling from this ticket. The reference law over `DslValue` rows passes 7/7 with the same receiving code.

No `[DEBUG]` lines remain in the files I touched.

## Follow-up 2: contract-values laws and locale label case

### `semio-ui-contract-values-laws`: green
```text
test result: ok. 43 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (a1-laws-3)
```
Two root causes:
- **Missing terminal destructor (production).** The fixed-map element `UiText` receiver (`🔤️text/🛬️receive/🦀️.rs`) and the recursive `UiValueNativeReceiver` (`🌱️values/🛬️receive/🦀️.rs`) did not implement `NativeReceiver::native_terminal_release`. The receiver backing close requires it (`🧬️protocol/♻️backing/🦀️.rs`: "native receiver header lacks an admitted real terminal destructor"). Both now provide it exactly as the value-crate receivers do (scalar, string, option, sequence, list, tuple).
- **Number oracle.** `UiValue` numbers are `f64`, so serde emits `9007199254740992.0` where the JSON fixture holds the integer `9007199254740992`. The law now compares JSON numbers by value (`numeric`, test-only, in the laws mount).

### `semio-ui-locale`: label case still red
```text
test result: FAILED. 16 passed; 1 failed   (a1-locale-42)
```
Root causes found and fixed:
1. **The law's custom Source did not implement the schema-key protocol.** `NativeValueSource::query_schema_key` defaults to `Err(UnsupportedOwner, "Source lacks a recorded schema-key producer")` (`🧩️record/🌳️source/🦀️.rs`). The record matcher of `NativeLabelTerminologies` needs it to read field names, so its first key step parked with zero work forever. That is what caused the "did not settle" stall. Every in-tree source (`DslValue`, `String`, tuples) implements it via `schema_key_source!` or `answer_schema_key`. `OriginalLanguages` and `OriginalLabels` now answer through `answer_schema_key`. (`recover_scalar` answering `pending` is protocol-conform, and in-tree sources do the same.)
2. **The fixture close producer had no Source turn.** `🔎️schema/🦀️.rs` `close_dispatch` requires `invocation.source_turn() == effect.work_source_turn()`, but `FixtureRetirementProducer` always answered `None` ("schema close changed Source or Work"). The fixture producer now carries an optional turn: `new(owner, turn)`, where `Authorities::source_turn()` exposes the payer's turn. Users: the locale law passes `Some(payer.source_turn())`; `retirement.rs` and `original-close-marker.rs` pass `None`. Admission crate after the change: all suites ok (`a1-admission-4`).

Remaining failure: one funded close turn of the label owner. `Wallet::post` sees heap `(128, 64)` while the close step reports `{items 1, copy 8, capacity 0, release 0}` (quote copy 8, depth 5). The construction recipient received no submission in that turn (traced, delta 0). So a nested label close action, five levels below the owner (mapped → record → schema matcher / key backing), allocates and frees without receipting it. The identical close path over `DslValue` rows passes in the reference law (7/7). The unreported allocation sits in the close of the label's nested schema/record receivers, which this ticket has not yet isolated.

## Follow-up 3: unreceipted close heap, contract receiving laws

### Root cause of the unreported close heap: a foreign `[DEBUG]` probe
I traced it allocation-free: `install_allocation_tap` wrote sizes into static atomics, and libc `backtrace()` captured frames into a static buffer; both are printed only after the turn.
- Result in the failing close turn: `+64 +64 -64`.
- All three come from `ControlledCore::step_original` in `🌱️value/♻️retirement/🎮️controlled/🦀️.rs:266`:
  ```
  {static N:AtomicUsize…;if (100000..100004).contains(&n){let names:Vec<_>=…debug_type_probe()…collect();eprintln!("[DEBUG] stack={:#?}",names);}} // [DEBUG]
  ```
- This line is not in HEAD. It was added by another agent (file mtime 2026-11-18 22:55).
- Its `Vec` and its first `eprintln!` (stdout mutex init) allocate inside the measured retirement turn, on whichever turn is process-wide call number 100000 of `step_original`.
- It is not an accounting bug of the record or schema close.

Effect on the laws:
- Locale label case: heap `(128,64)` / `(2197,1069)` against the receipt; the run prints `[DEBUG] stack=[…]` twice (`a1-locale-47`).
- `semio-ui-contract` graph law: heap `(537,287)` against the receipt; the run also prints `[DEBUG] stack=[…]` (`a1-contract-5`).

I did not remove another agent's in-progress probe. Removing the line at `🎮️controlled/🦀️.rs:266` is the fix for both.

### Fixed in this follow-up
- **Leaf stall (`receiver stalled: 1024 consecutive funded turns`)** in `semio-ui-contract` and `semio-ui-contract-presentation`. This was my regression: the `KeyMetadata` items gate in `🎛️drive.rs` idled schema metadata steps under the 1-item grants of `receiver-driver.rs`. The gate is removed and the step's one-item report (`min(1)`) is restored, so `drive.rs` behaves as in HEAD again; only `NATIVE_SCHEMA_METADATA_ITEMS` is added. `NativeReceivingOriginal` still covers the true 3 construction items through the per-turn accepted delta. It also starts a mid-turn nested action only while the remaining grant covers that many items.
- **Graph law `input.is_none() && arena.is_some()`.** The process-wide UI arena registry was taken by concurrently running laws that hold `UiArenaFixture`'s serial scope. The graph law now uses `UiArenaFixture` itself instead of its own unserialized admission (`🧬️contract/🧪️tests/🛬️receive/🦀️.rs`).
- **`{"type":"separator"}` refused (production, derive).** An internally tagged variant flattens its payload record. With an empty payload (`SeparatorProps {}`), no key ever matched the flattened field, so its position stayed `None` and the required-field check refused it. Flattened fields read from the whole original (`usize::MAX - i` projection), so the derive now initializes their positions as present (`✨️derive/🛬️receive/🦀️.rs`). Derive suites: all pass, except the 2 syn-oracle unit tests that already failed before (`a1-contract-4`).
- **Number oracle** in the shared leaf law (`🖼️presentation/🛬️receive/🦀️.rs`): JSON numbers are compared by value (`1.0` == `1`).

### Final test results
```text
semio-ui-contract-presentation: test result: ok. 42 passed; 0 failed            (a1-contract-5)
semio-ui-contract:              test result: FAILED. 19 passed; 1 failed          (a1-contract-5; graph law, foreign [DEBUG] probe)
                                test result: ok. 12 passed; 0 failed (catalogue-carrier-map)
semio-ui-contract-values-laws:  test result: ok. 43 passed; 0 failed            (a1-laws-3)
semio-ui-locale:                test result: FAILED. 16 passed; 1 failed          (a1-locale-47; label close, foreign [DEBUG] probe)
semio-framework-text-splice-native: test result: ok. 4 passed; 0 failed         (a1-splice-14)
semio-framework-value-testing-admission: all suites ok                          (a1-admission-4)
semio-framework-value-native-receiving-original: test result: ok. 7 passed      (a1-reg10-own)
```

## Follow-up 4: patch ownership limit, final state

- The coordinating session removed the foreign `[DEBUG]` probe. After that, `semio-ui-locale` passes 17/17 (the coordinator's run).
- **Graph law, "patch" case (`OwnershipLimit "native decoding ownership exceeds caller limit"`).** Cause: the shared decoder session in `🌱️value/🧪️testing/📥️admission/driven.rs` (`open_session`, used by `receiver-driver.rs`) gave its `NativeDecodeControl` a fixed `1<<24`-byte ownership ceiling. That ceiling has nothing to do with the funding profile. A patch whose ops reserve more native ownership than 16 MiB was refused, although the bank funds 1 GiB of capacity. The session ceiling is now the bank's own funded capacity stock (`bank.wallet().remaining().maximum_capacity_bytes`). The magic number is gone, and decoding ownership is bounded by the same stock that pays for it.

```text
semio-ui-contract:              test result: ok. 20 passed; 0 failed   (+ catalogue-carrier-map ok. 12 passed)   (a1-contract-6)
semio-ui-contract-presentation: test result: ok. 42 passed; 0 failed                                              (a1-contract-6)
semio-framework-value-testing-admission: all suites ok                                                            (a1-contract-6)
```
