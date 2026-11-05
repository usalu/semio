# Puzzle Runtime Integration

## Scope

The installed playground catalog declares puzzle2d, puzzle3d and puzzle5d, with aliases 2d, 3d and 5d. They mount editor surfaces of the same `puzzle` hub composition component. The hub closes six app types (editor and viewer for each artifact), declares all three artifacts, their mutation rosters and activation events.

## Validation

The existing Rust target `@semio-tech/puzzle-plugin:test` covers editor/viewer dialect parity, viewer immutability, and guest/native codec parity for all three registered artifacts. The normal JS suite covers renderer coordinate ownership.

The real UI harness is `@semio-tech/framework-os-dev:time-travel` with scenario `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts`. Initial command selects React, en/de, steps 1 and 2, with output in this ticket generated folder. It is currently waiting for repository capacity admission.

A scoped TypeScript local-import inventory finds standard extensionless imports and generated `.js` imports resolved to `.ts`; these are valid. A canonical mutation fixture path appears absent and is delegated to the schema owner for review. No runtime implementation defect is established by source inspection alone.

## Asset Identity Observation

The seven descriptor example assets all have source bodies. SHA-256 comparison matches both 2d examples, both 3d examples and 5d Concrete Forest. Current 5d Capsule Dream source is 3,058,536 bytes versus descriptor 3,058,496; current 5d Nakagin is 169,827 bytes versus descriptor 169,787. These source bodies may be changing concurrently in this ticket. Rebuild the owning plugin descriptor after final fixture edits so its deployed asset sizes and digests describe the corrected source.

## Descriptor Refresh Owner

The selected component descriptor refresh route is `repo run workspace:components describe puzzle --profile prototype`. It compiles fresh selected component bytes and writes semantic descriptor pairs. `@semio-tech/plugin-registry:generate` projects admitted descriptors and does not replace this build. The one-plugin browser deployment loop is `repo run workspace:components iterate puzzle`. Both retain repository compiler admission and publication control.

## Standalone Board Keyboard Fix

The Board2d Storybook host emulated `selectAll` and `deleteSelection` actions but never dispatched these shell-level keyboard commands. It now focuses its local wrapper on canvas pointer-down capture and handles Delete / Control-or-Meta+A only within its focused interactive board. Text controls, contenteditable elements, composition and modified key combinations retain their normal behavior. Node deletion now also removes edges referencing the removed nodes through handle endpoints.

A language-neutral keyboard corpus drives a Playwright regression comparing native browser text editing against board commands: selecting/deleting input text preserves board nodes, then selecting/deleting board nodes also removes incident edges. Existing select-all browser coverage now explicitly clicks the board to acquire keyboard focus. Independent source review by the audit agent found no further actionable defect. Actual browser execution is pending the repository route.

Validation: registered `repo tools exec --light bun` successfully parsed both modified TypeScript/TSX files with Bun.Transpiler. This checks syntax only; the UI behavior regression still requires browser execution. The declared UI route cleared its initial memory wait and graph fact/closure verification, then entered project inference (127 projects). No browser process has started at that observation.

## UI Execution Limitation

The declared time-travel target spent approximately 20 minutes in capacity and repeated graph fact/closure/project preparation without executing the harness or launching a browser. Only this ticket-owned run was cancelled, and it reported releasing its leases. The exact target owner command (`repo ./📜️script.ts verify time-travel` in framework-os-dev package) then refused immediately with `Original parent Script process invocation required`. Its existing router requires the actual parent process envelope. The registered generic tool runner forwards the current environment and does not provide that owner birth, so its still-unadmitted retry was cancelled as well. No process envelope was forged, no cache/store control was bypassed, and no UI or Cargo success is claimed from these runs.

## Supported Root Time-Travel Entry

Added `repo ./📜️script.ts dev time-travel` to the existing root DevScript delegator. The repository root owns the actual process birth via `runWorkspaceScriptMain`; this route calls the existing neutral `runTimeTravelCli` with that issued invocation and installs repository-designated script providers through the existing `issueRepositoryScriptProvidersV1` API. It requires an explicit catalog, e.g. `SEMIO_PLUGIN_CATALOG=/absolute/workspace/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`. With that catalog it reaches the puzzle2d activation owner and its ordinary machine-capacity admission. No new birth owner or process-envelope synthesis is introduced. Root command help metadata now advertises the route in English and German.

