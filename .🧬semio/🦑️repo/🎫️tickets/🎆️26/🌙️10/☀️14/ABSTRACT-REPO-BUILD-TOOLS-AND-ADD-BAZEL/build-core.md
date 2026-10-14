# Build Core Implementation And Evidence

The portable graph is the source authority for both backends. The public dispatcher preserves the original invocation, cancellation, resource capabilities and backend choice. Authored commands use repo:script/repo:noop and common utilities live under build. Nx consumes the exact pinned publication and translates execution only in its private projection.

## Validation

- Latest portable graph/selection/bootstrap/watcher/native-caller and catalog provider tests: 27 passed, 111 assertions. Includes independent Ajv, esbuild, Nx task graph, Nx glob, fast-glob and Bun digest oracles.
- Catalog provider tests after neutral source move and explicit source-reader admission: 7 passed, 19 assertions.
- Strict TypeScript diagnostics in owned runner/model/adapter/provider modules: zero. Full dependency closure currently reports the external-owner sourceBoundary diagnostic detailed below.
- Frozen Bun install after owned Nx patch lock digest refresh: exit 0.
- Tools setup all: exit 0, Bazel ready and Nx 23.2.0 reported without graph discovery.
- Installed Nx cache custody fixture: actual source mutation withheld cache promotion, second execution rebuilt, third restored warm cache.
- Final complete graph/task inventory and default Nx owner execution: pending live publication below.

## Moves

Main library inference was extracted from library/🟨️.mjs into build/graph/inference; original library entry is now a thin Nx projection. Pure argv authority was split from the retained private Nx executor, with no compatibility exports.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🔌️nx/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🔌️nx/🧪️tests/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🔌️nx/🧪️tests/🧫️fixtures/🔣️.json` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🔌️nx/🧬️schema/📣️contribution/🔣️.json` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🧬️schema/📣️contribution/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🔌️nx/🧬️schema/🔣️.json` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🎚️mode/🟨️.mjs` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🏃️execution/📥️preparation/🧬️schema/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/📥️preparation/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🏃️execution/🧪️tests/📥️preparation/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧪️tests/📥️preparation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🏃️execution/🧪️tests/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🏃️execution/🧪️tests/🥒️.feature` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧪️tests/🥒️.feature`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🏃️execution/🧬️schema/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🟨️.mjs` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🛡️policy/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📤️arguments/🧬️schema/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🌱️sources/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🌱️sources/🔣️.json`

## Created Or Updated Files

This inventory includes shared files whose references were updated during the ownership moves. Other agents also edited some listed files; their own reports distinguish their changes.

- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `bun.lock`
- `nx.json`
- `✏️s/🧑‍💻dev/📜️script.ts`
- `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🧪️tests/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🌐️playground-site-dist/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧩️native-prerequisites/🟦️.ts`
- `🧰️framework/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🧬️schema/📣️contribution/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🎚️mode/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧫️fixtures/nx-bootstrap/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧫️fixtures/🔁️graph-revision/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/📥️owner-command/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/📥️preparation/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧪️tests/📥️preparation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧪️tests/🥒️.feature`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🛡️policy/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🌱️sources/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️caller.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📤️arguments/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📤️arguments/🧪️tests/📥️native-caller/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📤️arguments/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️executable-owners/🧭️ownership/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts`


## Final Authority Refinements

- The CLI exports `runBuildCliV1` and admits authored `SEMIO_SCRIPT_POLICY` and neutral optional-tool resource grants before the original process envelope exists. Both backend choices expose the same native caller schema. Private Nx projection retains the parent's control, signal, deadline and resource values.
- Bootstrap readiness resolves executable parser entry points under this workspace's own node_modules and checks their locked versions. An absent neutral receipt alone does not trigger an install. Missing modules install through the system-only dependency owner.
- Local Cargo project enumeration closes over actual manifests, and dependency edges resolve each inherited path against that crate's own workspace. Backend-only `.nxignore` and `nx.json` cannot invalidate neutral structural authority.
- Direct private argv tests: 4 passed, 73 assertions. Complete native caller tests with original artifact environment: 17 passed, 317 assertions, including real Bun/Node owner handoff.
- Installed Nx source cache custody passed with a mandatory original fixture envelope and normalized output custody.

Additional created contract: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authoring.json`.


