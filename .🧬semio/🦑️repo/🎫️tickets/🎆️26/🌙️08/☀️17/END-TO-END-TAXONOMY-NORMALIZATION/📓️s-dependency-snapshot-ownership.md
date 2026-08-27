# Dependency Snapshot Ownership and Projection

## Exact Read-Only Finding

`🔒️dependencies.json` is authoritative, manually ratcheted policy state even though commands generate its serialization. Refreshing it automatically would approve new dependencies, not just update a cache. The read-only snapshot observed here has232 entries,146113bytes, SHA-256 `5e8b295a962aa774383862399561fdace5ee0ec714e817e7e871375f93fa692c`, schemaVersion2, generatedAt `2026-08-23T17:24:30.393Z`, commit `215e369d07d8014806a43f8f75a1bba3c6015908`.

The JCO coordinate is exactly `/entries/230/users/1`: the old JCO guest `Cargo.toml`. Its entry is `rust:wit-bindgen`, version0.57.1, kind `production-runtime`, productionReachable=true. The plugin and scale manifest users in the same entry must remain unchanged.

No production snapshot bytes, production manifests, or real Git state were modified. The extra read-only JCO global scan was stopped on root coordination to reduce memory/CPU pressure: verified owned PID63699, SIGINT, exit130 at10100/70881 candidates; no plan artifact was published. Future apply-authority capture must use the actual ticket admission, not the diagnostic no-ticket invocation.

## Two Distinct Writers

