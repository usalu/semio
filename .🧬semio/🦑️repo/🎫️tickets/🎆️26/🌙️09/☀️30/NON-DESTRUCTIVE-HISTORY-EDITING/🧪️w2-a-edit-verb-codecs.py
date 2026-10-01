"""🏷️ Hand-written half of the `verb` pass: the `Edit`/`MutationEnvelope`/`HistoryEdit` fields, their value, wire, `.spr`
and `.ops` codecs, the canonical edit form, the owned `.spr` decoder, the store's authoring authority and its stamping."""
import pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio")


def patch(rel, pairs):
    path = ROOT / rel
    text = path.read_text()
    for old, new in pairs:
        if text.count(new) == 1:
            continue
        count = text.count(old)
        assert count == 1, (rel, count, old[:160])
        text = text.replace(old, new)
    path.write_text(text)


patch("🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs", [
    ("""    pub mutation_meta: Vec<MutationMeta>,
    pub description: Option<String>,
    pub coalesce_key: Option<String>,
    pub sequence_number: i32,""", """    pub mutation_meta: Vec<MutationMeta>,
    pub description: Option<String>,
    /// 🏷️ The id of the action or command that authored this edit — never display text: history resolves it through
    /// the authoring app's registry to its label in every locale at projection time, so a reload or a peer keeps it.
    pub verb: Option<String>,
    pub coalesce_key: Option<String>,
    pub sequence_number: i32,"""),
    ("""        if self.description.is_some() {
            entries.push(("description".to_string(), crate::value::ToValue::to_value(&self.description)));
        }
        if self.coalesce_key.is_some() {
            entries.push(("coalesceKey".to_string(), crate::value::ToValue::to_value(&self.coalesce_key)));
        }
        entries.push(("sequenceNumber".to_string(), crate::value::ToValue::to_value(&self.sequence_number)));""", """        if self.description.is_some() {
            entries.push(("description".to_string(), crate::value::ToValue::to_value(&self.description)));
        }
        if self.verb.is_some() {
            entries.push(("verb".to_string(), crate::value::ToValue::to_value(&self.verb)));
        }
        if self.coalesce_key.is_some() {
            entries.push(("coalesceKey".to_string(), crate::value::ToValue::to_value(&self.coalesce_key)));
        }
        entries.push(("sequenceNumber".to_string(), crate::value::ToValue::to_value(&self.sequence_number)));"""),
    ("""        let mut description = None;
        let mut coalesce_key = None;
        let mut sequence_number = None;""", """        let mut description = None;
        let mut verb = None;
        let mut coalesce_key = None;
        let mut sequence_number = None;"""),
    ("""                "description" => description = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("description"))?,
                "coalesceKey" => coalesce_key""", """                "description" => description = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("description"))?,
                "verb" => verb = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("verb"))?,
                "coalesceKey" => coalesce_key"""),
    ("""            mutation_meta,
            description,
            coalesce_key,
            sequence_number: sequence_number.ok_or_else""", """            mutation_meta,
            description,
            verb,
            coalesce_key,
            sequence_number: sequence_number.ok_or_else"""),
])

