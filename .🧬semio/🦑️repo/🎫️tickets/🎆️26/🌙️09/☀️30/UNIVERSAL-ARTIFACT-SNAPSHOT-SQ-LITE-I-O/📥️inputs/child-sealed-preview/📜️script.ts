import { resolve } from "node:path";

const root = process.cwd();
const ticket = resolve(import.meta.dir, "../..");
const phase = process.argv[2] ?? "stage";
if (phase === "mount-demand" || phase === "mount-producer") {
  const input = await Bun.file(resolve(ticket, "📥️inputs/global-child-sealed-store-preview-producers-only-held-pairs.json")).json();
  const pairs = input.pairs.filter((pair: { phase: string }) => pair.phase === phase);
  const updates = [];
  for (const pair of pairs) {
    const file = Bun.file(resolve(root, pair.path));
    const before = await file.exists() ? await file.text() : "";
    if (before !== pair.before) throw Error("Fresh exact guard: " + pair.path);
    updates.push(pair);
  }
  for (const pair of updates) await Bun.write(resolve(root, pair.path), pair.after);
  for (const pair of updates) if (await Bun.file(resolve(root, pair.path)).text() !== pair.after) throw Error("Exact after readback: " + pair.path);
  await Bun.write(resolve(ticket, "🗑️generated/child-sealed-preview-" + phase + ".json"), JSON.stringify({ phase, state: "ExactProductionMountReadbackNoRuntime", paths: updates.map((pair: { path: string }) => pair.path) }, null, 2) + "\n");
  console.log(JSON.stringify({ phase, exactReadback: updates.length }));
  process.exit(0);
}
if (phase !== "stage") throw Error("Expected stage, mount-demand, or mount-producer");
const capsule = await Bun.file(resolve(ticket, "📥️inputs/global-child-current-immutable-frame-readonly-genuine-fixtures-kernel-issuer-held-pairs.json")).json();
const storePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs";
const toolPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs";
const timePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs";
type Region = { name?: string; source?: string; count?: number; before: string; after: string };
function apply(source: string, regions: Region[]): string {
  for (const region of regions) {
    if (source.split(region.before).length !== (region.count ?? 1) + 1) throw Error("Guard: " + (region.name ?? region.source));
    source = source.replaceAll(region.before, region.after);
  }
  return source;
}
async function target(path: string) {
  const pair = capsule.pairs.find((candidate: { path: string }) => candidate.path === path);
  let source = await Bun.file(resolve(root, path)).text();
  source = pair.regions ? apply(source, pair.regions) : pair.after;
  return {
    pair,
    add(name: string, before: string, after: string) {
      if (source.split(before).length !== 2) throw Error(name);
      (pair.regions ??= []).push({ name, before, after });
      source = source.replace(before, after);
    },
    between(name: string, start: string, end: string, after: string) {
      const first = source.indexOf(start);
      const last = source.indexOf(end, first + start.length);
      if (first < 0 || last < 0) throw Error(name);
      this.add(name, source.slice(first, last), after);
    },
  };
}
const store = await target(storePath);
const anchor = "//#region 🧬️OpaqueSnapshotRead\n";
store.add("sealed-store-derived-snapshot-and-replay-types", anchor, anchor + `
/// 🌱️ Immutable projection derived by one Store from its own event history or a provisional operation.
pub struct ArtifactDerivedSnapshot<P> {
    owner: Arc<P>,
    registry: Arc<SnapshotReadLeaseRegistry>,
    generation: u64,
    revision: [u8; 32],
}

impl<P> ArtifactDerivedSnapshot<P> {
    /// 👁️ Borrows the complete typed projection without transferring its derivation authority.
    pub fn snapshot(&self) -> &P {
        self.owner.as_ref()
    }

    /// 🪞️ Borrows the immutable alias used by renderer seams.
    pub fn snapshot_owner(&self) -> &Arc<P> {
        &self.owner
    }

    /// ♻️ Transfers the projection alias to the caller's registered bounded retirement.
    pub fn into_snapshot_owner(self) -> Arc<P> {
        self.owner
    }
}

/// ⏪️ Store-owned history replay whose input edits and projection origin cannot be supplied by the caller.
pub struct ArtifactDerivedReplay<P, Mu: self::Mutation<P>> {
    replay: EditReplay<P, Mu>,
    registry: Arc<SnapshotReadLeaseRegistry>,
    generation: u64,
    revision: [u8; 32],
}
`);
store.between("replace-arbitrary-alias-derived-read-with-sealed-producer", "    /// 🪞️ An erased read of `alias`", "    /// 🧵️ Immutable O(1) snapshot capability", `    /// 🌱️ Captures the current Store projection as the base of a provisional member fold.
    pub fn derived_snapshot_head(&self) -> ArtifactDerivedSnapshot<P> {
        self.seal_derived_snapshot(self.snapshot_owner())
    }

    fn seal_derived_snapshot(&self, owner: Arc<P>) -> ArtifactDerivedSnapshot<P> {
        ArtifactDerivedSnapshot { owner, registry: Arc::clone(&self.snapshot_read_leases), generation: self.generation(), revision: self.content_revision() }
    }

    fn admit_derived_source(&self, registry: &Arc<SnapshotReadLeaseRegistry>, generation: u64, revision: [u8; 32]) -> Result<(), VcsError> {
        if !Arc::ptr_eq(registry, &self.snapshot_read_leases) || generation != self.generation() || revision != self.content_revision() {
            return Err(VcsError::ValidationFailed("derived snapshot belongs to another Store or event frontier".into()));
        }
        Ok(())
    }

    /// 🪞️ Issues a member read only from this Store's genuine derived projection capability.
    pub fn snapshot_read_derived(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<ErasedSnapshotRead, VcsError>
    where
        P: Sync,
    {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        if !self.snapshot_read_leases.publish_authority(source.generation, source.revision) {
            return Err(VcsError::ValidationFailed("snapshot read commit authority is busy or exhausted".into()));
        }
        let alias = Arc::clone(&source.owner);
        let lease = self.snapshot_read_leases.try_issue(Arc::clone(&alias)).map_err(|_| VcsError::ValidationFailed("snapshot read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(ErasedSnapshotRead::new(alias, lease))
    }

    /// 🧩️ Derives a provisional member projection through this Store's own operation vocabulary and merge policy.
    pub fn derive_provisional_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<Option<ArtifactDerivedSnapshot<P>>, VcsError> {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        let outcome = <Mutation as self::Mutation<P>>::diff(op, source.snapshot());
        let applicable = outcome.is_applicable(protocol::MergePolicy::default());
        let (diff, _) = outcome.into_parts();
        let next = applicable.then(|| <<Mutation as self::Mutation<P>>::Diff as self::MutationDiff<P>>::apply(&diff, source.snapshot()).ok()).flatten();
        <<Mutation as self::Mutation<P>>::Diff as self::MutationDiff<P>>::retire_cold(diff);
        Ok(next.map(|owner| self.seal_derived_snapshot(Arc::new(owner))))
    }

    /// 🎞️ Derives a history-edit preview base from this Store's actual history and accepted draft inputs.
    pub fn derived_snapshot_before(&mut self, target: &MutationId, drafts: &BTreeMap<MutationId, protocol::InputReplacement>) -> Result<ArtifactDerivedSnapshot<P>, VcsError> {
        let owner = self.state_before(target, drafts)?;
        Ok(self.seal_derived_snapshot(owner))
    }

    /// ✏️ Derives one history preview input with the replay's keep-and-record application rule.
    pub fn derive_history_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<(Option<ArtifactDerivedSnapshot<P>>, Vec<protocol::MutationMessage>), VcsError> {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        let (diff, mut messages) = <Mutation as self::Mutation<P>>::diff(op, source.snapshot()).into_parts();
        let applied = <<Mutation as self::Mutation<P>>::Diff as self::MutationDiff<P>>::apply(&diff, source.snapshot());
        <<Mutation as self::Mutation<P>>::Diff as self::MutationDiff<P>>::retire_cold(diff);
        let next = match applied {
            Ok(owner) => Some(self.seal_derived_snapshot(Arc::new(owner))),
            Err(error) => {
                messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));
                None
            }
        };
        Ok((next, messages))
    }

    /// ⏪️ Starts a sealed replay of this Store's genuine event history.
    pub fn begin_derived_report_replay(&self, drafts: &BTreeMap<MutationId, protocol::InputReplacement>, from: Option<&MutationId>) -> Result<ArtifactDerivedReplay<P, Mutation>, VcsError> {
        Ok(ArtifactDerivedReplay { replay: self.begin_report_replay(drafts, from)?, registry: Arc::clone(&self.snapshot_read_leases), generation: self.generation(), revision: self.content_revision() })
    }

    /// ⏭️ Steps a sealed history replay against this Store's actual edit ledger.
    pub fn step_derived_report_replay(&self, replay: &mut ArtifactDerivedReplay<P, Mutation>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        self.admit_derived_source(&replay.registry, replay.generation, replay.revision)?;
        replay.replay.step(self.replay_edits(), deadline)
    }

    /// 🏁️ Returns a finished replay and its genuine complete projection for preview and subsequent publication.
    pub fn finish_derived_report_replay(&self, owner: &mut Option<ArtifactDerivedReplay<P, Mutation>>) -> Result<(EditReplayResult<P, Mutation>, Option<ArtifactDerivedSnapshot<P>>), VcsError> {
        let replay = owner.as_ref().ok_or_else(|| VcsError::ValidationFailed("derived replay owner is absent".into()))?;
        self.admit_derived_source(&replay.registry, replay.generation, replay.revision)?;
        if !replay.replay.is_finished() {
            return Err(VcsError::ValidationFailed("derived replay has not finished".into()));
        }
        let replay = owner.take().expect("admitted finished derived replay owner is present");
        let result = replay.replay.finish().expect("admitted finished replay completes without another fallible operation");
        let head = result.state().map(|owner| self.seal_derived_snapshot(Arc::clone(owner)));
        Ok((result, head))
    }

`);
const tool = await target(toolPath);
tool.add("member-run-retains-sealed-derived-bases", "    base: Option<Arc<P>>,\n    overlay: Option<Arc<P>>,\n    displaced: Vec<Arc<P>>,", "    base: Option<store::ArtifactDerivedSnapshot<P>>,\n    overlay: Option<store::ArtifactDerivedSnapshot<P>>,\n    displaced: Vec<Arc<P>>,");
tool.add("member-run-real-store-head-producer", "ToolRunMemberState::<P> { base: Some(store.snapshot_owner()), overlay: None, displaced: Vec::new() }", "ToolRunMemberState::<P> { base: Some(store.derived_snapshot_head()), overlay: None, displaced: Vec::new() }");
tool.add("member-run-rebase-releases-only-derived-aliases", "            let previous = state.base.replace(store.snapshot_owner());\n            state.displaced.extend(previous);\n            state.displaced.extend(state.overlay.take());", "            let previous = state.base.replace(store.derived_snapshot_head());\n            state.displaced.extend(previous.map(store::ArtifactDerivedSnapshot::into_snapshot_owner));\n            state.displaced.extend(state.overlay.take().map(store::ArtifactDerivedSnapshot::into_snapshot_owner));");
tool.add("member-provisional-fold-uses-sealed-store-derivation", `            let outcome = <Mu as ::protocol::Mutation<P>>::diff(&op, source);
            let next = outcome.is_applicable(protocol::MergePolicy::default()).then(|| <<Mu as ::protocol::Mutation<P>>::Diff as ::protocol::MutationDiff<P>>::apply(outcome.diff(), source).ok()).flatten();
            <Mu as ::protocol::Mutation<P>>::retire_cold(op);
            match next {
                Some(next) => state.displaced.extend(state.overlay.replace(Arc::new(next))),
                None => conflicts = conflicts.saturating_add(1),
            }`, `            let next = store.derive_provisional_snapshot(source, &op);
            <Mu as ::protocol::Mutation<P>>::retire_cold(op);
            match next.map_err(|error| error.into_fault())? {
                Some(next) => state.displaced.extend(state.overlay.replace(next).map(store::ArtifactDerivedSnapshot::into_snapshot_owner)),
                None => conflicts = conflicts.saturating_add(1),
            }`);
