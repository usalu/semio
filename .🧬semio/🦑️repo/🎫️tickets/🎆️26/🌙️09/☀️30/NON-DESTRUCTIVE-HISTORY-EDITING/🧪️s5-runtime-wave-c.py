"""🚫️ Wave C (design §22.1, §22.2 guest half, §22.6; coordinator decision "Restore"): the runtime half.

- Manifest `🔖️HistoryEdit`: `historyEditWithdraw{mutationId?, store?, generation?}` and the thirteenth verb
  `historyEditRestore{mutationId, store?, generation?}` (Rust, TS twin, fixture, schema).
- Kernel wire: `HistoryMutationEntry.withdrawable`.
- Plugin runtime (`TT`): a row's Withdraw opens the session on a withdrawn draft of ANY mutation the store's supersede
  law lets withdraw (no input schema needed), a row's Restore takes one accepted draft back, no panic on the user-input
  path (`timeTravel.not-editable`, `timeTravel.schema-unavailable`, `timeTravel.editor-closed`), the editor's withdrawn
  state is the session's.
- Design §22.20: a mutation is editable only when its payload schema describes at least one input an editor shows as a row
  (an editor with zero rows never opens); such a mutation keeps Withdraw.
- History panel (`PLG` `🔖️HistoryPanel`): row actions `[Edit, Withdraw | Restore]` with localized disabled reasons; blocking
  rows lead an entry's children.
- Design §22.7 guest half (`time_travel_input_row`): Segmented renders as a segmented select, IconSelect as an icon picker,
  Multiline as a multi-line field; a text longer than one UI text holds is read-only (a commit never writes it clipped).
- Laws: plugin scenario fixture (+ schema, Rust driver, TS oracle), a store-level law over an operation without editable
  inputs, and the three struct literals of other WPs' tests that gain the new fields.

Loaded by `🧪️s5-runtime-land.py`; applies on top of waves A and B. The two bundled TS twins (`MANIFEST_TS`, `KERNEL_TS`) land
as wave `c-ts` under the `serve` lock (rule 57).
"""

FW = "🧰️framework/🔨️modules"
OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
MANIFEST = f"{FW}/🛂️manifest"
KERNEL = f"{FW}/🎠️kernel"
PLUGIN = f"{OSM}/🔌️plugin"
TT = f"{PLUGIN}/⏪️time-travel/🦀️.rs"
PLG = f"{PLUGIN}/🦀️.rs"
WGPU_LAW = f"{OSM}/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs"

GENERATION_ARG = '{ "id": "generation", "schema": "integer", "required": false, "hidden": true }'
STORE_ARG = '{ "id": "store", "schema": "text", "required": false, "hidden": true }'

MANIFEST_RS = [
    (
        """/// 🚫️ Drafts the edited mutation as withdrawn: it folds as a no-op.
pub const HISTORY_EDIT_WITHDRAW_ACTION_ID: &str = "historyEditWithdraw";
""",
        """/// 🚫️ Drafts a mutation as withdrawn (it folds as a no-op): the one being edited, or — with `mutationId`, a history
/// row's Withdraw — the named one, opening (or retargeting) the session on it; no input schema is needed.
pub const HISTORY_EDIT_WITHDRAW_ACTION_ID: &str = "historyEditWithdraw";
""",
    ),
    (
        """/// ↩️ Drops the draft and returns to the previous stage.
pub const HISTORY_EDIT_DISCARD_ACTION_ID: &str = "historyEditDiscard";
""",
        """/// ↩️ Drops the draft and returns to the previous stage.
pub const HISTORY_EDIT_DISCARD_ACTION_ID: &str = "historyEditDiscard";
/// 🪃️ Takes the accepted draft of `mutationId` back while reviewing — the mutation applies with its recorded input again —
/// and replays the rest; taking the only accepted draft back leaves history editing with nothing changed.
pub const HISTORY_EDIT_RESTORE_ACTION_ID: &str = "historyEditRestore";
""",
    ),
    (
        """pub const HISTORY_EDIT_ACTION_IDS: [&str; 12] = [""",
        """pub const HISTORY_EDIT_ACTION_IDS: [&str; 13] = [""",
    ),
    (
        """    HISTORY_EDIT_DISCARD_ACTION_ID,
    HISTORY_EDIT_FINALIZE_ACTION_ID,
    HISTORY_EDIT_COMMIT_ACTION_ID,
    HISTORY_EDIT_BACK_ACTION_ID,
    HISTORY_EDIT_EXIT_ACTION_ID,
    HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID,
    HISTORY_EDIT_RERUN_ACTION_ID,
];
""",
        """    HISTORY_EDIT_DISCARD_ACTION_ID,
    HISTORY_EDIT_RESTORE_ACTION_ID,
    HISTORY_EDIT_FINALIZE_ACTION_ID,
    HISTORY_EDIT_COMMIT_ACTION_ID,
    HISTORY_EDIT_BACK_ACTION_ID,
    HISTORY_EDIT_EXIT_ACTION_ID,
    HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID,
    HISTORY_EDIT_RERUN_ACTION_ID,
];
""",
    ),
    (
        """/// 🪪️ `historyEditBegin`'s mutation id argument (`<editId>#<opIndex>`).
pub const HISTORY_EDIT_ARG_MUTATION_ID: &str = "mutationId";
/// 🧩️ `historyEditBegin`'s member store argument (`<slot>/<childId>`): the composed member store that holds the mutation
/// (design §12); absent for the document's own store.
""",
        """/// 🪪️ The mutation id argument (`<editId>#<opIndex>`) of `historyEditBegin`, of a row's `historyEditWithdraw` and of
/// `historyEditRestore`.
pub const HISTORY_EDIT_ARG_MUTATION_ID: &str = "mutationId";
/// 🧩️ The member store argument (`<slot>/<childId>`) beside every `mutationId`: the composed member store that holds the
/// mutation (design §12); absent for the document's own store.
""",
    ),
    (
        """/// ✏️ The twelve reserved history-edit verbs, part of [`history_action_definitions`]: never in the palette, no chords""",
        """/// ✏️ The thirteen reserved history-edit verbs, part of [`history_action_definitions`]: never in the palette, no chords""",
    ),
    (
        """    let verb = |id: &str, en: &str, de: &str, icon: &str| ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(id, LocalizedLabel::native(en, de), ActionKind::History, icon) };
    vec![
        verb(HISTORY_EDIT_BEGIN_ACTION_ID, "Edit Mutation", "Mutation bearbeiten", "pencil")
            .describe(LocalizedLabel::native(
                "Opens history editing on one applied mutation: the artifact shows the state right before it with its inputs editable, and nothing downstream is applied until the draft is accepted.",
                "Öffnet die Verlaufsbearbeitung für eine angewendete Mutation: Das Artefakt zeigt den Zustand direkt davor mit bearbeitbaren Eingaben, und nichts Späteres wird angewendet, bis der Entwurf übernommen ist.",
            ))
            .use_when(["edit an earlier step", "change a past operation", "fix a mutation in the history"])
            .with_args([
                history_edit_hidden_arg(ActionArgDef::text(HISTORY_EDIT_ARG_MUTATION_ID, LocalizedLabel::native("Mutation", "Mutation")).required()),
                history_edit_hidden_arg(ActionArgDef::text(HISTORY_EDIT_ARG_STORE, LocalizedLabel::native("Member store", "Mitgliedsspeicher"))),
            ]),
""",
        """    let verb = |id: &str, en: &str, de: &str, icon: &str| ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(id, LocalizedLabel::native(en, de), ActionKind::History, icon) };
    let mutation = || history_edit_hidden_arg(ActionArgDef::text(HISTORY_EDIT_ARG_MUTATION_ID, LocalizedLabel::native("Mutation", "Mutation")));
    let store = || history_edit_hidden_arg(ActionArgDef::text(HISTORY_EDIT_ARG_STORE, LocalizedLabel::native("Member store", "Mitgliedsspeicher")));
    vec![
        verb(HISTORY_EDIT_BEGIN_ACTION_ID, "Edit Mutation", "Mutation bearbeiten", "pencil")
            .describe(LocalizedLabel::native(
                "Opens history editing on one applied mutation: the artifact shows the state right before it with its inputs editable, and nothing downstream is applied until the draft is accepted.",
                "Öffnet die Verlaufsbearbeitung für eine angewendete Mutation: Das Artefakt zeigt den Zustand direkt davor mit bearbeitbaren Eingaben, und nichts Späteres wird angewendet, bis der Entwurf übernommen ist.",
            ))
            .use_when(["edit an earlier step", "change a past operation", "fix a mutation in the history"])
            .with_args([mutation().required(), store()]),
""",
    ),
    (
        """            .describe(LocalizedLabel::native(
                "Drafts the mutation being edited as withdrawn, so it no longer changes the artifact; use it for a step whose inputs cannot fix its error.",
                "Entwirft die bearbeitete Mutation als zurückgezogen, sodass sie das Artefakt nicht mehr verändert; für einen Schritt, dessen Fehler sich über die Eingaben nicht beheben lässt.",
            ))
            .use_when(["drop this step", "skip this operation"])
            .with_args([generation()]),
""",
        """            .describe(LocalizedLabel::native(
                "Drafts a mutation as withdrawn, so it no longer changes the artifact: the one being edited, or the one named by mutationId, which opens history editing on it; use it for a step whose inputs cannot fix its error or cannot be edited at all.",
                "Entwirft eine Mutation als zurückgezogen, sodass sie das Artefakt nicht mehr verändert: die bearbeitete oder die mit mutationId benannte, für die sich die Verlaufsbearbeitung öffnet; für einen Schritt, dessen Fehler sich über die Eingaben nicht beheben lässt oder dessen Eingaben gar nicht bearbeitbar sind.",
            ))
            .use_when(["drop this step", "skip this operation", "withdraw a mutation from the history"])
            .with_args([mutation(), store(), generation()]),
""",
    ),
    (
        """            .use_when(["discard this draft"])
            .with_args([generation()]),
""",
        """            .use_when(["discard this draft"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_RESTORE_ACTION_ID, "Restore Mutation", "Mutation wiederherstellen", "undo-2")
            .describe(LocalizedLabel::native(
                "Takes the accepted draft of one mutation back while reviewing, so it applies with its recorded input again, and replays every later mutation; taking the only accepted draft back leaves history editing with nothing changed.",
                "Nimmt beim Prüfen den übernommenen Entwurf einer Mutation zurück, sodass sie wieder mit ihrer aufgezeichneten Eingabe angewendet wird, und wendet alle späteren Mutationen neu an; wird der einzige übernommene Entwurf zurückgenommen, endet die Verlaufsbearbeitung ohne Änderung.",
            ))
            .use_when(["take this draft back", "bring the withdrawn step back", "undo one change of the history edit"])
            .with_args([mutation().required(), store(), generation()]),
""",
    ),
]

MANIFEST_TS = [
    (
        """/** ✏️ Mirrors Rust `HISTORY_EDIT_ACTION_IDS` — the twelve reserved history-edit verbs, host-driven on the instance's""",
        """/** ✏️ Mirrors Rust `HISTORY_EDIT_ACTION_IDS` — the thirteen reserved history-edit verbs, host-driven on the instance's""",
    ),
    (
        """  "historyEditDiscard",
  "historyEditFinalize",
""",
        """  "historyEditDiscard",
  "historyEditRestore",
  "historyEditFinalize",
""",
    ),
    (
        """/** 🪪️ `historyEditBegin`'s mutation id argument — mirrors Rust `HISTORY_EDIT_ARG_MUTATION_ID`. */""",
        """/** 🪪️ The mutation id argument of `historyEditBegin`, of a row's `historyEditWithdraw` and of `historyEditRestore` — mirrors Rust `HISTORY_EDIT_ARG_MUTATION_ID`. */""",
    ),
    (
        """/** 🧩️ `historyEditBegin`'s member store argument (`<slot>/<childId>`), absent for the document's own store — mirrors Rust `HISTORY_EDIT_ARG_STORE`. */""",
        """/** 🧩️ The member store argument (`<slot>/<childId>`) beside every `mutationId`, absent for the document's own store — mirrors Rust `HISTORY_EDIT_ARG_STORE`. */""",
    ),
]

