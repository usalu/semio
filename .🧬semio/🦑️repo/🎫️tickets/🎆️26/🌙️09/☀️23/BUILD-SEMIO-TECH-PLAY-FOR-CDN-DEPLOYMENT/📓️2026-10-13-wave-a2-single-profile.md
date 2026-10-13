# Wave A2: Single Shipped Profile, Semantic Descriptors, Receipts

Lane B of the scalable build architecture (`📓️2026-10-13-scalable-build-architecture.md` §10 A2, design §5.1–5.3).

## Outcome

| Item | State |
|---|---|
| One shipped profile `wasm-release`; `wasm-dev`, `component-dev`, `materialize-dev`, `support-dev`/`support-release` deleted (no aliases) | done |
| `wasm-iterate` local profile (single plugin, `dist/components-iterate`, 512 MiB cap) | done |
| Committed descriptors are semantic only; byte identity lives in `ComponentReceiptV1` | code, schema, fixtures and tests done; 3 of 70 committed pairs regenerated from release bytes (bim, animate, architect), the fleet run waits for the A1 driver pass (see Verification) |
| `ComponentReceiptV1` schema, Python-oracle fixture, language-agnostic tests | done (29/29) |
| describe, provenance, receipts, trusted catalog act on the shipped bytes | done |
| os-dev activation, hub preflight/publish, registry rebuild/projection/verification, plugin build plan/freshness | done |
| Launch entries | done (`.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json`) |

## Design

### Two descriptor forms

- **Semantic descriptor**: the guest's own `describe()` answer. `PackageDescriptor.hashes` is optional on the wire. `PackageHashes::is_unbound` skips it in serde and in the first-party value codec. Committed pairs, guest answers and the registry (`CompiledComponentOwnerV1`, `DeployedRegistryEntryV1`, `PackageDescriptorV1` in the descriptor-verification schema) carry no `hashes`. A carried binding is refused.
- **Bound descriptor**: semantic plus `hashes {wasmSha256, coreWasmSha256, descriptorSha256}` (two-pass self-hash). `bindDescriptorPackV1` in `🌐️browser-bundle/🛂️descriptor` is the only producer. It binds only where bytes are deployed:
  - the module page (materialize);
  - the hub fresh producer (`captureFreshComponentInputs`);
  - fresh catalog rows (`createFreshCatalogCommitMarker`).

  The runtime protocol is unchanged: hosts keep receiving bound descriptors.

### ComponentReceiptV1 (`🔌️plugin/🖨️describe/🏷️component-receipt/`)

- **Fields**:
  - `schema`, `pluginId`, `packageId`
  - `profile`, `target`
  - `wasmSha256`, `wasmBytes`, `semanticDescriptorSha256`
  - `toolchain` (`rustc X (40-hex commit)`)
  - `sourceTreeDigest`, `provenanceSha256`

  Both provenance digests are derived from the cargo provenance receipt of the build. Paths are normalized to `$BUILD`/`$CARGO_HOME`/`$REPO`/`$HOME`, and rows are ordered by UTF-8 bytes, so the digest is independent of checkout location.
- **Profile policy**: lives in the schema (`$defs.ComponentProfilePolicies`).

  | Profile | Directory | Cap | Shipped |
  |---|---|---|---|
  | `wasm-release` | `dist/components` | 256 MiB | yes |
  | `wasm-iterate` | `dist/components-iterate` | 512 MiB | no |
- **Receipt file**: `<crate>/dist/components/🧾️receipt.json`, canonical JSON, written atomically. Parsing is strict: canonical form only.
- **Oracle**: `wave-a2-single-profile/🧫️receipt-fixture.py` computes the fixture independently. The TS tests use the locked Ajv as the schema oracle.

### Targets (inferred per component crate, `📚️library/🟨️.mjs` `componentTargets`)

| Target | Cache | dependsOn | Output |
|---|---|---|---|
| `component` | false | `workspace:release` (A1 driver) | `{projectRoot}/dist/components/<crate>.wasm` |
| `describe` | false | `component`, `os-plugin-describe-rs:build` | `{projectRoot}/dist/components/🧾️receipt.json` |
| `materialize` | true | `describe`, `framework-plugin-web:support`, `workspace:deps-wasm-opt` | the module directory of the single `dist/🔌️plugin-modules` root |