| Writer | Inputs and effects | Normalization suitability |
| --- | --- | --- |
| Root `VerifyScript.run` → `runDependencyFreeze` → `dependencyFreezeWriteBaseline` in `📜️script.ts` | Recomputes all five ecosystem inventories, writes schemaVersion2, wall-clock generatedAt, current Git HEAD, current entries. Nx target `verify-dependencies-freeze-write-baseline` invokes `bun ./📜️script.ts verify dependencies write-baseline`. | Not suitable: silently adopts newly present dependency identities and changes metadata. |
| Test-domain `DependencyScript.run(write-baseline)` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` | Reads existing baseline, classifies it against the oracle registry, preserves top-level metadata, rewrites entries with derived classifications. | Not suitable: does not rederive manifest users and may change classifications outside the requested path projection. |

Root `dependencyFreezeCheck` uses baseline `(ecosystem,name)` identities as an allowed-set ratchet: additions fail, removals pass. Test-domain `loadClassifiedBaseline` feeds dependency checks and coverage metrics. Direct production code references found by a source-only search were these root/test-domain readers and writers; other source references were test fixtures. JSON oracle rationale mentions are prose, not additional writers.

## Root Writer Input Authority

- Rust: recursively discovered Cargo manifests plus root `Cargo.toml` workspace dependencies; manifest path/version/section determine user, version, classification, and first-party filtering.
- JavaScript: recursively discovered package manifests, all their package names for internal identity, and dependencies/devDependencies/optionalDependencies/peerDependencies.
- Go: `go.work`, its selected module manifests, module identities, local replace declarations and requirements.
- Python: recursively discovered pyproject manifests and supported dependency arrays.
- .NET: recursively discovered csproj PackageReference/IsTestProject data.
- Oracle classification: taxonomy location/contribution declarations, central oracle registry and discovered contribution manifests.
- Command prerequisite: the three exact repo policy routers and their owned library import target.
- Metadata: wall clock and read-only `git rev-parse HEAD`. The baseline writer does not use `bun.lock`; the separate truth/parity reports do.

This existing collector is not a ready pure-input generator: contribution discovery does not prune the two opaque paths; Go selection does not consistently reject the second opaque path; leaf reads use a catch-all reader rather than no-follow authority; traversal has no cancellation; ordering uses default-locale comparison. It was invoked only inside a small disposable fixture here. These findings do not authorize broad writer refactoring in this lane.

## Actual Writer Proof

`🔣️dependency-snapshot-writer.json` is the language-neutral observation matrix. `🧪️dependency-snapshot-writer.test.ts` invokes the actual imported root `VerifyScript` against an isolated ticket fixture, with the real JCO Cargo manifest and `@iarna/toml` as the independent dependency/version oracle. It copies only the three policy-router declarations and taxonomy, supplying an inert import-boundary file that is never executed. The fixture's empty Git repository deliberately yields commit=unknown; no real Git mutation occurs.

The writer prints its actual outputs:1 entry before relocation,1 after physical fixture relocation,2 after adding an unapproved fixture dependency. Both clock instants are controlled by the test. Exact command:

```text
bun test /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️dependency-snapshot-writer.test.ts
```

Result: **1 pass,0 fail,9 assertions,0.691s**. It proves physical-owner retargeting, metadata refresh, and automatic adoption of an unapproved identity. It does not claim the collector's live global enumeration is safe or complete for normalization.

## Approved Integration Direction

Root approved a bounded schema-owned **users-only authored-policy-state projection**, not a self-input generator and not invocation of either write-baseline command. Existing generator grammar explicitly rejects an output that is also an input; this must not be bypassed.

The new contract must bind the exact baseline path and JSON users coordinates to actual hash/size-proven moved Cargo manifests. Every old/new user token and containing baseline preimage must be exact. All other bytes—including generatedAt, commit, allowed dependency identities, versions, classifications, oracle metadata, unrelated users and formatting—remain unchanged. A source/canonical manifest pair must not introduce new dependency identities, omit an expected user, or ambiguously attribute a user to multiple owners.

TDD packet to implement next: exact JCO positive; changed/missing/duplicate users; unrelated same-token fields; ambiguous or unproved manifest owner; newly introduced dependency; no-follow/opaque rejection; source+baseline drift before mutation; rollback then same-ticket retry/commit; byte-preserved ratchet identity/metadata; empty second plan. Existing generic external-edit preimage protection and transaction WAL should carry the authored policy edit, with no alternate write path. The generated-consumer guard may be lifted only for tokens this exact state contract validates, never for the whole filename or arbitrary JSON.

## Implemented Bounded State Projection

`semanticPolicyStateCoordinateContracts.dependency-freeze-users-v1` now declares the exact state path, schemaVersion2, authored-policy disposition, users pointer, immutable remainder and nested-Cargo owner authority. This first contract is deliberately scoped to JCO's digest-bound `witDependency` evidence and exact Cargo manifest move. It does not implicitly approve WGPU's workspace dependency declarations. Discovery rejects changed path, pointer, package set or immutability fields.

Normalizer `dependencyPolicyStateTokens` validates the state shape, unique dependency identities and users, the source Cargo bytes/hash/size against both catalog and move preimage, and the approved dependency identity/version/runtime classification. Only one exact unescaped users token is actionable. Missing users, source/canonical ambiguity, changed dependency identity/version/classification, duplicate identities/users and non-regular state are rejected. A parsed-value census also rejects an extra source coordinate encoded with JSON escapes outside users. Other fields are not reserialized.

The normalizer's package-consumer guard is lifted only when that state proof returns the exact manifest source. The reference planner then admits only its exact token span/value and checks the baseline preimage again. Existing external-edit transaction authority supplies the complete baseline hash/mode/size. No separate writer, self-input generator, date refresh, Git-HEAD refresh, or baseline approval operation was added. Draw's existing explicitly declared consumers remain untouched; this is not a blanket whole-filename exclusion or a global policy gate.

The schema and parser were paired and freshly checked with `validateTaxonomy(loadTaxonomy()) === []`. No existing schema fields changed. The additional top-level property is ignored by the older loaded validator; operational normalizer schema shape was not changed. Schema/discovery edits were then frozen throughout root transaction11918, while only the bounded helper/tests continued.

## TDD and Transaction Evidence

The language-neutral contract/state/negative matrix is `🔣️dependency-policy-state.json`; runtime assertions are in `🧪️nested-cargo-package-integration.test.ts`.

- Initial schema-first red: **0 pass,1 fail,1 assertion,0.602s** (no state contract).
- After paired schema, the runtime remained red (**0/1/3,6.93s**): the generated-consumer guard still blocked the state, and the tiny positive fixture lacked a real JCO producer. The fixture now uses the same real root JCO producer registration as the lifecycle. An intermediate implementation referenced `preimage` instead of the existing move field `sourcePreimage`; it failed closed (**0/1/3,2.88s**, repeated after producer installation **0/1/3,3.90s**), and the exact field was corrected.
- First exact positive: **1/0/7,1.135s**, clean4-move/3-edit plan with one state users edit plus the two existing JCO edits.
- Initial positive/ten-negative/lifecycle packet: **3/0/64,18.53s**.
- Encoded undeclared-coordinate regression: **0/1/32,29.43s**, then **1/0/33,15.57s** after counting decoded values as well as requiring an exact raw users token.
- Final focused packet: **4/0/81,52.26s** under concurrent host pressure. It includes eleven invalid JSON states, four invalid contract mutations, symlink/directory state rejection, newly changed manifest dependency rejection and the real JCO transaction. A changed baseline commit field fails before apply. Injected post-generation failure restores the original baseline bytes. Same-ticket retry commits at ordinal000002; baseline bytes equal the original with only the single manifest path replaced. Identity/version/classification fields remain equal, Cargo metadata is preserved and the second plan is empty.
- The subsequent full21-case file completed **19 pass,2 fail,213 assertions,67.17s**. All policy/JCO cases remained green. Both failures are the independently changed WGPU browser-frame-transport test's source hash and the downstream guard test that cannot run after that authority rejection. The exact source diff/hashes are recorded in `📓️s-nested-cargo-package-integration.md`; no WGPU source or frozen authority was changed to hide that result.

Exact focused command:

```text
bun test /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️nested-cargo-package-integration.test.ts --test-name-pattern 'dependency policy|JCO transaction rolls'
```

The fixture retains the explicit unrelated plugin-registry isolation documented in `📓️s-nested-cargo-package-integration.md`; it does not claim live registry producer closure. No production apply follows from these fixtures. A fresh actual-ticket JCO plan remains deferred until the existing root/Draw global scans clear. WGPU dependency-state consumers, adapter/registration output producers, worker/boot generation and full reference/runtime closure remain unsupported by this bounded JCO state contract.
