"""🏷️ Projects history labels from the persisted locale-neutral `Edit.verb`: backfill (every reload, seed and ingest) and
`build_history_view` resolve it through the authoring app's registry, so a row keeps its en/de label after a reload."""
import pathlib

P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs")
text = P.read_text()
pairs = [
    ("""            let label = match label {
                Some(label) => label,
                None => match self.registry.get(action_id) {
                    Some(def) => def.label.clone(),
                    None => match self.registry.get_command(action_id) {
                        Some(def) => def.label.clone(),
                        None => LocalizedLabel::data(action_id),
                    },
                },
            };
            self.push_log_entry(CommandLogAppend { action_id, label, kind, edit_id, config_edit_id, transition_id: None, timestamp: None, inverse });""", """            let label = label.or_else(|| self.verb_label(action_id)).unwrap_or_else(|| LocalizedLabel::data(action_id));
            self.push_log_entry(CommandLogAppend { action_id, label, kind, edit_id, config_edit_id, transition_id: None, timestamp: None, inverse });"""),
    ("""        async fn backfill_command_log(&mut self) {
            self.refresh_supersede_ledger();""", """        /// 🏷️ The registry label (every locale) of the action or command `verb` names — how a persisted
        /// [`protocol::Edit::verb`] becomes a history label at projection time; `None` for a verb this app does not declare.
        fn verb_label(&self, verb: &str) -> Option<LocalizedLabel> {
            self.registry.get(verb).map(|definition| definition.label.clone()).or_else(|| self.registry.get_command(verb).map(|definition| definition.label.clone()))
        }

        async fn backfill_command_log(&mut self) {
            self.refresh_supersede_ledger();"""),
    ("""            let mut missing: Vec<(CommandLogAppendKey, LocalizedLabel, String)> = Vec::new();
            for edit in self.store.envelope().vcs.edits.iter().filter(|edit| !logged.contains(edit.id.as_str())) {
                let at = edit.mutation_meta.first().map(|meta| meta.timestamp);
                while let Some((index, _)) = transitions.next_if(|(_, timestamp)| at.is_some_and(|at| *timestamp < at)) {
                    let record = &self.supersedes.records[index];
                    missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.supersede_row_label(index), record.timestamp.physical_ms.to_string()));
                }
                let label = match edit.description.clone() {
                    Some(description) => description,
                    None => match edit.forwards.first() {
                        Some(op) => op.print_op(),
                        None => edit.id.clone(),
                    },
                };
                missing.push((CommandLogAppendKey::Edit(edit.id.clone()), LocalizedLabel::data(label), edit.started_at.clone()));
            }
            for (index, _) in transitions {
                let record = &self.supersedes.records[index];
                missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.supersede_row_label(index), record.timestamp.physical_ms.to_string()));
            }
            for (key, label, timestamp) in missing {
                match key {
                    CommandLogAppendKey::Edit(edit_id) => self.push_log_entry(CommandLogAppend { action_id: "apply", label, kind: ActionKind::Mutation, edit_id: Some(edit_id), config_edit_id: None, transition_id: None, timestamp: Some(timestamp), inverse: None }),
                    CommandLogAppendKey::Transition(transition_id) => self.push_log_entry(CommandLogAppend { action_id: semio_framework::HISTORY_EDIT_COMMIT_ACTION_ID, label, kind: ActionKind::History, edit_id: None, config_edit_id: None, transition_id: Some(transition_id), timestamp: Some(timestamp), inverse: None }),
                }
            }""", """            let mut missing: Vec<(CommandLogAppendKey, LocalizedLabel, String)> = Vec::new();
            for edit in self.store.envelope().vcs.edits.iter().filter(|edit| !logged.contains(edit.id.as_str())) {
                let at = edit.mutation_meta.first().map(|meta| meta.timestamp);
                while let Some((index, _)) = transitions.next_if(|(_, timestamp)| at.is_some_and(|at| *timestamp < at)) {
                    let record = &self.supersedes.records[index];
                    missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.supersede_row_label(index), record.timestamp.physical_ms.to_string()));
                }
                let label = Self::backfilled_edit_label(edit, self.verb_label_of(edit.verb.as_deref()));
                missing.push((CommandLogAppendKey::Edit { edit_id: edit.id.clone(), verb: edit.verb.clone() }, label, edit.started_at.clone()));
            }
            for (index, _) in transitions {
                let record = &self.supersedes.records[index];
                missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.supersede_row_label(index), record.timestamp.physical_ms.to_string()));
            }
            for (key, label, timestamp) in missing {
                match key {
                    CommandLogAppendKey::Edit { edit_id, verb } => self.push_log_entry(CommandLogAppend { action_id: verb.as_deref().unwrap_or("apply"), label, kind: ActionKind::Mutation, edit_id: Some(edit_id), config_edit_id: None, transition_id: None, timestamp: Some(timestamp), inverse: None }),
                    CommandLogAppendKey::Transition(transition_id) => self.push_log_entry(CommandLogAppend { action_id: semio_framework::HISTORY_EDIT_COMMIT_ACTION_ID, label, kind: ActionKind::History, edit_id: None, config_edit_id: None, transition_id: Some(transition_id), timestamp: Some(timestamp), inverse: None }),
                }
            }"""),
    ("""            for edit in self.config_store.envelope().vcs.edits.iter().filter(|edit| !logged_config.contains(edit.id.as_str())) {
                let label = match edit.description.clone() {
                    Some(description) => description,
                    None => match edit.forwards.first() {
                        Some(op) => op.print_op(),
                        None => edit.id.clone(),
                    },
                };
                missing_config.push((edit.id.clone(), LocalizedLabel::data(label), edit.started_at.clone()));
            }""", """            for edit in self.config_store.envelope().vcs.edits.iter().filter(|edit| !logged_config.contains(edit.id.as_str())) {
                let label = Self::backfilled_edit_label(edit, self.verb_label_of(edit.verb.as_deref()));
                missing_config.push((edit.id.clone(), label, edit.started_at.clone()));
            }"""),
    ("""                let leaf_label = entry.edit_id.as_deref().and_then(|edit_id| ops_by_edit.get(edit_id)).and_then(|ops| ops.first()).and_then(|op| A::mutation_label(op.operation));
                let label = match (edit, mutations.first()) {
                    (Some(edit), _) if edit.description.is_none() && tool_run_label.is_some() => tool_run_label.expect("tool run label checked above"),
                    (Some(edit), Some(first)) if transaction.is_some() && edit.description.is_none() => history_leaf_row_label(&first.label, op_count),
                    (Some(edit), _) if edit.description.is_none() && (op_count == 1 || entry.action_id == "apply") && leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
                    _ => entry.label.clone(),
                };""", """                let leaf_label = entry.edit_id.as_deref().and_then(|edit_id| ops_by_edit.get(edit_id)).and_then(|ops| ops.first()).and_then(|op| A::mutation_label(op.operation));
                let verb_label = edit.and_then(|edit| self.verb_label_of(edit.verb.as_deref()));
                let label = match (edit, mutations.first()) {
                    (Some(edit), _) if edit.description.is_none() && tool_run_label.is_some() => tool_run_label.expect("tool run label checked above"),
                    (Some(edit), Some(first)) if transaction.is_some() && edit.description.is_none() => history_leaf_row_label(&first.label, op_count),
                    (Some(edit), _) if edit.description.is_none() && (op_count == 1 || (entry.action_id == "apply" && verb_label.is_none())) && leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
                    (Some(edit), _) if edit.description.is_none() && verb_label.is_some() => verb_label.expect("verb label checked above"),
                    _ => entry.label.clone(),
                };"""),
    ("""            self.config_store.dispatch(config_command).await.map_err(|error| error.into_fault())?;
                let amended_same_config_edit""", """            self.config_store.set_authoring_verb(Some(verb.to_string()));
                let dispatched = self.config_store.dispatch(config_command).await;
                self.config_store.set_authoring_verb(None);
                dispatched.map_err(|error| error.into_fault())?;
                let amended_same_config_edit"""),
]
for old, new in pairs:
    if text.count(new) == 1:
        continue
    count = text.count(old)
    assert count == 1, (count, old[:160])
    text = text.replace(old, new)
P.write_text(text)
print("ok")
