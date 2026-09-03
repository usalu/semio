# Plugin/Artifact Catalog Completion Audit

## Outcome

The catalog is **not complete or reproducible**.  The generated registry currently has 59 rows (33 plugins and 26 extensions), but only 40 rows have the required three digest fields.  The remaining 19 have neither owner-root `🔣️.json` nor `🛂️.descriptor.semio`; this is source incompleteness, not a generated-file problem.  In particular, `stdio` is missing its descriptor and is the root of the full 59-row dependency closure.

The apparently broad dev-cache coverage is not an authority signal: it has 57/59 plugin directories, 56 core-WASM/component pairs, and only 38 staged descriptors.  Both the registry output and that cache are ignored by Git.  They are useful local build residue, not a deployable catalog or a trustworthy substitute for checked-in descriptor inputs.

No build was started for this audit.  `target/` fingerprint diagnostics below prove that a Cargo invocation previously emitted those diagnostics; they do not prove that the present shared working tree will emit the same result.

## Current census

| Surface | Count / status | Evidence and consequence |
| --- | --- | --- |
| Registry rows | 59 = 33 plugin + 26 extension | `📇️registry/🤖️generated/🔣️plugins.json`; rows are derived from Cargo package metadata in `📜️script.ts:228-293`. |
| Artifact source roots | 33 plugin roots, 92 top-level `🗿️artifacts/*` directories | Extensions normally contribute no artifact root.  The registry is a **plugin** registry; it generates Rust artifact constants (`📜️script.ts:933-951`), not a separately digest-addressed artifact-row registry. |
| Complete source descriptor/hash triplets | 40/59 | Each has `wasmSha256`, `coreWasmSha256`, and `descriptorSha256` in the generated row, derived from its owner-root descriptor. |
| Missing source descriptor/hash triplets | 19/59 | Every listed row lacks both owner-root descriptor files. |
| Dev cache directories | 57/59 | `draw` and `layout` have no directory.  Directory presence alone is not execution readiness. |
| Dev-cache core/component pairs | 56/59 | `draw`, `energy`, and `layout` lack a core WASM.  `energy` has a directory but no core. |
| Dev-cache staged descriptors | 38/59 | All 19 unmigrated rows have no staged descriptor; the absent `draw` and `layout` account for the other two. |
| Independently validated current component bytes | 0/59 | A fresh component artifact is required to validate the raw and core digests.  Existing `target/` and ignored cache output are not a clean build attestation. |

`📜️script.ts:149-165` deliberately resolves descriptors two levels above the Rust crate, at the tracked owner root.  The old `🤖️generated/` location is globally ignored and must not be revived.  `readDescriptorJson` uses that resolved path (`:201-210`), although its comments at `:196-200` and `parsePluginCargo` comments at `:214-226` still describe the former generated path and “0/N” migration state.  That is documentation drift, not a reason to write generated output by hand.

### Exact 19 missing rows

Each row has `hashes` absent, an empty activation-event set, and no owner-root `🔣️.json` or `🛂️.descriptor.semio`.  Paths below are the registered `cratePath`; the required descriptor owner is two parent directories above it.