## Fresh Dependency Runtime Proof

An isolated manifest and freshly locked package scope began without node_modules and without Nx. The actual default dependency owner installed locked TypeScript 5.9.3 and TOML 2.2.5 with `bun install --frozen-lockfile`. Parser entry points resolved under that isolated node_modules. A second admission used an installer that throws if called and completed successfully. The temporary fixture root was removed. No workspace ownership traversal or graph bootstrap occurred.

The actual CLI installed fast-path test additionally retained an authored 30-second owner policy, omitted-tool grant, custom transport byte/line limits, child deadline and offline network flag. The real receiving subprocess observed less than the initial remaining time, and no graph was acquired (1 test, 16 assertions).

Per-project native assembly keys now depend only on the project's own actual Cargo owner closure. The regression confirms an unrelated new crate does not change the key, while renaming its actual dependency owner does. Source-authority mismatches retry only after detecting real source changes; a stable missing project owner remains an error.

## Exact Consumed Source Authority

Whole-tree implementation timestamps have been replaced by complete admitted physical file/directory membership, type and mode, together with exact facts recorded by actual inference reads. The receipt is schema-defined as version 2 at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authority.json`. Content-aware checks admit identical byte rewrites. Compact source/lock closure declarations are observed and their SHA256 filenames are checked against actual bytes. Raw graph policy/schema initializers are explicit source facts.

The language-neutral fixture demonstrates unchanged graph authority for an unread Rust body edit and an identical declaration rewrite, invalidation for a consumed declaration edit, and physical membership invalidation for an added owner. A removed owner restores the original physical membership. The failed-read test proves a stable malformed declaration remains stable, while changed consumed bytes provide retry evidence. The focused graph suite passed 19 tests / 89 assertions. The original standalone observation laws passed 21 tests / 62 assertions; SDK-dependent Node tests require their original public owner context and are separately qualified.

Inventory races retry only when a directory/member previously observed by the same inventory disappears or changes type. Inference exceptions retry only if admitted membership or captured consumed facts changed. Stable parse errors, permission errors and missing semantic owners remain failures. Original cancellation is checked throughout inventory and fact validation.

Additional updated files:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs` (capture and inherited source-fact publication; later audit corrections belong to dashboard agent).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authority.json` (created).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/📥️membership/🟦️.ts` (explicit non-null piped stdout).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🟦️.ts` (bounded inventory callback returns void).

The final graph and provider test run passed 26 tests / 108 assertions after routing policy script reads through the observation boundary and requiring the catalog source reader port. Standalone application callers explicitly provide their own reader; inference supplies its original observed reader. Graph policy/schema data are read at actual use, avoiding a later observation claiming bytes different from a module-loader initializer. The provider read handoff is checked against its admitted contribution JSON and independent Ajv schema.

The dashboard agent subsequently completed the consumed-result observation correction with 34 tests / 109 assertions, including directory and TypeScript host races, conflicting repeated facts and cached inheritance. The graph source facts now use that implementation. The complete dependency strict check reports one existing external-owner error at caching/artifacts/native-build:36 (`sourceBoundary` absent from CargoArtifactBuildOptionsV1); runner, graph, provider and new provider test themselves have zero diagnostics.


The final narrow membership regression replaces a captured ancestor directory with a file. The third-party fast-glob oracle rejects the same descendant, and the owned membership inventory emits BUILD_MEMBERSHIP_CHANGED for retry. Both ENOENT and ENOTDIR are recognized only for previously discovered physical entries. The resulting combined focused suite passed 27 tests / 111 assertions.

## Concurrent Discovery And Memory

The diagnostic captured 101,247 facts and rejected 11 authored files whose SHA256 and timestamps changed during discovery. These included Cargo declarations, one project declaration, Rust topology sources and a script. This was real concurrent source mutation, rather than compaction or record-cache writes. The observation checks correctly refused to publish a mixed graph.

Directory inventory now reads at most 32 directories concurrently, preserves sorted admission and physical kinds, and checks the original cancellation before/after each batch. It publishes a completion event so inventory and primary inference can be distinguished.

Fresh discovery previously eagerly decoded and retained the old 99 MB graph, and retries recursed. Cache admission now checks the small schema-defined authority receipt and exact membership/facts before reading canonical graph bytes, verifies their digest before parsing, and parses directly only on explicitly requested reuse. Invalidated candidates return from a finite attempt helper before an iterative bounded retry begins. The focused suite still passed 27 tests / 111 assertions after both changes.

Tools setup passes the canonical original native offline grant explicitly into Bazel provisioning, rather than relying on ambient CARGO_NET_OFFLINE.


## Structural Source Analyses

Fresh diagnostic fourteen stopped through its original SIGINT control before source edits. It observed two distinct actual byte images of the Cargo admission TypeScript helper during one discovery, both live facts rather than module-load facts. That helper was consumed for its import topology. Raw byte observation therefore caused correct rejection under the former contract, but the structural contract was unnecessarily volatile. The process reached approximately 16.7 GB RSS after repeated invalidated attempts; the graph was not published or reported as qualified.

Structural analyses now have registered owned providers, with schema-defined persisted facts identifying the provider, evaluated parser revision, actual source byte identity, complete JSON result SHA256 and nested dependency observations. Receipt data never supplies executable code. Unknown or changed providers invalidate. The parser revision includes inference and command-parser authority plus the actual TypeScript compiler implementation bytes. Command import strings, module imports/production imports and Rust modules/includes/errors use this port. Raw declaration, routing and independently byte-consuming reads remain raw facts, and task execution input hashes still include full bytes.

Module metadata retains file identity, complete imports and production imports. Unconsumed diagnostic implementation-byte hashes were removed from the portable metadata schema and its consumers. The scheduling fixture adds ordinary body edits, dependency edits and mixed raw/semantic consumers; the module result is independently compared with esbuild. Node project receipt assertions now compare command-analysis result digests with TypeScript preprocessing output.

Additional edited sources attributable to core for this feature:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏗️scheduling/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏗️scheduling/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏗️scheduling/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧬️schema/📦️projects/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧫️fixtures/📦️projects/🔣️.json`

