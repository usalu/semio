# Extension Resource Retirement

The neutral ExtensionBundle now owns an explicit ExtensionResourceOwner and a retained manifest retirement cursor. Plain capabilities use function pointers, so their handler shells have constant destruction cost. Owned capabilities dispatch through the explicit resource owner; no generic captured Fn payload is claimed to be bounded. Closing seals invocation before resource retirement, preserves cancellation, validates item/byte receipts and terminal emptiness, releases the terminal owner shell on a later grant, then drains each handler name and every declarative manifest field through retained first-party cursors without serialization.

Bundle payload fields are shielded by ManuallyDrop. Live Drop refuses destruction while preserving resource and metadata payloads; only terminal-empty bundles dispose their empty shells. Cold callers explicitly use dispose_cold, into_manifest_cold or registry extension_dispose_cold; real interactive turns continue to use bounded close_step. The neutral fixture specifies the live/terminal owner Drop census, and its law catches an attempted in-place destructor, confirms the same metadata and invocable owner remain, then explicitly drains that retained bundle. Identity inspectors, native descriptor tests and installed test callers now dispose their exact owners explicitly.

The runtime admits one retained replacement; refusal keeps the exact candidate with the caller. Activation waits for the old owner to drain. Actor SuspendRequest begins retirement and real poll turns spend at most one item plus the existing byte grant on this owner. Reserved extension.close.cancel and extension.close.resume capabilities use the actual Request/Respond seam while normal invocation remains sealed. Both macro variants retain an inadmissible candidate and retry admission at subsequent runtime entry.

The actual S Flow BREP guest has one SessionCapture inside BrepExtensionResources. The same owner implements evaluation, tessellation and their cancellation capabilities, plus begin/step/pause/resume/terminal close. No Session is held in a handler closure. An application fixture exercises a real box, a partially retained tessellation and explicit session retirement through the final allocated Arc shells. The real WASM event bridge preserves granted/narrowed/revoked capability changes and quota changes as their corresponding kernel events; the actual polling law verifies all four changes preserve an active owner before suspension.

## Exact Created Manifest

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension/🚪️retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧩️extension-retirement/🔣️.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧩️extension-retirement/📐️schema.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️extension-retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️extension-retirement/🟦️.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧫️fixtures/🚪️retirement/🔣️.json
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧫️fixtures/🚪️retirement/📐️schema.json
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️extension-guest-standalone/🟦️.ts
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🧩️2026-09-30-extension-resource-retirement.md

## Exact Updated Manifest

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/📥️inbound-request/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️extension-guest-standalone/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📋️project.json
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🧪️tests/🔬️unit/🦀️.rs

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-extension-bundle-dependency/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/🧪️tests/🔬️extension-guest-standalone/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🧪️tests/🔬️extension-guest-standalone/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/🧪️tests/🔬️extension-guest-standalone/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🧪️tests/🔬️extension-guest-standalone/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🧪️tests/🔬️extension-guest-standalone/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🧪️tests/🔬️extension-guest-standalone/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🧪️tests/🔬️extension-guest-standalone/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/🧪️tests/🔬️extension-guest-standalone/🦀️.rs

## Exact Removed Manifest

None. Temporary compiler/test output is tracked separately below and removed when this worker finishes; no input, report, source, AGENTS file or Git state is removed.

## Executed Validation

- Red schema-first native fixture tests observed the initially absent production owner/registry APIs. The first red snapshot also exposed an incorrect local module include path; that worker mistake was fixed before further validation.
- Neutral portable owned Nx canonical-architecture --oracle-only passed: eight AJV and independent Buffer/TextEncoder UTF-8 assertions. serde_json in the native fixture law independently validates the same neutral JSON and the first-party DSL round trip; retained resource receipts equal its UTF-8 byte length.
- Neutral focused owned Nx test extension_bundle_resource_retirement passed four laws. First run: Nextest 026ba3e2-17c9-442e-b1a8-d7364ca9cd6c. Final grant/cancellation run: Nextest eaf461bd-8c52-4ed5-a69e-eff9e06123ca, four passed, 869 unrelated laws filtered out. This includes zero grants, paused cursor, exact old/replacement retention, rejected candidate preservation, false terminal/exceeded receipts, later terminal-shell disposal and actual SuspendRequest/Request/Respond/poll shutdown.
- Full neutral owned Nx canonical-architecture target passed all nine exact laws, preserving the original five and adding the four retirement laws. Exact harness receipt: exact-cargo-laws-ImB8ez/00, native assertions=9, binary SHA-256 896189cb4370dcd327d443ac8c07fbb898b0b72866dd086a54ed77aa17b1d6b9. Build, exact inventory and all nine native invocations completed successfully.
- Actual BREP portable oracle passed six AJV/application request assertions during its owned canonical invocation.
- Actual BREP owned canonical native build initially stopped at concurrent backend test migration errors: private test_serial, two removed with_kernel_read calls and BrepDeconstruct(SessionCapture). The backend owner fixed those actual callers; parent settled the exact-lock/action relocation diagnostics. The first actual native retirement invocation then exposed an application fixture mistake: a one-second request completed tessellation instead of retaining it. The fixture now uses an uncached tolerance and zero wall limit, which still performs one real budgeted step before the deadline. Assertions continue to require both unfinished work and positive units done.
- Actual BREP owned canonical native target passed all three exact laws after the fixture correction, receipt exact-cargo-laws-EM0Dlk/00, binary SHA-256 e009fbe0cb29ed6af820571981a870ff32cb160db834b6b0777acaf0988a8ec6. Runtime observation recorded actual guest retirement of 245 items and 282643 bytes after the unfinished tessellation, and terminal emptiness. That temporary diagnostic was removed and replaced with a retained positive-byte assertion; the final clean-source rerun passed, with the final guard-source receipt recorded below.
- The regular BREP test target initially stopped in a dependency before native tests because the newly introduced manifest input-label JSON was absent from exact WGPU source admission. Root owns and corrected that admission; this report does not claim a regular BREP test pass.

