# Embedded Flow and Procedural Questions

This implementation extends the existing procedural question provider in the open Forms ticket. Root, s and Playbook instructions, the build skill, and existing ticket audits were read. Commands enter through repo. No Git mutation, worktree, AGENTS.md edit, or dependency-cache receipt change was made by this worker.

## Delivered Source

The provider registers Forms kinds buildingComponent, flow and procedural3d, with six declared bodies: params, preview, flowParams, flowPreview, proceduralParams and proceduralPreview. Playbook remains the primary extension host; Forms activates this contributor through its consumed topic.

Flow/procedural3d authoring provides a localized Flow Document textarea. Graph authority comes exclusively from authoredParams when a host baseline exists; answer.__flow cannot replace configuration. Generic parameter controls retain exact parent question and Fill-window addressing. initialValueJson contains only authored input widget values and excludes graph configuration. The Forms core worker owns matching initialValueJson admission and consumer envelope changes.

Read-only answer/authoring controls are disabled and have no bindings. Read-only users may run/cancel ephemeral computations, as they may operate a preview camera. The exact null answer remains null: preview defaults are display/evaluation inputs and never fill or mutate that answer.

The native per-instance registry contains primitive/core, math, text, list, dictionary, logic, draw, brep and bim. Registry operator metadata is built once in a preborn per-instance template and shared by new hosts; each host and the template have explicit close ladders.

The retained evaluateEmbeddedFlow view action uses a resumable tool factory. Evaluation and tessellation advance with the genuine caller StepContext and operation/generation identity. Cancellation closes tessellation and host owners under supplied grants. Authored input changes invalidate the cached identity and refuse obsolete work before exposure. Previous completed previews transfer to the originating work close ladder, so repeated Run does not accumulate completed previews in the instance owner.

Interactive rendering reads cache/status and emits Run Flow/Cancel Flow controls. It no longer calls the synthetic unbounded tessellation loop. Generic output widgets become authored, labeled result fields: the tiny graph renders field answer with value 7 and key embedded-flow.output.answer. Output JSON is shown only for structured result values, without dumping the internal evaluation envelope. Progress shows evaluation/geometry phase and actual funded-turn count. Busy Run and idle Cancel controls are disabled. Completion explicitly requests full UI refresh. Empty geometry does not invent a fallback box. Explicit cold export/test helpers remain.

ModuleRenderPayload has separate authoredParams with invertible payload diffs and updated wire fixtures.

## Admission and Preview Limits

Shared schema/fixtures and native admission specify:

- Authored flow source: 16,384 UTF-8 bytes; nesting depth 32.
- At most 32 uniquely named widgets and 64 uniquely named synapses; endpoints must exist.
- Widget/synapse identities are nonempty and bounded to 128 UTF-8 bytes; __flow is a reserved widget identity.
- Evaluation setup payload: 65,536 UTF-8 bytes.
- A displayed output value: at most 512 UTF-8 bytes, 128 visited entries and depth 32; larger results show a localized diagnostic.

Setup decoding, bounded snapshot clone/host birth and mesh JSON serialization remain bounded synchronous substeps around the resumable evaluator/kernel. This work does not claim every setup allocation or serialization byte is individually metered. Output collection is split into one authored widget per work turn. All heavy evaluation/tessellation paths use the existing retained owners.

## Validation

- Passed: repo tools exec --light bun -- test ./✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️geometry-lifetime/🟦️.ts. Latest result: 4 tests, 25 assertions, zero failures, 184 ms. Tests use independent Ajv admission and shared vectors.
- Initial filtered native test invocation was cancelled while capacity-queued, exit 130; no assertion was run and no native red result is claimed.
- Two geometry-contract attempts stopped before compilation on repository cache budget consistency errors. The root worker repaired source ownership/path consistency; this worker did not alter cache custody.
- Third geometry-contract attempt stopped before compilation on a concurrent undefined envelope variable in repository strict storage. Log: generated/embedded-flow-native.log. Root corrected that source error.
- Declared native owner repo run @semio-tech/playbook-extension-procedural-rust:geometry-contract, exec session 72137, log generated/embedded-flow-native-retry.log, ultimately exited 1 before Rust compilation on a missing repository publication export. Dependency synchronization completed before final source edits resumed.
- Additional production check: repo cargo --store=shared --configuration=prototype -- check -p semio-s-plugin-playbook-procedural --lib; exec session 21124, log generated/embedded-flow-check.log. Exited 1 before compilation on Build native caller capabilities refused.

The exact native owner includes payload/geometry/retirement laws; authored graph execution and computed numeric output; input routes in English/German; read-only/null isolation; configuration authority and bounds; cancellation; and the mounted Run/output/cache lifecycle. Native results remain pending.

