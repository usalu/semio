# Public Puzzle IO Worker Acceptance Investigation

## Current Source Findings

No new production changes or compiler request were made for this investigation. Root request 76254 retains the native lane. These are source findings, not runtime results.

Puzzle2d subset `any/🦀️.rs:55–66` constructs its canonical IoDeclaration with `entries: &[]`. The artifact root explicitly describes this tree as its only schema/IO/viewer/editor registration channel. Its older definition advertises SVG/PDF/PNG/JSON/DWG/DXF composer capabilities, and the leaf IO module still contains real ComposerEntry implementations; those declarations alone do not install a new IoEntry.

The current tree lifting owner (`plugin/extension/lifted/tree`, lines 86–120) collects only each subset.io.entries and passes those batches to io_register. Native snapshot capabilities are registered separately. Consequently the empty subset cannot itself make drawing exports discoverable by the generic semio.io-run job; any other actual registration must be identified before claiming availability.

The reactor built-in bounded-job table has five kinds (IO run, IO sniff, infer, mutation plan, migrate). A scoped scan of all Rust files in the actual OS plugin module found semio.compose only in the host invocation, router/documentation and a test comment; it found no registration. This is not yet a whole-repository negative proof. The host compose helper explicitly documents job.unknown-kind until a separate registration is supplied.

Actual semio.io-run execution at reactor/jobs:671–694 parses the request under its actual IoRunControl recipient, resolves an exact local io_entries pair, and executes io_run_with_snapshot_control with genuine controls. It refuses a missing pair. The TwoPhaseBoundedJob offers a decoded phase and then invokes this conversion inside one execution step. Leaf progress and worker isolation do not establish caller-observable mid-conversion progress/cancellation.

The real PluginInstanceHandle runs a GuestColdRelayJob on the shared worker pool with actual Original retained input, wake authority and one relay/close opportunity per host poll. Public io_run creates an internal root cancellation token and returns only final bytes; it has no public progress callback or caller token argument. Cancellation by dropping its future and exact registry/guest terminal cleanup remain to be established through existing owner tests.

## Acceptance Design Decision Still Required

First identify the actual registration path and genuine artifact guest loading fixture. A material native host acceptance should instantiate the actual Puzzle component, resolve the advertised format through the public router, validate completed bytes with an independent reader, observe genuine running work, cancel through the actual supported caller path, prove terminal ownership cleanup, and dispatch a subsequent real interaction. Existing Mock/Recording guest tests or synchronous format leaf tests cannot supply this proof. No test-only globals, fabricated controls, or synthetic success backend will be introduced.

Parent investigation found no IO hook in time-travel. UI exportSnapshot is a separate native board DTO download and cannot substitute for the advertised format route. The host acceptance owner is therefore preferred if its actual fixture supports the required real component and witnesses; otherwise an explicit runtime interface gap must be repaired before adding acceptance.

## Completed Registration and Host Fixture Scan

A pruned source sweep of all Rust files under hub, extensions and framework found no semio.compose registration. The only real extension registrations found are FEM2d, FEM3d and Draw geometry custom job kinds. Builder registration uses its actual declared factory rows; no compose factory was found. Puzzle itself has no io_register or bounded-job-kind registration call. The old composer host helper therefore has no identified executable conversion body in current source; restoring a second legacy composer channel is not proposed.

Host tests `owned-instance-open` already compile and instantiate real components with both OwnedRuntime and WasmtimeRuntime. Their staged required component is the neutral counter component, not Puzzle. Their codec sweep can inspect staged real packages, but does not exercise Puzzle advertised format jobs. The public handle IO test scripts Running and Done on MockGuestRuntime. It qualifies dispatch mechanics only. The lifecycle cancellation suite also uses controlled mock guests; it cannot stand in for a material export.

A real Running progress payload is preserved as a cold-relay Preview publication. The mounted pump classifies PreviewReady and CheckpointReady as Continues, consumes their pages, acknowledges/resumes the session and eventually exposes only terminal output to its public future. There is no caller progress observer in PluginInstanceHandle.io_run. Dropping an incomplete GuestRelayMountedFuture invokes registry.detach; genuine detached cleanup machinery exists. Actual material conversion cancellation must still demonstrate that its guest execution can yield and reach that cancellation, rather than cancelling before conversion begins.

## Proposed Coherent Acceptance Owner and Repair Order

Use a normal integration target in the actual Puzzle hub/component acceptance owner, with its declared produced actor component as a prerequisite. Instantiate that component with the real host runtime and real Original fixture authority, then obtain its actual descriptor/router registration. Keep an independent schema-first corpus declaring a deterministic large board, output kind, cancellation point, expected material counts and terminal/application-state invariants. SVG is a suitable first drawing route because an independent XML parser can verify full node/edge material coverage without relying on the implementation serializer; ZIP is an analogous separate 5d proof with the existing third-party archive reader.