## Scope And Remaining Validation

No runtime dependencies or foreign public API types were introduced. The dedicated BREP canonical-architecture target uses the existing exact native harness; the neutral target preserves every original law. Root owns launcher/catalog generation, fresh descriptors and whole-corpus validation. Concrete ExtensionResourceOwner implementations must uphold terminal emptiness and shallow final shell destruction; actual BREP owns SessionCapture through its final release. Validation receipts below distinguish the earlier lifecycle/guard runs from the later allocation-admission source and its final clean focused runs.

Final clean-source neutral canonical rerun completed after adding non-closing capability/quota events and the freshness owner: assertions=9, binary SHA-256 0421b8795d9a5dda8f34635e0044cd3c574f7a3c0c78a2fd72c6fbdf7116334e. Both portable oracles ran on the clean source and all nine exact native invocations passed. The parent actual guest pipeline compiled the cfg-specific WASM bridge before the final bundle guard change; the parent owns refreshed guard-source producers.

The final guard adds a fifth retirement law, bringing the preserved canonical set to ten native laws and the neutral retirement oracle to nine cases. Final guard source owned Nx canonical-architecture passed all ten exact native laws and both portable oracles: retirement9 and freshness17. Receipt exact-cargo-laws-Y16B4t/00, binary SHA-256 1ed7a26e3e95d5c198a3f093508e9990278e104f620ad4d1861944a39681eef7. The refused destructor test printed the expected guard panic while retaining and subsequently retiring the same payload. Final guard-source actual BREP3 passed, as recorded below. The parent confirmed actual Sequence 1/1 and Playbook/procedural 2/2 guest pipelines passed before the final guard change; refreshed guard-source producers remain parent-owned.

The first guard-source actual BREP run compiled the full concrete source and passed both genuine guest laws, then its existing installed-bundle law hit the enforced TLS Drop boundary. This worker had placed explicit disposal beside an earlier identical geometry assertion rather than the installed caller. The close call was moved to extension_bundle_extends_flow_and_evaluates_box; its final three-law rerun passed. No production guard was relaxed.

Final clean guard-source actual BREP canonical-architecture passed all three exact native laws plus the six-case AJV/application oracle. Receipt exact-cargo-laws-fEr53J/00, binary SHA-256 1ce4950354ec527f54bc9de5e2116941d2a2733a8c061536f72b29d18f59f211. The exact laws confirm identity inspection retires its owner, genuine evaluation/unfinished tessellation closes through item/byte grants with cancellation, and the actual installed registry caller explicitly retires before TLS destruction. No temporary DEBUG logs remain in this worker’s source. All worker-owned generated logs, temporary filesystem inputs and exact-build artifacts have been removed after recording receipts in these persistent reports. Other workers’ generated outputs and the durable shared Cargo cache are preserved.

## Final Allocation Admission Follow-up

The independent Low review identified real metadata backing and dynamic resource-box byte gaps. Earlier passes above are historical proof of content/lifecycle behavior, not final allocation admission. Schema-first allocationAdmission vectors add one-byte hostile grants, reserved empty vector capacity, a string with spare capacity and a large concrete owner shell. The actual red canonical run compiled and executed: original ten exact laws passed, then the new owner law failed because the prior production removed its large Box under one byte. Receipt directory exact-cargo-laws-7fW1hK/00 records that red observation.