MANIFEST_FIXTURE = [
    (
        """`generation` is optional everywhere: absent addresses the live session.""",
        """`generation` is optional everywhere: absent addresses the live session. `historyEditWithdraw` with `mutationId` is a history row's Withdraw (it opens or retargets the session on that mutation with a withdrawn draft; without it the mutation being edited is withdrawn); `historyEditRestore` takes one accepted draft back.""",
    ),
    (
        f"""    {{ "id": "historyEditWithdraw", "iconId": "eye-off", "label": {{ "en": "Withdraw Mutation", "de": "Mutation zurückziehen" }}, "destructive": false, "args": [{GENERATION_ARG}] }},
""",
        f"""    {{ "id": "historyEditWithdraw", "iconId": "eye-off", "label": {{ "en": "Withdraw Mutation", "de": "Mutation zurückziehen" }}, "destructive": false, "args": [{{ "id": "mutationId", "schema": "text", "required": false, "hidden": true }}, {STORE_ARG}, {GENERATION_ARG}] }},
""",
    ),
    (
        f"""    {{ "id": "historyEditDiscard", "iconId": "x", "label": {{ "en": "Discard Draft", "de": "Entwurf verwerfen" }}, "destructive": false, "args": [{GENERATION_ARG}] }},
""",
        f"""    {{ "id": "historyEditDiscard", "iconId": "x", "label": {{ "en": "Discard Draft", "de": "Entwurf verwerfen" }}, "destructive": false, "args": [{GENERATION_ARG}] }},
    {{ "id": "historyEditRestore", "iconId": "undo-2", "label": {{ "en": "Restore Mutation", "de": "Mutation wiederherstellen" }}, "destructive": false, "args": [{{ "id": "mutationId", "schema": "text", "required": true, "hidden": true }}, {STORE_ARG}, {GENERATION_ARG}] }},
""",
    ),
]

MANIFEST_FIXTURE_SCHEMA = [
    (
        """"actions": { "type": "array", "minItems": 12, "maxItems": 12,""",
        """"actions": { "type": "array", "minItems": 13, "maxItems": 13,""",
    ),
]

KERNEL_RS = [
    (
        """/// one) and its editing state. `editable` = the op has an input schema and emits no foreign steps; `pending` = it is
""",
        """/// one) and its editing state. `editable` = the op has an input schema and emits no foreign steps; `withdrawable` = the
/// store's supersede law lets this actor withdraw it and it is not withdrawn yet (design §22.1); `pending` = it is
""",
    ),
    (
        """    #[serde(default)]
    #[value(default)]
    pub editable: bool,
    #[serde(default)]
    #[value(default)]
    pub pending: bool,
""",
        """    #[serde(default)]
    #[value(default)]
    pub editable: bool,
    #[serde(default)]
    #[value(default)]
    pub withdrawable: bool,
    #[serde(default)]
    #[value(default)]
    pub pending: bool,
""",
    ),
]

KERNEL_TS = [
    (
        """/** ✏️ One applied mutation of a history row, mirrored from Rust `HistoryMutationEntry`: `pending` = downstream of""",
        """/** ✏️ One applied mutation of a history row, mirrored from Rust `HistoryMutationEntry`: `withdrawable` = the store's
 * supersede law lets this actor withdraw it and it is not withdrawn yet, `pending` = downstream of""",
    ),
    (
        """  readonly editable?: boolean;
  readonly pending?: boolean;
""",
        """  readonly editable?: boolean;
  readonly withdrawable?: boolean;
  readonly pending?: boolean;
""",
    ),
]

KERNEL_SCHEMA = [
    (
        """        "editable": { "type": "boolean" },
        "pending": { "type": "boolean" },""",
        """        "editable": { "type": "boolean" },
        "withdrawable": { "type": "boolean" },
        "pending": { "type": "boolean" },""",
    ),
]

KERNEL_FIXTURE = [
    (
        """"editable": true, "store": "content/flow-content-1" }] }]""",
        """"editable": true, "withdrawable": true, "store": "content/flow-content-1" }] }]""",
    ),
    (
        """    { "id": "next-problem-without-mutation",""",
        """    { "id": "withdrawable-not-a-boolean", "patch": { "cursor": 1, "upserts": [{ "seq": 1, "editId": "e-1", "actionId": "apply", "label": { "native": { "en": "Apply", "de": "Anwenden" }, "reuse": { "en": "Apply", "de": "Anwenden" } }, "kind": "mutation", "timestamp": "t", "mutations": [{ "mutationId": "e-1#0", "position": 0, "opIndex": 0, "label": { "native": { "en": "x", "de": "x" }, "reuse": { "en": "x", "de": "x" } }, "withdrawable": "yes" }] }] }, "reason": "withdrawable is a boolean" },
    { "id": "next-problem-without-mutation",""",
    ),
]