- `describe` runs `ComponentScript`. It does a narrow cargo build with the release driver's exact unit shape (`-p <pkg> --lib --target wasm32-wasip2 --profile wasm-release`, `CARGO_RESOLVER_FEATURE_UNIFICATION=package`), so after a driver pass it is a fresh check. It then describes the bytes, refuses any difference from the committed pair with a canonical path diff, and writes the receipt.
- `materialize` checks the receipt against the bytes, transpiles (wasm-opt only for shipped bytes), and runs the module's own `describe()` under JSPI. The answer must hash to `receipt.semanticDescriptorSha256`. It then binds and copies the receipt into the module directory. Production distribution refuses a module whose receipt is not shipped.
- The `component` output names only the wasm file. The policy-check found 70 CACHE-03 overlaps with `describe` when it named the directory. They are gone after the fix: 544 → 473 CACHE-03, the remainder unrelated.

### Root `workspace:components`

- `components describe <ids…>|--all [--profile wasm-release|wasm-iterate]` is `describe --write`. It rebuilds, rewrites changed committed pairs and writes receipts.
- `components iterate <id>` is the os-dev single-plugin loop. It builds at `wasm-iterate` and does a local describe: drift is reported, never committed.
- `components profile-independence [--sample n] [--seed n] | ids` requires the semantic sha to be equal under both profiles. The sample rotates daily by seed.

### os-dev, hub, registry

- **os-dev**:
  - One plugin-modules root (`pluginModulesRoot()`, no profile) and one `support` target.
  - The build path (`buildPluginCargo`) is the iterate stage.
  - Distribution accepts only shipped modules.
- **Hub**:
  - `trusted-catalog-preflight` reads the shipped bytes and receipt per selected component.
  - The fresh producer refuses a fresh semantic descriptor that differs from the committed one, then binds.
  - The stdio catalog-root emits semantic, checks owner equality and binds into a work dir. The owner-pair publish and snapshot/restore helpers are gone.
- **Registry**:
  - Rebuild steps are `release` → `describe --all` → `materialize`.
  - The projection emits `PLUGIN_COMPONENT_DIRECTORY = "dist/components"`.
  - The descriptor gate is `shippedComponentFindings`, which reads the receipt.
  - The refresh no longer syncs built descriptors into owners.

### Cargo profiles

`🎚️profiles.py` (ticket input) rewrote 140 `Cargo.toml` files:

- canonical `wasm-release` (cgu 16, thin LTO, no package overrides);
- `wasm-iterate` (inherits release; opt 1, no LTO, cgu 256, incremental, assertions, symbols kept; the 4 hot packages at opt 2);
- `wasm-dev` deleted.

## Deviations and Remaining Deltas

1. **Hub trusted-catalog generations still store bound descriptors**, not semantic plus receipt. Storing the receipt per hub entry changes the runtime protocol: Rust `trusted-catalog::validate_descriptor`, the store worker, ProgramBridge and the MCP remote. It needs its own wave. The fresh producer already enforces the semantic-equals-committed law before binding.
2. **A1-owned synthetic fixtures** keep `wasm-dev` / `dist/component-release` labels as stub profile names in self-contained stub workspaces:
   - `🧩️component-pass` fixture and test;
   - the `🎟️admission` and `🔒️lease` fixtures;
   - `🏗️native-build/🧪️tests/{🟦️.ts, 🦀️cargo}`.

   They define their own `[profile.wasm-dev]` and do not read the repository profiles, so nothing breaks. Renaming them to `wasm-iterate` / `dist/components` is a vocabulary pass for lane A.
3. **Unrelated `dist/wasm-dev`**: the wgpu engine's `dist/wasm-dev` (trunk `dev`), the wasm-outputs cache fixture and the vite `playground-flow-wasm-dev-stub` name the wasm-bindgen renderer's dev build, not the component profile. They are untouched.
4. **Host tests**: `framework-plugin-host:test` now depends on the shipped `component` of the host test component, which depends on `workspace:release`. That is one shipped profile by design. On a warm build dir the driver pass is a freshness check.