tool.add("member-overlay-read-requires-source-capability", "store.snapshot_read_erased_of(Arc::clone(overlay))", "store.snapshot_read_derived(overlay)");
tool.add("member-overlay-bounded-alias-retirement", "None if self.everything => state.overlay.take().or_else(|| state.base.take()),", "None if self.everything => state.overlay.take().or_else(|| state.base.take()).map(store::ArtifactDerivedSnapshot::into_snapshot_owner),");
const history = await target(timePath);
history.add("history-preview-owners-retain-sealed-source", `    preview: Option<Arc<P>>,
    replay: Option<store::EditReplay<P, Mu>>,
    finished: Option<store::EditReplayResult<P, Mu>>,
    head: Option<Arc<P>>,`, `    preview: Option<store::ArtifactDerivedSnapshot<P>>,
    replay: Option<store::ArtifactDerivedReplay<P, Mu>>,
    finished: Option<store::EditReplayResult<P, Mu>>,
    head: Option<store::ArtifactDerivedSnapshot<P>>,`);
history.add("history-render-borrows-sealed-complete-parent", "self.document.shown(stage).unwrap_or(committed)", "self.document.shown(stage).map(store::ArtifactDerivedSnapshot::snapshot_owner).unwrap_or(committed)");
history.add("history-shown-borrows-sealed-source", "pub(crate) fn shown(&self, stage: TimeTravelStage) -> Option<&Arc<P>>", "pub(crate) fn shown(&self, stage: TimeTravelStage) -> Option<&store::ArtifactDerivedSnapshot<P>>");
history.add("history-retire-transfers-derived-owner", "fn retire(&mut self, store: &ArtifactStore<P, Mu>, snapshot: Option<Arc<P>>)", "fn retire(&mut self, store: &ArtifactStore<P, Mu>, snapshot: Option<store::ArtifactDerivedSnapshot<P>>)");
history.add("history-retire-exact-derived-alias", "store.retire_snapshot_alias(snapshot)", "store.retire_snapshot_alias(snapshot.into_snapshot_owner())");
history.add("history-query-borrows-complete-derived-payload", "owners.shown(stage)).map(|shown| semio_framework_value::ToValue::to_value(shown.as_ref()))", "owners.shown(stage)).map(|shown| semio_framework_value::ToValue::to_value(shown.snapshot()))");
history.add("history-start-replay-sealed-origin", "store.begin_report_replay(&drafts, Some(&from))", "store.begin_derived_report_replay(&drafts, Some(&from))");
history.add("history-read-only-sealed-derived-capability", "store.snapshot_read_erased_of(shown.clone())", "store.snapshot_read_derived(shown)");
history.add("history-preview-base-from-genuine-history", "store.state_before(target, drafts)", "store.derived_snapshot_before(target, drafts)");
history.add("history-preview-fold-store-vocabulary", `                let (diff, mut messages) = op.diff(&base).into_parts();
                let applied = diff.apply(&base);
                <<Mu as ::protocol::Mutation<P>>::Diff as MutationDiff<P>>::retire_cold(diff);
                self.discard(Some(op));
                match applied {
                    Ok(next) => {
                        self.retire(store, Some(base))?;
                        (Arc::new(next), messages)
                    }
                    Err(error) => {
                        messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));
                        (base, messages)
                    }
                }`, `                let derived = store.derive_history_snapshot(&base, &op);
                self.discard(Some(op));
                let (next, messages) = derived.map_err(|error| error.into_fault())?;
                match next {
                    Some(next) => {
                        self.retire(store, Some(base))?;
                        (next, messages)
                    }
                    None => (base, messages),
                }`);