TT_RS = [
    (
        "HISTORY_EDIT_RERUN_ACTION_ID, HISTORY_EDIT_USE_SELECTION_ACTION_ID,",
        "HISTORY_EDIT_RERUN_ACTION_ID, HISTORY_EDIT_RESTORE_ACTION_ID, HISTORY_EDIT_USE_SELECTION_ACTION_ID,",
    ),
    (
        "TIME_TRAVEL_COMMIT_FAILED_CODE, TIME_TRAVEL_INVALID_INPUT_CODE,",
        "TIME_TRAVEL_COMMIT_FAILED_CODE, TIME_TRAVEL_EDITOR_CLOSED_CODE, TIME_TRAVEL_INVALID_INPUT_CODE,",
    ),
    (
        "TIME_TRAVEL_NOT_EDITABLE_CODE, TIME_TRAVEL_NO_SELECTION_CODE,",
        "TIME_TRAVEL_NOT_EDITABLE_CODE, TIME_TRAVEL_NOT_WITHDRAWABLE_CODE, TIME_TRAVEL_NO_SELECTION_CODE,",
    ),
    (
        """    NameInvalid,
    SchemaUnavailable,
}
""",
        """    NameInvalid,
    SchemaUnavailable,
    NotWithdrawable,
    EditorClosed,
}
""",
    ),
    (
        """            Self::SchemaUnavailable => TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE,
        }
""",
        """            Self::SchemaUnavailable => TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE,
            Self::NotWithdrawable => TIME_TRAVEL_NOT_WITHDRAWABLE_CODE,
            Self::EditorClosed => TIME_TRAVEL_EDITOR_CLOSED_CODE,
        }
""",
    ),
    (
        """/// ✏️ The mutation whose inputs are being drafted: its identity, label, input descriptors and the validator of its
/// payload schema, the current draft payload and the draft's own outcome against the state before it. The operation
/// whose kind the draft rebuilds lives with the typed owners of the store it belongs to ([`TimeTravelStoreState`]).
pub struct TimeTravelEditor {
    pub target: MutationId,
    pub position: u32,
    pub op_index: u32,
    pub label: LocalizedLabel,
    schema: &'static str,
    pub inputs: Vec<ActionArgDef>,
    pub inputs_refused: Option<InputSchemaError>,
    validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
    pub value: DslValue,
    pub withdrawn: bool,
    pub outcome: Vec<protocol::MutationMessage>,
    pub refused: Option<(String, String)>,
}
""",
        """/// ✏️ The mutation whose inputs are being drafted: its identity, label, input descriptors and the validator of its
/// payload schema, the current draft payload and the draft's own outcome against the state before it. The operation
/// whose kind the draft rebuilds lives with the typed owners of the store it belongs to ([`TimeTravelStoreState`]).
/// A mutation without editable inputs — opened by a history row's Withdraw (design §22.1) — has no `schema` and no
/// inputs: its editor admits no draft but the withdrawal. Whether the draft is withdrawn is the session's pending draft.
pub struct TimeTravelEditor {
    pub target: MutationId,
    pub position: u32,
    pub op_index: u32,
    pub label: LocalizedLabel,
    schema: Option<&'static str>,
    pub inputs: Vec<ActionArgDef>,
    pub inputs_refused: Option<InputSchemaError>,
    validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
    pub value: DslValue,
    pub outcome: Vec<protocol::MutationMessage>,
    pub refused: Option<(String, String)>,
}
""",
    ),
    (
        """        let mut edited: BTreeSet<String> = session.accepted.iter().map(|draft| draft.target.mutation.0.clone()).collect();
""",
        """        let accepted: BTreeSet<String> = session.accepted.iter().map(|draft| draft.target.mutation.0.clone()).collect();
        let mut edited = accepted.clone();
""",
    ),
    (
        """            rows: time_travel_input_rows(&editor.inputs, &editor.value),
            inputs_refused: editor.inputs_refused.as_ref().map(|error| error.detail.clone()),
            withdrawn: editor.withdrawn,
""",
        """            rows: time_travel_input_rows(&editor.inputs, &editor.value),
            editable: editor.schema.is_some(),
            inputs_refused: editor.inputs_refused.as_ref().map(|error| error.detail.clone()),
            withdrawn: session.pending.as_ref().is_some_and(|pending| pending.replacement == protocol::InputReplacement::Withdrawn),
""",
    ),
    (
        """            outcomes,
            edited,
        })
    }
}
//#endregion 🔖️Ledger
""",
        """            outcomes,
            edited,
            accepted,
        })
    }
}
//#endregion 🔖️Ledger
""",
    ),
    (
        """    Open { target: MutationId, current: Option<protocol::InputReplacement> },
""",
        """    Open { target: MutationId, current: Option<protocol::InputReplacement>, withdraw: bool },
""",
    ),
    (
        """            TimeTravelStoreCommand::Open { target, current } => TimeTravelStoreOutput::Opened(self.open(store, &target, current.as_ref())),
""",
        """            TimeTravelStoreCommand::Open { target, current, withdraw } => TimeTravelStoreOutput::Opened(self.open(store, &target, current.as_ref(), withdraw)),
""",
    ),
    (
        """    /// ✏️ The editor of `target` and its effective input before the session (its payload schema, input descriptors,
    /// validator and the value it starts from: `current`, its accepted draft, else its input); its kind is staged until
    /// the session adopts it. Refused for an unknown operation and for one whose inputs cannot be edited.
    fn open(&mut self, store: &ArtifactStore<P, Mu>, target: &MutationId, current: Option<&protocol::InputReplacement>) -> Result<(TimeTravelEditor, protocol::InputReplacement), TimeTravelActionRefusal> {
        let ops = store.mutation_ops().map_err(|_| TimeTravelActionRefusal::UnknownMutation)?;
        let row = ops.iter().find(|row| row.mutation_id == *target).ok_or(TimeTravelActionRefusal::UnknownMutation)?;
        let schema_id = store.envelope().schema.clone();
        let original = match row.supersession {
            Some(supersession) => supersession.replacement.clone(),
            None => protocol::InputReplacement::Input { schema: schema_id, payload: <Mu as ::protocol::OpBinary>::encode_op(row.operation).map_err(|_| TimeTravelActionRefusal::NotEditable)? },
        };
        let decode = |replacement: &protocol::InputReplacement| match replacement {
            protocol::InputReplacement::Input { payload, .. } => <Mu as ::protocol::OpBinary>::decode_op(payload).ok(),
            protocol::InputReplacement::Withdrawn => None,
        };
        let kind = decode(&original).unwrap_or_else(|| row.operation.clone());
        if kind.may_emit_foreign_steps() || kind.input_schema().is_none() {
            kind.retire_cold();
            return Err(TimeTravelActionRefusal::NotEditable);
        }
        let current = current.unwrap_or(&original);
        let (value, withdrawn) = match decode(current) {
            Some(op) => {
                let value = op.payload_value();
                op.retire_cold();
                (value, false)
            }
            None => (kind.payload_value(), *current == protocol::InputReplacement::Withdrawn),
        };
        let schema = kind.input_schema().expect("an editable operation declares its input schema");
        let (inputs, inputs_refused) = match mutation_input_defs(schema, &registered_input_schema_document) {
            Ok(inputs) => (inputs, None),
            Err(error) => (Vec::new(), Some(error)),
        };
        let documents = time_travel_schema_documents(schema);
        let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema, &documents.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|error| error.to_string());
        let editor = TimeTravelEditor {
            target: target.clone(),
            position: u32::try_from(row.position).unwrap_or(u32::MAX),
            op_index: row.op_index,
            label: (self.label_of)(&kind),
            schema,
            inputs,
            inputs_refused,
            validator,
            value,
            withdrawn,
            outcome: Vec::new(),
            refused: None,
        };
""",
        """    /// ✏️ The editor of `target` and its effective input before the session (its payload schema, input descriptors,
    /// validator and the value it starts from: `current`, its accepted draft, else its input); its kind is staged until
    /// the session adopts it. Refused for an unknown operation and for one whose inputs cannot be edited: it plans foreign
    /// steps, declares no input schema, or its schema describes no input the editor shows as a row over that value (design
    /// §22.20: an editor with zero rows never opens; a schema that cannot be read still opens, naming why) — unless the
    /// editor opens to `withdraw` it (a history row's Withdraw, design §22.1): then the store's supersede law decides
    /// ([`time_travel_admits_withdrawal`]), and an operation without editable inputs opens on an editor without inputs.
    fn open(&mut self, store: &ArtifactStore<P, Mu>, target: &MutationId, current: Option<&protocol::InputReplacement>, withdraw: bool) -> Result<(TimeTravelEditor, protocol::InputReplacement), TimeTravelActionRefusal> {
        let refusal = if withdraw { TimeTravelActionRefusal::NotWithdrawable } else { TimeTravelActionRefusal::NotEditable };
        let ops = store.mutation_ops().map_err(|_| TimeTravelActionRefusal::UnknownMutation)?;
        let row = ops.iter().find(|row| row.mutation_id == *target).ok_or(TimeTravelActionRefusal::UnknownMutation)?;
        let schema_id = store.envelope().schema.clone();
        let original = match row.supersession {
            Some(supersession) => supersession.replacement.clone(),
            None => protocol::InputReplacement::Input { schema: schema_id, payload: <Mu as ::protocol::OpBinary>::encode_op(row.operation).map_err(|_| refusal)? },
        };
        let decode = |replacement: &protocol::InputReplacement| match replacement {
            protocol::InputReplacement::Input { payload, .. } => <Mu as ::protocol::OpBinary>::decode_op(payload).ok(),
            protocol::InputReplacement::Withdrawn => None,
        };
        let kind = decode(&original).unwrap_or_else(|| row.operation.clone());
        let value = match decode(current.unwrap_or(&original)) {
            Some(op) => {
                let value = op.payload_value();
                op.retire_cold();
                value
            }
            None => kind.payload_value(),
        };
        let read = kind.input_schema().filter(|_| !kind.may_emit_foreign_steps()).map(|schema| (schema, mutation_input_defs(schema, &registered_input_schema_document)));
        let read = read.filter(|(_, inputs)| inputs.as_ref().map_or(true, |inputs| !time_travel_input_rows(inputs, &value).is_empty()));
        let admitted = if withdraw { time_travel_admits_withdrawal::<P, Mu>(row.operation) } else { read.is_some() };
        if !admitted {
            kind.retire_cold();
            return Err(refusal);
        }
        let (schema, inputs, inputs_refused, validator) = match read {
            Some((schema, read)) => {
                let (inputs, inputs_refused) = match read {
                    Ok(inputs) => (inputs, None),
                    Err(error) => (Vec::new(), Some(error)),
                };
                let documents = time_travel_schema_documents(schema);
                (Some(schema), inputs, inputs_refused, semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema, &documents.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|error| error.to_string()))
            }
            None => (None, Vec::new(), None, Err(TimeTravelLabel::RefusalNotEditable.en().to_string())),
        };
        let editor = TimeTravelEditor {
            target: target.clone(),
            position: u32::try_from(row.position).unwrap_or(u32::MAX),
            op_index: row.op_index,
            label: (self.label_of)(&kind),
            schema,
            inputs,
            inputs_refused,
            validator,
            value,
            outcome: Vec::new(),
            refused: None,
        };
""",
    ),
    (
        """/// ✏️ The draft editor as the panel renders it: its rows ([`time_travel_input_rows`]) over the current draft, and the
/// entity label of every id its reference rows name, as the app reads them in the previewed document
/// (`ArtifactApp::entity_label`; an id without one shows as itself).
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelEditorPanel {
    pub target: String,
    pub label: LocalizedLabel,
    pub rows: Vec<TimeTravelInputRow>,
""",
        """/// ✏️ The draft editor as the panel renders it: its rows ([`time_travel_input_rows`]) over the current draft, whether
/// the mutation's inputs can be edited at all (`editable`: an editor a row's Withdraw opened on a mutation without them
/// holds none), and the entity label of every id its reference rows name, as the app reads them in the previewed
/// document (`ArtifactApp::entity_label`; an id without one shows as itself).
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelEditorPanel {
    pub target: String,
    pub label: LocalizedLabel,
    pub rows: Vec<TimeTravelInputRow>,
    pub editable: bool,
""",
    ),
    (
        """/// edits (`None` for the document's own), which mutations are pending (downstream of the edited one in that store while
/// editing) or edited, the replay outcomes that override the durable ones, why finalizing is refused, the first
/// blocking mutation, and the draft editor.
""",
        """/// edits (`None` for the document's own), which mutations are pending (downstream of the edited one in that store while
/// editing), edited (a draft, accepted or pending) or `accepted` (an accepted draft a row's Restore takes back), the
/// replay outcomes that override the durable ones, why finalizing is refused, the first blocking mutation, and the
/// draft editor.
""",
    ),
    (
        """    pub outcomes: BTreeMap<String, protocol::MutationReplayOutcome>,
    pub edited: BTreeSet<String>,
}
""",
        """    pub outcomes: BTreeMap<String, protocol::MutationReplayOutcome>,
    pub edited: BTreeSet<String>,
    pub accepted: BTreeSet<String>,
}

/// 🚥️ What the action controls of one mutation row answer in the open session and instance: why Edit (`begin`) and
/// Withdraw are refused (`None`: offered), whether the mutation holds an accepted draft — its row then offers Restore
/// in Withdraw's place — and why Restore is refused.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MutationRowRefusals {
    pub begin: Option<TimeTravelLabel>,
    pub withdraw: Option<TimeTravelLabel>,
    pub accepted: bool,
    pub restore: Option<TimeTravelLabel>,
}

/// 🛃️ The [`MutationRowRefusals`] of the mutation `mutation_id` held by `store` (`None`: the document's own): a session
/// on another store, or a `busy` instance, refuses Edit and Withdraw as busy; the session's own `Begin` law refuses both
/// (`TimeTravelSession::begin_refusal`) — except Withdraw on the mutation being edited, which is the editor's own; an
/// accepted draft is restored while reviewing (`TimeTravelSession::restore_refusal`).
pub fn mutation_row_refusals(panel: Option<&TimeTravelPanel>, busy: bool, mutation_id: &str, store: Option<&str>) -> MutationRowRefusals {
    let session = panel.filter(|panel| panel.store.as_deref() == store);
    let begin = match (panel, session) {
        (Some(_), None) => Some(TimeTravelLabel::RefusalBusy),
        (_, Some(session)) => session.begin_refusal.map(TimeTravelRefusal::label),
        (None, None) => None,
    }
    .or_else(|| busy.then_some(TimeTravelLabel::RefusalBusy));
    let edited = session.is_some_and(|session| session.stage == TimeTravelStage::Editing && session.status.target.as_deref() == Some(mutation_id));
    MutationRowRefusals {
        begin,
        withdraw: if edited { None } else { begin },
        accepted: session.is_some_and(|session| session.accepted.contains(mutation_id)),
        restore: session.and_then(|session| (session.stage != TimeTravelStage::Reviewing).then_some(TimeTravelLabel::RefusalIllegal)),
    }
}
""",
    ),
    (
        """/// 🧾️ One mutation row on the history wire: the durable row with the session overlay (replay outcome, pending,
/// edited, introduced) when a history edit is open. `introduced` marks a replay outcome carrying a message (level and
/// code) the durable pre-edit outcome of the same mutation does not ([`history_outcome_introduced`]).
""",
        """/// 🧾️ One mutation row on the history wire: the durable row with the session overlay (replay outcome, pending,
/// edited, introduced) when a history edit is open. `introduced` marks a replay outcome carrying a message (level and
/// code) the durable pre-edit outcome of the same mutation does not ([`history_outcome_introduced`]); a mutation the
/// session's replay withdrew is no longer `withdrawable`.
""",
    ),
    (
        """        editable: view.editable,
        pending,
""",
        """        editable: view.editable,
        withdrawable: view.withdrawable && !withdrawn,
        pending,
""",
    ),
    (
        """/// 🏷️ The history label of one document operation in every shell locale: its leaf's `SemanticMutation::label` — never the
/// operation's text line (design §16.2).
""",
        """/// 🪨️ Whether the store's supersede law admits withdrawing `operation` (`store::admit_replacement`, the one law every
/// authoring, ingest, load and fold site reads): what a mutation row offers its Withdraw by and what a row's Withdraw
/// opens by (design §22.1). A withdrawal names no schema, so the law is asked with none.
pub(crate) fn time_travel_admits_withdrawal<P, Mu>(operation: &Mu) -> bool
where
    Mu: ::protocol::Mutation<P> + ::protocol::OpBinary,
{
    store::admit_replacement::<P, Mu>(operation, &protocol::InputReplacement::Withdrawn, "").is_ok()
}

thread_local! {
    /// 🫙️ Whether a payload schema (by the address and length of its static text) describes an input an editor shows as a
    /// row ([`time_travel_schema_shows_inputs`]): read once per schema, since every mutation row asks.
    static TIME_TRAVEL_SCHEMA_SHOWS_INPUTS: std::cell::RefCell<HashMap<(usize, usize), bool>> = std::cell::RefCell::new(HashMap::new());
}

/// 👁️ Whether the payload schema `schema` describes at least one input a draft editor shows as a row (design §22.20: a
/// mutation whose inputs are all hidden is not editable, it keeps Withdraw): an input that is not hidden — an object one
/// through its fields ([`time_travel_input_rows`]). A schema that cannot be read answers `true` and is not remembered:
/// its editor opens naming why (the referenced documents may register later).
pub(crate) fn time_travel_schema_shows_inputs(schema: &'static str) -> bool {
    fn shown(input: &ActionArgDef) -> bool {
        input.presentation != Some(ArgPresentation::Hidden) && !matches!(&input.schema, ArgSchema::Object { fields } if !fields.is_empty() && !fields.iter().any(shown))
    }
    let key = (schema.as_ptr() as usize, schema.len());
    if let Some(known) = TIME_TRAVEL_SCHEMA_SHOWS_INPUTS.with(|cache| cache.borrow().get(&key).copied()) {
        return known;
    }
    let Ok(inputs) = mutation_input_defs(schema, &registered_input_schema_document) else { return true };
    let shows = inputs.iter().any(shown);
    TIME_TRAVEL_SCHEMA_SHOWS_INPUTS.with(|cache| cache.borrow_mut().insert(key, shows));
    shows
}

/// 🏷️ The history label of one document operation in every shell locale: its leaf's `SemanticMutation::label` — never the
/// operation's text line (design §16.2).
""",
    ),
    (
        """/// (a label of `labelled`, the previous projection, reused for an operation that was not superseded) with its durable
/// `outcome`; `superseded`/`withdrawn` read the store's effective supersession.
""",
        """/// (a label of `labelled`, the previous projection, reused for an operation that was not superseded) with its durable
/// `outcome`; `superseded`/`withdrawn` read the store's effective supersession; `editable` needs a payload schema that
/// shows an input ([`time_travel_schema_shows_inputs`], design §22.20) and no foreign steps; `withdrawable` asks the
/// store's supersede law for an operation that is not withdrawn yet ([`time_travel_admits_withdrawal`]); a `viewer`
/// edits and withdraws nothing.
""",
    ),
    (
        """    let editable = !viewer && !shown.may_emit_foreign_steps() && shown.input_schema().is_some();
""",
        """    let editable = !viewer && !shown.may_emit_foreign_steps() && shown.input_schema().is_some_and(time_travel_schema_shows_inputs);
""",
    ),
    (
        """    if let Some(effective) = effective {
        effective.retire_cold();
    }
    MutationView {
        mutation_id: id.to_string(),
        position: u32::try_from(op.position).unwrap_or(u32::MAX),
        op_index: op.op_index,
        label,
        worst: outcome.and_then(|outcome| outcome.worst),
        messages: outcome.map(|outcome| outcome.messages.clone()).unwrap_or_default(),
        superseded,
        withdrawn: op.supersession.is_some_and(|supersession| supersession.replacement == protocol::InputReplacement::Withdrawn),
        editable,
        store: store.map(str::to_string),
    }
""",
        """    if let Some(effective) = effective {
        effective.retire_cold();
    }
    let withdrawn = op.supersession.is_some_and(|supersession| supersession.replacement == protocol::InputReplacement::Withdrawn);
    MutationView {
        mutation_id: id.to_string(),
        position: u32::try_from(op.position).unwrap_or(u32::MAX),
        op_index: op.op_index,
        label,
        worst: outcome.and_then(|outcome| outcome.worst),
        messages: outcome.map(|outcome| outcome.messages.clone()).unwrap_or_default(),
        superseded,
        withdrawn,
        editable,
        withdrawable: !viewer && !withdrawn && time_travel_admits_withdrawal::<P, Mu>(op.operation),
        store: store.map(str::to_string),
    }
""",
    ),
    (
        """            HISTORY_EDIT_BEGIN_ACTION_ID => return self.begin_time_travel(args, meta, effects).await,
""",
        """            HISTORY_EDIT_BEGIN_ACTION_ID => return self.begin_time_travel(args, false, meta, effects).await,
""",
    ),
    (
        """            HISTORY_EDIT_WITHDRAW_ACTION_ID => TimeTravelEvent::Withdraw { generation },
            HISTORY_EDIT_ACCEPT_ACTION_ID => TimeTravelEvent::Accept { generation },
            HISTORY_EDIT_DISCARD_ACTION_ID => TimeTravelEvent::Discard { generation },
""",
        """            HISTORY_EDIT_WITHDRAW_ACTION_ID if time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID).is_some() => return self.begin_time_travel(args, true, meta, effects).await,
            HISTORY_EDIT_WITHDRAW_ACTION_ID => TimeTravelEvent::Withdraw { generation },
            HISTORY_EDIT_ACCEPT_ACTION_ID => TimeTravelEvent::Accept { generation },
            HISTORY_EDIT_DISCARD_ACTION_ID => TimeTravelEvent::Discard { generation },
            HISTORY_EDIT_RESTORE_ACTION_ID => {
                let Some(mutation) = time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation)) };
                if self.time_travel.member_store().as_deref() != time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE).filter(|store| !store.is_empty()) {
                    return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)));
                }
                TimeTravelEvent::Restore { generation, target: MutationId(mutation.to_string()) }
            }
""",
    ),
    (
        """        let preview_changes = matches!(event, TimeTravelEvent::Accept { .. } | TimeTravelEvent::Discard { .. } | TimeTravelEvent::Exit | TimeTravelEvent::Back { .. });
""",
        """        let preview_changes = matches!(event, TimeTravelEvent::Accept { .. } | TimeTravelEvent::Discard { .. } | TimeTravelEvent::Restore { .. } | TimeTravelEvent::Exit | TimeTravelEvent::Back { .. });
""",
    ),
    (
        """    /// ✏️ Opens (or retargets) the session on `mutationId` of the store `store` names (`<slot>/<childId>`, a composed
    /// member, design §12; absent: the document's own): refused while a mutating tool run or an agent transaction holds
    /// this instance, for a store the instance does not compose, for a session open on another store, for an unknown
    /// operation, and for one whose inputs cannot be edited. Opening a session delivers [`HostEvent::TimeTravelFrozen`] to
    /// every open window, so an open gesture there ends first.
    async fn begin_time_travel(&mut self, args: Option<&DslValue>, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(mutation) = time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation)) };
        if self.time_travel_busy() {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy));
        }
        let store = time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE).filter(|store| !store.is_empty());
""",
        """    /// ✏️ Opens (or retargets) the session on `mutationId` of the store `store` names (`<slot>/<childId>`, a composed
    /// member, design §12; absent: the document's own): refused while a mutating tool run or an agent transaction holds
    /// this instance, for a store the instance does not compose, for a session open on another store, for an unknown
    /// operation, and for one whose inputs cannot be edited. Opening a session delivers [`HostEvent::TimeTravelFrozen`] to
    /// every open window, so an open gesture there ends first. With `withdraw` (a history row's Withdraw, design §22.1)
    /// the session opens on a withdrawn draft and needs no editable inputs — refused only where the store's supersede law
    /// lets no withdrawal through; on the mutation already being edited it is the editor's own Withdraw.
    async fn begin_time_travel(&mut self, args: Option<&DslValue>, withdraw: bool, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(mutation) = time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation)) };
        let store = time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE).filter(|store| !store.is_empty());
        let session = &self.time_travel.session;
        if withdraw && session.stage == TimeTravelStage::Editing && self.time_travel.member_store().as_deref() == store && session.pending.as_ref().is_some_and(|pending| pending.target.mutation.0 == mutation) {
            let generation = time_travel_arg_generation(args).unwrap_or(session.generation);
            return self.apply_time_travel_event(TimeTravelEvent::Withdraw { generation }, Some(meta), effects).await;
        }
        if self.time_travel_busy() {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy));
        }
""",
    ),
    (
        """        let target = MutationId(mutation.to_string());
        let current = self.time_travel.session.accepted_draft(&target).map(|draft| draft.replacement.clone());
        let opened = match self.time_travel_run(TimeTravelStoreCommand::Open { target: target.clone(), current }).await? {
""",
        """        let target = MutationId(mutation.to_string());
        let current = if withdraw { Some(protocol::InputReplacement::Withdrawn) } else { self.time_travel.session.accepted_draft(&target).map(|draft| draft.replacement.clone()) };
        let opened = match self.time_travel_run(TimeTravelStoreCommand::Open { target: target.clone(), current, withdraw }).await? {
""",
    ),
    (
        """        let event = TimeTravelEvent::Begin { target: TimeTravelTarget { mutation: target, position: editor.position }, original };
        match self.time_travel.session.apply(event) {
""",
        """        let target = TimeTravelTarget { mutation: target, position: editor.position };
        let event = if withdraw { TimeTravelEvent::BeginWithdrawn { target, original } } else { TimeTravelEvent::Begin { target, original } };
        match self.time_travel.session.apply(event) {
""",
    ),
    (
        """        let Some(editor) = self.time_travel.editor.as_mut().filter(|_| self.time_travel.session.stage == TimeTravelStage::Editing) else {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)));
        };
        let unavailable = match (&editor.validator, &editor.inputs_refused) {
""",
        """        let Some(editor) = self.time_travel.editor.as_mut().filter(|_| self.time_travel.session.stage == TimeTravelStage::Editing) else {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)));
        };
        let Some(schema) = editor.schema else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::NotEditable)) };
        let unavailable = match (&editor.validator, &editor.inputs_refused) {
""",
    ),
    (
        """        let verdict = mutation_input_instance(editor.schema, &registered_input_schema_document, &candidate)
""",
        """        let verdict = mutation_input_instance(schema, &registered_input_schema_document, &candidate)
""",
    ),
    (
        """            Err(reason) => {
                let editor = self.time_travel.editor.as_mut().expect("editing keeps its editor");
                let outcome = refuse(editor, at, reason);
""",
        """            Err(reason) => {
                let Some(editor) = self.time_travel.editor.as_mut() else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::EditorClosed)) };
                let outcome = refuse(editor, at, reason);
""",
    ),
    (
        """            if let Some(editor) = self.time_travel.editor.as_mut() {
                editor.value = candidate;
                editor.withdrawn = false;
                editor.refused = None;
            }
""",
        """            if let Some(editor) = self.time_travel.editor.as_mut() {
                editor.value = candidate;
                editor.refused = None;
            }
""",
    ),
    (
        """    Accept,
    Discard,
    Withdraw,
    Finalize,
""",
        """    Accept,
    Discard,
    Withdraw,
    Restore,
    Finalize,
""",
    ),
    (
        """                Self::Withdraw => "Withdraw",
                Self::Finalize => "Finalize",
""",
        """                Self::Withdraw => "Withdraw",
                Self::Restore => "Restore",
                Self::Finalize => "Finalize",
""",
    ),
    (
        """                Self::Withdraw => "Zurückziehen",
                Self::Finalize => "Abschließen",
""",
        """                Self::Withdraw => "Zurückziehen",
                Self::Restore => "Wiederherstellen",
                Self::Finalize => "Abschließen",
""",
    ),
    (
        """/// ✏️ The draft editor as two tree sections: `framework.history.editor` — the edited mutation with its draft's own
/// outcome, why no input is editable, then Accept, Discard and Withdraw — and `framework.history.editor.inputs`, one row
/// per input row of [`time_travel_input_rows`] ([`time_travel_input_row`]). The inputs section is a tree window over
/// every row, so a payload of any size stays reachable: the host streams the slice it scrolls to.
""",
        """/// ✏️ The draft editor as two tree sections: `framework.history.editor` — the edited mutation with its draft's own
/// outcome, why no input is editable (a mutation without editable inputs reads the framework's own words, a payload
/// schema that describes none its reason), then Accept, Discard and Withdraw — and `framework.history.editor.inputs`,
/// one row per input row of [`time_travel_input_rows`] ([`time_travel_input_row`]). The inputs section is a tree window
/// over every row, so a payload of any size stays reachable: the host streams the slice it scrolls to.
""",
    ),
    (
        """    if let Some(reason) = editor.inputs_refused.as_deref() {
        let node = ui::tree_item(Label(UiText::clipped(HistoryPanelText::NoInputs.text(locale))))
            .description(UiText::clipped(reason))
""",
        """    let uneditable = (!editor.editable).then(|| TimeTravelLabel::RefusalNotEditable.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
    if let Some(reason) = uneditable.as_deref().or(editor.inputs_refused.as_deref()) {
        let node = ui::tree_item(Label(UiText::clipped(HistoryPanelText::NoInputs.text(locale))))
            .description(UiText::clipped(reason))
""",
    ),
]

