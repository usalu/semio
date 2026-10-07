# Shared Input Refusal Execution — 2026-10-06

## Confirmed Defect

The React retained interpreter treated every non-applied input completion as a recoverable conflict. A typed `timeTravel.frozen` refusal therefore preserved the refused draft even though the history notice said that the previous value was kept.

The language-neutral disposition corpus covers text, multiline text, number and date, plus a newer draft arriving before the refused completion and ordinary recoverable refusal. Ajv validates the fixture schema; React Testing Library exercises rendered controls and the DOM accessibility oracle verifies authored field names.

## Test-Driven Evidence

The initial renderer test run executed through `SEMIO_TEST_LEVEL=long bun nx run @semio-tech/framework-renderer-react:test --skipNxCache --testNamePattern=… ♿️editable-controls`. It failed the four typed-control restoration cases; three other selected cases passed. The four failures showed the refused draft instead of the published value. A shared lazy-field lifecycle run reached Vitest but exceeded the fundamental 15-second process budget before execution results; it is being retried using the existing long budget.

## Implemented Input Restoration

Input terminal outcomes now carry the explicit optional draft disposition `discard` or `retain`. The shell maps recognized history refusal codes, including nested fault causes, to `discard`; other failures remain recoverable. The interpreter discards only the submitted draft that still matches, preserves newer edits and suppresses stale completion changes after cancellation or owner replacement.

Shared Input and Textarea now compose consumer focus, blur and key callbacks with their internal lifecycle. Escape suppresses its blur commit, and Enter commits exactly once. Keyboard input-method composition is left intact. Their regression tests were executed red first: all three callback/commit cases failed; the fixture validation passed.

## Validation Status

- Renderer retained input, input ledger and fault notice suites: **156 tests passed across three files** through the existing Nx test target and long budget.
- Shared Input/Textarea and Tree suites: **67 tests passed across three files** through the existing UI Nx test target and long budget.
- Language-neutral typed dispositions explicitly include cancellation before a late refusal: the later draft remains visible without conflict state.
- Shared UI typecheck completed green; Slider/Input/translation-totality completed **32/32**.
- Final renderer retained input/input ledger/fault notice/drawing-choice run completed **177/177 across four files**.
- No live-browser claim is made in this report; root owns full editor E2E execution.

## Slider Accessibility Work

The audited exact-value readout was a non-focusable button role with only double-click activation; the slider thumb suppressed its focus ring. A language-neutral exact-entry corpus and Ajv/Testing Library/accessibility-oracle tests now cover Enter, F2, Space, exact commits, disabled/read-only controls and returned keyboard focus. The red run failed all three keyboard opening cases. The slider now supports Enter, F2 and Space exact entry, names the numeric editor, shows a thumb focus ring and returns focus to its readout after Enter/Escape. English/German hints describe keyboard activation. Final combined Slider/Input/translation totality run passed **32 tests across three files**. The run additionally exposed missing exports in the existing i18n port import list; those imports were repaired.


## Drawing Mutation Choices

Drawing layer storage intentionally preserves literal blend identifiers (including custom owner-defined strings), as confirmed by existing storage tests. Runtime mutation and scene admission already reject unsupported blend values. The create-layer mutation schema currently reuses the storage layer schema without constraining its input choices; its `layer.kind` and `layer.blendMode` therefore become free text controls.

The clean correction is to constrain the create-layer mutation's layer input properties while composing the existing storage layer schema with `allOf`. The controlled mutation input gains enum choices and bilingual option labels, while stored literal snapshots preserve their established ownership semantics. The generic manifest readers already support this schema composition and prefer the mutation input's own declared properties. The corrected red run failed both en/de schema-derived control checks: `text` was produced where `select` was expected; the neutral fixture validation passed. The create-layer mutation schema now declares both controlled choices with bilingual option labels. Full renderer verification passed **177/177**. The native parity wrapper reached Cargo but failed on five unrelated kernel compilation errors before the drawing test executed: missing `indexed_edits`/`mutation_positions` and missing `BorrowedDslField` group anchor/member traits. The core owner was informed; no native Drawing pass is claimed.

## Type Checking

The first shared UI typecheck reached TypeScript and exposed missing existing i18n port imports plus an import of the removed central fixture-ownership schema in the Layout test. The i18n import repair is complete and translation-totality tests pass. Root completed the Layout fixture ownership migration; shared UI typecheck now passes. Renderer typecheck reaches TypeScript but remains blocked by unrelated removed fixture/schema imports in TextEditor, MediaTransportHost, SpaceBrowser, UiDocumentStore and HubSignIn tests. Owned typed-label and option-label errors are repaired.

## Changed Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧫️fixtures/⌨️draft-lifecycle/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧬️schema/⌨️draft-lifecycle/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧪️tests/🧩️component/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧫️fixtures/⌨️exact-entry/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧬️schema/⌨️exact-entry/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️tests/🧩️component/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧪️fault-notices/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🚦️input-draft-disposition/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🚦️input-draft-disposition/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️editable-controls/🟦️.tsx`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/➕️create-layer/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/➕️create-layer/🧫️fixtures/🎛️input-choices/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/➕️create-layer/🧬️schema/🎛️input-choices/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/➕️create-layer/🧪️tests/➕️appends/🦀️.rs`

All test commands use existing Bun/Nx targets. No executable target or launch command was added. Temporary generated logs remain under the ticket's generated directory until their running commands finish; they will be removed after evidence is recorded here.

## Invocation-Scoped Cargo Inventory

A measured neutral 64-member fixture read/parsed its root manifest 64 times (320 aggregate parses). Per-invocation document and scope maps now reduce that inventory to 65 aggregate parses, with each manifest parsed once. Every subsequent call rebuilds admission and sees edits/removals; no global cache survives peer saves. The third-party TOML parser compares every admitted document.

Grouped package selection, lint/test package batches, and membership publication share one freshly admitted registry per invocation. Before correction, the real membership batch repeated a full repository discovery for each of 95 owners. The repaired real `members-check` completed **95 owners in 16.5 seconds**. Root scope contained 119 physically admitted members (~5.2 seconds); the editor scope contained 138 (~367 milliseconds), distinct from authored wildcard member counts. The complete Cargo contract passed **15 tests and 409 assertions**. Tools owns a subsequent package-scoped preparation closure repair and reports that extended contract green. Temporary inventory runtime traces have been removed.

## Native Input Completion Parity

Native input Enter retains its focused edit buffer while asynchronous dispatch settles. A separate receipt source now identifies addressed retained input submissions; 256 receipt slots have monotonically increasing tokens. Completion releases exactly one matching slot. Only explicit typed history refusal restores a submitted buffer, and only while surface generation, retained node generation/key, authored action, text and composition still match. Newer drafts, replacement owners, recoverable validation and cancellation retain their text. A discarded submission advances the presented interaction epoch and dirties candidate paint, ensuring the next frame adopts the restored text.

