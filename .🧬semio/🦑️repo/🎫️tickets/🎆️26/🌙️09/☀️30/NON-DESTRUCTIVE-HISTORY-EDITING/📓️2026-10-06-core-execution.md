# Core Execution — 2026-10-06

## Initial Scope

Read the root and s AGENTS.md, ticket design §22–23, latest status, shared time-travel reducer/schema/lifecycle oracle, and store identity definitions.

Concrete gap: §22.33 requires unchanged Accept to be refused. Both TypeScript and Rust reducers currently accept an unchanged pending input by treating it as resume/discard; the lifecycle matrix and XState oracle still encode that behavior. Proposed narrow ownership: schema-first lifecycle fixture change, explicit refusal/helper in both implementations, EN/DE refusal labels, and conformance tests using the existing Ajv/XState/fast-check oracle. No identity constructor refactor without coordinator approval, since §22.34–35 spans store and every app construction site.

Store §22.34–35 remains unimplemented at the inspected definitions: local_actor is Option<String>; persisted genesis identity remains a separately retained initial_digest. These decisions need one coordinated wave over store and plugin construction routes.

## Implementation and TDD

The first existing `bun nx run @semio-tech/framework-time-travel-rs:test` run exercised the changed fixture against the old implementation: 11 passed, 13 failed. Failures included reducer/XState disagreement on unchanged Accept, validating the intended behavioral gap.

Implemented `timeTravel.unchanged`, EN/DE `refusalUnchanged`, and accept-refusal query helpers in both TypeScript and Rust. The generation check still precedes the unchanged refusal. Accept compares against the active accepted draft, else original input, so returning an accepted draft to its original remains a valid change. Discard retains its prior behavior. The lifecycle matrix now refuses all three unchanged cases and the existing case corpus preserves changed input/revert/replay/finalize scenarios. The independent XState model agrees on all matrix rows and randomized event sequences (20 non-schema tests passed on the intermediate conformance run).

Repaired baseline schema-test omissions found by real runs: `LifecycleLawFixture` was missing; the external replay-report schema was not registered with Ajv; its reference pointed at nonexistent `definitions/Report` rather than the report root. The fixture schema explicitly models all existing sections and forbids unknown properties.

Added an app runtime route law and fixture refusal entry: unchanged `historyEditAccept` must retain session, input editor, painted body and committed store generation; Discard then leaves the session. Runs are pending under `🗑️generated/core-execution`.

## Guest Input Editor

The native plugin guest editor still emitted Accept as always enabled. Changed it to use the existing `editor.changed` query; a disabled row describes the shared EN/DE refusal reason. Discard stays enabled. The runtime law checks both locales' disabled control and reason, then changes the input and reverts it to verify Accept follows the actual draft value.


## Bounded History Preparation, Opening and Cancellation

The active native history input editor now starts a Store-owned preview cursor. Each slice resolves one target/draft identity through maintained mutation positions and exact generation-keyed ledger reads, then folds one effective prefix operation per deadline check. It excludes the selected operation and every downstream operation. Cached prefixes remain the fast path; their admission uses the Store frontier instead of rehashing the live stream. The old eager `state_before` convenience remains for cold/testing callers; the active plugin preview does not call it.

Report replay has a preparation cursor too. It copies one applied edit identity/count or one effective supersession per slice, then transfers the paged order/count catalogs to the replay without recounting the suffix. The replay uses generation-keyed applied reads, including jumps to cached prefix starts. Prefix folding now checks the deadline after every operation in a transaction. Finalize undo/redo authoring uses this same bounded derived report route.

Opening one mutation uses `applied_mutation` and no longer constructs all mutation rows. The maintained operation index validates the live applied edit and exact operation identity; Undo refuses the removed target and Redo restores its position. Foreign-unit facts are captured against each original operation's historical prestate during initialization, publication and replay, so opening does not refold the prefix merely to discover foreign steps. Changed replay suffixes update those facts. Batched and staged group publication retain the same facts; committed group readers use the staged read root before adoption.