PLG_RS = [
    (
        """    /// ✏️ One applied mutation of a history row: its replica-independent id, applied position and index in its edit,
    /// localized label (of its effective input), durable outcome and supersession state, and whether history editing
    /// may replace its inputs.
""",
        """    /// ✏️ One applied mutation of a history row: its replica-independent id, applied position and index in its edit,
    /// localized label (of its effective input), durable outcome and supersession state, and whether history editing
    /// may replace its inputs (`editable`) or withdraw it (`withdrawable`: the store's supersede law admits it, it is not
    /// withdrawn yet and this is no viewer).
""",
    ),
    (
        """        pub withdrawn: bool,
        pub editable: bool,
        pub store: Option<String>,
    }
""",
        """        pub withdrawn: bool,
        pub editable: bool,
        pub withdrawable: bool,
        pub store: Option<String>,
    }
""",
    ),
    (
        """    /// ✏️ A row with applied mutations is a tree window over ALL of them (gap N1): its projected ones first (the flagged
    /// ones leading), then every other operation of its edit in op order, read from `pages` for the slice a host window
    /// opened past the projection ([`time_travel::history_row_mutation_extent`]). Each child carries its label, a
    /// description naming its state and severity in words (colour is never the only carrier), tone and icon by severity,
    /// and an Edit row action (`historyEditBegin{mutationId}`, also the row's activation) when it is editable and this is
    /// not a viewer — disabled, naming why, wherever the session would refuse `Begin` (gap N15) or the instance is `busy`
    /// (an open tool transaction or run, a history step replaying, an undo being authored: audit W2A-9).
""",
        """    /// ✏️ A row with applied mutations is a tree window over ALL of them (gap N1): its projected ones first — the ones whose
    /// outcome blocks finalizing lead (so the review's next problem is inside the first slice a host materialises, design
    /// §22.2), then the other flagged ones — then every other operation of its edit in op order, read from `pages` for the
    /// slice a host window opened past the projection ([`time_travel::history_row_mutation_extent`]). Each child carries its
    /// label, a description naming its state and severity in words (colour is never the only carrier), tone and icon by
    /// severity, and its row actions ([`history_panel_mutation_row`]): Edit, then Withdraw (design §22.1) — or Restore, on a
    /// mutation holding an accepted draft — each disabled, naming why, wherever a viewer, the mutation, the store, the
    /// session or the instance refuses it ([`time_travel::mutation_row_refusals`]: gap N15, audit W2A-9, design §22.20).
""",
    ),
    (
        """            let mut shown: Vec<&semio_framework::kernel::HistoryMutationEntry> = mutations.iter().filter(|mutation| mutation.worst.is_some() || mutation.edited || mutation.superseded).collect();
            shown.extend(mutations.iter().filter(|mutation| mutation.worst.is_none() && !mutation.edited && !mutation.superseded));
""",
        """            let blocking = |mutation: &semio_framework::kernel::HistoryMutationEntry| mutation.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst));
            let flagged = |mutation: &semio_framework::kernel::HistoryMutationEntry| mutation.worst.is_some() || mutation.edited || mutation.superseded;
            let mut shown: Vec<&semio_framework::kernel::HistoryMutationEntry> = mutations.iter().filter(|mutation| blocking(*mutation)).collect();
            shown.extend(mutations.iter().filter(|mutation| flagged(*mutation) && !blocking(*mutation)));
            shown.extend(mutations.iter().filter(|mutation| !flagged(*mutation)));
""",
    ),
    (
        """            let begin_refusal = time_travel.and_then(|panel| panel.begin_refusal).map(semio_framework_time_travel::TimeTravelRefusal::label).or_else(|| busy.then_some(semio_framework_time_travel::TimeTravelLabel::RefusalBusy));
""",
        "",
    ),
    (
        """                history_panel_mutation_row(mutation, controller_id, locale, read_only, begin_refusal)
""",
        """                history_panel_mutation_row(mutation, controller_id, locale, read_only, time_travel::mutation_row_refusals(time_travel, busy, &mutation.mutation_id, mutation.store.as_deref()))
""",
    ),
    (
        """    /// ✏️ One mutation child of a history row: label, state-and-severity description in words, tone and icon by
    /// severity, and Edit (`historyEditBegin{mutationId, store?}`, also its activation) when editable and not a viewer —
    /// `store` names the composed member store a child mutation lives in (design §12). While the session would refuse
    /// `Begin` (`begin_refusal`: replaying, choosing, finalizing, or a changed draft still open) the Edit action is disabled
    /// because of that refusal (`RowAction::disabled_because`, announced as its description) and the row's activation fires
    /// nothing (gap N15).
    fn history_panel_mutation_row(mutation: &semio_framework::kernel::HistoryMutationEntry, controller_id: &str, locale: Locale, read_only: bool, begin_refusal: Option<semio_framework_time_travel::TimeTravelLabel>) -> UiAssemblyResult<BuiltNode> {
""",
        """    /// ✏️ One mutation child of a history row: label, state-and-severity description in words, tone and icon by
    /// severity, and its row actions on the one row target `{mutationId, store?}` — `store` names the composed member
    /// store a child mutation lives in (design §12): Edit (`historyEditBegin`, also the row's activation), then Withdraw
    /// (`historyEditWithdraw`, design §22.1) on every mutation that is not withdrawn — or, on a mutation holding an
    /// accepted draft of the open session, Restore (`historyEditRestore`) in its place. An action a viewer, a mutation
    /// without editable inputs (design §22.20), the store's supersede law, the session or the instance refuses is disabled
    /// because of that refusal (`RowAction::disabled_because`, announced as its description; `refusals`), and a refused
    /// Edit activates nothing (gap N15).
    fn history_panel_mutation_row(mutation: &semio_framework::kernel::HistoryMutationEntry, controller_id: &str, locale: Locale, read_only: bool, refusals: time_travel::MutationRowRefusals) -> UiAssemblyResult<BuiltNode> {
""",
    ),
    (
        """        if mutation.editable && !read_only {
            let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("history-panel.mutation-args"))?;
            args.push(semio_framework::HISTORY_EDIT_ARG_MUTATION_ID.to_owned(), UiValue::Text(UiText::clipped(&mutation.mutation_id))).map_err(|_| ui_assembly_error("history-panel.mutation-args"))?;
            if let Some(store) = mutation.store.as_deref() {
                args.push(semio_framework::HISTORY_EDIT_ARG_STORE.to_owned(), UiValue::Text(UiText::clipped(store))).map_err(|_| ui_assembly_error("history-panel.mutation-args"))?;
            }
            let edit_text = time_travel::HistoryPanelText::Edit.text(locale);
            let (edit, activation) = match begin_refusal {
                None => (row_action(IconName::Edit.as_str(), edit_text, semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, RowActionPlacement::Row)?, Some(semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID)),
                Some(refusal) => {
                    let reason = refusal.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
                    (row_action(IconName::Edit.as_str(), edit_text, semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, RowActionPlacement::Row)?.disabled_because(ui_label(reason.as_str(), "history-panel.mutation-edit-reason")?), None)
                }
            };
            builder = builder.try_row_action(edit).map_err(|_| ui_assembly_error("history-panel.mutation-actions"))?.target(row_target(controller_id, Some(UiValue::Map(args.finish())), activation)?);
        }
        builder.try_build().map_err(|_| ui_assembly_error("history-panel.mutation-build"))
""",
        """        let action = |icon: &str, text: time_travel::HistoryPanelText, verb: &str, refusal: Option<semio_framework_time_travel::TimeTravelLabel>| -> UiAssemblyResult<RowAction> {
            let action = row_action(icon, text.text(locale), verb, RowActionPlacement::Row)?;
            let Some(refusal) = refusal else { return Ok(action) };
            let reason = refusal.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
            Ok(action.disabled_because(ui_label(reason.as_str(), "history-panel.mutation-action-reason")?))
        };
        let viewer = read_only.then_some(semio_framework_time_travel::TimeTravelLabel::RefusalReadOnly);
        let begin = viewer.or((!mutation.editable).then_some(semio_framework_time_travel::TimeTravelLabel::RefusalNotEditable)).or(refusals.begin);
        let mut actions = vec![action(IconName::Edit.as_str(), time_travel::HistoryPanelText::Edit, semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, begin)?];
        if refusals.accepted {
            actions.push(action("undo-2", time_travel::HistoryPanelText::Restore, semio_framework::HISTORY_EDIT_RESTORE_ACTION_ID, refusals.restore)?);
        } else if !mutation.withdrawn {
            let withdraw = viewer.or((!mutation.withdrawable).then_some(semio_framework_time_travel::TimeTravelLabel::RefusalNotWithdrawable)).or(refusals.withdraw);
            actions.push(action("eye-off", time_travel::HistoryPanelText::Withdraw, semio_framework::HISTORY_EDIT_WITHDRAW_ACTION_ID, withdraw)?);
        }
        let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("history-panel.mutation-args"))?;
        args.push(semio_framework::HISTORY_EDIT_ARG_MUTATION_ID.to_owned(), UiValue::Text(UiText::clipped(&mutation.mutation_id))).map_err(|_| ui_assembly_error("history-panel.mutation-args"))?;
        if let Some(store) = mutation.store.as_deref() {
            args.push(semio_framework::HISTORY_EDIT_ARG_STORE.to_owned(), UiValue::Text(UiText::clipped(store))).map_err(|_| ui_assembly_error("history-panel.mutation-args"))?;
        }
        for action in actions {
            builder = builder.try_row_action(action).map_err(|_| ui_assembly_error("history-panel.mutation-actions"))?;
        }
        let activation = begin.is_none().then_some(semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID);
        builder = builder.target(row_target(controller_id, Some(UiValue::Map(args.finish())), activation)?);
        builder.try_build().map_err(|_| ui_assembly_error("history-panel.mutation-build"))
""",
    ),
]

