# Real Puzzle Actor Job Acceptance Design

## Actual Actor Publication, Distinct From Browser WASM

The published catalog sources row currently names pluginId puzzle, packageId semio:puzzle, cratePath `🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust`, packageName semio-hub-puzzle-component, wasmOut semio_hub_puzzle_component.wasm and deployment directory Puzzle. The component manifest declares the semio:puzzle package and cdylib. Its only Rust entry calls the actual plugin_exports macro with INTERACTIVE_COMPONENT and the real Hub plugin and app fleet. The component build driver uses wasm32-wasip2. This is separate from the browser-module BoardSession producer.

The existing root ComponentsScript supports `components iterate puzzle` for a genuine prototype Original and delegates to buildComponentClosureV1. That driver describes actual component bytes, publishes the model with its Cargo provenance and then materializes the selected browser facet. `components describe puzzle --profile prototype` is also declared but can write committed descriptors; iterate is the appropriate existing prototype prerequisite. Neither command has been launched by this agent.

The canonical reader is `plugin/model/reading::withComponentModelV1(workspace, invocation, intent, control, body)`. Intent is the existing closed shape version1, actual Original mode, pluginId puzzle and catalog cratePath. It owns the products publication read lease, validates the exact four-file roster, component WASM header, complete semantic descriptor pair, receipt identity/profile/bytes/SHA256/descriptor binding and artifact recheck. It supplies component, descriptor, descriptorPack and receipt paths. The test owner must use this callback and keep the lease across the child test, rather than reconstructing a cache path or searching Cargo target directories.

## Proposed Exact Test Owner And Ingress

Add normal `[[test]] guest-io` to the existing semio-hub-puzzle Cargo owner, source `🧪️tests/🧩️guest-io/🦀️.rs`. This domain acceptance owner already owns the actual three Puzzle artifact dependencies; test-only public Host/OwnedRuntime dependencies reuse the actual component loading APIs and avoid introducing a Puzzle runtime dependency in the domain-neutral framework. Its owner command should be `@semio-tech/puzzle-plugin:test-guest-io`, whose BundleScript enters withComponentModelV1, calls runRepositoryCargoTests selecting --test guest-io, supplies only validated model paths to the child environment, and rechecks the publication before leaving the reader. Missing fixture or unproduced model must fail, never skip.

Proposed test-only environment names are SEMIO_TEST_ACTOR_COMPONENT, SEMIO_TEST_ACTOR_DESCRIPTOR and SEMIO_TEST_ACTOR_RECEIPT. These are outputs supplied by the canonical owner callback, not caller-selected private cache locations. The Rust law independently validates expected package/plugin/world identities against the neutral input before compiling the actual bytes through GuestRuntimes and its real OwnedRuntime. Wasmtime may supply an independent backend case once the same lawful receipt/funding path is qualified; it is not a mock replacement.

## Public Job Protocol And Material Contract

Current GuestRuntime methods are Original APIs: start_job, step_job, read_job_output, acknowledge_job_output, cancel_job and close_jobs. A Running result is an OriginalJobOutcome descriptor; actual progress bytes require reading pages and acknowledging that exact output identity. Cancel acknowledgement only means admission; subsequent steps must separately observe source_closed. Completion requires output-page collection, acknowledgement, job closure and actor context/instance paid closure. A simplistic loop over the older JobStep shape would omit these controls.

A neutral JSON fixture will name real package/plugin/app/source/target coordinates, a deterministic large board recipe, independent expected node identifiers/positions/counts, the actual mounted policy selection, bounded turn/cleanup profile and cancellation after actual conversion progress. JSON is a suitable first advertised format with the already available independent Serde oracle; it avoids adding another runtime dependency and proves complete material fields. The completed output must be independently parsed and compared against the generated semantic recipe, not merely checked for a nonempty header. A second real job must emit conversion progress, then receive funded cancel, publish no final result, fully close its source/output ownership, and permit a subsequent real editor command and undo on the same actor.

The actual INTERACTIVE_COMPONENT grants are reused unchanged (32 items,32768 copy,16777216 capacity,1048576 release,4096 depth), rather than enlarging authority until a large export fits. Job execution funding must come from an actual OriginalJobFixtureCaller/borrowed original context or the existing explicitly issued headless JobFunds interface; no fabricated RetainedTurnInput struct or renewed loans. Existing real Host factory law shows the public begin/advance/take_original_actor_step and close_original_instance_step pattern with physical receipts. Its neutral counter bytes are not accepted as this fixture.

## Required Evidence Before Production Refactor

The test must distinguish decoded request progress from conversion progress. The current two-phase semio.io-run wrapper emits a decoded-hop Running preview before calling the synchronous conversion path; that preview alone is insufficient. The actual guest run should expose whether conversion yields/cancels or fails the unchanged budget. Only the resulting measured frontier justifies a producer/format job refactor. Native captured OwnedSerializer factories remain a separate researched interface; no broad new factory implementation is required to stage this real actor test.

No compiler or producer was launched while Root request76254 remains live. Test staging still needs exact public funding/receipt driver adaptation and declared owner integration; this document is a proposal, not an executed acceptance result.