The coordinated framework-owned artifact marker literal in build/📜️script.ts is now `.semio-artifact.json`; the Bazel agent owns the complete marker schema and other publishers/consumers.


The source-analysis integration passed its three scheduling tests with 48 assertions, including real esbuild and rustc dep-info oracles. The combined core/model/graph/scheduling gate passed 28 tests with 696 assertions. The production graph module passed its dedicated strict TypeScript check. Graph test initialization now establishes the actual inference module-load boundary before creating transient test roots; production loaded authority is never cleared.

The actual Node fixture runtime exposed a broad routing import containing a TypeScript parameter property unsupported by Node strip mode. Inference now imports the narrow canonical original-invocation functions, which contain no application Script class. Node then reached the expected fixture assertions. Its async module-source byte-read expectation changed from zero to one on a warm pass because semantic authority checks actual bytes; parser replay remains memoized. The independent oracle values remain unchanged.

The thin Nx entry no longer reexports common inference APIs. Dashboard agent owns the direct neutral-source import changes to its consumers. This removes eager common inference bootstrap from the adapter entry and removes convenience aliases through a backend authority.


## Configuration Identity And Node Qualification

Configurations override execution options only. Task executor identity, native/owner binding, scheduling dependencies, input/output sets, cache policy and continuity are target invariants. Both authoring and portable schemas reject those configuration fields and reject command/commands form changes. Same-form command overrides, arguments and environments remain supported. Portable semantic admission enforces the same invariants, including direct configuration resolution. The negative language-neutral fixture was run red before admission correction and green afterward. Strict Ajv and the owned validator agree (24 graph tests, 165 assertions).

The sole Nx target projector lives in the private adapter entry. It explicitly combines base and configured environments before Nx's shallow option combination. The real installed Nx combineOptionsForExecutor oracle confirms the base-only variable survives and an overridden variable receives the configured value. The neutral graph no longer exports Nx projection APIs.

Actual Node project fixtures now pass two tests with 25 assertions through their original process envelope and immutable installed SDK. A concurrent import edit is recomputed coherently: both parsed command and full-module observations contain the new `./extra.ts` result, verified by an independent TypeScript preprocessing oracle and SHA256 checks. The prior empty import result is distinct. Owner computation counts show only the two reverse-dependent owners recompute; the unrelated owner stays warm. A later physical source addition causes those same owners to recompute once more. The source-analysis standalone suite also explicitly refuses a captured import topology after a dependency change.