## Verification

Everything below was run in this session. Cargo ran through `🚦️cargo-slot.sh` with private `⚡️cache/a2/{target,build}`.

### Passing

| Gate | Result |
|---|---|
| `ComponentReceiptV1` (`bun test`, Ajv oracle, Python-oracle fixture) | 29/29 |
| descriptor-verification suite (`🛂️descriptor-verification/🧪️tests`) | 21/21 |
| registry discovery (`🔎️discovery/🧪️tests`, with `SEMIO_TEST_ARTIFACT_DIR`) | 10/10 |
| registry rebuild (`🧪️tests/🔁️rebuild`) | 5/5 |
| catalog producers (vitest, full level): catalog-complete 12/12, host-activation, generated-projection 5/6 | 48/49. The 1 failure is a pre-existing JSON import-attribute fault in a node child |
| fresh component (`os-plugin-describe-rs:test-fresh-component`: source epoch, staging incl. the semantic-equals-committed refusal, process laws) | green |
| graph native prerequisites (`🕸️graph/🧪️tests/🧩️native-prerequisites`) | 6/6 |
| publication source ownership law | 7/7 |
| os-dev composition ownership law | 9/11. The 2 failures are a peer's `🎮️playground-session` move to `✏️s/` |
| `repo:test-cache-policy` | green |
| `repo:policy-check` | no finding names an A2 target (544 → 473 CACHE-03 after narrowing `component`'s output); the remaining findings predate A2 |
| `componentTargets` contract over all 70 discovered crates (nx-contract vectors, no output overlap) | 70/70, 0 violations |
| Manifest codec (`semio-framework --lib package_descriptor_value_codec`) | 2/2 |
| `semio-framework-plugin` `--lib --tests` check (incl. the generated-test-contracts and descriptor unit-test changes) | compiles |
| Descriptor emitter (`os-plugin-describe-rs:build`) | built, 5 m 10 s cold |
| TypeScript (`tsc` over the 90 changed TS files) | 0 errors in A2 code. The remaining 173 are pre-existing or peer: `frameworkOsPlaygroundDefaultPort`, Bun/vite plugin typing, `whatwg-url` declarations |
| End-to-end describe from release bytes: bim, animate, architect | All 3 committed pairs rewritten and admitted by `verifySemanticDescriptorPairBytesV1`. Each receipt verified against the bytes and the pack (`componentReceiptFindingsV1` = 0). bim's diff is exactly the removed `hashes`; animate and architect also picked up real source drift (one `artifactJson` example). |

### Blocked by Peer Lanes

The tree did not compile for the component fleet at any point in the window:

| Time | Cause |
|---|---|
| 04:3x | os-kernel `ArtifactHistoryReadRequest` privacy, lane D/E1 |
| 04:5x | `store` backbone imports |
| 05:1x | the plugin crate reactor re-exports, E2 (landed 05:13) |
| 05:2x and later | `stdio-semio-*` split, lane C |
| 06:1x and later | os-kernel `app_contract` privacy, `base_foundation` |

The following waits on a compiling tree:

- describe equality on the remaining components;
- the profile-independence sample;
- `plugin-registry:check` (refuses the 67 still-bound pairs until then);
- `hub trusted-catalog-preflight`;
- the play unit gate (`semio-tech-play:test` reads the stale gitignored generated registry: 68 rows, `norm` and `stdio-media` absent);
- `os-mcp`/`os-run` checks, which are stale against the kernel independently of A2.

**Why the fleet run was stopped.**

- Sequential narrow builds cost 17–32 min per hub component on the loaded machine. The remaining 67 would take well over 10 hours.
- Large plugins add a describe of up to 14 min each in the owned interpreter.
- The designed path is the A1 driver's single parallel pass (`workspace:release`), after which each per-crate `describe` is a cargo fresh check plus the describe. That release build was out of bounds for this wave.
- Follow-up worth taking: overlap describe with the next build in `describeComponentsV1`. This needs an async `emitSemanticDescriptorV1`; it hides the up-to-14-min describe of large plugins.

The commands, in order:

```
bun nx run workspace:release
bun nx run workspace:components describe --all
bun nx run @semio-tech/plugin-registry:generate && bun nx run @semio-tech/plugin-registry:check
bun nx run workspace:components profile-independence --sample 8
bun nx run @semio-tech/hub:trusted-catalog-preflight
bun nx run @semio-tech/semio-tech-play:test
```

### Inherited, Not A2

- The describe crate's lib-test target does not compile at HEAD: its `cfg(test)` wasmtime host implements `operation_*` on `Host` without `HasData`, and an included interpreter test needs `semio_framework_trace`.
- The A1 synthetic fixtures keep the stub label `wasm-dev` (Deviation 2).

## Measurements

| What | Value |
|---|---|
| Descriptor emitter build, cold private dir | 5 m 10 s |
| First component closure at `wasm-release`, cold private dir (demonstrator attempt, until the stdio-semio failure) | 11 m 56 s, 2,219 s CPU, peak RSS 4.5 GB |
| bim from warm deps | 1,059 s total: about 14 min narrow build and link, about 3 min wasmtime `describe()` of the 60 MB component |
| animate | 1,026 s: about 17.5 min build under load average about 25, 23 s describe |
| architect | 1,903 s: about 18 min build, about 14 min describe (owned interpreter, 18.5 G fuel, 415 s execute plus the codec census) |
| Shipped component sizes | bim 59.9 MB, animate 60.8 MB, architect 97.3 MB (all under the 256 MiB shipped cap) |
| Fleet attempt, two workers split by workspace | 16 `✏️s` extensions refused by lane D's private `os_kernel::app_contract` (about 2 min each to fail); hub at 2 of 43 after 50 min. Stopped. |
| Committed pairs rewritten | 3 of 70 (bim, animate, architect). Discovery now finds 70 components, not 72; `norm` and `stdio-media` are new against the stale generated registry |
| Iterate component sizes against the 512 MiB cap | not measured: the iterate build needs the same compiling tree |

## Files

Paths are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` (written `🔌️/`) unless they are repo-rooted.

### New

- `🔌️/🖨️describe/🏷️component-receipt/`:
  - `🧬️schema/🔣️.json`
  - `🟦️.ts`
  - `🧫️fixtures/🔣️.json`
  - `🧪️tests/🟦️.ts`
- `🔌️/🖨️describe/🧩️component-stage/🟦️.ts`
- Ticket inputs in `wave-a2-single-profile/`:
  - `🎚️profiles.py`
  - `🧫️receipt-fixture.py`
  - `🚀️launch-entries.py`

### Rust

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` and its `🧪️tests/🔬️package-descriptor-value-codec/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`
- `🔌️/🖨️describe/🛂️descriptor-emission/🦀️.rs` and `🔌️/🖨️describe/🧪️tests/🔬️unit/🦀️.rs`
- The plugin crate:
  - `⚛️reactor/🏃️runtime/🦀️.rs` (`canonical_semantic_descriptor_bytes`)
  - `🧪️tests/🧬️generated-test-contracts/🦀️.rs`
  - `🛂️describe/🦀️.rs` and its unit test
- `🌉️mcp/🏠️workspace/🦀️.rs`, `🌉️mcp/🧪️tests/🧱️source-builders/🦀️.rs`, and the MCP long and quick test skip messages
- `🏃️run/🏗️bootstrap/🦀️.rs`
- 140 `Cargo.toml` profile blocks (root, hub, teaching, ✏️s, extensions)

### Registry

- `📇️registry/`:
  - `📽️projection/🟦️.ts`
  - `🔎️discovery/{🟦️.ts, 🧬️schema/🟦️.ts, 🧬️schema/🔣️.json, 🧫️fixtures/🔣️.json}`
  - `🛂️descriptor-verification/{🟦️.ts, 🧬️schema/🔣️.json, 🧪️tests/🧫️fixtures/🔣️.json}`
  - `✅️catalog-verification/🟦️.ts`
  - `🔁️rebuild/{🟦️.ts, 🔣️.json}`
  - `🔄️refresh/🟦️.ts`
  - `🧫️fixtures/📖️generated-projection.json`
- Registry tests in `🧪️tests/`:
  - `✅️catalog-complete`
  - `🔁️rebuild`
  - `📖️generated-projection`
  - `🎬️host-activation`

### Describe and Browser Bundle

- `🔌️/🖨️describe/`:
  - `🏗️component-build/🟦️.ts`
  - `🛂️descriptor-emission/🟦️.ts`
  - `🏭️fresh-component/🟦️.ts`
  - `📦️packages/🦀️rust/📜️script.ts`
  - `🧪️tests/🆕️fresh-component/🟦️.ts`
- `🔌️/🌐️browser-bundle/`:
  - `🛂️descriptor/🟦️.ts`
  - `🧾️describe/🟦️.ts`
  - `🏗️materialization/🚀️commands/🟦️.ts`
- `🔌️/📦️packages/🟦️typescript/📋️project.json`

### os-dev Build

- `🔌️/🏗️build/`:
  - `📋️plan`
  - `📦️materialization`
  - `🛂️descriptor`
  - `🏃️execution`
  - `🔍️freshness`
- `🧑‍💻dev/`:
  - `♻️activation/🟦️.ts` and its callers (preparation, execution, readiness, freshness, vite builder, wgpu server config, `🌐️browser-host/🏗️staging`)
  - `🚚️distribution/🔌️components/🟦️.ts`
  - `📦️packages/🟦️typescript/📋️project.json` (the `plugin` target now depends on the emitter build and `support`)
- `.storybook/main.ts`
- The wgpu `⌨️native-entrypoint/📦️modules/📜️script.ts`
- Play `🔨️modules/🧩️runtime/♻️activation`

### Library and Nx

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`:
  - `🟦️.ts`
  - `🟨️.mjs` (`componentTargets`, playground targets)
  - `⚡️caching/🦀️cargo/🟦️.ts` (region `ReleaseComponentPlan`)
  - `⚡️caching/🧫️fixtures/nx-contract/🔣️.json`
  - `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`
  - `🧫️fixtures/{📣️plugin-publication-source-ownership, 🧑‍💻os-dev-composition-ownership}/🔣️.json`
  - `🧪️tests/📣️plugin-publication-source-ownership/🟦️.ts`
- A1-owned, additive only:
  - the `📋️native-orchestration` `native component`;
  - the `🏗️native-build` `provenance` option.
- Root `📜️script.ts` (`ComponentsScript`, OsScript paths) and `📋️project.json` (`workspace:components`)
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`:
  - `📦️describe🦑️workspace🧩️components`
  - `⚖️gate🦑️workspace🧩️profile-independence`
  - `📦️iterate🖍️draw🦀️`
  - `📦️support🔌️plugin🟦️`
  - demonstrator and host-test-component `:component`

### Hub

- `🌎️hub/📦️packages/🦀️rust/{📜️script.ts, 📋️project.json}`
- `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` and `✅️trusted-stdio-catalog/🟦️.ts`
- The norm script
- The component-codecs schema, fixture and test, and the trusted-catalog-profile test
- `🌎️hub/🧫️fixtures/🌳️unit-census/🔣️.json` (profiles `wasm-iterate`, `wasm-release`)

### Fixtures and Tests With Path or Target Vocabulary

- Hub guest tests: count-lease, owned-instance-open, draw, note ×2, generation3d guest-memory
- note/bim committed-descriptor fixtures
- Dashboard fixtures and Rust tests
- graph-revision and bootstrap-deps fixtures
- cargo-provenance fixture
- `🧰️framework/🔨️modules/🕸️graph/🧫️fixtures/{🧩️native-prerequisites, 🎮️renderer-inputs}` and the test
- dev `🦀️wasm-profile-policy` fixture and schema
- watch-policy fixture
- ticket-owned browser-host staging test
- host test component `📜️script.ts`/`📋️project.json`
- scale fixture
