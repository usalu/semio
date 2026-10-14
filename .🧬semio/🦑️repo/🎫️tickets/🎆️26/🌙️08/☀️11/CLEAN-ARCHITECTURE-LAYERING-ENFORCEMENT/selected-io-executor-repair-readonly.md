# Selected IO Executor Repair Read-Only

Actor GUI12 reached selected Source closure and graph assembly, then Root reported terminal 1 after 25.210 seconds with no assertions: IO control's inactive `test-source` target still declares `nx:run-commands`. The portable interface accepts only `repo:script` and `repo:noop`; no executor alias or fake noop will be added.

## Exact Owning Inputs

- IO control project `🧰️framework/🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust/📋️project.json`, SHA `d9214546f4ab304bef65f6991d9dc47238999bd88313af793279cad011a1bc8f`: three script commands (`test-source`, `test-native`, `test-snapshot-receiving-native`) use the wrong external executor.
- SQLite model project `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🏛️model/📦️packages/🦀️rust/📋️project.json`, SHA `4eaa465cf09157d0d0ba4e42ab76e78ae79619742242b142473fe294bfe2942c`: `test-source` has the same wrong executor.
- A completed selected Actor ownership inspection found precisely these four unsupported authored targets among the admitted owners. It did not invoke a frontend or initializer.

The owning scripts already dispatch concrete Source/native classes. Repository search found no generator contract or production generator linked to either project identifier: the owning project documents are their direct authoring declarations. The only remaining production constructor spelling `nx:run-commands` found outside tests/assets is the separate cache-verification fixture builder, which does not generate these IO projects. It is not evidence of an IO generator.

## Prepared Batch

Handcraft the four owning target executors as `repo:script`. Keep their actual script commands, cwd, cache:false and prototype/release declarations. Add Source execution metadata to the two Source operations and native/test metadata to the two native operations, matching the classes and the existing execution schema. No graph/API change is required to legalize an external executor. Dependency-only target interfaces remain inert; production assembly already passes the exact active target roster to the normalizer and restores authored inactive interfaces without executing their command Source assembly. Neutral interface validation remains strict.

Root instructed that Product600/native3600 captures remain untouched until terminal or explicit coordination. No IO owning JSON, graph/API, script, signal, policy, lease or native output was changed during this inspection.

## Native10 Source Phase Inspection

The actual command in the existing log is `@semio-tech/framework-plugin:test-media-turn-native`, original prototype configuration. The owning target declares a complete plugin subtree input and imports the repository's TypeScript package facade from its script. Native normalization resolves native source groups plus exact command implementation. The native command fallback itself starts at the owned Cargo script and its relative import closure; it is not a whole workspace manifest walker.

The reported `build:selected-compile-imports` phase iterates an owner-local module roster, structurally parses all typed/runtime imports through the owned Source analysis, follows resolved relative modules, and emits a checkpoint per sixteen visits. Each owner inspection creates a fresh command source cache and visited set. Repeated source-group admissions can therefore revisit shared facade/kernel modules across owners even though filesystem analysis facts are content-aware. The facade import is an actual declared runtime module import, so its static import/reexport closure is currently required by this parser boundary. This does not establish that Specific catalog admission is unrelated or that a lock caused the delay.

The current log shows advancing source-input and compile-import checkpoints, but does not include owner names, exact file counters, cache hit counters or one loaded module identity. Thus the log proves ongoing bounded continuation; it cannot establish which owner/default group dominates time or prove a SQLite/cache lock. No duplicate expensive native closure probe, live process inspection with mutation, or new instrumentation was launched. Exact owner/counter instrumentation would require a separately coordinated Source change and fresh invocation.
