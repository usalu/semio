# GLTF Geometric Inference and Semantic Mutation Integration Report

## Scope and conclusion

The current GLTF implementation is not connected end to end. It has a real GLTF/GLB analyzer and composer, a process-local schema and inference catalogue, a generic event-sourced artifact store, and a reusable dependency cache, but those pieces stop at their own boundaries:

- analysis produces only `GltfParts { snapshot }`;
- the sole inference unions accessor-declared `POSITION.min/max` metadata;
- `GltfDiff` does not implement `DiffRegions`, so incremental invalidation cannot be correct;
- `ArtifactCodec` can parse, print, apply and mirror operations but cannot execute inference;
- the WIT world exposes composition but no schema catalogue or inference operation;
- the renderer has generic inspector/key-value primitives but no inference projection or subscription;
- inference facets are descriptive placeholders rather than executable text/binary codecs;
- mutations are positional array edits and whole-record replacements which do not repair GLTF index references;
- Nx ignores most non-Rust schema leaves in its cache inputs.

The feature must therefore be integrated as a read-side projection over the post-command artifact revision, not added as more fields beside `bounds`. Authored GLTF remains the event-sourced state. Geometric inference is deterministic derived state, keyed by document revision, policy, dependencies and inference schema version. Semantic commands produce validated diffs, inverses and touched paths; every local, remote, undo, redo, amend and checkpoint transition drives the same projection path.

## Existing end-to-end path and missing links

```text
GLTF/GLB source
  -> GltfAnalyzerAnalysis
  -> GltfParts.snapshot
  -> GltfBuilderConstruction / ArtifactStore<GltfSnapshot,GltfMutation>
  -> GltfMutation -> GltfDiff -> post-mutation GltfSnapshot
  -> [MISSING type-erased inference execution]
  -> [MISSING host/guest WIT transport]
  -> [MISSING revision-scoped projection and cache lifecycle]
  -> [MISSING localized generic inference inspector]
```

The output side is similarly incomplete:

```text
GltfInference Rust value
  -> descriptor embeds Rust/TS/GraphQL/JSON/Proto source strings
  -> process-local registry only
  -> [no callable JSON/text/binary/proto codec]
  -> [no host registry synchronization]
  -> [no GraphQL resolver/query surface]
  -> [no UI consumer]
```

The implementation must close all missing links. A unit test which calls `compute_gltf_bounds` directly is insufficient evidence.

## Exact integration changes

### 1. GLTF analysis, building and composition

Primary files:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️component.rs`

Required changes:

1. Keep `GltfAnalyzerAnalysis` responsible for GLTF/GLB recognition and decoding. Do not compute expensive indicators in `sniff` or `analyze`; analysis must remain responsive.
2. Extend analyzed parts with structured resource-resolution/validation diagnostics needed by inference, or expose those through a reusable validation service called by both analysis and inference. Missing external buffers must remain distinguishable from genuine empty buffers.
3. Treat `GltfParts.snapshot` as authored input. Inference is requested only after a successful snapshot materialization and is published separately.
4. Make the builder and analyzer use the same normalized validation/accessor service. Today the analyzer can parse real GLTF/GLB while the bounds inference trusts metadata and sees a materially different geometry.
5. Preserve `GltfComposerComposition` as source serialization. Inference must not be written into `extras`, extensions, GLTF JSON or GLB chunks.
6. Unify schema identities. The artifact currently mixes `STDIO_GLTF_DOCUMENT_SCHEMA = "stdio.gltf"`, `GLTF_ARTIFACT_SCHEMA_ID = "s.stdio.gltf"`, analyzer dialect kind `s.stdio.gltf`, and artifact kind `stdio.gltf`. Define distinct, consistently named constants for artifact-kind ID, document-schema ID, inference-schema ID and source-format IDs, then replace literals in every consumer. Two semio animation converters currently construct `GltfSnapshot.schema` as `s.stdio.gltf` while the canonical snapshot constant is `stdio.gltf`.
7. Model GLTF JSON and GLB as source forms/dialects of the same artifact, not as an absent fictitious `s.stdio.glb` artifact. CAD and puzzle contain comments documenting the current inability to select binary GLB through the catalogue. The registered dialect/composer roster must advertise both text and binary forms.

Integration invariant: analyzing GLTF JSON and equivalent GLB must yield canonical snapshots whose full inference binary encodings are byte-identical, apart from explicitly recorded source-resource diagnostics which must not affect geometry.

### 2. Inference service and dependency graph

Primary files:

- `.../🧬️schema/💡️inferences/🦀️component.rs`
- `.../🧬️schema/💡️inferences/📦bounds/🦀️component.rs`
- `.../🧬️schema/🔺️diff/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`

The GLTF inferrer must be a staged dependency graph rather than a monolithic result cache:

```text
snapshot + resolved buffers + policy
  -> validation/resource states
  -> decoded accessors
  -> canonical primitive geometry
  -> topology, triangles, normals and acceleration structures
  -> mesh-local metrics
  -> scene/node world instances
  -> world-space metrics and aggregates
  -> broad-phase pair candidates
  -> clearance/contact/interference/adjacency
  -> symmetry/repetition/orientation consistency
  -> final document inference
