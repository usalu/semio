# 🧭️ Handover 2026-10-28 — Demonstrator End To End (wave 1028)

Ticket stays OPEN. Rules: `1028-brief.md` (+ addenda 17:15 detached-wait, 18:35 gate is policy, 19:55 free-fix). Timeline + every decision: `1028-coordination.md`. Per-agent state: `1028-result-<name>.md`. Official runner: `1028-tools/act.sh <label> [repo args]` → `🗑️generated/1028/<label>.{log,status}`.

## Goal status
- `activate-prototype` (Bazel): NOT yet re-run since act02 was stopped (18:38) — it can't pass until the wasm closure compiles.
- `serve` 6029 / `test-e2e`: not run (need activation).
- Demonstrator TS: GREEN — `:test`, `:canonical-architecture`, `:test-runtime-catalog` on Bazel, tsc 0 (demots, e2eprep).

## Done today (all live, uncommitted working tree)
- Relocks (lock01, lock02) for peer/demots dependency edits.
- Source-boundary gate: 23 violators → 0 (census `bun 1028-tools/gatecensus/census.ts <roots>` = real gate). Splits: store-engine (4 crates), decl-views (3), plugin-job (+receiving), tool-run (+operations), window-config (+registry, +mutation), instance-app (+initialization, +construction), energy native-mutations (+lifecycle, +singletons), snapshot (+sqlite-admission), 11 energy mut-* crates; ifc definition-only catalog (stdio-ifc light + -catalog, hub stdio-catalog-projection + -definition; gis/vcs on definition roster).
- Graph inference unblocked: lintglob `buildscript` batch landed (from ticket 26/10/18), fem `native_node!` macro-mount moved out of the macro, window-config retained-pack-load fixture restored (deleted by 15:06 commit).
- Native storage: childstore fixed retire recovery (`retireNxGraphNativeChildV1`), dev-number identity binding, absent-namespace settle; open: row `1fbbda3a` creator-claim recovery + regression tests run.
- NativeReceive derive (fwwasm), I: ?Sized design; replication/value/pack/ui fallout fixed.
- Store bound unification: blanket traits `semio_framework_os_spr::{StoreState, StoreMutationFor<P>}` (puzzleplugin, sweep tool `1028-tools/puzzleplugin/sweep.py`); plugin-ownership 737→0, tool-run-operations 253→0.
- Registration gaps of new crates fixed (sourcing), policy-audit fixes (demots, uncompiled).

## Open frontier (owners)
1. Bound propagation above store (ArtifactEditor/Viewer/App assoc types → document-media ~800, decl-surface, command, tool-run, composition-publication) — puzzleplugin ONLY.
2. Flow persisted model → DSL-native (flow; plan `1028-plan-flow-dsl-native.md`); dag same check (flowext).
3. plugin-window-config residue (fem), window-transient E0119 (sweepplugin), store-document Send/Sync (sweepstore), space-member (fwwasm).
4. Then: every component wasm check green → `act.sh act03` → serve 6029 → `test-e2e` (`1028-audit-e2e-cases.md` has the case map).

## Restart recipe
`cd /Users/ueli/Documents/semio && nohup "<C>/1028-tools/act.sh" act03 >/dev/null 2>&1 &` (Bazel activate-prototype). Watch `grep -v '"stage":"running"' <C>/🗑️generated/1028/act03.log`.

