# Supported Thesis Rendering Ingress

## Ordinary Authored Script Ingress

The workspace registry's “Run an authored script” tool is not an independent issuer. Its exact declared command is repo exec --projects={project} --dry-run={dry-run} -- repo {workspace}/{script}. See root 📋️project.json, metadata.semio.dashboard.tools[id=run-script], and the root README authored-operation table.

The concrete Print command, preserving the full current rendering matrix, is:

~~~sh
SEMIO_LANGUAGE=en repo --tool=semio exec --projects=@semio-tech/print --parallel=1 -- repo /home/deck/semio/🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📜️script.ts test thesis-render
~~~

The backend injects an uncached repo-exec task rooted at the selected project. semioTaskBaseEnvironmentV1 supplies its project identity; issueBuildTaskEnvironmentV1 supplies genuine task authority. Selection is the literal Print owner and its closure, rather than intentionally all owners. However every ordinary exec still reaches loadBuildInvocationGraphV1 before backend execution. loadBuildGraphSliceV1 explicitly requires a usable sealed generation as the source of the selected graph; if none is usable, generation construction happens first. Consequently this supported script entry cannot promise to avoid the currently occupied graph builder slot.

Sources:

- repo library/build/🧭️selection/🟦️.ts:103: exec inserts the actual command as repo-exec with cache false.
- repo library/build/🕸️graph/🚪️admission/🟦️.ts: buildInvocationGraphSelectionV1 selects literal owners for exec.
- repo library/build/⌨️command/⚙️runtime/🟦️.ts:107-120: every ordinary backend invocation loads and pins its graph first.
- repo library/build/🕸️graph/🟦️.ts:744: a selected slice starts from a sealed generation and only then verifies or refreshes its selected owner closure.

## Routes That Do Not Supply Print Authority

The launcher’s direct script route merely hands the script to its pinned runtime (launcher Go entry Host.Execute, lines385-458); it does not issue a Script process Original. Print’s authored script immediately calls receiveScriptProcessInvocation, whose implementation rejects a missing actual parent envelope (“Original parent Script process invocation required”). Therefore repo /absolute/print/script.ts test thesis-render is valid only beneath an already issuing parent, such as repo exec.

repo tools exec has independent registered-tool capacity admission but does not create a Script Original or native Print storage capabilities. Removing --light changes capacity accounting, not Original issuance. It cannot directly run the authored Print script with valid authority.

The officially supported --graph-policy=current can serve a usable sealed generation without verification (generation loader text says exactly this). It is therefore a changed freshness policy, not equivalent evidence that fresh modified declarations have passed ordinary input admission. No current-policy or environment override was executed during this research.

## Sanctioned Graph-Free Test Birth Interface

A separate first-party test API explicitly supports bare test files with genuinely issued native build grants:

1. process/testing/invocation declares a closed BuildTestEnvelopeV1 via declareBuildTestEnvelopeV1. Its schema permits grant kind native-build-caller with authored command arguments and sandbox/checkout scratch.
2. repo library/testing/native-birth registers the real native-build-caller issuer. Its prepare method supplies the official repository providers, workspace identity and test sandbox. Its capabilities method calls createBuildBootstrapCapabilitiesV1 using the declared command; its matches method verifies genuine inherited grants when an actual parent exists.
3. process/testing/invocation withBuildTestInvocationV1 receives an actual parent or, for a declared bare test, issues that declared birth through the registered issuer. The API retains issued callbacks and cancellation through settlement.
4. createBuildBootstrapCapabilitiesV1 parses/adjudicates the authored command and creates native resource capabilities. Its inspected implementation does not load the build graph. Print withPrintExecutionV1 accepts the issued phase and reads that genuine native caller authority before opening its declared stores.

This is an existing supported test issuer, not handwritten capabilities, a manufactured wire environment or replacement runtime code. Existing library test files use the wrapper declareLibraryBuildTestEnvelopeV1 so that bare bun test gives the same grant truth as the suite runner.

The current wrapper only names repo-lib suite targets. A durable Print test entry would need to declare its own authored generic test envelope using the registered real issuer (arguments run,@semio-tech/print:test-thesis-render), then invoke the unchanged verifyThesisRendering function through withBuildTestInvocationV1 and withPrintExecutionV1. No such Print entry was added or run in this read-only research. A general exported native test-declaration wrapper would be cleaner than importing a library-specific wrapper solely for its issuer-registration side effect.