Cancellation transfers preview/planner/replay ownership immediately to the existing retirement queue. One fixed metadata page, one metadata entry, or one registered child retirement advances per close step. Nested strings, payload bytes, messages, inverse mutations and projection aliases retire through their exact owners. The cancelled cursor cannot publish; stale frontiers refuse further stepping. Closing or refusing an authoring replay transfers its owners too. `retire_derived_history_preview` and `retire_derived_report_replay` require `P: Sync` locally, as the existing snapshot alias retirement does; the general Store bound remains `Send`.

The neutral bounded-history fixture now covers accepted upstream input, an intra-transaction selected target, excluded downstream input, complete report replay, cancellation stops at steps 1/4/8/20, per-step ownership grants, terminal cancellation and stale-generation refusal. Ajv validates the fixture; existing fast-json-patch independently produces its prefix, selected preview, replayed head and cancelled unchanged head. Native tests require at most one deadline work unit per call and at most one item/byte released per cancellation retirement call. The existing group visibility corpus now covers exact keyed readers before/after the shared decision and after abort.

Canonical EN/DE preparation progress labels are in both shared implementations/schema/lifecycle fixture and the native progress band. The UI execution agent owns their React dictionary/band consequences.

## Verified Runs and Current Limits

- Original unchanged Accept red: 11 passed, 13 failed against the edited law and old reducers.
- `NX_DAEMON=false bun nx run @semio-tech/framework-time-travel-rs:test --skip-nx-cache`: shared TypeScript 24/0 and native nextest 17/0; target completed successfully. This target did not use `--excludeTaskDependencies`.
- Fresh shared TypeScript conformance including preparation labels: `NX_DAEMON=false bun nx exec --projects=@semio-tech/framework --excludeTaskDependencies --skip-nx-cache -- bun test <absolute conformance source>`: 24 passed, 0 failed, 3413 expectations (`conformance-preparation-final.log`). The preceding run found fixture label order drift; repaired fixture order to match both implementations.
- Fresh Store neutral/oracle including cancellation law: same Nx exec form over the Store supersede-replay TypeScript test: 2 passed, 0 failed, 31 expectations (`bounded-oracle-cancel.log`).
- Initial native bounded law reached compilation and failed before assertions: six stale tests directly assumed `replay.state`, four staged read-root privacy errors, and twelve concurrent BorrowedDslField omissions. Our ten errors are repaired; tools execution owns the twelve borrowed-field fixes. No native bounded pass is claimed from this run.
- Fresh native bounded/cancellation and plugin unchanged input route runs are in flight: `bun nx run @semio-tech/framework-os-kernel:test --excludeTaskDependencies --skip-nx-cache -- bounded_history_read_cursors_obey_the_neutral_law` and the coordinator's `bun nx run @semio-tech/framework-plugin:test --excludeTaskDependencies --skip-nx-cache -- time_travel_tests::accepting_an_unchanged_input`. The kernel target forwards its filter to `cargo test --lib`; it must not receive the unrelated `quick` prefix. Logs are `bounded-native-cancel.log` and `runtime-unchanged-final.log` under ticket generated/core-execution. No passing/runtime claim before results.

## Mandatory Actor and Stored Genesis Audit

Design §22.34 and §22.35 remain unimplemented in current source. This is a source audit, not a runtime verdict; a coordinated construction wave is still required before blanket ticket completion.

§22.34 requires `ArtifactStore::new(envelope, actor: ActorId)` and every retained initializer to take a required actor; Store must retain `ActorId`, with no optional actor, guessed actor on reload, refusal path or local literal. Current `ArtifactStore::new` still takes only the envelope (Store source around18000); `ArtifactStoreInitializationRuntime` takes artifact id/schema/current/initial digest with no actor (around15827); both retain `Option<String>` actor. Cold construction seeds that actor from the last applied edit (around18033). `transition_actor` falls back to the literal `local` (around20750), and another mutation-author fallback does likewise (around22635). Plugin routes bind admitted actors on dispatch/open, but that convention does not enforce construction and bare codec/test routes. The latest decision explicitly supersedes the earlier runtime-refusal proposal, so a refusal-only fix would not satisfy it. Existing authored-only durable-history validation and exact actor route tests need reassessment during the wave.