```

Each stage must be an `InferredField` with a globally unique field ID, a schema/algorithm version and the exact dependency bytes it reads. Policy values—tolerances, sampling/resolution, scene scope, pose, material-density resolver identity and work budget—are part of every affected dependency hash. Public ordering is stable by scene, node-instance path, mesh, primitive, component and entity-pair address. Parallel reductions must merge in that order with deterministic summation.

`GltfDiff` must implement `DiffRegions`. The touched-path vocabulary must include at least:

- `document/scene`, `document/scenes/{i}`;
- `document/nodes/{i}/hierarchy`, `/transform`, `/mesh`, `/skin`, `/weights`;
- `document/meshes/{i}/primitives/{j}` and individual attributes/indices/material/mode;
- `document/accessors/{i}`, `document/bufferViews/{i}`, `document/buffers/{i}` and `buffers/{i}`;
- material/texture/image/sampler fields used by material-volume or density policies;
- skin/morph/animation fields when an evaluated pose is requested;
- policy paths independent of authored state.

Collection insertion/removal is not a narrow single-index touch in GLTF: indices are references. A correct diff either carries the complete reference remap or conservatively touches the affected collection suffix plus every referencing family. `SetSnapshot`/cold load touches the document root and clears the session.

Cache/session scope must be `(plugin owner, artifact schema, document identity, branch/lane or checkpoint identity, materialized revision, inference schema version, policy fingerprint)`. The current `InferenceCache` and `InferenceSession` use mutable `HashMap`s and are not synchronization primitives. Own them inside one document-projection actor or guard them; never share a raw mutable cache among concurrent guest calls. Publish a result only if its requested revision/generation still equals the current document revision. An older long-running computation must be discarded rather than overwrite a newer result.

The cold, disabled-cache, warm-cache, diff-hit and persisted-projection paths must produce the same canonical inference bytes. Cache statistics are diagnostics/evidence, never semantic output.

### 3. Semantic commands, CQRS and event sourcing

Primary files:

- `.../🧬️schema/🧬️mutations/🦀️component.rs` and all its existing facets/codecs;
- `.../🧬️schema/🔺️diff/🦀️component.rs` and all its existing facets/codecs;
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`;
- the causal actor/backbone files under `🧰️framework/🛍️products/💻️os`.

The store already provides the correct command boundary: apply/amend/apply-in-lane/undo/redo materialize a new snapshot and record forward/inverse envelopes. Integration must hook inference after successful materialization, never before validation and never from UI-local state.

Important current mismatch: `MutationEnvelope.diff` and `.inverse` are documented/type-carried as opaque schema-tagged operation binaries, not as structural `GltfDiff` values. Dependency invalidation therefore cannot read `envelope.diff` directly. The artifact-specific projection must decode the forward operation against the causally current base, plan/validate it, obtain its structural diff and touched paths, apply it, then infer the post-state. The same applies to decoded inverse operations during undo/rollback.

The current index operations are unsafe semantic commands:

- inserting/removing an accessor changes primitive attributes/indices/targets, animation sampler input/output and skin inverse-bind references;
- inserting/removing a node changes scenes, child lists, skins and animation targets;
- mesh, material, buffer and buffer-view indices have corresponding reverse references;
- insertion currently clamps positions and missing set/remove targets become silent empty effects.