After such an entry is authored and reviewed, the legitimate registered-tool command would be repo tools exec bun -- test ./<durable-print-rendering-test-entry> (no --light), which retains machine admission and normal compiler/storage controls. It would not ask the global graph planner to execute a synthetic task.

Sources:

- process/🧪️testing/📥️invocation/🟦️.ts:23-84: grant schema, issuer registration, declared birth, genuine inheritance matching and retained callbacks.
- process/🧪️testing/📥️invocation/🧬️schema/🪪️envelope.json: native-build-caller arguments and scratch declarations.
- repo library/🧪️testing/🪪️native-birth/🟦️.ts:18-63: official environment, capabilities issuer, matches and library-specific declaration wrapper.
- repo library/build/📥️caller/📤️child/🟦️.ts:11-46: genuine bootstrap capabilities without a graph load.
- print/📦️artifacts/🎛️operation/🟦️.ts:27-39: actual issued control and native caller authority required for Print execution.

## Compiler Prerequisites and Scope

The existing test-thesis-render target declares fonts, deps-tectonic, deps-tex and generate prerequisites. Direct exec or a bare declared test entry does not automatically execute that target's prerequisite graph. They must use the already prepared owned stores or call the corresponding ordinary preparation interfaces under the same genuine issued invocation before running the complete matrix. Missing prerequisites must produce ordinary owned preparation/refusal, not manually written receipts, copied cache state or alternate tool libraries.

Research was read-only except this ticket report. The existing root native build remains live; no duplicate heavy process, graph-policy change, capability mutation or compiler launch was made.

## Durable Entry Implementation and Readiness

The supported test ingress is now implemented, using only the existing official native issuer registration and generic declared test envelope. Print contains no createScriptProcessEnvelope, capability constructor, custom issuer registration or wire fabrication. Both entries assert that the genuinely issued native arguments equal repo followed by the authored grant, and that native workspace authority equals the discovered checkout. The rendering grant names run @semio-tech/print:test-thesis-render; the publication grant names run-many of the existing build-thesis and build-phd targets in Print, parallel one. These are grants for direct production API tests, not evidence that the build backend executed those graph targets.

The one-hour Original deadline and each heavy Bun callback timeout are both declared as3600000 milliseconds in shared JSON. Issued child controls inherit that finite deadline; no deadline is disabled. Rendering prepares materials in a document operation, closes it, then calls the unchanged complete matrix under the same probe operation kind as the standard router. Publication uses a document operation. Both prepare pinned tectonic/tex/fonts through preparePrintMaterialV1, and run the production token generator as an actual owned child beneath the genuine Original. Publication invokes buildRegisteredPrintTemplate for thesis/phd, retains withPublishedPrintDocumentV1 readers, validates exact light/dark artifact rosters and signatures, and independently consumes every actual PDF through inspectPrintPdfV1.

Commands, sequentially after the root scheduling decision:

~~~sh
SEMIO_LANGUAGE=en SEMIO_BUILD_TOOL=semio SEMIO_TEST_ARTIFACT_DIR='/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/EXTEND-PRINT-PORTFOLIO-WITH-THESIS/🗑️generated/publication' repo tools exec bun -- test './🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/📤️publication/🚪️entry/🟦️.ts'
SEMIO_LANGUAGE=en SEMIO_BUILD_TOOL=semio SEMIO_TEST_ARTIFACT_DIR='/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/EXTEND-PRINT-PORTFOLIO-WITH-THESIS/🗑️generated/rendering' repo tools exec bun -- test './🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🖨️rendering/🚪️entry/🟦️.ts'
~~~

No --light is used for these heavy commands. The Bun registry declares capacity mode inherit, operation command, kind script. Normal heavy tool execution activates the supported host gate; command has a default2048MB/oneCPU reservation, with ordinary measured admission and process-tree cancellation. All native compiler/PDF/storage effects remain behind the production issued interfaces. Root retains sole-heavy-job scheduling; neither full entry has run yet.

Cheap evidence:

