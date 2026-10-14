# Jack Retained Graph Interface Evidence

## Completed Boundary

Jack's TypeScript topology and flat-position inference now consume the schema-first `JackGraphQuery` behavioral port. Its fields are retained `ArtifactChild` identity, iterable literal node IDs, iterable literal edge IDs/endpoints, and a numeric displacement query. The port is owned by the Jack inference consumer because root selection and u/v displacement semantics are intrinsic to Jack. It does not clone Stdio's node/edge records, value representation, ports, metadata or serialization schema.

Admission compares childId, artifactId and all three dialect coordinates to the persisted parent's selected child. The intrinsic algorithm accepts any matching graph owner. Unique node and edge IDs, retained endpoints, deterministic FIFO Kahn traversal, stable edge-order component walks, literal prototype/Unicode IDs and explicit root retention remain enforced. Cycles preserve the existing contract: topology reports cycleFree=false and excludes blocked nodes from order/depth; flat positions retain the deterministic connected-component walk. No new cycle-free result is invented and no legacy signature is retained.

The specific s dev Trinity composition owns exact s.stdio.semio/v1/graph selection and typed numeric u/v interpretation. `bindSemioJackGraph` retains the source graph and exposes queries; it indexes original edge references without copying the rich snapshot. The two former Jack/Semio integration tests and their topology/flat fixture files moved out of Rewriting into this authored composition. The composition has a permanent jack-graph-check command, Nx target and matching authored/generated launch row.

## Executed Verification

- Test-first scoped Nx exec ran current intrinsic source before public APIs were added: 1 pass / 5 fail, with the expected missing public inference functions. An initial fixture URL typo was corrected before recording this genuine implementation red.
- Final actual Jack permanent package script, invoked by the supported repository Nx bootstrap through the existing ticket-fixed-slots seed, completed successfully: snapshot suite 38/38, AST wire suite 2/2, query suite 9/9; 49 tests total, zero failures. It built the published bundle and declarations, compiled the generated consumer and strictly checked all three registered suites. Runtime output: `[artifact-typescript] tested @semio-tech/trinity-jack exports=12 suites=3`.
- Final scoped architecture-source-verification:test ran the current intrinsic query and specific Semio composition files: 11/11 tests, zero failures, 74 assertions, skip-nx-cache. These fixtures compare topology/depth/flat outputs against an independent Graphology MultiDirectedGraph oracle and existing independent SQLite composition witnesses. Graphology oracle keys are injectively encoded so the library's prototype-key adjacency implementation cannot corrupt literal __proto__/constructor IDs; product IDs are unchanged.
- Final architecture-source-verification:strict completed successfully against both query/composition test roots and their production transitive closure with strict, noEmit and bundler resolution. No stale source copies were used.
- The current public Jack source bundles successfully while an esbuild onLoad guard refuses every source under the removed Stdio plugin. The GraphQL parser independently checks the port/endpoint fields. This is a TypeScript public-source closure check; it does not prove Rust deletion independence.
- The supported bootstrap dispatched the actual specific permanent jack-graph-check command: its source check and both Semio/Jack laws passed, with `[s-dev] checked retained Semio/Jack graph composition` runtime output.
- Authored launch seed and generated launch configuration each contain exactly one matching JackGraph command; the Nx target points to `bun ./📜️script.ts jack-graph-check`. Maximum new/updated path length is 159 characters, below 256.

The attempted canonical root `bun nx run @semio-tech/trinity-jack:test` could not prepare the contended repository graph and ended with “No cached ProjectGraph is available” after approximately 190 seconds. Its success is not claimed. The successful package run above uses the exact current permanent script implementing that target through the supported Nx/bootstrap seed route and an isolated ticket-owned Nx data/cache directory.