Native unit tests consume the same language-neutral input disposition corpus as React, plus capacity exhaustion/recycling and duplicate completion checks. The first native wrapper did not reach execution before its old broad preparation was cancelled. The fresh renderer integration wrapper completed successfully: **2/2 native tests passed** (1,684 unrelated tests skipped), covering the same seven-case disposition law and the bounded receipt generation/source/capacity/recycling fences. This proves native completion routing through Interpreter and renderer receipt settlement. Earlier wrapper failures were compiler/preparation blockers, so no executed native TDD red is claimed. An additional existing shared UI-engine target exercises the same visible-value law and checks that discarding a focused submission schedules paint. Its first executed attempt failed a fixture precondition: accepted presentation had moved the candidate tree into the presented owner. The fixture now performs the existing candidate reconcile before checking dirty flags and accepts the rebased candidate after restoration, matching the engine ownership protocol. Its corrected run is pending and does not replace the integration receipt test.

## Preview Preparation Progress

The shared corpus now covers Editing with preparation `done`/`total`, the canonical bilingual preparation count label and `timeTravel.unchanged`. Native refusal/label catalogues match those additive rows. React exposes preparation progress in the persistent band while Editing. Actual TDD red was **3 failed / 59 passed**, covering missing count text, visible progress and missing catalogue row; corrected run passed **62/62**. Accept remains governed by the canonical editor readiness rather than an invented refusal during preview work.

## Universal Retained Journey Reader

Step 24 previously returned `u-NOT-PROBED-on-this-renderer` before entering the shared journey on wgpu. It now inventories controls and Actions rail verbs from authored accessibility keys, drives keyboard value/text edits and accessible option/switch activations, unfolds the native rail and executes its staged forms, and reads native transient notices. Existing shared History/replay/finalize/row helpers remain the journey body.

A local schema and language-neutral inventory corpus cover eight role families, nested pointers, min/max/step, read-only controls, selected radios/list options, reference chip count and rail category admission. Actual TDD red was **1 failed / 1 passed**; final expanded inventory/module-import suite passed **4/4** against Ajv and the independent DOM accessibility oracle. This validates projection and source loading, not live control operations. The actual native universal journey requires the fresh wgpu serve and is not claimed passing. The shared body now performs both concrete outcomes: Overwrite, then a second clean input edit finalized as a named New alternative. It checks the named history row, current alternative and retained prior head; switching to the prior head restores its exact history, and switching back selects the new one. After this addition the existing inventory/module-import suite passed **4/4** again. These authored live assertions are not claimed executed.

The disposition schema additions were rechecked with the renderer editable-controls suite: **105/105 passed**. Shared UI typecheck after preparation catalogue additions passed. The repaired native renderer input command passed **2/2**; earlier completed attempts failed before renderer tests on four private durable history accumulator field accesses and a borrowed DSL temporary static-field lifetime. Core/tools repaired these shared-source blockers. The lower UI-engine law has reached native Cargo compilation.

## Manifest-Admitted Universal Serve Boot

A missing universal serve formerly started the hardcoded puzzle2d family. Universal boot now resolves only catalog-declared route plugins/aliases, an explicit validated `--variant`, or a unique renderer port assignment. Conflicting route/explicit selections, unknown plugins and an unclaimed port without variant are refused before startup. The native route is rewritten with its canonical admitted plugin. Specialized puzzle steps retain their declared puzzle2d default. Acceptance evidence records the admitted variant.