- Each entry's name-filtered genuine birth test passed1 test with the actual native issuer, matching exact arguments and checkout authority; the heavy callback was filtered out. These cheap checks used --light because no compilation or PDF effect ran.
- Direct thesis contract suite36passed,116assertions, including AJV admission of the authored native declaration data and exact target scopes.
- Entry-first strict TypeScript check initially failed on the missing shared birth/publication modules; after implementing them and fixing a missing scriptInvocationEnvironment base-environment argument, the full strict entry closure passed with zero diagnostics.
- Independent audit confirmed the declared issuer/control boundary and found an English publication heading fixture mismatch, corrected to Master’s Thesis with apostrophe normalization.

New durable files:

- print/🧪️tests/🎓️thesis/🎟️birth/🟦️.ts
- print/🧪️tests/🎓️thesis/🖨️rendering/🚪️entry/🟦️.ts
- print/🧪️tests/🎓️thesis/📤️publication/🟦️.ts
- print/🧪️tests/🎓️thesis/📤️publication/🚪️entry/🟦️.ts

Updated shared files: thesis JSON fixture and thesis contract tests. Standard owner target execution remains separately unverified; direct issuer evidence will not be substituted for it.

Final current complete strict entry import closure (rendering entry, publication entry, shared fixture tests and every imported implementation) passed with zero diagnostics, exit0, session44899. This includes the final probe/document operation split, actual grant/workspace assertions, issuer smoke checks and publication NFC/apostrophe matcher. Root canceled its graph-waiting native job and launched the normal-capacity publication entry as sole heavy job (session70935); actual publication/compiler results are pending and not claimed here.

## Actual Live-Store Fence Refusal

The normal-capacity publication entry (root session61662, explicit1024MB wrapper) admitted the genuine birth but terminated before any compiler effect. Opening the Print document operation reached the repository test fence: “A test reached live repository store checkout /home/deck/semio; give privateRepositoryWorkspaceV1.” The stack was requireMaterializedPath → repositoryCacheBudgetV1 → withRepositoryCacheStoreV1 → publication entry. The fence remains enabled and no capabilities were rewritten.

Read-only investigation confirms that privateRepositoryWorkspaceV1 is the supported isolated store facility. It creates a short managed private root, publishes SEMIO_PRIVATE_REPOSITORY and retires actual partitions through normal protocols. Its invoke helper uses fixed show/projects/nx arguments, so it cannot replace the exact declared Print grant. The existing native issuer must instead declare its genuine birth while private.within pins the working directory, after the private checkout markers and required Source inputs exist.

Print currently resolves font/library/template inputs relative to its genuine effect workspace and exposes no separate readonly Source root. Existing staging rejects escape through symlinks, and material readers reject hardlinks/symlinks. A legitimate private fixture therefore needs exact physical snapshots of the required domain input assets under the short managed root, with pre/post SHA256 checks of live originals. No TypeScript/runtime import closure needs copying. No checked source may be copied into this ticket. The ticket path is too deep for the private factory's256-unit precheck, so source snapshots must use the factory's existing short managed location or a separately approved domain projection. Nothing has been copied yet.

Token preparation must also avoid a private path to an uncopied JS generator: its existing public pure renderer can derive a private fixture stylesheet and compare the live canonical bytes without writing the live checkout. Before any heavy retry, a cheap genuine-private-birth test must open and close an actual isolated store under the declared private native authority.

## Managed Private Source and Store Implementation

Root approved the standard short framework-managed privateRepositoryWorkspaceV1 allocation as an owned runtime test resource, not ad-hoc agent scratch. Checked Source input snapshots exist only in that managed root and are automatically retired by the existing API. Custom scripts, logs, manifests and exported previews remain under ticket generated output; no checked Source was copied into the ticket.

The new thesis/workspace helper materializes the exact registered shared library, both registered documents and four font inputs: the actual first run captured188files and23301519original bytes. It rejects links, bounds file counts/bytes/paths and records original plus consumed SHA256 values. Catalog/font catalog and token renderer identities are provenance inputs. The private token stylesheet is derived through the actual pure production renderer, with its consumed hash distinct from the live original. Every original hash and roster and every consumed private file is rechecked after the test. Private metadata is an authored minimal fixture; no JS/TS runtime import tree is copied.

The artifact directory is made absolute against the real checkout before any chdir. Official native declaration and every withBuildTestInvocation callback run inside private.within; authority assertions require the genuine private root. No capabilities are rewritten and private.invoke is not substituted for the declared target grant.