The root time-travel delegator passed an independent source review and its command help executes successfully. With explicit Specific catalog and owned repository providers, the real harness has reached `playground-activation:puzzle2d:react:prototype`; that native activation target is still waiting for machine memory behind unrelated layout validation at the five-minute observation. No time-travel browser verdict is available yet.

## Storybook WASM and Factory Wiring

The `@semio-tech/puzzle-wasm` package name and all three parser exports are current. Storybook lacked the existing managed-WASM delivery resolver used by OS Vite. Its config now projects the owned engine catalog and installs `managedWasmPackagesVitePluginV1`, selecting the play facet for puzzle scopes. This resolves the true generated package under the mode-specific products store, with its declared finite files; there is no fallback to checkout pkg output or stub. The preview editor loader now imports its canonical bare package instead of the invalid pkg-prefixed subpath. The managed resolver refusal diagnostic names the rejected specifier and package.

Both standalone 2d stories additionally lacked the BoardSessionFactoryContext required by Board2dHost. They now resolve the actual puzzle artifact session roster with the exact editor identity, preserve a stable scope per story host and provide that factory around the board. The constructor initializes the actual WASM module and page runtime assets. Independent audit found no additional mandatory context missing in 2d, 3d or 5d hosts. All touched story/config files passed syntax parsing. Actual scoped Storybook compilation and browser tests are coordinated by the parent agent.

## Integration-Lane Changed Files

- `.storybook/main.ts`
- `.storybook/preview.tsx`
- `.storybook/🧪️tests/🧪️catalog-integrity/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️2d-board/🧪️.story.tsx`
- `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️2d-fixtures/🧪️.story.tsx`
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/◻️storybook-2d/🟦️.ts` (shared with parent suite repairs)
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧫️fixtures/⌨️story-keyboard/🔣️.json` (new)
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🕸️wasm/📦️delivery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🔣️.json`
- This `integration.md` report.

## Managed Generated Files vs Source Exports

Actual compilation exposed `@semio-tech/framework-os-node-graph-rs/index` being rejected by the managed resolver. This is a current public source export (its bare package exposes a different runtime API), so rewriting callers to bare root would be incorrect. Source-entry delivery rows now claim only their explicit generated files and leave source entrypoints to their source package export maps. Generated-entry delivery rows continue to own and restrict their full package namespace. Six language-neutral vectors cover source root, source host export, refused undeclared source export, generated file of a source package, generated package root and a refused undeclared generated file. The Storybook catalog-integrity regression compares source entry resolution with the actual Node package export resolver.

Additional created file: `.storybook/🧪️tests/🧫️fixtures/🕸️managed-wasm/🔣️.json`.

## Actual Browser Attachment Failure

Scoped Storybook compilation succeeded. Actual 2d browser runs reached the real session, then timed out waiting for the imperatively published parsed-snapshot verdict. Cleanup reported `attempted to take ownership of Rust value while it was borrowed` from BoardSession.free. This is not evidence of stale JavaScript: current source also publishes the extra debug attributes imperatively.

The lifecycle published the session to scene/theme/vitals effects and peers, and observed resizes, before its asynchronous canvas attachment settled. It now retains private ownership throughout attachment and only publishes runtime refs, peer registration and resize observation after that operation completes. Teardown waits for the boot operation to settle before freeing. Five language-neutral React cases cover pending unmount, pending scene/resize, disposed rejection, live successful publication and live rejection reaching an actual React error boundary. The independent borrowed-session oracle throws on calls during attachment; no session errors are suppressed.

New/updated lifecycle files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🔁️board-lifecycle-and-sync/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🧪️tests/⏳️attachment/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🧪️tests/⏳️attachment/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`

Fresh Native root time-travel run exact activation refusal is now retained in generated/time-travel.log: `Undeclared original Repo build selection` in `readOwnedBuildInvocationV1`. This is the owner validating the requested activation against the original capability launch list. The parent owns the closed initial-issuance policy correction; no retrofit or bypass is applied.

