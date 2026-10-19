# Workspace Public Export Test Resolution

The course tests imported valid workspace packages, but their Vitest configuration contained only three handwritten aliases. Current missing surfaces included `@semio-tech/repo-lib/cargo`, `@semio-tech/schema-validator-ts`, styling builder exports and framework identity helpers.

## Existing Infrastructure

The existing styling `createWorkspaceViteResolveConfig` only forwards caller-provided aliases; it does not resolve package exports. The repository's existing `declaredWorkspaces` API does compute current native source membership across authored installation scopes. The new neutral test helper reuses that API rather than maintaining a transitive source map or scanning the tree independently.

## Implementation

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🟦️.ts` reads each current package manifest and projects only exact public runtime exports into anchored aliases. It handles ESM/node/default conditional selection, array alternatives, blocked and type-only entries, and packages without a public surface. It rejects wildcard or nonportable export paths, parent escapes, node_modules targets and mixed conditional/subpath maps. All exposed types belong to the repository. It uses system APIs and existing repository membership code; no new external runtime library is added. Callers can cancel and receive manifest progress.

The course Vitest config now consumes those current public aliases, preserving the registered launch and sessions tests. It also registers the neutral resolution and release-tag tests. When SEMIO_TICKET_DIR is set, its test cache is beneath that ticket's generated directory. The course manifest declares its actual styling build dependency, the already-installed resolve.exports2.0.3 oracle and tar-stream2.2.0 oracle as development dependencies.

## Validation

The fixture/schema were authored first. An actual stub baseline executed15 tests:14 failed and1 passed. After implementing public export selection,14 fixture/oracle tests passed; the real membership integration test initially exceeded Vitest's default5s budget because current native ownership computation takes about22s.

After giving that integration proof an explicit60s budget, the integrated selected resolution and launch run passed18 tests across2 files, exit0. The current-workspace proof checks genuine cargo/schema/styling/framework/course/pets exports, exact matching without private suffix admission and progress completion. Cancellation is also tested. The independent third-party oracle is resolve.exports2.0.3; Ajv checks the language-neutral normalized export schema. The real workspace scan took35.2s under concurrent activity. This cost is preserved rather than substituting a stale ownership snapshot.

Executed from C:/git/semio with SEMIO_TICKET_DIR set to this ticket:

```powershell
node node_modules/vitest/vitest.mjs run --config '🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts' '🧪️tests/📦️resolution/🟦️.ts' '🧪️tests/🚀️launch/🟦️.ts' --reporter=verbose
```

Temporary logs: generated/resolution-red.log, resolution-green.log and resolution-integrated.log. The first green-attempt log records the membership test timeout; it is not represented as a passing run.

## Dependency Installation Boundary

The installed isolated Bun graph contains tar-stream2.2.0 as Nx's declared dependency, but the course has no materialized direct link yet. The permanent image test keeps its direct tar-stream import; the newly authored development dependency must be materialized by canonical dependency synchronization. No fake package, raw store path, replacement compiler policy or full-repository install was introduced. The package/lock installation state remains the root agent's canonical synchronization responsibility.

Focused deployment and local-stack diagnostics now reach real assertions with the generic package resolution. Broader canonical command, native runtime and deployed behavior remain separate verification steps.

## Deployment Follow-Up

The selected deployment suite now passes23/23 assertions after replacing stale consumer workflow expectations with the actual privileged finite command contract. This confirms that the generic test aliases admit the current production modules sufficiently to execute the deployment suite; it does not prove a native image build or deployed service.

The release-tags test has also been registered at the runtime agent's request. Immediate Node resolution from Nx's own package.json confirmed its genuine installed tar-stream2.2.0 dependency exports an extractor; no package links or store content were changed. The permanent direct course tar-stream dependency still awaits normal canonical materialization.

## Current Source Ownership and Cancellation

The helper is now asynchronous and uses the existing withCurrentBunWorkspaceOwnershipV1 read-only source custody API. Every exposed ownership advance forwards the caller signal/progress and yields the event loop; the course config explicitly selects the genuine teaching installation owner. It provides the exact resolved alias projection to its integration test through Vitest providedContext, avoiding a second complete source scan inside the test.

A real during-scan cancellation baseline rejected only after 32205 ms, violating the neutral 5000 ms bound. The asynchronous version passed cancellation after three actual source advances in 10 ms and confirmed an event-loop heartbeat. The selected launch/resolution run passed21 tests, including exact current teaching lock dependency edges after root's lock repair and the quick minimum test-level contract.

Using the strict current read-only source API exposed stale native workspace projections. An all-scope attempt failed at root package.json; the selected teaching attempt failed at teaching/package.json. Both were actual startup failures with zero tests, despite historic filenames containing green. No verification was weakened.

Root authorized source-only teaching projection publication through the existing native API. It succeeded on the first attempt:120 published members became119, removing only ../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/🕸️bindings. All non-workspaces fields were asserted identical. Hashes, exact delta and native proof are retained temporarily in generated/teaching-workspace-publication.json and its log. No installation, store, root manifest or compiler policy was changed.

The installed tar-stream2.2.0 oracle was independently exercised through Node's genuine resolution from Nx's package.json: all six actual courseImageContext archive fixture entries decoded with identical names, modes, kinds and bytes. Output: generated/tar-oracle.log. This does not fabricate the currently unmaterialized direct course dependency link.

The existing native source API still contained synchronous fresh-plan drains inside its later recheck fences. Root assigned that separate repair with explicit requirements to preserve every source fence and original control; work and results are tracked in workspace-cancellation.md.

## Final Focused Verification

After the native recheck cancellation repair, actual installed Node/Vitest diagnostics passed launch, workspace resolution and deployment together: 44 tests across three files, zero failures, 14.96 seconds (course-fence-final.log). Teaching owner metadata and source-lock parity remain asserted. The original source API now uses awaited original-control advances throughout fresh membership verification; its separate source-only native regression passed 40 tests and 947 assertions. See workspace-cancellation.md.