First assert route discovery for the real Puzzle dialect to the advertised SVG dialect. Current canonical entries:&[] gives this a concrete expected-red frontier. Repair should publish actual IoEntry routes from the same canonical subset declaration, using existing owned format conversion interfaces; it should not revive semio.compose. Before claiming large-operation success, expose caller-observable progress/cancellation through the existing domain-neutral worker operation interface and ensure actual conversion yields bounded opportunities. The existing one-execute-step IO wrapper is insufficient evidence of that property. Preserve Original receipts, paid close and existing worker isolation.

The acceptance must finish one real material export and validate it with the independent reader, then cancel a second export after observed conversion work (not merely after request decoding), drain genuine guest/relay ownership, and execute a real app command/undo with unchanged document identity. It must assert that the cancelled operation publishes no final download/result. A worker-only synthetic counter, mock job script or leaf SQL progress callback is excluded. No production change is made pending the combined parent decision on this public interface boundary.

## Existing Captured Serializer Interface

The current domain-neutral interface already exists at OS IO `serialization/owned`: IoEntry optionally declares OwnedSerializerFactory, admitted by io_begin_owned_serialization. OwnedSerializerRequest carries actual ErasedSnapshotRead source, exact source/target route, generation, revision, options and captured child authority. Admission checks the registered route, unchanged source/child commit authority and physical birth demand. OwnedSerializerJob extends the existing InteractiveJob protocol and exposes typed output/fault handoffs and terminal frame release demand. This is the preferred seam for actual bounded conversion acceptance; do not add an ad hoc public host callback without first integrating this existing owner.

A scoped scan of all Puzzle Rust source found no OWNED_FACTORY, io_begin_owned_serialization or OwnedSerializerJob implementation. Its current format leaf implementations therefore do not demonstrate this retained conversion path. A broader producer scan is pending. The registry also synthesizes native-to-SQLite and reverse routes from native snapshot registration, so the empty subset entries observation is specifically about advertised drawing/foreign formats, not absence of all native IO.

## Canonical Route Publication Staged

A schema-first neutral full inventory fixture and independent Serde roster law are staged under the existing Hub surface owner. Existing seven Hub laws remain; new law makes eight. All three canonical subset declarations now reference their typed route module. Typed Serializer/Deserializer bindings use actual existing format leaf functions and native entry constructors; no ComposerEntry adapter or second registry. Exact advertised direction totals are 2d nine, 3d four and 5d seven. Native SQLite synthesized routes stay separate.

Source review found a genuine prerequisite: the three snapshot types currently lack ArtifactNativeSnapshot, which the actual standard constructors require. Their existing DslSchema derive already emits controlled metadata, record projection and record binding methods. An explicit canonical trait implementation must retain exact native envelope and intermediate record custody through the original NativeDecodeControl/NativeEncodeControl. This prerequisite is under active coordination with Root; no bound weakening, uncontrolled decode fallback or claimed successful compilation.

Existing reusable owned factories belong to Draw drawing, not Stdio Semio drawing. SVG/PNG/PDF factory jobs use genuine DocumentVectorJob/DocumentRasterJob, scene paint, bounded writers and paid close. Puzzle2d projects to a different Stdio Semio drawing model; that source authority cannot be fabricated as a Draw SnapshotRead. Root owns this bounded format/projection design. Current publication bindings qualify only ordinary format routes once compiled; they do not provide expensive operation progress/cancellation by themselves.

The actual Source reader after staging reports 5d mutation owner production 184259 bytes /25 files /3269 active lines, tests 228918 /30 /3896; physical probes equal admitted sets. Caps remain unchanged. Runtime compilation and material roundtrip acceptance remain pending.

## Full Physical Specimen Route Assertions Staged

The existing 2d, 3d and 5d declared-JSON physical laws now additionally execute the canonical registered entry functions over the complete authored snapshots and every binary64 word. Two-dimensional baseline and 43 numeric roles remain; three-dimensional complete word specimens remain; five-dimensional all-word None/Some branches and seven optional pointer assertions remain. No leaf assertion was removed. Route export uses a genuine original native encode then original caller decode authority; import bytes come from independent Serde encoding, then the original import entry and controlled native decode. A test guard explicitly retires the restored snapshot and the complete relational projection compares all entities and exact numeric words. These are native route acceptance assertions, not evidence of expensive worker conversion progress/cancellation.

The three owning snapshot binary facets now explicitly implement ArtifactNativeSnapshot by delegating to the already-existing store native record helpers, matching Note/Draw. No shared helper or trait signature was changed. Existing helper postconstruction cancellation custody is an acceptance frontier; no runtime panic or illegal drop has been established from source alone.