§22.35 requires the envelope to retain immutable genesis pack bytes beside the decoded snapshot; identity must hash those stored bytes, and persistence/backbone must emit them verbatim. Current `ArtifactVcs` retains only decoded `initial_snapshot: P` (VCS source around1447), and `ArtifactEnvelopeOwners` adds no stored genesis pack. `artifact_initial_digest` hashes `initial.encode_pack()` (Store around15541); cold construction and state replacement call that helper (around18016/18296). `print_document_pack` re-encodes genesis (around13904), backbone Genesis re-encodes it (around22804), while `verify_genesis` compares incoming stored bytes against that re-encoded digest (around23485). Thus differing encoders can still assign different identities to one loaded document. Retained initializers also accept a separately supplied digest rather than retaining original genesis bytes. A digest alone or canonical re-encode is explicitly rejected by the recorded decision. The required two-encoder/stored-byte law has not been added or run in this execution wave.

Parent coordination requested this audit after immediate bounded native/live validation and directed that widespread constructor work not overlap the current compiling cohort. No actor/genesis source was changed here.


## Validation Coordination and Changed Files

The duplicate unchanged native wrapper was cancelled at the coordinator's request after `lsof` proved its stdout belonged to our `runtime-unchanged-final.log`; exact owned pids38102/37921/37920/37918 were terminated. The coordinator's unchanged native wrapper55106 and `2026-10-06-coord/plugin-unchanged-native-schemas-fixed.log` were preserved as the sole proof run. Kernel bounded/cancellation wrapper19433 continues. Shared native time-travel wrapper34151 was started to validate the newly added preparation labels after the earlier shared native run.

Our changed source/fixture/test paths (peers have independent edits in several of these same files):

- `🧰️framework/🔨️modules/⏪️time-travel/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/⏪️time-travel/🧫️fixtures/🧫️lifecycle-law/🔣️.json`
- `🧰️framework/🔨️modules/⏪️time-travel/🟦️.ts`
- `🧰️framework/🔨️modules/⏪️time-travel/🦀️.rs`
- `🧰️framework/🔨️modules/⏪️time-travel/🧪️tests/🧪️conformance/🟦️.ts`
- `🧰️framework/🔨️modules/⏪️time-travel/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🧪️tests/🔬️group-history-visibility/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧫️supersede-replay/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (only retained initializer exact key arguments)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧫️time-travel/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` (retained initializer exact keys)
- This Markdown report.


## Fresh Native Results

`bounded-native-cancel.log`: kernel bounded history cursor/cancellation law passed **1/0** (1197 filtered out), assertions0.04s, native test-profile compile6m46, Nx target14m37. This actual run includes cancellation of prepared prefixes, active replay and a completed review result. No Store state changed during cancelled work.

`shared-preparation-native.log`: the shared time-travel target completed TypeScript **24/0** and native nextest **17/0** with preparation labels. Native compile7m08; assertions0.198s.

A focused native cohort is now running under existing `@semio-tech/framework-os-kernel:test-native`, quick level, `--excludeTaskDependencies --skip-nx-cache`, selecting the bounded law, all four supersede-law rows, generation-keyed group visibility and the Store-derived capability law. It uses the repository nextest filter grammar and `--nocapture`; metadata lives under ticket generated/core-execution/native-cohort. Its command/log is `history-core-cohort.log` (wrapper30465); no pass is claimed before completion. This cohort is necessary because cached unit facts and exact generation keys affect those existing laws beyond the single bounded law.

The coordinator's first unchanged native run55106 passed all14 source oracles but compilation stopped at a stale test-kit protocol path. The coordinator repaired that facade path and owns its replacement native33143; the actual unchanged runtime result is still pending.


