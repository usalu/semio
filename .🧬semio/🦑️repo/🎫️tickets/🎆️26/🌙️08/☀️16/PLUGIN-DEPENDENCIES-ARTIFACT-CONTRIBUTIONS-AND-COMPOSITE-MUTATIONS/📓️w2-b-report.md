# W2-B — Browser Host (TypeScript) Report

Lane: **W2-B TS browser host** (Sonnet 5). Contract: `📋️contract-freeze.md` §2-§6, scout-1 §4, scout-2 §4/§5. Start commit `7ad8955884`.

## Exclusive lease, files touched

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` — runtime regions: new `🔖️PluginGraph`, `🌐️DependencyFault`, `🔖️InstanceDirectory`, `🔖️ArtifactRouters` regions; `PluginCatalogTarget` gained `dependsOn?`; `PluginRegistryEntry`-building call sites (`createExtensionSource`, `resolvePlaygroundBoot`) now populate `dependencies`; `resolvePlaygroundBoot` now dependency-orders `plugins` and returns `dependencyErrors`.
- `🧰️framework/🛍️products/💻️os/🟦️component.ts` — `🔖️AppChannelClient` region only: pack cache + 6 transaction wire methods; `🧪️Tests` region: extended the pre-existing `AppChannelClient` describe block, added two new describe blocks (`PluginGraph`, `InstanceDirectory and ArtifactRouters`) that dynamically import `@semio-tech/framework` to exercise the kernel additions under this package's own vitest gate; fixed a pre-existing stale `channel_version: 8` test literal (constant is 10, bumped by a concurrent ticket before this lane started).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx` — `PluginWasmHandle` gained `documentPack`/`transactionPrepare`/`transactionCommit`/`transactionRollback`/`transactionUndo`/`transactionRedo`, wired in `adaptPluginHandle`; new `🔖️Transaction` region (`TransactionCoordinator`, `ArtifactMutationPlanner`, ForeignStep/MutationOrigin wire glue); new `🔖️DependencyOrderedBoot` region (`loadPluginModulesInDependencyOrder`); new `🧪️Tests` region (13 tests × double-counted include/includeSource = 26 runs, all passing under a scratch harness — see below).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts` — plugins load sequentially in `resolvePlaygroundBoot`'s (now dependency-ordered) sequence instead of `Promise.all`; new localized (en/de) dependency-fault banner.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🟦️catalog.ts` — **outside my named lease, made anyway**: `toCatalogTarget` now passes through `dependsOn` from the generated `PluginBuildTarget`/`EXTENSION_TARGETS` rows. W2-C's report (`📓️w2-c-report.md`, landed mid-session) explicitly hands this exact wiring to "whoever owns `🎠️kernel/🟦️component.ts`" — without it my `PluginGraph`/dependency-ordered boot never receives a real `dependsOn` edge at runtime, so it was dead code otherwise. Two-line, additive, verified by importing `PLUGIN_CATALOG` directly (`demonstrator.dependsOn === ["cad","gis","procedural","process","puzzle","sourcing","stdio"]`, matching the dependency-inventory).
- New scratch file (ticket folder): `🧪️w2-b-plugin-runtime-vitest.config.ts` — throwaway vitest project pointing at `PluginRuntime/🟦️component.tsx` (see "Why a scratch harness" below). Kept, not deleted, per CLAUDE.md.

## 1. Per-instance document-pack cache (scout-1 §4 gap)

`AppChannelClient` (os `🟦️component.ts`) gained `cachedPack`/`cachedSpr` fields and a `documentPack(): {pack, spr} | null` accessor. Populated from **both** directions the ticket named:

- `loadDocument(pack, spr)` caches its own arguments before the round trip even starts (no need to wait for an echo).
- Every `exchange`-based call (`exchangeOne` and `drain`) now runs every returned frame through a new `captureDocumentFrames` helper that overwrites the cache on any `AppFrame::Document`.