Hub now retains all seven original laws plus the independent full roster and a seven-pair guest-genesis dispatch smoke (nine total). The native ArtifactCodec uses its current hash array and explicit reference().print_mirror capability; the distinct guest ArtifactCodecTableV1 retains its actual function fields. No reactor API compatibility change was needed or made.

Final physical same-reader/probe counts: 2d 75571 bytes /3 inputs /888 active lines; 3d snapshot-sqlite 32242 /2 /381; 5d 95158 /5 /1244. Hub production 6904 /1 /104 and tests 18186 /3 /236. All probes exactly match admitted input sets. The new physical-source-census.ts input is retained. No native or producer was launched during Root request 76254.


## Independent finite Source and semantic carrier review

The current three canonical route roots publish UTF8 JSON/TXT as IoPayload::Text, additionally 2d DXF/SVG as Text; 2d DWG/PDF/PNG and 5d PNG/ZIP remain Binary. String::from_utf8 consumes the existing Vec backing and explicitly rejects invalid UTF8; no new copy/conversion library or compatibility wrapper was introduced. All original real leaves and initial IoRunControl checkpoint remain. Imports accept borrowed bytes from either carrier, with existing leaf validation unchanged. The 13 advertised exports divide into 8 text and 5 binary outputs. This is source verification, not execution.

The actor fixture now explicitly authorizes one finite lifetime Source distinct from unchanged mounted per-turn preparation/maintenance/close grants. Formula rechecked: 16MiB VM + 2*(2*2MiB requests + 2*2MiB results + 2*4MiB native) = 48MiB copy; 2*16MiB VM + 2*(2*2MiB requests + 3*2MiB results + 2*4MiB native) + 2MiB scaffolding = 70MiB capacity/release. Items 2000*32=64000; depth4096. Actual code admits this stock once, uses the real monotonic 120s deadline and 2000 user-visible fuel quanta, retains 10000 total turn limit, and does not renew the Source. Explicit bounds check actual VM and output sizes; they are declared allowances rather than measured sufficiency. No global cap/profile increase. Actual completion/cancellation/material oracle execution remains mandatory.

Fresh guest-io Source capture currently 35053 bytes / 5 inputs / 312 active lines, physical probes exact; unchanged admission caps. Cancellation-fault strengthening remains pending in the current driver observed for this review, so the prior drain false-positive finding is not marked repaired yet.


## Current post-carrier actual Source probes

Re-ran the existing canonical Source reader census inputs, without executor/compiler: physical2d 75607 bytes/3 inputs/888 lines; physical3d 32278/2/381; physical5d 95194/5/1244. Hub lib-test current16930/3/214; production4245/1/74. Every physical probe total equals admitted inputs, all below unchanged 262144/32/4000. These supersede prior physical pre-carrier census numbers, not actual native proof. Existing Hub test count verified separately against current source.


## Current Stronger Actor Evidence and Portable Status

The current genuine phased OriginalCore instance has a known source contradiction, not merely an unexecuted happy path: phased startup hands off original_core=Some/actor=None, while current legacy context, poll and job crossings go through operation begin/resume paths that reject original_core. The existing original scalar helper is not a general interpreter/host-call engine and cannot be cited as proof those public actor paths are implemented. No actual actor run has qualified a repaired general path. The native borrowed JSON owned-export proposal complements actor acceptance; it does not replace actual model publication, public guest IO, strict cancellation, same-actor post-cancel dispatch or UI/browser gates.

Cumulative VM execution stock and the operationwide protocol-turn ceiling remain open source-demonstrated gaps. Actual job receipt instructions are recorded without a separate lifetime debit; context APIs discard backend VM measurement; startup advance returns an explicit receipt which this test currently ignores. Current test startup count is carried into Self from epoch, but startup loops lack a shared predispatch bound and direct instance close does not debit/check the global counter. Preserve the authored2000 execution quanta/120s/10000 total turns; no increased limit or semantic/VM unit conflation.

Production qualification: actual OS Run wasmtime-node-host.rs245/251 already checks out and receives phased startup execution receipts before finish_actor_step. Do not characterize the actor-specific execution ports as globally test-only. The missing stock transport is specifically current context/shard/public crossing settlement and the acceptance ledger.

Parent-authoritative portable result: cold-launch Vitest actual31/31 passed, terminal log independently read by Root. This proves its current portable controls, not real fresh actor/browser startup. Older browser-readiness request19193 refused before tests at an operand guard; it is not a test pass. No fresh managed WASM publication, native physical IO, actor end-to-end, browser20 or full en/de app journey completion is claimed.
