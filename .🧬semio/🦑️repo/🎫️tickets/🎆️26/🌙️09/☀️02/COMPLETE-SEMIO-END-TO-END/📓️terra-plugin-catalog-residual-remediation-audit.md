# Plugin Catalog Residual Remediation Audit

Date: 2026-09-03  
Scope: source-only, read-only audit after the separate `stdio` catalog-root repair. No production or test files were changed, and no Cargo, catalog, or runtime build/test was run for this audit.

## Result and evidence boundary

The current source preflight (`auditPluginCatalogSources`) discovers **59** component manifests, orders all 59 without a dependency-invalid diagnostic, and reports **31** source diagnostics: the separately-owned `stdio` missing pair plus the **30 residuals** in this report. The resulting source-only count is 28 currently valid descriptor pairs, 18 absent pairs, four invalid CAD placeholders, and eight JSON/pack disagreements.

This is not evidence that any currently checked-in pair matches a new component build. `catalog-complete` first rejects every source diagnostic and only then checks raw WASM, extracted core WASM, and descriptor bytes in a dedicated fresh build root. It rejects both `target/` and the dev cache as ambient roots. Thus neither a generated registry row nor residue under either location is deployable evidence.

Primary gate anchors are `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2233-2284` (pair identity), `:2287-2344` (independent source audit), `:2400-2425` (fresh receipts), and `:2430-2467` (fail-closed completion). The existing neutral gate test is `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️catalog-complete.test.ts:71-193`.

## Residual census

### 18 missing owner-root descriptor pairs

Every following owner root lacks both required committed forms, `🔣️.json` and `🛂️.descriptor.semio`. The Cargo component identity and dependency are discovered from its current `Cargo.toml`; none comes from the generated registry.

| Owner root and plugin id | Cargo package / component | First dependency | Current producer condition |
| --- | --- | --- | --- |
| `✏️s/🔌️plugins/🧱️block` — `block` | `semio-s-plugin-block` / `semio:block` | `stdio` | `📦️packages/🦀️rust/📜️script.ts:13-22` exposes `describe`, but no pair is present. |
| `✏️s/🔌️plugins/📖️playbook` — `playbook` | `semio-s-plugin-playbook` / `semio:playbook` | `stdio` | `📦️packages/🦀️rust/📜️script.ts:13-22` exposes `describe`, but no pair is present. |
| `✏️s/🔌️plugins/🔱️trinity` — `trinity` | `semio-s-plugin-trinity` / `semio:trinity` | `stdio` | `📦️packages/🦀️rust/📜️script.ts:13-22` exposes `describe`, but no pair is present. |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim` — `flow-extension-bim` | `semio-s-plugin-flow-extension-bim` / `semio:flow-extension-bim` | `flow` | `📦️packages/🦀️rust/📜️script.ts:1-19` has only `package`. |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw` — `flow-extension-draw` | `semio-s-plugin-flow-extension-draw` / `semio:flow-extension-draw` | `flow` | `package` only. |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control` — `imperative-extension-control` | `semio-s-plugin-imperative-control` / `semio:imperative-extension-control` | `imperative` | `package` only. |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect` — `imperative-extension-effect` | `semio-s-plugin-imperative-effect` / `semio:imperative-extension-effect` | `imperative` | `package` only. |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic` — `imperative-extension-logic` | `semio-s-plugin-imperative-logic` / `semio:imperative-extension-logic` | `imperative` | `package` only. |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math` — `imperative-extension-math` | `semio-s-plugin-imperative-math` / `semio:imperative-extension-math` | `imperative` | `package` only. |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text` — `imperative-extension-text` | `semio-s-plugin-imperative-text` / `semio:imperative-extension-text` | `imperative` | `package` only. |
| `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural` — `playbook-module-procedural` | `semio-s-plugin-playbook-procedural` / `semio:playbook-module-procedural` | `playbook` | `package` only. |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete` — `process-extension-concrete` | `semio-s-plugin-process-concrete` / `semio:process-extension-concrete` | `process` | `package` only. |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal` — `process-extension-metal` | `semio-s-plugin-process-metal` / `semio:process-extension-metal` | `process` | `package` only. |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic` — `process-extension-robotic` / `semio:process-extension-robotic` | `process` | `package` only. |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood` — `process-extension-wood` / `semio:process-extension-wood` | `process` | `package` only. |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams` — `sourcing-module-beams` | `semio-s-plugin-sourcing-beams` / `semio:sourcing-module-beams` | `sourcing` | `package` only. |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs` — `sourcing-module-slabs` | `semio-s-plugin-sourcing-slabs` / `semio:sourcing-module-slabs` | `sourcing` | `package` only. |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows` — `sourcing-module-windows` | `semio-s-plugin-sourcing-windows` / `semio:sourcing-module-windows` | `sourcing` | `package` only. |

