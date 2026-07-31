//#region 🔖TextFormat
/// @emoji 📄 The two files a textual VCS document is made of: the DSL text (initial projection) and
/// the append-only op log (every edit ever created, forwards-only — see {@link parse_document_text}).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentTextFiles {
    pub dsl: String,
    pub ops: String,
}

/// @emoji 🧩 The result of loading a document from text: the reconstructed envelope plus the live
/// projection folded from every edit, so a caller never has to replay again after loading.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedDocumentText<P, Operation> {
    pub envelope: DocumentEnvelope<P, Operation>,
    pub projection: P,
}

//#region 🔖OpsHeaderGrammar
/// @emoji 🖋️ One `by=[...]` list entry on a `checkpoint` header line: id then name, both positional
/// (bare-preferred, quoted only when needed — e.g. a name containing a space). `Author::avatar` is
/// never part of the textual `.ops` format (this mirrors the pre-derive printer, which never carried
/// it either — see {@link Author}).
#[derive(Clone, Debug, PartialEq, DslRecord)]
struct OpsAuthor {
    #[dsl(positional)]
    id: String,
    #[dsl(positional)]
    name: String,
}

impl From<&Author> for OpsAuthor {
    fn from(author: &Author) -> Self {
        Self { id: author.id.clone(), name: author.name.clone() }
    }
}

impl From<OpsAuthor> for Author {
    fn from(author: OpsAuthor) -> Self {
        Self { id: author.id, name: author.name, avatar: None }
    }
}

/// @emoji 🧾 One `.ops` header/structural line — `doc`/`edit`/`change`/`checkpoint`/`alternative`/
/// `active` — re-derived directly on the `dsl_schema` grammar engine (`#[derive(DslOps)]` generates
/// `OpText::parse_op`/`print_op` from this declaration; see {@link print_edit_lines}/
/// {@link print_document_text}/{@link parse_document_text}, its only callers). Sigil-free lowercase
/// keywords (bare `doc`, never `@doc` — `@` is reserved for connection points everywhere else in the
/// unified DSL syntax); `id` is always the first positional field on every line; every other field is
/// a plain `key=value` attribute that is simply OMITTED when absent (no more `-` placeholder
/// sentinel); `edits`/`changes`/`checkpoints`/`by` are real DSL lists (`by=[ u1 "Ueli Saluz" ]`), not
/// comma-joined, percent-escaped strings.
#[derive(Clone, Debug, PartialEq, DslOps)]
enum OpsHeaderLine {
    Doc {
        #[dsl(positional)]
        id: String,
        schema: String,
    },
    Edit {
        #[dsl(positional)]
        id: String,
        started: String,
        actor: Option<String>,
        finished: Option<String>,
        key: Option<String>,
        description: Option<String>,
    },
    Change {
        #[dsl(positional)]
        id: String,
        saved: String,
        edits: Vec<String>,
        description: Option<String>,
    },
    Checkpoint {
        #[dsl(positional)]
        id: String,
        at: String,
        changes: Vec<String>,
        parent: Option<String>,
        by: Vec<OpsAuthor>,
        message: Option<String>,
    },
    Alternative {
        #[dsl(positional)]
        id: String,
        name: String,
        checkpoints: Vec<String>,
    },
    Active {
        #[dsl(positional)]
        id: String,
    },
}
//#endregion 🔖OpsHeaderGrammar

/// @emoji 📤 Prints one edit as an `edit ...` header line followed by one two-space-indented
/// `print_op` line per forward operation — the hot-path append unit for the op log. Backwards
/// operations and per-operation metadata are never serialized; they are recomputed during
/// {@link parse_document_text}'s load replay.
pub fn print_edit_lines<Operation: OpText>(edit: &Edit<Operation>) -> Result<String, VcsError> {
    let header = OpsHeaderLine::Edit {
        id: edit.id.clone(),
        started: edit.started_at.clone(),
        actor: edit.actor.clone(),
        finished: edit.finished_at.clone(),
        key: edit.coalesce_key.clone(),
        description: edit.description.clone(),
    };
    let mut out = header.print_op();
    out.push('\n');
    for operation in &edit.forwards {
        let printed = operation.print_op();
        if printed.contains('\n') {
            return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
        }
        out.push_str("  ");
        out.push_str(&printed);
        out.push('\n');
    }
    Ok(out)
}