patch("🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs", [
    ("""/// the committed tool transaction that authored the operation (`None` for every transition and for
/// operations authored outside a tool transaction).""", """/// the committed tool transaction that authored the operation (`None` for every transition and for
/// operations authored outside a tool transaction). `verb` is the id of the action or command whose edit carried the
/// operation (`None` for every transition and for operations no verb authored), so a peer labels its history row
/// exactly as the author does."""),
    ("""    pub transaction: Option<crate::mutation::TransactionRef>,
}

impl crate::value::ToValue for MutationEnvelope {""", """    pub transaction: Option<crate::mutation::TransactionRef>,
    pub verb: Option<String>,
}

impl crate::value::ToValue for MutationEnvelope {"""),
    ("""        if self.transaction.is_some() {
            entries.push(("transaction".to_string(), crate::value::ToValue::to_value(&self.transaction)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for MutationEnvelope {""", """        if self.transaction.is_some() {
            entries.push(("transaction".to_string(), crate::value::ToValue::to_value(&self.transaction)));
        }
        if self.verb.is_some() {
            entries.push(("verb".to_string(), crate::value::ToValue::to_value(&self.verb)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for MutationEnvelope {"""),
    ("""        let mut transaction = None;
        for (key, entry) in fields {
            match key.as_str() {
                "mutationId" => mutation_id""", """        let mut transaction = None;
        let mut verb = None;
        for (key, entry) in fields {
            match key.as_str() {
                "mutationId" => mutation_id"""),
    ("""                "transaction" => transaction = <Option<crate::mutation::TransactionRef> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("transaction"))?,
                _ => {}""", """                "transaction" => transaction = <Option<crate::mutation::TransactionRef> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("transaction"))?,
                "verb" => verb = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("verb"))?,
                _ => {}"""),
    ("""            timestamp: timestamp.ok_or_else(|| crate::value::ValueError::new("MutationEnvelope missing timestamp"))?,
            transaction,
        })""", """            timestamp: timestamp.ok_or_else(|| crate::value::ValueError::new("MutationEnvelope missing timestamp"))?,
            transaction,
            verb,
        })"""),
    ("""            transaction: meta.and_then(|m| m.transaction.clone()), verb: None,""", """            transaction: meta.and_then(|m| m.transaction.clone()), verb: edit.verb.clone(),"""),
    ("""/// inverse.payload bytes | hlc | transaction (0 | 1 id str tool str)`.
pub fn encode_envelope(""", """/// inverse.payload bytes | hlc | trailing flags varint (bit 0 transaction, bit 1 verb) | [transaction id str tool str] |
/// [verb str]` — an envelope with neither keeps the single `0` flag byte.
pub fn encode_envelope("""),
    ("""    encode_hlc(out, &envelope.timestamp);
    match &envelope.transaction {
        Some(transaction) => {
            crate::wire::write_varint_u64(out, 1);
            crate::write_str(out, &transaction.id);
            crate::write_str(out, &transaction.tool);
        }
        None => crate::wire::write_varint_u64(out, 0),
    }
}""", """    encode_hlc(out, &envelope.timestamp);
    crate::wire::write_varint_u64(out, u64::from(envelope.transaction.is_some()) | u64::from(envelope.verb.is_some()) << 1);
    if let Some(transaction) = &envelope.transaction {
        crate::write_str(out, &transaction.id);
        crate::write_str(out, &transaction.tool);
    }
    if let Some(verb) = &envelope.verb {
        crate::write_str(out, verb);
    }
}"""),
    ("""    let timestamp = decode_hlc(bytes, pos)?;
    let transaction = match crate::wire::read_varint_u64(bytes, pos)? {
        0 => None,
        1 => Some(crate::mutation::TransactionRef { id: crate::read_str(bytes, pos)?, tool: crate::read_str(bytes, pos)? }),
        flag => return Err(crate::ProtocolError::Malformed { what: "mutation envelope", offset: *pos as u64, detail: format!("transaction flag {flag}") }),
    };
    Ok(MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff: ArtifactDiff { schema: diff_schema, payload: diff_payload }, inverse: InverseMutation { schema: inverse_schema, payload: inverse_payload }, timestamp, transaction })""", """    let timestamp = decode_hlc(bytes, pos)?;
    let flags = crate::wire::read_varint_u64(bytes, pos)?;
    if flags > 0b11 {
        return Err(crate::ProtocolError::Malformed { what: "mutation envelope", offset: *pos as u64, detail: format!("trailing flags {flags}") });
    }
    let transaction = if flags & 0b01 != 0 { Some(crate::mutation::TransactionRef { id: crate::read_str(bytes, pos)?, tool: crate::read_str(bytes, pos)? }) } else { None };
    let verb = if flags & 0b10 != 0 { Some(crate::read_str(bytes, pos)?) } else { None };
    Ok(MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff: ArtifactDiff { schema: diff_schema, payload: diff_payload }, inverse: InverseMutation { schema: inverse_schema, payload: inverse_payload }, timestamp, transaction, verb })"""),
    ("""        let transaction_at = position;
        let transaction = match read_document_backbone_u64(bytes, &mut position)? {
            0 => None,
            1 => Some(crate::mutation::TransactionRef {
                id: read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?,
                tool: read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?,
            }),
            _ => return Err(document_backbone_batch_malformed(transaction_at, "transaction-flag")),
        };
        envelopes.push(MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff: ArtifactDiff { schema: diff_schema, payload: diff_payload }, inverse: InverseMutation { schema: inverse_schema, payload: inverse_payload }, timestamp, transaction });""", """        let flags_at = position;
        let flags = read_document_backbone_u64(bytes, &mut position)?;
        if flags > 0b11 {
            return Err(document_backbone_batch_malformed(flags_at, "trailing-flags"));
        }
        let transaction = if flags & 0b01 != 0 {
            Some(crate::mutation::TransactionRef {
                id: read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?,
                tool: read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?,
            })
        } else {
            None
        };
        let verb = if flags & 0b10 != 0 { Some(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?) } else { None };
        envelopes.push(MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff: ArtifactDiff { schema: diff_schema, payload: diff_payload }, inverse: InverseMutation { schema: inverse_schema, payload: inverse_payload }, timestamp, transaction, verb });"""),
])

