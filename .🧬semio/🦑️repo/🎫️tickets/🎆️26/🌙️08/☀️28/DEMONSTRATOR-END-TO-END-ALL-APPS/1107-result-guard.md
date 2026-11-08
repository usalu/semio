STATUS: RED/PARTIAL. Wasm32-wasip2 lib: 93 of 100 value/job crates compile, 7 red (guard-101). Native --tests: 23 crates red (guard-104). Tests PASS column UNVERIFIED for every crate: the five runs that tried to run tests were each stopped before any test binary by a source-boundary refusal or a lib compile error (guard-105, 106, 107, 108, 110, 111, 112, 113), the last time by the in-flight job-context edit of joblaws.

# 1107 guard lane

## statik-233 clusters (step 1)
The only `error` in `statik-233.txt` is `semio-os-store-snapshot-clone` (58 errors: unresolved `semio_framework_value_original_vector` import, `RetainedWorkPoll` field/method misuse, missing `begin_close`). It is store4's claimed crate (claims 15:48, 17:38; LOCK NEEDED for the `original-vector` dep). The other named crates (value/receiving, job escrow, native-owned protocol, ui-contract accessibility) show only warnings in that log (2157 `unnecessary qualification`, unused imports); they were compiled green, not erroring. No guard edit; store4 owns the fix.

## Test-run blockers found and fixed (all EDITED, each UNVERIFIED until a green run; claims lines 20:1x to 23:1x)
1. `Missing Rust source input` (guard-102): 731 folded fixtures into paged/append/work/📐️.json and left stale include sites: shared-utf8/native tests and paged/receive tests now read `sharedUtf8.native` / `receiveLaw` of that file.
2. `Missing or ambiguous Rust module: payload_case` (guard-105, also in 10 older logs): job/🧪️tests/🔬️retained-ownership and job/♻️retirement/📄️payload/🧪️tests had one `../` too many in the `#[path]` to outcome/builder/testing.
3. value root crate lib-test closure over budget (guard-106), NOT fixed, NOT raised: 271,101 B without testkit, 273,711 B with, limit 262,144. Growth since HEAD is concentrated in paged/receive/map +4.5 KB, receive +1.8 KB, retained-clone/paged +1.1 KB, shared-utf8 +0.8 KB, test wire +0.7 KB. Owners must split; candidates are the in-crate white-box tests (append work tests 7.6 KB, shared-utf8 native 4.9 KB and admission 3.6 KB tests, numeric reservation 2.4 KB, JSON fixture 4.3 KB). Measure with `bun run 1105b-tools/guard/closure.ts <entry> [testkit]` (entry is the Cargo.toml `[lib] path`, usually under 📦️packages/🦀️rust).
4. snapshot-read test closure 33 files vs 32 (guard-107): inlined 📋️field-clone/🧪️laws as `cfg(test) mod laws` (old file stashed at 1105b-tools/guard/field-clone-laws.moved.rs), now 32 files, 246,456 B.
5. value-factory lib red (guard-108, 4 errors): OwnedFactory test impl in 🏭️factory/🧪️testing ported to the effect-based trait (`prepare_close_state_step`, `close_state_step`, work quotes). Past it in guard-110. The factory `🧪️tests/{boxed,owned,construction}` and arc-custody factory tests still use the old signature (170 errors in lib test).
6. job-context test closure 262,260 B vs 262,144 (guard-111/112): removed unused imports (rustc-flagged) in job-context root, continuation sources and clock-stride, now 262,117 B (27 B margin). joblaws then began deleting dead pub fns while callers remain (4 errors, guard-113); see claims note. joblaws also moves the capture seam out (-8.6 KB), which supersedes this margin.
7. job-worker was dropped from the test set because its dev-dependency job-conformance is red.

## Red crates, wasm lib (guard-101)
dictionary (equal.rs mounted from 🧾️dictionary/🟰️equal still on the grant API: bind/close_one/release_empty_page), job-conformance, job-headless, job-preview, value-failure-tests (unwrap on RetainedWorkPoll in testing helpers), value-native-schema (🔎️schema half-migrated), value-native-leaf (70 errors: NativeOwnedStorage take_box/reserve/initialize_with, NativeEffectCapturePlan missing). These are the open receivers/value port, not new regressions.

