# FEM Native Window Admission

The ninth 2D native run exercised the actual retained commands, exact window stores, document replacement, and restored window Pack/SPR history. The ownership runtime law passed: two instances of each window kind stayed isolated, current values survived replacement and reopen, and application/document bytes remained unchanged by window preferences. The fixture explicitly uses an 8 MiB worker stack; this is not evidence of normal-stack execution.

The two codec laws failed against the existing language-neutral invalid corpus. Both native camera decoders accepted an unknown nested field; the results configuration also accepted the invalid mode `harmonic`. The independent Ajv schema rejects these cases. The correction will make the shared FEM result-display vocabulary finite and enforce exact camera record admission in native code.

Evidence: generated log `fem2d-window-config-native-9.log`, one passed and two failed, runtime 0.08 seconds. The goal remains active; this checkpoint only covers the FEM window slice.

## Strict Admission Correction

The shared FEM app-surface schema now defines `ResultMode` once, with Rust, TypeScript, JSON Schema, GraphQL, and Protocol Buffers facets. Both concrete results-window configurations consume that vocabulary. Native command handlers reject an unknown mode before emitting a window mutation, and render projections exhaustively map the finite mode. Both native camera records reject unknown fields.

The obsolete free-form result parser had no production callers and silently mapped unknown modes to static. It and its two fallback-specific unit tests are removed; native and independent schema tests now exercise the finite result-window contract. Protocol Buffers window packages use valid identifiers and import the shared result-mode schema from the repository root. Protocol Buffers and GraphQL compiler execution has not been performed.

The tenth 2D native run passed all three selected laws, including invalid JSON corpus rejection and the actual retained-command/replacement/reopen runtime law (0.17 seconds test runtime). Both registered 2D/3D neutral targets passed against Ajv and fast-json-patch, including strict TypeScript compilation. The first 3D native run is underway. Evidence: `fem2d-window-config-native-10.log`, `fem-window-config-neutral-10.log`.

## 3D Runtime Boundary

The first 3D native run stopped during compilation on a missing trait import in an existing preparation test. After correction, native2 compiled and passed both codec laws, now covering static, modal and buckling values from the neutral fixture. Its exact-window runtime law overflowed the explicit 8 MiB test thread during the first camera dispatch.

Both runtime fixtures now heap-pin their full async body, nested case, dispatch and settlement futures. A diagnostic separates command admission from settlement and measures the dispatch future. The next runs deliberately use 2 MiB fixture threads; they have not yet run and no normal-stack success is claimed. No production stack limit was raised. Forms Try currently holds this fleet's native slot.

The macOS crash report for native2 resolves the failing frames to FEM3D `live_visual::maintenance_step` and its thread-local registry closure, called through `mounted_job_maintenance_step` and the public plugin maintenance dispatcher. This narrows the production investigation beyond fixture allocation. The extracted frames are retained temporarily in `🗑️generated/fem3d-native2-crash-frames.json`; the original report is `semio_s_artifact_fem_3d-303b1ac4cf73da08-2026-09-12-143052.ips`.

Native3 passed both codecs, admitted the camera command and measured its future at 4,448 bytes, then reproduced the maintenance overflow on 2 MiB. Disassembly confirms `recover_abandoned_one` reserves `0xa3d * 4096 + 0x1c0` stack bytes before reading the empty recovery slot. Its `MountedState` carried the full numerical child and visual candidate inline, so merely moving that state through recovery inflated the frame above 10 MiB.

The mounted state now owns these two independent retained jobs through unique boxed owners. Recovery and terminal-placeholder transfers move their owners while each job retains its existing step and close protocol. This production correction is awaiting native runtime and recovery-law verification; full rendered-job allocation/retirement checks are still required.

Native4 confirms that the large-stack recovery fault is resolved: both codecs and all three retained commands run on 2 MiB, document replacement and exact reopened values are verified. Final close then stalled because the FEM mounted-job close callback always returned a pending retirement scan even when its own terminal witness was empty. It now reports `Complete` on that witness. Pending snapshot cleanup is also qualified by the exact app-instance identity, so a modulo-slot neighbor is neither cleared nor mistaken for this app’s pending work. The new close correction still requires a native rerun; the overall run is not passing.