PLUGIN_FIXTURE = [
    (
        """`begin` names a seeded edit by index;""",
        """`begin` names a seeded edit by index; `withdrawRow` is that edit's history row Withdraw (`historyEditWithdraw{mutationId}`: it opens or retargets the session on a withdrawn draft) and `restore` its row Restore (`historyEditRestore{mutationId}`: the accepted draft is taken back); a status names the seeded edit of the review's `nextProblem` exactly while it blocks;""",
    ),
    (
        """      "status": { "stage": "reviewing", "blocking": true, "acceptedCount": 1 },
      "body": "count=5 label=a",
      "finalizeRefusal": "timeTravel.blocked"
    },""",
        """      "status": { "stage": "reviewing", "blocking": true, "acceptedCount": 1, "nextProblem": 2 },
      "body": "count=5 label=a",
      "finalizeRefusal": "timeTravel.blocked"
    },""",
    ),
    (
        """    {
      "id": "discarding-the-only-draft-exits",
      "steps": [{ "begin": 3 }, { "input": { "path": "/value", "value": 8 } }, { "discard": null }],
      "status": null,
      "body": "count=5 label=a"
    }
  ],""",
        """    {
      "id": "discarding-the-only-draft-exits",
      "steps": [{ "begin": 3 }, { "input": { "path": "/value", "value": 8 } }, { "discard": null }],
      "status": null,
      "body": "count=5 label=a"
    },
    {
      "id": "a-row-withdraw-opens-the-session-on-a-withdrawn-draft-and-nothing-downstream",
      "steps": [{ "withdrawRow": 1 }],
      "status": { "stage": "editing", "blocking": false, "acceptedCount": 0 },
      "body": "count=1 label=",
      "pending": [2, 3]
    },
    {
      "id": "a-row-withdraw-of-the-failing-mutation-unblocks-finalizing",
      "steps": [{ "begin": 2 }, { "input": { "path": "/children", "value": ["not a uri"] } }, { "accept": null }, { "replay": "fatal" }, { "withdrawRow": 2 }, { "accept": null }, { "replay": "clean" }],
      "status": { "stage": "reviewing", "blocking": false, "acceptedCount": 1 },
      "body": "count=5 label=a",
      "finalizeRefusal": null,
      "withdrawn": [2]
    },
    {
      "id": "an-accepted-row-withdrawal-folds-the-mutation-as-a-no-op",
      "steps": [{ "withdrawRow": 3 }, { "accept": null }, { "replay": "clean" }],
      "status": { "stage": "reviewing", "blocking": false, "acceptedCount": 1 },
      "body": "count=1 label=a",
      "finalizeRefusal": null,
      "withdrawn": [3]
    },
    {
      "id": "an-overwritten-row-withdrawal-is-a-withdrawn-supersede",
      "steps": [{ "withdrawRow": 3 }, { "accept": null }, { "replay": "clean" }, { "finalize": null }, { "commit": { "choice": "overwrite" } }],
      "status": null,
      "body": "count=1 label=a",
      "superseded": [3],
      "withdrawn": [3]
    },
    {
      "id": "restoring-one-of-two-accepted-drafts-replays-the-other",
      "steps": [
        { "begin": 1 },
        { "input": { "path": "/value", "value": "b" } },
        { "accept": null },
        { "replay": "clean" },
        { "withdrawRow": 3 },
        { "accept": null },
        { "replay": "clean" },
        { "restore": 3 },
        { "replay": "clean" }
      ],
      "status": { "stage": "reviewing", "blocking": false, "acceptedCount": 1 },
      "body": "count=5 label=b",
      "finalizeRefusal": null,
      "withdrawn": []
    },
    {
      "id": "restoring-the-only-accepted-draft-leaves-history-editing",
      "steps": [{ "withdrawRow": 3 }, { "accept": null }, { "replay": "clean" }, { "restore": 3 }],
      "status": null,
      "body": "count=5 label=a",
      "superseded": [],
      "withdrawn": []
    }
  ],""",
    ),
]