Root/integration_validation owns descriptor and catalog regeneration. Browser_execution owns child action/effect/progress routing and real Forms browser journeys. No browser extension success is claimed by this report.

## Files Owned by This Worker

All production implementation paths are under ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural: Cargo.toml; 📦️packages/🦀️rust/Cargo.toml; 🦀️.rs; 🎒️payload-codec-and-diff/🦀️.rs; 🧱️learning-program-and-geometry/🦀️.rs; 👁️preview-and-params-body/🦀️.rs; 💾️media-export-and-command/🦀️.rs; new 🌊️embedded-flow/🦀️.rs; new 🧵️embedded-evaluation/🦀️.rs; 🧬️schema/🔣️.json; 🧬️schema/🧬️mutations/📦️set-payload/🦀️.rs.

Tests/fixtures: 🧪️tests/🔬️unit/🦀️.rs; 🧪️tests/🔬️geometry-lifetime/🟦️.ts; 🧪️tests/🔬️rust-package-commands/🟦️.ts; 🧪️tests/🧫️fixtures/🛂️actor.json; 🧪️tests/🧫️fixtures/🧩️embedded.json; 🧪️tests/🧫️fixtures/🔁️payload-mutations.json; new 🧪️tests/🧫️fixtures/🌊️embedded-flow.json; 🧪️tests/🧫️fixtures/🎚️controls.json; the two existing set-payload wire fixture JSON files under 🧪️tests/🧫️fixtures/🧬️mutations/📦️set-payload.

Ticket artifact: embedded-flow-results.md. Generated logs belong to the ticket generated directory and must be removed at final ticket closure after their meaningful outcomes are retained here.

## Bounded UI Transport Follow-up

The shared UiText scalar ceiling is 512 UTF-8 bytes. Provider envelope decoding now accepts only paramsJsonChunks, and Run uses only payloadJsonChunks; each list is bounded to 256 chunks and 65,536 total UTF-8 bytes. Parent parameter baselines use initialValueJsonChunks (16,384 total bytes), excluding graph/configuration. All chunks use the shared UiValue::try_text_chunks helper owned by the root worker. No global scalar limit is raised and no legacy string route is retained. The browser worker implemented change-commit settings on the existing paged TextEditorScene. Provider source uses that paged scene with exact parent patchQuestions routing; note controls use its JSON-chunk answer encoding and exact parent SetTryValue window arguments.

Busy previews now include the shared indeterminate Progress component, with localized phase/value text, alongside status and cancellation. Cache retirement uses inline ControlledRetirement slots and exact child demands, not an unreserved retirement queue. A dedicated native tiny/zero-grant close law is registered.

The production repo cargo check exited 1 before compilation: Build native caller capabilities refused at repository build/caller, retained in generated/embedded-flow-check.log. No compiler result is claimed. An auxiliary rustfmt check was cancelled while capacity queued before running (no parser result). The geometry owner has released its own graph loader and waits for the peer graph build; source implementation native verification is still pending.

## Read-only Integration Review

Reviewed the root worker's canonical UiValue text chunk splitter/joiner, Forms SetTryValue chunk fields/start/apply/native fixture consumers, and Responses long-answer read-only paged scene. No concrete compile/API blocker was found by source inspection. The Responses test include_str path resolves to an existing fixture; updated SetTryValue constructors use defaults for new optional fields. This is source inspection, not compiler evidence. One existing semantic behavior remains: apply_single maps malformed complete JSON to Null; new valueJsonChunks currently inherits that behavior. Root was notified. No production source was changed by this review.

Reviewed browser-owned cancellation fixture at 🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🧫️fixtures/📋️forms/🌐️cancellation.json through repo tools exec --light bun -- -e with Ajv EmbeddedFlowParameters admission plus independent exact comparisons against four canonical column copies. Passed: compact source 6,353 UTF-8 bytes, 28 unique widgets, 24 unique synapses, all endpoints and ports match; sides value/max is 120. An initial comparison expected canonical edge numbering e1..e6 and refused c0-e1; corrected inspection to fixture's intentional e0..e5 identities, then passed. This validates authored graph structure, not native evaluation duration or live cancellation.

Read-only Cargo/install API review through repo tools exec --light bun -- -e and Bun.TOML.parse passed for all seven added internal operator dependencies: primitive, text, list, dictionary, logic, draw and bim. Each workspace relative path resolves to an existing manifest with the exact expected package name; provider package entries select workspace=true. Each library glue publicly reexports its owner and its unguarded pub fn register(registry: &mut neural_engine::Registry). All seven reference the same internal neural engine and Flow SDK paths/packages as the provider. No concrete API/path mismatch was found. Compiler verification is still pending. Root subsequently replaced malformed complete-answer JSON normalization with a refusal and added shared invalid JSON vectors; that source behavior now supersedes the earlier review observation.