The compiled native4 recovery function reserves three 4 KiB pages plus `0xbf0` bytes (15,344 bytes, excluding its small saved-register prologue), compared with 10,736,064 bytes previously. This confirms the carrier-ownership correction at the compiled function boundary. Scoped whitespace checking for the close changes passes in `fem-close-diff-check.log`.

The expanded all-mode 2D/3D neutral runs and strict TypeScript compile pass together in `fem-window-config-neutral-11.log`. Scoped diff whitespace checking found trailing whitespace and surplus end-of-file blank lines in the touched fixtures. Those are corrected; `fem-scoped-diff-check-2.log` passes. The shared test helper `assert_undo_redo_round_trip` now qualifies its member bounds through its parent module, correcting a compile error exposed by the Forms build; this is a separate test-helper name-resolution correction.

The close protocol now has three language-neutral cases: no pending work, its own pending snapshot, and a different app instance colliding in the same modulo slot. Ajv validates the corpus and fast-json-patch produces its expected remaining-owner state; the registered neutral target and strict TypeScript pass in `fem3d-window-config-neutral-close.log`. The native counterpart executes the actual domain close callback and compares against the same corpus; it is included in the next native window run.

## Normal-Stack 3D Verification

Native5 passes all five selected laws on explicit 2 MiB threads: two strict window codecs, the three-case exact-instance close callback, actual retained window-command isolation/replacement/reopen/close, and the existing queued/running/state-drop owner-recovery law. The registered Nx target completed successfully; evidence is `🗑️generated/fem3d-window-config-native-5.log`. The runtime reports one addressed window page for camera and result-display commands, zero window pages for document replacement, and preserved application/document bytes and exact restored window Pack/SPR. This establishes the corrected close and boxed retained-owner paths for these selected cases. It does not establish the full visual rendering/allocation suite. FEM2D native11 is now running with the expanded all-mode corpus and heap-pinned 2 MiB fixture.

FEM2D native11 also passes all three selected laws on the 2 MiB fixture, including the static/modal/buckling corpus and actual retained command, document replacement, reopening and final close paths. Its runtime was 0.06 seconds and the Nx target exited successfully. Evidence: `🗑️generated/fem2d-window-config-native-11.log`. Both dimensions now have normal-stack ownership evidence for their selected window laws.

The temporary per-dispatch start, future-size, admission and settlement diagnostics are now removed from both fixtures. Their final runtime confirmation remains. The registered 3D target now includes all existing `live_visual::tests::` laws, extending recovery validation to numerical/visual allocation and retirement after the retained owner representation changed; this expanded run is pending. No new executable command or launch entry is required because the existing target owns these checks.


## Populated Document And Numerical Construction

FEM3D native11 passes all five selected window/document laws on 2 MiB, including populated artifact/snapshot admission and refusal of foreign camera, locale and result mode. The shared variant field-casing correction is covered separately by 16 passing Serde-comparison laws. The subsequent complete visual module passes five initial laws, then aborts on stack overflow in the production solid/reaction/modal numerical-child fixture. The combined Nx target exits nonzero; no full visual success is claimed.

The native11 crash report identifies `core::array::from_fn<Option<Fem3dMeshedSolid>, 32>` through `FixedSlots::new` and `Fem3dNumericalChild::new`. The fixed-capacity array is initialized using a generator even though every entry is the same absent value. It now uses inline constant absent initialization, preserving its capacity, ownership and admission protocol without the generator's nested aggregate copies. This source correction still needs a 2 MiB native rerun. Evidence: `🗑️generated/fem3d-window-config-native-11.log` and extracted `fem3d-native11-crash-frames.json`.


Native12 passes the five window/document laws again, then runs eleven visual laws. Eight visual laws report success before the process aborts. The first numerical fixed-slot constructor now completes; the crash moves to `MountedPlanarDomain::new` through another `array::from_fn`, while constructing the shared mesh domain inside `step_model`. The actual owner is `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs`. Its polygon constructor is now constant and the fixed hole array uses inline constant initialization. The current native11 constructor disassembly and fixture disassembly are diagnostic artifacts only; no compiled size reduction is inferred for the new mesh change before a rerun.