PLUGIN_FIXTURE_SCHEMA = [
    (
        """the verbs each scenario dispatches, the session status it ends in (absent = inactive),""",
        """the verbs each scenario dispatches (a history row's Withdraw and Restore included), the session status it ends in (absent = inactive; a blocking review names the seeded edit of its next problem),""",
    ),
    (
        """        "withdraw": { "type": "null" },
""",
        """        "withdraw": { "type": "null" },
        "withdrawRow": { "type": "integer", "minimum": 0 },
        "restore": { "type": "integer", "minimum": 0 },
""",
    ),
    (
        """        "acceptedCount": { "type": "integer", "minimum": 0 }
      }
    },""",
        """        "acceptedCount": { "type": "integer", "minimum": 0 },
        "nextProblem": { "type": "integer", "minimum": 0 }
      }
    },""",
    ),
]

PLUGIN_ORACLE = [
    (
        """type Scenario = Readonly<{ id: string; steps: readonly Step[]; status: Readonly<{ stage: string; blocking: boolean; acceptedCount: number }> | null; finalizeRefusal?: string | null }>;""",
        """type Scenario = Readonly<{ id: string; steps: readonly Step[]; status: Readonly<{ stage: string; blocking: boolean; acceptedCount: number; nextProblem?: number }> | null; finalizeRefusal?: string | null }>;""",
    ),
    (
        """    case "withdraw":
    case "accept":
    case "discard":
      return apply(session, { type: name, generation }, scenario);
""",
        """    case "withdraw":
    case "accept":
    case "discard":
      return apply(session, { type: name, generation }, scenario);
    case "withdrawRow": {
      const position = argument as number;
      if (session.stage === "editing" && session.pending?.target.mutation === `m${position}`) return apply(session, { type: "withdraw", generation }, scenario);
      return apply(session, { type: "beginWithdrawn", target: { mutation: `m${position}`, position }, original: input(`original-${position}`) }, scenario);
    }
    case "restore":
      return apply(session, { type: "restore", generation, target: `m${argument as number}` }, scenario);
""",
    ),
    (
        """    assert.equal(session.report !== null && replayReportBlocksFinalize(session.report), scenario.status.blocking, `${scenario.id}: blocking`);
""",
        """    assert.equal(session.report !== null && replayReportBlocksFinalize(session.report), scenario.status.blocking, `${scenario.id}: blocking`);
    const problem = session.report?.outcomes.find((outcome) => outcome.worst === "error" || outcome.worst === "fatal")?.mutationId;
    assert.equal(problem, scenario.status.nextProblem === undefined ? undefined : `m${scenario.status.nextProblem}`, `${scenario.id}: next problem`);
""",
    ),
]

