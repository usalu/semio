# P8yk Independent Framework Foundation Audit

Date: 2026-08-22  
Scope: Phase-8 fail-closed framework foundation only  
Verdict: **PASS foundation only. Phase 8 itself remains RED / REJECTED.**

## Scope and method

This was an independent source-first audit of the current working tree. I read the repository instructions, master Phase-8 plan, P8yh acceptance audit, and P8yj closeout, then inspected the current script and Rust/TypeScript paths rather than trusting either report. No Cargo command, native/Wasm build, browser test, ticket API operation, or modifying Git operation was run. The only files created are this report and the two verifier-generated P8yk ledger snapshots in this ticket directory.

## Foundation decision

The limited foundation passes because it is materially fail-closed:

- Owner-local proof rows are declared in the four owner files (Draw 1, Flow 2, Forms 2, Remodel 4), not in a central fixed-count table. `ArtifactBoundedFirstStepProof::new::<A>` derives a private `ToolOwnerWitness` from the Rust owner type; the runtime catalog loop checks owner, live controller, schema, factory, generated migrated command set, and duplicates before accepting a catalog. The static companion checks the claimed owner file and exact `Migrated` declaration. This is an exact owner/controller/file/factory/tool/schema bijection across the source and runtime boundaries, with no hard-coded nine-row success condition.
- `ArtifactApp` and `ArtifactEditor` defaults contribute empty proofs, no factories, and `None` builders. They are non-authoritative defaults, not blanket registration. `ActionSemantics::for_kind` and `bounded_catalog` remain unclassified; the static scan confirms neither assigns `Migrated` by default.
- The bounded reducer rows cannot activate by themselves. `with_registry_on_bus` validates the dynamic catalog, deliberately discards the proof vector, sets `bounded_tool_contracts` to `Vec::new()`, and registers only explicit app-owned factories. There is no call to `register_framework_reserved_factories`. The ledger's production registrations is therefore exactly zero. A declared handler proof reaches `ActionBus::admit_exact_wire`, which has no factory key and fails before decoding/handler execution. Thus the nine `Migrated` declarations are UI-addressable metadata but are not execution-reachable fake migrations; their click/intent path fails closed before the former handler work.
- `ActionBus::dispatch`, `dispatch_wire`, and `admit_exact_wire` use only `factory_by_key`; aliases are metadata and cannot select a factory. Registration keeps the factory type identity and schema with the key. The qualified proof additionally compares owner witness, controller/tool key, schema, contract, `TypeId`, and concrete factory type name.
- The source keeps the eight framework-reserved generic envelope cursor jobs and the configuration decode job, but no longer activates their factories. `run_framework_reserved_job` requires an `AppOwned` proof, so these routes fail before their generic jobs, legacy history/clipboard work, or config commit. This is honest failure closure, not a claim that their envelope cursors are sufficient jobs.
- The source still serializes import media before job creation; typed command preparation/ephemeral application/emit commit still occur outside the worker job; configuration decodes the whole raw buffer in one stage; and `Drop` calls whole-map `cancel_all`. The verifier reports all of these as failures rather than admitting them.
- Segmented output is source-side bounded: `ArtifactOutputChunks` has operation identity via `Arc::ptr_eq`, checked total credit, 4,096-byte nonempty chunks, explicit sealing, and FIFO `VecDeque::pop_front`; terminal download output rejects foreign/unsealed chunks. `plugin_take_segmented_download_chunk` takes one chunk by instance and operation. The browser `ShellHost`/`ShellHelpers` do not recognize `semio-segmented-handle-v1` or drain chunks, so this is explicitly a Rust seam only, not an end-to-end streaming claim.

## Reproduced checks

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=38 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean; one recorded, allowlisted test-only blocking bridge |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json --output ...` | Expected fail-closed exit: 8 failures, zero admissions |
| Two fresh JSON generations | Byte-identical |
| `git diff --check` over script/framework/four proof owners | PASS |

Fresh P8yk ledger facts:

| Item | Count |
| --- | ---: |
| Macro hosts / invocations | 50 / 50 |
| Macro rows / unique | 775 / 773 |
| Literal registrations | 656 |
| Live registered command rows | 884 |
| Admitted complete operations | 0 |
| Owner-local handler proofs | 9 |
| Production `ToolJobFactory` implementations | 11 |
| Bounded-first-step production activations | 0 |
| Typed dispatches / aliases | 3 / 4 |
| Framework reserved residual routes | 8 |
| Pending app-owned importers | 35 |
| Global-payload-store candidates | 34 |
| Negative self-tests | 38 |

The two P8yk files are 310,953 bytes with SHA-256 `ba8812f7db90637cb67e582e5b67465f7ccf100043106aca363bee44f4c8a722`. They are byte-identical to each other. They differ from P8yj only in two current Layout factory line numbers (both moved eight lines during concurrent work); all counts and all failure rows match P8yj. The P8yj snapshots remain mutually byte-identical at their reported 310,953 bytes and SHA-256 `d154d9b75394b827e778d92be7aa2c5e3c66d98397f8f46b1e027a03b5c86a0d`.

## Adversarial review and limits

- Copied controller/schema/tool strings do not confer runtime authority because the proof and factory comparison include the owner `TypeId`/type name. The source includes the copied-owner rejection regression.
- An alias cannot bypass exact dispatch: the public dispatch paths never consult `aliases`.
- A fabricated `Migrated` declaration without the owner-local row is rejected by catalog construction/static ledger; a row without an activated exact factory is rejected at ActionBus admission. The current nine rows take the latter path intentionally.
- A direct old handler bypass is absent from `dispatch_typed_command_inner`; it performs no `A::handle(&command)`, `run_to_completion`, or `run_on_worker_async` call. This does not make its surrounding prepare/job/commit sequence bounded, which is why it remains unadmitted.
- The script is a textual verifier, not a Rust parser/typechecker. Its 38 fixtures substantially exercise expected textual evasions, but it cannot establish borrow/Send correctness, macro expansion equivalence, or runtime timing. The source inspection above is therefore the basis for the limited foundation pass; compilation and runtime gates remain required before any activation.

## Phase-8 blockers and acceptance gates

Phase 8 remains red because the machine ledger correctly retains these hard failures:

1. Replace all pre-job typed preparation and post-job ephemeral/emit commit work with a bounded persistent operation state machine.
2. Replace the eight framework-reserved envelope-only cursor routes and monolithic config decode with concrete route-owned jobs; only then register them.
3. Remove whole-media pre-serialization from import submission and transfer operation-owned media authority to concrete importer jobs.
4. Replace whole-map synchronous `Drop::cancel_all` with O(1) scope/generation cancellation plus bounded async cleanup and saturation/close tests.
5. Complete the host/browser segmented-handle recognition and bounded drain sink.
6. Resolve all 34 global payload stores and classify/migrate or explicitly fail-close all 884 live command rows.
7. Before activation or Phase-8 acceptance, run native Rust tests/build, Wasm build, browser segmented drain, watchdog timing, cancellation/supersession/close-under-load, and concurrency/stale-result gates. These were intentionally not run because Cargo is prohibited under the stated disk-pressure constraint.

No Phase-8 completion, runnable migration, or end-to-end segmented download is claimed by this result.
