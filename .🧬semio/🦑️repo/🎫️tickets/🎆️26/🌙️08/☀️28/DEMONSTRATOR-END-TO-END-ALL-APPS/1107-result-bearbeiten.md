STATUS: UNVERIFIED — blocked on runtime's job-context source-bytes split. W-bearbeiten-102..107 (04:0x-05:0x) all rc 1 pre-compiler: semio-framework-job-context still over maxSourceBytes (crate 267,439 > 262,144; the refusing file changes per run as runtime edits: park, return/place, failure, owner). Lock/sync OK (marker exists). Last test proof: W-pane-process3d-1 (1105b) RED 1/6.

Log: 🗑️generated/1105b/W-bearbeiten-107.txt
Rerun: 1105b-tools/bearbeiten/smoke.sh <n>  (W-bearbeiten-<n>, native pane smoke, en+de)

Static root-cause notes (UNVERIFIED, need a live run):
- history_edits close UnsupportedOwner "time travel close retains an undeclared original session ..." comes from plugin ⏪️time-travel/🧵️instance/📖️ref-reads time_travel_retirement_demands (residue check, runtime has a [DEBUG] residue print there). Runtime-owned.
- "document archive load never reached a terminal state" is the harness guard fixture-dispatch-steps-and-document-loading:320 (G2 family, runtime); runtime edited step-outcome:67/result:77 at 04:30 to name the cause.
- No fault in process3d crates is known.

## Group B port (decode work -> OriginalWork*), 05:3x
EDITED-unverified (nothing compiled; bearbeiten-110 energy snapshot check refused upstream: semio-framework-job-native-decode "Unbalanced Rust source delimiter", group A):
- fem 2d+3d snapshot/native.rs: unused AuthoringWork removed.
- energy snapshot original bridge: StepWork implements OriginalWorkRecipient (Context=()), call site OriginalWorkAuthority::with_loan + with_work_recipient.
- ui presentation receive (test), store space-history validation + codec workspace-receiving (tests): same pattern.
- store composition open receiving: MemberWorkProducer/MemberWorkContext/MemberWork (authority replaces work_issuer in MemberHistoryAllocation).
- NOT done: plugin (document pub use NativeDecodeWorkRecipient, archive-load close_step param, maintenance-ladder StepWork). They pass the recipient into spr history close_step/step, so they follow spr's new signature (group A, koordinator).
Rerun check: 1105b-tools/chk.sh bearbeiten-111 <energy Cargo.toml> native -p semio-s-artifact-energy-model-snapshot

## Group B plugin port, 06:0x
EDITED-unverified against koordinator's final spr signature (step/close_step take &mut OriginalWorkLoan): plugin document-archive-load (ArchiveWorkTurn trait + ArchiveWorkProducer + `work` authority field, close_step(grant,&dyn ArchiveWorkTurn)), document (re-exports), maintenance-ladder (StepTurn implements ArchiveWorkTurn; with_loan around decoder.step and close_step), plugin test app-child-member-registry.
Fixed my paren slip in store validation + workspace-receiving (unbal.ts: all 7 edited files "unbalanced 0").
Compile: bearbeiten-112 (root Cargo.toml, -p semio-plugin-document-media) rc 1, errors only in value crates (equality, paged-text, retained-clone, retirement-queue) = other lanes. Rerun: 1105b-tools/chk.sh bearbeiten-113 Cargo.toml native -p semio-plugin-document-media

## Projection trait port (value-130), 06:1x
EDITED-unverified, unbal 0: energy ImmutableRootProjection impls (EntityId, ScheduleId, EnergyModelSnapshot) take `effect`, add root_projection_work_demand (1/1/11), alias via effect.reborrow + try_duplicate(&mut child) with summed work.
OPEN: energy snapshot tests (original/tests:12 cursor.take(g); state/tests + helpers), store paged-one-item immutable-root test, replication diff sequence test: need EffectBank.run + RetainedWorkPoll matching.

## Construction API + projection tests, 06:3x
Discovery: with_construction_recipient now takes an OriginalConstructionLoan too (NativeDecodeConstructionRecipient gone). Ported (EDITED-unverified, unbal 0): energy bridge, fem 2d+3d native.rs, ui receive test, store validation + workspace-receiving tests, energy state-tests helpers (OriginalWork/OriginalConstruction, advance_original) and 5 cursor.take sites via new helpers bank()/run_take (EffectBank.run); original test asserts turn.work == take_work_demand (>=11).
Dev-dep added to energy snapshot Cargo.toml: LOCK NEEDED (claims).
OPEN: store tests still call native.close_retirement_recipient(grant) (now takes effect; need EffectBank dev-dep in store); store paged-one-item immutable-root test; replication diff sequence test; energy tests inverse.take/set.take untouched (different API, unverified).

## bearbeiten-113 (06:4x)
rc 1, `🗑️generated/1105b/bearbeiten-113.txt`: compile stops upstream of the plugin: semio-framework-value (198/278 errors: RetainedWorkPoll .result/.work, RetireOwned retirement/RetirementCursor), value-edit (140), value-native-receiving-original (22), job-receiving (7). None in my crates, none reached. Plugin/store/energy/fem/ui port stays UNVERIFIED. Smoke blocked behind these.
Rerun: 1105b-tools/chk.sh bearbeiten-114 Cargo.toml native -p semio-plugin-document-media ; then 1105b-tools/bearbeiten/smoke.sh <n>.

