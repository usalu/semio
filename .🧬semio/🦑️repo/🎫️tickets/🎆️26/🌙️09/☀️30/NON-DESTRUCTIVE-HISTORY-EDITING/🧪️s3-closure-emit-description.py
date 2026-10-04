"""🏷️ S3-CLOSURE §20.6: deletes `Emit.description` and `Emit::commit` from the runtime — no emission labels its edit, no row
label is read from an edit description. Every anchor is asserted; `--write` applies."""
import re, sys
ROOT = '/Users/ueli/Documents/semio/'
P = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
EDITS = {}
def edit(path, old, new, count=1):
    EDITS.setdefault(path, []).append((old, new, count))

R = P + '🦀️.rs'
edit(R, '''    /// 🏷️ A backfilled edit row's label: the registry label of the verb that authored it, else the app's description,
    /// else its first operation's label `first` (a document leaf's localized `SemanticMutation::label`) — the rule a live
    /// row records (see `authored_row_label`); an edit without operations is named by its id.
    fn backfilled_edit_label<Mu>(edit: &protocol::Edit<Mu>, verb_label: Option<LocalizedLabel>, first: impl FnOnce(&Mu) -> LocalizedLabel) -> LocalizedLabel {
        verb_label.or_else(|| edit.description.clone().map(LocalizedLabel::data)).unwrap_or_else(|| edit.forwards.first().map_or_else(|| LocalizedLabel::data(edit.id.clone()), first))
    }''', '''    /// 🏷️ A backfilled edit row's label: the registry label of the verb that authored it, else its first operation's label
    /// `first` (a document leaf's localized `SemanticMutation::label`) — the rule a live row records; an edit without
    /// operations is named by its id. An edit's stored description is never a label (design §20.6).
    fn backfilled_edit_label<Mu>(edit: &protocol::Edit<Mu>, verb_label: Option<LocalizedLabel>, first: impl FnOnce(&Mu) -> LocalizedLabel) -> LocalizedLabel {
        verb_label.unwrap_or_else(|| edit.forwards.first().map_or_else(|| LocalizedLabel::data(edit.id.clone()), first))
    }''')
edit(R, '''    /// for free), plus an optional description for the resulting edit(s), host effects
    /// (navigate/export/spawn…), and app events.''', '''    /// for free), plus host effects (navigate/export/spawn…) and app events. An emission never labels its edit: the history
    /// row reads its leaves' localized labels (design §20.6).''')
edit(R, '''        pub draft_mutations: Vec<DraftMutation>,
        pub description: Option<String>,
        /// 🛠️ The `ToolTransaction` that authored `artifact_mutations`''', '''        pub draft_mutations: Vec<DraftMutation>,
        /// 🛠️ The `ToolTransaction` that authored `artifact_mutations`''')
edit(R, '''                draft_mutations: Vec::new(),
                description: None,
                transaction: None,
                transaction_phase: TransactionPhase::Commit,''', '''                draft_mutations: Vec::new(),
                transaction: None,
                transaction_phase: TransactionPhase::Commit,''')
edit(R, '''        /// 📌️ One described DOCUMENT edit outside a tool transaction. A gesture never streams through it: it is a tool
        /// machine whose commit publishes [`Self::commit_transaction`]. See `🔖️ToolContract`.
        pub fn commit(artifact_mutations: Vec<Mutation>, description: impl Into<String>) -> Self {
            Self { artifact_mutations, description: Some(description.into()), ..Default::default() }
        }

''', '')
edit(R, '''        /// published as ONE document edit whose every op carries `transaction`; with no description the history row
        /// is labelled from the mutations' own `MutationKind` labels.''', '''        /// published as ONE document edit whose every op carries `transaction`; the history row is labelled from the
        /// mutations' own `MutationKind` labels.''')
edit(R, '''                        if let Some(description) = emit.description.as_mut().filter(|value| !value.is_empty()) {
                            return Ok(Self::retire_string_scalar(description, maximum_bytes).expect("nonempty description"));
                        }
''', '')
edit(R, '''        child_emits: Vec<ChildEmit>,
        description: Option<String>,
        transaction: Option<protocol::TransactionRef>,
        receipt: Option<ChildPublicationResultV1>,''', '''        child_emits: Vec<ChildEmit>,
        transaction: Option<protocol::TransactionRef>,
        receipt: Option<ChildPublicationResultV1>,''')