### MP4 borrowed diff compile repair and construction wave boundary

The native matrix independently demonstrated `E0277` for `IndexedDiff<Mp4Sample, Mp4SampleDiff>` and `IndexedDiff<Mp4Track, Mp4TrackDiff>` before assertions. The MP4 declared diff owner already uses explicit concrete `Mp4SamplesDiffRecord` and `Mp4TracksDiffRecord` owners for its owned DSL field role. Two additive borrowed field implementations now delegate to those same generated record owners, preserving identical field identities and avoiding duplicate generic metadata. Tools owns the containing native matrix rerun; this repair is not yet claimed compiled.

Root authorized the next actor/stored-genesis construction wave, after the focused native eight-law cohort is captured. At this boundary no actor/genesis representation source has changed. The cohort remains in the existing Cargo build route.

The clean stored-genesis design must distinguish snapshot birth (encode once) from stored pack ingress (decode supplied bytes and keep those exact bytes). The VCS lower layer currently owns an eager decoded `P` while the Store layer owns the `ArtifactPack` trait, whose unrelated native registration and SQLite methods refer back into Store. An immutable decoded `Arc<P>` with immutable shared raw pack bytes therefore needs a codebase-owned lower-layer genesis encoding/decoding capability, separate from those Store registration methods. Identity, backbone publication, and document pack persistence must consume the retained bytes; active preview must alias the decoded genesis in constant work. Current JSON fresh envelope decoding and retained initializer routes also require explicit treatment, not merely changing the cold `ArtifactStore::new` constructor.


### Captured native cohort and required actor API wave

Fresh `@semio-tech/framework-os-kernel:test-native` focused cohort: **8 passed, 0 failed, 1286 skipped**, assertion time 0.262 seconds, Nx duration 16 minutes 30 seconds, `--excludeTaskDependencies --skip-nx-cache`; native artifacts retained under generated/core-execution/native-cohort/semio-nextest-Z9dJKq. The filter includes bounded history reads and cancellation, supersede laws with indexed unit facts, staged history key visibility, and exact Store frontier capability fences. The actor/genesis representation wave starts only after this capture.

Actor signatures: `ArtifactStore::new(envelope, actor: ActorId)`; initialization runtime `new(artifact_id, schema, current, initial_digest, actor: ActorId)` and `new_with_owner_catalog(..., initial_digest, actor: ActorId, catalog)`; `local_actor_id() -> &ActorId`; Store `set_local_actor_id(actor: ActorId) -> Result<(), VcsError>` and runtime setter takes `ActorId`. Runtime current will become a shared `Arc<P>` when immutable genesis ownership lands. Root owns plugin/app/space/MCP callers outside Store/VCS; UI owns renderer/dev/browser/native host callers; Tools keeps matrix/stdio compile carriers. All wrapper `VcsArtifactApp` constructors must likewise require actors and preserve opened instance actors, with explicit LOCAL only for standalone birth/test contexts.

Schema-first neutral actor/genesis law now lives at Store `🧬️schema/🔣️actor-genesis-law/🔣️.json`, with shared corpus `🧫️fixtures/🧫️actor-genesis/🔣️.json`. It pins named opened/explicit actors and two distinct valid JSON pack byte sequences decoding to the same value, plus birth, ingress, identity, verbatim persistence and constant-work immutable decoded alias laws. Implementation and independent hash/pack oracle follow this contract.

### Actor/genesis construction wave: current ownership boundary

The pre-wave focused native cohort captured 8 passing laws and zero failures. The subsequent architecture wave is incomplete and has no native green claim. `ArtifactGenesis<P>` now owns private immutable `Arc<P>` and exact `Arc<Vec<u8>>`, with a cached canonical digest. Birth encodes once; stored ingress decodes supplied bytes once without encoding. `initialPack` is the canonical VCS field. Preview/report constructors use `share_snapshot()` in constant time. Common fresh native VCS admission captures and hashes lowercase Pack hex under fuel, deadline, operation, generation and cancellation checks, including single-fuel steps. Exact snapshot and raw-byte retirement owners close separately. Member opening captures the original framed Pack incrementally before history admission and passes its cached digest into retained hydration.