Native12 also reproduces a separate close-grant failure: the stale candidate's zero-byte close returns one released item. Inspection confirms that the token-abort path can subsequently release a full page without checking the byte grant, and that region/element backing arrays were taken without their byte-size guard. The candidate now checks a full page grant before token abort progression and each backing owner's exact byte size before taking it. Existing lease close already requires a full page grant. Terminal-empty candidates still complete without needing a grant.

A new six-case language-neutral corpus covers region and element backing with zero, one-byte and full-page grants. Ajv validates the corpus and fast-json-patch independently produces the expected retained-owner states; the registered neutral target and strict TypeScript pass in `🗑️generated/fem3d-close-grants-neutral.log`. The native counterpart checks exact retained pointers, released item/byte counts, grant bounds, and final backing-credit emptiness against the same corpus. It is authored but not yet executed. The existing stale-token zero-grant failure remains the original runtime regression for token cleanup. Both new corrections await native13; no complete visual success is claimed.

Additional files for this continuation: shared mesh root above; FEM3D session root and its unit fixture; common window-contract `🟦️.ts`, `🧫️fixtures/💳️visual-close-grants/🔣️.json` and `🧬️schema/💳️visual-close-grants/🔣️.json`. Both editor publication docstrings now describe the actual host-effect and exact-window lanes; the unused FEM2D DisplayMode import is removed.


Native13 completes without either construction stack overflow on the same 2 MiB setting. All five window/document laws pass, then 11 of 12 visual laws pass. The six neutral backing-grant cases pass natively with exact pointer and credit assertions; the existing stale-token cancellation/close law now also passes. The remaining numerical solid/reaction/modal case returns `fem3d.numerical-child-fault`. That is a domain failure, so the combined target remains non-green. Shared mesh failure currently discards its supplied diagnostic message; temporary `[DEBUG]` output in that helper and a stage-qualified test failure are authored to identify the actual next failure in native14. These temporary diagnostics must be removed after the fault is resolved. Evidence: `🗑️generated/fem3d-window-config-native-13.log`.


## Completed-Face Edge Authority

Native14 passes the five window/document laws and 11 of 12 visual laws on 2 MiB. Temporary diagnostics isolate the remaining failure to `mesh-fixed-edge-authority-backing` at `SolidMesh/reserve-edge-authorities`; no stack overflow occurs. Evidence: `🗑️generated/fem3d-window-config-native-14.log`.

The shared mesh job reserved three independent edge vectors from the caller's maximum triangle allowance. Two vectors were unused outside allocation and cleanup; the actual constraint-adjacency index was reserved for intermediate-face worst-case size before triangulation completed. The FEM3D allowance of 42 triangles therefore refused a small valid mesh before any edge index was built. The correction removes the unused owners and admits the actual adjacency index after triangulation finishing, bounded by three slots per completed face. It preserves the 16 KiB backing cap for mounted work and performs overflow/byte admission before allocation. The same allocation now serves the unbounded job path, which previously reached indexing without reserving its actual authority vector.

Three neutral triangulation cases cover both square diagonals and one face under the large caller allowance. Ajv validates them and Graphlib independently computes edge and adjacency counts. The registered neutral target and strict TypeScript pass (`🗑️generated/fem-edge-authority-neutral.log`). The native counterpart checks completed-face capacity, shared-edge adjacency, full-capacity refusal without pointer/length/capacity change, bounded item disposal, exact backing-byte admission and final remaining-owner close. It replaces a prior test of an unused edge vector. Native execution remains queued; no mesh or full numerical success is claimed.

Files: shared mesh `🦀️.rs` and `🧪️tests/🔬️unit/🦀️.rs`; new mesh `🧪️tests/🧫️fixtures/🕸️edge-authority/🔣️.json`, `🧪️tests/🧬️schema/🕸️edge-authority/🔣️.json`, `🧪️tests/🕸️edge-authority/🟦️.ts`; common FEM window contract; root and ticket validation `📜️script.ts`. Existing registered FEM3D native route now includes the focused mesh ownership law. No new executable target is introduced.