## Executed Lifecycle Regression and Built-Source Audit

The real failing Storybook renderer chunk `🟦️-yEhGSTnX.js` contains the current delayed-publication lifecycle, including the guarded cleanup; the continued borrow panic is not a stale JavaScript hypothesis. Actual source and built adapters all preserve the attachment return value: the puzzle factory returns a genuine BoardSession, the app resolver carries its factory unchanged, the frame-demanding proxy returns method.apply and the generated WASM glue returns the exported attachment result. No discarded Promise defect was found.

Retained input script `run-lifecycle.ts` obtains its selected renderer owner policy through `repositoryVitestPolicyV1` and executes `runVitestV1` with one worker, long-suite inclusion and a 60-second bounded budget. It fabricates no process Original. Actual result: one file, five tests passed, exit zero, 4.38 seconds total / 49 milliseconds in cases. The expected live refusal is delivered to a real React error boundary. JSDOM also reports its unavailable canvas getContext implementation for the unrelated trace canvas; the suite models session ownership, not GPU drawing.

The declared `@semio-tech/puzzle-plugin:wasm` producer is queued to compile the current Rust source and publish the actual managed package. Its current Rust attach_canvas implementation returns future_to_promise with cloned Rc state specifically to avoid self borrowing across await. The parent/core lane has no parallel direct Cargo compiler. Generated output remains under this ticket for cleanup after validation.

Additional retained input: `run-lifecycle.ts`.

## Declared Producer Source Admission

The exact owned `repo run @semio-tech/puzzle-plugin:wasm --configuration=prototype --output=stream` completed graph expansion but refused before Cargo in the Nx authorizer source capture: 1,787 files and 16,791,011 bytes against 2,048 / 16,777,216 limits. The supported Semio backend was tried with unchanged ownership, Native, custody and source controls. It reaches the same compiler command source capture and refuses before any producer with 1,821 files and 16,777,958 bytes (742 bytes over). This is a distinct compiler closure from the Flow native API source extent; no limits, custody or source controls were changed. Generated logs record both refusals. No compiler or producer remains live from this lane. The root ownership lane is reducing actual dependencies rather than increasing the closure boundary.

## Activation Launch Responsibility

The harness imported the complete ActivationScript execution/materialization owner merely to delegate to an activation producer. Moved that exact Original-authenticated delegation into `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🚀️launch/🟦️.ts`. The harness imports this direct owner; execution no longer exports the helper. This is a source responsibility separation, with no import hiding or compatibility facade. Exact target derivation, caller Original, bounded build budget, cancellation and repository command ingress remain unchanged. All three changed TypeScript files parse through the registered Bun transpiler. Actual producer retry will establish whether the compiler closure reduction is sufficient.

## Concrete Snapshot Borrow Fault and Error Preservation

An actual first-case rerun against the retained owned Storybook artifact captured the primary console failure in generated/p2: `recursive use of an object detected which would lead to unsafe aliasing in rust` on `defersDescriptorSyncFromJs`, preceding the cleanup free failure. The independent Rust audit found the preceding cause: `loadBoardSnapshotJson` passed four independently acquired `self.state.borrow_mut().host` field references in a single call, so RefMut temporaries overlap. The bridge now acquires one state guard and passes disjoint fields of one host. No other line in that bridge has multiple borrow_mut calls. The actual browser first-case regression consumes the language-neutral `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧫️fixtures/📸️board-session/🔣️.json` and checks successful parsing followed by actual session reads through Playwright. This browser check remains pending the fresh producer, with snapshot readiness unchanged.

The lifecycle scheduler previously discarded renderFrame failures; it now forwards them to the existing React session failure path. A cleanup failure accompanying the primary error is retained as an AggregateError instead of replacing the primary. The independent React/session oracle now covers seven language-neutral cases, adding render refusal and simultaneous attach/release refusal. Actual result: one file, seven tests passed, exit zero, 4.83 seconds total / 48 milliseconds in cases, using the retained legitimate repository policy helper. No expected failures are suppressed.