The 15 extension owners use the same non-descriptor package helper, `runExtensionComponentPackage` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:3009-3043`). It builds into the ambient Cargo target and emits a runtime `.sxt`; it does **not** emit the tracked owner pair. This is a shared producer defect, not fifteen independent missing assets. For example, the flow BIM script exposes only `test` and `package` (`…/🏗️bim/📦️packages/🦀️rust/📜️script.ts:1-19`) and its Nx project exposes the same two targets (`…/📋️project.json:10-50`).

The other three are only superficially better: their existing `describePluginComponent` currently accepts one component file (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts:56-75`). It must use the corrected two-artifact raw/core emission path from the `stdio` repair before any new pair can be trusted.

### Four CAD placeholder identities

Each path below has a committed placeholder JSON (`role: "plugin"`, `manifest.pluginId: "empty"`, version `0.0.0`) at `🔣️.json:1-8`; source preflight rejects its descriptor as not matching the Cargo identity. Its sibling pack is likewise not a valid owner descriptor. Every Cargo metadata declaration says `role = "extension"`, `extends = "cad"` (`📦️packages/🦀️rust/Cargo.toml:11-16`), but its router is again package-only (`📜️script.ts:1-19`).

| Owner | Expected identity |
| --- | --- |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building` | `cad-extension-aec-building`, extension of `cad` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy` | `cad-extension-aec-building-energy`, extension of `cad` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure` | `cad-extension-aec-building-structure`, extension of `cad` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape` | `cad-extension-spatial-shape`, extension of `cad` |

These are source incompleteness, not stale generated output. Replacing only the placeholder JSON or pack is unsafe: both forms, their self hash, raw/core hashes, and generated registry hash triple must come from the same fresh component receipt.

### Eight semantic JSON/pack disagreements

The strict comparator normalizes enum representation before comparing (`registry/📜️script.ts:2274`), so these are semantic—not whitespace/key-order—differences. Each has both owner files (`🔣️.json`, `🛂️.descriptor.semio`) and a root `describe` route, but the pair itself fails before its hashes could be treated as evidence.

| Owner / plugin id | Current divergent declaration, first material symptom |
| --- | --- |
| `✏️s/🔌️plugins/🏛️architect` / `architect` | Activation names `data.program` in JSON versus `data.🏛️program` in pack. |
| `✏️s/🔌️plugins/🎪️demonstrator` / `demonstrator` | Consumer/app selector uses `s.sourcing.curation` versus `s.sourcing.curate`. |
| `✏️s/🔌️plugins/🔋️energy` / `energy` | Activation names `data.model` versus `data.🔋️model`. |
| `✏️s/🔌️plugins/📜️imperative` / `imperative` | App identity is `procedure` versus `imperative`. |
| `✏️s/🔌️plugins/➗️mathematical` / `mathematical` | App identity is `equation` versus `mathematical`. |
| `✏️s/🔌️plugins/🌀️procedural` / `procedural` | Artifact activation is `2d.generation` versus `2d.procedural`. |
| `✏️s/🔌️plugins/🪵️sourcing` / `sourcing` | App identity is `sourcing.curation` versus `sourcing.curate`. |
| `✏️s/🔌️plugins/✒️writer` / `writer` | JSON-only `interactiveJob: "migrated"` contributions are absent from the packed form. |

The owner scripts in all eight root `📦️packages/🦀️rust/📜️script.ts` files import `describePluginComponent` and expose `describe`. Their defect is therefore a drifted/hand-altered pair or source declaration disagreement, not absence of a route. Resolve the canonical component declaration first; do not choose JSON or pack as truth by hand.

## Root causes and severity

| Severity | Shared cause | Consequence |
| --- | --- | --- |
| High | The descriptor producer takes a single component artifact, while strict publication requires independent raw and extracted-core receipts. | A pair can appear complete yet fail fresh core-hash verification; it cannot attest a trusted catalog. |
| High | Extension `package` produces `.sxt`, not a tracked JSON/pack descriptor pair; 15 residuals and all four CAD owners lack a descriptor producer. | 19 identities cannot enter the strict catalog. |
| High | Four CAD descriptor files are explicit empty/plugin placeholders. | They violate component identity and could misclassify an extension as an unrelated plugin if a loose loader consumed them. |
| High | Eight root pairs have two conflicting semantic declarations. | Server/open-plan capability selection has no authoritative descriptor value. |
| Medium | `catalog-complete` is intentionally isolated from generated/cache output, but no owner-level receipt workflow currently connects all producers to it. | Hand-edited `🤖️generated` rows or ambient target artifacts can look green outside the release gate. |
| Medium | A descriptor advertises metadata, not a linked native implementation. | Re-emitting all 30 pairs must not synthesize Rust codec/factory bindings or claim native openability; native linkage needs its own explicit generated receipt and duplicate/ambiguity checks. |

## Bounded remediation packet

### P0 — prerequisite outside this residual packet: finish `stdio`

Land the owner-root JSON/pack pair produced from independently staged raw and extracted core WASM, then verify it in a fresh, non-cache root. Do not start final catalog publication before this. `block`, `playbook`, and `trinity` directly depend on it; registry ordering is dependency-first (`registry/📜️script.ts:2141-2218`).

### P1 — one shared descriptor producer contract (first residual blocker)

