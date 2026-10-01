"""🏷️ One label rule for live, backfilled and reloaded history rows, derived only from persisted facts: a single
undescribed leaf keeps its `SemanticMutation` label, else the authoring verb's registry label (every locale), else the
app's description, else the leaf label with its count; live rows record the same so a reload never relabels them."""
import pathlib

P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs")
text = P.read_text()
BACKFILL_DOC = """        /// 🕰️ Appends a command-log entry for every VCS edit not yet referenced by the log —
        /// covers seeded (`app.seed`), ingested (`ingest_operations*`), and loaded
        /// (`load_document_text`/`load_document_pack`) edits that never passed through `dispatch_emit`.
        /// Invariant: after this runs, every `envelope.vcs.edits` entry is referenced by exactly one
        /// `CommandLogEntry`. Idempotent — re-running finds nothing missing. Always `push_log_entry`
        /// (never `record_command`) — a backfilled edit is always its own distinct row, never folded.
        ///
        /// 🌉️ Hoisted out of a `.map()` closure into an explicit loop (sync — `Iterator::map`
        /// cannot take an async closure, and `OpText::print_op` is genuinely async) — see
        /// `build_history_view`'s sibling comment for the same pattern.
        ///
        /// 🧮️ Same backfill, for the CONFIG store's own edits — a config edit reached via `seed`/
        /// `load_config_pack`/ingest never passes through `dispatch_emit` either.
"""
VERB_LABEL = """        /// 🏷️ The registry label (every locale) of the action or command `verb` names — how a persisted
        /// [`protocol::Edit::verb`] becomes a history label at projection time; `None` for a verb this app does not declare.
        fn verb_label(&self, verb: &str) -> Option<LocalizedLabel> {
            self.registry.get(verb).map(|definition| definition.label.clone()).or_else(|| self.registry.get_command(verb).map(|definition| definition.label.clone()))
        }
"""
AUTHORED = """
        /// 🏷️ The label a row authored by `verb` records: the verb's registry label in every locale, else the app's
        /// `description` as locale-invariant data — the same rule [`backfilled_edit_label`] applies after a reload.
        fn authored_row_label(&self, verb: &str, description: Option<&str>) -> Option<LocalizedLabel> {
            self.verb_label(verb).or_else(|| description.map(LocalizedLabel::data))
        }
"""
pairs = [
    (BACKFILL_DOC + VERB_LABEL + "\n        async fn backfill_command_log(&mut self) {", VERB_LABEL + AUTHORED + "\n" + BACKFILL_DOC + "        async fn backfill_command_log(&mut self) {"),
    ("""    /// 🏷️ A backfilled edit row's label: the app's description, else the registry label of the verb that authored it,
    /// else its first operation's text.
    fn backfilled_edit_label<Mu: protocol::OpText>(edit: &protocol::Edit<Mu>, verb_label: Option<LocalizedLabel>) -> LocalizedLabel {
        match (&edit.description, verb_label) {
            (Some(description), _) => LocalizedLabel::data(description.clone()),
            (None, Some(label)) => label,
            (None, None) => LocalizedLabel::data(edit.forwards.first().map_or_else(|| edit.id.clone(), |op| op.print_op())),
        }
    }""", """    /// 🏷️ A backfilled edit row's label: the registry label of the verb that authored it, else the app's description,
    /// else its first operation's text — the rule a live row records (see `authored_row_label`).
    fn backfilled_edit_label<Mu: protocol::OpText>(edit: &protocol::Edit<Mu>, verb_label: Option<LocalizedLabel>) -> LocalizedLabel {
        verb_label.or_else(|| edit.description.clone().map(LocalizedLabel::data)).unwrap_or_else(|| LocalizedLabel::data(edit.forwards.first().map_or_else(|| edit.id.clone(), |op| op.print_op())))
    }"""),
    ("""            let Some((edit_id, _description)) = edit else { return };""", """            let Some((edit_id, description)) = edit else { return };"""),
    ("""            let verb = mounted.verb.clone();
            if artifact_lane {
                self.record_command(&verb, kind, None, Some(edit_id), None, None);
            } else {
                self.record_command(&verb, kind, None, None, Some(edit_id), None);
            }""", """            let verb = mounted.verb.clone();
            let label = self.authored_row_label(&verb, description.as_deref());
            if artifact_lane {
                self.record_command(&verb, kind, label, Some(edit_id), None, None);
            } else {
                self.record_command(&verb, kind, label, None, Some(edit_id), None);
            }"""),
    ("""                if !(published_window_config && config_edit_id.is_none() && matches!(kind, ActionKind::View)) {
                    self.record_command(verb, kind, description.clone().map(LocalizedLabel::data), None, config_edit_id, None);
                }""", """                if !(published_window_config && config_edit_id.is_none() && matches!(kind, ActionKind::View)) {
                    let label = self.authored_row_label(verb, description.as_deref());
                    self.record_command(verb, kind, label, None, config_edit_id, None);
                }"""),
    ("""            let log_label = description.clone().map(LocalizedLabel::data);""", """            let log_label = self.authored_row_label(verb, description.as_deref());"""),
    ("""            self.record_command(verb, kind, description.clone().map(LocalizedLabel::data), parent_edit_id, config_edit_id, None);""", """            let label = self.authored_row_label(verb, description.as_deref());
            self.record_command(verb, kind, label, parent_edit_id, config_edit_id, None);"""),
    ("""                    (Some(edit), _) if edit.description.is_none() && (op_count == 1 || (entry.action_id == "apply" && verb_label.is_none())) && leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
                    (Some(edit), _) if edit.description.is_none() && verb_label.is_some() => verb_label.expect("verb label checked above"),
                    (None, Some(first)) if !child_edits.is_empty() && child_edits.iter().all(|child| child.description.is_none()) => history_leaf_row_label(&first.label, op_count),
                    _ => entry.label.clone(),""", """                    (Some(edit), _) if edit.description.is_none() && op_count == 1 && leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
                    (Some(edit), _) if verb_label.is_some() || edit.description.is_some() => verb_label.unwrap_or_else(|| LocalizedLabel::data(edit.description.clone().unwrap_or_default())),
                    (Some(_), _) if leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
                    (None, Some(first)) if !child_edits.is_empty() && child_edits.iter().all(|child| child.description.is_none()) => history_leaf_row_label(&first.label, op_count),
                    _ => entry.label.clone(),"""),
]
for old, new in pairs:
    if text.count(new) == 1:
        continue
    assert text.count(old) == 1, (text.count(old), old[:140])
    text = text.replace(old, new)
P.write_text(text)
print("ok")