Replace these with validated semantic command outcomes containing preconditions, exact diff, exact inverse, diagnostics, touched paths and reference remaps. Deletion must explicitly select reject-if-referenced, cascade or replacement. Accepted operations must leave no dangling reference. Rejected operations emit no event and do not schedule inference.

For concurrent/remote causal application, positional operations require a real `MutationTransform` policy or stable semantic addresses plus base fingerprints/preconditions. This repository forbids CRDTs; use deterministic causal transformation/conflict rejection. Test the same concurrent batch under every permitted delivery ordering and require convergent snapshot and inference bytes.

Projection triggers:

| Store transition | Inference action |
|---|---|
| initial/open/analyze | cold inference for materialized revision |
| local apply/amend | use planned structural diff and post-state |
| remote envelope | decode/transform/apply in causal order, then use resulting structural diff |
| undo/redo | use decoded inverse/forward diff respectively |
| checkpoint/branch switch | select revision-scoped session or cold rebuild |
| rejected/conflicting command | retain current result; publish localized diagnostic only |
| configuration/policy change | authored state unchanged; invalidate policy-dependent fields |

Keep the last complete result visible while recomputation runs, but mark it stale with the revision it represents. This supports short connection shortages without freezing or misrepresenting the current document.

### 4. Type-erased runtime registry

Primary files:

- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` (`ArtifactInferenceDescriptor`);
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (`ArtifactCodec`);
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (`ArtifactDeclaration`, `ArtifactInferrer`);
- GLTF artifact declaration `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️component.rs`.

Today descriptors and codecs are sibling process-local registries. The descriptor proves source text exists; it cannot execute the inferrer. Add a type-erased inference service entry registered by `ArtifactDeclaration`, analogous to a composer entry, with:

- owner/plugin ID, artifact kind, document schema and inference schema;
- inference schema/algorithm version and supported policy version;
- canonical descriptor/presentation metadata;
- cold inference function;
- incremental inference function accepting current snapshot bytes, optional touched paths/revision/session identity and policy bytes;
- canonical inference text/binary encode/decode functions;
- cancellation/work-budget support and structured diagnostics.

Native and component guests must traverse the same serialized request/result contract. Registration must be deterministic. Duplicate identical registration may be idempotent; a conflicting owner/schema/version must fail loudly. Current global `HashMap::insert`-style catalogues risk silent last-writer replacement under parallel plugin initialization.

The descriptor needs localized presentation metadata keyed by stable inference field IDs. Do not overload `ArtifactKindSpec.name: String` or derive labels from Rust/GraphQL names.

### 5. WIT guest/host boundary

Primary file:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`

The world currently exports manifest, app instantiation/exchange, migration, guard clearing, artifact dialect listing and artifact composition. It has no schema/inference export. A zero-app stdio plugin cannot rely on app `exchange` to surface derived data.

Add artifact-level exports, using canonical `list<u8>` envelopes rather than a giant WIT record coupled to GLTF fields:

```wit
list-artifact-inferences: func() -> result<list<u8>, plugin-error>;
artifact-infer: func(request: list<u8>) -> result<list<u8>, plugin-error>;
```

The request identifies owner, artifact/document/inference schemas, revision/generation, canonical snapshot pack, policy and optional changed paths/session token. The result identifies the same revision/generation and contains canonical inference binary, diagnostics, completion/quality state and optional cache evidence. Omitted changed paths must remain correct by forcing a conservative/cold computation.

On plugin load the host queries and merges inference descriptors once, keyed by plugin ownership. On document projection it calls `artifact-infer`, validates the echoed revision/schema/version, decodes the canonical result and publishes only if current. Guest failure, cancellation or unload retains the prior result as stale and exposes a diagnostic.

The WIT instance has an explicit top-level instance guard and `clear-instance-guard`; inference calls must participate in the same serialization/reentrancy policy. Do not issue concurrent calls into one guest instance. Parallelism is safe across separate plugin instances or inside deterministic host/guest geometry workers which do not re-enter WIT.

Add native/guest parity tests that register the same GLTF implementation through both paths and compare descriptor and inference bytes. Version the wire request/result independently of the geometric schema.

### 6. Schema facet completion

Primary inference directories:

- `.../🧬️schema/💡️inferences/` Rust, TypeScript, GraphQL, JSON Schema and Proto leaves;
- `.../🧬️schema/💡️inferences/📝️text/` Rust, TS, GraphQL, JSON, Proto, EBNF, ANTLR and Semio grammar leaves;
- `.../🧬️schema/💡️inferences/💾️binary/` Rust, TS, ABNF, Kaitai, Spicy and Semio protocol leaves.

All facets must encode the same canonical model and version. The current text grammar accepts an opaque payload and the binary files merely declare a magic/protocol; neither is a runtime codec. Required representation rules:

- **Rust:** canonical domain model, availability/quality/provenance/diagnostics, optional measurements, stable entity addresses and deterministic ordering; no foreign runtime types in public API.
- **TypeScript:** readonly discriminated unions and fixed vector/tensor shapes; exported parser/decoder result types owned by this repo.
- **GraphQL:** explicit vector/tensor/quantity/status types, stable IDs, no unbounded anonymous numeric lists where dimensionality is fixed, and a query/resolver returning inference by document revision and policy.
- **JSON Schema:** `$defs`, exact required/optional semantics, finite JSON numbers only, stable version and derived annotations. Undefined or invalid metrics are status plus absent value, never `NaN`/`Infinity`.
- **Proto:** presence-bearing messages/`optional` values, deterministic repeated keyed records rather than unordered maps, fixed field numbers, and reserved removed numbers.
- **Text:** real canonical parse/print for the complete inference, with golden and malformed tests. It cannot remain `OCTET+` passthrough.
- **Binary:** real encoder/decoder with version, length bounds, canonical order, malformed/truncated rejection and golden bytes. Handwritten mutation ordinal tags must likewise be frozen explicitly rather than depend on enum declaration order.

Add schema-parity tests that enumerate every canonical field/variant/status/unit in all five main facets and both wire facets. Descriptor SDL/JSON/proto source presence is not parity evidence.

### 7. UI projection, accessibility and localization

Primary framework surfaces:

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` (`UiKeyValueNode`, `UiInspectorFieldGroup`);
- shell/renderer files under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer`;
- manifest/schema TS and Rust presentation types.

No production code currently consumes the inference registry. Implement a host-owned, artifact-neutral `Analysis`/`Geometry` read-only inspector backed by the runtime inference projection. A zero-app artifact must still display results, so it cannot depend on a GLTF-specific app panel. The existing generic artifact or inspection tab and `UiInspectorFieldGroup`/key-value nodes are the appropriate presentation boundary.

UI state must distinguish unavailable, pending, complete, stale, partial, unsupported, invalid and failed. Display the represented document revision, policy/method, units, quality/coverage and diagnostics. Large per-instance/per-pair results need collapsible/virtualized sections; histograms require a textual/table equivalent and must not be chart-only. Values and statuses must be screen-reader accessible, keyboard navigable, focus stable across refresh and not encoded by color alone.

All category, quantity, unit, quality and diagnostic labels use `LocalizedLabel` keyed by stable semantic IDs, with complete English and German strings and resolution through the active locale. There is no implicit language. Diagnostic inference payloads carry stable code/parameters, not localized prose. Plugin-provided descriptor labels are validated for both locales before registration.

UI concurrency contract:

1. document event increments generation;
2. panel renders last result as stale and schedules/cancels work;
3. async result includes requested generation;
4. host discards a mismatched generation;
5. matching result replaces the projection and emits one UI refresh;
6. cache-only activity never emits authored document events.

### 8. Fixtures and consumers

Extend existing GLTF test regions and the existing demo/metabolism examples; repository instructions prohibit adding standalone test/example files. Construct analytic cases in the existing Rust test modules and update the existing GLTF/GLB assets only where an external binary fixture is necessary.

Fixture matrix:

| Family | Minimum cases and exact truths |
|---|---|
| basic measure | unit cube, rectangular box, tetrahedron, thin plate, slender rod; analytic AABB/area/volume/centroid/inertia |
| transforms | translation, rotation, nonuniform/negative scale, matrix-vs-TRS invalidity, hierarchy, multiple mesh instances |
| accessor/storage | indexed/unindexed, interleaved stride, normalized components, sparse overlay, dishonest min/max, missing/external/truncated buffer |
| topology | open plane, watertight shell, torus/handle, disconnected components, inverted face, degenerate triangle, nonmanifold edge |
| concavity/void | convex body, concave L/U body, nested closed shell, re-entrant surface |
| relations | separated, touching and intersecting cubes; known clearance/contact/interference and adjacency graph |
| orientation/shape | principal-axis box, symmetric/mirrored/rotational/repeated assembly, rough/wavy surface |
| source parity | equivalent `.gltf`, `.glb`, internal text envelope and pack envelope |
| mutation | every semantic command, reference repair policy, inverse, diff regions, stale precondition, concurrent transform/conflict |