edit(R, '''        fn new(artifact_mutations: Vec<A::Mutation>, child_emits: Vec<ChildEmit>, description: Option<String>, transaction: Option<protocol::TransactionRef>) -> Result<Self, Fault> {''', '''        fn new(artifact_mutations: Vec<A::Mutation>, child_emits: Vec<ChildEmit>, transaction: Option<protocol::TransactionRef>) -> Result<Self, Fault> {''')
edit(R, '''            Ok(Self { artifact_mutations, child_emits, description, transaction, receipt: None, fault: None, phase: PendingChildGroupPublicationPhase::Ready })''', '''            Ok(Self { artifact_mutations, child_emits, transaction, receipt: None, fault: None, phase: PendingChildGroupPublicationPhase::Ready })''')
edit(R, '''            if let Some(description) = self.description.as_mut().filter(|value| !value.is_empty()) {
                return MountedTypedCommandFullOperation::<A>::retire_string_scalar(description, maximum_bytes).expect("nonempty child-group description");
            }
            if self.description.take().is_some() {
                return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
''', '')
edit(R, '''self.artifact_mutations.is_empty() && self.child_emits.is_empty() && self.description.is_none() && self.receipt.is_none()''', '''self.artifact_mutations.is_empty() && self.child_emits.is_empty() && self.receipt.is_none()''')
edit(R, '''    fn artifact_lane_commands<M>(mutations: Vec<M>, description: Option<String>, transaction: Option<protocol::TransactionRef>, phase: TransactionPhase, open: Option<&store::OpenToolTransaction>) -> Vec<ArtifactCommand<M>> {''', '''    fn artifact_lane_commands<M>(mutations: Vec<M>, transaction: Option<protocol::TransactionRef>, phase: TransactionPhase, open: Option<&store::OpenToolTransaction>) -> Vec<ArtifactCommand<M>> {''')
edit(R, '''            (transaction, _) => vec![ArtifactCommand::Apply { mutations, description, transaction }],''', '''            (transaction, _) => vec![ArtifactCommand::Apply { mutations, description: None, transaction }],''')
edit(R, '''            let edit = if artifact_lane { self.store.envelope().vcs.edits.last().map(|edit| (edit.id.clone(), edit.description.clone())) } else { self.config_store.envelope().vcs.edits.last().map(|edit| (edit.id.clone(), edit.description.clone())) };
            let Some((edit_id, description)) = edit else { return };''', '''            let edit = if artifact_lane { self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) } else { self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) };
            let Some(edit_id) = edit else { return };''')
edit(R, '''            let label = self.authored_row_label(&verb, description.as_deref());
            if artifact_lane {''', '''            let label = self.verb_label(&verb);
            if artifact_lane {''')
edit(R, '''        /// 🏷️ The label a row authored by `verb` records: the verb's registry label in every locale, else the app's
        /// `description` as locale-invariant data — the same rule [`backfilled_edit_label`] applies after a reload.
        fn authored_row_label(&self, verb: &str, description: Option<&str>) -> Option<LocalizedLabel> {
            self.verb_label(verb).or_else(|| description.map(LocalizedLabel::data))
        }

''', '')
edit(R, '''                    (Some(edit), _) if edit.description.is_none() && tool_run_label.is_some() => tool_run_label.expect("tool run label checked above"),
                    (Some(edit), Some(first)) if transaction.is_some() && edit.description.is_none() => {''', '''                    (Some(_), _) if tool_run_label.is_some() => tool_run_label.expect("tool run label checked above"),
                    (Some(_), Some(first)) if transaction.is_some() => {''')
edit(R, '''                    (Some(edit), _) if edit.description.is_none() && op_count == 1 && leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
                    (Some(edit), _) if verb_label.is_some() || edit.description.is_some() => verb_label.unwrap_or_else(|| LocalizedLabel::data(edit.description.clone().unwrap_or_default())),
                    (Some(_), _) if leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
                    (None, Some(first)) if !child_edits.is_empty() && child_edits.iter().all(|child| child.description.is_none()) => history_leaf_row_label(&first.label, op_count),''', '''                    (Some(_), _) if op_count == 1 && leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
                    (Some(_), _) if verb_label.is_some() => verb_label.expect("verb label checked above"),
                    (Some(_), _) if leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
                    (None, Some(first)) if !child_edits.is_empty() => history_leaf_row_label(&first.label, op_count),''')
