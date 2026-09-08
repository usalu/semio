# Shell Document Opening Scope

## Current Evidence

The real Space create/open/open-with commands already emit document ID, space ID and schema directly in `ReplayShellCommand.args`. The generic AppCommand opening relay is a separate host-app selection path; its missing tuple is not evidence that Space drops identity. The collaboration harness diagnostic claiming that cause is stale and will be corrected.

The actual React Shell opening path uses mutable current-route space to choose default bindings. This lets an unscoped local document inherit a Hub space and permits shared intent to fall back to a folder when identity is absent. The route-index branch also computes a newly created Space session but fails to pass it into document opening before React publishes that session.

## TDD In Progress

The existing binding computation has been extracted without changing behavior into the existing opening helper. A closed language-neutral six-row corpus now covers exact shared scope, local isolation, identity refusal, Hub-only storage, local unsigned operation, and exact surface refusal. The test checks actual helper output against the corpus and fast-json-patch's independently constructed binding array, with AJV validating the closed corpus.

The first runnable scope test (session 5176) failed behaviorally: exact space A was expected but both Hub scope and folder path used the current route's space B. An earlier invocation selected the quick-only suite and ran no tests; the next exposed an AJV strict-schema error, corrected before this behavioral RED. Production now uses the opening reference's optional space, refuses shared opening without identity or exact surface, and sends the protocol's current requestedSurfaceId field. The route-index call now supplies both exact space and the newly returned Space session/plugin. No generic AppCommand schema expansion was performed.

The registered document-opening-scope-check passed all 3 tests (session 98081, 93 ms test time), including all six scope vectors; every DEBUG row was read. The existing live-router and explicit-new-session helper tests also passed. Route-to-helper wiring was inspected; a mounted Shell route was not exercised.

A second behavioral RED (session 99914, first-open test 140 ms) reproduced the worker's circular first-open guard: the actual handleTsRequest(open) entrypoint emitted no HTTP requests without a prior installedTarget. The internal installer already supported requestedSurfaceId-only acquisition, so only the outer guard changed. Local-only opens no longer receive the same spurious socket failure.

Cold-document-pair-browser-check then passed all 4 selected laws (session 75493, 18.95 s total). The new entrypoint law used bounded mocked server responses containing the existing verified component/descriptor fixtures, installed a real private worker lease, issued exactly open-plan → manifest → component → descriptor → socket-grants, created one mocked socket, and sent one actual encoded SocketHello. It supplied no prior installedTarget, kept the actor unauthenticated before Session, sent no document writes, preserved local-only operation, and refused an unselected shared request. Runtime DEBUG was read: requested-surface-only=1, verified-assets=3, socket=1, hello=1, authenticated-session=0, local-failures=0, unselected-refusal=1, writes=0.

This is worker runtime and helper evidence with mocked network/socket endpoints, not a native GIS component, authenticated Hub process, mounted Shell or second-peer end-to-end result. The earlier initial-scene worker laws remained GREEN.

The new exact scope target is implemented in the existing renderer 📜️script.ts, registered in its project.json, and added to the launch seed. Registry generation passed (session 32956: 59 plugin crates, 60 playgrounds, 45 framework packages); scoped Prettier and git diff --check passed. The full registry check (session 23606) failed on the repository-wide plugin taxonomy, with 4,187 output lines including missing artifact schema/io/engine/examples directories and undeclared Rust modules. This is not a launch-freshness result and is not attributed to the opening changes. The narrower existing check-generated gate passed (session 96804): generated catalog and launch bytes are fresh.

The expanded entrypoint corpus also passed in session 93270: 5 selected worker laws, 17.62 s total. Four hostile first opens made exactly 2/4/2/3 asset requests for foreign plan scope, corrupt component, cancellation at manifest, and retirement at component respectively; every case emitted zero socket grants, sockets and writes, with no live published lease. All four DEBUG rows were inspected. No additional production bypass or alternate restore path was added for these tests.

## Separate Remaining Work

Opening-attempt cancellation and generation correlation remain separate: socket actor responses are currently only runtime-keyed, so a replaced attempt can interfere with a successor. This scope slice does not claim to repair that lifecycle issue or prove a native GIS/Shell/Hub end-to-end opening.
