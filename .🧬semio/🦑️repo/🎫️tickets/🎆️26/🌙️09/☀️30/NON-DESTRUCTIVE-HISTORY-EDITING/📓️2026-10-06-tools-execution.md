# Tool Execution and Input Metadata

## Scope and Current Sources

Read repository/plugin AGENTS, design §22.32 and §23, and S5 tools/acceptance reports. WFC grid2d/grid3d and block3d already emit framework `tool_once` transactions; no duplicate rewrite was needed. Old demonstrator/fem/lowpoly bracket action names appear only in generated hub descriptors, not canonical source verbs; coordinator owns descriptor rebuilding.

## Implemented

Architect uses the closed shared node-graph row decoder, rejects malformed/unknown-node batches, and emits one transaction per connect/disconnect/delete batch. Repeated disconnects/deletes and an already-connected pair yield no redundant mutations. A local adjacency ledger makes connect-then-delete produce applicable ordered leaves. The action decoder rejects unknown root fields. Graph movement, sliders and variadic ports remain refused because this artifact has no corresponding persisted fields.

Puzzle3d/5d transform drivers implement `GestureChart` and use the shared gesture driver. The committed action path accepts `once`, suppresses zero-offset yields, and obtains a fresh transaction identity. These are context-free once actions; the helper supplies no persisted window gesture slot. This report does not claim streamed window-slot integration.

The framework one-step driver now advances the process-wide logical clock, so many independent dispatches in one millisecond cannot reuse a transaction ID. A native uniqueness law covers 256 calls.

GIS inputs now directly declare localized metadata for insertion/destination indices, feature identity/data and imported maps. The nested imported-map collection and property-member labels were necessary: the real manifest reader exposed absent labels after the independent validator resolver was repaired. Playbook procedural payload parameters now carry direct localized metadata, completing the coordinator's one bare input finding.

New language-neutral graph/transform/GIS cases are consumed by native or existing TypeScript implementations and independent Graphology/Ajv, XState and jsonschema oracles. Tool-machine fixture envelope schemas are now owned by the testing corpus: conformance tests no longer refer to three fixture definitions absent from the runtime schema.

## Executed Validation

| Nx Target | Result |
|---|---|
| `@semio-tech/architect-js:test` | 11 passed, 0 failed, including nine graph cases |
| `@semio-tech/gis-js:test` | 8 passed, 0 failed, including four input-metadata cases |
| `@semio-tech/puzzle-js:test -- transform-gesture` | 5 passed, 0 failed |
| Tool-machine source conformance through Nx | 36 passed, 0 failed; Ajv/XState/fast-check and closed row oracle agree after moving removed fixture definitions to test-owned schema. Native test-quick also passed 40 selected tests,0skipped |
| Tool-machine native test-quick | 40 passed, 0 skipped; target succeeded |
| Architect graph native law | Compile-red on existing stdio duplicate modules; repaired modules and current rerun pending |
| Puzzle3d/5d phase native laws | Compile-red on Store static slice lifetime; repaired and current rerun pending |

## Every-Editor Matrix

Current source discovery finds 115 acceptance macro registrations across 93 artifact crates. This is source coverage, not runtime proof. The reusable `native-matrix/📜️script.ts` discovers canonical macro owners, reads the actual Cargo feature declarations, then invokes the existing repository Cargo test contract through Nx. It includes stdio and separates crates requiring `component-app-assembly` from crates which do not declare that feature. Filter `history_edit` runs edit/inputs/cascade and composed child-history laws where declared. Canonical launch registrations live in `.vscode/🧩️launch.seed.jsonc`.