| Plugin ID | Role / dependency | Crate path | Required raw component output |
| --- | --- | --- | --- |
| `block` | plugin; `stdio` | `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust` | `semio_s_plugin_block.wasm` |
| `flow-extension-bim` | extension; `flow` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust` | `semio_s_plugin_flow_extension_bim.wasm` |
| `flow-extension-draw` | extension; `flow` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust` | `semio_s_plugin_flow_extension_draw.wasm` |
| `imperative-extension-control` | extension; `imperative` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust` | `semio_s_plugin_imperative_control.wasm` |
| `imperative-extension-effect` | extension; `imperative` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust` | `semio_s_plugin_imperative_effect.wasm` |
| `imperative-extension-logic` | extension; `imperative` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust` | `semio_s_plugin_imperative_logic.wasm` |
| `imperative-extension-math` | extension; `imperative` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust` | `semio_s_plugin_imperative_math.wasm` |
| `imperative-extension-text` | extension; `imperative` | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust` | `semio_s_plugin_imperative_text.wasm` |
| `playbook` | plugin; `stdio` | `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust` | `semio_s_plugin_playbook.wasm` |
| `playbook-module-procedural` | extension; `playbook` | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust` | `semio_s_plugin_playbook_procedural.wasm` |
| `process-extension-concrete` | extension; `process` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust` | `semio_s_plugin_process_concrete.wasm` |
| `process-extension-metal` | extension; `process` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust` | `semio_s_plugin_process_metal.wasm` |
| `process-extension-robotic` | extension; `process` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust` | `semio_s_plugin_process_robotic.wasm` |
| `process-extension-wood` | extension; `process` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust` | `semio_s_plugin_process_wood.wasm` |
| `sourcing-module-beams` | extension; `sourcing` | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust` | `semio_s_plugin_sourcing_beams.wasm` |
| `sourcing-module-slabs` | extension; `sourcing` | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust` | `semio_s_plugin_sourcing_slabs.wasm` |
| `sourcing-module-windows` | extension; `sourcing` | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust` | `semio_s_plugin_sourcing_windows.wasm` |
| `stdio` | plugin; no dependency | `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust` | `semio_s_plugin_stdio.wasm` |
| `trinity` | plugin; `stdio` | `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust` | `semio_s_plugin_trinity.wasm` |

## Dependency and build-path findings

`stdio` has 33 direct dependents: `animate`, `architect`, `block`, `cad`, `dag`, `demonstrator`, `draw`, `energy`, `fem`, `flow`, `flow-extension-brep`, `forms`, `gis`, `imperative`, `layout`, `lowpoly`, `mathematical`, `norm`, `note`, `playbook`, `procedural`, `process`, `puzzle`, `raster`, `reasoning-mindmap`, `remodel`, `s`, `sequence`, `shooting`, `sourcing`, `trinity`, `vcs`, and `writer`.  Traversing `dependsOn` makes its closure all 59 rows.  Therefore, a trusted complete catalog cannot accept even the 40 apparently described dependents while `stdio` has no descriptor/digest identity.

The registry filter closes dependencies (`📇️registry/📜️script.ts:533-559`).  The dev build path does not provide a catalog-completeness proof: it regenerates the ignored registry, synchronizes descriptors only if an output directory already exists (`🧑️‍💻️dev/📜️script.ts:1134-1140`), runs Cargo serially, then materializes with bounded parallelism while collecting and continuing past failures (`:1093-1126`).  It only rejects the requested batch after every attempted target (`:1129-1132`).  The passed `orderedTargets` must become an explicit dependency topological order, and a child must not be published if any required dependency’s fresh descriptor/artifact pair failed.

The materializer does have the correct *publication* seam: after a new Cargo component it transpiles, asks the component to describe itself, hashes raw and core bytes, writes both owner-root descriptor forms, then stages the descriptor (`:941-972`, `:994-1017`).  It intentionally does not copy raw component WASM to the browser cache (`:1001-1004`).  The fix is to drive this seam from clean artifacts and commit the resulting owner-root descriptors; it is not to edit `🤖️generated/🔣️plugins.json` or `🔌️plugin-modules`.

## First deterministic blocker and archived build evidence

**First deterministic completion failure (high):** all 19 owner-root descriptor pairs are absent.  `validateDescriptors` recognizes the absence at `📇️registry/📜️script.ts:1947-1953`, but treats it as a warning; `check` prints warnings and exits successfully unless an already-present descriptor is malformed (`:2069-2079`).  The comments explicitly preserve this transitional asymmetry (`:1927-1940`).  A complete-catalog authority must make missing descriptor, missing artifact, and unverified digest fatal.

The following are *recorded* Cargo diagnostic files under untracked `target/wasm32-wasip2/debug/.fingerprint`.  They are useful source leads but must be rerun in a clean target directory after their source repairs.  A “no recorded diagnostic” means neither success nor failure was established.

| Recorded state | Rows | First recorded error / interpretation |
| --- | --- | --- |
| Recorded error (12) | `animate`, `architect`, `block`, `cad`, `draw`, `energy`, `fem`, `mathematical`, `procedural`, `puzzle`, `remodel`, `trinity` | `animate`: missing presentation mutation module; `architect`/`mathematical`: stale stdio exported types; `block`: missing `mutation` child; `cad`/`fem`: missing mutation module; `draw`: missing stdio `subsets::any` paths/types; `energy`: unavailable `semio_framework`; `procedural`: unsupported tuple/unit derive; `puzzle`: missing `Deserialize`; `remodel`: mutation source owner mismatch; `trinity`: unavailable `framework_editor`/UI contract. |
| Recorded no error (3) | `process`, `sourcing`, `stdio` | A prior compiler invocation produced no JSON error in the newest fingerprint, but this is not a fresh identity validation.  An older `stdio` fingerprint contains a missing `📄txt` path, demonstrating why fingerprints cannot be treated as current state. |
| No matching recorded diagnostic (44) | all other registry rows, including `layout` and every 19-row extension except `block` and `trinity` | Not attempted, diagnostic was cleaned, or an output was built elsewhere; no conclusion. |

Four archived failures still match present source anchors and are the first focused repair probes:

- `block`’s current inverse still resolves `super::super::move_camera2d::mutation` at `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥️move-camera2d/↩️inverse/🦀️.rs:8`; its diagnostic records `E0433` (1,569 errors total).
- `draw` currently imports the absent stdio `drawing` snapshot names and `svg/.../subsets::any` at `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:16-19`; its recorded `E0433`/`E0432` set has 211 errors.  Its dev-cache directory is absent.
- `energy` still imports `semio_framework` at `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs:7`; the archived build records 279 errors.  It has descriptor metadata and cache directory, but no core WASM.
- `trinity` still re-exports `framework_editor` at `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:7`; the recorded build has 105 errors and it additionally lacks its descriptor pair.

If the current best-effort all-catalog build is used without the strict preflight, its stable generated order attempts `animate` first and its archive starts with an unresolved presentation mutation import.  That is **not** claimed as a present deterministic compiler failure: only a fresh isolated rebuild can establish it.

## Blocker classification

| Severity | Blocker | Classification | Required resolution |
| --- | --- | --- | --- |
| High | 19 owner descriptors and all three hashes absent, including `stdio` | Source incompleteness; current gate defect | Produce descriptors only from fresh component `describe`, commit owner-root pair, then regenerate registry.  Make complete-catalog validation fatal on any absence. |
| High | `stdio` sits below every row | Dependency closure / trust defect | Validate and publish `stdio` first; do not authorize any descendant until its raw/core/descriptor association is verified. |
| High | `draw` and `layout` absent from cache; `energy` lacks core | Missing outputs, not proof of one common compiler cause | Repair source failures, then build/materialize in isolation.  `layout` has no retained diagnostic, so diagnose rather than invent a cause. |
| High | Recorded errors in 12 rows | Upstream compile blockers, archived evidence only | Repair one source family at a time and rerun targeted components; do not use cache survivors as pass evidence. |
| Medium | Registry `check` warns for absent descriptors/WASM and skips digest validation if `target` is absent | Generator/gate defect | Add a strict complete/ship mode with clean-artifact input and fail closed.  Retain a non-authoritative development diagnostic only if useful. |
| Medium | Cache and generated registry are ignored (`.gitignore:89`, `:91`); cache can contain orphan core files | Stale-output/deployment defect | Treat them as rebuildable work area only.  A release must have an explicit signed/catalog manifest or deterministic assembly input, never an ambient local path. |
| Medium | Cargo loop is serial but accepts arbitrary entry ordering; materialization overlaps children | Build orchestration gap | Topologically order the 59-row graph; gate child publication on verified parent outcome and retain fixed concurrency/budget/cancellation reporting. |
| Low | Descriptor comments still cite `🤖️generated` and “0/N” migration | Documentation drift | Correct comments and tests after the source contract is strict; no behavior workaround. |

## Dependency-ordered implementation packet

1. Add a schema-first `catalog-complete` verification to the registry script.  It must independently enumerate every discovered plugin/extension Cargo manifest, require a non-symlink owner-root JSON+pack pair, schema-decode both, require 64-hex raw/core/descriptor digests, and reject an incomplete dependency closure.  It must compare the rendered registry against those source inputs and never repair output.  Keep `check-generated` as byte freshness only (`📇️registry/📜️script.ts:1997-2009`).
2. Make a clean-artifact verifier the only producer of completion evidence.  It must receive an explicit fresh build root, hash the raw component and extracted core separately, check descriptor self-hash/canonical bytes, then copy the descriptor to the ephemeral dev cache only after verification.  Do not read a shared `target/` or pre-existing `plugin-modules` directory as success.
3. Repair and clean-build `stdio`; invoke guest `describe`, commit the owner-root JSON/pack pair, regenerate the registry, and run strict verification.  This is the prerequisite for every other row, not merely the 33 direct dependents.
4. Repair recorded source families before the rows that depend on them: mutation-module ownership/paths (`animate`, `block`, `cad`, `fem`, `remodel`); stdio export/API consumers (`architect`, `draw`, `mathematical`); framework contract imports (`energy`, `trinity`); derive/serialization constraints (`procedural`, `puzzle`).  Re-diagnose `layout` rather than borrowing `draw`’s diagnosis.
5. Build the 33 direct `stdio` dependents in a declared topological batch, publish/describe only verified successes, then build the 26 extensions after their host.  The 19 missing descriptor rows specifically proceed: `block`, `playbook`, `trinity`; then `playbook-module-procedural`; `flow` extensions; `imperative` extensions; `process` extensions; and `sourcing` extensions.  The executor must record per-row start/end, bounded progress, cancellation, error class, raw/core/descriptor digests, and dependency outcome.
6. Generate, rather than hand-edit, every ignored registry/cache output.  A release/authority consumer receives the strict registry plus the declared artifact set from an explicit assembly location; a local dev cache remains disposable.  Update launch registration with a `catalog-complete` command only after its Nx target calls `📜️script.ts` and it is suitable for the existing launch configuration convention.

## Neutral and independent oracle plan

- Create a language-neutral fixture listing discovered package ID, crate path, dependency IDs, expected descriptor owner, and raw/core/descriptor SHA-256.  A filesystem-only oracle must discover the same 59 rows and 92 top-level artifact roots without importing generator helpers.
- Compare the implementation’s Node digest with an independent SHA-256 implementation (Rust `sha2` test oracle or a system SHA-256 utility in CI) for raw WASM, core WASM, JSON canonical pack, and deliberately corrupted bytes.
- In a fresh build root, swap one core WASM between two rows, mutate the descriptor digest, delete one owner descriptor, and add a stale cache core.  Each must fail strict completion; `check-generated` alone may remain green only for the stale-output case, proving the two gates have distinct responsibilities.
- Exercise a three-node parent/child/extension graph with a forced parent compile failure.  The independent harness must observe no child publication, deterministic topological progress, bounded cancellation, and a complete per-row result ledger.
- After all focused component checks are green, use a clean host launch to enumerate and instantiate every declared row; compare its loaded raw/core identity with the completion manifest.  This is the runtime oracle, not cache-directory counting.

## Focused commands for the implementation sequence

These are proposed post-change commands; none was run for this audit.

```sh
bun nx run @semio-tech/plugin-registry:check-generated
bun nx run @semio-tech/plugin-registry:check
bun nx run @semio-tech/plugin-registry:test
bun nx run @semio-tech/framework-os-dev:plugin -- lint
```

Use the registered narrow builder with an explicitly fresh `CARGO_TARGET_DIR` outside the repository’s shared `target/`, first for `stdio`, then each repaired failure family.  For example:

```sh
CARGO_TARGET_DIR=/absolute/empty/catalog-target-stdio SEMIO_PLUGIN_ONLY=stdio bun nx run @semio-tech/framework-os-dev:plugin -- s
CARGO_TARGET_DIR=/absolute/empty/catalog-target-block SEMIO_PLUGIN_ONLY=block bun nx run @semio-tech/framework-os-dev:plugin -- s
CARGO_TARGET_DIR=/absolute/empty/catalog-target-draw SEMIO_PLUGIN_ONLY=draw bun nx run @semio-tech/framework-os-dev:plugin -- s
CARGO_TARGET_DIR=/absolute/empty/catalog-target-energy SEMIO_PLUGIN_ONLY=energy bun nx run @semio-tech/framework-os-dev:plugin -- s
CARGO_TARGET_DIR=/absolute/empty/catalog-target-trinity SEMIO_PLUGIN_ONLY=trinity bun nx run @semio-tech/framework-os-dev:plugin -- s
```

Do not use the React streaming `dev` path as the completion gate: it intentionally starts serving before the full plugin build completes (`🧑️‍💻️dev/📜️script.ts:1866-1873`).  Run the eventual explicit `catalog-complete` target only after the narrow source repairs and use its isolated build root for the independent digest/orchestrator tests.

## Exit criteria

Completion is honest only when all 59 rows have checked-in owner descriptors, a clean isolated run builds/materializes every row in dependency order, independently verifies all three digests, has no skipped `draw`/`layout`/`energy` output, produces a fresh generated registry from those inputs, and an authority/runtime enumeration loads exactly that declared catalog.  The present 57/59 cache directory count, `target/` fingerprints, or warn-only registry check do not meet any of these criteria.