Root caught and corrected a Responses native-law visibility issue after the read-only review: this test is compiled in the response-events mirror crate, so private production answer_row was unavailable. The law now calls public render with one response and recursively locates the long-answer scene before exact carrier-text comparison. The prior no-concrete-blocker source review did not model mirror-crate ownership and is not compiler validation. Config-test new SetTryValue constructors use public fields/API.

Browser worker subsequently varied each cancellation-copy radius (.5 + index × .05) and height (6 + index × .5), within authored slider bounds, to ensure distinct content-addressed geometry rather than shared tessellation. The earlier exact canonical-copy comparison predates this intentional default variation; schema/identity/port reasoning remains applicable.

## Initial Text Paste Correction

Root identified a first-edit transport gap: text controls previously switched to paged editors only after the stored value exceeded 512 bytes, so the first large paste into an empty scalar Input could refuse. All provider text and longText controls now use the existing paged TextEditorScene from their initial empty state. The scalar Input branch now serves only number controls. InputImage/Variable node admission accounts for four nodes regardless of current text length. Canonical JSON-chunk parent routing and read-only settings are unchanged. Shared controls vectors now require a surface for short and empty text; shared longInput vectors cover initial empty text/longText, and the native mounted-control law asserts both route through text-editor/valueJsonChunks/json-chunks. These regression assertions were authored before the production fix; a native red was not executed because the existing declared native owner remains capacity queued.

Reran repo tools exec --light bun -- test ./✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️geometry-lifetime/🟦️.ts after the correction: 4 passed, 0 failed, 25 assertions, 141 ms. This oracle validates schema/bounds/UTF-8 fixtures; native control projection assertions remain pending.

## Artifact Viewer Extension Gap

Read-only source assessment confirms Forms artifact Viewer does not currently route extension questions: viewer Try render_view_question returns a plain (kind) field. FormsViewer.render supplies only doc.snapshot; its Config is NoConfig and it ignores render config/view context. Provider read-only/native laws therefore test direct mounted payload behavior, not actual artifact Viewer integration. Editor contributions::render_extension_question accepts question, exact values Object, ProgramContributionEntry slice, ExtensionSurface, interactive flag and editor FormsLabels; it emits params+preview external slots. Editor questions/extensions::render_payload also hardcodes Forms Try windowKind. Recommended artifact-level shared routing/envelope helpers take explicit controller id, optional window identity and resolved diagnostic label, with contribution context injected into the Viewer independently of editor ownership. Provider already accepts read-only surface=try with no window and exposes only child Run/Cancel view computation. Root owns the Viewer/core refactor; this review changed no production source.

## Shared Forms Extension and Config Ownership

Moved the complete Forms question extension subtree from any/✏️editor/❓️questions/🧩️extensions to any/🧩️extensions. The canonical crate::question_extensions now owns QuestionKindContribution, topic route lookup, exact render payload and bounded params/preview slots. Its render_extension_question accepts explicit controller id, ExtensionSurface, interactive flag and localized unavailable label. ExtensionSurface::Try receives explicit window id/kind; View emits read-only Try presentation with no mutation lease. Editor wrappers supply their own labels/controller. Rust mirrors/public exports, TS imports/public exports, schemas, fixtures and consumers use the canonical ownership; no old route remains. Shared schema permits omitted window identity only for read-only payloads and preserves exact Forms Try answer leases. A shared Viewer no-lease vector is consumed by TS/Ajv and native serde tests. Initial oracle caught overly broad windowKindId schema admission; restored exact forms-try lease restriction and reran successfully.

Moved the complete app host-projection config subtree from any/✏️editor/🎚️config to any/🎚️config. Canonical crate::forms_config owns FormsConfig, FormsConfigMutation, SetContributions, schema and IO. Types, wire/schema identities and contribution JSON remain unchanged. Config package paths/workspace references were rebased, semantic consumers and bridges use the shared namespace, and dependent crate roots reexport it. Editor modules retain structural carve reexports only. An independent Bun.TOML/filesystem audit found no missing relative path targets across Forms Cargo manifests. No package.json dependency edits were made. Root owns the actual Viewer contribution injection, command and renderer.