The initial mixed-feature run was canceled before compilation because some selected crates do not declare that feature. An inline Nx exec attempt failed from Nx shell quote stripping before any Cargo execution; the saved command avoids inline JavaScript quoting. Current all-editor run is pending. The initial93-crate discovery omitted three composed-only applications. Wires, imperative and DAG each declare a child-history acceptance law rather than a parent-leaf law; discovery now includes them. The149 editor files consist of148 concrete implementations and theZIP shared targeting utility, which implements no ArtifactEditor. Thirty concrete stdio subset editors lacked direct acceptance registrations (ninePDF profiles, fourDOCX/PPTX profiles and17 Semio subsets); each now declares the existing generic law in its own unit tests. There are now145 direct history-edit law registrations plus three child-only editor implementations across96 artifact crates. The historical96-crate table must not be treated as current executed results.

O6 camera source revalidation: puzzle2d reads the rendered window's own configuration partition; current laws explicitly cover two windows and reload, session-only undo, and immediate camera publication. Structural/history rerender acceptance is not yet executed in this pass, so the historical live reset is neither declared fixed nor reproduced here.

## Changed Owners

Architect graph command/editor decoding, shared graph fixtures/tests and TS test router; puzzle3d/5d transform utilities/editor calls/tests, shared phase fixture/oracle/config/test router; GIS canonical mutation/imported-map schemas and fixture/oracle/router; playbook procedural payload schema; framework tool-machine logical clock/native uniqueness law/corpus schema/conformance oracle; repository Cargo preparation selector forwarding/queued owner package closure and neutral fixture/schema/oracle; ticket native matrix script; canonical launch seed and its generated launch file.

Generated logs remain under `🗑️generated/tools-execution` while native processes are active; coordinator will retire ticket outputs after recording final results.