## Numerical Child Outcome Ownership

Source review of the five delegated numerical stages found preview, checkpoint and commit outcomes matched with `_`, discarding the shared retained payload carriers. `RetainedJobPayload::Drop` explicitly requires bounded retirement and otherwise preserves backing; discarding these child results is therefore an ownership defect even though the earlier mesh failure prevented that path from executing.

FEM3D now retains one shared `StepOutcome` at the numerical-child boundary, including both commit state and output, and drains it through the existing shared one-page close protocol before advancing any child or downstream numerical stage. The same owner is retired first on cancellation/fault close. No FEM-specific payload format or replacement disposal protocol is added. Zero-fuel, expired-deadline and cancellation opportunities retain the exact outcome; bounded close preserves pages below their byte grant. The fault-only carrier is removed because the shared outcome already owns all payload variants.

Four neutral cases cover preview, checkpoint, dual-payload commit and fault. Independent Ajv/JSON Patch plus strict TypeScript pass via the existing registered FEM3D route (`🗑️generated/fem-child-outcomes-neutral.log`). The native law checks all four cases, exact retained carrier identity, zero/insufficient byte grants, one-page release, zero-fuel/deadline/cancel preservation, and final numerical-owner close. Source parses with rustfmt. Native execution remains queued with FEM15; these are authored corrections, not a full numerical pass.

Files: FEM3D session root and unit fixture, common window contract, new `🧫️fixtures/🧒️child-outcomes/🔣️.json` and `🧬️schema/🧒️child-outcomes/🔣️.json` under that common contract.


## Native15 Runtime Checkpoint

Native15 exits nonzero after the five window/document laws pass on 2 MiB. The newly registered mesh filter selected zero tests: FEM3D reexports this engine from FEM2D. The route is corrected to run that filter in `semio-s-artifact-fem-2d`; no mesh-native success is attributed to the zero-test command.

The visual module runs 13 tests: 11 pass, two fail. The numerical child gets beyond meshing and all serialized mesh outcomes into `PrepareAssembly`, where the current construction reports `FemError::Singular`. The existing diagnostic message labels that as a mechanism, but source inspection shows this same error also represents backing admission failure; the mathematical cause is not established. The mesh temporary failure log is removed because that boundary is resolved.

The new outcome law used a 20-byte payload while expecting a full-page byte grant. Shared `RetainedJobPayload::close_step` currently reports and admits logical payload length, so the 16KiB-minus-one grant released it with 20 reported bytes. The fixture now uses a full page for each stream, matching the tested page grant and the numerical runtime's existing full-page opportunities. This correction does not establish physical backing accounting for short payloads; that shared distinction remains an audit item. No shared job close contract is changed by this continuation.

The shared assembly constructor also deserves a separate physical-owner correction: `AssemblyPartitionBuffer` stores contiguous full/free triplet vectors, and its BuildPartitions phase reserves a dense maximum per logical partition through a 4 KiB single-owner gate. FEM passes one logical partition, so several valid tetrahedra can exceed the physical gate before assembly begins. Logical partition count should not substitute for physical backing pages. The current `reserve_exact_owner_page` allocates before checking the cap, potentially retaining an oversized rejected vector. These are source findings; the exact native15 phase is not yet instrumented.

Evidence: `🗑️generated/fem3d-window-config-native-15.log`. FEM16 must run the correct engine package, the full-page outcome law, and the still-failing numerical path before this work can be considered complete.


Further inspection identifies an earlier concrete assembly refusal: none of Bar3, Frame3 or Tet4 override the required mounted node-count/ID/cell methods. Their inherited None is converted to Singular in ValidateElementReferences, before factorization. The physical partition backing issue above is a subsequent source finding, not the proven native15 failure stage. Exact implementation handoff and proposed neutral storage reuse are in `fem-numerical-owner-follow-up.md`.