Added a shared config TS/Ajv oracle against the moved committed config mutation corpus, plus public FormsConfig/parseFormsConfig exports. First oracle run refused the existing x-semio-state schema annotation; registered that annotation with Ajv without weakening validation. Final command: repo tools exec --light bun -- test ./✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪶️consumers/🟦️.ts. Result: 21 passed, 0 failed, 78 Bun assertions, 833 ms. The shared extension function also passed a direct repo Bun invocation with independent Ajv, including the no-lease Viewer vector. Native code for these ownership changes has not compiled yet; existing geometry owner remains active and root reports unrelated drawing macro compiler failures in its own native attempt.

Final shared-config cleanup supersedes the earlier structural-reexport note: all obsolete editor::forms::config wrapper modules were removed (nine wrappers and one empty definition IO mount). Canonical config native unit/contract-vector laws remain mounted at the config-tests crate root. No semantic editor config namespace or former physical path remains in Forms sources; root owns the separate old Viewer empty-config declaration replacement. The config mutation subject now calls public forms_config::io rather than the private former component::io path. No further source edits are planned except validation fixes.

Removed obsolete Forms Viewer empty configuration declarations after root switched Viewer to canonical shared FormsConfig/FormsConfigMutation: deleted the seven-file any/👁️viewer/🎚️config subtree, removed its public artifact export and config-tests schema mirror mount. No FormsViewConfig alias or duplicate config schema remains; no old Viewer config references remain in Forms sources. Root owns Viewer entry, Try rendering, view command wrapper and coverage.

## Viewer Retained Path Read-only Review

Reviewed root's new Viewer command, retained factory, config publication/proof and shared slots against ArtifactViewer SDK signatures and Generation3d contribution reference. Production Viewer implementation is compiled by the response-events/tests2 crate, whose dsl/protocol/store aliases and direct job/dispatch/DSL/config dependencies match the new macro/factory requirements. No direct editor imports were found in new Viewer source. The protocol boundary remains public invocation string capacity 4,096 bytes versus the new factory's 16,384 raw-byte bound; Generation3d explicitly pages larger contribution closures. Root was notified that large host closure strings can refuse before retained admission.

Found a concrete native law owner mismatch before compiler evidence: the Viewer unit file is mounted by the questions carve crate, whose root does not expose forms_config/question_extensions/schema/FormsSnapshot/FormStep/forms_snapshot_with_state/FORMS_DOCUMENT_SCHEMA or protocol alias required by the two new laws. Recommended moving those laws beside actual Viewer implementation in tests2 rather than adding editor-related imports to Viewer files. Root owns the correction. No duplicate native command was started.

Native owner mapping is retained in native-owner-map.md. Full main-editor mirror review corrected an initial incomplete-search conclusion: Try unit tests, including the new initial-empty/full paged text law, are already cfg(test) mounted by semio-s-artifact-forms-forms-forms. No extra mount was needed. The two Viewer integration laws were deliberately moved out of the questions crate and mounted beside actual Viewer implementation, in tests2; fixture include paths were computed with path.relative and verified. The actual provider manifest projection now has a native public-ingress cost assertion registered in geometry-contract; pending execution.

Latest native owner terminal outcome: session 72137 completed with exit 1 after native owner launch. Its script loader refused missing named export packageAdmittedExtensionComponentV1 from framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/📤️publication/🟦️.ts. No provider Rust compiler or assertion executed. Root/integration workers were notified; no duplicate retry started before source export repair. This supersedes earlier pending owner/queue notes.

## Dedicated Viewer Cargo Owner

Root comprehensive native invocation failed before compiler on tests2 SourceFiles admission reaching its existing 32-source ceiling. Created canonical semio-s-artifact-forms-forms-viewer at any/👁️viewer/📦️packages/🦀️rust and registered it in the Forms Cargo workspace. It owns the Viewer entry, retained contribution work, mode/Try and all Viewer unit/window/integration laws. Removed Viewer implementation/test mounts from tests2/questions; main editor/root facade consume the new owner directly, while tests2 consumes its shared child projection. No backward implementation wrapper or budget increase was introduced. Config and question routing stay in their common owners.

A Bun.TOML/filesystem dependency audit found no missing Forms dependency paths. A DFS of normal Forms package dependencies proved the new Viewer closure acyclic and excluded tests2. Native owner map is updated; production source count is approximately six, plus three native test files. No compiler success is claimed. Root was given the new package name for comprehensive retry. No package.json edits were made.

## Typed Variable Relay Semantics

Read the actual Flow Widget::Variable model, host mutation and core.variable operator. Variables hold authored name/schema and relay an incoming typed dictionary; they do not hold a scalar answer value. Removed the erroneous answer control/default and set_variable_name application so submitted answers cannot rename graph channels. Only true slider/note/image input widgets project answer controls. Added the policy to schema documentation and a neutral three-case typed-relay corpus with core.number, core.boolean and dictionary.pack source neurons. Native law compares exact relay output against independent serde JSON, asserts authored name survives an adversarial answer and excludes relays from initial answer defaults. It is registered in geometry-contract. Ajv/JSON oracle passed 5 tests, 40 assertions, 313 ms; native execution remains pending.

