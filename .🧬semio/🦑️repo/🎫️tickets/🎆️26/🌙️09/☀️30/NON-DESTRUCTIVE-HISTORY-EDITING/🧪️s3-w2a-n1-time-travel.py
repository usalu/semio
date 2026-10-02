"""🧪️ S3-W2A gap N1 + N15, time-travel half (`🔌️plugin/⏪️time-travel/🦀️.rs`), applied in one write so the file stays
compile-atomic: `TimeTravelPanel.begin_refusal`, the per-operation mutation row builder, and the history mutation pages
(every mutation of a transaction reachable through the row's tree window)."""

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
FILE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs"

EDITS = [
    (
        "    pub rerun_refusal: Option<TimeTravelRefusal>,\n    pub next_problem: Option<String>,\n",
        "    pub rerun_refusal: Option<TimeTravelRefusal>,\n    pub begin_refusal: Option<TimeTravelRefusal>,\n    pub next_problem: Option<String>,\n",
    ),
    (
        "            rerun_refusal: session.rerun_refusal(),\n            next_problem,\n",
        "            rerun_refusal: session.rerun_refusal(),\n            begin_refusal: session.begin_refusal(),\n            next_problem,\n",
    ),
    (
        """    let mut views = Vec::new();
    for (index, op) in ops.iter().enumerate() {
        let id = op.mutation_id.0.as_str();
        let outcome = outcomes.get(id);
        let flagged = op.supersession.is_some() || outcome.is_some_and(|outcome| outcome.worst.is_some());
        if index >= HISTORY_ROW_MUTATION_ROWS && (!flagged || views.len() >= 2 * HISTORY_ROW_MUTATION_ROWS) {
            continue;
        }
        let effective = match op.supersession.map(|supersession| &supersession.replacement) {
            Some(protocol::InputReplacement::Input { payload, .. }) => <Mu as ::protocol::OpBinary>::decode_op(payload).ok(),
            _ => None,
        };
        let shown = effective.as_ref().unwrap_or(op.operation);
        let label = match labelled.get(id).filter(|_| op.supersession.is_none()) {
            Some(previous) => previous.label.clone(),
            None => label_of(shown),
        };
        let editable = !viewer && !shown.may_emit_foreign_steps() && shown.input_schema().is_some();
        let superseded = match op.supersession.map(|supersession| &supersession.replacement) {
            None => false,
            Some(protocol::InputReplacement::Withdrawn) => true,
            Some(protocol::InputReplacement::Input { payload, .. }) => <Mu as ::protocol::OpBinary>::encode_op(op.operation).map_or(true, |original| original != *payload),
        };
        if let Some(effective) = effective {
            effective.retire_cold();
        }
        views.push(MutationView {
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
        });
    }
    views
}
""",
        """    let mut views = Vec::new();
    for (index, op) in ops.iter().enumerate() {
        let outcome = outcomes.get(op.mutation_id.0.as_str()).copied();
        let flagged = op.supersession.is_some() || outcome.is_some_and(|outcome| outcome.worst.is_some());
        if index >= HISTORY_ROW_MUTATION_ROWS && (!flagged || views.len() >= 2 * HISTORY_ROW_MUTATION_ROWS) {
            continue;
        }
        views.push(history_mutation_view_of::<P, Mu>(op, outcome, labelled, &label_of, viewer, store));
    }
    views
}

/// 🧾️ One operation's mutation row — [`history_mutation_views_of`] without the row cap: labelled from its effective input
/// (a label of `labelled`, the previous projection, reused for an operation that was not superseded) with its durable
/// `outcome`; `superseded`/`withdrawn` read the store's effective supersession.
pub(crate) fn history_mutation_view_of<P, Mu>(op: &store::AppliedMutation<'_, Mu>, outcome: Option<&protocol::MutationReplayOutcome>, labelled: &HashMap<&str, &MutationView>, label_of: &impl Fn(&Mu) -> LocalizedLabel, viewer: bool, store: Option<&str>) -> MutationView
where
    Mu: ::protocol::Mutation<P> + ::protocol::OpBinary,
{
    let id = op.mutation_id.0.as_str();
    let effective = match op.supersession.map(|supersession| &supersession.replacement) {
        Some(protocol::InputReplacement::Input { payload, .. }) => <Mu as ::protocol::OpBinary>::decode_op(payload).ok(),
        _ => None,
    };
    let shown = effective.as_ref().unwrap_or(op.operation);
    let label = match labelled.get(id).filter(|_| op.supersession.is_none()) {
        Some(previous) => previous.label.clone(),
        None => label_of(shown),
    };
    let editable = !viewer && !shown.may_emit_foreign_steps() && shown.input_schema().is_some();
    let superseded = match op.supersession.map(|supersession| &supersession.replacement) {
        None => false,
        Some(protocol::InputReplacement::Withdrawn) => true,
        Some(protocol::InputReplacement::Input { payload, .. }) => <Mu as ::protocol::OpBinary>::encode_op(op.operation).map_or(true, |original| original != *payload),
    };
    if let Some(effective) = effective {
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
}

//#region 🔖️MutationPages
/// 🗂️ The node key of the history body's commands section, the window every history row is a row of.
pub const HISTORY_COMMANDS_SECTION_KEY: &str = "framework.history.commands";
/// 🗝️ The node key prefix of one history row (`framework.history.entry.<seq>`), React's and wgpu's `HISTORY_ROW_KEY_PREFIX`.
pub const HISTORY_ROW_KEY_PREFIX: &str = "framework.history.entry.";

/// 📖️ The mutation rows of one history row past its projected ones, as one host window shows them: `rows` are the edit's
/// remaining operations from the `from`-th on, in op order (gap N1).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryMutationPage {
    pub from: usize,
    pub rows: Vec<MutationView>,
}

/// 📚️ The [`HistoryMutationPage`] of every history row whose window shows rows past its projection, keyed by the row's `seq`.
pub type HistoryMutationPages = BTreeMap<u64, HistoryMutationPage>;

/// 📏️ How many mutation rows a history row projects and how many it holds in all (gap N1): the projected ones (the first
/// [`HISTORY_ROW_MUTATION_ROWS`] of its edit and its flagged ones, plus a composed member's rows), then every other
/// operation of its own edit in op order. A row without an applied operation of its own holds nothing further.
pub fn history_row_mutation_extent(entry: &CommandView) -> (usize, usize) {
    let projected = entry.mutations.len();
    let own = entry.mutations.iter().filter(|mutation| mutation.store.is_none()).count();
    let rest = if own == 0 || entry.edit_id.is_none() { 0 } else { entry.op_count.saturating_sub(own) };
    (projected, projected + rest)
}

/// 🪟️ The window path of history row `seq`'s mutation group inside the commands section — what a host files its
/// `TreeWindowRequest` under.
pub fn history_row_window_path(seq: u64) -> String {
    format!("{HISTORY_COMMANDS_SECTION_KEY}{}{HISTORY_ROW_KEY_PREFIX}{seq}", semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR)
}

/// 🔢️ The logical mutation rows `[start, end)` a host's open window request over a group of `total` rows materialises —
/// the slice `TreeWindows` cuts for a requested container (the offset clamped so the window ends on the last row).
pub fn history_row_requested_rows(request: &TreeWindowRequest, total: usize) -> Option<(usize, usize)> {
    if request.open != Some(true) || total == 0 {
        return None;
    }
    let rows = (request.rows as usize).min(UI_BUILT_CHILDREN_MAX);
    let start = (request.offset as usize).min(total.saturating_sub(rows.max(1)));
    Some((start, start + rows.min(total - start)))
}
//#endregion 🔖️MutationPages
""",
    ),
    (
        """        history_mutation_views::<A>(&ops, &outcomes, &labelled)
    }

    /// 🧲️ Resolves every editor row's `snapSource`""",
        """        history_mutation_views::<A>(&ops, &outcomes, &labelled)
    }

    /// 📖️ The mutation rows the history body's windows show past each row's projection (gap N1): for every row whose
    /// mutation group a host opened past its projected rows ([`history_row_mutation_extent`]), the requested slice of its
    /// edit's remaining operations in op order — every mutation of a transaction is reachable however long it is, while the
    /// projection and the wire stay bounded. The store is read only when some window reaches past a projection.
    pub(crate) fn history_mutation_pages(&self, history: &HistoryView, view: &ViewModel) -> HistoryMutationPages {
        let wanted: Vec<(&CommandView, usize, usize)> = history
            .commands
            .iter()
            .filter_map(|entry| {
                let (projected, total) = history_row_mutation_extent(entry);
                let path = history_row_window_path(entry.seq);
                let request = view.tree_windows.iter().find(|request| request.body_key == FRAMEWORK_HISTORY_BODY_KEY && request.node_key == path)?;
                let (start, end) = history_row_requested_rows(request, total)?;
                (end > projected).then(|| (entry, start.max(projected) - projected, end - projected))
            })
            .collect();
        if wanted.is_empty() {
            return HistoryMutationPages::new();
        }
        let applied_ops = self.store.mutation_ops().unwrap_or_default();
        let durable = self.store.mutation_outcomes().unwrap_or_default();
        let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
        let unlabelled = HashMap::new();
        let mut pages = HistoryMutationPages::new();
        for (entry, from, to) in wanted {
            let Some(edit_id) = entry.edit_id.as_deref() else { continue };
            let projected: HashSet<&str> = entry.mutations.iter().map(|mutation| mutation.mutation_id.as_str()).collect();
            let rows = applied_ops
                .iter()
                .filter(|op| op.edit_id == edit_id && !projected.contains(op.mutation_id.0.as_str()))
                .skip(from)
                .take(to - from)
                .map(|op| history_mutation_view_of::<A::Snapshot, A::Mutation>(op, outcomes.get(op.mutation_id.0.as_str()).copied(), &unlabelled, &time_travel_mutation_label::<A>, A::ROLE == AppRole::Viewer, None))
                .collect();
            pages.insert(entry.seq, HistoryMutationPage { from, rows });
        }
        pages
    }

    /// 🧲️ Resolves every editor row's `snapSource`""",
    ),
]


def main():
    text = FILE.read_text(encoding="utf-8")
    for old, new in EDITS:
        if text.count(old) != 1:
            sys.exit(f"anchor not unique ({text.count(old)}): {old[:90]!r}")
        text = text.replace(old, new)
    FILE.write_text(text, encoding="utf-8")
    print(f"applied {len(EDITS)} edits to {FILE.name}")


if __name__ == "__main__":
    main()