Extension metadata now uses an intrusive typed frontier whose every node and concrete metadata Box is admitted by its actual layout before destruction. A String releases only when its entire owned capacity plus both shells are admitted. Vectors move one child at a time, retain their original backing, and only destroy the empty backing under its capacity-based layout grant. The pending Vec and delegated RetainedCloneClose were removed from this owner, eliminating hidden stack capacity and delegated unmeasured allocations. Empty metadata means no frontier nodes or retained backing. Private handlers use a sorted Vec with binary-search lookup and cold insertion; close moves one key/function pointer per item, then transfers its empty backing through the same admitted cursor instead of guessing opaque std BTreeMap node layouts.

The concrete dynamic resource Box is admitted by size_of_val before final removal. Manifest and handler terminal witnesses require zero capacity, including empty-but-reserved fields. The registry current Option stores its bundle inline: after every nested allocation is empty, moving/dropping that inline value frees no heap, so its receipt correctly remains zero bytes. The new law validates this distinction against actual Layout::for_value/Layout::array allocation sizes, preserves exact cursors across zero/tiny/just-insufficient grants, and confirms one-child expansion retains the still allocated vector backing. The portable oracle uses existing AJV plus independent ArrayBuffer/Buffer byte spans alongside its UTF-8 encoder checks.

Next-close-byte demand is first-party. Cold disposal reads the exact demand instead of assuming 64KiB always suffices; interactive actor retirement respects the supplied max_patch_bytes grant and retains an under-granted allocation. Actual BREP resources now own SessionCapture, keep borrowed Session invokes through Deref, and remain nonterminal through the concrete final Arc/layout release before the generic Box shell is removed. Begin/cancel/resume guard an empty capture and shell_byte_requirement supplies cold demand. No raw Session final destruction is hidden in the resource-box release.

Updated scope remains precisely the already listed owner, plugin root, reactor turn, fixture/schema/native and portable law, owned canonical script and actual BREP owner files. No additional source files were created or removed. Generic canonical now preserves original five plus six retirement laws (eleven native), portable retirement eleven cases and unchanged freshness seventeen cases. Final validation is running; parent owns guest producer pipelines and full aggregate, and the Cargo owner owns concrete Session allocator validation.

Final accounting-source generic canonical-architecture completed: eleven exact native laws passed, retirement portable oracle eleven and freshness seventeen passed. Exact receipt pKkZDp/00, binary SHA-256 d84fe4a7457c123af733d782a3f87e85aa36405566896fbaf7b46b9912b3ced9. No assertions or originals were weakened; old exact-grant owner law now additionally refuses undersized final-shell disposal before granting its actual Layout. No new DEBUG logs were introduced. Actual BREP capture-owner native validation remains in progress.

The first allocation-source BREP run compiled successfully, then its identity driver exhausted its fixed 64KiB grants. A temporary DEBUG run measured the actual parked status: AwaitingInput for a metadata allocation whose exact demand was 135525 bytes. The concrete capture initially released its actual 200-byte state shell and progressed through family retirement; the stop was the correctly retained metadata allocation. The diagnostic was removed. Both actual BREP bounded test drivers now read max(fixture minimum, next allocation demand), and preserve per-step item/byte receipt checks against the grant actually supplied. No production admission was widened.

The canonical first-party API extension_next_close_byte_demand exposes the installed registry's exact retained demand without advancing it. A closed neutral actorAllocation vector and independent Buffer oracle now accompany the exported actor law: an actual 131072-byte manifest String parks under 64KiB, preserves exact demand across zero fuel and cancellation, then progresses with an actual 2MiB supplied budget and QuotaChanged without reopening invocation. Existing production browser maintenance budgets provide 2097152 bytes in plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts:354; the native renderer default provides 1048576 bytes in renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4560. These concrete grants exceed the measured BREP allocation. Permanently insufficient grants correctly retain and park the owner; callers must supply a grant meeting its demand. No new wire protocol or speculative larger-payload behavior is claimed. The final unchanged eleven-law native set now includes this expanded actual actor assertion; portable retirement has thirteen assertions, freshness retains seventeen.

Clean final-source retries reached the actual shared UI contract while its InputProps snaps field was being edited by another worker. BREP receipt EenDj0/00 and neutral receipt NuWl7P/00 stopped at compile errors: omitted snaps in the typed field catalogue and builder initializer, plus copy/compare/retirement exhaustive field assertions. No owned retirement runtime law ran in those retries, so they are not pass claims. The portable retirement thirteen and unchanged freshness seventeen assertions did execute and pass. This worker preserved the concurrent UI source and awaits its owner's settled compile notification before repeating the two focused native targets.