PLUGIN_LAWS = [
    (
        """        "withdraw" => verb(app, fixture, "historyEditWithdraw", Vec::new()).await,
""",
        """        "withdraw" => verb(app, fixture, "historyEditWithdraw", Vec::new()).await,
        "withdrawRow" | "restore" => {
            let mutation = seeded_mutation(app, value.as_u64().expect("edit index") as usize);
            verb(app, fixture, if name == "restore" { "historyEditRestore" } else { "historyEditWithdraw" }, vec![("mutationId".into(), DslValue::String(mutation))]).await
        }
""",
    ),
    (
        """                assert_eq!(u64::from(status.accepted_count), expected["acceptedCount"].as_u64().expect("accepted"), "{id}: accepted drafts");
""",
        """                assert_eq!(u64::from(status.accepted_count), expected["acceptedCount"].as_u64().expect("accepted"), "{id}: accepted drafts");
                let problem = expected.get("nextProblem").and_then(Value::as_u64).map(|index| seeded_mutation(&app, index as usize));
                assert_eq!(status.next_problem.as_ref().map(|problem| (problem.mutation_id.clone(), problem.store.clone())), problem.map(|mutation| (mutation, None)), "{id}: the next problem");
                assert_eq!(status.next_problem.is_some(), status.blocking, "{id}: a review names its next problem exactly while it blocks");
""",
    ),
    (
        """    assert!(refused.is_some(), "an undeclared filter value is refused");
    close(&mut app);
}
//#endregion 🎚️HistoryFilter
""",
        """    assert!(refused.is_some(), "an undeclared filter value is refused");
    close(&mut app);
}
//#endregion 🎚️HistoryFilter


//#region ☑️LongOptionRows
/// ☑️ LAW (audit W1E-2, conformance `💬️row-semantics` `selectedRows`): a choice with more options than one fixed list holds is
/// windowed option rows, each a tree row holding its choose button and stating its choice as `selected` — the chosen option
/// `Some(true)`, every other `Some(false)` — so both renderers announce (`aria-selected`, the mirror's `selected`) and paint it,
/// never by its icon alone.
#[test]
fn long_option_rows_state_their_choice_as_selected() {
    let option = |index: usize| semio_framework::ActionArgOption { value: format!("o{index}"), label: LocalizedLabel::native(&format!("Option {index}"), &format!("Option {index}")) };
    let mode = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::String { options: (0..40).map(option).collect(), option_source: None, min_len: None, max_len: None, pattern: None, format: None },
        required: true,
        ..semio_framework::ActionArgDef::text("/mode", LocalizedLabel::native("Mode", "Modus"))
    };
    let value = dsl(&serde_json::json!({ "mode": "o3" }));
    let editor = TimeTravelEditorPanel { target: "m-1".into(), label: LocalizedLabel::native("Set mode", "Modus setzen"), rows: time_travel::time_travel_input_rows(&[mode], &value), editable: true, inputs_refused: None, withdrawn: false, outcome: Vec::new(), refused: None, changed: false, reference_labels: Default::default() };
    let panel = TimeTravelPanel { status: Default::default(), store: None, stage: TimeTravelStage::Editing, pending_after: None, finalize_refusal: None, review: None, rerun_refusal: None, begin_refusal: None, next_problem: None, editor: Some(editor.clone()), outcomes: Default::default(), edited: Default::default(), accepted: Default::default() };
    let sections = time_travel::time_travel_editor_sections(&panel, &editor, &TreeWindows::unhosted(), "toy", Locale::En).expect("the editor builds");
    fn option_rows(node: &BuiltNode, rows: &mut Vec<(String, Option<bool>)>) {
        if let semio_framework_ui_contract::Component::TreeItem(props) = &node.component {
            if node.key.as_str().contains(".option.") && node.key.as_str().ends_with(".row") {
                rows.push((node.key.as_str().to_string(), props.selected));
            }
        }
        node.children.iter().for_each(|child| option_rows(child, rows));
    }
    let mut rows = Vec::new();
    sections.iter().for_each(|section| option_rows(section, &mut rows));
    assert!(rows.len() >= 4 && rows.iter().any(|(_, selected)| *selected == Some(true)), "the first paint holds the chosen option among several: {rows:?}");
    for (key, selected) in &rows {
        assert_eq!(*selected, Some(key == "framework.history.editor.input.mode.option.3.row"), "{key}");
    }
}
//#endregion ☑️LongOptionRows

//#region 🚫️RowWithdraw
/// 📍️ The rendered mutation row of seeded edit `index`, its history row opened as a host window shows it.
async fn seeded_mutation_node(app: &mut ToyApp, index: usize) -> Value {
    let id = seeded_mutation(app, index);
    let seq = app.history_patch(true).await.expect("history patch").upserts.iter().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == id)).map(|entry| entry.seq).expect("the history row of the seeded edit");
    let body = render_history_window(app, seq, 0, 8).await;
    find_node(&body, &format!("framework.history.mutation.{id}")).cloned().unwrap_or_else(|| panic!("the mutation row of seeded edit {index}: {body}"))
}

/// 🎬️ The row actions of a rendered mutation row: each verb, label, whether it is disabled and why.
fn row_actions(row: &Value) -> Vec<(String, String, bool, Option<String>)> {
    let text = |value: &Value| value.as_str().map(str::to_string);
    row["component"]["rowActions"].as_array().into_iter().flatten().map(|action| (text(&action["verb"]).unwrap_or_default(), text(&action["label"]).unwrap_or_default(), action["disabled"] == Value::Bool(true), text(&action["reason"]))).collect()
}

/// ⚖️ LAW (design §22.1, coordinator decision "Restore"): every applied mutation row offers Withdraw beside Edit on the one
/// row target. A row's Withdraw opens the session on a withdrawn draft (the preview is the state before the mutation, the
/// store untouched); on the mutation being edited it stays the editor's own while every other row names why it is blocked;
/// a running replay refuses it naming why; a mutation holding an accepted draft offers Restore in Withdraw's place and is
/// no longer withdrawable on the wire; restoring the only accepted draft leaves history editing with zero trace.
#[semio_framework_async_macros::async_test]
async fn a_mutation_row_offers_withdraw_and_restore_takes_an_accepted_draft_back() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let generation = app.store.generation();
    let (first, target) = (seeded_mutation(&app, 0), seeded_mutation(&app, 3));
    let enabled = |verb: &str, label: &str| (verb.to_string(), label.to_string(), false, None);
    let idle = seeded_mutation_node(&mut app, 3).await;
    assert_eq!(row_actions(&idle), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditWithdraw", "Withdraw")], "Edit stays first, Withdraw follows: {idle}");
    assert_eq!((idle["component"]["target"]["args"]["mutationId"].as_str(), idle["component"]["target"]["activation"].as_str()), (Some(target.as_str()), Some("historyEditBegin")), "one row target for both actions: {idle}");
    let wire = seeded_mutation_row(&mut app, 3).await;
    assert!(wire.editable && wire.withdrawable && !wire.withdrawn, "the wire row is withdrawable");

    let withdrawn = app.handle_action("historyEditWithdraw", Some(&dsl(&idle["component"]["target"]["args"])), &meta(&fixture)).await.expect("the row's Withdraw dispatches");
    assert_eq!(rejected(&withdrawn), None, "{:?}", withdrawn.output);
    let session = app.time_travel.session();
    assert_eq!((session.stage, session.pending.as_ref().map(|pending| (pending.target.mutation.0.clone(), pending.replacement.clone()))), (TimeTravelStage::Editing, Some((target.clone(), protocol::InputReplacement::Withdrawn))), "the session opens on a withdrawn draft");
    assert_eq!((render_body(&mut app).await.contains("count=1 label=a"), app.store.generation()), (true, generation), "the preview is the state before the mutation; the store is untouched");
    let editing = row_actions(&seeded_mutation_node(&mut app, 3).await);
    assert_eq!((editing[0].2, editing[0].3.as_deref().is_some_and(|reason| reason.starts_with("Blocked")), &editing[1]), (true, true, &enabled("historyEditWithdraw", "Withdraw")), "the withdrawn draft blocks Edit; Withdraw stays the editor's own: {editing:?}");
    let other = row_actions(&seeded_mutation_node(&mut app, 0).await);
    assert!(other.iter().all(|action| action.2 && action.3.as_deref().is_some_and(|reason| reason.starts_with("Blocked"))), "another row names the open draft: {other:?}");
    let blocked = verb(&mut app, &fixture, "historyEditWithdraw", vec![("mutationId".into(), DslValue::String(first.clone()))]).await;
    assert_eq!(rejected(&blocked), Some("timeTravel.blocked"), "the verb refuses what the row disables");

    verb(&mut app, &fixture, "historyEditAccept", Vec::new()).await;
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let replaying = row_actions(&seeded_mutation_node(&mut app, 0).await);
    assert!(replaying.iter().all(|action| action.2 && action.3.as_deref() == Some("Not possible right now")), "a running replay disables Edit and Withdraw, naming why: {replaying:?}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    let status = app.time_travel.status().expect("a session is open");
    assert_eq!((status.accepted_count, status.blocking, status.next_problem.is_none()), (1, false, true), "the withdrawal replays clean");
    assert_eq!(row_actions(&seeded_mutation_node(&mut app, 3).await), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditRestore", "Restore")], "an accepted draft offers Restore in Withdraw's place");
    assert_eq!(row_actions(&seeded_mutation_node(&mut app, 0).await), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditWithdraw", "Withdraw")], "the review withdraws other rows again");
    let wire = seeded_mutation_row(&mut app, 3).await;
    assert!(wire.withdrawn && !wire.withdrawable && wire.edited, "the replayed row reads withdrawn and is no longer withdrawable");
    let unaccepted = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(first.clone()))]).await;
    assert_eq!(rejected(&unaccepted), Some("timeTravel.illegal"), "a mutation without an accepted draft has nothing to restore");

    let restored = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(target.clone()))]).await;
    assert_eq!(rejected(&restored), None, "{:?}", restored.output);
    assert!(app.time_travel.status().is_none(), "restoring the only accepted draft leaves history editing");
    assert_eq!((render_body(&mut app).await.contains(text(&fixture["committedBody"])), app.store.generation()), (true, generation), "zero trace: the committed document, the store untouched");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// ⚖️ LAW (design §22.1, §22.20): a mutation row always names Edit and, unless it is withdrawn, Withdraw — or Restore on a
/// mutation holding an accepted draft. A mutation without editable inputs keeps Withdraw and reads why Edit is refused, a
/// mutation the store's supersede law lets nobody withdraw reads that, a viewer reads why both are refused, in both
/// locales; only an offered Edit is the row's activation.
#[test]
fn a_mutation_row_names_why_edit_or_withdraw_is_refused() {
    let entry = |editable: bool, withdrawable: bool, withdrawn: bool| semio_framework::kernel::HistoryMutationEntry {
        mutation_id: "e-1#0".into(),
        position: 0,
        op_index: 0,
        label: LocalizedLabel::native("Set label", "Beschriftung setzen"),
        worst: None,
        messages: Vec::new(),
        superseded: withdrawn,
        withdrawn,
        editable,
        withdrawable,
        pending: false,
        edited: false,
        introduced: false,
        store: None,
    };
    let row = |entry: &semio_framework::kernel::HistoryMutationEntry, locale: Locale, read_only: bool, refusals: time_travel::MutationRowRefusals| {
        let row = history_panel_mutation_row(entry, "toy", locale, read_only, refusals).expect("the row builds");
        let semio_framework_ui_contract::Component::TreeItem(props) = &row.component else { panic!("a mutation row is a tree row") };
        let actions: Vec<(String, Option<String>)> = props.row_actions.iter().map(|action| (action.verb.as_str().to_string(), action.reason.as_ref().map(|reason| reason.0.as_str().to_string()))).collect();
        assert!(props.row_actions.iter().all(|action| action.disabled == action.reason.is_some()), "a disabled action names why: {actions:?}");
        (actions, props.target.as_ref().and_then(|target| target.activation.as_ref().map(|verb| verb.as_str().to_string())))
    };
    let offered = |verb: &str| (verb.to_string(), None);
    let refused = |verb: &str, reason: &str| (verb.to_string(), Some(reason.to_string()));
    let idle = time_travel::MutationRowRefusals::default();
    assert_eq!(row(&entry(true, true, false), Locale::En, false, idle), (vec![offered("historyEditBegin"), offered("historyEditWithdraw")], Some("historyEditBegin".to_string())), "an editable mutation");
    assert_eq!(
        row(&entry(false, true, false), Locale::En, false, idle),
        (vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), offered("historyEditWithdraw")], None),
        "a mutation without editable inputs keeps Withdraw and activates nothing"
    );
    assert_eq!(
        row(&entry(false, true, false), Locale::De, false, idle).0,
        vec![refused("historyEditBegin", "Die Eingaben dieser Mutation können nicht bearbeitet werden"), offered("historyEditWithdraw")],
        "the reason is the reader's language"
    );
    assert_eq!(
        row(&entry(false, false, false), Locale::En, false, idle).0,
        vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), refused("historyEditWithdraw", "This mutation cannot be withdrawn here")],
        "the store's supersede law refuses the withdrawal"
    );
    assert_eq!(
        row(&entry(false, false, false), Locale::En, true, idle),
        (vec![refused("historyEditBegin", "History cannot be edited in a read-only view"), refused("historyEditWithdraw", "History cannot be edited in a read-only view")], None),
        "a viewer reads why"
    );
    assert_eq!(row(&entry(true, false, true), Locale::En, false, idle).0, vec![offered("historyEditBegin")], "a withdrawn mutation offers no second withdrawal");
    let accepted = time_travel::MutationRowRefusals { accepted: true, ..idle };
    assert_eq!(row(&entry(false, false, true), Locale::En, false, accepted).0, vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), offered("historyEditRestore")], "an accepted draft is restored from its row");
    let replaying = time_travel::MutationRowRefusals { accepted: true, restore: Some(semio_framework_time_travel::TimeTravelLabel::RefusalIllegal), ..idle };
    assert_eq!(row(&entry(false, false, true), Locale::En, false, replaying).0[1], refused("historyEditRestore", "Not possible right now"), "outside the review Restore names why");
}

/// 🫥️ A payload schema whose one input is hidden: a draft editor would hold no row for it (design §22.20).
const HIDDEN_ONLY_SCHEMA: &str = r#"{"type":"object","additionalProperties":false,"required":["value"],"properties":{"value":{"type":"string","x-semio-ui":{"widget":"hidden","label":{"en":"Label","de":"Beschriftung"}}}}}"#;

/// 🪵️ A document operation over the toy kinds with two kinds an editor cannot repair — the label kind, whose payload
/// schema hides its only input and which refuses to apply on a negative count, and the children kind, which declares no
/// input schema: the smallest operations an upstream edit breaks without inputs to edit.
#[derive(Clone, Debug, PartialEq)]
struct InertLabelOp(TestMutation);

impl semio_framework_value::ToValue for InertLabelOp {
    fn to_value(&self) -> DslValue {
        semio_framework_value::ToValue::to_value(&self.0)
    }
}

impl semio_framework_value::FromValue for InertLabelOp {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <TestMutation as semio_framework_value::FromValue>::from_value(value).map(Self)
    }
}

impl ::protocol::OpBinary for InertLabelOp {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::protocol::OpBinary::encode_op(&self.0)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        <TestMutation as ::protocol::OpBinary>::decode_op(bytes).map(Self)
    }
}

impl ::protocol::OpText for InertLabelOp {
    fn print_op(&self) -> String {
        ::protocol::OpText::print_op(&self.0)
    }

    fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        <TestMutation as ::protocol::OpText>::parse_op(line).map(Self)
    }
}

impl store::Mutation<TestSnapshot> for InertLabelOp {
    type Diff = <TestMutation as store::Mutation<TestSnapshot>>::Diff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = <TestMutation as store::Mutation<TestSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        store::Mutation::<TestSnapshot>::descriptor(&self.0)
    }

    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::Diff> {
        match &self.0 {
            TestMutation::SetLabel(_) if base.count < 0 => ::protocol::MutationOutcome::error("mutation.target-mismatch", "a label needs a count that is not negative", ["count"]),
            operation => store::Mutation::<TestSnapshot>::diff(operation, base),
        }
    }

    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect())
    }

    fn conflict_target(&self) -> Vec<String> {
        store::Mutation::<TestSnapshot>::conflict_target(&self.0)
    }

    fn may_emit_foreign_steps(&self) -> bool {
        false
    }

    fn input_schema(&self) -> Option<&'static str> {
        match &self.0 {
            TestMutation::SetLabel(_) => Some(HIDDEN_ONLY_SCHEMA),
            TestMutation::SetSlotChildren(_) => None,
            operation => store::Mutation::<TestSnapshot>::input_schema(operation),
        }
    }

    fn payload_value(&self) -> DslValue {
        store::Mutation::<TestSnapshot>::payload_value(&self.0)
    }

    fn with_payload_value(&self, value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        store::Mutation::<TestSnapshot>::with_payload_value(&self.0, value).map(Self)
    }
}

/// ⏭️ Replays `drafts` from `from` on `document` to completion through its history-edit owners and answers the report.
async fn replayed_report(owners: &mut time_travel::TimeTravelStoreState<TestSnapshot, InertLabelOp>, document: &mut ArtifactStore<TestSnapshot, InertLabelOp>, drafts: &BTreeMap<MutationId, protocol::InputReplacement>, from: &MutationId) -> protocol::ReplayReport {
    let started = owners.run(document, time_travel::TimeTravelStoreCommand::StartReplay { drafts: drafts.clone(), from: from.clone() }).await.expect("the replay starts");
    assert!(matches!(started, time_travel::TimeTravelStoreOutput::ReplayStarted(true)), "the store starts the Report replay");
    for _ in 0..65_536 {
        match owners.run(document, time_travel::TimeTravelStoreCommand::StepReplay { deadline_us: u64::MAX, clock: semio_framework_job::default_now_us }).await.expect("a replay slice") {
            time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Completed(report)) => return report,
            time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Pending { .. }) => {}
            _ => panic!("the replay faulted"),
        }
    }
    panic!("the replay never completed");
}

/// ⚖️ LAW (design §22.1, §22.20): a blocking mutation without editable inputs is always resolvable. A mutation whose
/// payload schema hides every input, and one that declares no input schema, are not editable and still withdrawable (never
/// for a viewer). An upstream edit makes the hidden-only one fail; opening it for editing is refused
/// `timeTravel.not-editable` (an editor with zero rows never opens); opening it to withdraw it is admitted by the store's
/// supersede law on an editor without inputs; the replay with the withdrawal no longer blocks and finalizes as one
/// overwrite whose head holds the edit and skips the withdrawn operation.
#[semio_framework_async_macros::async_test]
async fn a_blocking_mutation_without_editable_inputs_is_withdrawn_and_the_review_becomes_ready() {
    use time_travel::{TimeTravelActionRefusal, TimeTravelCommit, TimeTravelOwners, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
    let schema = "semio.test.inert-label/v1";
    let genesis = store::create_document_envelope::<TestSnapshot, InertLabelOp>(schema, "inert-label", TestSnapshot::default(), None);
    let mut document = Box::pin(ArtifactStore::new(genesis)).await.expect("the document store");
    document.install_document_store_owners_exact(bounded_document_store_owners::<TestSnapshot, InertLabelOp>());
    document.set_local_actor_id(Some("local".to_string())).expect("local actor");
    assert!(semio_framework::mutation_input_defs(HIDDEN_ONLY_SCHEMA, &semio_framework::registered_input_schema_document).is_ok_and(|inputs| inputs.len() == 1 && inputs[0].presentation == Some(semio_framework::ArgPresentation::Hidden)), "the hidden-only schema reads as one hidden input");
    for operation in [TestMutation::SetCount(SetCount { value: 1 }), TestMutation::SetLabel(SetLabel { value: "a".into() }), TestMutation::SetSlotChildren(SetSlotChildren { children: Vec::new() }), TestMutation::SetCount(SetCount { value: 5 })] {
        Box::pin(document.dispatch(ArtifactCommand::Apply { mutations: vec![InertLabelOp(operation)], description: None, transaction: None })).await.expect("a seed edit applies");
    }
    let ids: Vec<MutationId> = document.mutation_ops().expect("applied operations").iter().map(|op| op.mutation_id.clone()).collect();
    let (count, label, children) = (ids[0].clone(), ids[1].clone(), ids[2].clone());
    {
        let ops = document.mutation_ops().expect("applied operations");
        let unlabelled = HashMap::new();
        let row = |index: usize, viewer: bool| {
            let view = time_travel::history_mutation_view_of::<TestSnapshot, InertLabelOp>(&ops[index], None, &unlabelled, &|_: &InertLabelOp| LocalizedLabel::data("operation"), viewer, None);
            (view.editable, view.withdrawable)
        };
        assert_eq!((row(0, false), row(1, false), row(2, false)), ((true, true), (false, true), (false, true)), "hidden-only inputs and no input schema: not editable, still withdrawable");
        assert_eq!((row(0, true), row(1, true), row(2, true)), ((false, false), (false, false), (false, false)), "a viewer edits and withdraws nothing");
    }

    let mut owners = TimeTravelStoreState::<TestSnapshot, InertLabelOp>::new(|_| LocalizedLabel::data("operation"));
    let negative = protocol::InputReplacement::Input { schema: schema.to_string(), payload: ::protocol::OpBinary::encode_op(&InertLabelOp(TestMutation::SetCount(SetCount { value: -1 }))).expect("the edited count encodes") };
    let mut drafts = BTreeMap::from([(count.clone(), negative)]);
    let blocked = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    let problem = blocked.outcomes.iter().find(|outcome| outcome.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst))).map(|outcome| outcome.mutation_id.clone());
    assert_eq!((blocked.blocks_finalize(), problem), (true, Some(label.clone())), "the edited count breaks the label operation downstream: {blocked:?}");

    for (target, why) in [(&label, "an editor with zero rows never opens"), (&children, "an operation without an input schema has no editor")] {
        let refused = owners.run(&mut document, TimeTravelStoreCommand::Open { target: target.clone(), current: None, withdraw: false }).await.expect("the open runs");
        assert!(matches!(refused, TimeTravelStoreOutput::Opened(Err(TimeTravelActionRefusal::NotEditable))), "{why}");
    }
    let TimeTravelStoreOutput::Opened(Ok((editor, original))) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: label.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true }).await.expect("the open runs") else {
        panic!("the store's supersede law admits withdrawing an operation without editable inputs");
    };
    assert!(editor.inputs.is_empty() && editor.inputs_refused.is_none() && editor.target == label, "its editor holds no inputs and names no schema fault");
    assert!(matches!(original, protocol::InputReplacement::Input { .. }), "its original input is its recorded operation");
    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true)).await.expect("the session adopts the kind");

    drafts.insert(label.clone(), protocol::InputReplacement::Withdrawn);
    let ready = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(!ready.blocks_finalize() && ready.outcomes.iter().any(|outcome| outcome.mutation_id == label && outcome.withdrawn), "withdrawing the failing operation leaves a report that finalizes: {ready:?}");
    let committed = owners.run(&mut document, TimeTravelStoreCommand::Commit { drafts: drafts.clone(), finalization: store::HistoryFinalization::Overwrite, actor: None }).await.expect("the commit runs");
    assert!(matches!(committed, TimeTravelStoreOutput::Committed(TimeTravelCommit::Finalized { .. })), "the finished replay commits as one overwrite");
    let head = document.snapshot().expect("head");
    assert_eq!((head.count, head.label.as_str()), (5, ""), "the head holds the edit and skips the withdrawn operation");

    owners.settle(&document, TimeTravelStage::Inactive).expect("the owners settle");
    for _ in 0..65_536 {
        if owners.retire_step(64, usize::MAX).expect("the owners retire").is_none() {
            break;
        }
    }
    assert!(owners.terminal_is_empty(), "the history-edit owners retire everything they held");
    for _ in 0..65_536 {
        match document.close_owned_step(64, 4096).expect("the document store closes under its exact grant") {
            store::SnapshotRetirementStep::Pending { .. } => {}
            store::SnapshotRetirementStep::Blocked => panic!("the document store has no external owner"),
            store::SnapshotRetirementStep::Complete => {
                assert!(document.close_owned_terminal_is_empty());
                return;
            }
        }
    }
    panic!("the document store did not close");
}
//#endregion 🚫️RowWithdraw
""",
    ),
]