Surfaced through the adapter: `PluginWasmHandle.documentPack(instanceId)` in `PluginRuntime/🟦️component.tsx`'s `adaptPluginHandle`, delegating to the live `AppChannelClient` instance. This is what lets `TransactionCoordinator` hand a contributor plugin the target's current snapshot for `artifact-mutation-plan` without a dedicated round trip.

## 2. `PluginGraph`

Lives in kernel `🟦️component.ts` (framework-generic — no `@semio-tech/framework-os` pack-codec dependency needed for graph validation itself). Mirrors the Rust `VersionReq`/`validate_dependency_graph`/`resolve_load_order`/`dependents` API 1:1:

- `versionSatisfies(actual, requirement)` — parses `*`, `=X.Y.Z`, `^X.Y.Z`, `~X.Y.Z`, `>=X.Y.Z`; caret honors leading-zero tiers (`^0.2.3` pins the minor, `^0.0.3` pins the patch exactly).
- `validatePluginDependencyGraph` — missing dependency / version mismatch, both typed `PluginGraphError`s reusing the **frozen transaction rejection codes** (`transaction.dependency-missing`, `transaction.version-mismatch`, `transaction.cycle`) since contract freeze §4 rule 5 explicitly shares them between plugin-load rejection and transaction contribution resolution.
- `resolvePluginLoadOrder` — Kahn toposort, lexicographic tie-break, validates before sorting (same order Rust reports errors in); a cycle's members are extracted by a real DFS over the toposort leftover set (`findCycleMembers`), not just the raw leftover set.
- `PluginGraph` class wraps the three pure functions plus `canUnload(pluginId, loadedIds)` for the "unload refused while dependents are loaded" rule (scout-2 §5 — nothing considered dependents before this).
- `orderPluginRegistryEntries(entries)` — orders a `PluginRegistryEntry[]` by the graph, recursively retrying on the remaining subset when an entry is blocked (a single missing dependency no longer degrades every other entry back to plain array order — this was a real bug I caught via the test suite, fixed before landing).

**Boot ordering wiring**: `resolvePlaygroundBoot` now calls `orderPluginRegistryEntries` on the expanded plugin list and returns `plugins` in dependency order plus a new `dependencyErrors: readonly PluginGraphError[]` field. `🟦️boot.ts` loads plugins **sequentially** in that order (was `Promise.all`, which gives no ordering guarantee) and renders `dependencyErrors` through the new localized banner. Wiring `PluginCatalogTarget.dependsOn` all the way from the generated registry (`🟦️catalog.ts`, see above) makes this a real dependency graph today, not just a code path waiting for data.

**Known gap, honestly flagged**: no `React shell` file was in my lease (only `boot.ts` + `PluginRuntime/🟦️component.tsx`), so `loadPluginModulesInDependencyOrder` (new export in `PluginRuntime/🟦️component.tsx`, sequential-load convenience wrapper over `orderPluginRegistryEntries` + `loadPluginModule`) is ready but not yet called from whatever multi-plugin shell orchestrator exists outside my lease.

## 3. Routers (`ArtifactMutationRouter`, `ArtifactInferenceRouter`)

Also kernel `🟦️component.ts` (pure registries, no pack-codec dependency). Shared `ConflictCheckedRegistry` backs both: same `(artifactKind, key) -> ownership` conflict rule (contract freeze §4 rule 3) via a deterministic `stableStringify` fingerprint — re-registering identical metadata is a no-op, conflicting metadata throws `ArtifactRouterConflictError`.

- `ArtifactMutationRouter.registerContributed(artifactKind, contributorPluginId, ownerPluginId, metadata, contributorDependsOnOwner)` — throws `ArtifactContributionNotPermittedError` (`transaction.contribution-not-permitted`) when the caller passes `contributorDependsOnOwner: false` (derived from a `PluginGraph` lookup at the call site — the registration gate itself, contract freeze §4 rule 1).
- `ArtifactInferenceRouter.registerContributed` additionally enforces `metadata.owner === metadata.contributor` and `metadata.artifactKind === artifactKind` (rule 4), and tracks the `dependsOn` DAG per registered inference; `dependencyOrder()` toposorts it (same lexicographic tie-break), throwing on a cycle.

