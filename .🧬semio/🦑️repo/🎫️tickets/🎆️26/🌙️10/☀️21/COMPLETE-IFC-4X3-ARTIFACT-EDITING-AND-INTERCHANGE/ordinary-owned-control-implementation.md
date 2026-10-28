# Ordinary And Owned Staged Controls

The IFC goal and ticket remain active. This is implementation progress; complete IFC4x3, all MVDs and actual native/web end-to-end editing are not complete.

## Implemented Source

The shared Interpreter now renders Input, Select, Button, Toggle, Slider, NumberStepper, Ring and IconSelect through a first-party UiControlPort. The document adapter calls the real document's current reads, revision subscription and intent mint. It rejects a replaced/removed rendered record, a disabled target, a missing binding and an ambiguous trigger before calling the existing intent sink. DOM naming uses the original surface/key directly. Existing blur-commit and continuous gesture lanes consume the same narrow interface; Slider now reflects disabled authority as the other controls do.

OwnedStagedControls uses a tool-independent surface interface which the actual admitted OwnedUiInstanceSurface supplies. It registers original subscriptions itself and authorizes only those exact reads. Each control port captures the actual publication object and issued snapshot; every value/disabled/event access checks current publication identity, exact subscription snapshot, matching publication revision and exact record identity. Status comes only from a current issued framework.stagedFormStatus Extension with a closed bounded payload. It does not create a UiDocumentStore projection.

The owner keeps one stable StagedControlSource and checked nonwrapping u64 sequence across individual React controls. Resident Input/Select/Button bindings are consumed synchronously through StagedFormSession.handle with args:null; unsupported ordinary actions/continuous payloads have no asynchronous borrowed-argument fallback. Text-preserving drafts display immediately through the host session and a presentation subscription. Structural edits retire the old proof and page intents preserve command args identity.

The existing useOwnedRead hook is now generic over the exact source subscription/snapshot pair; runtime subscription/read/commit-ack behavior is unchanged. OwnedStagedStatusView installs status only after a committed original read. OwnedStagedControlView lends the current ordinary record to the same shared control renderer and rechecks it at dispatch.

A Low audit found notification reentrancy could forward an obsolete staged result after a listener closed/revised the session. New failing regressions now pass: close, newer nested edit, changed publication, throwing listener and source unsubscribe failure. Dispatch rechecks owner/read/status and exact host request token/args after notification. Failed source unsubscribe stays retained for retry during close. No borrowed resident payload is handed to the host follow-up; the existing central staged reducer owns fixed/choice values before the callback.

## Executed Evidence

- New ordinary control neutral schema/cases and independent Lodash/JSON Patch comparisons: 12 tests /43 assertions.
- New structural owned-adapter probes:18 tests /67 assertions. These probes validate the adapter law with a structural source; they do not authenticate a native mint or physical work receipt.
- Original guard, paging, owned adapter and ordinary port combined:75 tests /313 assertions, zero failures.
- Expanded actual framework test-staged-form selection contains all12 neutral/wire/refresh/browser/source/control files. Executed exact same12-file selection:153 tests /692 assertions, zero failures,1.100s.
- Narrow strict TypeScript check passes for the new ordinary/owned adapters and original guard/page suites. The combined12-file type graph still fails on existing missing framework aliases; no full combined type pass is claimed.
- Wider shared renderer/fixture type graphs fail on existing missing public UI/framework aliases. A new test-render API misuse was found and corrected from getByLabelText to supported getByRole; latest fixture/hook diagnostics are only the missing existing aliases. This is not a full component or owned-instance type proof.

Red outputs, final logs and diagnostic output are under the ticket generated folder while the fleet is active. Authored inputs remain outside that folder.

## Authentic DOM Fixture, Not Yet Executed

UiDocumentStore/tests/owned-staged-controls is registered through the existing typedwire in-source registration. It uses nativeInstanceFixture first-party activation/lifetime, real OwnedUiInstance/surface lookup, encoded node operations, original retained patch offers/input receipts, native protocol acknowledgement and bounded maintenance. It mounts the shared Input/Select/Button through StrictMode and tests rapid incomplete text, choice layout replacement, page args identity, an active old root after replacement and retirement witness after cleanup.

This fixture's worker transport is an existing test authority. It is not native code execution. Renderer React exhaustive config is the correct discovery owner. A real config invocation stopped before suite execution with Explicit Vitest policy required; repo and repo MCP remain absent. No manual policy, compiler replacement, fake package export, force write, build repair or enforcement bypass was introduced.

## Remaining Integration Boundaries

The real owned form parent/body renderer and original-source controller/mount are not complete. The native standalone request donor transfer remains unfunded at the pre-guest event clone boundary, so the form route is still unadvertised/unmounted. Generic shared staged reducer cloning/browser work frontiers are not original-native physical accounting proof. The remaining source invalidation coverage, nonzero/default-generating details reducers, geometry/interchange semantics and executable all-MVD constraints remain active work.

Low independent source reviews: ordinary-control-port-independent-audit.md and owned-staged-adapter-independent-audit.md. No new confirmed P1/P2 remains in the reviewed slice after reentrancy correction; actual DOM, native type/runtime/heap/receipt/cancellation and end-to-end proof remain gates.

## Source Roster

- Interpreter/control-port: TS interface/document adapter, shared ordinary control switch, owned hook views, neutral cases/schema/feature/tests.
- Interpreter/input-views,display-views,action-dispatch,stable-dom-ids,node-tree: real ordinary view adaptation.
- UiDocumentStore/owned: generic exact original read hook.
- UiDocumentStore/tests/owned-staged-controls and typedwire: actual owned DOM fixture registration.
- ShellHelpers/window-panes/staged-args/intents: session canInteract, owned adapter, neutral cases/schema/feature/tests.
- Framework TypeScript script: two new selected staged-form test files.