## Native tests compile red (guard-104)
23 crate/targets, headed by native-owned (13 integration targets plus lib test), native-decoding-tests, factory lib test, snapshot-projection, dynamic-clone-tests, original-vector, native-leaf, native-schema, job-recorded-clock and 13 more; all stale tests against the final work-accounting API. Full list in `🗑️generated/1105b/guard-104.txt` (grep "could not compile").

## Per-crate table
`ok` = no error reported in that run; crates behind a failed dependency are not attempted by cargo, so `ok` for tests-compile is an upper bound. lib native is from guard-104 (factory lib was red there, green in guard-110).

| crate (semio-framework-) | lib wasm | lib native | tests compile | tests pass |
|---|---|---|---|---|
| job | ok | ok | ok | UNVERIFIED |
| job-clock | ok | ok | ok | UNVERIFIED |
| job-conformance | red | ok | ok | UNVERIFIED |
| job-context | ok | ok | ok | UNVERIFIED |
| job-failure | ok | ok | ok | UNVERIFIED |
| job-fault-publication | ok | ok | ok | UNVERIFIED |
| job-headless | red | ok | ok | UNVERIFIED |
| job-native-decode | ok | ok | ok | UNVERIFIED |
| job-native-encoding | ok | ok | ok | UNVERIFIED |
| job-outcome | ok | ok | ok | UNVERIFIED |
| job-outcome-borrow | ok | ok | ok | UNVERIFIED |
| job-owned-text | ok | ok | ok | UNVERIFIED |
| job-park | ok | ok | ok | UNVERIFIED |
| job-payload | ok | ok | ok | UNVERIFIED |
| job-preview | red | ok | ok | UNVERIFIED |
| job-progress | ok | ok | ok | UNVERIFIED |
| job-publication | ok | ok | ok | UNVERIFIED |
| job-receiving | ok | ok | ok | UNVERIFIED |
| job-recorded-clock | ok | ok | red | UNVERIFIED |
| job-registry | ok | ok | ok | UNVERIFIED |
| job-replacement | ok | ok | ok | UNVERIFIED |
| job-return-book | ok | ok | ok | UNVERIFIED |
| job-return-budget | ok | ok | red | UNVERIFIED |
| job-return-escrow-publication | ok | ok | red | UNVERIFIED |
| job-scope | ok | ok | ok | UNVERIFIED |
| job-testing-caller | ok | ok | ok | UNVERIFIED |
| job-unit-tests | ok | ok | ok | UNVERIFIED |
| job-work-grant | ok | ok | ok | UNVERIFIED |
| job-worker | ok | ok | ok | UNVERIFIED |
| job-worker-authority | ok | ok | ok | UNVERIFIED |
| value | ok | ok | ok | UNVERIFIED |
| value-allocation-return | ok | ok | ok | UNVERIFIED |
| value-call | ok | ok | red | UNVERIFIED |
| value-clone-source | ok | ok | ok | UNVERIFIED |
| value-clone-source-testing | ok | ok | ok | UNVERIFIED |
| value-control-contract | ok | ok | ok | UNVERIFIED |
| value-control-physical | ok | ok | red | UNVERIFIED |
| value-derive | ok | ok | ok | UNVERIFIED |
| value-derive-expansion | ok | ok | ok | UNVERIFIED |
| value-derive-resources | ok | ok | ok | UNVERIFIED |
| value-dictionary | red | ok | ok | UNVERIFIED |
| value-dynamic-clone-tests | ok | ok | red | UNVERIFIED |
| value-edit | ok | ok | ok | UNVERIFIED |
| value-equality | ok | ok | ok | UNVERIFIED |
| value-factory | ok | red | red | UNVERIFIED |
| value-failure-tests | red | ok | red | UNVERIFIED |
| value-intrinsic | ok | ok | red | UNVERIFIED |
| value-native-decoding | ok | ok | ok | UNVERIFIED |
| value-native-decoding-tests | ok | ok | red | UNVERIFIED |
| value-native-default | ok | ok | red | UNVERIFIED |
| value-native-encoding | ok | ok | ok | UNVERIFIED |
| value-native-encoding-action-tests | ok | ok | red | UNVERIFIED |
| value-native-leaf | red | ok | red | UNVERIFIED |
| value-native-number | ok | ok | red | UNVERIFIED |
| value-native-outcome | ok | ok | ok | UNVERIFIED |
| value-native-owned | ok | ok | red | UNVERIFIED |
| value-native-owned-try-map | ok | ok | red | UNVERIFIED |
| value-native-receiver-protocol | ok | ok | ok | UNVERIFIED |
| value-native-receiving-contract | ok | ok | ok | UNVERIFIED |
| value-native-receiving-original | ok | ok | red | UNVERIFIED |
| value-native-reservation | ok | ok | red | UNVERIFIED |
| value-native-schema | red | ok | red | UNVERIFIED |
| value-native-schema-engine | ok | ok | ok | UNVERIFIED |
| value-native-shared-field | ok | ok | ok | UNVERIFIED |
| value-native-shared-slots | ok | ok | ok | UNVERIFIED |
| value-native-source | ok | ok | ok | UNVERIFIED |
| value-native-storage | ok | ok | ok | UNVERIFIED |
| value-native-tree-engine | ok | ok | red | UNVERIFIED |
| value-ordered | ok | ok | ok | UNVERIFIED |
| value-original-aliases | ok | ok | ok | UNVERIFIED |
| value-original-slot | ok | ok | ok | UNVERIFIED |
| value-original-vector | ok | ok | red | UNVERIFIED |
| value-paged-clone | ok | ok | ok | UNVERIFIED |
| value-paged-list | ok | ok | ok | UNVERIFIED |
| value-paged-text | ok | ok | ok | UNVERIFIED |
| value-persistent | ok | ok | ok | UNVERIFIED |
| value-read | ok | ok | ok | UNVERIFIED |
| value-receiving | ok | ok | ok | UNVERIFIED |
| value-registry | ok | ok | ok | UNVERIFIED |
| value-resident | ok | ok | ok | UNVERIFIED |
| value-retained-clone | ok | ok | ok | UNVERIFIED |
| value-retained-clone-integration | ok | ok | ok | UNVERIFIED |
| value-retained-map | ok | ok | ok | UNVERIFIED |
| value-retained-work | ok | ok | red | UNVERIFIED |
| value-retirement | ok | ok | ok | UNVERIFIED |
| value-retirement-contract | ok | ok | ok | UNVERIFIED |
| value-retirement-owner | ok | ok | ok | UNVERIFIED |
| value-retirement-queue | ok | ok | ok | UNVERIFIED |
| value-schema | ok | ok | ok | UNVERIFIED |
| value-shared-issuance | ok | ok | ok | UNVERIFIED |
| value-snapshot-projection | ok | ok | red | UNVERIFIED |
| value-snapshot-read | ok | ok | ok | UNVERIFIED |
| value-sort | ok | ok | ok | UNVERIFIED |
| value-source-turn | ok | ok | ok | UNVERIFIED |
| value-testing-admission | ok | ok | ok | UNVERIFIED |
| value-testing-allocator | ok | ok | ok | UNVERIFIED |
| value-testing-effect-bank | ok | ok | ok | UNVERIFIED |
| value-testing-presence-retirement | ok | ok | ok | UNVERIFIED |
| value-tests | ok | ok | ok | UNVERIFIED |
| value-tests-2 | ok | ok | ok | UNVERIFIED |