The first actual private store smoke passed snapshot and authority validation, then the genuine dependency producer refused an empty authored Bun lock under frozen install. A real Bun1.4.2 oracle showed that an entirely empty workspace deletes its lock. The fixture now has one internal input workspace and zero external dependencies. The authored lockfileVersion2/configVersion1 record exactly equals Bun's emitted lock, and a real frozen install succeeded. This is input fixture correction, not a fabricated ready receipt. The ordinary producer still builds and validates the private dependency generation.

The normal1024MB store-only retry session94393 completed exit0: one pass, one full compiler/PDF callback filtered out, zero failures;10.44seconds actual execution. It produced and published a genuine private dependency generation, opened and closed every actual Print document store, rechecked original and consumed Source hashes/rosters, and completed managed retirement. The live-store fence stayed enabled. No Tectonic compiler or PDF worker ran during the smoke. The current private entry strict closure passed with zero diagnostics after the separately owned dependency retirement fix; no compiler result is claimed.

Publication now exports only already validated private PDFs with private- prefixes and a canonicalWorkspaceProduct:false proof under the configured ticket output before private retirement. These are private test evidence, never canonical live checkout products. Standard CLI/live workspace publication remains a separate open gate.

## Actual Entry and Private Preview Proof

The first full private publication attempt session68497 reached material preparation and correctly refused the actual Linux target x86_64-unknown-linux-musl because the artifact layout schema excluded underscores. The core agent corrected the owned schema with seven tests and177assertions across all seven supported platform targets. No compiler ran in that failed attempt.

Publication now invokes the actual unchanged physical compilation script as an owned child under the same genuine private Original. Preparation closes before the child starts, and the read-only PDF operation opens only after all build children finish, avoiding nested same-store custody. The reviewed real entry now derives its effect root solely from actual native caller authority. The proof records buildIngress actual-original-root-entry; this proves that physical script route against private products, while standard graph/backend/live-workspace publication remains separately open.

Publication assertion version2 checks worked title/author/qualification, chapter/appendix text, actual bibliography titles and resolved citation keys, plus invalid glyph/unresolved-reference markers. Each parsed PDF and bounded page-text snapshot is exported for diagnosis; the aggregate proof remains incomplete until all assertions and previews pass. The independent owned raster interface produces cover and first Arabic-numbered body page PNGs at scale2 for each of four theme PDFs, with actual page/dimensions/SHA256 recorded. All exports are private-prefixed and canonicalWorkspaceProduct remains false.

The exact final strict import closure session1572 passed exit0 with zero diagnostics after the actual-child integration, preview/text export and diagnostic ordering changes. Independent audit reviewed the child control/cwd lineage, store phases, content assertions, raster page selection and private-only export distinction without blockers. Root launched sole full publication job77983 under the supported normal1024MB wrapper. Actual compiler/PDF results are pending.

### Actual compilation diagnostic recovery

Publication run 77983 terminated with status 1 after the private store smoke and all pinned Tectonic, TeX and font preparation succeeded. The actual compilation entry launched Tectonic, whose first failure was not visible after managed compiler cleanup. This is a compiler failure awaiting its concrete transcript, not a successful publication.

The publication transport now uses separated bounded owned capture, with byte/line ceilings no larger than the genuine caller transport (at most 16 MiB and 65,536 lines). It retains actual native status and signal in a typed primary immediately, writes the bounded separated transcript to ticket-generated actual-thesis-entry.log or actual-phd-entry.log, and preserves that exact primary if diagnostic persistence fails. Capture cancellation/overflow propagate; genuine control cancellation also takes precedence during persistence. No stdout/stderr forwarding adds an asynchronous failure path. The production compiler separately preserves a bounded diagnostic tail before its existing refusal cleanup.

The failed private root r1bEEdD remains subject to the framework's official stale-root recovery. The next fresh publication fixture invokes privateRepositoryWorkspaceV1 before declaring its test Original; its existing dead-owner reaper performs storage retirement through genuine maintenance authority. No raw deletion or failed-Original bypass is used. If cleanup remains necessary at completion, the sanctioned smoke is a fresh Bun test using only the same official factory and normal lifecycle, with no failing declared Original. No separate cleanup job is scheduled while the next full publication is pending.