The parent settled the UI field routes; the next concrete BREP build, wPZD5E/00, advanced past those routes and observed an older unqualified HISTORY_EDIT_COMMIT_ACTION_ID at plugin line26978. Current source already contained the parent's canonical semio_framework qualification, so this worker made no source edit and launched a fresh BREP build against that inspected fix. The neutral retry remained active. These stopped builds are retained only as diagnostic receipts until final focused validation is recorded.

Final clean neutral canonical-architecture passed all eleven exact native laws, including the actual parked metadata actor assertions for zero fuel, cancellation and later QuotaChanged with the supplied maintenance-sized budget. Both portable oracles passed: retirement13 and freshness17. Receipt exact-cargo-laws-Z6T0xK/00, binary SHA-256 3f3c8a1c219a8f5f6fe96ac170c5924796655bae4fa3cb119b0b6a3595a9dbdf. Actual BREP retry g5x2Ih/00 advanced to another concurrent UI initializer, wgpu component line2702 UiInputNode missing snaps. The shared field owner was notified; this worker did not edit that source or claim any BREP native runtime pass for the stopped build.

After recording the red, measured diagnostic, stopped-build and final neutral receipts, eighteen completed worker-owned allocation logs/receipt directories were removed using exact direct-child path and file/directory guards. Inputs, reports, other workers' artifacts and durable Cargo cache were preserved. The only remaining owned generated output is the active final WGPU-fixed BREP run until its result is recorded.

Backend receipt scope was explicitly handed to the parent and Cargo owner: the pre-existing S native PayloadRetirement decremented an admitted payload-byte remaining counter while retaining its Vec until the final decrement, and did not measure its own boxed/list shells. That older backend receipt semantics is distinct from this worker's actual freed-layout accounting for neutral metadata, dynamic resource boxes and concrete capture Arc shells. The parent/Cargo owner owns assessment or correction of that native engine; this report does not silently reinterpret its older payload credits as physically freed allocator bytes.

Read-only compiler lock diagnosis is persisted in 🔒️2026-09-30-brep-native-cache-lock-assessment.md. A one-second owned-process sample confirmed prebuild_lock_exclusive -> LockManager::lock -> flock. Private target roots still share compiler-unit locks through the configured build-dir. After the parent independently confirmed the same blocked state, it authorized stopping only this worker's queued Cargo90935 and rerunning the same three exact BREP laws with BOTH CARGO_TARGET_DIR and CARGO_BUILD_BUILD_DIR isolated inside this ticket's generated directory. The owned queued process was terminated, its runner exited, and the same Nx target was launched with those two first-party overrides. No peer job, durable cache, source or Git state was touched.

Final clean actual BREP canonical-architecture passed all three exact native laws and the six-case existing AJV/application oracle. The fully isolated build completed in2m07s and the entire target in2m09s. Receipt exact-cargo-laws-U4rnId/00, binary SHA-256 b1392915fda97c340330811d083844bc3716a6cfd8aeb9a4838ff52f517b89fd. The same original laws verify identity disposal, a real evaluated box plus unfinished tessellation, zero grants, cancellation, actual supplied item/byte receipt bounds, terminal-empty capture/manifest/handler ownership, and explicit installed registry disposal. No temporary DEBUG code remains in this worker's production or tests. Final neutral11, retirement13, freshness17 and BREP3/oracle6 are executed passes; producer pipelines and whole-corpus validation remain parent-owned.

The exact production/test/fixture/script manifest above remains unchanged by this allocation follow-up. The additional created persistent artifact is ticket report 🔒️2026-09-30-brep-native-cache-lock-assessment.md; extension and freshness reports were updated. The old backend cumulative-payload versus physical-shell accounting distinction was assessed read-only by the Cargo owner and preserved in both reports; no native engine correction or stronger full-family allocator claim is asserted here.

Settled owned source identities after the final passes: private extension retirement owner SHA-256 c9b2bf061c4a25b9997bdd327c9d382ffe8bc939542c5b673f96c56f7a2fe75f; neutral native retirement law 7aa1bd8548eb2f242c002da54ab7d3d5d46041770aa2906fca885b1f688c63ea; actual BREP guest native law 30c166ba7b5ca8f0226c4355a7e2e40ae452cbf7283eb90427ec4fa2598b5aa9. Shared plugin/BREP root files also contain peer-owned edits, so whole-file ownership is not claimed.

Final cleanup removed the six remaining owned diagnostic/queued-build/isolated-build output entries after recording all receipts, including the isolated compiler cache. This worker has no pending jobs or generated output. Persistent reports, fixture/source inputs, all peer artifacts and the durable shared Cargo cache remain intact. Ticket/goal lifecycle and parent final validation are unchanged.