Schema compilation is strict in both test implementations. A missing conditional subschema type was observed red by the independent surface gate and corrected explicitly; strict:false was not used to hide the error. The final surface and all-authored declaration census belong to the final-audit report.

Additional shared schema edits: `🏗️build/🧬️schema/🔣️.json` and `🏗️build/🧬️schema/📋️project/🔣️.json`. All existing Node-project fixture schema/test changes listed above remain part of core's inventory. The dashboard agent owns direct common-import migrations, parent provider check identity correction, watch source projection, helper schemas and helper laws.


## Final Structural Classifiers And Provider Admission

Final fifteen did not publish a graph. Its first two candidates correctly refused new physical native-owner membership and changed Cargo declarations, then an actual TypeScript import-topology edit. The completed third candidate was invalidated after an independently byte-observed native-host script changed during dependency inference. No stale loaded producer fact was identified. Success-path diagnostics now include exact added/removed membership and stale observation rows rather than only reporting a refresh.

The last raw script consumers were classified precisely. Go toolchain discovery and Wasm tooling selection now use a closed, schema-defined `{go,wasm}` structural result with the exact prior predicates. Body-only rewrites preserve this result; compiler-selection changes invalidate it. The native-host owner now consumes its explicit bounded-script admission port through a registered semantic provider; Cargo/project declarations, root router AST reads and genuinely textual catalog reads retain their original raw authority. The dashboard report owns the native-host API/schema/caller changes.

Catalog provider imports now bind the admitted source SHA256 in their module URL and verify the same source after import. The focused fixture demonstrates that a canonical ESM import remains cached after a source overwrite while the owned revision loader receives the new contribution. This addresses top-level provider evaluation provenance; no claim is made that arbitrary transitive dynamic imports are automatically reloaded.

The final changed graph gate passed 26 tests / 185 assertions. Script classification is compared with an independent TypeScript AST oracle and strict Ajv. Source-revision hashing is checked against the system crypto implementation. Additional core-edited files for this final scope are the graph inference module, graph module, build schema, graph test and graph fixture already listed above, plus the existing Binaryen toolchain test whose Wasm predicate port now accepts its actual boolean result.

The final fifteen natural project phases remained approximately 1.4–4.8 GB RSS across ordinary retries. One diagnostic forced collection reduced live heap but transiently raised RSS from approximately 3 GB to 9 GB, so no permanent forced-GC policy was introduced. Final sixteen runs with original cancellation, bounded invalidation guard and concise memory counters; its graph/default-Nx outcome is still pending.


Final sixteen correctly retried a newly added envelope crate and changed Cargo workspace declaration, then stopped on a stable invalid authored execution annotation. `⚡️caching/📋️project.json` declared its Bun `test-repository-exact-cargo` fixture as native with unsupported scope `fundamental`, despite the owner having no Cargo.toml. It now uses the source execution contract already used by sibling test-rust-units; the leaf fixture's native grants remain unchanged. Production admission errors include the project:target identity. A cheap full preflight passed all 662 rg-visible authored project files and 76 execution annotations using the production contract and native Cargo owner check. The independent strict Ajv admission fixture passed one test / 24 assertions. The unprepared broader execution harness was not claimed green: its separate generated-family vector requires an original prepared inventory and was deliberately filtered from the scoped admission proof.

Additional edited source attribution: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json` (only the exact-cargo fixture execution annotation). Final seventeen is the next fresh qualification.

Exact final classifier test source attribution: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🟦️.ts` (the two Wasm selection port call sites).


## Command Shorthand And Selected Rust Aggregates

Final seventeen did not publish. All three invalidations had exact live structural causes: command/module imports changed, two storage enforcement owner files were added, then a test module's import result changed with stable physical membership. The last two completed candidates demonstrated the success-path diagnostics. The process was stopped by its original control guard. RSS grew after completed candidates while reported JavaScript heap remained roughly 1.0–1.4 GB; the next qualification therefore uses a fresh process and a two-invalidation guard with phase walltimes. No observation was weakened.

Authoring command shorthand now becomes execution.command and normalizes before command input inference as well as portable conversion. Duplicate or cross-form command declarations are refused by both owned semantic admission and strict Ajv. The real installed Nx option-combination oracle receives the same selected command/environment.

