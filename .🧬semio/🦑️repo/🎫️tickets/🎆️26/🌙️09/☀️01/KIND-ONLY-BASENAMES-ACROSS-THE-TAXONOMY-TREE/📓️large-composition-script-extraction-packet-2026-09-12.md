# Large Composition Script Extraction

This queued work is two separate Sol lanes after the current font/assets, manifestless plugin-build and descriptor/registry API slices. Root re-read the actual public APIs, source regions and static/dynamic consumers on 2026-09-12. The OS dev command source is roughly 5,685 lines and the Hub Rust package command source roughly 17,245 lines; line counts are triage only. These sources combine runtime/build behavior, verification libraries and command routing. Re-read live sources and preserve concurrent feature work. Never use a whole-file rewrite from an old snapshot.

## OS Dev Domain Boundaries

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`.

Extract incrementally into existing neutral owners and narrowly named child concerns, each with anonymous TypeScript leaves. Do not move the entire monolith into an undifferentiated helper. The current coherent regions are:

- Plugin catalog/build/materialization/watch and extension publication: component profile/arguments, descriptor staging, pipeline concurrency, target selection, complete-catalog checks and watcher behavior. Preserve the underlying plugin build owner being moved by the manifestless-source lane.
- Browser-host staging: `TestBrowserHostStageInputV1`, bounded regular-file ownership, exact Space descriptor arguments, selected GIS byte identity, private staging, fresh Space materialization and atomic closure. Existing owner `dev/♻️activation/🌐️browser-host` already holds receipts and roots. Place additional behavior under the appropriate staging child; do not duplicate its existing contract.
- Engine preparation and activation: session-linked engine selection, toolchain prerequisites, component activation, source digests, daemon readiness and finite build leases. Existing activation/source owners remain authoritative.
- Verification: capability/layering/export-path/host-handle checks, Studio and collaboration scenarios, catalog smoke, renderer parity, fixture scale generation and measurement reporting. Each is a separate existing test/verification concern. Pure owned pixel comparison is algorithm implementation and belongs to the parity concern, not to a generic command helper.
- Distribution: `DistributionBundlePlan`, source traversal and witness collection, symlink/ancestor checks, compiler invocation, manifest preflight, publication/recovery and stale ownership. Existing `dev/🚚️distribution/🟦️.ts` is the contract owner. Extend its semantic builder/publication concerns; retain the preview protocol and byte freshness.
- Contract compilation and canonical-bootstrap-folder-mirror checks. Keep Ajv behind the existing owned predicate API.

The command entry must retain routing and delegation. Source-bearing class method bodies count as implementation even if the class extends BundleScript. Existing local argument parsing or CLI spawning is not permission for arbitrary hidden domain computation.

## OS Dev Consumer Closure

The following public exports require concrete changes to their consumers, not facade exports in the script:

- `stageTestBrowserHostV1`: Hub dynamically imports the dev script around its selected browser-host process proof. Hub also asserts the staging call and receipt relationship in source. Read and rebase those source contracts to the real owner. The dev `ticket-owned-browser-host-staging` test currently slices the command text between local materializer/staging function names.
- `ensurePluginRegistry`: the registry playground-session test reads its function body from the dev command source. The session fixture's `producerScript` remains a command entry only if it means executable routing; add or update a distinct source owner contract when the test means behavior.
- `materializeDistributionBundle`: the command starts a separate Bun eval process which dynamically imports `import.meta.url` to avoid the current Vite configuration cycle. After extraction, use the real neutral module URL and prove that the cycle remains broken. Preserve preview/check/generate outputs, literal error behavior and publication partitions.
- `syncBuiltExtensionsToInstallRoot`, `buildPlugins`, `buildPluginsStreaming`, `linkedSessionEngines`, `buildEngineWasm`, staged module facts/verdicts, `DEV_SCHEMA_URL`, and `devContract`: inspect direct and test registration use before extracting.
- `blake3Hex`/`Blake3Hasher` is only a facade over `framework/modules/hash/🟦️.ts`. Remove the script reexport and import the existing hash owner directly at all consumers. Root found live imports in Hub (two), OS root Rust command source, OS host Rust command source (two), store initial-child-identity test, descriptor source and replication Rust command source (two). These are distinct from executable references to the dev script, which remain valid.

The final `import.meta.vitest` block supplies a broad explicit dependency object to the moved test owner. Refactor that seam with the actual owner exports and native tests. Do not import Node/build/test dependencies into browser facades. Keep canonical playground session output bytes and mtime isolated with the completed session test contract. Shard worker output identity is queued in metadata; use the fresh producer path and preserve actor URL/staging consumers.

## Hub Domain Boundaries

Source: `🌎️hub/📦️packages/🦀️rust/📜️script.ts`. This TypeScript executable is under a Rust package because it routes Cargo; its many implemented domains still require neutral owners. The existing Hub native/bootstrap source was already moved by the OS source lane and must not be overwritten or duplicated.

First extract concrete public APIs and their local dependency closures:

- `HubFixtureExpectationV1` and `assertHubFixtureExpectation`: fixture admission stage/result checks shared across multiple proofs. Place under Hub verification/expectations, with the existing schema/hostile vectors.
- `orderedDirectoryPublicationOracle`: directory publication proof, exact fixture contract and native consumer.
- Native/MCP credential delivery: `deliverNativeCredentialEnvelope`, `deliverMcpCredentialEnvelope`, child environment sealing, frame/FD transport and cancellation/exit semantics. Preserve the shared transport and own its public interfaces. Runtime behavior must remain covered by native child proofs, not just source string checks.

Then split local composition by its actual domain: authenticated local admin/browser relays; framed local bootstrap and readiness; MCP workspace/checkpoint publication process proofs; credential rejection/transport proofs; browser document opening; admin authority/revocation; native codec/provider receipts; selected execution-target publication and browser process staging. The registered classes and named native law lists provide bounded seams for each move. Do not turn a single command source into another anonymous monolith that mixes these concerns.

Hub has source-as-data assertions reading its own command file at several verification sites (at review around lines 8887, 9047, 9295, 13509, 15012 and 16667). Inspect the assertion's meaning. When it constrains behavior, update it to read the real neutral owner and validate consumer registration. Do not merely weaken string assertions until the test passes. Existing source hashes are provenance unless they bind protocol or generated bytes.

The descriptor/registry and app-verification lanes own API imports currently coming from other command files: fresh component production, registry/session projections, GIS cold-map and VCS native-codec proof functions. Re-read the agreed owner map after those lanes; no legacy command-module facade.

## Validation And Attribution

For each bounded extraction, record schema/portable vectors first, installed native/third-party verification, exact source/consumer ownership and ancestor registration, registered Nx target results and observable runtime output. Keep signal/deadline/progress behavior and ordinary launch/project routes. Use ticket-generated artifact roots and the existing process cancellation infrastructure; no broad process kills, shared Nx reset or global session fixture mutation.

Distinguish tested pure checks, exact native law execution, component compilation and browser process acceptance. Existing self-source assertions can expose feature drift; retain specific failed contracts without inventing missing behavior or unproven baseline attribution. Report every exact changed path per bounded batch so final ticket attribution can be a union of owned maps rather than the shared Git diff.

Current native checker-bound API closures are recorded in 📓️composition-current-declaration-closures-2026-09-12.md: OS dev5,690 lines/379 statements/998 local edges and Hub17,239 lines/435 statements/1,526 edges, zero parser diagnostics. Seven explicit seed closures are bounded without treating remaining proof/command class bodies as already extracted.

Current ordinary configuration consumer findings are retained in `📓️current-vitest-configuration-consumer-baseline-2026-09-12.md`: Hub admin test/setup selectors climb one parent above their existing semantic owners; OS MCP does the same for its test glob; WGPU package-specific worker/preview test argv and configuration still select package-local cases while the named case owners live under renderer engine. Preserve these explicit existing command consumers during the corresponding composition/native-proof extraction; loaded configuration alone is not runtime suite acceptance.

## Coordinator Selector Closure — 2026-09-13

Preserve the eight exact coordinator product paths in `📓️coordinator-test-selection-consumer-repair-2026-09-13.md`. Admin current Node command owner/environment setup and test discovery are repaired; actual Nx18pass. MCP includes six native-case owners, keeps the core in-source consumer and excludes only its import-only helper; focused core8pass and inference-process argv points to the current anonymous source. Remodel selects its schema suite and all134 fixture consumers/35 mutation tags at their actual subset paths; restored full suite has1023pass/330 current parity failures, with no missing-path/module errors. Do not redo these selector repairs or weaken domain expectations to hide the exposed parity result. Anonymous configuration filename/editor work and the remaining Animate/2D/WGPU consumers are separate pending tasks.