Final architecture audit found tests2 depended on the new Viewer solely for the artifact-neutral forms_child_restore_projection helper. Moved that helper into the existing shared Snapshot schema source and exported it through snapshot crate. All app/facade aliases now consume Snapshot directly; tests2 no longer depends on Viewer. Existing Snapshot dependencies already supply composition admission and plugin Fault. No extra source file, crate or budget was introduced. Viewer assembly dependencies remain only where the app is assembled.

Current geometry owner retry is session 23987, log generated/embedded-flow-native-final.log, after root fixed stale extension-package imports through the canonical process interface. Latest schema/relay oracle after explicit fixture shape guard: 5 passed, 0 failed, 40 assertions, 175 ms. Native provider compiler evidence remains pending.

## Authored Graph Commit Transport Audit

Read-only audit found a concrete remaining end-to-end gap: the authored graph paged editor still emits plain value, while public command strings cap at 4,096 and PatchQuestions has value_json String only. The browser cancellation graph is over 6KB compact. Paged rendering alone does not make that commit admissible. Root/browser workers were notified to add canonical generic PatchQuestions valueJsonChunks and switch graph editing to generic json-chunks encoding. The existing encoding helper also caps serialized text at 16,384 bytes; an authored graph can occupy 16,384 raw bytes before JSON-string escaping, so this path needs an explicit caller-owned encoded budget (up to existing 65,536 transport ceiling) while Try answers retain their16KB canonical bound. No production source was changed by this audit; root owns command transport, browser worker owns neutral settings schema/mapper.

## Canonical Paged Command Ingress

The graph transport gap above is implemented. PatchQuestions now has one typed valueJsonChunks carrier with strict mixed-transport refusal, 65 chunks of at most512 UTF8 bytes and32,770 total encoded bytes. Target admission bounds64 question identities,512 bytes per identity and8,192 aggregate identity bytes, with128-byte field/parameter keys. Scalar controls normalize into this same typed payload. Schema, TypeScript twin and neutral good/refusal fixtures precede the native reducer law. Provider authored graph and Blueprint text editors use generic json-chunks encoding and explicit65,536-byte editor encoding allowance; the graph parser still enforces16,384 raw UTF8 bytes. The shared near-bound graph contains16,375 raw bytes and24,516 JSON-encoded bytes.

Split exactly setTryValue and patchQuestions into FormsPagedValueCommandJobFactory:128KiB raw request,640 decoded items,64 work units,4KiB output and7,500 microsecond first-step contract. The27 remaining tools retain their16KiB/64-item contract. Publication lanes and proof rows have exactly one owner per command; no global ceiling changed.640 decoded items cover both independently permitted256-page SetTry arrays plus metadata. Added native laws in the existing editor unit owner for the near-bound graph through actual reducer mutation, public invocation character cost of near-bound answer plus baseline, and actual registered factory staging with both256-page arrays. A257th page is refused by the shared join decoder. Root reported public consumers25 passed and strict selected TypeScript check exit0. Rust compilation/assertions remain pending; no browser result is claimed.

Read-only API review confirmed ToolExecutionContract fields max_raw_wire_bytes and max_decoded_items, the generic producer preserves the selected payload_schema_id, canonical UiValue serde emits ordinary JSON arrays, and every new include_str target resolves. Corrected new Forms native references to the existing semio_ui_contract Cargo crate name; provider-specific aliases were untouched.

Geometry session23987 ended1 after actual Cargo compilation failed in prerequisite neural cache/replication imports (too many super qualifiers, missing Dictionary and HistoryFoldIndexRead); no provider native assertion ran. Root repaired the canonical prerequisite imports/reexports. Geometry retry is intentionally sequential after root comprehensive native session88782 (forms-native-paged.log), avoiding duplicate native builds.

## Exact Geometry After Parameter Edit And Cancel Rerun

The durable geometry corpus now evaluates heights3.0,6.0,6.5 through the actual kernel and independently parses mesh coordinates to assert exact height/radius, detecting a stale6.0 mesh for6.5. The retained cancellation law now consumes the browser canonical four-column procedural3d graph, cancels its original operation, reruns the same route, settles real evaluation/tessellation, and reconstructs the public paged World3d mesh/instance lanes. It requires all four cN-column-preview instance identities to address their computed mesh, independently verifies each mesh vertex height/radius against authored inputs, and checks four distinct bounds. Parent snapshot is exact before/after both cancel and completion. No debug bridge or fabricated geometry was introduced. Native execution remains pending. The existing neutral schema/serde oracle rerun passed5 tests,40 assertions,148ms.