## Explicit Operation Source Authorization Design

The first staged driver incorrectly used the mounted preparation TURN grant as the entire OriginalJobFixtureCaller lifetime stock. The actual fixture caller consumes its stock cumulatively. Birth traced the genuine production GuestRelayDriverPolicy.operation as a separate caller-supplied operation Source, with admission/drive/wake grants separate; there is no shipped interactive total default. Therefore a 32 KiB lifetime interpretation is a test category error, not a demonstrated product export limit.

Use an explicit finite ORIGINAL FIXTURE operation authorization, declared once in the neutral case before admission, rather than copying an MCP or standalone headless profile. Keep every per-turn preparation/maintenance/close grant equal to unchanged INTERACTIVE_COMPONENT (32 items /32 KiB copy /16 MiB capacity /1 MiB release /4096 depth). Keep the existing operation work authorization of 2000 user-visible fuel quanta, a real monotonic 120-second absolute deadline, and at most 10000 protocol turns. All loans debit actual spending; none renews the operation stock.

The proposed material envelope is 1024 nodes, a maximum 2 MiB serialized request, 2 MiB result, 4 MiB native value working allowance (4 KiB per authored node), a 16 MiB actual VM memory ceiling, and 2 MiB admission/command scaffolding allowance. The source/request/output bounds will be asserted by the fixture and actual output; VM memory uses the real instance accessor. These are acceptance authorizations and failure ceilings, not a claim that current implementation allocation costs have been measured or proven beneath them.

The two jobs (completed baseline and cancellation candidate) receive one common lifetime Source:

- items: 2000 ×32 =64000;
- copied bytes: VM ceiling +2 jobs ×(2 request +2 output +2 native working) =16 +2×(4+4+8) MiB =48 MiB (50331648);
- capacity bytes: 2 VM ceilings +2 jobs ×(2 request +3 output +2 native working) +scaffolding =32 +2×(4+6+8)+2 MiB =70 MiB (73400320);
- release bytes: the same finite 70 MiB capacity authorization;
- depth: unchanged4096.

This separates whole-operation permission from turn admission. It does not raise any production cap or issue new credit after refusal. If an actual material, VM, receipt, work or deadline bound is exceeded, preserve the first actual frontier and investigate it. Oracle/reference parsing buffers are test analysis and are not represented as product guest spending. The protocol records actual returned guest retained/logical effects and separately measured backend instructions with identity/deadline checks.

The neutral schema should close these exact case values and reject attempts to shrink the material, substitute decode-only progress, remove source-closure assertions or add a headless copy override. The first-party validator and existing third-party Ajv already agreed on the initial case and four such negative mutations; this is schema evidence only, not native actor success. Root authorized the design and Low reviewed it; the explicit operation Source is now implemented once at OriginalJobFixtureCaller admission, with unchanged mounted per-turn loans and executable formula/material/VM assertions. Sufficiency and runtime behavior remain unproven.

## Current Staged State And Verification

The normal Hub guest-io integration target and uncached test-guest-io owner are registered. The owner builds the genuine model facet, retains withComponentModelV1 through the exact Cargo test and rechecks it. Birth owns the script-local nested-control/preparation correction and the remaining transitive TypeScript Source responsibility cut. Its full entry Source currently refuses1844 inputs /16851413 bytes,74197 above the unchanged16 MiB bound; the small Rust Source result cannot substitute for that gate.

The Rust driver uses a real monotonic clock and one finite48/70 MiB operation authorization, validates and posts actual original job receipts, reads/acks bounded output pages, separately observes source_closed and uses shared Actor::retire for all typed test owners. The complete1024-node material oracle uses independent Serde prediction and parsing. Cancellation now requires actual execute-phase positive/incomplete progress and a strict PACK-decoded typed Fault with canceled/framework/error/nonretryable before accepting source closure; arbitrary Failed outcomes cannot pass. Source-only Low review is green, last Rust closure36242 bytes /5 inputs /321 lines with identical physical probes.

Root supplied real actor context and instance lifecycle acknowledgement plus paged createAlternative and independent before/after ReadHistory checks on the same actor. Jobs close before app/context closure, then the actual VM instance and original caller retire. Raw backend Fuel/Deadline faults fail honestly; no guessed CPU spend or unreceipted renewal/resume is accepted. There is no fresh actor execution result.

Canonical foreign carrier bindings now use Text for JSON/TXT/SVG/DXF and Binary for DWG/PDF/PNG/ZIP, reusing owned UTF8 backing. Neutral route inventory declares carriers; Hub runs13export assertions and7exact pairs; the physical full-field/word JSON consumers require Text. Independent recipe size prediction gives1201173 foreign pretty JSON bytes,1377054 text-envelope bytes versus5072386 binary-number-envelope bytes; semantic carrier correction avoids needless transport expansion without relaxing2 MiB bounds. The actual produced material size remains a native assertion.

Root request76254 ended Cargo101 before tests at unrelated surface begin_close bool/unit mismatch. Root corrected the narrow semicolon; no new native launch follows until fleet dependency maintenance completes. This agent enters Source quiet after this report update. No actor/native/producer success is claimed.