The concurrently introduced selected Rust graft used configuration-specific scheduling fields that violated the tightened target identity contract. Its aggregate now owns noop execution, cache policy, input/output sets and all producer dependencies at the base target. Every authored/selected mode remains selectable through empty execution options. The actual synthetic graph produced by that helper passes owned graph admission and strict Ajv; the Bazel planner and installed Nx task graph agree on its exact selected/producer task set and dependency edges. Focused combined graph validation passed 28 tests / 210 assertions.

Additional edited source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🟦️.ts` (only aggregate construction/helper; another chat created the surrounding producer). Existing graph schema/test/fixture and inference sources contain the shorthand changes. Final eighteen starts fresh after these gates.

## Fresh Publication And Physical Policy Ownership

The interrupted eighteenth process did not publish. A fresh nineteenth process retained its original CLI policy, capabilities, cancellation, progress and deadline through the authored ticket qualification harness. Its first candidate correctly rejected a live byte change in the membership-journal worker script with unchanged physical membership. The policy-script contribution consumes every admitted script through observed full-text policy/export and policyFile matching, so this raw observation was retained without weakening. The second candidate published atomically and the process exited successfully after 173.25 seconds. The immutable graph digest was `c9d6df4f84aa51f053fc206e4bacd592b24d631b41830232536bca05a9419845`, containing 1,179 projects, 33,853 edges, 9,848 base targets and 131,150 authority observations. This snapshot is historical evidence; the ownership correction below supersedes it.

Actual public default Nx execution against that exact pin exposed six generated policy facets with the same physical roots as existing owners. Nx's real root merger replaced the owners, then rejected their dependency sources (`@semio-tech/cad-js` was first). This was a real runtime failure before task execution. The six owners were cad-js, workspace, repo-cli-go, repo-client, repo-coordinator and repo-test-domain.

Policy inference now contributes `lint-policy` to the existing physical owner. Ordinary `lint` keeps its actual execution and previous dependencies and additionally schedules `lint-policy`; an absent lint gets a noop scheduling target. A declared lint-policy collision is refused. Synthetic policy owners remain only for roots with no other owner. The canonical model refuses duplicate physical roots. Dependencies are inferred after the corrected contributions are assembled, so they use the actual owner identities.

The two new language-neutral ownership tests were run red before correction and green afterward. Their fixture lists the actual six owner/root pairs. The complete focused graph suite passed 30 tests / 247 assertions, including strict Ajv, the real Nx createNodes root merger, ProjectGraphBuilder dependency admission, task graph and Bazel planner. Existing lint commands, scheduling edges and exact policy script paths are checked. The production graph module passed its dedicated strict TypeScript check with no diagnostics. Source attribution remains the existing graph module, policy inference module, graph test and graph fixture. The authored qualification harness is retained at `qualify/📜️script.ts`; generated logs are temporary.

The configured-command source union correction is separately owned and documented in `configured-inputs.md`. Its combined graph gate passed 31 tests / 467 assertions, including twelve effective command/cwd/list/args/fallback recipes. The briefly started twentieth qualification was stopped through its original SIGINT port before subsequent source refinements, and did not publish. The twenty-first qualification correctly rejected a new native interpreter-runtime owner and two live Cargo workspace declaration images. Its second candidate was stopped through the original SIGINT control before the separately discovered scalar-argument runtime correction; neither candidate published. That correction and its array-only contract are owned by the configured-input agent. A fresh graph and successful actual default Nx execution remain pending after that gate.

## Literal Nx Process Bridge

Neutral argument declarations now use literal arrays; the configured-input report owns the eight handcrafted declaration corrections and schema admission. Installed Nx itself joins argument arrays with spaces and executes through the system shell. Its direct path therefore loses empty/literal shell argument boundaries even when common authoring is correct.

The private generic Nx command executor keeps Nx's actual process lifecycle and delegates a fixed Bun bridge command. The bridge receives only a 64-character digest argument. Immutable source files carry a closed schema, exact source SHA256 and the original Script process wire hash. They use the caller's test output directory when present, otherwise the neutral build cache. Runtime reset removes that cache; files remain available for concurrently starting commands and continuous tasks. User paths travel through quoted private environment values, and large source/policy payloads do not enter cmd.exe command text. The bridge verifies actual file bytes and original invocation before Bun exec, whose portable shell receives the same POSIX literal serialization as Bazel. Original deadline, resource capabilities and cancellation are received unchanged. Owner/native arguments continue through their existing separate executor.

The actual pinned Nx resolver admitted the new executor registry entry, schema and implementation without a graph discovery. The private bridge passed its dedicated strict TypeScript check without diagnostics. Negative/control tests passed one test / 21 assertions: malformed digest, altered source bytes, wrong process wire, invalid closed source schema and expired deadlines are refused before execution. A real child received the identical original process wire/capabilities and was cancelled by the unchanged two-second deadline. The original private native executor gate passed four tests / 73 assertions. Actual installed Nx and real Bazel execution agreed on eight authored/default argument vectors plus original caller space, empty, shell, quote and boundary values (one test / 24 assertions). The final combined source gate is recorded with the next publication below.

Exact additional source attribution:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` (generic executor projection).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📤️arguments/🟨️.mjs` (generic literal command adapter; native route unchanged).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📤️arguments/📜️script.ts` (created private receiver, with no public developer command).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📤️arguments/🧬️schema/📥️source.json` (created closed source protocol).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📤️arguments/🧬️schema/🔣️command.json` (created Nx command adapter schema).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🏃️execution/🟨️.mjs` (created registered generic executor module).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️executors.json` (registered command adapter).