Final strict TypeScript import-closure check after bounded child persistence and production async-safe diagnostic changes: terminal 0, zero diagnostics (thesis contract test, rendering native entry and publication native entry roots). Full publication rerun is owned and scheduled by the root agent; no additional heavy process was started here.

After the actual KOMA dedication collision was corrected to makededication, the exact strict thesis/rendering/publication native-entry import closure passed again with zero diagnostics (session61161, terminal0). Core reports the new neutral command-ownership regression was red before the fix and the full thesis contract suite is now37pass/121assertions. Next actual publication remains root-owned.

### Successful compilation and owned recovery

Full publication5888 built both actual compilation entries, thesis and phd, including all four theme PDFs. Independent PDF inspection then rejected non-plain inspection DATA; no semantic PDF proof completed. The failed private root rIlIoQ1 is retained for official adopted-reader recovery. Ticket input recover-publication.ts adopts that exact root through privateRepositoryWorkspaceV1, asserts owned=false, issues genuine existing publication authority within the private workspace, rechecks original/consumed input hashes, and reads products only through owned document/artifact/material APIs. It exports bounded PDFs and SHA proof without compiler or inspection. Adoption does not transfer retirement ownership; a subsequent ordinary fresh fixture uses the existing dead-owner reaper. Normal publication now exports each owned PDF before inspection so future adapter failures retain it.

Reader-only inspection replay input inspect-publication.ts uses the same validated retained-root adoption and genuine publication birth. It verifies all four recovery export hashes and original/consumed input hashes, then invokes the existing full publication verification through owned artifact/scratch/PDF-worker APIs without compilation or material preparation. Root owns recovery35806 under a justified512MiB accounted estimate; no duplicate job was launched here. The retained root must remain adopted until replay concludes, after which ordinary fresh-fixture stale-root retirement remains the sanctioned cleanup route.

The inspection replay now imports the external pinned PDF.js oracle before entering any private working-directory scope. Its default inputs remain the authored Helvetica sample and all four SHA-verified recovered PDFs; optional command-line file arguments apply only when the oracle is its own entry. The raw strict-capture expected-failure assertion remains intact. The same accounted reader-only process then runs production owned verification, allowing external oracle and production adapter checks under one admission without compilation or material preparation.

After narrow showText Glyph normalization, thesis/adapter import closure26902 passed zero diagnostics. Explicitly adding the colocated PDF test suite in strict35149 found one new Bun expect.toEqual generic mismatch for the oracle expected glyph map (unknown values versus PrintPdfValueV1). Core was notified for a test-only typing correction before the final closure/replay. No heavy replay was launched on the failing closure.

Final exact strict closure51684 passed terminal0 with zero diagnostics after the oracle test-only deepStrictEqual correction. Roots explicitly include thesis contract, both native entries and the colocated PDF suite containing the actual pinned third-party Glyph oracle. Combined normal-budget reader-only replay is ready for root scheduling.

Reader71266 reached a worker ReferenceError because the worker entry is loaded by URL and was absent from the earlier TypeScript import closure. Core fixed the worker factory call and oracle data comparison. The permanent Print CheckScript type-check suites now explicitly include both the PDF worker script and its colocated tests. Final strict72721 explicitly rooted the worker, PDF suite, CheckScript router, thesis contract and both native entries: terminal0, zero diagnostics. This closes the static coverage gap; actual reader replay remains required.

### Completed actual owned PDF proof

Reader99370 passed terminal0:13tests/129expectations. Both pinned external oracle and production worker passed after the narrow operator adapter correction. The owned replay verified all four real products, semantic academic text, bibliography/resolved citations, and Arabic first-body numbering. Complete proof version2 records thesis32pages/body13 in both themes and phd24pages/body11 in both themes. Eight cover/body PNGs at1191×1684 were exported, with complete PDF/PNG SHA256 facts. This is complete private publication evidence (canonicalWorkspaceProduct=false), not canonical workspace publication. Full rendering matrix24299 and the separately core-owned ordinary canonical build remain pending; root/audit visual review is also separate. No additional job was launched by this agent.

### Semantic foreground correction awaiting rebuilt evidence

