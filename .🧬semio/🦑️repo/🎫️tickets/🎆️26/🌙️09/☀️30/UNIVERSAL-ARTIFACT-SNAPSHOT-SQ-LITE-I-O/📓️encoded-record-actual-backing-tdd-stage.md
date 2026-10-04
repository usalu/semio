# EncodedRecord Actual Backing TDD Stage

The read-only design report `📓️encoded-record-hashmap-allocation-authority-design.md` identified the canonical EncodedRecord::new guessed count-times-slot admission versus actual HashMap::try_reserve backing. This stage adds contract/tests only. Production EncodedRecord, RecordValue, controls, compiler aliases, and all provider behavior remain unchanged. A genuine Native assertion RED has not been run by this lane; Root is the sole Cargo owner.

## Closed Neutral Contract and Independent Source

Four changed/added paths beneath `🧰️framework/🔨️modules/🗣️dsl/🧬️schema`:

- `🧫️fixtures/💰️record-backing/🔣️.json`: authored cardinalities0/1/3/4/7/8/15/16/255/256/4095; distinct u16 fields beginning1; primitive integers beginning-2048. Requests are observed allocator layouts, admission must cover all backing, measured exact ceiling must accept, minus-one must preserve OwnershipLimit, zero permits only truly allocation-free backing, and retirement must not refund cumulative charges. Insertions equal the descriptor’s declared count. No bucket sizes, alignment constants, runtime ABI guesses or payload carrier are fixture authority.
- `🧫️fixtures/💰️record-backing/🧬️schema/🔣️.json`: handcrafted closed draft2020-12 schema including closed nested primitive/policy/descriptor contracts.
- `🧪️tests/🧱️ownership/🟦️.ts`: one registered Source law. Ajv rejects incomplete cardinalities, logical-count request authority, and unknown properties. A third-party SQLite recursive field relation independently validates every authored field identity and signed literal integer at all cardinalities. Buffer writes exact u16/i64 primitive words and DataView reads them independently. This Source law validates the neutral literal contract, not HashMap backing byte amounts.
- `🧪️tests/🫳️borrowed-object/🦀️.rs`: one actual Native law appended through the existing mounted unit module and sole DSL test GlobalAlloc.

Actual registered Source route `SEMIO_TEST_LEVEL=quick NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket-generated-unique-directory> bun nx run @semio-tech/framework-dsl-record-rs:test-ownership --skip-nx-cache` returned GREEN: 6/6 laws, 9491 assertions, 117 ms Bun / 2.1 s critical path / 2.2 s Nx, exit zero. Receipt `🗑️generated/encoded-record-backing-source-current.log`. This route runs the existing strict Source consumer typecheck then Bun tests and no Cargo. Prior five Source laws are preserved; exactly one Source law is added.

## Actual Allocator Request Witness

The Native law directly calls canonical native_encoding::EncodedRecord::new followed by exactly its declared number of unique primitive FieldValue::Int insertions. Fixture parsing, callback/control construction, literal verification, assertions, error formatting and retirement occur outside observation. No fake record producer, guessed byte multiplier, serialized native text, JSON carrier or alternate allocation API is measured.

The existing allocator continues measuring all alloc/alloc_zeroed Layout.size and realloc full new request size, never a size delta. A fixed stack-sized32-entry layout observation records actual size/alignment tuples with no observer heap allocation. RAII restores the previous per-thread observation state during success and unwind; nested requests merge into a previously enabled parent. The old borrowed-object law and its byte bound remain unchanged. Observed overflow is explicit and fails after observation is disabled. No second GlobalAlloc or new runtime dependency exists.

For each cardinality, the law observes the complete actual constructor-plus-insert request aggregate at an ample native allowance, validates all literal owned fields, and requires admitted owned bytes to cover that observed backing. Zero cardinality separately requires no backing request and zero payment. It replays the same real constructor under the measured exact aggregate allowance and requires exact settled ownership. For positive cardinalities, exact-minus-one and zero must refuse with OwnershipLimit and settle zero ownership. Refusal diagnostics may allocate their authored error text, so the witness checks that no layout matching the independently observed actual backing requests is requested during refusal; it does not falsely count diagnostic allocation as successful table backing. Two successive constructors retain both charges after retirement; the third must refuse without requesting the observed backing again.

The insertion frontier is exactly the authored descriptor count. Exceeding that declared count through arbitrary public insert calls remains a separate design concern; this law does not invent an overfill policy or claim it is already rejected. AllocationFailed/cancellation and cold full-publication retirement remain separate existing/future contracts, not weakened by this law.

Native selector `sqlite_snapshot_native_encoding_record_backing_matches_actual_allocator_requests` is mounted as a test only. Existing registered route `@semio-tech/framework-dsl-record-rs:test-native` accepts existing extra arguments. Readback finds12 old sqlite_snapshot_native prefix laws plus this one new candidate,13 total; exact actual Native counts and compiler/runtime receipt await Root. The same target can select the single law without creating a script/target. Parser-only rustfmt returned zero: `🗑️generated/encoded-record-actual-request-law-parser-current.log`; this is syntax proof only. No Native pass or feature RED is claimed.

No production repair, provider opt-in, feature mount, permanent script, runtime dependency, Git/worktree action or AGENTS edit was made. Existing launcher routes are reused.

## Exact Existing Native Selector Arguments

Readback of the registered DSL NativeScript forwards resolveTestLevel’s remaining arguments to runCargoTestsV1. Its existing partitionNextestExecutionFilters keeps --lib in build selection and the unique test-name substring/--no-fail-fast in assertion selection. Reuse Root’s existing explicit SEMIO_CARGO_TEST_POLICY; this target reads that required policy from the environment and does not invent a default.

```sh
SEMIO_TEST_LEVEL=quick NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<unique-ticket-generated-directory> bun nx run @semio-tech/framework-dsl-record-rs:test-native --skip-nx-cache --args='--lib sqlite_snapshot_native_encoding_record_backing_matches_actual_allocator_requests --no-fail-fast'
```

For all twelve prior prefix laws plus this one new candidate, use `--args='--lib sqlite_snapshot_native_ --no-fail-fast'`. Both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` already register this Native target and its Source test-ownership target. No selector invocation or Cargo run was performed by this lane; exact actual runtime selection/counts await Root.