The schema-owned corpus uses the existing generated playground metadata (including Draw's declared React/native ports) and a JSDOM independent WHATWG URL oracle. Actual TDD red executed **4 failed / 4 passed**. The corrected combined boot and inventory/module-load suite passed **12/12**. This confirms admitted selection and source loading; runtime server startup and the live universal journey remain the root agent's ongoing checks.

The native receipt integration fixture compiler errors are repaired: typed records are constructed as owned JSON records, and replacement node identity comes through public `FocusChanged`. The lower engine test uses the owned `NodeFlags.set` API. The repaired fresh renderer receipt integration run passed **2/2**. The lower engine paint-law run remains pending; no success is inferred from source loading or compilation alone.

## Additional Changed Files

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-action-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-input-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧾️frame-action-ledger/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-renderer-standalone/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel/🗣️labels/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel/🚦️refusals/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧫️fixtures/📇️invocation-inventory/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧬️schema/📇️invocation-inventory/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🪞️inventory/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🪞️inventory/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🪞️inventory/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🪞️inventory/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🚀️boot/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🚀️boot/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🚀️boot/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🚀️boot/🧫️fixtures/🔣️.json`

## Explicit Actor Constructor Callers — Read-Only Admission Audit

Core's next constructor law requires `protocol::ActorId` for ArtifactStore and retained initialization runtimes, with typed identity setters and no optional actor. A source search found no direct constructors or local-actor setters under renderer/dev. Native host owns one wrapper constructor, `OsWorkflowStore::new(document)`, delegating to `ArtifactStore::new(envelope)`; its host unit tests contain two direct artifact fixtures and four workflow fixture/reopen calls. The wrapper must require and forward the caller's actor. Birth/test contexts may explicitly admit `LOCAL_ACTOR_ID`; reopened contexts must preserve their actual actor identity. This is a read-only inventory: no constructor/API changes are claimed applied yet. Parent owns external app/plugin/space routes.


## Native Retained Input Paint and Shared Preparation Band

The lower UI engine's neutral seven-case draft-disposition test now passes **1/1**, with 786 other tests excluded. It confirms that an explicitly refused matching submission schedules paint and that the subsequent presented accessibility value returns to the accepted value; newer unsent drafts remain intact. Renderer receipt tests already pass **2/2**. These are executed native test laws, not a live all-editor UI claim.

The native shared host band corpus executed and failed **1/1** on the new Editing preparation progress row: `progress` was null instead of localized `Preparing history preview: 2 of 5 steps`. The host band now selects preparation labels for Editing, presents determinate progress geometry and accessibility counts, and requests frames while preview work remains. Changed Accept remains available according to the canonical panel predicate. The corrected run stopped before assertions on five core constructor/type errors during the ongoing explicit-actor API wave; its green status remains pending.

## Viewport Borrowed Typed Metadata

Viewport2d and Viewport3dOrbit now provide borrowed static Record metadata matching the existing schema-owned ordinary binding fields: planar x/y/zoom Float fields and orbit position/target/up fixed Float tuples plus zoom Float, with up optional. The new neutral fixture test checks the borrowed metadata against the same field law used by the independent Serde projection test. The initial preimplementation run failed to compile on four missing borrowed trait bounds, so no executed TDD red is claimed for that run. The next full run executed 7/8 successfully, with an overly strict test-only pointer-address identity assertion failing; Rust const promotion does not promise that identity. That assertion was replaced by structural validation. The final full native viewport suite passed **8/8** using `bun nx run @semio-tech/framework-ui-viewport-rs:test-native --skip-nx-cache -- long`.

Additional changed paths:

- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🪆️binding/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🪆️binding/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs`

## Required Host Actor Pass-Through

`OsWorkflowStore::new(document, actor: protocol::ActorId)` now forwards the actor directly to the kernel constructor. Named actor fixture cases cover reopen with the original actor and reopen with another explicitly opened actor. Both existing persistence reopens preserve the original store's actor; birth fixtures explicitly construct LOCAL_ACTOR_ID, and the concurrent peer fixture names peer B. The host task's existing test command validates the neutral fixture through Ajv and independently projects expected actor lists through JSON; the native law checks store identity and persisted edit attribution after a binary round-trip. The focused native run remains pending; the test was authored before the wrapper migration, and no executed red or green is claimed yet.

Additional changed paths:

- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️host-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🖥️host/🧬️schema/🧾️actor-context/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🖥️host/🧫️fixtures/🧾️actor-context/🔣️.json`

The host project named inputs now include its local JSON schema/fixture authority. The existing test-long command was used with `rust --features os-host-full workflow_host_preserves_the_opened_actor_context`; the initial unqualified invocation was stopped before execution because the host module test law is feature-gated. No new executable command or launch entry was added.


The current host native run executed the **two-case Ajv/JSON oracle successfully**, then stopped before native assertions on five kernel actor API sites: child member creation in store at 28315, retained configuration runtime construction/setter at 239/496, and hydration runtime construction/setter at 319/692. These remain core-owned; the old setter sites infer optional actors from the last applied edit. Exact diagnostics were sent to core for opened-actor authority propagation. No native host green is claimed.

The host project JSON cache input addition is in `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📋️project.json`. A repeated constructor/setter inventory over the actual renderer and dev paths returned no direct callers.


## Plugin Test Actor and Immutable Genesis Caller Wave

Parent assigned all framework plugin test callers except checkpoint tests to this lane. Production plugin root, builder/window modules, checkpoint module/tests, and time-travel/tool-run production remain parent-owned; Core owns Store/VCS fixtures, and Tools owns all s families.

Named fixture actors are now supplied at app construction, before the four stores and genesis. Former document-only actor setters are identity assertions, preserving fixture author intent across time-travel, folder/label reload, acceptance, and tool runs. Standalone local fixture births explicitly construct LOCAL_ACTOR_ID. Member creation uses the live parent's actor or the explicitly admitted composition-opening fixture actor. The custom composed initializer carries its actor, uses the cached canonical genesis.digest() in O(1), shares the immutable genesis snapshot, reads current projection through current_ref(), and adopts owned replay results with bounded shared-owner retirement. It no longer infers an actor from the last historical edit or mutates a shared immutable genesis snapshot.

The production builder declaration test now consumes Core's actor-genesis fixture, creates actor:alice, validates document/config/draft/interaction actor identity through an independent Serde projection, and reloads the document preserving that actor. The native reactor opening test consumes the same fixture, rejects missing and empty admitted actor authority, captures and ACKs real native open, performs bounded native document reload, closes exactly, and reopens with preserved actor. The existing lifetime getter test accepts the new Result-returning actor API. These new native assertions are authored but not yet executed; Core requested no further broad runs until the immutable-genesis source wave settles.

The two in-flight captures launched just before that instruction completed compile-red before native assertions on 14 parent-owned plugin production actor call sites. Exact checkpoint/window/tool-run/time-travel/builder/root diagnostics were sent to parent; no final actor/genesis green is claimed.

Additional caller/proof paths updated:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️folder-reload-route/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️tool-run-member/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️surface-view-state-routing/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️node-drag-history/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-label-reload/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs`

The remaining required new_app/new_viewer fixture calls now pass actors explicitly: the retained publication lane tests preserve their declared `fixture` metadata actor; Dummy catalogue preserves its declared `actor`; standalone local/meta(local) fixtures admit LOCAL_ACTOR_ID. The handcrafted composed raw envelope decode fixture now names authoritative `initialPack` hex. Renderer/dev/host contain no old initialSnapshot or vcs.initial_snapshot reads.

Additional caller paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎞️media-owner-context/🦀️.rs`


## Hub Test-Only Actor Callers

Parent additionally assigned all Hub test-only constructor/factory callers. Twelve test paths now supply required ActorId, including the multiline async stdio shipped-fleet helper and the shared Space app test context. Existing local/meta(local) standalone births explicitly admit LOCAL_ACTOR_ID; the socket SupersedeReplica opens with the actual actor obtained from its socket grant and verifies that identity instead of rebinding afterward. The GIS cold-map law compares output bytes against the stored genesis.pack(), and the Space example law borrows genesis.snapshot().collections. No test success is claimed for this new wave. A full Hub source inventory leaves only the three parent-owned inference production initialization constructors outside tests; no old initialSnapshot/vcs.initial_snapshot or optional actor setter remains in Hub tests.

Changed Hub test paths:

- `🌎️hub/🗿️artifact-authority/📌️check-in/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🧩️compositions/🌊️flow/🧪️tests/🔬️surface/🦀️.rs`
- `🌎️hub/🧩️compositions/📕️norm/🖥️app-surface/🧪️tests/🖥️app-surface/🦀️.rs`
- `🌎️hub/🧩️compositions/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs`
- `🌎️hub/🧩️compositions/🪐️space/🧪️tests/🔬️interactive-job-catalog/🦀️.rs`
- `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🎬️set-active-example/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs`
- `🌎️hub/🧩️compositions/➗️mathematical/🧪️tests/🔬️surface/🦀️.rs`
- `🌎️hub/🧩️compositions/🔋️energy/🧪️tests/🔬️surface/🦀️.rs`
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`


## Current Browser Activation and Bilingual Journey Audit

Audit performed on the current source after semantic script routing moved dev operations into their domain owners; no component or native builds launched during Core's unsettled actor/genesis wave.

Canonical authority is `plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts`, loaded by `plugin/🏗️build/📋️plan/🟦️.ts`. Puzzle variants resolve to composition plugin `puzzle`, crate `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust`:

| Variant | App | React Port | WGPU Browser Port | Aliases |
| --- | --- | --- | --- | --- |
| puzzle2d | s.puzzle.puzzle2d@1/*#editor | 6012 | 6112 | 2d, puzzle 2d |
| puzzle3d | s.puzzle.puzzle3d@1/*#editor | 6013 | 6113 | 3d, puzzle 3d |
| puzzle5d | s.puzzle.puzzle5d@1/*#editor | 6014 | 6114 | 5d, puzzle 5d |
| draw | s.draw.drawing@1/*#editor | 6064 | 6164 | none |

All puzzle variants declare the authored dev contribution `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle/🧬️schema/🔣️.json`. This selects its local Vite config/browser entry and declares `@semio-tech/puzzle-2d` browser session factory with the composition puzzle engine. Puzzle2d also lists that same engine explicitly in catalog; Nx preparation deduplicates declared engine roots through a Set. Puzzle3d/5d assets declare mesh collection and infinite assets. The session-generated app identity differs per variant; no guessed family or bare plugin substitution is needed.

Cold producer closures are generated by repo library `playgroundPreparationTargets`, not handwritten dev project entries. Canonical commands:

```sh
bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-react-dev
bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-wgpu-dev
```

The React producer closure includes session publication, browser support, fonts, flow WASM, declared puzzle session engine WASM, and app-scoped host component materialization. WGPU additionally includes renderer WASM, generated browser boot, frame worker, and renderer boot. Preparation verifies the session/variant/plugin identity, admitted healthy host component, browser support files, and fonts; activation publishes digests and the runtime receipt. Actual source prerequisites remain meaningful even if a prior receipt exists; the serve freshness pass reports staged source changes.

For a cold interactive launch, the generated `dev-puzzle2d-react-dev` and `dev-puzzle2d-wgpu-dev` targets include activation and then listener startup. Warm React `serve-puzzle2d-react-dev` intentionally depends only on session; WGPU serve still depends on activation. The executable launch seed is `.vscode/🧩️launch.seed.jsonc`, with puzzle devLauncher metadata (2d order220/220.1, 3d230/230.1, 5d250/250.1); generated `.vscode/launch.json` is the user-facing launch surface.

The canonical full journeys already exist in both seed and generated launch as `⚖️gate⏪️time-travel⚛️react` and `⚖️gate⏪️time-travel🧊️wgpu`. Use their existing commands with a ticket-owned output path:

```sh
bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6012/ --renderer react --locales en,de --chords en,de --out .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/full-react
bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6112/ --renderer wgpu --locales en,de --chords en,de --out .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/full-wgpu
```

No `--only` means steps1–24 plus reload; default long-history minimum200. EN/DE are both actual shell locale runs. `--chords en,de` exercises Accept/Discard/Exit keyboard chords; step11 also exercises buttons, so both gesture paths remain covered. `--universal` defaults to steps24 and9, which is a distinct all-editor law rather than the full specialized puzzle journey. The boot selector pins specialized journeys to puzzle2d and resolves universal variant by admitted explicit argument, route plugin/alias, or unique renderer port. It rewrites the universal URL to its admitted canonical variant.

Cold-start gap verified by source: the time-travel target itself has no activation dependency. `ensureDevServe` spawns raw React/WGPU serve scripts and does not run the producer closure. React ServeScript may publish a missing receipt but does not build; WGPU ServeScript consumes completed artifacts. Therefore a missing listener is zero-touch only after the producer closure has completed. Parent was notified; no claim that cold artifact construction is already zero-touch. The fixture reuses existing answering listeners, refuses canonical hub port7800/non-HTTP occupied ports, disables HMR on spawned listeners, reports progress every5s with300s boot bound, honors cancellation, and stops only its owned process. Reused listeners preserve their prior settings.

History control payload audit: `timeTravelControlActionV1` sends the session generation for every band action except nextProblem, which sends only mutationId and optional admitted store. The renderer wire is presentation-only HistoryPatch/HistoryTimeTravel: it carries neither genesis bytes nor birth actor authority. Core initialPack and required constructor ActorId therefore require no renderer payload field migration. Full renderer/dev source search found no `initialSnapshot`, `vcs.initial_snapshot`, or stale genesis serialization. Host construction forwards an explicit ActorId; renderer open/bootstrap fixtures retain the actor-lost reopen failure notice.

A current source audit found the component test named as schema validation had lost its actual Ajv validation after central UI schema removal. This lane restored direct canonical kernel HistoryTimeTravel and HistoryPatch checks for every neutral band session, plus invalid generation/stage and misplaced initialSnapshot/initialPack/actor rejection. It does not restore obsolete central UI schema. The focused current-source oracle was launched through workspace `bun nx exec`; result recorded below when complete. This is a validation repair, not a new TDD-red claim.

Focused current-source schema oracle stopped before Vitest execution: Nx graph cycle from mathematical edit-graph-config through dsl-derive, ui-locale, value, value-derive, then value. Command exit1, zero assertions executed; log `🗑️generated/ui-current-history-wire-schema.log`. Parent notified.

Additional owned source/documentation correction: `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` docstring now describes stored genesis.pack byte sharing instead of obsolete initial_snapshot re-encoding. Repeated plugin/Hub test source audit found no obsolete optional actor setter/getter or genesis field access; unchanged initial_snapshot trait births remain valid authored constructors. This adds one documentation-only Hub test file beyond the twelve caller files above.


## Cold Journey Producer Ownership Repair and Current Native Capture

Parent explicitly assigned the verified cold-start gap to this lane. The canonical journey now supplies `ensureDevServe.beforeSpawn` with the admitted variant's existing `activatePlaygroundRuntime` owner. The fixture calls it only after the port decision admits a new owned listener; active serves are reused and occupied/canonical-hub ports refuse before preparation. Activation delegates the same generated Nx owner target, preserving its cache and declared producer closure. The activation helper now awaits existing `runRepositoryCommand`, so the process emits bounded progress and follows the journey's AbortSignal on all supported platforms rather than blocking in a synchronous process call. No new executable command was introduced, so existing launch registrations remain authoritative.

Startup is schema-first in the existing boot fixture/schema. The actually executed red run reported4 failed/12 passed: both cold renderer starts omitted activation, activation failure still spawned a listener, and cancellation during activation still spawned a listener. After implementation the boot and universal inventory suites passed20/20 across2files. A second neutral race law caught2 failed/16 passed: a peer listener or occupied non-server appearing during a long activation was ignored. The provider now rechecks the port after preparation and reuses/refuses accordingly, then checks cancellation immediately before owning the new child. Race green result recorded when complete. Fixture validation uses Ajv; catalog/route selection uses the independent JSDOM URL oracle and Vitest asserts the expected neutral operation trace.

Updated files for cold producer ownership:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts`
- existing boot `🧫️fixtures/🔣️.json`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`

The earlier audit's cold-start gap is now source repaired; an actual cold browser journey remains a separate runtime proof and is parent-owned. Canonical full journey commands above can prepare an absent listener's artifacts themselves.

Focused renderer schema oracle successfully rerun through `@semio-tech/framework-renderer-react:test-long`:1/1pass,61skipped, canonical neutral band sessions validated as HistoryTimeTravel and HistoryPatch. The whole-workspace Nx exec cycle is avoided by explicitly selecting the existing workspace project for source oracles (`--projects=workspace`); no cycle-ignore or native gate bypass used.

Core announced stable actor/genesis source. Fresh production declaration factory proof actually passed1/1 (1053skipped), Nextest run3945e9bf-3bd2-4259-8237-dcaea670105f, log `🗑️generated/ui-plugin-production-factory-actor-final-wave.log`: all document/config/draft/interaction stores receive actorAlice before genesis and production document reload retains actorAlice. Fresh WGPU band suite stopped before assertions on one parent-owned FlowStore production constructor missing ActorId (`framework/os/modules/flow/host` line2430); parent notified. Fresh host actor and native-open proofs remain running at this report update.

Current broad plugin compile caught seven remaining test-only migrations; all are now corrected: TT fresh fold adopts the original app's typed actor; standalone interaction live/capture/query Store births explicitly admit LOCAL; both builder-contract bus constructors receive LOCAL; the codec refusal construction callback receives required ActorId. Additional owned fixture paths are plugin `🕹️interaction/📡️live/🧪️tests/📡️live/🦀️.rs`, `🕹️interaction/📖️capture/🧪️tests/📖️capture/🦀️.rs`, `🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs`, and `🧪️tests/🔬️app-declarations-fixture/🦀️.rs`. Existing TT/builder paths are already inventoried above. Every bind_actor call in owned tests checks the same actor supplied at construction. The plain emit author law now opens Grace's separately admitted instance from Ada's authoritative document before Grace dispatches; it no longer changes the acting user context of Ada's instance.

Native admitted opening/reload/reopen proof actually passed1/1 (1053skipped), Nextest9b400f12-f952-4604-91c4-825c61664ac8, log `🗑️generated/ui-plugin-native-open-actor-final-wave.log`. A race-green combined source run had all18 boot assertions pass but the inventory module-load assertion exceeded its5s test timer under concurrent native compilation (21pass/1timeout overall). Activation import is now cold-path lazy, and the journey import witness is a normal static test-module import rather than timing dependency transformation inside the assertion. No budget was raised; current combined rerun pending. Additional edited test file: dev/time-travel/inventory/tests/🟦️.ts (already listed earlier).


Final cold-start/source contract rerun actually passed22/22 across2files (10 startup traces, catalog selection laws, neutral inventory laws), log `🗑️generated/ui-cold-journey-final-source.log`. The inventory journey module is transformed during normal module collection; no test timeout policy was changed. The canonical journey places its spawned serve log inside the caller's `--out` directory, so this ticket's actual live runs keep owned logs under generated output.

Parent assigned a new current native unchanged Accept failure. Source trace found the parent-owned fixture projection `artifact_app_laws::project_fixture_node` omitted `BuiltNode.disabled` for every node: it serialized keys/components/bindings/accessibility/children only. Native button production already sets disabled from authoritative `editor.changed`, which is computed from the pending replacement and the authoritative starting replacement. Owned test now separately asserts initial pending is unchanged and native panel.changed=false before asserting actual disabled observation. Parent was asked to expose the true disabled flag in its fixture observer; no assertion weakened, and no speculative production availability change made. Focused current capture pending.

Fresh host actor capture ended before native assertions on ten current compilation diagnostics: four host-unit obsolete kernel::json values, two workflow-unit obsolete IoError.message fields, three parent-owned generic space import missing Sync errors, and host create_backbone_document missing genesis codec bound. Owned fixtures now use their existing direct semio-framework-pack-json Value and typed ValueError cause. Parent owns the two generic production bounds and received exact signatures. Additional edited file: `🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️workflow-unit/🦀️.rs`; host-unit already inventoried above. No host native pass claimed. WGPU current Flow-actor band suite remains compiling; unchanged native availability probe remains preparing/compiling.

## Current Native History Presentation Follow-Through

The current WGPU history suite executed 41 native cases: 38 passed and three failed (Nextest `25ed1018-eb7c-4fdb-8231-3d4924daefcd`, `ui-native-band-current-flow-actor.log`). This is an assertion result, not a compile-only capture. The failures were the tree-row list description, a scroll-region test publishing a second registry without a second candidate frame, and the long history page not reaching `m-100`.

The existing schema-owned `TreeItem.description` now contributes the row's own accessibility description in both TypeScript and Rust; an explicit `AccessibilitySpec.description` retains precedence. The existing neutral accessibility projection fixture pins both cases. Its independent JSDOM/`dom-accessibility-api` oracle executed before the production change; the native-contract twin failed on the missing intrinsic description. After the twin changes, the same Nx invocation passed 321 checks (`ui-accessibility-description-{red,green}.log`). Native assertions remain pending in `ui-native-band-description-scroll-current.log`.

The history harness now registers its panel scroll region and retained row hits in one painted frame before one matching candidate acknowledgement. The scroll-region law consumes that returned registry instead of attempting to seal an unpainted second frame. Production candidate sealing remains unchanged. The history paging failure remains under investigation.

The unchanged Accept native law now executed and passed 1/1 (1053 excluded, Nextest `a8d35cd0-30ce-4734-baf6-2a4ab7a60d98`, `ui-native-unchanged-availability-current.log`). Its added pre-render assertions confirm an unchanged pending edit and `editor.changed == false`; the original serialized button assertions are preserved. Root repaired the fixture observer to publish the authoritative `UiNodeRecord.disabled` flag, which it previously omitted for every button.

A concurrent DWG compile caught the new Rust description fallback mixing `Label` with `UiText` (`E0308`). The current source maps each typed carrier to its owned description string separately before applying the fallback. The fresh native band capture will measure this corrected source.

The corrected native WGPU history cohort executed 41 cases: 40 passed and only the long-history paging law failed (Nextest `5e937467-c4dd-4159-a791-4f76d6b7c08c`, `ui-native-band-description-scroll-current.log`). Both newly addressed failures are now green, including the native EN/DE bounded list editor descriptions. A focused paging capture adds a failure-only census of presented scroll offsets/layout and the existing tree-window measurements; it does not alter scrolling or bypass presentation acknowledgement. The separate row-semantics conformance expectation describes the raw authored AccessibilitySpec, not the rendered projection; its raw description remains null. The shared accessibility projection fixture independently pins the component-description fallback.

## Shared History Scroll Ownership

The current history producer assembled a Stack root, while native wheel routing can scroll only an ancestor whose canonical `LayoutSpec` is Scroll. The existing neutral command-window fixture now pins a vertical/fill Scroll root and its CSS reading; the native producer law compares the authored layout through Serde to that fixture. The producer's final root assembly now uses that layout explicitly.

React's Tree interpreter previously discarded the authored root layout and supplied an inner hardcoded overflow box. Its new neutral fixture test executed red 1/1 on the missing viewport, then green 1/1 after the interpreter began reading the same Scroll root. The DOM `CSSStyleDeclaration` independently confirms vertical auto overflow, hidden horizontal overflow, and fill width; the tree-window observer selects that authored root as the viewport. Existing non-scroll Tree representations keep their existing layout path. Full Interpreter regression and current native paging are pending (`ui-react-history-scroll-interpreter-current.log`, `ui-native-history-paging-census.log`).

Owned files for this continuation: plugin main's `ui_history_panel` final root assembly; plugin command-window neutral fixture and builder-contract test assertion; React Interpreter TreeView/window viewport reader and tree-window component test; shared TS/Rust accessibility projection and its existing neutral projection fixture/oracle; row-semantics expectation; native WGPU history harness. A temporary failure-only native scroll census is under investigation and will be removed once the paging result explains the remaining failure.

The full React Interpreter regression executed 194 cases: 193 passed; its one failure identified that distinction in the raw conformance expectation. The inappropriate raw expectation change was reverted, preserving the new rendered-projection law. Current full regression is being repeated.

The full current React Interpreter regression now passes **194/194**, including the authored Scroll viewport, tree-window observers, existing input controls and unchanged raw conformance expectations (`ui-react-history-scroll-interpreter-final.log`). The native history harness also reads the same neutral command-window root-layout fixture and compares the real plugin producer's layout before publishing its lease.

The focused native paging census capture stopped before assertions on the shared replication retirement `retire_fields` macro (zero selected tests executed; `ui-native-history-paging-census.log`). The host actor capture actually executed its neutral law and failed 1/1 at the first workflow edit: `ValidationFailed("edit history insertion requires its exact mutation retirement factory")` (`ui-host-actor-description-type-current.log`). `OsWorkflowStore::new` constructed an actor-bound store without its exact owner catalogue; it now installs the existing framework-owned `store::bounded_artifact_store_owners` catalogue immediately after construction. A fresh native assertion is deferred during the Core fold source integration. The new plugin and native fixture layout comparisons now serialize `&cold.layout` and `&built.layout`, preserving their owned root for subsequent inspection/publication.

Static paging investigation found that the history harness flattened BuiltNode into preorder ordinal UiNodeIds each generation. This contradicts the actual publisher’s documented `(parent, key)` reconciliation identity: inserting an editor section changes those positional IDs and loses the retained opened transaction. The harness now invokes the real bounded `SurfaceReconcileJob`, captures its canonical document, preserves the current reconciler between generations, and closes ready patch/job/publisher owners incrementally after readers release. It explicitly compares the transaction ID before/after the editor insertion; all original mirror paging, EN/DE refusal, action dispatch and footer laws remain. The renderer Rust manifest adds the existing workspace UI runtime only as a test dependency. This source correction has not yet executed natively; the last observed cohort remains 40/41.

The host actor fixture now releases each store with the existing bounded close cursor before shadowing it for reopen and after its final document/actor projection. This preserves the strict terminal-empty Drop witness rather than adding a production synchronous Drop loop. That closure is source-authored and remains unexecuted in the current hold. No broad native reruns were started during the Core fold integration.

## GIF Owner-Path Compiler Continuation

Tools had already repaired the fifteen GIF compiler diagnostics when this lane received the assignment: three empty duplicate diff-codec mounts, seven repeated flat87 IO exports (retaining canonical89), removed subsets paths and a dead engine barrel. Their initial existing GIF SQLite source oracle actually executed six cases: three 89a passed and three 87a failed because the 87a tests incorrectly read IO APIs from the schema-only root gif87 facade (`tools-execution/gif-snapshot-source-current.log`).

The 87a tests now import their actual SQLite snapshot IO leaf and 87a snapshot type, matching the 89a test authority; all existing neutral data, SQL editing, ownership/refusal, progress/cancellation and independent Bun SQLite/Ajv assertions are preserved. The external Animate Presentation video encoder and snapshot types now explicitly name the canonical GIF89a IO/schema owners. No raster codec algorithm or binary fixture changed.

The same source oracle through explicit workspace Nx executed green **6/6**, **58 assertions**, **1.78s** (`ui-gif-snapshot-source-current.log`). A focused GIF native SQLite cohort with the real `component-app-assembly` feature is running through the canonical artifact Nx test target (`ui-gif-native-owner-current.log`); this is one artifact, not the paused broad matrix. Native assertions and the current Rust module graph are not yet confirmed.

Owned GIF continuation files: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` and `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️.rs`. Tools retains authorship of the preceding GIF15 crate module repairs. Current focused native invocation has reached cargo build; the owner elapsed duration is not a measurement of active compilation alone.

A current source audit found no remaining code consumers of the removed GIF engine barrel in GIF/Animate. Two adjacent artifact-root comments still incorrectly promised those removed paths; this continuation removed those obsolete comments, preserving every declaration and codec. Added owned file: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🦀️.rs` (documentation only in this lane; preceding compiler module edits remain Tools-owned).

Registry publication through `@semio-tech/plugin-registry:generate` actually passed in 48.9 s (its generator-inputs prerequisite 17.9 s, producer 30.7 s; `ui-gif-launch-generation-current.log`). The generated launch contains all three GIF rows 408.78086/87/88 and Tools rows 900.0580992/3/4. Existing launch seed neutral reconciliation/independent JSONC parser law then passed **5/5**, **142 assertions**, **872ms** (`ui-gif-launch-seed-contract-current.log`), preserving parallel seeds and generated rows. Publication reported 2 admitted plugin crates, 4 playgrounds, 82 framework packages and 67 withheld plugins; that is the current admitted catalog, not a complete all-family live activation proof.

Focused GIF native assembly compilation then stopped before assertions on its next local ownership seams (`ui-gif-native-owner-current.log`): both binary diff leaves call private schema `diff_pack_err` functions, and 89a analyzer invokes the shared generic `sniff_magic` through a module that does not own it. Each existing standard-specific helper moved verbatim from schema to its physical binary diff codec; refusal labels/detail remain identical. The 89a analyzer now calls the existing 87a-owned generic sniffer with unchanged `GIF89a` signature bytes. A fresh focused native assembly/SQLite retry is active (`ui-gif-native-diff-owner-current.log`), with no pass claimed yet.

Additional owned files: `.vscode/🧩️launch.seed.jsonc`; generated `.vscode/launch.json` and registry outputs through their canonical generator; both GIF87a and GIF89a physical `🧬️schema/🔺️diff/🦀️.rs` and `🚪️io/💾️binary/🔺️diff/🦀️.rs`; GIF89a `🚪️io/🦀️.rs` (shared sniff authority).

The post-Core launch preflight actually exited 0: zero generated-only configurations/inputs, zero unresolved edited seed rows, zero misplaced inputs (`ui-nine-launch-seed-preflight-current.log`). Core has authored its three fold/closure/oracle controls in the canonical seed at orders 900.0580941/2/3. Command auditing caught the oracle row's unnecessary project-cycle bypass; Core corrected it to the already established explicit workspace Nx invocation without `--nxIgnoreCycles`. Fresh canonical generation is running (`ui-nine-launch-generation-current.log`), initially waiting for shared project-graph construction; all-nine publication verification remains pending. GIF's existing source/native targets are each admitted in its authored project/router, and its third launch row uses the real assembly feature plus the existing test command.

Fresh registry publication actually exited 0 (`ui-nine-launch-generation-current.log`): 1m50s target duration, generator-inputs1m6s, producer42.6s, after project-graph waiting. All twelve newly authored controls occur once in each seed/generated document: GIF408.78086/87/88, Core900.0580941/2/3, Tools900.0580992/3/4/5/6/7 (`ui-twelve-launch-order-existence-current.log`). Core's exact published commands/names and the workspace oracle correction were inspected. The existing independent JSONC/neutral reconciliation law reran green **5/5**, **142 assertions**, **863ms** (`ui-twelve-launch-seed-contract-current.log`). Tools corrected newest nullable control900.0580996's native runner flags just after publication; a final canonical generation is active to publish that current seed, rather than altering generated launch directly (`ui-twelve-launch-final-generation-current.log`).

Final current-seed publication actually exited 0 in1m05s (`ui-twelve-launch-final-generation-current.log`): producer36.3s, generator-inputs28.5s. The twelve requested orders still each occur once in both documents, and Tools' nullable control now carries the identical corrected `--status-level all --final-status-level all` command (`ui-twelve-launch-final-existence-current.log`). Final canonical seed preflight exited 0 in38.8s with zero generated-only configurations/inputs, zero unresolved edited rows, zero misplaced inputs (`ui-twelve-launch-final-preflight-current.log`). This closes the twelve-control publication proof. The focused GIF Cargo process was sampled with no compiler child while the owner continued build-stage heartbeats; elapsed heartbeat time is therefore not asserted as active compiler time. Its native assertions are still pending.

The focused GIF native assembly/SQLite retry actually executed and passed **20/20**,126excluded,1.970s assertions, Nextest `ede3084d-df0f-4841-a33a-ff3667905388` (`ui-gif-native-diff-owner-current.log`). Nx target exited0 after21m36s including shared waits/preparation; Cargo reported19m53s for its build stage. This confirms current canonical GIF crate modules and both87a/89a SQLite semantic edits, native owned state and progress/cancellation laws under the real assembly feature. The two existing text/binary diff round-trip laws are now running through that same cached native owner to exercise the physical codec helper moves (`ui-gif-native-diff-roundtrip-current.log`); their assertion result remains pending. Animate Presentation's external89a consumer was source-checked against the public encoder and matching public snapshot owner but is not part of this GIF target's native proof.

Current dev journey source audit confirms the universal control/verb readers now consume the retained native mirror for WGPU, with reference-list actions, slider/spinbutton keyboard edits, Tab-committed text edits, switches and typed accessible options. The earlier native unsupported gap is no longer present in this source path. This is source readiness only: full EN/DE native/browser universal-family assertions still require authoritative activation and the actual journey. No live proof is inferred from the reader branches.

Both existing physical GIF text/binary diff round-trip laws actually passed **2/2**,144excluded,0.019s assertions, Nextest `81f9602c-a187-4419-aadc-a69cc0a34be5` (`ui-gif-native-diff-roundtrip-current.log`). Its Nx target exited0 in8m33s including shared waits/build. Combined GIF native proof now confirms22selected cases, alongside6source cases/58assertions. This completes this lane's primary GIF owner-path repair proof; the external Presentation import remains attributed to its later family compilation/live proof.

Core reported its frozen native cohort74301 green3/3 and lower3/3, and root lifted the native hold. Fresh owned host actor and native paging captures started (`ui-host-actor-core-stable-current.log`, `ui-native-history-paging-core-stable-current.log`); source-owner mtimes were recorded in `ui-core-stable-source-boundary.txt`. Process inspection found no remaining activation/component-dev owner, and old full activation64785's terminal log reported91m16s and failed Puzzlewasm/component+deps-cargo. Fresh canonical React activation is now running with its normal21 prerequisites (`ui-activate-react-core-stable-current.log`), shared compiler cache retained. No live journey pass is claimed yet.

Root requested two new canonical clamp controls at900.0580944/5. This lane authored both adjacent to Core's three gates, with root's exact source command and four final native law names (including fatal mutation status preservation). Root owns their producer integration/assertion receipts. Fresh canonical publication is active (`ui-fourteen-launch-generation-current.log`); no direct generated-launch edits.

Fresh native paging compiled and entered its selected assertion case (Nextest `0b50bdd8-0af6-4222-a82f-c791bd87704c`), but the fixture stalled at its final document cleanup. A two-second OS sample of the owned test PID74298 captured768/770test-thread samples in `close_retained_body` -> `UiDocumentLease::close_step`, after the paging/refusal assertions (`ui-native-paging-current-sample.txt`). The captured documents are reader aliases of the real SurfaceReconciler owner; full owner close correctly waits for that still-live publisher. The fixture now releases those exact reader aliases using the existing bounded `close_read_step_with_grant` API before retiring the publisher, matching `SurfaceDocumentOutcome`'s native close law. Only the owned stalled assertion process was terminated; target exited1, no green claimed. Fresh focused retry is active (`ui-native-history-paging-reader-close-current.log`). The temporary failure-only scroll census was removed from the interpreter and panic, retaining existing tree-window/mirror diagnostics and every original assertion.

The two clamp gates plus all previous twelve controls were published through the canonical generator, followed by preflight exit0 with zero generated-only/edited/misplaced rows (`ui-fourteen-launch-generation-current.log`, `ui-fourteen-launch-preflight-current.log`). Native clamp control includes all four root-requested law names. Root owns their actual assertion results.


## Resumed UI Ownership Boundary

The prior host actor capture actually executed one law and aborted at the strict ArtifactHistoryLedger Drop witness. Production `with_backbone_envelope` copied a populated history ledger, then dropped `envelope.into_owners()` without retiring its entries. The actor fixture also immediately dropped both `store.document()` copies. The source now routes the copied envelope through the existing exact `bounded_artifact_store_owners().retire_envelope_uninstalled` authority, grants one item/4096 bytes per close opportunity, and requires terminal emptiness. The copied sync document and both actor projections move their owners into that same canonical shell. Existing serialization and Serde actor assertions remain unchanged. This closes the observed retirement violation in source; current native assertions remain pending. These existing host persistence functions still resolve synchronously and are not claimed as new cancellable asynchronous APIs.

Process revalidation found no surviving owner for the interrupted native paging reader-close retry or React activation. Those incomplete logs establish no runtime pass. Fresh focused host33379 and paging58118 captures are active under canonical Bun/Nx owners. Shared compiler caches and peer processes remain preserved.

Additional owned files: native host production `🖥️host/🦀️.rs` and its existing `🧪️tests/🔬️host-unit/🦀️.rs` actor fixture. No native-browser assertions have run in this resumed boundary.


The resumed host wrapper executed its independent neutral actor-context oracle for both named reopen cases before native compilation; its native actor assertion remains pending. The decoder/import paths also now transfer edit-message ledger entries out of their exact ledger instead of cloning them and dropping a populated ledger; original row order is retained. Envelope close reads the authority's required byte demand before granting its next page, and genesis retirement remains the shared lease authority inside the existing owner cursor.

Canonical WGPU activation has now started too (`ui-activate-wgpu-resumed-current.log`) through its complete generated target. Its unchanged prerequisite source receipts may reuse the existing canonical cache, while producer admission continues to validate current source. React61779 and the two focused native captures remain active. No prerequisite is omitted and no browser success is inferred from activation startup.


WGPU activation's current canonical `repo:generator-inputs` prerequisite failed on `/nativeCodecs/19/protocolSourceSha256`. The exact declared row is Stdio's `s.stdio.png`, whose published source digest still reads `94abe1a4afafee64bc8d4b2fa7ff405cfc4def66ce3a81582988888b030622cf`; its authority is PNG 1.2 any IO/binary snapshot's `📡️.protocol.semio`. The publication verifier rejected source drift before registry generation. This is not a browser test. Tools confirmed that its resumed lane has not edited this protocol; active BMP/PNG peer publication jobs do not establish authorship. The root coordinator received the exact source/publication path. The existing broad native-codec-projection task asserts protocol equality before its packSchemaHash write mode, so it is not a source-digest refresh command and was not misused to bypass the refusal. No digest has been edited in this lane.


## Current Native Receipt Publication Authority

The exact PNG `owned_fixture_publication_reports_canonical_logical_carriers` producer previously printed logical carriers only and did not publish a codec receipt. Its current source now reports the actual first-party native factory, actual codec schema/hash/extension, and framework SHA256 of its compiled protocol bytes; it refuses when the current disk protocol disagrees with those compiled bytes. Existing neutral carriers and independent png raster oracle remain intact.

A new first-party source publication contract has its own draft-07 receipt schema, language-neutral fixtures, and Ajv/independent Node SHA256 oracle. Its initial canonical Nx run actually failed **1/6** at the deliberately unavailable projection; the other five schema/refusal cases passed (`ui-native-codec-publication-red.log`). The immutable projection now admits one exact native identity and current digest, refuses stale/source/factory/schema/zero-hash mismatches, and updates the factory receipt, definition binding, authored native catalog and matching open targets together without mutating its inputs. The current green invocation15896 is waiting for shared Nx graph construction; no pass is claimed.

The existing Stdio `native-codec-projection` script now accepts `refresh <artifact>` and invokes that artifact's existing Bun/Nx native carrier producer under bounded owned execution/progress/cancellation. It publishes only after process success, exactly one live receipt, the independent Node digest comparison, and a final source-snapshot check. This routes provenance through the producer instead of changing a stale catalog digest by hand. Scoped PNG refresh38585 is active (`ui-png-current-native-receipt-publication.log`), currently waiting for the shared graph. No committed digest has changed yet. The authored target declares all three contributions as outputs. Existing source verification remains strict and unchanged.

Two runnable controls were added to canonical seed900.0580948/9 for this producer and its source oracle; they and Tools'900.0580998 still require canonical generation after the current receipt. Broad peer PNG30260/BMP30259 native jobs remain preserved and are not interpreted as source receipt publication. React activation and focused Host/paging captures remain live; their owner heartbeats include shared preparation time and are not compiler-time measurements.


The receipt source law then actually passed **6/6**,2.53s (`ui-native-codec-publication-green.log`), following the recorded unavailable-stub red. Source-only admission proof does not assert current native receipt generation. The native producer target is explicitly uncached; its exact artifact carrier target still uses the canonical owned native build cache.

The resumed paging58118 retry terminated compile-red before any selected assertion: Core's replay operation retirement reexport was too private(E0365/E0603) and Store18845 referenced the temporarily absent `with_retirement_factories` method(E0599). Exact diagnostics were delivered to Core. Core reported both current source seams fixed; a fresh focused paging23270 capture is active (`ui-native-history-paging-current-operation-owner.log`). The previous40/41 native cohort remains the last assertion census. Host33379 and canonical activation remain active. The new seed/source route is being checked by the existing launch-neutral/independent JSONC law66434 (`ui-native-receipt-launch-seed-contract.log`).


Root restricted cleanup to inactive ticket-owned captures. After lsof returned no open handles and process inspection found no owner references, this lane removed only its generated ui-nx-inventory cache (316,755,509 logical bytes,314248KiB allocated). It contained generated Nx graph/source/file maps, source-fact JSON and cache indexes; no scripts, configs or Markdown inputs. The full clean skill was not executed because it would terminate peer processes, contrary to root's explicit active-peer preservation instruction. Shared Cargo caches and active captures remain intact.


The existing canonical launch-seed neutral/independent JSONC reconciliation law actually passed **5/5**,142assertions,5.38s (`ui-native-receipt-launch-seed-contract.log`). Canonical generation/preflight remains pending the live PNG publication. The next publication also includes Core's cooperative operation control900.05809425 and Tools' Puzzle2d authored-delta/retained-clone controls900.0580998/999. This confirms fixture behavior of seed reconciliation, not full current generated-launch equality.