InitializationRuntime now takes `Arc<P>`; `current_mut()` only uses `Arc::get_mut`. Domain bounded copy hands an owned workspace to `adopt_current_owned` with its exact retirement factory, then pumps `settle_current_retirement_step` before adoption. Empty history retains the genesis alias. Historical author attribution cannot rebind the opened actor. Stored config hydration receives `ArtifactGenesis<P>` as its first argument; document hydration receives decoded projection plus exact Pack bytes and digest. External factory and domain call sites belong to Root/UI/Tools as coordinated.

The independent actor/genesis oracle passed 3 tests and 44 assertions. Native `immutable_stored_genesis_obeys_the_neutral_law` now asserts birth encode count, zero reload encoding, supplied-byte persistence, cached canonical identity, distinct identities for equal decoded content with different bytes, and snapshot/pack alias identity. Invocation is pending in `🗑️generated/core-execution/genesis-native-current.log`; no assertion execution yet.

### Current actor/genesis native capture and dependency audit

The repaired immutable-genesis native law ran through the existing `@semio-tech/framework-os-kernel:test` Nx target with `--excludeTaskDependencies --skip-nx-cache`: 1 passed, 0 failed, 1201 filtered. A subsequent fresh `test-native quick` nextest cohort covered immutable stored genesis, opened actor authority, bounded history cursors and captured group envelope reads: 4 passed, 0 failed, 1294 skipped, assertions 0.248 seconds, total Nx 3m34s. Its current source includes immutable actor construction, no actor setter APIs, named fixture births and transaction abort preserving the constructor actor. Console evidence is in `🗑️generated/core-execution/genesis-current-cohort.log`. The independent actor/genesis oracle remains 3 tests/44 assertions. This capture establishes the core laws; plugin/member runtime production proof belongs to the parallel caller validation wave and is not claimed here.

The Nx cycle `value-rs -> value-derive-rs -> value-rs` has no reverse runtime edge: Value's Cargo normal dependency is Value Derive, while Value Derive's Value dependency is in `[dev-dependencies]` for existing test/oracle integration. Its runtime dependencies are `syn`, `quote` and `proc-macro2`. Genesis introduced no Cargo, package or project manifest dependency changes. Handling that specific cycle as a fixture graph cycle is therefore supported by the manifest audit; this is not a claim that arbitrary cycles can be ignored.

### Additional verified scope and pending native admission law

The fresh independent oracle now compares streamed noble BLAKE3 chunks at grants 1, 2 and 7 with the complete canonical digest, using the same neutral Pack rows. It passed 3 tests and 76 assertions (`genesis-incremental-oracle.log`). The native admission law exercises one-byte fuel, exact retained bytes, generation refusal, cancellation refusal, incremental digest equivalence and exact byte retirement; its invocation remains in progress (`genesis-capture-native.log`), so no assertion outcome is claimed yet. Config hydration now derives identity from `initial.digest()` and does not accept a second potentially divergent digest. `DocumentStoreOwners::retire_genesis_owned` exposes exact registered disposal of both immutable owners.

Current proof limits: the native empty-history alias law covers the main ArtifactStore; retained config hydration still receives caller-supplied validation/current workspaces and is not a zero-copy empty-config proof. Those caller copies are controlled by the window loader. Existing cold backbone announcement copies the exact stored Pack, and incoming genesis verification hashes its supplied bytes in the existing synchronous network mechanism. No statement here claims those cold/network helpers became stepped jobs. Neither helper re-encodes genesis, and active BindGenesis/preview/report use cached digest and immutable aliases.

### Current Runtime Undo Failures and Additional Ingress Proof