## Independent Geometry Output Oracle

Added a test-only interface accepting plain column dimensions and returning plain height/radius. Its existing Three.js implementation constructs CylinderGeometry, rotates its axis toZ, obtains Box3 bounds from actual vertex attributes and independently measures radial vertex distance. No runtime or manifest dependency was added; no external type is exported. Both native mesh assertions and this oracle consume the same language-neutral durable3/6/6.5 parameter corpus and browser four-column authored graph (6,6.5,7,7.5 with distinct radii). Native kernel assertions are retained. TDD first failed because the oracle interface was absent. Exact rerun command: repo tools exec --light bun -- test './✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️geometry-lifetime/🟦️.ts'. Result:6 passed,0 failed,56 assertions,199ms. Third-party geometry law measured all seven input cases and four distinct bounds, rather than merely validating schema. Native kernel/cancellation law still awaits its actual owner run.

Count qualification: the shared cancellation graph explicitly enables both each extrude neuron preview and each outputPreview. The spatial projection therefore correctly emits eight preview mesh/instance records for four distinct authored columns. The exact native oracle scopes the four cN-column-preview instance identities, then follows each meshId; it does not incorrectly require all authored preview records to total four. Browser owner notified to use the same scope or distinct bounds.

## Blueprint Source And Patch Admission Boundary

Fixed the concrete40000-byte ASCII note mismatch from wire-admission-audit.md. The provider schema now declares QuestionControlTransport.blueprintEncodedBytes=32770 and answerEncodedBytes=16384. Both authored flow source and Blueprint text parameter scenes use the same local BLUEPRINT_VALUE_MAXIMUM_BYTES constant; Try text scenes use ANSWER_VALUE_MAXIMUM_BYTES. This is the existing Forms encoded value domain contract, without importing the Forms implementation crate. No global UI limit, PatchQuestions limit, factory128KiB contract or raw graph16KiB limit changed. A valid raw graph of16384 UTF8 bytes remains fully representable: JSON string encoding needs at most2N+2=32770 bytes.

Changed provider production files: 🦀️.rs (two local domain constants), 🌊️embedded-flow/🦀️.rs (graph source setting), 👁️preview-and-params-body/🦀️.rs (Blueprint/Try source setting), 🧬️schema/🔣️.json (language-neutral budgets). Extended existing provider native flow-note law to compare actual emitted scene settings against schema for both surfaces. Extended existing Forms native patch ingress law to consume the shared at/over boundary fixture. Native assertions remain pending under their existing owners.

Shared TextEditor encodingBoundaries fixture, authored by browser owner, supplies a maximally escaped quote source: encoded32770 bytes accepts,32771 refuses. Canonical Forms PatchQuestions test now drives actual textEditorValueSourceActionV1 through normalizePatchQuestions and checks admission/refusal against Buffer byte counts, JSON.parse and independent Ajv schema validation. It also sends the existing nearbound graph (16375 raw/24516 encoded) through that complete source mapper→domain ingress route. The neutral generic TextEditor65536 support fixture remains intact; domain callers now select their exact lower budget.

TDD reproduced the real boundary mismatch before the production fix: source mapper with the old65536 setting failed the new over-limit refusal assertion (Missing expected exception). After alignment, exact focused command repo tools exec --light bun -- -e 'const {testFormsQuestionChunkIngress}=await import("./✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-questions/🧪️tests/🟦️.ts");testFormsQuestionChunkIngress();' exited0. Provider geometry/schema oracle exact prior command rerun:6 passed,0 failed,56 assertions,184ms. No native/compiler/browser success is inferred from these TypeScript checks.

Typed mapper follow-up: generic TextEditor source refusal is now an owned {kind:refused,reason:source-too-large} result, with dispatch/local alternatives. Updated the Forms boundary consumer to narrow dispatch before reading action.args and assert exact typed refusal for32771 bytes. Focused same canonical ingress oracle rerun exited0. This replaces the previous exception expectation only at the renderer interface; canonical PatchQuestions over-budget ingress still refuses, and32770/nearbound graph acceptance remains intact.

## Stable Provider Text Edit Identity

Provider Try text/note scenes now author generic editIdentityArguments from only key,paramKey,windowId,windowKindId. Full editArguments remains unchanged, including initialValueJsonChunks. Blueprint/graph source continues to use its existing stable full target arguments. Generic renderer has no Forms-specific baseline-key exclusions. Added a neutral expected identity vector to existing embedded-flow fixture, and extended the existing native flow-note law to verify actual scene identity stays exact while two baseline payloads differ; dispatch args retain both baseline arrays. No Rust file, queue or dependency was added.