history.add("history-replay-step-owning-ledger-only", "Ok(match replay.step(store.replay_edits(), &mut deadline)", "Ok(match store.step_derived_report_replay(replay, &mut deadline)");
history.add("history-finish-sealed-source-and-report", `                match replay.finish().and_then(|result| store.replay_report(&result).map(|report| (result, report))) {
                    Ok((result, report)) => {
                        let head = result.state().cloned();`, `                match store.finish_derived_report_replay(replay).and_then(|(result, head)| store.replay_report(&result).map(|report| (result, head, report))) {
                    Ok((result, head, report)) => {`);
history.add("history-finish-admits-borrowed-owner-before-transfer", `                let replay = self.replay.take().expect("a finished replay was held");
                match store.finish_derived_report_replay(replay)`, `                match store.finish_derived_report_replay(&mut self.replay)`);
const unitPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs";
const unit = await target(unitPath);
const lawAnchor = "#[semio_framework_async_macros::async_test]\nasync fn apply_undo_redo_transfers_each_snapshot_root_to_one_exact_retirement_owner()";
unit.add("genuine-derived-source-other-store-stale-and-complete-actual-replay-law", lawAnchor, `#[semio_framework_async_macros::async_test]
async fn derived_child_reads_require_the_exact_store_frontier_and_genuine_operation_or_history_fold() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪆️derived-read/🔣️.json")).expect("closed language-neutral origin corpus");
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "derived-source", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners());
    let mut foreign = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "derived-foreign", DemoSnapshot { n: Some(0) }, None)).await;
    foreign.install_document_store_owners_exact(demo_closable_store_owners());
    let base = store.derived_snapshot_head();
    assert!(Arc::ptr_eq(base.snapshot_owner(), &store.snapshot_owner()));
    let operation = DemoMutation::SetN(SetN { n: 9 });
    let derived = store.derive_provisional_snapshot(&base, &operation).expect("genuine source").expect("applicable operation");
    <DemoMutation as Mutation<DemoSnapshot>>::retire_cold(operation);
    assert_eq!(derived.snapshot().n, Some(9));
    assert_eq!(store.snapshot_ref().n, Some(0));
    assert!(foreign.snapshot_read_derived(&derived).is_err(), "same concrete type and event numbers do not grant another Store authority");
    let read = store.snapshot_read_derived(&derived).expect("exact Store source");
    let actual = read.get::<DemoSnapshot>().expect("complete concrete derived projection");
    assert!(std::ptr::eq(actual, derived.snapshot()));
    assert_eq!(actual.n, Some(9));
    let portable: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(derived.snapshot())).expect("own portable JSON output");
    assert_eq!(portable, fixture["derived"]);
    assert_eq!(portable, serde_json::to_value(derived.snapshot()).expect("independent complete Serde output"));
    drop(read);
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 6 })], description: None, transaction: None }).await.expect("actual event publication");
    assert!(store.snapshot_read_derived(&derived).is_err(), "a prior derived source cannot be relabelled with the new frontier");
    drive_retirement_terminal(store.retire_snapshot_alias(derived.into_snapshot_owner()).expect("derived alias retirement"));
    drive_retirement_terminal(store.retire_snapshot_alias(base.into_snapshot_owner()).expect("base alias retirement"));
    let mut replay = Some(store.begin_derived_report_replay(&BTreeMap::new(), None).expect("genuine history replay"));
    let owner = replay.as_ref().expect("retained replay") as *const _;
    let projection = Arc::as_ptr(replay.as_ref().unwrap().replay.state.as_ref().unwrap());
    let registry = Arc::as_ptr(&replay.as_ref().unwrap().registry);
    assert!(store.finish_derived_report_replay(&mut replay).is_err(), "unfinished admission retains its actual owner");
    assert_eq!(owner, replay.as_ref().expect("unfinished replay retained") as *const _);
    assert_eq!(projection, Arc::as_ptr(replay.as_ref().unwrap().replay.state.as_ref().unwrap()));
    assert_eq!(registry, Arc::as_ptr(&replay.as_ref().unwrap().registry));
    assert!(matches!(store.step_derived_report_replay(replay.as_mut().expect("retained replay"), &mut || false).expect("own actual ledger"), ReplayStep::Finished(_)));
    assert!(foreign.finish_derived_report_replay(&mut replay).is_err(), "foreign finishing cannot consume the source owner");
    assert_eq!(owner, replay.as_ref().expect("foreign-refused replay retained") as *const _);
    assert_eq!(registry, Arc::as_ptr(&replay.as_ref().unwrap().registry));
    assert_eq!(replay.as_ref().unwrap().replay.state.as_ref().unwrap().n, Some(6));
    let (result, head) = store.finish_derived_report_replay(&mut replay).expect("sealed completed replay");
    assert!(replay.is_none());
    let head = head.expect("complete actual history projection");
    assert_eq!(head.snapshot().n, Some(6));
    let portable: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(head.snapshot())).expect("own portable history JSON output");
    assert_eq!(portable, fixture["published"]);
    assert_eq!(portable, serde_json::to_value(head.snapshot()).expect("independent complete history Serde output"));
    assert!(Arc::ptr_eq(head.snapshot_owner(), result.state().expect("same replay result")));
    assert!(foreign.snapshot_read_derived(&head).is_err());
    drop(result);
    drive_retirement_terminal(store.retire_snapshot_alias(head.into_snapshot_owner()).expect("replay alias retirement"));
    let mut stale = Some(store.begin_derived_report_replay(&BTreeMap::new(), None).expect("second actual replay"));
    let owner = stale.as_ref().expect("retained stale replay") as *const _;
    let projection = Arc::as_ptr(stale.as_ref().unwrap().replay.state.as_ref().unwrap());
    let registry = Arc::as_ptr(&stale.as_ref().unwrap().registry);
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], description: None, transaction: None }).await.expect("new actual frontier");
    assert!(store.finish_derived_report_replay(&mut stale).is_err(), "stale admission cannot consume the actual replay owner");
    assert_eq!(owner, stale.as_ref().expect("stale replay retained") as *const _);
    assert_eq!(projection, Arc::as_ptr(stale.as_ref().unwrap().replay.state.as_ref().unwrap()));
    assert_eq!(registry, Arc::as_ptr(&stale.as_ref().unwrap().registry));
    assert_eq!(stale.as_ref().unwrap().replay.state.as_ref().unwrap().n, Some(6));
    drop(stale);
    close_demo_artifact_store(&mut store);
    close_demo_artifact_store(&mut foreign);
    eprintln!("[DEBUG] Store-derived provisional and history reads retain their genuine complete owners and refuse foreign or stale authority");
}

` + lawAnchor);
const fixturePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🪆️derived-read/🔣️.json";
if (await Bun.file(resolve(root, fixturePath)).exists()) throw Error("Derived fixture already exists; inspect before replacement");
capsule.pairs.push({ path: fixturePath, before: "", after: JSON.stringify({ law: "ExactStoreDerivedRead", initial: { n: 0 }, derived: { n: 9 }, published: { n: 6 }, admission: { genuine: true, foreign: false, stale: false }, origin: ["StoreHead", "StoreTypedMutation", "StoreHistoryLedger"], forbiddenInput: "CallerSnapshotAlias" }, null, 2) + "\n" });
capsule.state = "HELD: exact Store-derived preview authority; full domain mutation and emission closure pending";
capsule.sourceCapsules.push("global-child-current-immutable-frame-readonly-genuine-fixtures-kernel-issuer-held-pairs.json");
const receipts = [];
for (const pair of capsule.pairs) {
  const file = Bun.file(resolve(root, pair.path));
  const before = await file.exists() ? await file.text() : "";
  const after = pair.regions ? apply(before, pair.regions) : pair.after;
  if (!pair.regions && before !== pair.before) throw Error("Whole guard: " + pair.path);
  pair.before = before;
  pair.after = after;
  receipts.push({ path: pair.path, regions: pair.regions?.length ?? 1, guard: true });
}
await Bun.write(resolve(ticket, "📥️inputs/global-child-current-immutable-frame-genuine-fixtures-and-sealed-preview-producers-held-pairs.json"), JSON.stringify(capsule, null, 2) + "\n");
await Bun.write(resolve(ticket, "🗑️generated/global-child-current-sealed-preview-guard.json"), JSON.stringify({ evidence: "SourceReplayNoCompilerNoRuntime", paths: receipts.length, receipts }, null, 2) + "\n");
console.log(JSON.stringify({ paths: receipts.length, storeRegions: store.pair.regions.length, toolRegions: tool.pair.regions.length, historyRegions: history.pair.regions.length, state: capsule.state }));
const isolated = [];
for (const path of [storePath, toolPath, timePath, unitPath]) {
  const full = capsule.pairs.find((pair: { path: string }) => pair.path === path);
  const regions = full.regions.filter((region: Region) => region.name);
  const before = await Bun.file(resolve(root, path)).text();
  const after = apply(before, regions);
  isolated.push({ path, phase: path === unitPath ? "mount-demand" : "mount-producer", before, after, regions });
}
isolated.push({ ...capsule.pairs.find((pair: { path: string }) => pair.path === fixturePath), phase: "mount-demand" });
const command = 'bun nx run @semio-tech/framework-os-kernel:test --skip-nx-cache --args="derived_child_reads_require_the_exact_store_frontier_and_genuine_operation_or_history_fold --no-fail-fast"';
const launch = { name: "⚖️derived-child-source-authority🦀️native", type: "node-terminal", request: "launch", command, cwd: "${workspaceFolder}", env: { NX_DAEMON: "false", NX_CACHE_PROJECT_GRAPH: "false", SEMIO_TEST_LEVEL: "quick" }, presentation: { group: "4_gate", order: 900.0126 } };
for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
  const before = await Bun.file(resolve(root, path)).text();
  if (before.includes(launch.name)) throw Error("Registered derived law already exists: inspect " + path);
  const eol = before.includes("\r\n") ? "\r\n" : "\n";
  const anchor = eol + '  "configurations": [';
  if (before.split(anchor).length !== 2) throw Error("Catalog anchor " + path);
  const after = before.replace(anchor, anchor + eol + "    " + JSON.stringify(launch, null, 2).split("\n").join(eol + "    ") + ",");
  isolated.push({ path, phase: "mount-demand", before, after });
}
await Bun.write(resolve(ticket, "📥️inputs/global-child-sealed-store-preview-producers-only-held-pairs.json"), JSON.stringify({ state: "ISOLATED COHERENT FAMILY: demand first, exact producer after bounded audit", command, origin: "New authored regions only; immutable global37 remains HELD", pairs: isolated }, null, 2) + "\n");
