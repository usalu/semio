STATUS: PARTIAL. Read-only audit lane aud-open. All 50 inputs (`1030-review-*.md` x9, `1030-result-*.md` x41) read in full. Current-code verification was done only for the items marked CONFIRMED below; the rest are UNVERIFIED (claimed fixed or open in the result file, source not re-checked, nothing built or run). No cargo run, no source edits.

Legend: RESOLVED-IN-CODE = grep confirms the symbol the result file says it added. CLAIMED-FIXED = result file says fixed plus tested, code not re-checked, not compiled. OPEN = result file says open or not done. UNVERIFIED = nothing decisive found.

Note on method: broad greps over the whole tree timed out, so checks were run on the framework and plugin subtrees only. A combined grep of 16 claim symbols returned hits for `abandon_trapped` (shard event, H1), `has_blank_cell` (try-build-definition, i18n), `validate_live` (token API, policy). No hits in the narrowed scope for `reentrantPassCeilingMilliseconds`, `hostTreeWaitsV1`, `admit_record_rows`, `OUTPUT_PAGE_BYTES`, `stock-exhausted`, `maxOutputLength`, `OriginalTextSpliceAdvance`, `forcedUnmountDissenters`, `startCacheMilliseconds`. Those are UNVERIFIED because the search scope may have missed them (the brief-listed trees were not all searched), not because they are proven absent.

## 1. Review findings (1030-review-*.md)

### framework (1030-review-framework.md)
| Item | Code path | Verdict |
|---|---|---|
| F1 funding claim stranded / phase clobbered (medium) | `🏃️process/.../💰️funding/🔁️replacement/🟦️.ts` | CLAIMED-FIXED (fwfix: busy guard, catch never overwrites phase; issuer.test.ts x2) |
| F2 NativeReceive derive silently changes `default="path"` (medium) | `🌱️value/✨️derive/🛬️receive/🦀️.rs` | CLAIMED-FIXED (fwfix: path default never optional, test `a_path_default_never_substitutes...`) |
| F3 const-eval hash degenerates on common suffix (low-med) | `📡️replication/.../🪪️leaf-descriptor-validation/🦀️.rs` | CLAIMED-FIXED (fwfix/mutderive: head+stride+tail; test `identity_hash_separates...`). Doc in mutderive result still says "last 48 bytes": doc drift, UNVERIFIED |
| F4 aborting Box::new in original-construction closures (low) | typing edit + text-splice native | CLAIMED-FIXED (fwfix: `std::alloc::alloc` with AllocationFailed; allocator-exhaustion tests) |
| F5 text-splice advance cannot report unfunded (low) | `🖱️ui/.../✂️text-splice/🧬️native/🦀️.rs` | CLAIMED-FIXED (fwfix: `OriginalTextSpliceAdvance{Unfunded,Progress,Ready}`); symbol not found in narrowed grep, UNVERIFIED |
| F6 accepted terminal lease release not idempotent (low) | `🏃️process/🔒️leases/🟦️.ts` | CLAIMED-FIXED (fwfix: closed same lease marks released; test `terminal-release`) |
| F7 fabricated close progress (low) | record-body-cursor, document driver | DECIDED-NOT-FABRICATED (fwfix: one item per funded transition, documented; starved-grant test). Accepted by design |
| F8 leftover `[DEBUG]` probe | `🫴️receiving/🧪️tests/🦀️.rs` `debug_size_probe` | CLAIMED-FIXED (fwfix, docfix, stdiofix say removed); not re-grepped (narrowed search missed pack path). UNVERIFIED |