The root fresh 76-law plugin cohort ran23 assertions:20 passed and3 failed before failfast. Two owned failures were the alternative scoped undo and counted-clock interior undo. The long-history fixture opened the neutral fixture actor, explicitly published an `author` edit, then expected that foreign edit to be undoable by changing action metadata; §22.34 now preserves the actual opened actor. The fixture now opens as the intended undo author, explicitly publishes neutral seed/downstream authors, and asserts the exact opened authority before burying its own edit. The same correction covers its existing cancel/adoption companion law. Alternative undo/redo now wait for the existing bounded supersede-authoring job before asserting the original restoration, scoped transition, active alternative and redo result. Pending console evidence is included; fresh runtime result remains required.

The extra bounded Pack capture wrapper reached kernel test compilation but no assertions: its new hash assertion required an unavailable Debug trait, and new retained-hydrator test generic inference was ambiguous. Both test-only compile defects are repaired. The retained actorless ingress test is authored ahead of guards and its red wrapper is still pending. Retained doc/config currently bypass the strict cold-store actor validation, so the next exact repair rejects absent/empty edit authors and operation authors through their registered cleanup lanes.

The neutral actor/genesis law now records seven concrete retained-admission inputs: missing, empty, and whitespace-only edit/operation authors are refused; explicit nonlocal edit and operation authors are admitted without changing the opened actor. The schema declares the authoritative HistoryAuthors record; the existing JSON Patch oracle constructs each input and independent Ajv decides the same acceptance. The native law exercises each input through both retained document and config statecharts. This expands the actorless law from a boolean expectation to concrete language-independent input/output pairs.

Remaining exact interaction limit: both retained document/config Begin phases call HistoryFile.fold() over the complete decoded history synchronously. This existing protocol fold has not been converted into a job cursor by the genesis wave. The main preview/report planning and Pack capture are sliced; their verified result cannot be generalized to that admission fold. No additional fold construction wave is started during the active native closure cohort.

Fresh independent neutral oracle after concrete admission cases:3passed/0failed/83assertions. The first oracle attempt applied the entire root law schema as a $ref sibling and independently rejected even the positive projected authors record; repaired reference compilation retains only schema version and definitions at the reference root. The green run checks all7 authors cases, existing full-history/preview JSON Patch laws and independent streaming Pack identity hash. Exact command is unchanged existing Nx exec target with fixture-only cycle flag; logs are actorless-admission-oracle.log (red) and actorless-admission-oracle-green.log.

Actual native TDD red captured: retained_hydrators_refuse_actorless_history_from_the_neutral_law compiled and ran1assertion cohort, then failed on config=false/editAuthor=null/operationAuthor=actor:alice: accepted=true versus expected=false. Runtime0.02s; Nx11m6s including coupled artifact waiting. The doc/config statecharts now refuse absent or whitespace-only edit and operation authors before copying/adopting them, and reject empty opened authority at Begin. Config owned source edit transfers into its registered active retirement before rejecting; metadata remains in pending owner. Document sources remain in retained history for exact cleanup. Cold durable history author checks now use the same whitespace rule. Signatures stable; next fresh native selection will include Pack fuel/fences and original opened actor cancellation/admission.


### Retained Admission Fold and Transition Decoder Wave

The schema-first `semio.history.bounded-fold/v1` law now owns work grants 1/2/7, byte grants 1/7/4096, cancellation positions 0/1/2/7/31, and valid/invalid UTF-8 checkout wire examples. The lower retained fold uses an explicitly polled pinned coroutine, indexed event/owner/redo/checkpoint facts, and tracked local owners transferred to exact registered retirement on cancellation. Transition decoding yields across repeated members and byte pages, including split UTF-8 scalars, all seven transition variants, and malformed/trailing-byte refusal. Immutable native `HistoryLog` normalization captures an Arc in constant work, resolves quarantined operation identities without copying opaque operation payloads, and returns normalized transition owners with the derived fold.

Actual TDD red: the fresh replication native invocation `bounded_history_fold_obeys_the_neutral_law` ended with undeclared `HistoryFoldJob`, `HistoryFoldJobStep`, and `fold_history_for_controlled` before module registration. The subsequent cursor implementation/registration is awaiting native validation. The independent Ajv/platform UTF-8/JSON Patch oracle was run through the existing Nx executable route: 4 passed, 0 failed, 107 assertions. Native success is not yet claimed.