Additional changed source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌉️bridge/🦀️.rs`. The producer lane owns the serialized fresh Rust build. No producer or static server remains live from this integration lane.

## Exact Full-App Validation After the Producer

The scenario declares variant `puzzle2d`, app `s.puzzle.puzzle2d@1/*#editor`, React port 6012 and WGPU port 6112. Its generated catalog row declares the actual Puzzle WASM engine owner. The root command below owns fresh birth and predeclares the exact activation launch before issuance; it must be run after the current producer lane settles, with no concurrent direct Cargo. It has not been started in this lane.

```sh
SEMIO_BUILD_MODE=prototype \
SEMIO_PLUGIN_CATALOG=/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated \
repo ./📜️script.ts dev time-travel \
  --scenario ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts \
  --variant puzzle2d --renderer react --locales en,de --only 1,2 \
  --out .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END/🗑️generated/react \
  > .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END/🗑️generated/time-travel.log 2>&1
```

The delegated child selection is exactly `repo run @semio-tech/framework-os-dev:activate-puzzle2d-react-prototype`. Its inferred prepare target owns registry session, framework-plugin-web support, Infinite fonts, renderer engine producers, the declared Puzzle engine producer, and the boot component closure (the Puzzle component manifest declares neither extends nor depend-on). Component compilation and browser materialization are owned producer responsibilities; the harness does not replace them or regenerate descriptor metadata by hand. Activation publishes only after preparation. A cold full-app run can therefore compile component and engine work even after the standalone Puzzle WASM has been refreshed.

The WGPU command uses `--renderer wgpu`, the same explicit variant and catalog, a distinct ticket output directory, and delegates `@semio-tech/framework-os-dev:activate-puzzle2d-wgpu-prototype`. Its prepare chain additionally owns WGPU wasm and the browser/frame-worker/renderer boot generators instead of the React renderer engines. Start with steps 1,2 as a bounded boot/pointer/history oracle, then run the full scenario without `--only` after those pass. All selected locales have fresh browser contexts.

## Explicit Selection Policy Without Shell Scope

The shared selection-options effect previously returned unless a ShellScope existed, so standalone lasso scenes silently retained default rectangle selection. It now applies the explicit scene selection method and flags regardless of ShellScope, uses the shell merge policy when present and the canonical replace policy otherwise, then optionally subscribes. The existing live-success language-neutral lifecycle vector now specifies lasso with disabled edge/handle selection and the independent session oracle checks the exact arguments. Canvas appearance updates invalidate the existing scheduler rather than directly rendering in a swallowed-error catch. The prior seven-case pass predates these last two refinements; their rerun is currently queued under normal repository capacity and must settle before reporting the final seven-case result.

## Selection Reset Regression and World Empty Picks

The final selection regression executed and exposed a second actual reset: applySnapshotToSession forced all selection flags true before the policy effect. Removed that competing setter from Board2dHost/session-sync and made the single selection effect reapply its explicit policy after snapshot changes. The neutral live-session oracle now refreshes a parsed snapshot and verifies both applications keep lasso with edges/handles disabled. That updated rerun remains queued; the immediately preceding actual result was six passed and one failed, not a final pass.

The standalone 3d/5d story reducers also coerced worldPick id:null to numeric zero, selecting the first instance on an empty click. Both now share a strict numeric-index/null picker and shared merge algebra at `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎯️selection/🟦️.ts`. Strings, booleans, empty strings, fractional/negative/out-of-range values preserve selection. Null follows the actual generic app interaction port: only Replace clears; Range/additive/subtractive/toggle preserve current selection when there is no target. Sixteen neutral vectors under that helper are consumed by the already configured catalog-integrity unit owner. Root owns actual browser pick/clear checks. The helper and both story files parse through registered Bun; the unit owner has not yet rerun those new vectors.

Latest final renderer result after selection refresh/reset repair: one file, seven tests passed, exit zero, 4.40 seconds total / 55 milliseconds in cases. This includes exact lasso/flag application on initial and refreshed snapshots without ShellScope.

Parent-owned configured Storybook unit rerun passed all 60 tests, including all sixteen world-picker vectors and the full 143-case browser discovery inventory. The initial keyboard focus group now resolves its accessible name through the existing useLabel(ui.diagram.label) locale/tier port; the parent owns actual EN/DE focus/name browser assertions.