Audit every GLTF producer/consumer after schema-ID cleanup and semantic mutation changes. Direct consumers found include stdio semio mesh/animation, process3d, CAD, procedural3d, remodel, lowpoly, GIS terrain and puzzle 3D. They must use public artifact interfaces/constants and never pattern-match a removed ad hoc operation or assume `s.stdio.glb`.

### 9. Glue, Bun, Nx and launch integration

Primary files:

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`;
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📋️project.json`;
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`;
- `.vscode/launch.json`.

`📦️glue.rs` manually mounts GLTF inference and mutation leaves around the current `artifacts::gltf::standards::v2_0::subsets::any::schema` region. It is a central collision point; update it once after contracts stabilize, with one integration owner.

The current Nx `namedInputs.default` is:

```json
["{workspaceRoot}/✏️s/🔌️plugins/🗄️stdio/**/*.rs", "{projectRoot}/**/*"]
```

The project root is only the Rust package. Changes to GLTF TypeScript, GraphQL, JSON, proto, grammar, protocol and fixtures outside that package do not invalidate cached `test`, `test-quick` or `test-long` targets. Expand named inputs to the complete stdio artifact tree or explicit schema/fixture extensions. Keep `test-exhaustive` uncached until all generated/wire inputs are proven complete.

Extend the existing `📜️script.ts`; do not create another script. Add routed commands for a focused GLTF geometry gate and an end-to-end runtime evidence scenario. Add Nx targets which only call `bun ./📜️script.ts <command>...`. Register both in `.vscode/launch.json` beside:

- `⚖️gate🗄️stdio-catalog` (order 410);
- `⚖️gate🗄️stdio-lossless-roundtrips` (order 410.05).

Suggested registrations:

- `⚖️gate🗄️stdio-gltf-geometry` at 410.01;
- `⚖️gate🗄️stdio-gltf-runtime` at 410.02;
- retain exhaustive lossless roundtrips after them.

Permanent evidence output should be structured and stable, for example revision, cold/warm/reuse/recompute counts, result digest, command outcome and UI projection generation. Temporary exploratory logs must use `[DEBUG]` and be removed after evidence is captured.

## End-to-end verification matrix

No row is complete without the stated observable evidence. “Compiles” is not runtime evidence.

| Gate | Scenario | Required assertion | Required evidence |
|---|---|---|---|
| accessor | sparse/interleaved/indexed analytic mesh | decoded vertices/indices equal fixture truth | focused Rust test values and diagnostic-free decode |
| analyzer | GLTF JSON and GLB pair | canonical snapshots and inference bytes agree | printed snapshot/inference digest pair |
| geometry basic | analytic box/tetra/plate | dimensions, area, volume, centroid, PCA and inertia within declared tolerance | per-indicator expected/actual/tolerance output on failure |
| geometry validity | open/nonmanifold/degenerate/unresolved | unavailable/partial states are exact; no fabricated zero or nonfinite number | serialized status/diagnostic codes |
| topology | torus, open plane, disconnected shells | Euler/boundary/genus/components match truth | canonical topology result bytes/text |
| relations | separated/touching/intersecting solids | clearance/contact/interference/degree match truth | ordered pair records and graph digest |
| determinism | repeat under varied worker counts/order | canonical inference bytes identical | digest for each worker count/order |
| cache cold/warm | same revision/policy twice | same bytes; second run reuses expected stages | hits/misses/computations/evictions plus digest |
| cache selective | node transform only | accessor/mesh-local stages reused; instance/aggregate/relation stages recompute | stage-level cache counters and touched paths |
| cache storage | buffer splice | only dependent accessor/primitive/ancestors and relations recompute | dependency trace, not only aggregate hit count |
| cache policy | tolerance/sample policy change | authored revision unchanged; affected fields recompute; result records policy | old/new policy fingerprint and digest |
| cache disabled | disabled vs enabled | result bytes identical | two digests and cache-disabled stats |
| semantic command | every accepted variant | exact intended state, no dangling refs, nonempty exact touched paths | command ID, before/after digest, validation status |
| rejection | stale/missing/referenced target | no state event, no inference generation change | typed rejection and unchanged revision/digest |
| inverse | command then inverse | byte-identical original snapshot and inference | before/roundtrip digests |
| diff law | plan/apply/absorb | mutation application equals structural diff application | property/law suite |
| concurrency | concurrent inserts/removes/reparents | all accepted delivery orders converge or deterministically reject | final snapshot/inference digests and conflict codes |
| local CQRS | store apply/amend/undo/redo | one projection per resulting revision, correct stale/complete transitions | event/revision/generation trace |
| remote CQRS | causal envelope delivery/reconnect | causally materialized state drives same inference as local replay | local/remote snapshot and inference digests |
| checkpoint/branch | switch revisions | no cache leakage across branch/revision | session key and result digest trace |
| schema Rust/TS/GQL/JSON/proto | full model enumeration | every field/variant/unit/status has one equivalent representation | parity test count and schema-version digest |
| text codec | full result and malformed inputs | canonical print/parse roundtrip; bounded rejection | golden text digest and negative cases |
| binary codec | full result, truncated/unknown version | canonical encode/decode; deterministic bounded rejection | golden byte digest/length/version |
| descriptor registry | native and guest registration | identical descriptor; conflicting owner rejected | catalogue digest and conflict diagnostic |
| WIT | guest infer request | result echoes schema/revision/policy and equals native bytes | native/guest digest comparison |
| WIT lifecycle | cancellation/unload/reload | no stale publication or leaked session | generation trace and recovered complete result |
| GraphQL | query by document/revision/policy | typed result/statuses, no NaN, revision matches | query response snapshot checked in existing tests |
| UI EN | open generic analysis inspector | all categories/statuses/units visible and revision-correct | runtime tree/screenshot plus accessibility assertions |
| UI DE | switch locale | complete German labels; numeric semantics unchanged | UI tree label assertions and same inference digest |
| UI stale race | slow old request, fast new request | old result discarded; last complete marked stale until replacement | generation/revision console trace |
| source compose | inferred snapshot composed to GLTF/GLB | inference never contaminates authored source | roundtrip source/snapshot digest |
| Nx cache | change each facet/fixture extension | appropriate target invalidates | Nx input/affected evidence |
| launch | run both registered gates | zero-touch from launch config on supported platforms | exit code and structured runtime evidence |

Final release gate:

```text
bun/nx focused geometry gate
  -> schema/wire parity
  -> mutation/diff/inverse/reference laws
  -> cache selectivity and determinism
  -> native/guest runtime parity
  -> local/remote CQRS replay
  -> localized accessible UI projection
  -> exhaustive GLTF/GLB lossless roundtrips