Document and config retained admission now have separate Fold and BindGenesis phases, retaining immutable history aliases until the fold job is terminal. Cursor IDs transfer one at a time; normalized transitions and effective supersessions transfer by ownership. Document change and alternative member vectors move without whole-vector clones; checkpoint authors transfer one per turn. Cancellation closes the fold before releasing its source history, then retires each owned result and existing hydration owners. Progress has a monotone high-water witness across existing phase-local counters. Remaining compile/native verification is required before this wave is considered complete.

The actorless diagnostic native run executed all seven document admission cases correctly, including exact rejection cleanup and successful authored input. The first config actorless case failed its finite exact-cleanup assertion. Investigation found `ArtifactGenesisRetirement` waiting for the still-live source store's shared immutable Pack alias. It now uses the existing `shared_lease_retirement` authority: release this lease immediately, and transfer the final Pack payload to bounded retirement. This is a production cleanup defect rather than an assertion relaxation. Fresh native validation remains required.

The public full `materialize_document_snapshot` helper is distinct from retained admission: it clones the initial value and synchronously folds in its cold convenience path. Verified production callers are native-host `materialize_backbone_snapshot` and `SpaceMember::pack_at_checkpoint` (including its Space child routing). No direct call exists in plugin time-travel editing, Accept, or finalize sources. This call-graph finding does not establish that the helper caused the observed browser freeze. Document retained replay still uses the existing `loaded_history_replay` preparation/adoption helpers; its separate order/supersession cloning and replay settlement scans are identified for precise follow-up rather than claimed fixed by the fold wave.


### Frozen Retained Fold, Decoder, Replay Preparation and Adoption Boundary

The actual kernel retained normalization law executed and passed 1 test, 0 failures (1204 filtered; assertions 0.01 seconds, Nx 29m18s). This is a compile/runtime receipt for the admitted retained fold; the source was being extended during its queued/compile window, so a fresh exact cohort is required for the final frozen boundary. The current independent neutral law passed 4 tests, 0 failures, 139 assertions, including quarantined envelope UTF-8, exact opaque payload bytes, flags and trailing-byte refusal (`bounded-fold-quarantine-oracle.log`). The strict Rustfmt parser accepted all 8 explicit owned source files (`bounded-fold-all-owners-parse.log`); this checks syntax, not Rust typing or runtime behavior.

The frozen final source moves a cooperatively copied replay order and the effective supersession map into the counted replay constructor; it uses an admitted edit-position index for every replay and adoption lookup. Replay completion caches the worst severity while processing each operation. Inverse adoption and message-ledger adoption transfer one edit row per turn and retire displaced owners through the exact mutation/message catalog. Superseded target reconstruction uses the bounded transition decoder and the admitted mutation-position index, one transition/input per turn. Checkpoint duplicate pins use a per-checkpoint index rather than scanning all previously admitted pins. Quarantined envelopes and conflict fields normalize through the same byte-granted coroutine, preserving exact opaque payloads. A cancellation closes active decoder/fold owners before releasing their immutable history aliases, then closes each typed replay/result owner through the shared read-retirement authority.

Current production signatures are frozen. The exact focused kernel cohort includes retained normalization, 14 original document/config actor admission cases, and bounded Pack capture/fences. Lower `bounded_history` selects the fold, transition decoder and quarantine envelope decoder laws. Both commands use existing Nx test targets, `--excludeTaskDependencies` and `--skip-nx-cache`; fresh results remain pending. No additional production expansion is underway during this capture unless a concrete failure requires repair.

Precise remaining limits are distinct from this admission wave: cold full `materialize_document_snapshot` remains synchronous, and domain mutation `inverse`/`diff` application still follows its existing per-operation API. The existing `EditReplay::close_edit` message clamping can scan one edit's full accumulated messages/outcomes; the controlled admission preparation/adoption changes do not establish that this separate replay application work became bounded. Existing cold fold/decoder helpers also still use their original implementations pending the bounded semantic-law proof; no compatibility API was added.