Independent visual review rejected light gray normal text on cream and cream text on teal dark-theme filled headings. The existing schema-authored mutedForeground and accentForeground are now emitted for Print and consumed for body/cover and brand-filled heading rungs. Outlined headings and the neutral panel rung retain theme foreground. Hyperref normal-size links/citations/URLs/files use theme foreground; palette colors were not edited. New neutral contrast vectors were red before missing role emission and then green, independently checked with existing colord a11y. Full direct thesis/contrast suite57pass196assertions. Matrix probes now log actual xcolor body/outline/filled/citation paints for comparison with neutral hex fixtures. Current sources require fresh compilation/inspection: earlier privatePDF proof is stale for visual acceptance. No heavy job launched here.

### Matrix fixture numbering and retained diagnostics

Matrix83411 compiled its first case, then failed the expected THESIS-MAIN-DEPTH=3 assertion. The original probe transcript was not exported before its callback failure and ordinary probe scratch retirement; its actual depth value is not claimed. Core source review established that the fixture set depth3 before document startup hooks which initialize the default unlimited depth. The fixture now explicitly sets its requested caller depth after begin{document}, before the title suspends numbering; production numbering is unchanged.

The main degree/language/theme callback now reads owned thesis.log and register-toc before any inspection/assertion and, when the explicit absolute ticket artifact directory is configured, exports bounded per-case log/ToC/PDF and marker facts through the held probe control, abortable writes and path256 checks. Numbering assertion messages include actual THESIS typeout values. This retains concrete evidence for future failures instead of requiring another compiler run to recover diagnostics. No compiler was launched by this agent.

### Canonical live-product inspection entry

Ticket verify.ts now reads both thesis and phd, light/dark, through genuine current-checkout Script authority and owned publication/material/scratch/PDF readers. It asserts the physical checkout equals the native authority repository root; it neither adopts a private root nor accesses cache paths directly. Strong assertions reuse the neutral publication expectations for academic text, bibliography, resolved citations and first Arabic body marker/footer; Roman frontmatter is also checked. Owned snapshots supply actual cover/body normal text, filled section and inline citation foregrounds. Graphics save/restore is tracked, actual page/section fills are observed, and contrast agrees with independent colord at or above4.5. It exports bounded PDF/text/paint diagnostics, eight cover/body rasters, hashes and a canonicalWorkspaceProduct=true manifest which becomes complete only after all four artifacts and rechecks pass. Output is fixed under ticket 🗑️generated/canonical-inspection. This is an independent ticket verification runner; the private test helper remains explicitly private and was not relabeled or modified during live builds.

Supported MCP invocation after canonical build success: exec with projects=[@semio-tech/print], command=[repo,/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/EXTEND-PRINT-PORTFOLIO-WITH-THESIS/verify.ts], language=en, tool=semio. CLI fallback is SEMIO_LANGUAGE=en repo --tool=semio exec --projects=@semio-tech/print -- repo followed by that absolute script path. Both routes issue normal repository task authority; direct bare repo tools/Bun execution would not issue it and is not used. The exact strict95370 closure, including this entry plus explicit worker, PDF suite, thesis/native entries and CheckScript, passed terminal0/zero diagnostics. No canonical inspection was launched here; canonical69241 and matrix87291 remain root-owned.

### Negative-only retained-workspace replay

Matrix87291 passed all17main degree/language/theme/phd probes, both required-only/Babel probes and both customization probes before its first negative classifier rejected the absence of one actual refusal report. Positive per-case evidence remains intact. Ticket input replay-refusals.ts adopts exact retained rsLAcRv through the official private factory (owned=false), issues the real rendering grant within that workspace and rechecks all recorded original/consumed hashes. It runs no preparation or positive cases. The current authored body is read as literal data from its single String.raw template (line endings normalized as JavaScript templates require), combined with the same semantic baseline metadata and the existing neutral refusal mutations. No checked source file is copied into the ticket.

The replay invokes the unchanged fixed refusal inspector entry with the same owned scratch, immutable request, digest, genuine child environment and caller/policy transport ceilings as the production inspector. Before invoking the unchanged classifier, it exports separated stdout/stderr/native status/signal, request/source hashes and before/after cancellation/remaining observations to the configured ticket-generated directory. Capture cancellation/overflow remain actual errors; no expected compiler failure is fabricated. Default replay count1 diagnoses the first failure; explicit SEMIO_THESIS_NEGATIVE_COUNT=6 retains all six negative cases for final coverage. No product edits or compiler launch occurred during preparation.