patch("🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs", [
    ("""    pub coalesce_key: Option<String>,
    pub description: Option<String>,
    pub ops: Vec<OpPayload>,""", """    pub coalesce_key: Option<String>,
    pub description: Option<String>,
    /// 🏷️ The id of the action or command that authored this edit (mirrors `crate::os_spr::command::Edit::verb`).
    pub verb: Option<String>,
    pub ops: Vec<OpPayload>,"""),
    ("""const F_EDIT_DESCRIPTION: u16 = 5;""", """const F_EDIT_DESCRIPTION: u16 = 5;
const F_EDIT_VERB: u16 = 6;"""),
    ("""            FieldSpec::new(F_EDIT_DESCRIPTION, "description", Shape::Text).optional(),
        ],""", """            FieldSpec::new(F_EDIT_DESCRIPTION, "description", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_VERB, "verb", Shape::Text).optional(),
        ],"""),
    ("""        coalesce_key: Option<String>,
        description: Option<String>,
    }

    let mut log = HistoryLog::default();""", """        coalesce_key: Option<String>,
        description: Option<String>,
        verb: Option<String>,
    }

    let mut log = HistoryLog::default();"""),
    ("""                    description: field_text(&record, F_EDIT_DESCRIPTION),
                });""", """                    description: field_text(&record, F_EDIT_DESCRIPTION),
                    verb: field_text(&record, F_EDIT_VERB),
                });"""),
    ("""        if let Some(description) = &edit.description {
            fields.push((F_EDIT_DESCRIPTION, FieldValue::Text(description.clone())));
        }""", """        if let Some(description) = &edit.description {
            fields.push((F_EDIT_DESCRIPTION, FieldValue::Text(description.clone())));
        }
        if let Some(verb) = &edit.verb {
            fields.push((F_EDIT_VERB, FieldValue::Text(verb.clone())));
        }"""),
    ("""// REC_EDIT layout: format u8, presence u8 (bit0 actor, bit1 finished, bit2 key, bit3 description,
// bit4 explicit_meta, bit5 has_backwards_section), id, started(ts), [actor(dictref)],
// [finished(ts)], [key(str)], [description(str)], op_count varint,""", """// REC_EDIT layout: format u8, presence u8 (bit0 actor, bit1 finished, bit2 key, bit3 description,
// bit4 explicit_meta, bit5 has_backwards_section, bit6 lane, bit7 verb), id, started(ts), [actor(dictref)],
// [finished(ts)], [key(str)], [description(str)], [lane(str)], [verb(str)], op_count varint,"""),
    ("""    if edit.lane.is_some() {
        presence |= 1 << 6;
    }
    out.write_u8(presence);""", """    if edit.lane.is_some() {
        presence |= 1 << 6;
    }
    if edit.verb.is_some() {
        presence |= 1 << 7;
    }
    out.write_u8(presence);"""),
    ("""    if let Some(lane) = &edit.lane {
        write_str_field(&mut out, lane).await;
    }
    if edit.ops.len() as u64 > ProtocolLimits::default().max_op_count_per_edit as u64 {""", """    if let Some(lane) = &edit.lane {
        write_str_field(&mut out, lane).await;
    }
    if let Some(verb) = &edit.verb {
        write_str_field(&mut out, verb).await;
    }
    if edit.ops.len() as u64 > ProtocolLimits::default().max_op_count_per_edit as u64 {"""),
    ("""    let lane = if presence & (1 << 6) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let op_count = input.read_varint_u64()?;""", """    let lane = if presence & (1 << 6) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let verb = if presence & (1 << 7) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let op_count = input.read_varint_u64()?;"""),
    ("""    Ok(HistoryEdit { id, actor, started_at, finished_at, coalesce_key, description, ops, inverse, meta, lane })""", """    Ok(HistoryEdit { id, actor, started_at, finished_at, coalesce_key, description, verb, ops, inverse, meta, lane })"""),
])
print("replication + spr ok")