Existing graph test/fixture changes above include the bridge fixture and independent schema/hash/control laws. The configured-input agent owns the actual Nx/Bazel runtime parity fixture changes in the existing Bazel test/fixture files. Native Windows runtime was not executed on this host; no cmd.exe literal-quoting fidelity is claimed.

The private Nx projection additionally hashes the exact 32-file executor/bridge/schema/registry and owned process-helper import closure, plus the actual Nx/TypeScript dependencies and Node/Bun runtime versions. Its string-only internal parser port uses the existing installed TypeScript compiler and never acquires a second workspace graph. Explicit target inputs remain; absent inputs retain Nx's effective `default` and `^default`. Per-invocation source files and process wire values do not enter task identity. The actual native Nx HashPlanner/TaskHasher test was run red before the correction: changed bridge bytes left the key unchanged. Green afterward (one test / five assertions), bridge bytes and Nx package changes alter the key, while changing wire staging preserves it. Actual Nx/Bazel eight-vector argument parity was rerun after this projector correction and passed one test / 24 assertions.

A concurrent cache-budget implementation moved the public graph path into its fixed partition. The retained qualification harness and two test fixture readers now use the public `buildGraphPathV1` boundary. The initially queued twenty-second process had loaded the historical end-summary reader, but naturally exhausted its unchanged twenty-minute deadline before reaching discovery or that reader. It produced no graph or summary. Read-only queue records, resource hashes, lsof and process state proved it waited behind the active independent workspace-release graph producer, with original cancellation/deadline intact. No peer lease was altered and no completed release workflow held that writer: the peer had active inference record descriptors, no child task, and no current authority receipt. A new admission waits for actual holder release or a current receipt.

The newly enforcing fact-store API also changes isolated inference fixture requirements. Its native image/queue namespace uses the same workspace argument as logical source discovery; there is no separate operational cache-root field. The existing ticket fixture example has 150 UTF-16 units for its workspace plus 24 for its lease directory, 71 for resource hash/queue and 47 for a minimal native image ticket, producing a 292-unit materialized path (286 Unicode scalars), exceeding the same enforced 256 limit. No temporary alias, symlink, outside-ticket source copy, or storage bypass was introduced. The bounded read-only manifest-walk cancellation law passes independently (one test / 21 assertions); primary owner-loop cancellation and configured-source actual inference retain their separate required evidence during fixture adaptation.

Read-only stage qualification retains both prior cancellation laws: production invokes the extracted native owner loop, and its seventeen-owner fixture aborts at the same first sixteen-owner continuation under `build:cargo-owners`, before fact writes. The distinct manifest walk retains its own `build:manifest-discovery` cancellation law. Together with the independent engine-reference parser laws, the focused gate passed four tests / 58 assertions. The configured command source law uses the exact production normalizer's existing source representation and retains all earlier execution, schema, Nx, TypeScript and esbuild assertions; it additionally verifies no cache directories were created (one test / 189 assertions). This is explicitly read-only source inference; full storage admission remains the public qualification's responsibility.