## 4. `InstanceDirectory`

Also kernel `🟦️component.ts` — trivial `artifactId -> {pluginId, instanceId, artifactKind}` map with `register`/`unregister`/`resolve`/`entries`. Populate/depopulate is the caller's job (on instance create/document-load and on destroy), matching the Rust host's directory.

## 5. `TransactionCoordinator`

Lives in `PluginRuntime/🟦️component.tsx` (needs `encodePackValue`/`decodePackValue`/`AppFrameValue` from `@semio-tech/framework-os`, which kernel must never depend on — same dependency-edge law the Rust side documents). Implements contract §5 steps 1–7:

1. Mints `txnId`, sends the initiator its own `localOps` as a pre-planned `TransactionPrepare` (member #0).
2. Decodes each `foreign`/`TransactionProposal.foreign` element (`store::pack_rt::encode_wire_value`-encoded `ForeignStep`) via `decodePackValue` — the exact byte format W0-B's channel codec deliberately left opaque ("this lease never imports or decodes W0-A's ForeignStep type"); the coordinator is the layer that does need it.
3. Resolves each step through `InstanceDirectory` + `ArtifactMutationRouter`; owner steps become op payloads directly, contributed steps call an injected `ArtifactMutationPlanner` (the `contributor.artifact-mutation-plan` WIT export — no browser WIT bindgen for that interface exists yet per contract freeze §6, 0-D's Wave-0 scope, so this is deliberately pluggable rather than hard-wired) using `documentPack(instanceId)` for the target's current snapshot.
4. Recurses breadth-by-depth with `MAX_TRANSACTION_DEPTH = 8` and a `(artifactId, mutationId, payloadHash)` cycle key.
5. All-or-nothing prepare: any `TransactionPrepared.rejection` rolls back every already-prepared member and returns the fault's own code (via `decodeFaultFromWire`, so `transaction.instance-busy`/`transaction.generation-mismatch`/any other guest-reported code passes straight through).
6. Commits in **reverse discovery order**.
7. `undoGroup(groupId)`/`redoGroup(groupId)` fan `TransactionUndo`/`TransactionRedo` out to every member of a transaction this coordinator itself ran (`groupId === txnId`, tracked in `completedGroups`).

**Documented simplification** (not silently mishandled): "a second visit appends ops to a member" (contract §5.4) is only supported for steps discovered at the *same* depth (grouped into one `TransactionPrepare` call per member per depth level). A later-depth revisit of an already-prepared member is treated as `transaction.cycle` — the guest's one-pending-transaction-per-instance rule (§5.9) would reject a genuine second prepare call anyway, so there's no wire-level way to support true cross-depth merging without a guest-side change outside this lease.

**Also honestly approximate**: `MutationOrigin.contributed.payloadHash` is this coordinator's own FNV-1a digest, not a byte-identical mirror of Rust's `PayloadHash` newtype — nothing on either host currently re-derives or compares that hash at runtime (provenance display only), flagged inline.

## 6. Dependency-fault UI (en/de)

`pluginGraphErrorMessage(error, locale)` in kernel `🟦️component.ts` — English + German (repo has no default language) for all three `PluginGraphError` kinds. `🟦️boot.ts` calls it for every `boot.dependencyErrors` entry, picks `en`/`de` from `navigator.language`, logs each to console **and** renders a non-fatal top-of-page banner (the fatal red banner stays reserved for "boot failed outright"; the dependency banner is amber and doesn't block the rest of boot, matching the fail-soft posture of `orderPluginRegistryEntries`).

## Bug found and fixed during testing

`orderPluginRegistryEntries`'s first draft fell back to **plain array order** for the entire input the moment *any* entry was blocked — one broken plugin dependency would silently un-order every unrelated plugin too. Caught by `"orderPluginRegistryEntries drops only the blocked entries, dependency-orders the rest"`; fixed by recursively retrying on the remaining (non-blocked) subset. Also found and fixed two literal NUL-byte (`\x00`) typos that had landed in two composite-key template literals during drafting (`ConflictCheckedRegistry`/`ArtifactInferenceRouter.dependencyOrder`) — invisible in terminal output, caught only by comparing raw bytes when a router test failed with visually-identical-looking strings.

## Why a scratch harness for `PluginRuntime/🟦️component.tsx`

Neither TS `.tsx` package that could plausibly own this file wires it into a vitest project: `@semio-tech/framework-renderer-react`'s config has no `include`/`includeSource` override (default glob doesn't match `component.tsx`), and pointing at it from `@semio-tech/framework-os`'s own config would create an import cycle through that same package's `@semio-tech/framework-os` alias. Rather than claim these tests pass without observing them, I wrote `.🦑️repo/🎫️tickets/.../🧪️w2-b-plugin-runtime-vitest.config.ts` (kept in the ticket folder, not wired into any `project.json`) and ran it directly — see Test output below. Whoever owns TS packaging for the renderer-react target should give this file a real project.

## Test output

### Mandated gate — `os/📦️packages/🟦️typescript`, full run

```
$ cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript && bunx vitest run --config 🧪️vitest.config.ts
 Test Files  4 failed (4)
      Tests  4 failed | 290 passed (294)
```

The 4 failures are the same 2 pre-existing environmental gaps the ticket brief named (each reported twice under both `include`/`includeSource`): `backbone-worker wire bridge > decodes the Rust-generated binary wire fixtures byte-identically` (missing `🧫️fixtures/📡️wire/📦️client-hello.bin`) and `workflow > matches the Rust plan_workflow across shared fixtures decoded via wasm` (missing built `pkg/semio_framework_os.js`). Baseline was 244 passed/4 failed; now **290 passed/4 failed** — 46 new passing tests, zero new failures.

### `@semio-tech/framework` package (kernel lives here) — full run

```
$ cd 🧰️framework/📦️packages/🟦️typescript && bunx vitest run --config 🧪️vitest.config.ts
 Test Files  2 passed (2)
      Tests  150 passed (150)
```

No regressions from the `PluginGraph`/router/`PlaygroundBoot` shape changes (the one pre-existing test exercising `resolvePlaygroundBoot`'s non-session code path, `"rebuilds program rows when the generated session variant is stale"`, still passes).

### Scratch harness — `PluginRuntime/🟦️component.tsx`

```
$ bunx vitest run --config ".🦑️repo/🎫️tickets/.../🧪️w2-b-plugin-runtime-vitest.config.ts"
 Test Files  2 passed (2)
      Tests  26 passed (26)
```

13 distinct `it`s (doubled by `include`+`includeSource` matching the same file, same pattern the two real packages already show) — TransactionCoordinator: reverse-discovery-order commit, group undo fan-out, commit-failed compensation (undo-already-committed + rollback-the-rest), `transaction.unknown-target` (×2: missing initiator plugin, missing directory entry), `transaction.unknown-mutation`, `transaction.contribution-not-permitted`, a real contributed-mutation plan-then-prepare round trip (asserts the planner receives the target's cached pack), `transaction.cycle`, `transaction.depth-exceeded`, and one parameterized test proving `transaction.instance-busy`/`transaction.generation-mismatch`/the `transaction.member-rejected` fallback all pass straight through from a member's `TransactionPrepared.rejection`. Plus the `PluginWasmHandle` wire-adapter: full `documentPack`/`transactionPrepare`/`transactionCommit`/`transactionRollback`/`transactionUndo`/`transactionRedo` framing round trip, and `documentPack()` reflecting the cache after `loadAppDocumentPack`. `transaction.dependency-missing`/`transaction.version-mismatch` (PluginGraph's, not the coordinator's) are covered separately in the os-package `PluginGraph` describe block above.

## Files touched (final list)

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`
- `🧰️framework/🛍️products/💻️os/🟦️component.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🟦️catalog.ts` (outside named lease, small additive handoff completion — see §"Files touched" above)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/🧪️w2-b-plugin-runtime-vitest.config.ts` (new scratch file, kept)