edit(R, '''            let Emit { artifact_mutations, config_mutations, window_config_mutations, draft_mutations, description, transaction, transaction_phase, effects, extension_invocations, events, ui_scope, child_emits, interaction_writes, tasks } = emit;''', '''            let Emit { artifact_mutations, config_mutations, window_config_mutations, draft_mutations, transaction, transaction_phase, effects, extension_invocations, events, ui_scope, child_emits, interaction_writes, tasks } = emit;''')
edit(R, '''                    self.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, description: description.clone().unwrap_or_default(), foreign });''', '''                    self.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });''')
edit(R, '''                let config_command = ArtifactCommand::Apply { mutations: config_mutations, description: description.clone(), transaction: None };''', '''                let config_command = ArtifactCommand::Apply { mutations: config_mutations, description: None, transaction: None };''')
edit(R, '''                    self.window_config_store.dispatch(&authority, &meta.actor, mutation, description.clone()).await?;''', '''                    self.window_config_store.dispatch(&authority, &meta.actor, mutation).await?;''')
edit(R, '''                let result = self.dispatch_emit_group(verb, &artifact_mutations, &child_emits, &description, effects, events, ui_scope, config_edit_id, meta, None, transaction).await?;''', '''                let result = self.dispatch_emit_group(verb, &artifact_mutations, &child_emits, None, effects, events, ui_scope, config_edit_id, meta, None, transaction).await?;''')
edit(R, '''            let log_label = self.authored_row_label(verb, description.as_deref());
            let vcs_commands = artifact_lane_commands(artifact_mutations, description, transaction, transaction_phase, self.store.open_transaction());''', '''            let log_label = self.verb_label(verb);
            let vcs_commands = artifact_lane_commands(artifact_mutations, transaction, transaction_phase, self.store.open_transaction());''')
edit(R, '''                .dispatch_emit_group(&format!("transaction:{txn_id}"), &ops, &children, &description, Vec::new(), Vec::new(), UiDirtyScope::Full, None, meta, Some(txn_id.to_string()), None)''', '''                .dispatch_emit_group(&format!("transaction:{txn_id}"), &ops, &children, description, Vec::new(), Vec::new(), UiDirtyScope::Full, None, meta, Some(txn_id.to_string()), None)''')
edit(R, '''            child_emits: &[ChildEmit],
            description: &Option<String>,
            effects: Vec<Effect>,''', '''            child_emits: &[ChildEmit],
            description: Option<String>,
            effects: Vec<Effect>,''')
edit(R, '''            let group_meta = GroupMeta { actor: Some(meta.actor.clone()), description: (*description).clone(), group_id, transaction };''', '''            let group_meta = GroupMeta { actor: Some(meta.actor.clone()), description, group_id, transaction };''')
edit(R, '''            let label = self.authored_row_label(verb, description.as_deref());
            self.record_command(verb, kind, label, parent_edit_id, config_edit_id, None);''', '''            let label = self.verb_label(verb);
            self.record_command(verb, kind, label, parent_edit_id, config_edit_id, None);''')
edit(R, '''            let result = self.dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, &pending.description, Vec::new(), Vec::new(), UiDirtyScope::None, None, &mounted.meta, None, pending.transaction.clone()).await;''', '''            let result = self.dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, None, Vec::new(), Vec::new(), UiDirtyScope::None, None, &mounted.meta, None, pending.transaction.clone()).await;''')
edit(R, '''                                self.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, description: emit.description.clone().unwrap_or_default(), foreign });
                                emit.artifact_mutations.clear();
                                emit.description = None;
                                return Ok(());''', '''                                self.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
                                emit.artifact_mutations.clear();
                                return Ok(());''')
edit(R, '''                        let mutations = std::mem::take(&mut emit.artifact_mutations);
                        let description = emit.description.take();
                        let transaction = emit.transaction.take();''', '''                        let mutations = std::mem::take(&mut emit.artifact_mutations);
                        let transaction = emit.transaction.take();''')