Current artifact fixtures use the real operational repository store and a declared original publisher invocation through `withRepositoryBuildGraphArtifactV1`; three tests / 19 assertions passed, covering changed publication refusal, ordinary metadata and compact token expansion. The retained `qualify/📜️script.ts` now uses that same artifact callback inside its still-live original invocation, and derives canonical and receipt paths exclusively through `buildGraphPathV1`. It never accepts the historical cache path as current evidence. The independent writer exited naturally; no peer process or lease was changed. Final publication waits for the controlled reset and canonical/store-receipt filename separation regression.

The initial GUI resource fixtures were handcrafted to the current `.cargo/config.toml` shared target declaration, including Windows/POSIX serialized expectations. Handoff checks retain exact resources/policy and verify a live deadline no later than the original; the process transport computes `now + floor(original remaining)`, whose monotonic bound may shorten the serialized wall deadline by one millisecond. The test's log observer now ignores storage provisioning debug messages emitted before any invocation wire exists.

The GUI law's current isolated logical workspace now declares its own Cargo target in `.cargo/config.toml`, checked through the independent TOML parser. It does not inherit the repository's physical Cargo declaration implicitly. Both GUI authority and portable serialization laws pass (two tests / 82 assertions). The first combined suite exposed only fixture integration defects; final combined acceptance is deferred until the controlled reset source lands. An unrestricted workspace whitespace check reported four unrelated Rust EOF blank lines, which were left untouched; the parent owns the task-scoped inventory check.

After controlled reset and canonical filename separation landed, the coherent combined graph suite passed 42 tests / 715 assertions in 9.53 seconds. The three registered GUI declarations in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` now match the fixture's current relative Cargo target exactly; their command, launch selections and finite grants are preserved. A direct invocation of the registered package `📜️script.ts test build-surfaces`, issued through its original declared 120-second process envelope, passed ten tests / 27,323 assertions in 0.99 seconds. A bare receiver correctly refused the absent parent envelope first. Qualification twenty-three then started with the same original twenty-minute policy and two-invalidation guard; its current-only output is `🗑️generated/qualified-graph-current.json`.

Qualification twenty-three initially queues before discovery behind a different active producer, PID2690 (`repo show projects`). Its current image lease, build-graph-state lease and active primary-inference pack descriptors were verified read-only. An external native test PID12803 briefly queued and then exited. The old PID23688 is absent. The current producer has not published `canonical.json` or `authority.json`; the 114-byte `graph.json` storage charge remains intact. Our original twenty-minute budget includes this queue wait and has not been restarted or extended.

A final private publication law ran red through actual plugin nodes/dependencies: an existing file was accepted without a digest. The private reader now admits only a closed explicit path/digest pair, an absolute host path and the exact mandatory lowercase SHA256 before JSON parsing. There is no historical cache fallback. The new contract is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🧬️schema/📥️publication.json`, read by the actual projector and included in its engine closure. The existing pure JavaScript owned contract validator keeps the Node-loaded plugin independent of TypeScript loading. Actual plugin negatives/success, Ajv/native SHA checks and the actual native Nx hasher law pass together (two tests / 26 assertions). Low source review is clean.

The caching oracle `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts` now reads canonical graph and its three compact source inventories through the public current graph/fact-location helpers. Its full graph-dependent oracle was not run by this focused source gate. One test initialization raced an incomplete concurrent cache-admission export; after that external source became coherent the focused laws passed without modifying that cache implementation.

Qualification twenty-three was cancelled through its own original SIGINT handler after 15 minutes 31 seconds, still before discovery. During its external queue wait, the common graph path implementation changed from a loaded literal to the shared declaration consumer. Its module observes file bytes later during discovery, so retaining that process could attest newer source bytes with older loaded code. No such receipt was accepted: it exited with `Original Script process interrupted`, produced no current summary/pin, and no peer was signalled. The next admission waits for actual peer release/current receipt and settled source validation.