An exploratory broad current-source run of Jack + Rewriting + the new query/composition suites had 104 tests, 87 passes and 17 failures. One Jack fixture-base URL lacked a trailing slash; this bounded adjacent test defect was fixed, and the final full Jack suite passes. The other 16 failures in that exploratory run concerned Rewriting's missing schema references, existing fixture URL separators, unavailable parseSemioGraphJsonValue export and native-source assertions. No whole Rewriting green is claimed. A wider strict check including the entire s dev script also reached unrelated existing TSX/implicit-any errors; the focused production/query strict closure passes.

## Exact Remaining Rust Dependency

No Rust source was edited or Rust runtime tested. Jack still directly owns a concrete Stdio child in `🪆️content/🦀️.rs`: line 10 exports `JackContentChild = ArtifactChild<SemioGraphSnapshot>`; lines 14–15 expose the concrete snapshot through JackContentOwner construction/access; line 103 hardcodes the Semio dialect in jack_content_for_handle; line 151 exposes SnapshotReadRef<SemioGraphSnapshot>; line 168 accepts SemioGraphMutation leaves. Native inference topology calls this retained concrete owner; native flat-position additionally imports SemioGraphEdge and SemioValue. Public viewer render, editor content_to_workflow, executor preparation step and reorganize tool layout/build_job APIs also accept the concrete Semio snapshot. The Jack Cargo manifest still declares Stdio Semio, Semio base, Semio graph/value and concrete IO artifact dependencies. The top-level Rust app aggregate trait also requires VcsArtifactApp<...,SemioMembers>. This broader native ownership/API composition requires a separate complete schema-first native refactor; the TypeScript result must not be presented as whole-plugin/native deletion independence.

## Attribution

Paths below identify this subtask's work even when concurrent external commits change Git's visible baseline. No modifying Git commands, worktrees or AGENTS.md edits were performed. Ticket lifecycle remains owned by the parent task.

### Created

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪆️graph-query/🔗️.graphql`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪆️graph-query/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🪆️query/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🧫️fixtures/🪆️query/🔣️.json`
- `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🟦️.ts`
- `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/🟦️.ts`
- `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/🧫️fixtures/🧭️topology.json`
- `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/🧫️fixtures/🎛️flat.json`
- `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/🧫️fixtures/🪆️retained.json`

### Updated

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`
- `✏️s/🧑‍💻dev/📜️script.ts`
- `✏️s/🧑‍💻dev/📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`

### Removed

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/💡️inference/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/💡️inference/🎛️flat/🔣️.json`

### Tested Source Digests

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪆️graph-query/🔗️.graphql`: `188f647350cf10e125ffb44f4b183b771f502342653a8355274817c64c9ca810`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪆️graph-query/🟦️.ts`: `dc107ba2ddf25368fba6f81bdedfe57802e07c6e94d50413be92745057e20991`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🪆️query/🟦️.ts`: `fb0b5be7a113198134ea735fe47daec7b0e8ac12c46c8bfe2748ea79638300df`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🧫️fixtures/🪆️query/🔣️.json`: `a7df837c7f7e5445dd63784dc2d5cf51ff8c376bbf151d9c8e0b4f38d0c8df6e`
- `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🟦️.ts`: `c87a93eb82b0196b2e10a0890eccb6d4107dffd738c2ec48d804dd05a52b1330`
- `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/🟦️.ts`: `03657858ffae8af6e711cfb6c55109a5c2318d3d56045751cd152f1201368ba4`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🟦️.ts`: `0017a1934a78c554a0f4643807175a0da27d5ba9ad525caf514ef4d15f090b71`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🟦️.ts`: `1174eb3432dd97b1413b8ef7b8446aa16a766f33dde79113448a1b6edd395bb6`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🟦️.ts`: `991a1a7353ffc48b32b471b9f74d08b6919d9e851707d2725ad7365ea65ced92`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🟦️typescript/📜️script.ts`: `b6b9f9f29a2f817d2563708b9a71ab3b33cf0a8074ac979fc0402bbdae0fe162`