Added focused TypeScript law over the same neutral provider identity: independent Ajv2020 validates actual generic settings schema, semantic route identity is invariant under baseline/label changes, another Fill window changes identity, and actual mapper dispatch retains each full baseline. First run exposed the test oracle's draft mismatch (Ajv default draft7 could not load2020-12), corrected by existing Ajv2020 implementation. Exact geometry/schema command rerun:7 passed,0 failed,64 assertions,232ms. Native settings law remains pending the provider native owner. Provider production source is stable; root88782 must terminate before sequential geometry retry.

## Sequential Native Geometry Retry 19689

After root confirmed Forms batch88782 terminal before compiler on editor aggregate Source byte budget, started one original declared route: repo run @semio-tech/playbook-extension-procedural-rust:geometry-contract. Retained session19689; log🗑️generated/embedded-flow-native-current.log. Initial authoritative retained poll is live in outer run capacity admission; no provider compiler or assertion has run yet. No second provider command or direct Cargo bypass was started. Integration owns compact shared TextEditor recipe/consumer changes for the unrelated Forms Source budget refusal; provider source remains stable.

## Read-Only Native Settings And Scene API Review

Reviewed latest provider source against actual dependency definitions while19689 remains queued. Confirmed ModuleTestApp binds the same fixture instance identity used by dispatch/settlement; settle_registered_typed_operation accepts the supplied receiver/policy/authority/caller; fixture tree projection emits exact key/component/children and serializes text dataAttributes; paged carrier keys and ascending padded attribute names agree with the cancellation/rerun lane extractor. Mesh data is intentionally a JSON string and parsed independently before bounds assertions. UiValue::try_text_chunks and TextEditorScene settings fields match current calls.

Concrete potential compiler error found in the new identity helper: Map is pack_json::Object, whose insert key argument is impl Into<String>. Calling it with key.into() or stringliteral.into() leaves the intermediate target type unconstrained. Passing the &str directly resolves this E0283 inference pattern without behavior changes. Root notified before any source mutation under the read-only review request. No additional compiler process started; retained19689 owns eventual actual evidence.

Root authorized the concrete inference fix before compiler admission. Corrected only 👁️preview-and-params-body/🦀️.rs:248 and:263 to pass key and editIdentityArguments string directly into Object::insert. Related geometry/schema oracle rerun passed7 tests,64 assertions,283ms; native compile evidence remains pending19689. No second Rust command.

## Canonical Provider Source Inventory And Fixture Formatting

Used the public readRustSourceClosureV1 interface with the actual provider Cargo manifest directory, entry📦️packages/🦀️rust/🦀️.rs, native macOS/aarch64 host cfg and separate production/test cfg. No compiler or admission grant was invoked. The only current provider native package is semio-s-plugin-playbook-procedural.

Initial inventory exposed an actual test-owner admission gap:263884 bytes,27 files,2905 active lines against262144/32/4000. Production was127980 bytes,15 files,2088 active lines. The embedded-flow fixture occupied47312 bytes, with only20368 bytes of parsed compact JSON values, so removed redundant outer JSON indentation while retaining readable top-level and individual case rows. No field, schema, recipe, assertion or consumer shape changed. All998 parsed string values retain exact UTF8 bytes; authored graph source strings were never parsed/minified internally.

Fixture source hash before:edce6363f4ea00d89999a39f76cc65745406a24c2052fe7fcd53794f270acb26; after:4eff433ad90686f93bf692ea9dfc4f7cad4d54c31496089bd02db218cb0be5ce. Source extent47312→20579 bytes. Canonical parsed JSON SHA256 is unchanged:8d72fa0d6b8683419e4168f0b4b8e5d6fe60b119e1468ee916a7d0568dd6147c. Digest over all string paths, UTF8 extents and SHA256 values is unchanged:eeedafd89f61bcb6cd2f5f54b7957a7b09dd35c544f5c8efa11de9e2dd3f1ee7.

Final exact inventories:

| Owner | Bytes /262144 | Files /32 | Active lines /4000 |
| --- | --- | --- | --- |
| Provider production lib |127980 |15 |2088 |
| Provider lib-test |237151 |27 |2905 |

All three boundaries pass both owners; test byte headroom24993. Full input inventories and formatting receipt are under ticket🗑️generated pending final cleanup. Focused geometry/schema oracle rerun passed7 tests,64 assertions,286ms. Retained native19689 remained LIVE after formatting and inventory; no duplicate owner was launched. Browser/root workers notified that parsed fixture shape and source strings are unchanged.

Cross-platform diagnostic selection repeated through the same canonical source reader (not a compiler):