Independent negative replay review passed. Observation keys explicitly name parentOriginalCancelled, parentScratchCancelled and parentRemainingMilliseconds; the parent cannot directly observe the child compiler inner signal, and no such claim is inferred from these facts. Exact child native status/signal and separated streams remain independently captured.

Exact strict64796 passed terminal0/zero diagnostics after the narrow shared-reader settlement correction. Explicit roots cover the actual reader, new adjacent settlement helper and its neutral tests, Print worker/PDF suite, refusal module/entry, thesis/native entries and normal CheckScript. Core reports lifecycle mutation red then six-vector/AJV/SQLite focused58assertions and combined reader3pass81assertions. Same-root count6 replay is ready under root scheduling; temporary bounded diagnostic output remains until real confirmation, and will then be removed before final verification. No additional heavy job or fixture/source change was performed by this agent.

Exact strict49832 passed terminal0/zero diagnostics with the current exact-primary fanout terminal schema/helper and refusal vectors/entry explicitly rooted alongside shared reader/settlement/tests, Print worker/PDF suite, normal CheckScript and thesis/native entries. Core reports11focused tests/127assertions; independent audit passed. The unchanged same-root six-case replay is ready for root scheduling; bounded temporary diagnostics remain until actual object identity is confirmed. No heavy job was launched here.

Composed27scenario evidence and the six-vector shared-reader lifecycle file inventory are recorded in composed-acceptance.md. All six actual raw refusal captures pass the current no-DEBUG classifier independently; fresh original+consumed matrix snapshot hash checks match all current authored bytes. Canonical build/new visual proof remain pending.

### Current Direct Script Authority And One-Job Exec Fallback

Read-only current-source reinspection confirms a bare repo absolute-script route still does not issue a native Script Original. Launcher Host.Execute lines386–455 chooses the registered runtime and hands off the script with the runtime environment; its forwarding issuer hook is restricted to kind=build at line407. The canonical script calls receiveScriptProcessInvocation, whose actual-parent wire requirement remains unchanged. No bare script attempt, synthesized capability or special issuer was used.

The exec operation catalogue declares project selection/exclusion/dry-run/graph but does not declare a parallel flag for exec. The supported environment SEMIO_BUILD_PARALLEL is explicitly read by both buildCapacityAskV1 (host-capacity line57) and normal build selection (line125). A pure sizing-DATA evaluation with value1 produced preferredJobs1, fixed1536MiB and per-job2048MiB, nominal3584MiB; this does not grant capacity or override learned scaling/admission. The normal gate remains authoritative.

The root-owned MCP inspector334 expired at its300second outer limit before verify.ts started. Root/core verified its launcher/backend children621461/621492/621643 and thesis-owned leases/grants were gone before launching a replacement. Supported CLI fallback48868 is now the sole inspector: SEMIO_LANGUAGE=en SEMIO_BUILD_PARALLEL=1 repo --tool=semio exec --projects=@semio-tech/print -- repo followed by the absolute ticket verify.ts path. Output is retained in 🗑️generated/canonical-inspection/run.log. This route preserves genuine native workspace/task/control authority and normal graph/source validation. No duplicate job or direct-script shortcut was launched by this agent; actual inspection result remains pending.

Inspection 48868 completed, then actual visual review identified mixed prefix/leaf number ink on filled section chips. Current-source acceptance is reopened. New neutral filledHeadings data and a shared owned paint-run/numeral validator reject all six heading instances in the four retained actual PDF paint reports. Full current declaration/contrast/paint tests pass 84/282; fresh canonical build and stronger inspection remain pending. See heading-number-contrast.md. No compiler was launched by this agent.

The prior canonical-inspection outputs, including the numeral regression JSON and all four old paint reports, are now archived in generated/canonical-before-number. A clean canonical-inspection directory is reserved for the final rebuilt proof. Inventory now includes the four shared paint helper/schema/corpus/test sources plus heading-number-contrast.md and the core-owned heading-number-strict.ts: 64 source + 58 retained ticket paths = 122 close paths. Old 27-scenario source hashes are explicitly an earlier checkpoint and do not assert equality with the final changed core/window bytes.