def store():
    patch("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs", [
        ("""struct ArtifactStoreEditRetirementState {
    strings: [Option<String>; 6],""", """struct ArtifactStoreEditRetirementState {
    strings: [Option<String>; 7],"""),
        ("""        let Edit { id, actor, forwards, inverse, mutation_meta, description, coalesce_key, sequence_number: _, started_at, finished_at } = edit;
        drop(forwards);
        drop(inverse);
        Ok(Self { state: std::mem::ManuallyDrop::new(Some(ArtifactStoreEditRetirementState { strings: [Some(id), actor, description, coalesce_key, Some(started_at), finished_at], mutation_meta, active_meta: None, active_bytes: None })) })""", """        let Edit { id, actor, forwards, inverse, mutation_meta, description, verb, coalesce_key, sequence_number: _, started_at, finished_at } = edit;
        drop(forwards);
        drop(inverse);
        Ok(Self { state: std::mem::ManuallyDrop::new(Some(ArtifactStoreEditRetirementState { strings: [Some(id), actor, description, verb, coalesce_key, Some(started_at), finished_at], mutation_meta, active_meta: None, active_bytes: None })) })"""),
        ("""    OwnedSchemaFieldSpec { id: 10, key: "finishedAt", required: false },
];""", """    OwnedSchemaFieldSpec { id: 10, key: "finishedAt", required: false },
    OwnedSchemaFieldSpec { id: 11, key: "verb", required: false },
];"""),
        ("""    strings: [std::mem::ManuallyDrop<Option<String>>; 6],
    forwards: std::mem::ManuallyDrop<Option<Vec<Mutation>>>,""", """    strings: [std::mem::ManuallyDrop<Option<String>>; 7],
    forwards: std::mem::ManuallyDrop<Option<Vec<Mutation>>>,"""),
        ("""            if token.kind == OwnedSchemaTokenKind::Null && matches!(field_id, 2 | 6 | 7 | 10) {""", """            if token.kind == OwnedSchemaTokenKind::Null && matches!(field_id, 2 | 6 | 7 | 10 | 11) {"""),
        ("""            9 => Some(4),
            10 => Some(5),
            _ => None,""", """            9 => Some(4),
            10 => Some(5),
            11 => Some(6),
            _ => None,"""),
        ("""            Some(Edit { id, actor: self.strings[1].take(), forwards, inverse, mutation_meta: Vec::new(), description: self.strings[2].take(), coalesce_key: self.strings[3].take(), sequence_number, started_at, finished_at: self.strings[5].take() });""", """            Some(Edit { id, actor: self.strings[1].take(), forwards, inverse, mutation_meta: Vec::new(), description: self.strings[2].take(), verb: self.strings[6].take(), coalesce_key: self.strings[3].take(), sequence_number, started_at, finished_at: self.strings[5].take() });"""),
        ("""        key: Option<String>,
        description: Option<String>,
    },
    /// ⏪️ A `Revert` transition""", """        key: Option<String>,
        description: Option<String>,
        verb: Option<String>,
    },
    /// ⏪️ A `Revert` transition"""),
        ("""        key: edit.coalesce_key.clone(),
        description: edit.description.clone(),
    };
    let mut out = header.print_op();""", """        key: edit.coalesce_key.clone(),
        description: edit.description.clone(),
        verb: edit.verb.clone(),
    };
    let mut out = header.print_op();"""),
        ("""        coalesce_key: Option<String>,
        description: Option<String>,
    }
    let mut pending_edit: Option<PendingEdit> = None;""", """        coalesce_key: Option<String>,
        description: Option<String>,
        verb: Option<String>,
    }
    let mut pending_edit: Option<PendingEdit> = None;"""),
        ("""            OpsHeaderLine::Edit { id: edit_id, sequence, started, actor, finished, key, description } => {""", """            OpsHeaderLine::Edit { id: edit_id, sequence, started, actor, finished, key, description, verb } => {"""),
        ("""finished_at: finished, coalesce_key: key, description });""", """finished_at: finished, coalesce_key: key, description, verb });"""),
        ("""                &(chains.meta as u64).to_be_bytes(),
                &chains.meta_digest,
            ],
        );
        (digest, Some(chains))""", """                &(chains.meta as u64).to_be_bytes(),
                &chains.meta_digest,
            ],
        );
        let digest = match edit.verb.as_deref() {
            Some(verb) => Self::hash_record(b"edit-verb", &[&digest, verb.as_bytes()]),
            None => digest,
        };
        (digest, Some(chains))"""),
        ("""        for value in [&mut self.edit.actor, &mut self.edit.description, &mut self.edit.coalesce_key, &mut self.edit.finished_at, &mut self.local_actor] {""", """        for value in [&mut self.edit.actor, &mut self.edit.description, &mut self.edit.verb, &mut self.edit.coalesce_key, &mut self.edit.finished_at, &mut self.local_actor] {"""),
        ("""            && self.edit.description.is_none()
            && self.edit.coalesce_key.is_none()""", """            && self.edit.description.is_none()
            && self.edit.verb.is_none()
            && self.edit.coalesce_key.is_none()"""),
        ("""    coalesce_key: Option<String>,
    transaction: Option<protocol::TransactionRef>,
    outbound: bool,
    announce_from: usize,
}""", """    coalesce_key: Option<String>,
    verb: Option<String>,
    transaction: Option<protocol::TransactionRef>,
    outbound: bool,
    announce_from: usize,
}"""),
        ("""    pub fn set_coalesce_key(&mut self, key: Option<String>) {
        self.coalesce_key = key.filter(|value| !value.is_empty());
    }
""", """    pub fn set_coalesce_key(&mut self, key: Option<String>) {
        self.coalesce_key = key.filter(|value| !value.is_empty());
    }

    /// 🏷️ The id of the action or command this gesture publishes for, stamped on the gesture's new edit (an amend keeps
    /// the verb of the edit it extends). See [`crate::os_spr::command::Edit::verb`].
    pub fn set_verb(&mut self, verb: Option<String>) {
        self.verb = verb.filter(|value| !value.is_empty());
    }
"""),
        ("""        if self.coalesce_key.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.receipt.take().is_some() {""", """        if self.coalesce_key.take().is_some() || self.verb.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.receipt.take().is_some() {"""),
        ("""&& self.receipt.is_none() && self.fault.is_none() && self.coalesce_key.is_none()
    }""", """&& self.receipt.is_none() && self.fault.is_none() && self.coalesce_key.is_none() && self.verb.is_none()
    }"""),
        ("""        } else if self.coalesce_key.is_some() {
            "coalesce-key"
        } else if self.receipt.is_some() {""", """        } else if self.coalesce_key.is_some() {
            "coalesce-key"
        } else if self.verb.is_some() {
            "verb"
        } else if self.receipt.is_some() {"""),
        ("""            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
            coalesce_key: None,
            transaction,
            outbound,""", """            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
            coalesce_key: None,
            verb: None,
            transaction,
            outbound,"""),
        ("""            stage.edit.coalesce_key = publication.coalesce_key.clone();
            stage.edit.started_at""", """            stage.edit.coalesce_key = publication.coalesce_key.clone();
            stage.edit.verb = publication.verb.clone();
            stage.edit.started_at"""),
        ("""    member_inbox: VecDeque<BackboneMessage>,
    pending_report: std::mem::ManuallyDrop<PendingCommandReport>,""", """    member_inbox: VecDeque<BackboneMessage>,
    /// 🏷️ The id of the action or command whose `Apply` this store is dispatching, stamped on the edit it mints (an
    /// amend keeps the verb of the edit it extends). Set by the dispatching runtime via
    /// {@link set_authoring_verb}; not part of the wire envelope.
    authoring_verb: Option<String>,
    pending_report: std::mem::ManuallyDrop<PendingCommandReport>,"""),
        ("""        local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
        member_inbox: VecDeque::new(),
        merge_policy""", """        local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
        member_inbox: VecDeque::new(),
        authoring_verb: None,
        merge_policy"""),
        ("""            local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
            member_inbox: VecDeque::new(),
            merge_policy""", """            local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
            member_inbox: VecDeque::new(),
            authoring_verb: None,
            merge_policy"""),
        ("""    pub fn set_local_actor_id(&mut self, actor_id: Option<String>) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.replace_local_actor_retained(actor_id)
    }
""", """    pub fn set_local_actor_id(&mut self, actor_id: Option<String>) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.replace_local_actor_retained(actor_id)
    }

    /// 🏷️ The verb id the next locally minted edit is stamped with (see {@link set_authoring_verb}).
    pub fn authoring_verb(&self) -> Option<&str> {
        self.authoring_verb.as_deref()
    }

    /// 🏷️ Names the action or command whose `Apply` the caller dispatches next, so the edit it mints carries a
    /// locale-neutral label source every reload and peer resolves the same way; the caller clears it after dispatch.
    pub fn set_authoring_verb(&mut self, verb: Option<String>) {
        self.authoring_verb = verb.filter(|value| !value.is_empty());
    }
"""),
        ("""            mutation_meta,
            description,
            coalesce_key: None,
            sequence_number: self.edit_sequence,
            started_at,
            finished_at: Some(now_iso()),
        };""", """            mutation_meta,
            description,
            verb: self.authoring_verb.clone(),
            coalesce_key: None,
            sequence_number: self.edit_sequence,
            started_at,
            finished_at: Some(now_iso()),
        };"""),
        ("""mutation_meta, description: None, verb: None, coalesce_key, sequence_number: self.edit_sequence""", """mutation_meta, description: None, verb: self.authoring_verb.clone(), coalesce_key, sequence_number: self.edit_sequence"""),
        ("""            transaction: envelope.transaction.clone(),
        }],
        description: None, verb: None,""", """            transaction: envelope.transaction.clone(),
        }],
        description: None, verb: envelope.verb.clone(),"""),
    ])
    print("store ok")


