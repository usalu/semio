/// 🧬️ One persisted history edit decoded exactly as [`parse_decoded_document_spr`] decodes each of its edits, but borrowing the record, so the resumable persisted merge can decode a document's edits one at a time.
pub async fn decode_history_edit_record<P, Mutation>(index: usize, history_edit: &semio_os_spr::HistoryEdit, schema: &str) -> Result<Edit<Mutation>, TextError>
where
    P: Clone + 'static,
    Mutation: OpText + OpBinary + super::Mutation<P> + semio_os_spr::ColdMutation<P> + 'static,
{
    fn refusal(message: String) -> TextError { TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, TextSpan::at(1, 1)) }
    fn decode_payload<P, Mutation: OpText + OpBinary + super::Mutation<P>>(payload: &semio_os_spr::OpPayload) -> Result<Mutation, TextError> {
        match (&payload.binary, &payload.text) {
            (Some(bytes), _) => Mutation::decode_op(bytes).map_err(|error| refusal(error.to_string())),
            (None, Some(text)) => Mutation::parse_op(text),
            (None, None) => Err(refusal("op payload carries neither binary nor text".to_string())),
        }
    }
    let edit_id = history_edit.id.clone();
    if let Some(lane) = &history_edit.lane {
        HistoryLane::from_value(DslValue::String(lane.clone())).map_err(|error| refusal(format!("history edit {edit_id} names an unknown lane: {error}")))?;
    }
    let forwards = history_edit.ops.iter().map(decode_payload::<P, Mutation>).collect::<Result<Vec<_>, _>>()?;
    let inverse = history_edit.inverse.iter().map(decode_payload::<P, Mutation>).collect::<Result<Vec<_>, _>>()?;
    let metas = history_edit.meta.as_ref().ok_or_else(|| refusal(format!("history edit {edit_id} has no authoritative operation metadata")))?;
    if metas.len() != forwards.len() {
        return Err(refusal(format!("history edit {edit_id} has {} metadata entries for {} forward operations", metas.len(), forwards.len())));
    }
    let mutation_meta = metas
        .iter()
        .cloned()
        .map(|meta| mutation_meta_from_history_op_meta(meta).map(|(meta, _messages)| meta))
        .collect::<Result<Vec<_>, String>>()
        .map_err(refusal)?;
    let mut edit = Edit {
        id: history_edit.id.clone(),
        actor: history_edit.actor.clone(),
        forwards,
        inverse: inverse.into(),
        mutation_meta,
        verb: history_edit.verb.clone(),
        line: history_edit.line.clone(),
        sequence_number: index as i32 + 1,
        started_at: history_edit.started_at.clone(),
        finished_at: history_edit.finished_at.clone(),
    };
    stamp_edit_semantics::<P, Mutation>(&mut edit, schema);
    Ok(edit)
}