/// @emoji 📤 Builds just the op-log half of a textual/pack document — `doc` header, every edit ever
/// created as an `edit` block, then `change`/`checkpoint`/`alternative`/`active` records. Shared by
/// `print_document_text` and `print_document_pack`: the op-log grammar never touches
/// `initial_projection`, so it is provably format-invariant and both printers thin out to this plus
/// their own initial-projection encoding.
fn print_ops_log<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> Result<String, VcsError>
where
    Operation: OpText,
{
    let mut ops = String::new();
    ops.push_str(&OpsHeaderLine::Doc { id: envelope.id.clone(), schema: envelope.schema.clone() }.print_op());
    ops.push('\n');
    for edit in &envelope.vcs.edits {
        ops.push_str(&print_edit_lines(edit)?);
    }
    for change in &envelope.vcs.changes {
        let header = OpsHeaderLine::Change {
            id: change.id.clone(),
            saved: change.saved_at.clone(),
            edits: change.edit_ids.clone(),
            description: change.description.clone(),
        };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    for checkpoint in &envelope.vcs.checkpoints {
        let header = OpsHeaderLine::Checkpoint {
            id: checkpoint.id.clone(),
            at: checkpoint.timestamp.clone(),
            changes: checkpoint.change_ids.clone(),
            parent: checkpoint.parent_id.clone(),
            by: checkpoint.authors.iter().map(OpsAuthor::from).collect(),
            message: checkpoint.message.clone(),
        };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    for alternative in &envelope.vcs.alternatives {
        let header = OpsHeaderLine::Alternative {
            id: alternative.id.clone(),
            name: alternative.name.clone(),
            checkpoints: alternative.checkpoint_ids.clone(),
        };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    if let Some(active_id) = &envelope.active_alternative_id {
        ops.push_str(&OpsHeaderLine::Active { id: active_id.clone() }.print_op());
        ops.push('\n');
    }
    Ok(ops)
}

/// @emoji 📤 Prints the full textual VCS document: the DSL text (initial projection) and the complete
/// op log (`doc` header, every edit ever created as an `edit` block, then `change`/`checkpoint`/
/// `alternative`/`active` records). Replaces the JSON envelope as the canonical persisted form.
pub fn print_document_text<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> Result<DocumentTextFiles, VcsError>
where
    P: DocumentDsl,
    Operation: OpText,
{
    let dsl = envelope.vcs.initial_projection.print_dsl();
    let ops = print_ops_log(envelope)?;
    Ok(DocumentTextFiles { dsl, ops })
}

/// @emoji 📤 Pack counterpart of `print_document_text`: identical op-log body (`print_ops_log`), but
/// the initial projection is encoded to pack bytes (`DocumentPack::encode_pack`) instead of printed
/// to DSL text.
pub fn print_document_pack<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> Result<DocumentPackFiles, VcsError>
where
    P: DocumentPack,
    Operation: OpText,
{
    let pack = envelope.vcs.initial_projection.encode_pack();
    let ops = print_ops_log(envelope)?;
    Ok(DocumentPackFiles { pack, ops })
}

/// @emoji 📥 Replays `ops` against an already-obtained `initial_projection` — the parse-independent
/// tail shared by `parse_document_text` (which obtains the projection via `P::parse_dsl`) and
/// `parse_document_pack` (via `P::decode_pack`). Every `edit` in the log is treated as applied, in
/// file order — mirroring the existing JSON `load_document` semantics (undo/redo position and
/// checkout-alternative state are runtime-only and are not restored across a save/load cycle either
/// way).
fn replay_ops<P, Operation>(initial_projection: P, ops: &str) -> Result<ParsedDocumentText<P, Operation>, TextError>
where
    P: Clone,
    Operation: OpText + crate::Operation<P>,
{
    let mut schema = String::new();
    let mut id = String::new();
    let mut edits: Vec<Edit<Operation>> = Vec::new();
    let mut changes: Vec<Change> = Vec::new();
    let mut checkpoints: Vec<Checkpoint> = Vec::new();
    let mut alternatives: Vec<Alternative> = Vec::new();
    let mut active_alternative_id: Option<String> = None;
    let mut projection = initial_projection.clone();

    /// @emoji 🕰️ An `edit` header line's fields, held until its trailing indented op-lines are all
    /// read (its final `Edit` can only be built once `forwards` — and therefore `backwards`/
    /// `operation_meta`, both computed by replaying against `projection` — are known).
    struct PendingEdit {
        line_no: u32,
        id: String,
        actor: Option<String>,
        started_at: String,
        finished_at: Option<String>,
        coalesce_key: Option<String>,
        description: Option<String>,
    }
    let mut pending_edit: Option<PendingEdit> = None;
    let mut pending_forwards: Vec<Operation> = Vec::new();

    let flush_pending_edit =
        |pending_edit: &mut Option<PendingEdit>, pending_forwards: &mut Vec<Operation>, edits: &mut Vec<Edit<Operation>>, projection: &mut P| -> Result<(), TextError> {
            let Some(header) = pending_edit.take() else {
                return Ok(());
            };
            let forwards = std::mem::take(pending_forwards);
            let mut backwards = Vec::with_capacity(forwards.len());
            let mut operation_meta = Vec::with_capacity(forwards.len());
            for operation in &forwards {
                operation.validate(projection).map_err(|message| TextError::new(message, TextSpan::at(header.line_no, 1)))?;
                let mut back = operation.backwards(projection);
                back.reverse();
                backwards.extend(back);
                operation_meta.push(OperationMeta {
                    operation_id: Some(operation.operation_id().unwrap_or_else(|| OperationId(create_document_vcs_id("operation")))),
                    dependencies: operation.dependencies(),
                    base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                    author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                    timestamp: operation.timestamp().unwrap_or_else(|| protocol::HybridLogicalTimestamp::new(0, now_ms())),
                    undo_policy: operation.undo_policy(),
                    payload_hash: None,
                });
                *projection = apply_operation(projection, operation);
            }
            edits.push(Edit {
                id: header.id,
                actor: header.actor,
                forwards,
                backwards,
                operation_meta,
                description: header.description,
                coalesce_key: header.coalesce_key,
                sequence_number: edits.len() as i32 + 1,
                started_at: header.started_at,
                finished_at: header.finished_at,
            });
            Ok(())
        };

    for (index, raw_line) in ops.lines().enumerate() {
        let line_no = index as u32 + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if raw_line.starts_with("  ") && pending_edit.is_some() {
            let operation = Operation::parse_op(trimmed)
                .map_err(|error| TextError::new(error.message, TextSpan::at(line_no, error.span.column)))?;
            pending_forwards.push(operation);
            continue;
        }
        flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits, &mut projection)?;
        let line = OpsHeaderLine::parse_op(trimmed).map_err(|error| TextError::new(error.message, TextSpan::at(line_no, error.span.column)))?;
        match line {
            OpsHeaderLine::Doc { id: doc_id, schema: doc_schema } => {
                schema = doc_schema;
                id = doc_id;
            }
            OpsHeaderLine::Edit { id: edit_id, started, actor, finished, key, description } => {
                pending_edit = Some(PendingEdit { line_no, id: edit_id, actor, started_at: started, finished_at: finished, coalesce_key: key, description });
                pending_forwards = Vec::new();
            }
            OpsHeaderLine::Change { id: change_id, saved, edits: edit_ids, description } => {
                changes.push(Change { id: change_id, edit_ids, description, saved_at: saved });
            }
            OpsHeaderLine::Checkpoint { id: checkpoint_id, at, changes: change_ids, parent, by, message } => {
                checkpoints.push(Checkpoint {
                    id: checkpoint_id,
                    change_ids,
                    parent_id: parent,
                    authors: by.into_iter().map(Author::from).collect(),
                    message,
                    timestamp: at,
                });
            }
            OpsHeaderLine::Alternative { id: alternative_id, name, checkpoints: checkpoint_ids } => {
                alternatives.push(Alternative { id: alternative_id, name, checkpoint_ids });
            }
            OpsHeaderLine::Active { id: active_id } => {
                active_alternative_id = Some(active_id);
            }
        }
    }
    flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits, &mut projection)?;

    let envelope = DocumentEnvelope {
        schema,
        id,
        vcs: DocumentVcs {
            initial_projection,
            edits,
            changes,
            checkpoints,
            alternatives,
        },
        backbone: None,
        active_alternative_id,
    };
    let last_operation = envelope.vcs.edits.last().and_then(|edit| edit.forwards.last());
    let (projection, _conflicts) = reconcile_with_last(last_operation, projection);
    Ok(ParsedDocumentText { envelope, projection })
}

/// @emoji 📥 Parses the textual VCS document back into an envelope plus its live (fully-replayed)
/// projection — obtains the initial projection via `P::parse_dsl` then shares `replay_ops`.
pub fn parse_document_text<P, Operation>(dsl: &str, ops: &str) -> Result<ParsedDocumentText<P, Operation>, TextError>
where
    P: Clone + DocumentDsl,
    Operation: OpText + crate::Operation<P>,
{
    let initial_projection = P::parse_dsl(dsl)?;
    replay_ops(initial_projection, ops)
}

/// @emoji 📥 Pack counterpart of `parse_document_text`: obtains the initial projection via
/// `DocumentPack::decode_pack` instead of `DocumentDsl::parse_dsl`, then shares the same
/// `replay_ops` tail.
pub fn parse_document_pack<P, Operation>(pack: &[u8], ops: &str) -> Result<ParsedDocumentText<P, Operation>, TextError>
where
    P: Clone + DocumentPack,
    Operation: OpText + crate::Operation<P>,
{
    let initial_projection = P::decode_pack(pack).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
    replay_ops(initial_projection, ops)
}
//#endregion 🔖TextFormat