## bearbeiten-114
rc 1 (`🗑️generated/1105b/bearbeiten-114.txt`): value lib passes now; remaining reds are other lanes' crates below the plugin: async, deflate, io-control (78), io-sqlite-model (36), machine, pack-json-canonical (35), replication-retained-field (56) / -key, tool-run-entities (24), value-edit (45). My crates not reached. Smoke not run.

## Assigned compile reds (07:5x)
- machine: FIXED, bearbeiten-122 rc 0 (RetireOwned: derive on Status/Snapshot, artifact_retire_leaf!(NodeId); old retirement() impls removed).
- deflate: green by now without my edit (someone moved encode cursor to effect), bearbeiten-125 shows no deflate error.
- replication-retained-key: FIXED, bearbeiten-127 (close_step and advance take &mut RetainedAllocationEffect, return OriginalRetirementPoll / u128 work). Its plugin callers (generation3d, procedural flow-source/host-scratch, layout) must thread an effect: their lanes.
- replication-retained-field (56 errors): NOT fixed. It sits on the retained-clone cursor trait migration (advance_demands/advance_work_demand take source ref; take(output,effect,reply); equality.advance(.., effect), decide(.., actual) arity, u128 work) owned by hostcheck/ledger/value; blind edits would collide. Needs those APIs final first. Log: 🗑️generated/1105b/bearbeiten-127.txt.
My plugin/store/energy/fem/ui port still UNVERIFIED (document-media chain blocked by these crates, io-control, io-sqlite-model, async, pack-json-canonical, value-edit, tool-run-entities).

## process3d own modules (testpath audit), 08:xx
- editor/📬️store-preparation: both ArtifactStoreOneItemPreparation impls ported to the current trait (advance returns OriginalReceivingPrimary through advance_inner, the four receiving-recovery items added as no-pending, close_step returns OriginalRetirementFailure). EDITED-unverified, unbal 0.
- host/🧰️owned snapshot-decode + mutation-decode (+ decode-owner-catalog): BLOCKED on the store trait. They implement store's owned-schema `close_step(grant)` (field-decoder-registry-and-empty-array-authority:104, still grant-based) but call `artifact_retirement_box_close_step(&mut retirement, effect)` which now needs an effect and returns a poll (value 📸️snapshot/👁️read:795). Store lane must move that trait to effect/poll first; then I change my three impls (:182, :536) the same way.
- host retained-cursors, host owned root, io binary mutations: no old-API use found.

## Store owned-schema close_step trait to effect/poll: scoped, NOT started (08:xx)
Survey (no edits): the trait ArtifactEnvelopeFieldDecoder::close_step(grant)->Result<Step,OwnedSchemaDecodeDiagnostic> has ~50 implementors in 10 plugins (writer 3, generation2d 5, generation3d 3, gis 2, animate 3, process3d 3, jack 3, draw 2, raster 2, plus store core/tests ~25) and sits under the store decode ladder, which threads explicit grants everywhere (decode-page-state close_granted :216-340 with `child` grants; fresh-vcs-active-and-verifier :30-145; decode-rejections :80-204; owner-bundle :211-238; fresh-record-target :390-426; owned-spr edit/mutation :273-305). Moving only the leaf to an effect is impossible (the leaf calls artifact_retirement_box_close_step(slot, effect)->Poll, value 📸️snapshot/👁️read:795); a bridge through AuthoringWallet would drop work from the real StepContext (breaks work conservation).
Sound shape: trait close_step(&mut self, effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll-like Poll<Step,OwnedSchemaDecodeDiagnostic>; `grant` = effect.grant(); children via effect.reborrow(child_grant, effect.maximum_work_units()); the single entry `ArtifactEnvelopeDecodeAuthority::close_step(cx)` (decode-page-state:487) wraps the ladder in cx.with_original_effect(grant, |cx, effect| ..) or job close_original_effect_step(cx, demand, ..). Needs one owner for ~15 store files first, then plugins in parallel. I stopped before editing: a half-migrated trait breaks every plugin's compile.

## process3d decoders on the store effect/poll trait (09:5x)
EDITED-unverified, unbal 0: host/🧰️owned snapshot-decode, mutation-decode (close_step(effect)->ArtifactEnvelopeClosePoll via the 09:35 recipe: grant=effect.grant(), admit_owned_retirement(&mut value, &mut effect.nested(child)), artifact_envelope_close_resume_with(box_close_step(.., effect), failure->diagnostic), artifact_envelope_close_settle) and decode-owner-catalog (leaf). All process3d own modules named by the testpath audit are now ported (store-preparation, decoders).
Compile bearbeiten-130 (process3d host): upstream reds in artifact-reference, job-failure (22), pack-record-preflight, pack-record-retained (21), replication-mutation (22), ui-viewport. None mine. Smoke still blocked.