WGPU_LAW_RS = [
    (
        """                    editable: true,
                    pending: false,
""",
        """                    editable: true,
                    withdrawable: true,
                    pending: false,
""",
    ),
    (
        """superseded: false, withdrawn: false, editable: true, store: None }""",
        """superseded: false, withdrawn: false, editable: true, withdrawable: true, store: None }""",
    ),
    (
        """        outcomes: Default::default(),
        edited: Default::default(),
    }
}
""",
        """        outcomes: Default::default(),
        edited: Default::default(),
        accepted: Default::default(),
    }
}
""",
    ),
    (
        """                row("/targets", targets, serde_json::json!(["n1", "n2"]), None, None),
            ],
            inputs_refused: None,
""",
        """                row("/targets", targets, serde_json::json!(["n1", "n2"]), None, None),
            ],
            editable: true,
            inputs_refused: None,
""",
    ),
]

COMPOSED_CHILD_LAW_RS = [
    (
        """        withdrawn: false,
        editable: true,
        store: Some(store.clone()),
""",
        """        withdrawn: false,
        editable: true,
        withdrawable: true,
        store: Some(store.clone()),
""",
    ),
]

TT_CONTROLS_RS = [
    (
        """    Cleared,
    Alternatives,
""",
        """    Cleared,
    TooLong,
    Alternatives,
""",
    ),
    (
        """                Self::Cleared => "Cleared (no value)",
""",
        """                Self::Cleared => "Cleared (no value)",
                Self::TooLong => "Too long to edit here: shown shortened",
""",
    ),
    (
        """                Self::Cleared => "Geleert (kein Wert)",
""",
        """                Self::Cleared => "Geleert (kein Wert)",
                Self::TooLong => "Zu lang, um hier bearbeitet zu werden: gekürzt angezeigt",
""",
    ),
    (
        """/// 🎛️ One input row, its id the pointer with `.` for `/` under `framework.history.editor.input` (`….row`): a tree item
""",
        """/// 📜️ The text field of a text input — one line, or `LongText` for a multi-line one (design §22.7) — committing on blur.
/// A value longer than one UI text holds is shown clipped and read-only, so a commit never writes the clipped text back.
fn time_travel_text_control(kind: InputKind, shown: &str, id: &str, label: &str, action: ActionId, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    input(kind)
        .value(UiText::clipped(shown))
        .commit(UiText::clipped("blur"))
        .disabled(shown.len() > UI_TEXT_MAX_BYTES)
        .try_id(id)
        .map_err(|_| error("time-travel-panel.input-id"))?
        .try_label(label)
        .map_err(|_| error("time-travel-panel.input-label"))?
        .try_on_with(Trigger::Commit, action, args)
        .map_err(|_| error("time-travel-panel.input-binding"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.input"))
}

/// 🎛️ One input row, its id the pointer with `.` for `/` under `framework.history.editor.input` (`….row`): a tree item
""",
    ),
    (
        """/// derives (`ActionArgDef::control`), bound to the row's pointer. A list's row holds Add item (`historyEditInput{edit:
""",
        """/// derives (`ActionArgDef::control`) as itself (design §22.7: a segmented choice is a segmented select, an icon choice an
/// icon picker, a multi-line text a multi-line field), bound to the row's pointer. A list's row holds Add item (`historyEditInput{edit:
""",
    ),
    (
        """        ActionArgControl::Select { options } | ActionArgControl::Segmented { options } => {
            let mut builder = select(UiText::clipped(value.as_str().unwrap_or_default()));
            for option in options.iter().take(UI_FIXED_LIST_ITEMS) {
""",
        """        ActionArgControl::Select { options } | ActionArgControl::Segmented { options } => {
            let appearance = if arg.presentation == Some(ArgPresentation::Segmented) { SelectAppearance::Segmented } else { SelectAppearance::Menu };
            let mut builder = select(UiText::clipped(value.as_str().unwrap_or_default())).appearance(appearance);
            for option in options.iter().take(UI_FIXED_LIST_ITEMS) {
""",
    ),
    (
        """        ActionArgControl::Color { alpha } => time_travel_color_control(value, alpha, &id, label, locale, &action, generation, pointer)?,
        _ => {
            let shown = match value {
                DslValue::String(text) => text.clone(),
                other => time_travel_json(other),
            };
            input(InputKind::Text)
                .value(UiText::clipped(&shown))
                .commit(UiText::clipped("blur"))
                .try_id(&id)
                .map_err(|_| error("time-travel-panel.input-id"))?
                .try_label(label)
                .map_err(|_| error("time-travel-panel.input-label"))?
                .try_on_with(Trigger::Commit, action, args)
                .map_err(|_| error("time-travel-panel.input-binding"))?
                .try_build()
                .map_err(|_| error("time-travel-panel.input"))?
        }
    };
""",
        """        ActionArgControl::Color { alpha } => time_travel_color_control(value, alpha, &id, label, locale, &action, generation, pointer)?,
        ActionArgControl::IconSelect { classifier_kind } => icon_select(UiText::clipped(value.as_str().unwrap_or_default()), UiText::clipped(&classifier_kind))
            .try_id(&id)
            .map_err(|_| error("time-travel-panel.input-id"))?
            .try_label(label)
            .map_err(|_| error("time-travel-panel.input-label"))?
            .try_on_with(Trigger::Change, action, args)
            .map_err(|_| error("time-travel-panel.input-binding"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.input"))?,
        control => {
            let shown = match value {
                DslValue::String(text) => text.clone(),
                other => time_travel_json(other),
            };
            if shown.len() > UI_TEXT_MAX_BYTES {
                shown_unit = Some(HistoryPanelText::TooLong.text(locale).to_string());
            }
            let kind = if matches!(control, ActionArgControl::Multiline) { InputKind::LongText } else { InputKind::Text };
            time_travel_text_control(kind, &shown, &id, label, action, args)?
        }
    };
""",
    ),
]


def files(_root):
    return {
        f"{MANIFEST}/🦀️.rs": MANIFEST_RS,
        f"{MANIFEST}/🧫️fixtures/🧫️history-edit-actions/🔣️.json": MANIFEST_FIXTURE,
        f"{MANIFEST}/🧫️fixtures/🧫️history-edit-actions/🧬️schema/🔣️.json": MANIFEST_FIXTURE_SCHEMA,
        f"{KERNEL}/🦀️.rs": KERNEL_RS,
        f"{KERNEL}/🧬️schema/🔣️history-patch/🔣️.json": KERNEL_SCHEMA,
        f"{KERNEL}/🧫️fixtures/🧫️history-patch/🔣️.json": KERNEL_FIXTURE,
        TT: TT_RS + TT_CONTROLS_RS,
        PLG: PLG_RS,
        f"{PLUGIN}/🧫️fixtures/🧫️time-travel/🔣️.json": PLUGIN_FIXTURE,
        f"{PLUGIN}/🧫️fixtures/🧫️time-travel/🧬️schema/🔣️.json": PLUGIN_FIXTURE_SCHEMA,
        f"{PLUGIN}/🧪️tests/🧪️time-travel/🟦️.ts": PLUGIN_ORACLE,
        f"{PLUGIN}/🧪️tests/🧪️time-travel/🦀️.rs": PLUGIN_LAWS,
        f"{PLUGIN}/🧪️tests/🧪️composed-child-history/🦀️.rs": COMPOSED_CHILD_LAW_RS,
        WGPU_LAW: WGPU_LAW_RS,
    }