The exact loaded-code gap is permanently closed. The common graph module captures SHA256 fingerprints for its own implementation and shared publication declaration at module evaluation. It checks both before storage custody, after queued writer admission, during cached admission and immediately before canonical renames. Source receipt protocol version three requires that closed module authority, and the exact production receipt-admission helper rejects older/unbound receipts or different loaded fingerprints before source-fact reuse. This prevents a previously started producer from presenting current on-disk bytes as its loaded behavior. Language-neutral source-change vectors, Ajv and independent native SHA checks prove both implementation and declaration changes refuse authority without copying or editing any live implementation. The existing authority schema is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authority.json`.

Neutral and private pinned reuse now share `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️pin.json`; the temporary private publication schema was consolidated into this owned common contract. Both require an explicit absolute path and mandatory exact lowercase SHA256 before reading the graph. Neutral `reuse` has no unbound canonical-file fallback. Actual common/plugin negative and success laws, loaded authority admission and original bootstrap fixture pass together (four tests / 69 assertions). The retained qualifier verifies receipt version/module bindings independently before retaining its immutable artifact.

Shared test inference `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` now derives its no-override path from the real declared `inference` namespace; both production nodes/dependencies call that helper. Its explicit override retains its owned namespace semantics. User-facing dependency/refresh errors now name the neutral build. The read-only namespace law passed (one test / four assertions), and enforcing writer custody remains unchanged.

Current managed dependency integration required truthful fixture changes. Bootstrap's injected installer now creates the same declared managed symlink/layout and is invoked on all four controlled synchronizations; missing/deleted dependencies and exact original invocation identity remain tested. GUI help-only handoff uses the real operational repository root, avoiding an impossible long native lease path in the isolated ticket workspace. Third-party glob/minimatch/Babel oracles resolve through the actual installed Nx SDK, and the replaced-ancestor law checks glob's empty result, actual `ENOTDIR` and canonical membership refusal. The surfaces oracle now declares `string-argv` 0.4.0 as a test-only dependency in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`; `bun.lock` was refreshed through the defining owned dependency module under real original storage custody. Its registered ownership router first refused a concurrent physical prepared-source change correctly; the defining module's original controlled lock/sync then succeeded, publishing 96 bindings after a 74-second install. No raw unmanaged installer or storage bypass was used.

Final coherent source gate passed 49 tests / 817 assertions in 10.42 seconds. Registered `test build-surfaces` preflight passed ten tests / 27,385 assertions in 1.285 seconds. Low source review is clean. External producer PID40489 has exited, with storage charge still valid and no current canonical/authority files. Qualification twenty-four started as a new process only after that meaningful release, with the original twenty-minute policy/two-invalidation guard and the required version-three producer binding.

Qualification twenty-four reached real producer work immediately: dependency synchronization/discovery and the original Cargo owner loop completed in 22 seconds; command analysis finished at 22.7 seconds. Its first candidate encountered a real concurrent procedural Cargo structural edit: the required new geometry owner was absent from the starting 1,240-owner assembly, membership changed, and three distinct consumed Cargo manifest SHA facts were recorded. At 77.45 seconds it issued one original refresh. The new module guard then refused the next attempt because the loaded graph implementation bytes had changed on disk; the publication declaration had not changed. No own source edit occurred after launch. The process exited before any canonical/receipt, providing actual runtime evidence that loaded-code authority cannot silently attest the newer source. Fingerprint refusal diagnostics now carry the exact loaded/current SHA pair for precise future provenance.

Final guard, pin and test-integration file inventory (repository-relative exact paths):

- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authority.json`.
- Created `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️pin.json`.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts`.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json`.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs`.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts`.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json` and root `bun.lock` for the test-only parser.
- GUI fixture alignment updated `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (only the three named rows in each).
- Created and then removed before final implementation: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🧬️schema/📥️publication.json` (consolidated into common pin schema) and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️module-authority.json` (consolidated into authority schema's module property).
- Shared engine-reference declaration source attribution: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️engine.json`.

The retained authored qualifier remains `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/qualify/📜️script.ts`. Its new quiet-source mode imports no producer, keeps an original 180-second maximum with cancellation/progress, and requires graph implementation/publication/invocation/build-router/admission API sources to remain byte-identical for thirty seconds with expected exports present. The pre-twenty-six gate passed after 30.093 seconds, and the current combined suite passed 51 tests / 888 assertions in 16.31 seconds. Qualification twenty-six then started immediately as a new process with unchanged twenty-minute/two-invalidation controls. Source code remains frozen within this task while it runs.