```

Run on native macOS/Linux/Windows and the devcontainer through registered launch configurations. Any platform-specific floating-point divergence must be eliminated by the deterministic numeric contract, not accepted as per-platform goldens.

## Maximum-parallel workforce plan

The active agent system allows four concurrent slots including the coordinator. Use three implementation agents plus one coordinator/integrator per wave. More simultaneous writers would increase collision risk without increasing throughput because the core roots are monolithic.

### Freeze barrier A — contracts

Before implementation, freeze schema IDs, inference schema/version, semantic command tags, touched-path vocabulary, policy defaults, units/validity/quality types and wire request/result version. Record them in the ticket. No agent starts facet or runtime code against an unfrozen contract.

### Wave 1 — foundations, three parallel agents

1. **Geometry foundation:** resolved resources, accessor/index decoding, canonical primitive/transform/topology input and analytic tests. Owns IO and bounds/kernel regions only.
2. **Mutation foundation:** semantic command planner, reference graph/remaps, exact diff/inverse and `DiffRegions`. Owns mutation/diff regions only.
3. **Runtime foundation:** type-erased inference entry, cache/session scoping, projection state and WIT wire contract. Owns framework runtime/WIT regions only.
4. **Coordinator:** reviews contract conformance, runs affected Nx gates, owns no overlapping production region.

Barrier B requires decoded analytic geometry, valid semantic diff paths and a native type-erased cold inference call.

### Wave 2 — indicator shards, three parallel agents

1. exact size/area/volume/centroid/inertia/PCA/topology;
2. curvature/thickness/roughness/normal/sharp-feature indicators;
3. hull/concavity/symmetry/repetition and spatial pair/contact/clearance/interference graph;
4. coordinator owns final aggregation ordering, policy salting and cache stage graph.

Agents edit disjoint regions inside the existing inference files. Only the coordinator assembles the root `GltfInference` and changes shared imports/re-exports.

Barrier C requires cold results for every requested indicator and deterministic analytic fixture results before cache optimization.

### Wave 3 — representations and transport, three parallel agents

1. TypeScript + GraphQL + JSON Schema parity;
2. Proto + text grammar/parser/printer;
3. binary protocol/encoder/decoder + WIT native/guest parity;
4. coordinator owns descriptor assembly and schema-version/golden digest freeze.

Barrier D requires representation enumeration parity and executable text/binary codecs, not source-string presence.

### Wave 4 — CQRS, UI and consumers, three parallel agents

1. local/remote/undo/redo/checkpoint projection hooks and race/cancellation tests;
2. generic accessible inference inspector plus EN/DE localization;
3. GLTF producer/consumer schema-ID and dialect audit, existing fixture updates;
4. coordinator owns event-to-projection acceptance tests.

Barrier E requires the same result digest through direct, native store, remote replay and guest paths, plus runtime UI evidence.

### Wave 5 — zero-touch integration, three parallel agents

1. focused/analytic/property/exhaustive test target shaping;
2. Nx inputs, `📜️script.ts` routing and launch registrations;
3. platform/runtime evidence collection and performance/budget/cancellation stress;
4. coordinator alone updates `📦️glue.rs`, shared manifests and final golden versions, then runs the complete launch gate sequence.

At every barrier, re-read files before editing and never assume another agent's in-memory view is current.

## Dirty-tree and concurrency hazards

The tree is shared and already dirty: the ticket directory is untracked and `.🦑️repo/💬️prompts/🐙️ueli.md` is modified. Those changes are not part of this workstream. Do not use modifying git commands, stash, reset, checkout or worktrees.

High-collision files must have a single owner at a time:

- `.../💡️inferences/🦀️component.rs` root model/descriptor;
- `.../🧬️mutations/🦀️component.rs` and `.../🔺️diff/🦀️component.rs`;
- stdio `📦️glue.rs`;
- framework plugin/store/inference roots;
- `world.wit`;
- stdio `📋️project.json` and `📜️script.ts`;
- `.vscode/launch.json`.

Use region ownership, small `apply_patch` changes, immediate compile after each integration and a coordinator-controlled barrier. Do not have agents independently regenerate or reorder schema facets. Freeze explicit binary/operation tags before parallel facet work.

Runtime hazards requiring tests:

- silent duplicate catalogue overwrite during parallel plugin registration;
- unsynchronized `InferenceCache`/session access;
- stale async inference publishing after a newer mutation;
- cache leakage between documents, branches, checkpoints or policies;
- non-deterministic `HashMap` iteration/parallel floating-point reduction;
- WIT re-entry into one guarded guest instance;
- positional GLTF reference corruption and concurrent index transport;
- relation metrics whose neighborhood dependencies extend beyond the locally edited primitive;
- Nx returning cached success after a non-Rust facet/fixture change;
- UI refreshing from cache activity as though authored state changed.

## Baseline evidence gathered during planning

- `bun nx show project @semio-tech/stdio-plugin --json` resolved successfully and confirmed `test`, `test-quick`, `test-long` and uncached `test-exhaustive` all route through the existing Bun script.
- The resolved named input only includes all stdio `*.rs` files plus the Rust package root, confirming the non-Rust facet cache hole.
- No test target was run by this planning workstream; this report does not claim the existing GLTF suite passes.
- No production code was edited by this workstream.

## Definition of done

The feature is done only when a real GLTF or GLB can be analyzed, semantically edited through the event-sourced store, incrementally re-inferred with dependency evidence, transported through native and WIT plugin paths, queried through schema surfaces, and displayed in an accessible English/German generic inspector; when every requested indicator carries correct units, validity, quality and provenance; when reference-safe semantic mutations converge or reject deterministically; and when all routes produce the same canonical snapshot and inference digests under the registered Bun/Nx/launch gates.