| Explicit cfg tuple | Production bytes/files/activeLines | Test bytes/files/activeLines |
| --- | --- | --- |
| unix, target_os=macos, target_arch=aarch64, target_family=unix, target_pointer_width=64, target_vendor=apple, target_env=empty |127980/15/2088 |237151/27/2905 |
| unix, target_os=linux, target_arch=x86_64, target_family=unix, target_pointer_width=64, target_vendor=unknown, target_env=gnu |127980/15/2088 |237151/27/2905 |
| windows, target_os=windows, target_arch=x86_64, target_family=windows, target_pointer_width=64, target_vendor=pc, target_env=msvc |127980/15/2088 |237151/27/2905 |

Each test selection adds cfg(test); this package declares no own feature flags. All six source selections pass262144B/32files/4000activeLines. These are explicit cfg inventory diagnostics only; no Linux, Windows or macOS compile success is claimed.

## Native Handle Recovery Boundary

Native19689 no longer has a recoverable owning-namespace handle after the agent finish/resume boundary. Its log ends in outer capacity wait2197 seconds with no compiler/admission/test output and no exit receipt. A read-only process inventory found no matching geometry command. No cancel or restart was issued. See provider-session-recovery.md for exact observations. Root controls any fresh sequential retry; no native result is claimed.

## Execution Support Custody Review

After locale work stabilized, re-read provider ownership and close paths without edits or compiler launches. Live evaluation owns exactly one host close slot, one building cache and one prior completed cache awaiting controlled retirement; publication transfers the new cache into the instance and retires the previous cache in the operation close ladder. Cancellation closes an in-flight tessellation using its original operation/generation and caller StepContext before host/cache retirement. Instance shutdown closes the preview, host template, registry retirement and session in order. Current geometry retention preserves imported handles alongside the current preview handles. No new concrete source mismatch was found in this review.

The missing19689 handle is not being reused or restarted. Root owns the next Forms native handle and will provide its log/terminal boundary; provider geometry can be retried only afterward. Any future provider live process must remain in the same active agent turn until its authoritative terminal receipt, rather than finishing the owning turn to free a review slot.

## Actual Forms Native Prerequisite Failure And Current Outcome Review

Root's Forms workspace run66076 reached actual Cargo compilation and then failed with Cargo101: neural budgeted evaluation treated OperatorJob::close_step as Result, although the current return type is RetainedWorkOutcome{result,work}. The compiler reported E0599/E0277 at the normal job-close site and E0599/E0308 at the cancellation-close site. Root owns authoritative process termination; no Forms assertion or provider compile success is inferred.

Root delegated this exact prerequisite fix. Before any edit, the authoritative file had already been changed by a concurrent writer: 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⏱️evaluation/🦀️.rs now records turn.work and uses turn.result at both sites. Normal evaluation debits the recorded CPU count even for a failed close. Cancellation exposes last_turn_work; EvalRetirement and adjacent Flow source/pipeline callers retain that work alongside native progress on successful or failing closure. Reviewed the actual RetainedWorkOutcome contract and preserved the concurrent implementation; no shared source edit was made by this worker.

Existing neutral regression command: repo tools exec --light bun -- test './🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⏱️evaluation/🧪️tests/🟦️.ts'. Result1 passed,47 assertions,215ms, using Graphlib/JSONPatch and Ajv. This proves the shared evaluation-value corpus remains consistent; it is not direct native proof of the new close-work error paths. No additional Cargo owner was started.

A separate current API review found the provider's tessellate/cancel double-? calls will need owned GeometryWorkFailure admission: that type now implements OriginalDomainFailure, with no From conversion into OriginalTurnFailure. Dropping its original work/domain receipt into a string or bare ValueError would be lossy. Root was notified; this finding is separate from the neural fix and remains pending the necessary provider adaptation.

## Funded Retirement Failure Admission Boundary

Confirmed no existing canonical cancellation-admission route: the three reservation execution methods reject actual cancellation, and OriginalFailureContext has no authenticated retirement phase observation. Saved provider-domain-failure-design.md with the exact four proposed shared production files, explicit retirement-only preparation/body/publication routes, provider-owned reservation lifecycle, and native proof requirements. Shared production edits are held while root93510 is live.

Draft provider-domain-failure-law.json and matching schema contain10 admission vectors covering normal cancellation/deadline refusal, genuine funded retirement despite cancellation/deadline expiry, missing permission, insufficient work/items, and foreign caller identity. Independent Ajv plus JSONPatch movement oracle passed all10 and exact original failure/work/receipt object identity. These are authored design inputs and an independent expectation oracle, not a production or native test pass. No provider/shared production file changed for this design stage.