### host (1030-review-host.md)
| Item | Code path | Verdict |
|---|---|---|
| H1 trap on event: no fault frame, retry slot wedged (medium) | `🖥️host/🧵️shard/🎬️shard-loop-frame-turns-and-job-control/🦀️.rs` | CONFIRMED in part: `abandon_trapped` present in `🖥️host/🧵️shard/🎟️grant/📨️event/🦀️.rs:23`. hostfix says fault frame sent, retry slot freed, test `a_trapped_guest_turn_emits_one_fault_frame...`. Fix path present; test not run |
| H2 ColdReplayProjection clones base, charges header only (medium) | `🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs` | CLAIMED-FIXED (hostfix: `ColdReplayProjection::bounded`, metered; test). Symbol not in narrowed grep, UNVERIFIED |
| H3 ColdReplayEdit half-applied, re-pollable (medium) | same file | CLAIMED-FIXED (hostfix: `failed` flag, inverse after apply). No regression test (stated by hostfix) |
| H4 BackboneDocument.transitions panicking Drop (medium) | `🖥️host/🔌️plugin-host/🗃️backbone-document/` | CLAIMED-FIXED (hostfix: transitions plain Vec, `BackboneDocument::retire`). Note: vcs.* ledgers handled by retire(); other consumers "need .retire() calls" (hostfix own note), OPEN partially |
| H5 producer raw pointer after unlock (low-med) | `🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️producer/` | CLAIMED-FIXED (hostfix: no raw pointer, `held` field; lending tests). Also review-today #4 (lender drop leak) fixed: `return_held()` in Drop |
| H6 task-resume wire output silently discarded (low) | `⚛️reactor/🔄️turn/📬️task-resumes-and-wire-effects/` | CLAIMED-FIXED (hostfix: `plugin.resume-output-undecodable` fault) |
| H7 non-live instance resume requeued forever (low) | same | CLAIMED-FIXED (hostfix: shell fault; only lifecycle-busy requeues) |
| H8 GIS inference unbounded budget (low) | `🌎️hub/🧩️compositions/🌍️gis/💡️inference/🦀️.rs` | PARTLY-REFUTED by hostfix: cancellation works through checkpoint; fuel now budget.work_units; deadline still u64::MAX. Deadline part OPEN. Note: `u64::MAX` pair not re-grepped |
| H9 member_stores partial drop (low) | same | OPEN (hostfix: not changed; design item for bundle) |