## Added Acceptance Registrations

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/✏️editor/🧪️tests/🔬️unit/🦀️.rs`

## Scoped Native Preparation

The matrix revealed that grouped native tests forwarded only the first manifest into preparation. Preparation consequently visited every package in that workspace and every dependency workspace. The repaired invocation forwards each selected package name, defaults an unqualified child manifest to its own package, and traverses only each selected package’s authored local dependency paths. Workspace-only manifests retain complete workspace preparation. Member inventories and parsed documents are held only within that invocation; queued ownership and publication remain unchanged.

A new language-neutral selected-preparation corpus exercises explicit selection, implicit child selection, and workspace fallback. The independent Cargo resolved dependency graph and TOML parser confirm the expected closure; Ajv validates the closed corpus. Before the implementation, Nx contract-check produced 15 passing tests and one failing test: unrelated model-b was prepared alongside selected model-a. After the change, the same target passed all 16 tests with 428 assertions in 2.49 seconds. The queued preparation contract also passed one test with six assertions in31.66seconds, confirming a queued owner survives longer than one active recipe budget.

The first matrix compilation selected 39 non-assembly crates but ran zero acceptance assertions because the concurrently updated shared store failed compilation: CursorRevisionAccumulator omitted indexed_edits/mutation_positions/unit_flags at durable-group line557, and retained-tail cache consumed pre_snapshot before further use at store line20213. These failures have been referred to the shared store owner. The 57 assembly crates had not selected runtime assertions when that run was canceled. Fresh scoped matrix and focused architect/puzzle-chart native runs are now launched. No source census is presented as runtime verification.

The matrix now requests per-test pass/final-all Nextest reporting and retains binary metadata under the ticket generated directory, enabling concrete registration-by-registration selection evidence rather than relying on aggregate success. The current pending run log is native-matrix-repaired.txt; earlier selected runs stopped in shared compilation before assertions.

The puzzle2d camera target completed with zero selected tests because shared semio-framework-artifact-infinite-dag failed compilation: BorrowedDslField is missing from DagNodeKind, PropertyBag (five uses), DagPreviewContent and DagExpandedPaths (nine E0277 errors). This is a shared borrowed-DSL rollout blocker and has been referred to the coordinator; no camera runtime behavior is claimed.

## Shared Borrowed DSL Compile Triage

Current DagNodeKind and DagPreviewContent already derive DslEnum. DslEnum emits BorrowedDslVariants metadata, while scalar record fields require BorrowedDslField. The derive owner must expose tagged enum shapes through the correct borrowed field role rather than treating them as scalar enums. PropertyBag is an authored Vec of key/value members with a handwritten value encoder; DagExpandedPaths is an authored sorted literal string set. Their borrowed field shapes need to match those existing encoded representations. These are shared framework ownership questions, so the coordinator was given the current compiler failures before any change to those concurrent sources.

The coordinator authorized repairs of the demonstrated borrowed-owner gaps. Existing DAG tagged_field now declares its borrowed Statements shape against the derive-owned variant table; PropertyBag declares Map(Value), and DagExpandedPaths inherits Vec<String>’s List(Text). Collection ArtifactBody supplies static document/blob variant metadata matching its existing handwritten flattened BlobRef encoding. ArtifactChild, ArtifactLink and the child/link target ArtifactRef now expose the same static records as their existing owned schema factories; LinkPin preserves its optional checkpoint/blob fields. The controlled Refusing test carrier declares Text. No shared trait blanket implementation or derive role changed. Existing neutral IO binding and collection native-schema corpus laws now compare borrowed metadata through the independent serde_json oracle. Focused IO and collection native targets are queued/executing, so these repairs are not yet claimed runtime-green.

A later ungated matrix compile failed two shared-store retirement erasure bounds (P lacks Sync at the two Box::new(retirement) casts); the core owner is settling those bounds. Optional temporary preparation stage timing is enabled only by SEMIO_CARGO_PREPARATION_DEBUG=1 and emits [DEBUG] lines to stderr. The Cargo selection contract rerun after instrumentation passed all16 tests,428 assertions in2.67seconds. The diagnostics will be removed after a real-root timing measurement.

The current real-root scoped matrix preparation measured discovery95 owners4.861s, root membership119 packages963ms and plugin membership138 packages157ms. Its transitive closure171 packages across17 scopes took6.357s, followed by membership publication4.072s (10.429s total). Temporary opt-in diagnostics were removed after this measurement. Earlier elapsed preparation observations included queue waits.

Current native architect compilation exposed preexisting duplicate Rust modules:3 duplicate TSV io mounts,18 duplicate Semio io mounts and duplicate brep/text diff_codec wrappers. Their source modification times predated the measured recipe; the Semio conversion recipe only publishes Cargo/dependency JSON authority and does not modify Rust. The compiler-demonstrated duplicate mounts were removed, the unique brep JSON decoder moved into the retained codec module, and the empty text wrapper removed. Fresh matrix compilation continues; no editor acceptance assertions have yet passed. The earlier focused IO command was rejected because its canonical router accepts an execution level only; it was rerun using its actual target without a test-name argument. Store LinkPin and Collection body metadata now use explicit static const field slices after E0716 red compilation.

The repaired matrix ungated group advanced to five Workflow E0277 errors: WorkflowMediaPort (two), MediaContract (one), WorkflowInput (two). The three local handwritten carriers now declare static borrowed records matching their flattened owned fields and reuse the existing canonical class/form/direction/multiplicity label tables. A new native law compares fields, optional flags and enum labels with the existing language-neutral manual-schema fixture through serde_json. Assembly group remains running; the repaired ungated group is queued separately. Collection current compilation also exposed one stale test-only crate::snapshot_sqlite namespace, now corrected to its canonical crate::io::sqlite::snapshot owner. Architect compilation additionally demonstrated27 codec imports colliding with locally defined functions; only those proven stale imports were removed.

The canonical IO schema native target completed successfully with9 selected tests,9 passed,0 skipped (Nextest74ms; whole Nx wrapper8m16 including queue/compilation). Its package root explicitly registers the ArtifactRef ownership corpus module containing the new borrowed metadata law. No artifact-editor acceptance result is inferred from this framework-only success.

Collection borrowed body corpus native check passed1 selected test,1 passed,24 unrelated skipped (Nextest18ms; Nx14m39 including queue/compile). The first run had failed the stale SQLite test namespace; the repaired run confirms the actual static variants. Current assembly diagnostics are classified separately in 📓️2026-10-06-native-matrix-compile-triage.md. Root owns remaining TSV8 fixes; local EPW/AVI/WAV/MP4/HTML and WFC bitmap namespace gaps are under this owner. The shared stdio SnapshotPatch borrowed field uses its existing structured Value carrier; its committed wire/locale law now also round-trips that field carrier. AVI Statements uses its derive-owned variant table; WAV borrowed records match its existing neutral manual-schema fields/tags. Core owns the two MP4 IndexedDiff alias delegates. No editor acceptance assertion has yet run.


## Current Native Capture and Further Canonical Owners

Workflow's new borrowed manual-schema fixture law completed natively:1 selected,1 passed,61 unrelated skipped (Nextest16ms, whole Nx17m05 including queue/compilation). The latest39/57 package matrix groups stopped before execution on13/10 kernel constructor arity/type errors during the coordinator's mandatory ActorId/stored-genesis representation wave. Focused Draw closed choices also stopped on the same10 kernel errors; no Draw assertion ran. The final matrix rerun waits for the core owner's stable-source boundary.

Architect's repaired compilation demonstrated73 remaining Semio/ZIP baseline errors. Fourteen Semio IO owners now consume their actual same-subset helper definitions rather than unrelated subset models; foreign schema references use canonical standards/subsets paths. Drawing's local schema binding module is native_binding to avoid collision with its schema wrapper component, preserving private wrapper boundaries. ZIP's six demonstrated missing imports now target its IO conformance/analyzer declarations and the actual base snapshot types.

The independently advanced puzzle build additionally demonstrated DWG borrowed-role and geometry IO namespace failures. DWG's handwritten XRecord/evaluation/visual-property carriers (and the table-entry generic witness) now publish borrowed metadata from the same authored dwg_metadata declaration, through a shared static shape branch. The existing neutral11-record JSON fixture and serde_json native metadata law compare five borrowed record descriptors and a generic record child. STL/PLY/OBJ native_pack references target their actual binary IO owner; PLY binary snapshot imports the four existing binary diff reader helpers. LAS errors from that earlier log are already repaired in current peer sources and were preserved. Current DWG canonical metadata target is in flight; no result is inferred before its completion.

The DWG canonical controlled-metadata target executed its existing independent TypeScript oracle:2 passed,194 assertions. Its native phase stopped before DWG compilation on five current kernel constructor errors during the ActorId wave, so neither the new borrowed native law nor existing native metadata laws ran in that attempt.


## Plugin Actor and Immutable Runtime Routes

The coordinator assigned all ✏️s constructor routes for the mandatory actor representation wave. Direct standalone ArtifactStore/alias fixtures explicitly supply ActorId; all15 public domain store guards require a caller actor and forward it to the exact store constructor. Semio member creation/opening helpers require and forward the caller actor; the two retained member-open test requests carry fixture actors. Eight custom initialization authorities retain the supplied actor, all25 editor/viewer initialization overrides receive it, and domain/bounded job delegates forward it. There are no handwritten SpaceMember implementations under ✏️s; the shared space_members macro publishes the coordinator's required borrowed local_actor_id authority.

Core specified immutable InitializationRuntime current as Arc<P> with other argument order retained; the eight custom domains and two Semio runtime fixtures now construct that owner explicitly. The existing Writer initialization law reads the shared language-neutral actor-genesis corpus through serde_json, initializes with its named opened actor, and checks the actual candidate actor. Its history_edit name makes it part of the final matrix selection. No native compilation or actor behavior result is claimed during the concurrent representation edits; broad reruns remain held until core announces its stable-source boundary.

## Immutable Genesis Domain Initializers and Explicit Fixture Actors

All eight retained initialization statecharts now bind `vcs.genesis.share_snapshot()` and the cached O(1) `vcs.genesis.digest()` before seeding history. No initializer encodes or hashes the initial snapshot during binding. The existing bounded domain clone cursors begin only when an effective forward requires a unique mutable workspace. Clone completion adopts the owned projection through `adopt_current_owned` and each initializer grants `settle_current_retirement_step` before replay resumes. Withdrawn forwards avoid mutable workspace access. Raster control admission remains an explicit retained step even for empty histories; Process3d validation census remains bounded before binding.

Applied and redo commits supply the exact stable `edits.key_at(edit)` history ledger key. Historical authors no longer overwrite the opened actor. Drawing no longer clones a historical author into an unused preparation cache. Common framework test context births now carry an explicit standalone fixture actor: 153 calls across 103 test owners, plus two viewer births. Production initialization jobs and overrides propagate the caller's required actor.

The neutral fixture `deferred-genesis/🔣️.json` describes empty, withdrawn, first-mutation and cancelled-copy journeys. Its XState oracle and eight source contracts are exposed by `native-matrix/📜️script.ts genesis-oracle`, registered in the canonical launch seed. The existing Writer/GIS/Jack/Raster publication laws now also assert that an empty initialized store aliases its original decoded genesis Arc and are selected by the history-edit matrix filter.

Validation status at this checkpoint: the initial unspecific Nx invocation stopped at the pre-existing DSL/value project cycle before assertions, and the framework-scoped retry stopped because its relative ticket script was resolved under that project. Fresh workspace-scoped invocations use absolute paths. Native matrix execution remains held until the coordinated core ActorId/immutable genesis constructor wave settles. These authored source contracts and runtime assertions are not yet claimed as runtime passes.

The fresh workspace-scoped absolute-path oracle completed successfully: 12 tests, 68 assertions, 0 failures (four neutral journeys with independent XState agreement plus eight source contracts). A separate workspace-scoped Rustfmt invocation parsed all eight domain initializer sources successfully. Both proofs are source/neutral checks; the new empty-open Arc alias assertions and broad editor matrix still await fresh native execution after the coordinated constructor wave.

A reported DSL native fixture mismatch was revalidated against the current source before editing: `variants_binary::RetainedDecodedOperation` exists, retained `encode_op_into` accepts `FieldProjectionSource`, and `decode_op_span` takes six arguments and returns `Result<(), ProtocolError>`. The existing unit fixture calls already match those current declarations, so no API reversal or compatibility shim was introduced. The earlier compiler diagnostics were produced against an older source version.

The 104 Rust test owners carrying the explicit common helper/viewer actors all passed an independent Rustfmt parse invocation. The two retained Semio value fixture births now also reuse their envelope's cached genesis digest and decoded Arc instead of cloning the seed and re-encoding it. The one manually authored Presentation history wire fixture now supplies its distinct `presentation-fixture` historical author; other manual production-history fixtures already supplied named authors and remain unchanged.

## Current Matrix and Canonical IO Compile Cohorts

After core's stable-source boundary and its 1/1 immutable-genesis native witness, the 96-crate/148-registration matrix resumed. Its first 39-crate group loaded an older shared DagStore actor omission, and the assembly group failed a shared FlowStore actor birth plus an MP4 test import. Root repaired both framework routes; the MP4 test now imports the actual IO analysis owner. That run selected zero editor assertions. A fresh all-editor run writes `native-matrix-pdf-dwg-current.log`, still compiling at this checkpoint.

Twenty-three direct VcsArtifactApp registry fixture births additionally supply the required explicit ActorId. No production route supplies a fallback actor.

The independent Draw choices native run exposed PDF412/DXF38/DWG3 compile diagnostics before its assertion. Root owns DXF; this owner repaired PDF canonical mutation payload/tag/IO conformance paths and the required sibling snapshot projection visibility, preserving native_pack/inverseRows payload laws. DWG's existing eleven authored metadata declarations now expose their borrowed role through the shared single declaration macro, and the neutral native descriptor comparison covers all eleven. The current independent source oracle passed2/2,194 assertions. Native metadata remains pending. Exact changes and provenance are in 📓️2026-10-06-pdf-dwg-compile-cohort.md.

Root's real activation next exposed SVG17 and GLTF38 compiler diagnostics. Root owns GLTF. SVG's exact canonical IO/XML/retirement/conformance owners are repaired, preserving existing codec behavior and corpus data. See 📓️2026-10-06-svg-compile-cohort.md. No live or editor runtime assertion is inferred from these compile repairs.

The completed loaded matrix reported170 compiler diagnostics and zero acceptance assertions; current source addresses that cohort and the fresh all96-crate run writes `native-matrix-svg-pdf-tests-current.log`. Follow-up SVG source corpus8/8,67 assertions and PDF source corpus54/54,1814 assertions passed. The later SVG38 and PDF31 test namespace repairs and exact provenance are documented in their cohort reports. DWG native metadata remains pending after a concurrent UI type failure was repaired by UI; fresh retry is active.

## Native DWG Metadata Witness

The fresh canonical controlled-metadata native target completed exit0:7 tests selected,7 passed,90 unrelated skipped; Nextest28ms, whole Nx14m48 including queue/preparation/compilation. The target selects the exact `dwg_controlled_metadata_` prefix; its seven declared laws include `dwg_controlled_metadata_borrowed_manual_carriers_match_the_authored_json`, alongside the existing all-eleven, generic shape, cancellation, interior loops, ceiling and depth laws. The independent source corpus previously passed2/2 with194 assertions. Exact native log:`dwg-controlled-metadata-ui-current.log`. This is native metadata proof, not editor history acceptance.

## Puzzle3d / IFC and Current Ungated Follow-up

The root's current component and hub compiler captures exposed 53 Puzzle3d and 44 IFC diagnostics before assertions. Canonical source ownership repairs and the existing-record metadata completion are detailed in `📓️2026-10-06-puzzle3d-ifc-compile-cohort.md`. Puzzle3d's neutral metadata source gate was actually red (18 pass / 13 fail; new missing borrowed metadata plus twelve malformed fixture-directory URLs) and then green **31/31, 589 assertions, 4.72s** after metadata roles and the directory separator correction. IFC's existing independent SQLite/Ajv suite passed **24/24, 241 assertions, 1.81s**. Their Rust/native editor proof remains pending.

Current 39-crate ungated matrix stopped before all editor laws on TIFF7 and EN1998 22 compiler errors. TIFF canonical source/duplicate mount repairs passed the existing SQLite/Ajv suite **17/17, 128 assertions, 1.065s**. EN1998's new neutral empty-diff role gate actually failed **30 pass / 1 fail / 1540 assertions / 2.70s** before the two missing record-role derives were added. The fresh green capture passed **31/31, 1543 assertions, 1.90s**. The new native empty-delta text/binary roundtrip witness is registered but unexecuted. The 57-crate assembly matrix and both Puzzle transform/camera captures are still compiling: **zero editor acceptance assertions observed**. No new broad native command was launched while Core's retained fold integration is under its explicit source-stability hold.

The current57 assembly cohort ended before assertions on BMP25/GIF15. BMP direct IO registry/payload imports and exact unframed text-paint leaf decoding owners are repaired; current source suite **9/9,186 assertions,613ms**. GIF15 canonical compiler fixes landed before UI received the follow-up; UI now owns its actually failing3/6 source laws and external Presentation codec route. Root owns Semio's remaining TIFF projection/test routes. Details are retained in `📓️2026-10-06-bmp-gif-compile-cohort.md`. Current39+57 native acceptance assertions remain **zero**.

Structured syntax validation accepted973 explicit current Rust owners in Puzzle3d/IFC/EN1998/TIFF/BMP; exact malformed-source negative control failed with the expected unclosed delimiter. No writes or module traversal occur. One existing formatter-only trailing-whitespace issue is separately reported. Earlier failed inline quoting and disabled-format attempts are not used as evidence. Native compilation and editor acceptance remain separate obligations.

The later Scale arity source corpus passed31/31,592 assertions,6.56s, including explicit0/2/4-item counterexamples. Camera62753 ended31m5s/exit1 before assertions on only two Puzzle3d pilot diff grammar issuer paths; both source paths are repaired. Transform99582 remains active. Consolidated current native matrix proof awaits Core's shared fold stable-source boundary, as requested.

## Current Puzzle5d Nullable Boundary

The5d metadata source suite passed34/34,813assertions,8.64s. Completed transform capture99582 still executed zero3d/5d assertions: one concrete3dfill IO import is repaired, while5d9E0277 diagnostics exposed its authored nested optional delta role. See [Typed Nullable Diff Role](📓️2026-10-06-nullable-diff-role.md) for the new34pass/1fail neutral red, exact schema mechanism and pending focused native witness. New broad runs remain held until Core's retained fold stability capture.

## Scoped Owned Output Cleanup

Current tools lane inventory contained384MiB:307MiB in the obsolete private `source-nx` cache,77MiB current/retained logs and small empty Nextest receipt directories; no ticket-owned compiler target remains. `lsof +D` confirmed zero open files in the private cache before it was removed. Current logs, reports, inputs and shared peer Cargo cache were retained. The clean skill was read; its blanket process killing and ticket sweep conflict with the parent's explicitly scoped emergency cleanup request and concurrent-task preservation, so only the authorized owned inactive cache was removed.

Current shared nullable-role native83681 passed1selected/1passed/97excluded, Nextest03619008-c835-4517-b63f-98d7ec54a91e,36ms/Nx8m8; source terminal closure passed35/35,827assertions2.11s. See [Typed Nullable Diff Role](📓️2026-10-06-nullable-diff-role.md). Fresh matrix39+57 and focused Puzzle captures remain active with zero editor assertions observed.

## Resumed Puzzle2d and Exact Editor Ledger

Puzzle2d typed sparse-delta TDD red actually reached the missing metadata after independent authored Ajv admission (2pass/37fail/19expectations/296ms across the source corpus; 36 older failures separately exposed stale physical IO oracle paths). All twelve retained artifact/diff carriers now derive DslRecord, and both pilot diff grammars name their authoritative physical IO owner. The existing SQLite source oracle now imports the physical facet and reads the unchanged authored top-level control and semanticCells fixture fields. Current full source green actually passed **39/39,200expectations,1.88s**. Structured Rustfmt accepted **3 current Rust owners**, exit0. See `📓️2026-10-06-puzzle2d-diff-role.md`; native camera/text/binary/editor acceptance is still pending.

The exact declaration ledger now inventories **148uniqueeditorimplementations/96crates**, with **154macroinvocations**:145direct plus9child. Six editors invoke both forms, accounting for the difference from the earlier148source-file count. Expected generic native assertions are **444**:435direct plus9child; the39ungated editors select115generic laws and the109assembly editors in57crates select329generic laws. `📓️2026-10-06-editor-assertion-ledger.md` retains every named editor/source authority. This inventory is not runtime proof.

Core is currently integrating the genuine per-operation preparation factory, result settlement and the Drawing domain producer. Broad39+57reruns are prepared and remain held until Core announces its producer compile-ready boundary. The concrete missing controlled producers in Puzzle2d/Writer and the generic document wrapper are retained in `📓️2026-10-06-domain-replay-preparation.md`; no synchronous live wrapper has been relabeled as a cooperative replay producer. Current resumed native editor assertions remain zero.

At22:08CEST the resumed exact process inventory had no surviving native-matrix or Puzzle3d/5dtransform owner. `puzzle-camera-fold-stable.log` completed exit1 after14m58s with the previously described eight Puzzle2d compiler diagnostics. `puzzle-transform-fold-stable.log` ends at its build-running770s heartbeat and contains no selected assertion receipt or terminal summary; its owner is no longer active. This interrupted capture is not claimed as a pass or left described as a live test. Fresh native capture will start only at the coordinated compile-ready boundary.