store()


def db_history_cursor():
    patch("🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs", [
        ("""    ClockLogical,
    TransactionFlag,
    TransactionId,
    TransactionTool,
    Done,
}""", """    ClockLogical,
    TrailingFlags,
    TransactionId,
    TransactionTool,
    Verb,
    Done,
}"""),
        ("""    target_segments: u64,
    mutation_id: Option<std::ops::Range<u64>>,
}""", """    target_segments: u64,
    verb: bool,
    mutation_id: Option<std::ops::Range<u64>>,
}"""),
        ("""field: HistoryEnvelopeField::MutationId, dependencies: 0, target_segments: 0, mutation_id: None })""", """field: HistoryEnvelopeField::MutationId, dependencies: 0, target_segments: 0, verb: false, mutation_id: None })"""),
        ("""                self.field = HistoryEnvelopeField::TransactionFlag;
            }
            HistoryEnvelopeField::TransactionFlag => {
                self.field = match pages.read_varint(&mut self.pos, self.end)? {
                    0 => self.finish()?,
                    1 => HistoryEnvelopeField::TransactionId,
                    _ => return Err(DbError::Corrupt("history envelope transaction flag is invalid".to_string())),
                };
            }
            HistoryEnvelopeField::TransactionId => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::TransactionTool;
            }
            HistoryEnvelopeField::TransactionTool => {
                self.skip_text(pages, scratch)?;
                self.field = self.finish()?;
            }""", """                self.field = HistoryEnvelopeField::TrailingFlags;
            }
            HistoryEnvelopeField::TrailingFlags => {
                let flags = pages.read_varint(&mut self.pos, self.end)?;
                if flags > 0b11 {
                    return Err(DbError::Corrupt("history envelope trailing flags are invalid".to_string()));
                }
                self.verb = flags & 0b10 != 0;
                self.field = match (flags & 0b01 != 0, self.verb) {
                    (true, _) => HistoryEnvelopeField::TransactionId,
                    (false, true) => HistoryEnvelopeField::Verb,
                    (false, false) => self.finish()?,
                };
            }
            HistoryEnvelopeField::TransactionId => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::TransactionTool;
            }
            HistoryEnvelopeField::TransactionTool => {
                self.skip_text(pages, scratch)?;
                self.field = if self.verb { HistoryEnvelopeField::Verb } else { self.finish()? };
            }
            HistoryEnvelopeField::Verb => {
                self.skip_text(pages, scratch)?;
                self.field = self.finish()?;
            }"""),
    ])
    print("db ok")


db_history_cursor()