edit(R, '''mounted.meta.actor.clone(), mutations, description, self.artifact_one_item_factory.as_ref(), trans''', '''mounted.meta.actor.clone(), mutations, None, self.artifact_one_item_factory.as_ref(), trans''')
edit(R, '''                                mutations,
                                description,
                                HistoryLane::Document,
                                self.artifact_one_item_factory.as_ref(),''', '''                                mutations,
                                None,
                                HistoryLane::Document,
                                self.artifact_one_item_factory.as_ref(),''')
edit(R, '''                                let (reason, mutations, description) = rejected.into_owners();
                                emit.artifact_mutations = mutations;
                                emit.description = description;
                                return Err(plugin_sdk_fault(reason));''', '''                                let (reason, mutations, _) = rejected.into_owners();
                                emit.artifact_mutations = mutations;
                                return Err(plugin_sdk_fault(reason));''')
edit(R, '''PendingChildGroupPublication::new(std::mem::take(&mut emit.artifact_mutations), std::mem::take(&mut emit.child_emits), emit.description.take(), emit.transaction.take())?''', '''PendingChildGroupPublication::new(std::mem::take(&mut emit.artifact_mutations), std::mem::take(&mut emit.child_emits), emit.transaction.take())?''')
edit(R, '''        pub(crate) local_ops: Vec<Vec<u8>>,
        pub(crate) description: String,
        pub(crate) foreign: Vec<protocol::ForeignStep>,''', '''        pub(crate) local_ops: Vec<Vec<u8>>,
        pub(crate) foreign: Vec<protocol::ForeignStep>,''')

edit(R, '''local_ops: proposal.local_ops, description: proposal.description, coalesce_key: String::new(), foreign }''', '''local_ops: proposal.local_ops, description: String::new(), coalesce_key: String::new(), foreign }''')

TM = P + '🛠️tool-machine/🦀️.rs'
edit(TM, '''                    emit.transaction = Some(transaction);
                    emit.description = None;
                }
                vec![step]''', '''                    emit.transaction = Some(transaction);
                }
                vec![step]''')
edit(TM, '''                emit.artifact_mutations = mutations;
                emit.transaction = Some(transaction);
                emit.description = None;
            }''', '''                emit.artifact_mutations = mutations;
                emit.transaction = Some(transaction);
            }''')

WC = P + '🪟️window/🎚️config/🦀️.rs'
edit(WC, '''    fn dispatch<'a>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation, description: Option<String>) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>>;''', '''    fn dispatch<'a>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>>;''')
edit(WC, '''    fn dispatch<'a>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation, description: Option<String>) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>> {''', '''    fn dispatch<'a>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>> {''')
edit(WC, '''            let command = store::ArtifactCommand::Apply { mutations: vec![*typed], description, transaction: None };''', '''            let command = store::ArtifactCommand::Apply { mutations: vec![*typed], description: None, transaction: None };''')
edit(WC, '''    pub(crate) async fn dispatch(&mut self, authority: &WindowConfigAuthority, actor: &str, mutation: WindowConfigMutation, description: Option<String>) -> Result<(), Fault> {''', '''    pub(crate) async fn dispatch(&mut self, authority: &WindowConfigAuthority, actor: &str, mutation: WindowConfigMutation) -> Result<(), Fault> {''')
edit(WC, '''            .dispatch(actor, mutation, description)
            .await''', '''            .dispatch(actor, mutation)
            .await''')

S = '✏️s/🔌️plugins/'
FLOW = S + '🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
edit(FLOW, '''            || !emit.draft_mutations.is_empty()
            || emit.description.is_some()
            || !emit.effects.is_empty()''', '''            || !emit.draft_mutations.is_empty()
            || !emit.effects.is_empty()''')
NOTE = S + '🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs'
edit(NOTE, '''        if self.accumulated.description.is_some() && emit.description.is_some() && self.accumulated.description != emit.description {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("note.retained.description"), "Note semantic units produced incompatible edit descriptions"));
        }
        if self.accumulated.description.is_none() {
            self.accumulated.description = emit.description.take();
        }
''', '')
edit(NOTE, '''            || self.accumulated.description.take().is_some()
            || self.accumulated.transaction.take().is_some()''', '''            || self.accumulated.transaction.take().is_some()''')