## UPDATE 22:55 (CHECKPOINT at weekly 96 %)
- Store layer compiles end to end (engine split + bound unification). Bound unification DONE: `semio_framework_os_spr::{StoreState, StoreMutationFor<P>}` incl. ArtifactEditor/Viewer/App assoc types; sweep tools `1028-tools/puzzleplugin/{sweep,augment}.py` (inline `<P: .., M: ..>` in store/vcs/flow trees not re-swept: grep `ColdMutation<`).
- Green: value/replication/pack/dsl/trace/mesh trees (97 crates, native --lib --tests + wasip2 --lib, sweepvalue); surface + editor wasm (fwwasm); demonstrator TS targets; e2e prep.
- Storage infra fixed (childstore): retire recover, dev-number bind, absent namespace / creator sidecar / retiring row settle, mapping-writer leak-on-throw (4 sites). Orphan Nx daemon 10518 (ours) stopped. depswasm06 queued behind fleet capacity at checkpoint.
- CRITICAL PATH: flow persisted model → DSL-native (`1028-plan-flow-dsl-native.md`, PROGRESS/TODO/DECISION lines; helpers: fwwasm neural DSL step 1 + Dictionary FieldProjectionSource conflict, ifc fixtures/oracle). Until then `artifact-flow-flow-document` (`ProducedMemberSnapshotOpen` removed) blocks every component lane. dag analogue: flowext.
- Other open: reactor-context/reactor-wit-bridge drain loop over `poll_kernel(&mut Option<Event>)` (puzzleplugin), draw-drawing-model 35 files split (sweepvalue), taxonomy audit (childstore → `1028-audit-taxonomy.md`), policyfix verification (demots).
- After compile is green: `act.sh act03` (Bazel activate-prototype) → serve 6029 → test-e2e (sequence in `1028-result-e2eprep.md`).
- e2e path headroom: with this ticket as SEMIO_TICKET_DIR the Playwright output path is 255/256 chars (e2eprep).
- REVIEW: energy adapted typed-operations intent dispatch to the new 6-arg handle_intent_frame with local state (not resumable across polls) — verify against the reactor contract before act03 (C/1028-result-energy.md).
- REVIEW: demobundle deleted the neural engine-model 📤️projection impl (FieldProjectionSource conflict) → Dictionary projection_identity now defaults to object address instead of original_root_identity(); fwwasm worked on the same conflict — reconcile semantics (C/1028-result-demobundle.md, C/1028-result-fwwasm.md).
- REVIEW: reactor-context poll_owned refuses batches >1 event ('a reactor turn admits at most one event', like reactor-wit-bridge) instead of a drain loop — accepted; verify the host/kernel side only ever delivers one event per poll before act03, else runtime refusals in the panes.
- REVIEW overlap: fwwasm wrote NativeReceive/NativeDefault for OrderedSet (value-native-owned 🧺️set) + neural DslField for Atom/Dictionary/Value (📝️dsl); flow wrote DslField/BorrowedDslField for OrderedMap/OrderedSet (dsl-record 🪆️binding/🗂️ordered, mounted by one mod line). Neither compiled at checkpoint (fwwasm-neu1, flow-n3 queued). Check for duplicate impls first on resume.
- depswasm06 (23:32) RED after childstore fixes: 'Exact original descendant creator identity required' — new storage refusal (stack in 🗑️generated/1028/depswasm06.log); likely the creator-sidecar recovery path (childstore follow-up). Investigate first on resume.
- 23:50 fwwasm: flow plan STEP 1 GREEN (native + wasip2: value-native-owned + 4 neural crates). REVIEW: OrderedSet NativeReceive builds the set with unadmitted inserts after the funded array read — make the inserts admitted/accounted (heap-witness laws) before relying on it.
- 23:55 flow: DSL-native MIGRATION DONE (native green: flow-model, flow-document, facade, flow-copy, instance-initialization; twin deleted; SnapshotOpen = OriginalRecordMemberSnapshotOpen<FlowHostSnapshot>). Remaining for flow component wasm: plugin-instance-render (unresolved semio_framework_job → fwwasm) + reactor-context (puzzleplugin). Open: fixtures regen to new grammar + flow unit tests (flow), round-trip oracle (ifc, prepared).
- 00:25 (10/29) childstore-run4 = repo run workspace:deps-wasm GREEN rc 0 (storage infra fixed + residue cleaned; wasm-bindgen-cli 0.2.129 installed). fwwasm: instance-render fixed; next blocker WindowConfigMutation Clone → approved clone_boxed design (fwwasm).
- 00:35 weekly 99 %. reactor-context/wit-bridge GREEN, flow-document GREEN, WindowConfigMutation Clone (clone_boxed) GREEN; window-config/tool-run/instance-render compile on wasip2. SINGLE remaining known blocker for flow component (and 3d puzzle): plugin-instance-dispatch (85: half-finished split, staged modules lack imports + deps; M: ReferenceSpaceMember) — puzzleplugin in progress. Next session: finish instance-dispatch → re-run all component wasm checks → act.sh act03.
- 00:40 flow: fixtures are canonical JSON (no regen needed); dsl-native-snapshot test moved to flow-document, #[ignore] removed. RUNTIME BUG: Dictionary::from_value livelocks (RetainedDictionaryInput capacity quote not funded by step) on any widget with {} params → fwwasm owns. flow-document tests compile; semio_payload_law_flow_mutation passes.

## FINAL 00:50 (10/29) — weekly usage 100 %
- deps-wasm GREEN. Flow DSL-native migration DONE (native). 3d puzzle closure fails ONLY in `plugin-instance-dispatch`: slot→store parent fix applied (unverified); remaining: rebuild `dispatch_typed_command_inner` callers + `admit_command_wire`/`absorb_created_children` callers (host-events L35, typed-command test mount, emit L509) on `OriginalTypedCommandSource<A>` admission (no old definitions exist anywhere — design from the new source's constructor), 5 `ArtifactDiff.payload` PagedBytes sites, `proof` borrow split in producers staging. Details: 1028-result-puzzleplugin.md.
- Runtime bug open: Dictionary::from_value livelock (fwwasm, 1028-result-fwwasm.md).
- Resume order: instance-dispatch → Dictionary livelock → re-run every component wasm check (scripts in 1028-tools/<agent>/) → act.sh act03 → serve 6029 → test-e2e. Review list above (REVIEW lines) before act03.