## Watch items
- packjson output-birth quote: `WRITER_BIRTH_COPY_BYTES` = size_of Option<String> (24) + 2*usize (16) + size_of JsonWriteFrame (40) + u8 = 81 B on 64-bit; quote (controlled-and-borrowed-bridge:129) and receipt (:175) use the same constant, so quote == receipt; 81 <= 128-9 = 119 reserve. It is the payload actually moved (writer field store + one frame + phase byte), honest for the per-turn copy grant. Not re-run by guard (pack-json is outside the value/job crate set).
- std `Mutex`: no non-test occurrence in value/job except `value/retirement/mutex` (the RetireOwned impl itself) and `job/context/encoding`, which uses stack-local turn Mutexes already at HEAD (3 occurrences before, same shape). No new retained-structure Mutex found.
- `AuthoringWallet`: new callers (modeling-job finish_with_progress, tessellate, mesh-io, image decode, fold, sqlite validation, pack-json parser) are synchronous driver functions, not inside a job `step`/turn. None inside job/context. Sqlite-snapshot validation:38 uses it within a `control` loop; judged driver code, flagged for io owner to confirm.
- testdebt-h removal: `retry_unstarted`, `validate_capture_plan`, `reclaim_input`, `failure_allocation_phase`, `take_failure_protocol_refusal` have zero references repo-wide under 🧰️framework. The 8 unnamed receiving recovery wrappers: no compile error mentions a missing wrapper in guard-101.