### infra (1030-review-infra.md)
| Item | Verdict |
|---|---|
| 1 wait-marker ps fork per poll (high) | CLAIMED-FIXED (actinfra: start time cached, marker written once per episode, child table read once per heartbeat; `startCacheMilliseconds`, not found in narrowed grep, UNVERIFIED) |
| 2 marker non-atomic, one bad file aborts read (medium) | CLAIMED-FIXED (actinfra: temp+rename, per-file tolerance) |
| 3 wait markers never swept (medium) | CLAIMED-FIXED (actinfra: `cleanup()` removes `<id>.wait.*`; `.wait.` grep not confirmed, UNVERIFIED) |
| 4 forced-unmount journal drift (high) | CLAIMED-FIXED (actinfra: `forcedUnmount` in journal, audit jsonl, stopGraceMilliseconds removed) |
| 5 grow tools synchronous without timeout (medium) | CLAIMED-FIXED (actinfra: timeouts + abort, lsof exit other than 0/1 refuses). Async spawn partly not done (timeouts bound block) |
| 6 forced-unmount allowlist name-only (security, low-med) | CLAIMED-FIXED (actinfra: full executable paths + `codesign -R="anchor apple"`); `anchor apple` not confirmed in grep, UNVERIFIED |
| 7 6h holder ceiling deadlock / 1h writer preference starvation | CLAIMED-CHANGED (actinfra: ceiling removed; writer preference unbounded for native-fixed-image). Trade-off: a holder tree now always passes a waiting writer. Starvation of the writer by a continuously-issuing holder tree is NOT bounded. OPEN as design (see review-today #7) |
| 8 headroom learns whole-image free-space fall (medium-low) | CLAIMED-FIXED (actinfra: counts only when no other command in ledger; failed command teaches only growth) |
| 9 ramp-idle counts unobserved grant as idle (low) | CLAIMED-FIXED (actinfra: needs numeric observation; oracle corpus) |
| 10 cross-platform gaps (lending off on win32, busybox ps) | CLAIMED-FIXED (actinfra: /proc, ps, CIM listing; TZ-independent start identity) |
| 11 Cargo.lock missing-path marker English-only | CLAIMED-FIXED (actinfra: `(os error 2|3)`, locale independent; `cargoLockMissingPathManifestV1`, not found in narrowed grep, UNVERIFIED) |
| 12 quiet-wait wall clock vs mtime | CLAIMED-FIXED (actinfra: future mtime means no quiet wait) |
| 13 singleton publication remove-then-link not crash-atomic | CLAIMED-FIXED (scopereact: tombstone swap, post-condition, crash injection test) |
| 14 artifact scratch lease accumulation | VERIFIED-NO-LEAK (actinfra: retirement sweep covers it). Closed |
| 15 wasm-build lease ordering unverified | VERIFIED (actinfra: pkgDir per invocation; live act13 built wasm through it) |
| 16 receipt capacity breaks budget resolution | CLAIMED-FIXED (actinfra: invalid receipt ignored for that partition; test) |

### repo (1030-review-repo.md)
| Item | Verdict |
|---|---|
| 1 Cargo.lock lacks build-graph crate and edges (high) | CLAIMED-FIXED (deps round 3: `repo cargo update --workspace --offline` rc 0). Not re-grepped in Cargo.lock. `--locked` metadata verification still OPEN (deps result) |
| 2 source-owner resolution semantics changed, schema not enforced | CLAIMED-FIXED (deps: `requireJavascriptDependencySourceOwnerHoistedProtocolV1`; digest already binds the three fields). Profile/version left at 1 by choice |
| 3 headroom learns only lane growth | CLAIMED-FIXED (deps: `commandWriteBytesV1` = max(lane growth, image free-space fall, 0)) |
| 4 elastic vs pinned receipt disagree | CLAIMED-FIXED (deps: one parser `readImageReceiptBytesV1`; elastic clamped to maximum) |
| 5 alone rule stalls behind stuck lease | CLAIMED-FIXED (deps: `strandedGraceMilliseconds` 120000) |
| 6 warm/cold temperature decided before admission | CLAIMED-FIXED (deps: lane choice after acquisition re-records class) |
| 7 two workspace walks, two skip rules | CLAIMED-FIXED (deps: `OutputDirectoryPattern` in schema, shared). Review sub-point on pkg/bindings: non-bug (gitignored anywhere) |
| 8 recycleReasonV1 unchecked baseline | CLAIMED-FIXED (deps: non-finite baseline falls back to ratio rule) |

### plugins (1030-review-plugins.md)
| Item | Code path | Verdict |
|---|---|---|
| 1 process3d mounted-registry calls deleted API (HIGH, compile break) | `✏️s/.../🧪️tests/🔬️mounted-registry/🦀️.rs:265` | CLAIMED-FIXED (policy: dead call removed). `process3d_release_publication_authority` not found in narrowed grep, UNVERIFIED |
| 2 energy engine projection drops rows silently (medium) | `...snapshot/🧳️state/👁️read/🦀️.rs` | CLAIMED-FIXED (energy: `ProjectionFault` latch, reduce fails typed). `from_value(...).ok()` grep not confirmed, UNVERIFIED |
| 3 energy native-cold apply half-applied, full copies (medium) | `...native-cold/🦀️.rs` | PARTLY-FIXED (energy: base swapped after success, `to_reference` once). Still 3 projections per edit; per-edit single projection OPEN (energy result) |
| 4 cold one-item factory drops per-family size guard (medium-low) | `store/📬️paged-one-item/❄️cold-apply` | CLAIMED-FIXED (policy: `ColdPreparationBounds` + bounded factory, Playground/Playbook use it; oversize refused at preflight). No law pinning the 1 MiB case stated, UNVERIFIED |
| 5 deleted publication leases lose checks (medium-low) | gen2d/gen3d/process3d | PARTLY-OPEN. Restored: `authoritative_publication_token_fails_closed_on_operation_generation_and_base` (gen2d, gen3d, process3d); real-path rejection law added (UNRUN). Domain credit laws and authority-missing: not restored (policy, by design) |
| 6 hub gis: run_controlled first-turn, catalog, flatten | `🌎️hub/.../🌍️gis/💡️inference/🦀️.rs` | OPEN (only fuel budget changed per hostfix H8; catalog and map_err not addressed) |
| 7 minor: stale docs in generation3d shells; whitespace in fem2d tests; shell.navigate non-optional | bundle manifest | OPEN (shell.navigate `optional: false` kept deliberately by bundle result; review suggests optional: true for that one capability) |

### stdio (1030-review-stdio.md)
| Item | Verdict |
|---|---|
| 1 HIGH sqlite binary ignores limits, encode unpriced | CLAIMED-FIXED (stdiofix: `from_record_binary` takes limits, `admit_record_rows` before decode, encode under writer control). `admit_record_rows` not found in narrowed grep, UNVERIFIED |
| 2 MED kind laundering drops cancel/limit kinds | CLAIMED-FIXED (stdiofix: `into_value_error` in macro; test) |
| 3 MED Document driver reports item progress unfunded | CLAIMED-FIXED (stdiofix: progress = `maximum_items.min(1)`); `maximum_items.min(1)` not grepped, UNVERIFIED |
| 4 export step zero-work turn / livelock | CLAIMED-FIXED (gen: `exported == 0 || !cx.should_yield()`; regression test not run). `exported == 0` not grepped, UNVERIFIED. catch_unwind over StepContext: OPEN (stepctx note, not addressed) |
| 5 decode oracle accepts what Rust rejects (varint, trailing bytes) | CLAIMED-FIXED (stdiofix: TS/Py refuse >64-bit varint; uncontrolled decode refuses trailing bytes). Oracle memory bound across segments: OPEN (test-only, low) |
| 6 leftover `[DEBUG]` probe; 542 files lost allow(long_running_const_eval) | `[DEBUG]` CLAIMED-FIXED. allow removal: mutderive reports 0 hits outside tickets. Not re-grepped, UNVERIFIED. Other-crate const-eval failures for epw, zip, step, mp3, mp4, bcf, jpg, pptx, wav, tiff, bmp, fem/hub crates: OPEN (not re-checked) |
| 6b record_binary ignores caller PackEncodeOptions (omit_nulls) | OPEN (no result file addresses it) |

### tests (1030-review-tests.md)
| Item | Verdict |
|---|---|
| A1 pack-record copyBytes 4096 -> 65536 (weakening) | RESOLVED-PER-RESULT (fem round 4/5: Document driver boxed, fixture back to 4096, grant law). Fixture value not re-grepped, UNVERIFIED |
| A2 leftover debug_size_probe test | CLAIMED-FIXED (fwfix, fem, docfix). UNVERIFIED |
| A3 gen2d/gen3d lost end-to-end token rejection law | PARTLY-RESOLVED: real-path rejection law added to gen2d and gen3d (policy addendum 2), UNRUN. Compile not confirmed |
| A4 fem2d inference/bounds unit tests deleted | RESOLVED-PER-RESULT (fem2d: ported to `roundtrip-tests.rs`, bounds hand-built extent). Not run (fem2d-14 blocked) |
| A5 fem2d DSL/pack agreement tests deleted | RESOLVED-PER-RESULT (fem2d: roundtrip-tests.rs, DSL/pack agreement). Not run |
| A6 shared snapshot lease law reversal | INTENDED (stdiofix: framework alias Clone + explicit retire; test `last_snapshot_alias_retires_the_payload_through_funded_turns`). Accepted |
| A7 typing-law fixture entries -> net | JUSTIFIED by schema. Accepted |
| A8 fault position 0.97 -> 0.93 | OPEN (deps caveat: confirm 0.93 is after final manifest replay in the case trace; not checked) |
| A9 bun key formula expectation rewritten | JUSTIFIED (deps, greenfield). Accepted |
| B1 record-document laws not wired into any target | RESOLVED-PER-RESULT (stdiofix: nx target `verify-record-document`, OwnedVerifyScript branch). Not run through nx |
| B2 fem2d test compile not green | OPEN (fem2d: not compiled green) |
| B3 pubtoken gen2d/gen3d/raster unverified | OPEN (policy: gen2d/gen3d/process3d/raster/playground/playbook never reached rustc for pubtoken-touched crates) |

### today (1030-review-today.md)
| Item | Verdict |
|---|---|
| 1 SealedShared<T>: Clone unfunded alias panics on drop (HIGH) | RESOLVED-PER-RESULT (fem round 8: infallible alias clone, Drop only debug_assert, release leaks instead of panic; laws `sealed_shared_*`). Silent sites in energy and fem2d not audited by grep (fem: "compiler plus full native check is the oracle"). Energy re-run OPEN. `impl Clone for SealedShared` not re-grepped, UNVERIFIED |
| 2 wait-marker lending rule (tree rule) | CLAIMED-FIXED (actinfra patch: `hostTreeWaitsV1`, JSON markers with process start). Not found in narrowed grep, UNVERIFIED |
| 3 fuel accounting zero-fuel still funds work | CLAIMED-FIXED (stepctx: `unfunded`/`settle`/`fuel_budget` no max(1); law test). `fn unfunded` not in narrowed grep (stepctx path is geometry, outside scope), UNVERIFIED |
| 4 hostfix lender drop leak / begin_close lost | CLAIMED-FIXED (hostfix: `Drop` calls `return_held()`; `close_step` re-issues begin_close; test) |
| 5 set-field leaf Drop requires closing | PREPARED-NOT-CONFIRMED. mutderive wrote `set-field-drop-safe.py --apply` to run after freeze; result does not say it ran. UNVERIFIED (grep for `closing` in set-field file did not run) |
| 6 cache grow resume trusts stale oldDevice / skips drain | CLAIMED-FIXED (actinfra: resume re-reads mount device and re-runs holder check from `drained`; test) |
| 7 holder-reentrant shared admission past writers unbounded | CHANGED, NOT BOUNDED. See infra 7. OPEN |
| 8 derive completeness count; docs stale | Behaviour sound. Doc drift OPEN (mutderive text says 48-byte tail; code is head 16 + stride + tail 32, per review) |
| 9 i18n: `?? ""` silent missing placeholder | OPEN. i18nfw removed the `?? "en"` fallbacks but did not address `demonstratorFormat` `params[name] ?? ""` (not grepped, UNVERIFIED) |
| 10 energy RetainedRowPatchApply boxing | No defect found. Accepted |

### ts (1030-review-ts.md)
| Item | Code path | Verdict |
|---|---|---|
| 1 document actor stock cumulative, never replenished (medium-high) | `...🎭️actor/🎟️retained-turn/🟦️.ts` | CLAIMED-FIXED as typed fault (tsfix: `original-turn.stock-exhausted`, re-reserve child with fresh source). Per-turn refill not possible (guest conserves grant). Not grepped, UNVERIFIED |
| 2 non-receipt poll failure wedges source, opaque error | same | CLAIMED-FIXED (tsfix: source held, first failure rethrown, cause chain). `holdNativeRefusal` present in `...browser-bundle/.../🌉️component-bridge-source/🟦️.ts:186` (confirmed by grep) |
| 3 job output stall, 64 bytes per crossing | guest `call/🦀️.rs` | MIXED. jobpage: removed guest `.min(64)`, added `OUTPUT_PAGE_BYTES` (schema). tsfix: says guest 64-byte cap remains, not changed. CONFLICT between the two results. Narrowed grep for `min(64)` and `OUTPUT_PAGE_BYTES` returned no hit in the call module. OPEN until one of them is checked |
| 4 job crossings funded outside actor admission | `...component-bridge-source` | PARTLY-FIXED (tsfix: shard bridge funds from actor turn; document child keeps fresh self-issued funds per request, by design). Open for document child |
| 5 spawned jobs never cancelled, sequential | poll-lanes | CLAIMED-FIXED (tsfix: cancelJob, concurrent jobs, abort, progress). Jobs still run inside reconcileUiPatches (OPEN) |
| 6 record codec oracle unbounded inflate, __proto__, bigints | `...🎒️record-document/🟦️.ts` | CLAIMED-FIXED (tsfix: maxOutputLength, count bound, null-prototype maps, narrow()). `maxOutputLength` not found in narrowed grep, UNVERIFIED |
| 7 stale unwrapOption docs, event-count asymmetry, untracked actor-module | various | OPEN (tsfix: untouched). Not grepped |

## 2. Open or unfinished items stated in result files (not covered by a review)

| Lane (result file) | Item | Verdict |
|---|---|---|
| actinfra | Grow of rust image: executed, 48 -> 96 GiB (`repo cache grow` completed 03:14) | RESOLVED-PER-RESULT |
| actinfra | Holder-reentrant writer starvation / convoy | OPEN (see infra 7) |
| actinfra | act12/act13 build failures: puzzle wasm rebuilt every pass; os-plugin-describe-rs source admission churn; brep component registry reader refusal (Fault 5) | Fault 5 CLAIMED-FIXED (registryRefusalV1 names reason). Describe-rs churn: CLAIMED-FIXED (quiet-wait in churn protocol, applies to new processes). puzzle wasm uncacheable: OPEN |
| actinfra | Lock-order inversions (a) config view vs capture, (b) artifact-capture vs compiler-writer | CLAIMED-FIXED (patched in clone, applied; order tests 76/76). Live proof pending |
| actinfra | Freeze and patches applied to real tree | RESOLVED-PER-RESULT (11 patches applied after act12 exit) |
| bundle | Parity test not compiled/run; frontier errors in procedural-3d-io-geometry, cad-modes, procedural-2d-editor-transient, hub-gis | OPEN (blocked by foreign crates) |
| cleanup | Type-check blocked by job crate source admission (later resolved per pjob); 10 check batches not run | OPEN, not re-run |
| deps | `repo cargo ... metadata --locked` verification | OPEN (queued) |
| deps | Budget test 3 pre-existing failures ("Original parent Script process invocation required") | OPEN (environment) |
| e2eprep | Targets test, canonical-architecture, test-runtime-catalog: rc 0 | RESOLVED-PER-RESULT (final section 9). Not re-run |
| e2eprep | React singleton at install level | RESOLVED by scopereact (see below) |
| energy | Hub energy wasm green (chk29); snapshot tests; mounted bestest test; membership laws (AMBER) | PARTIAL. Mounted test NOT RUN; membership laws unrun after heap witness fix. OPEN |
| energy | Projection per edit still 3x; diff-carried inverse not done | OPEN (design) |
| fem | 3d snapshot 17/17 green; 3d wasm, mounted statik pane test, fem-laws, hub compile | PARTIAL. Round 14 engine GREEN; payload-turns registered; 3d/2d model + snapshot-tests + laws + chain3 blocked by foreign os-store crates. OPEN |
| fem2d | Editor chain, mounted tests, undo/set-active-example tests, native check | OPEN (blocked by ◻️2d / 🌱️value) |
| fwfix | Wasm checks green for booleans, regions, pixels, surface, editor, flow-drawing (partly) | RESOLVED-PER-RESULT (fwfix-w19 green). flow-drawing/node-graph/kernel: OPEN (pending value-retirement compile) |
| gen | chk16 red: os-store-child, os-store-ledger, neural budgeted, framework-3d; hub gen components | Later ports GREEN (fwfix/pasync). Hub gen wasm: OPEN. gen2d/gen3d tests unrun |
| hostfix | Batch b10/b11 tests (trapped_guest, backbone_transitions_seat, lending, begin_close) | OPEN (blocked upstream at last note; earlier b4 fixes applied) |
| hubplug | puzzle3d editor and gismap companion tests not compiled | OPEN (blocked by 2d-booleans, later resolved per fwfix; not re-run) |
| i18n | `ArtifactPresentation.name` and `MediaPortSpec.label` sweep to LocalizedLabel | OPEN (deferred, needs ◻️2d migration) |
| i18n | Demonstrator tsc errors in brand.ts from stale generated manifest | OPEN (regen needs value-retirement compile) |
| i18n | Brand text German-only (lock); ui vitest not run | OPEN (design; picker/introduction vitest not run) |
| i18nfw | IntroductionInteraction.label localized; picker in ShellHost | CLAIMED-FIXED. Picker vitest not run |
| ifcdata | Geometry tests 23/23 green (t5) | RESOLVED-PER-RESULT |
| ifcdata | 4x3 other tables (validation/program 5.7 MB, metadata, templates, mvd) not packed; ifc-4 `details` split not started | OPEN. stdio-bim gate still refused |
| jobpage | Output page 16384 bytes via schema; rust/TS tests green | RESOLVED-PER-RESULT (jobpage GREEN). Conflicts with tsfix note (see ts 3) |
| mutderive | Rust roster-law epw/gltf/energy test targets | OPEN (stale unrelated test code) |
| mutderive | Hub wasm batch (2d-booleans, wfc2d/3d/bitmap transients) | OPEN |
| mutderive | Home ApplyDirectoryPage preparation not implemented | OPEN (design) |
| oracle | Energy schema hash cross-language law; mutation roster from Rust consumption | OPEN (cargo run not done) |
| p2dflow | flow-drawing lib tests, wasm check | OPEN (blocked by 2d-booleans BooleanJob; now presumed fixed per fwfix-w19, not re-run) |
| p2dpix | pixels lib tests, surface paint, raster io compile | OPEN (blocked by value-retirement) |
| pasync | actor native tests (5 failing stale), tool-run lib tests | PARTLY-RESOLVED (pasync wave 6: tool-run 5/5, presentation 6/6, entities 5/5 PASS; actor native re-run queued) |
| perf | Flow editor compile, perf chk, two new laws | PARTLY. Compile fixes applied (flow editor); perf-chk1-flow red at last note. OPEN. FEM/energy perf items OPEN (audit rows 3-7) |
| pio | io lib tests green (31) | RESOLVED-PER-RESULT |
| pjob | Downstream callers on old worker API (plugin, compute-pool, renderer, reactor); job-worker tests not ported | OPEN (multiple lanes listed; partly ported later per pjob follow-up) |
| playbook | Hub playbook component and procedural ext blocked by 2d-booleans | OPEN (last note, not re-run) |
| policy | pubtoken checks gen2d, gen3d, process3d, raster, playground, playbook not reached rustc; NativeReceiveSlot law blocked (source-extent of value-native-owned) | OPEN |
| policy | Taxonomy gate report-mode, 77008 findings repo-wide | OPEN (not a gate for this change set) |
| ppack | `cargo test -p semio-framework-pack-json --lib` refused (source extent) | OPEN (use companion crate; companion 109/109 GREEN) |
| prepl | retained tests green; replication-history companion green (47) | RESOLVED-PER-RESULT. Replication-history lib re-check after move pending |
| psnap | snapshot-read wasm GREEN; lib tests blocked by semio-framework-async | PARTIAL. OPEN (lib tests) |
| pui | ui-locale, ui-raster-owner, ordered-map tests green (17) | RESOLVED-PER-RESULT (pui: maptest 17 passed; axis-value test not run) |
| resid | hub-gis inference port (21 errors), cad-editor deps, sweep x4 | PARTIAL. hub-gis compile not confirmed (OPEN) |
| rtpanes | 8 pane smoke tests written, none run | OPEN. Blockers: source admission (fixed by coordinator), gismap companion stale, cad-editor lib, puzzle3d fixture, value source extent. Note: a process `1101-tools/rtpanes/seq.sh` is running from another lane; not touched |
| runtime | Fresh-editor law GREEN; cancel law green; editor-action-producer / task-resume settle WorkLimit (fuel) | CLAIMED-FIXED (settle fuel fix, cancellation issuer birth). Mounted tests not run after fix. Presence law compile pending. DBG1030 debug code removed (hostfix confirmed) |
| scopereact | Install-level React singleton | RESOLVED-PER-RESULT (1 copy of react for all four packages; demonstrator test rc 0, canonical rc 0) |
| scopereact | Installer tests: 17-21 pre-existing failures (anchors moved, invocation-bound) | OPEN (not caused by singleton change) |
| smoke | Puzzle3d activation blocked (describe-rs exit 1, real fault); browser proof | OPEN. Describe-rs real error not extracted |
| stdiofix | Sqlite-law failures (recipient wrapping applied to 312 sites); "SQL close exceeds authored turn ceiling" laws | PARTLY-RESOLVED. Ceiling laws OPEN (cause not established) |
| stepctx | Test targets and brep boolean fixture law not run; zero-fuel law | OPEN (blocked by framework-2d and pixels). Review fix applied (unfunded/settle) |
| testsplit | window-config carve (bulk + original); pack-json companion; value-native-owned | PARTLY-RESOLVED. Offline gate 0 over. Native checks partially run (w6). value-native-owned round 2 OPEN; stale runtime-lane tests (tests-05-2/-05-3/-19/-02/-01) OPEN |
| tsfix | GREEN for codec, job drive, funding, source hold; not run in browser/wasm | RESOLVED-PER-RESULT (TS suites) / OPEN (browser) |
| tshost | WIT lowering oracle green; vitest staging 72/75 (3 environmental) | RESOLVED-PER-RESULT (3 environmental failures remain) |

## 3. Counts
- Review findings listed: 9 files, ~70 items (sections 1 above).
- Items confirmed in source by grep: 4 (`abandon_trapped`, `has_blank_cell`, `validate_live`, `holdNativeRefusal`).
- Items with an explicit OPEN verdict: about 25.
- Everything else: CLAIMED-FIXED or RESOLVED-PER-RESULT, not re-verified in code.

## 4. Recommended next checks (not done)
1. Re-run the claim symbols over the whole tree in a single `grep -rn` with an exclude list, using a time budget, and mark the UNVERIFIED rows.
2. Resolve the conflict on guest output page size (jobpage vs tsfix) by reading `OS/.../call/🦀️.rs`.
3. Confirm whether `set-field-drop-safe.py --apply` ran.
4. Confirm the holder-reentrant writer starvation bound (review-today 7) is accepted as a design choice, or add a bound.

## 5. Addendum: completed claim-symbol search (framework, plugin, hub, demonstrator trees)

Search output: `/private/tmp/claude-501/-Users-ueli-Documents-semio/87a69f25-5b5f-44e5-9b4a-748a23581f8f/tasks/b076psvos.output`. Symbol-level results change these rows:

| Row | New verdict | Evidence |
|---|---|---|
| host H1 / ts-host shard trap | CONFIRMED present | `void_trapped_checkout` in `🎭️actor/🎟️retained-turn/🦀️.rs:104`; `abandon_trapped` in `🖥️host/🧵️shard/🎟️grant/📨️event/🦀️.rs:23` |
| infra 7 / review-today 2 (wait-marker tree rule) | CONFIRMED present | `hostTreeWaitsV1` in `🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts:159` and used at `:348` |
| stdio 1 (sqlite limits) | CONFIRMED present | `admit_record_rows` in `✉️base/.../🛬️native/🦀️.rs:140`, called from the macro at `✉️base/🦀️.rs:173` |
| ts 1 (stock exhaustion) | CONFIRMED present | `original-turn.stock-exhausted` test at `🎟️retained-turn/🧪️tests/💼️job-funds/🟦️.ts:140`; `#stock()` in `🎟️retained-turn/🟦️.ts:116` |
| jobpage / ts 3 (output page) | CONFLICT STILL OPEN | `OUTPUT_PAGE_BYTES` and `for_output_page` present (`📥️source/🧪️tests/🦀️.rs:35`). `.min(64)` still present at `🎭️actor/🎟️retained-turn/💼️job/📥️source/🦀️.rs:72` (`grant.maximum_copy_bytes.min(64)`), so the 64-byte cap is not removed in this path. The guest reader in `🌉️call/🦀️.rs` was not in the output, so its state remains unknown |
| infra 6 (grow allowlist) | CONFIRMED present | `"anchor apple"` / `/usr/bin/codesign` in grow schema `🧬️schema/🔣️.json:48` |
| infra 11 (missing-path marker) | NOT CONFIRMED | only `os error (28|112)` found, in unit-cache schema. The `(2|3)` marker for cargo-workspace lock not seen |
| stepctx (zero fuel / unfunded) | CONFIRMED present | `pub fn unfunded` in `generation3d/.../⚙️compute/🦀️.rs:60` |
| review-today 1 (SealedShared Clone) | IMPLEMENTATION PRESENT, behaviour not read | `impl Clone`/`Debug` for `SealedShared` present in `🔗️arc-custody/🔒️sealed/🦀️.rs` and `🔗️issuance-clone/🦀️.rs`; drop contract not read |
| review-today 8 (hash doc) | DOC DRIFT CONFIRMED | `leaf-descriptor-validation/🦀️.rs:135` docstring; "48 bytes" claim not re-read in full |
| review-today 9 (demonstrator `?? ""`) | NOT FOUND | pattern `params[name] ?? ""` not in output, so it may already be fixed or moved. Left UNVERIFIED |
| plugins 1 (process3d deleted API) | NOT FOUND | no `process3d_release_publication_authority` hits: consistent with removal |
| plugins 2 (energy `.ok()` rows) | NOT FOUND | no hit for `from_value(row.to_value()).ok()`: consistent with the latch fix |
| review-stdio 6 `long_running_const_eval` | NOT FOUND | no hits in framework/ws/hub code paths searched |
| infra 14/15, review-host H9 | unchanged | not in search |
| `RetainedJobPayloadWriter` (pjob / energy / ported callers) | STILL REFERENCED | `semio_framework_job_payload::payload_writer_and_grant::RetainedJobPayloadWriter` in `🧵️job/⏱️context/📦️owner/🧪️tests/🦀️.rs:6` and `action-bus` test. The job-payload writer migration is not finished in these test files |
| `shell.navigate` optionality (review-plugins 7) | UNCHANGED | only manifest/test references found; no `optional: true` change seen |
| `has_blank_cell` (i18n / policy addendum 7) | RESOLVED | defined `🖱️ui/🌐️locale/🏷️label/🦀️.rs:135`, used at `try-build-definition/🦀️.rs:324` |
| `validate_live` (policy token API) | RESOLVED | defined `🌱️initialization/♻️retirement/🦀️.rs:53`, used by `playbook`/`gen` declarations at `📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs:265` and `🪟️surface/.../🦀️.rs:255` |
| `dashboard_build_graph` dep (repo 1) | PRESENT in manifests | `Cargo.toml` of daemon and registry reference `dashboard_build_graph`. Cargo.lock entry not re-read |
| `requireJavascriptDependencySourceOwnerHoistedProtocolV1` (repo 2) | RESOLVED | defined in `📥️read-context/🟦️.ts:36`, tested in `🔭️providers/🟦️.ts:77` |
| `holdNativeRefusal` (ts 2) | RESOLVED | used in `component-bridge-source/🟦️.ts:186` |
| `[DEBUG]` tags | FOUND ONLY IN GENERATED CAPTURE LOGS | `🤖️generated/process/captures/*` (not code). No code hit in the searched source |
| `unwrapOption` docs (ts 7) | STILL OPEN | `segmented-download/🟦️.ts:46` and a `*-reserved-response-settlement` test doc still mention it; `bridge.js:66` still defines `unwrapOption` in an extension module (not a stale doc). Cosmetic item still open |
| `OriginalTextSpliceAdvance` (review-framework F5) | NOT FOUND | only the `owner.advance` tests appear; the enum name is not in the searched tree. Left UNVERIFIED |
| `strandedGraceMilliseconds` (repo 5) | PRESENT in tests | headroom test files use the stuck-row case (`admittedMs`); schema constant not explicitly matched |

Counts after addendum: confirmed present in source: 14 rows (was 4). Still open or conflicting: output page size (ts 3), `unwrapOption` docs (ts 7), infra 11 marker, demonstrator `?? ""` (unverified), `RetainedJobPayloadWriter` test references (open).