edit(NOTE, '''            && self.accumulated.description.is_none()
            && self.accumulated.transaction.is_none()''', '''            && self.accumulated.transaction.is_none()''')
P5 = S + '🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️puzzle5d-retained-retirement-laws/🦀️.rs'
edit(P5, ''' children={} cap={} description={:?}) ephemeral(''', ''' children={} cap={}) ephemeral(''')
edit(P5, '''                        emit.description.as_ref().map(String::len),
''', '')
P3 = S + '🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs'
edit(P3, '''    assert_eq!(emit.description, None, "an example load's history row is labelled by its leaves, never by a hand-written description");
''', '', 2)

TESTS = P + '🧪️tests/'
TEST_EDITS = {
    TESTS + '🔬️plugin-runtime-plugin-builder-contract/🦀️.rs': [
        (', description: Some("increment".into()), ..Default::default() }),', ', ..Default::default() }),'),
        (', description: Some("targetWindow".into()), ..Default::default() }),', ', ..Default::default() }),'),
        (', description: Some("bulk edit".into()), ..Default::default() }),', ', ..Default::default() }),'),
        (', child_emits, description: Some("retained composite edit".into()), ..Default::default() }', ', child_emits, ..Default::default() }'),
        (', description: Some("retained bulk edit".into()), ..Default::default() },', ', ..Default::default() },'),
        ('        #[dsl(key = "commit-label")]\n        CommitLabel { value: String },\n', ''),
        ('            TestCommand::CommitLabel { .. } => "commitLabel",\n', ''),
        ('            TestCommand::CommitLabel { value } => Ok(Emit::commit(vec![TestMutation::SetLabel(SetLabel { value: value.clone() })], "commit label")),\n', ''),
        ('        let builder = ::semio_framework_async::poll::resolve_ready(builder.mutation("commitLabel", LocalizedLabel::data("Commit Label")));\n', ''),
        ('    /// 🧷️ A streamed gesture is one undo step on the typed route too, while each committed label is its own edit: one undo\n    /// only reverts the last commit.', '    /// 🧷️ A streamed gesture is one undo step on the typed route too, while each plain label edit is its own edit: one undo\n    /// only reverts the last one.'),
        ('            app.dispatch_typed(TestCommand::CommitLabel { value: value.into() }, &meta()).await.expect("commitLabel");', '            app.dispatch_typed(TestCommand::SetLabel { value: value.into() }, &meta()).await.expect("setLabel");'),
    ],
    TESTS + '⏳️completion/🦀️.rs': [
        ('    "commitLabel",\n', ''),
        ('    ArtifactToolPublicationContract { tool_id: "commitLabel", lanes: &[ArtifactToolPublicationLane::Artifact] },\n', ''),
    ],
    TESTS + '🧬️mutation-fixtures-dummy/🦀️.rs': [(', description: Some("increment".into()), ..Default::default() }', ', ..Default::default() }')],
    TESTS + '🧬️mutation-fixtures-surface/🦀️.rs': [(', description: Some("increment".into()), ..Default::default() }', ', ..Default::default() }', 2)],
    TESTS + '🧬️mutation-fixtures-transaction/🦀️.rs': [(', description: Some("increment".into()), ..Default::default() }', ', ..Default::default() }', 2), (', description: Some("increment-and-notify".into()), ..Default::default() }', ', ..Default::default() }', 2)],
    TESTS + '🧪️history-label-reload/🦀️.rs': [
        ("//! locale-neutral `verb`, its description and its operations).", "//! locale-neutral `verb` and its operations; an edit's description is never a label, design §20.6)."),
        ("/// 🎮️ The fixture app's commands: an app-described rename under an undeclared verb, a retitle whose declared verb\n/// outranks its description, an undescribed two-operation reset under a declared verb, an undescribed single count and\n/// an undescribed two-operation pair under an undeclared verb.", "/// 🎮️ The fixture app's commands: a single-leaf rename under an undeclared verb, a single-leaf retitle under a declared\n/// verb, a two-operation reset under a declared verb, a single count and a two-operation pair under an undeclared verb."),
        ('/// ✍️ The operations and description a fixture command authors.\nfn authored(command: &LabelReloadCommand) -> (Vec<TestMutation>, Option<String>) {\n    match command {\n        LabelReloadCommand::Rename { value } => (vec![set_label(value)], Some(format!("Renamed to {value}"))),\n        LabelReloadCommand::Retitle { value } => (vec![set_label(value)], Some(format!("Retitled to {value}"))),\n        LabelReloadCommand::Reset { value } => (vec![set_count(*value), set_label("reset")], None),\n        LabelReloadCommand::Count { value } => (vec![set_count(*value)], None),\n        LabelReloadCommand::Pair { value } => (vec![set_count(*value), set_label("pair")], None),\n    }\n}', '/// ✍️ The operations a fixture command authors.\nfn authored(command: &LabelReloadCommand) -> Vec<TestMutation> {\n    match command {\n        LabelReloadCommand::Rename { value } | LabelReloadCommand::Retitle { value } => vec![set_label(value)],\n        LabelReloadCommand::Reset { value } => vec![set_count(*value), set_label("reset")],\n        LabelReloadCommand::Count { value } => vec![set_count(*value)],\n        LabelReloadCommand::Pair { value } => vec![set_count(*value), set_label("pair")],\n    }\n}'),
        ('            LabelReloadCommand::Rename { .. } => "renameDescribed",', '            LabelReloadCommand::Rename { .. } => "renameUndeclared",'),
        ('        let (artifact_mutations, description) = authored(command);\n        Ok(Emit { artifact_mutations, description, ..Default::default() })', '        Ok(Emit::mutations(authored(command)))'),
        ("/// the fixture's, live, after a text reload and after a pack reload — an app-described, a verb-described and a\n/// leaf-described row alike — and the reloaded edits keep their verbs, declared by this app or not.", "/// the fixture's, live, after a text reload and after a pack reload — a verb-labelled and a leaf-labelled row alike —\n/// and the reloaded edits keep their verbs, declared by this app or not."),
        ('        let (artifact_mutations, description) = authored(&command(case));\n        let meta', '        let artifact_mutations = authored(&command(case));\n        let meta'),
        ('Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { artifact_mutations, description, ..Default::default() }', 'Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::mutations(artifact_mutations)'),
    ],
    TESTS + '🧪️history-label-reload/🟦️.ts': [
        ('type Case = Readonly<{ id: string; command: Readonly<{ kind: string; value: string | number }>; verb: string; description: string | null; leaves: readonly Label[]; expected: Label }>;', 'type Case = Readonly<{ id: string; command: Readonly<{ kind: string; value: string | number }>; verb: string; leaves: readonly Label[]; expected: Label }>;'),
        ('/** 🏷️ A single undescribed leaf keeps its label; else the declared verb\'s label, the description, the first leaf (+N). */\nexport function historyRowLabel(registry: Fixture["registry"], testCase: Case): Label {\n  const [first, ...rest] = testCase.leaves;\n  const verbLabel = registry.find((row) => row.verb === testCase.verb)?.label ?? null;\n  if (testCase.description === null && rest.length === 0) return first!;\n  if (verbLabel !== null) return verbLabel;\n  if (testCase.description !== null) return { en: testCase.description, de: testCase.description };', '/** 🏷️ A single leaf keeps its label; else the declared verb\'s label, else the first leaf (+N). An edit\'s description is never a\n * label (design §20.6). */\nexport function historyRowLabel(registry: Fixture["registry"], testCase: Case): Label {\n  const [first, ...rest] = testCase.leaves;\n  const verbLabel = registry.find((row) => row.verb === testCase.verb)?.label ?? null;\n  if (rest.length === 0) return first!;\n  if (verbLabel !== null) return verbLabel;'),
    ],
}
for path, pairs in TEST_EDITS.items():
    for pair in pairs:
        edit(path, *pair)

write = '--write' in sys.argv
failures = []
texts = {}
for path, items in EDITS.items():
    text = open(ROOT + path, encoding='utf-8').read()
    for old, new, count in items:
        found = text.count(old)
        if found != count: failures.append(f'{path.split("/")[-2]} {old[:90]!r}: {found}')
        text = text.replace(old, new)
    texts[path] = text
if write and not failures:
    for path, text in texts.items():
        open(ROOT + path, 'w', encoding='utf-8').write(text)
print(f'files={len(EDITS)} failures={len(failures)} written={write and not failures}')
print('\n'.join(failures))