The frozen lower `bounded_history` native selection is now actually green: 3 tests passed, 0 failed, 277 skipped, assertions 0.049 seconds and Nx 7m45s (`bounded-fold-decoder-current.log`). It includes the newly authored quarantined envelope law. A repeated capture requests successful console output because the first Nextest summary suppressed the successful debug logs; no source change is being made for that capture. The fresh kernel 3-law closure cohort remains pending. The current lower/native/independent-oracle commands are registered adjacent to the ticket editor gates in launch.json.

The first frozen kernel cohort invocation did not execute assertions: kernel `test` routes directly to Cargo Test rather than the Nextest shared native route and rejected `-E`. The corrected existing target passes the three law names after the Cargo `--` to the Rust test harness with `--nocapture`. Its registered launch configuration was corrected in place; concurrent unrelated launch writes twice removed our earlier entries, so the three exact commands were restored by a narrow insertion. This command correction changes no production source.

Launch correction: the canonical registry is generated from `.vscode/🧩️launch.seed.jsonc`. The three exact lower, kernel and oracle configurations are now authored in that seed adjacent to existing ticket gates at orders 900.0580941/2/3. The earlier disappearance was legitimate generation from the seed, not arbitrary peer deletion. UI owns publishing/verification through its generator.

The canonical oracle launcher now uses the parent-verified explicit workspace source project without `--nxIgnoreCycles`. Earlier successful source captures used the verified fixture-only cycle exception; current authority no longer needs that flag. Production source remains frozen.


### Exact Frozen Boundary Runtime Receipt

The corrected kernel harness invocation actually passed **3 tests, 0 failed, 1202 filtered**, assertions **0.12 seconds**, Nx **22m25s** (`retained-fold-closure-console.log`). Its console logs confirm every one of the 14 original retained document/config ingress cases: six missing/empty/whitespace author variants are rejected, and the explicit foreign edit/operation authors case is admitted, separately through both hydrators. Every case reaches finite exact cleanup, including the previously failing config Pack lease. The same native cohort confirms bounded Pack capture with one-byte fuel, incremental identity hashing, stale/cancel refusal and exact retirement; native normalization includes indexed replay preparation, duplicate pins, bounded conflict/envelope fields and cancellation source preservation.

The successful-console lower capture also passed **3 tests, 0 failed, 277 skipped**, assertions **0.030 seconds**, Nx **20m41s** (`bounded-fold-decoder-console.log`). All three debug witnesses are captured: bounded semantic fold, all transition variants/UTF-8/trailing refusal, and exact quarantined opaque payload decoding/cancellation. These console-backed native receipts are current source proof, not cached Nx results. The canonical workspace oracle passed **4/4 and 139 assertions** without graph-cycle bypass (`bounded-fold-workspace-oracle.log`).

The parent/UI/Tools were notified immediately that this production boundary is stable and the broader editor/browser/native hold can be lifted. Root now owns bounded close-edit message clamping/settlement; Core continues cold fold/decoder consolidation and per-operation capability auditing. Canonical launch seed rows were published twice and independently checked by UI with zero drift.

### Shared Cold Fold and Decoder Consolidation

The canonical cold transition decoder and protocol fold now delegate to the same controlled semantic cursors, driven to completion explicitly. Native HistoryLog.fold delegates to its retained normalizer and exactly retires unused normalized owners. Full lower Replication regression actually executed276 passing tests, zero failures, four skipped (2.431s assertions; Nx8m29s), in generated/core-execution/shared-fold-cold-all-laws.log. Native HistoryLog independent fold expectations are pending in shared-fold-native-history-laws.log. Per-operation inverse/diff/apply and checkpoint export limits are recorded separately in 📓️2026-10-06-per-operation-replay-audit.md; these changes do not claim those seams are bounded.