1. Replace the one-file `describePluginComponent` contract at `…/📇️describe/📦️packages/🦀️rust/📜️script.ts:56-75` with an explicit `(rawComponentPath, extractedCorePath, ownerRoot)` receipt. It must write canonical pack and JSON from a single decoded descriptor; hash raw/core separately; blank exactly `descriptorSha256` for self hashing; reject non-regular/out-of-root inputs; use a bounded deadline/cancellation callback; and atomically replace both owner forms only after all checks pass.
2. Keep `runExtensionComponentPackage` focused on `.sxt` installation. Add a separate shared extension `describe` route using the contract above, then add `describe` targets to the 15 extension `📜️script.ts`/`📋️project.json` owners. Do not infer a descriptor from `.sxt` or copy it from a host.
3. Extend the descriptor-emitter test and `catalog-complete.test.ts` fixtures: a distinct raw/core byte pair must yield distinct hashes; a swapped core must fail; owner pair write must not be half-published; cancellation/deadline must leave old pair intact. Cross-check SHA-256 and canonical pack encoding with WebCrypto, as the current neutral test already does at `:115-166`.

This is one shared Sol-sized change plus thin owner registrations. It is the first deterministic blocker remaining after `stdio`: without it, recreating pairs cannot satisfy the actual strict artifact gate.

### P2 — parallel source-identity corrections (after P1 API is fixed)

These source edits may be prepared in parallel, but descriptors must be re-emitted only through P1.

1. Correct the canonical Rust/component declarations behind the eight semantic rows, then re-emit each complete pair. Delete neither form independently and do not edit either hash by hand.
2. Add P1 `describe` registration to `block`, `playbook`, and `trinity`, and emit their missing pairs after the `stdio` receipt is available.
3. Replace all four CAD placeholders by descriptors emitted from their actual component manifests. Assert `role=extension`, exact component-derived `pluginId`, `extends=cad`, and first dependency/host equality. No placeholder may be retained as a fallback.
4. Add `describe` registration for the remaining 15 extensions. The host identity is source-derived and must be fixed before the child is accepted: flow before BIM/draw; imperative before its five extensions; playbook before procedural; process before its four children; sourcing before beams/slabs/windows; cad before its four children.

### P3 — deterministic receipt and generated-projection wave

1. Build/re-emit descriptors in the graph order above into a per-run absolute fresh root—not `target/` and not `🧑️‍💻️dev/🔌️plugin-modules`—while staging `raw/<wasmOut>`, `core/<wasmOut>`, and `descriptor/🛂️.descriptor.semio` for every row.
2. Run source audit, then regenerate the registry only from valid owner pairs. `🤖️generated` is a projection; it is never an input or repair target.
3. Require `catalog-complete` to verify all 59 staged rows before any release publication. It withholds publication after a parent failure and bounds diagnostics/cancellation (`catalog-complete.test.ts:71-113`).
4. Generate native factory metadata only from a separate explicit code-generated linkage receipt keyed by the verified `(pluginId, package/component identity, descriptor digest, artifact kind/schema)`. A valid WASM descriptor alone must yield no linked codec. Reject duplicate or unlinked claims rather than silently selecting one.

## Tests and independent oracle packet

Add focused tests, not a broad workspace run:

1. Expand `registry/🧪️catalog-complete.test.ts` with all 30 neutral source-shaped fixtures: eighteen absent owner pairs, four CAD `(role, pluginId, host)` mismatches, and eight normalized JSON/pack divergences. Assert one bounded diagnostic per owner and that the source audit count is 30 when stdio is supplied as valid.
2. Add an emitter fixture with a known raw/core/descriptor triple and verify it independently using WebCrypto SHA-256 plus a second pack decoder/encoder implementation. It must prove JSON and canonical pack decode to the same semantic value, and reject hand-matched registry hashes with altered owner bytes.
3. Add an extension fixture proving `package` alone cannot satisfy catalog preflight, while `describe` emits its owner pair and preserves the declared host as dependency zero.
4. Add an ambient-cache oracle: inject matching-looking data under repo `target/`, dev cache, or generated registry and assert `createFreshCatalogBuildVerifier` rejects it. The existing test already covers the target/cache boundary (`catalog-complete.test.ts:168-181`); extend it to a full 59-row staged fixture.
5. After code lands, use only focused existing targets: `bun nx run @semio-tech/plugin-registry:test -- --run 🧪️catalog-complete.test.ts`; the existing root `describe` targets for block/playbook/trinity; each newly registered extension `describe`; `bun nx run @semio-tech/plugin-registry:check-generated`; finally `bun nx run @semio-tech/plugin-registry:catalog-complete -- --build-root <absolute dedicated fresh root>`. The final command is intentionally not suitable until P0–P3 are complete.

## Exit criteria

The residual is complete only when source audit returns zero issues for all 59, every owner pair semantically agrees and has correct component/host identity, the 59 fresh raw/core/descriptor receipts match those pairs, generated projection is reproducible without hand edits, and native factory linkage remains explicit rather than inferred from descriptor presence. This audit provides no such runtime/build evidence.
