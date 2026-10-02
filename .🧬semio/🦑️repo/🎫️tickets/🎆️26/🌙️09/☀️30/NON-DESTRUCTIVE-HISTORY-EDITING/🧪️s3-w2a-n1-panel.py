"""🧪️ S3-W2A gap N1 + N15, panel half: `ui_history_panel` builds every history row's mutations as a tree window over ALL
of its edit's mutations (projected rows flagged first, then the remaining operations from `HistoryMutationPages`), the Edit
row action is disabled with its reason wherever `Begin` would be refused, and the render site plus every test call site
pass the pages. One run writes `🔌️plugin/🦀️.rs` and the two test files back to back (the signature change and its callers
land together)."""

import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
MAIN = PLUGIN / "🦀️.rs"
TESTS = [PLUGIN / "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs", PLUGIN / "🧪️tests/🔬️app-panel-kit/🦀️.rs"]

EXPORTS = "TimeTravelPanel, HISTORY_ROW_MUTATION_ROWS, TIME_TRAVEL_TURN_WALL_US};"
EXPORTS_NEW = "TimeTravelPanel, HistoryMutationPage, HistoryMutationPages, HISTORY_ROW_MUTATION_ROWS, TIME_TRAVEL_TURN_WALL_US};"

MAIN_EDITS = [
    (
        "pub async fn ui_history_panel(history: &HistoryView, time_travel: Option<&TimeTravelPanel>, remote_replay: Option<&semio_framework::kernel::HistoryRemoteReplay>, controller_id: &str,",
        "pub async fn ui_history_panel(history: &HistoryView, time_travel: Option<&TimeTravelPanel>, remote_replay: Option<&semio_framework::kernel::HistoryRemoteReplay>, pages: &HistoryMutationPages, controller_id: &str,",
        1,
    ),
    (
        """    /// ✏️ A row with applied mutations expands to one child per mutation (the flagged ones first, at most
    /// [`HISTORY_PANEL_MUTATION_ROWS`]): its label, a description naming its state and severity in words (colour is
    /// never the only carrier), tone and icon by severity, and an Edit row action (`historyEditBegin{mutationId}`,
    /// also the row's activation) when it is editable and this is not a viewer.""",
        """    /// ✏️ A row with applied mutations is a tree window over ALL of them (gap N1): its projected ones first (the flagged
    /// ones leading), then every other operation of its edit in op order, read from `pages` for the slice a host window
    /// opened past the projection ([`time_travel::history_row_mutation_extent`]). Each child carries its label, a
    /// description naming its state and severity in words (colour is never the only carrier), tone and icon by severity,
    /// and an Edit row action (`historyEditBegin{mutationId}`, also the row's activation) when it is editable and this is
    /// not a viewer — disabled, naming why, wherever the session would refuse `Begin` (gap N15).""",
        1,
    ),
    (
        """            let mut shown: Vec<&semio_framework::kernel::HistoryMutationEntry> = mutations.iter().filter(|mutation| mutation.worst.is_some() || mutation.edited || mutation.superseded).collect();
            shown.extend(mutations.iter().filter(|mutation| mutation.worst.is_none() && !mutation.edited && !mutation.superseded));
            for mutation in shown.into_iter().take(HISTORY_PANEL_MUTATION_ROWS) {
                builder = builder.try_child(history_panel_mutation_row(mutation, controller_id, locale, read_only)?).map_err(|_| ui_assembly_error("history-panel.mutation-rows"))?;
            }
            builder.try_build().map_err(|_| ui_assembly_error("history-panel.command-build"))
        })?;""",
        """            let mut shown: Vec<&semio_framework::kernel::HistoryMutationEntry> = mutations.iter().filter(|mutation| mutation.worst.is_some() || mutation.edited || mutation.superseded).collect();
            shown.extend(mutations.iter().filter(|mutation| mutation.worst.is_none() && !mutation.edited && !mutation.superseded));
            let (projected, total) = time_travel::history_row_mutation_extent(entry);
            if total == 0 {
                return builder.try_build().map_err(|_| ui_assembly_error("history-panel.command-build"));
            }
            let (page, begin_refusal) = (pages.get(&entry.seq), time_travel.and_then(|panel| panel.begin_refusal));
            tree_window_indexed_item(&windows, builder, id.as_str(), false, total, |index| {
                let paged;
                let mutation = match shown.get(index) {
                    Some(mutation) => *mutation,
                    None => {
                        let view = page.and_then(|page| page.rows.get(index.checked_sub(projected + page.from)?)).ok_or_else(|| ui_assembly_error("history-panel.mutation-page"))?;
                        paged = history_mutation_entry(view, time_travel);
                        &paged
                    }
                };
                history_panel_mutation_row(mutation, controller_id, locale, read_only, begin_refusal)
            })
        })?;""",
        1,
    ),
    (
        """        let commands_section = tree_window_section(&windows, "framework.history.commands", ui_label(""",
        """        let commands_section = tree_window_section(&windows, time_travel::HISTORY_COMMANDS_SECTION_KEY, ui_label(""",
        1,
    ),
    (
        """            let id = UiText::try_format(format_args!("framework.history.entry.{}", entry.seq)).ok_or_else(|| ui_assembly_error("history-panel.command-id"))?;""",
        """            let id = UiText::try_format(format_args!("{}{}", time_travel::HISTORY_ROW_KEY_PREFIX, entry.seq)).ok_or_else(|| ui_assembly_error("history-panel.command-id"))?;""",
        1,
    ),
    (
        """    /// ✏️ Mutation children one history row shows at most (flagged ones first); the wire carries up to
    /// [`HISTORY_ROW_MUTATION_ROWS`].
    pub const HISTORY_PANEL_MUTATION_ROWS: usize = 8;

""",
        "",
        1,
    ),
    (
        """    /// ✏️ One mutation child of a history row: label, state-and-severity description in words, tone and icon by
    /// severity, and Edit (`historyEditBegin{mutationId, store?}`, also its activation) when editable and not a viewer —
    /// `store` names the composed member store a child mutation lives in (design §12).
    fn history_panel_mutation_row(mutation: &semio_framework::kernel::HistoryMutationEntry, controller_id: &str, locale: Locale, read_only: bool) -> UiAssemblyResult<BuiltNode> {""",
        """    /// ✏️ One mutation child of a history row: label, state-and-severity description in words, tone and icon by
    /// severity, and Edit (`historyEditBegin{mutationId, store?}`, also its activation) when editable and not a viewer —
    /// `store` names the composed member store a child mutation lives in (design §12). While the session would refuse
    /// `Begin` (`begin_refusal`: replaying, choosing, finalizing, or a changed draft still open) the Edit action is disabled,
    /// its label names the reason, and the row's activation fires nothing (gap N15).
    fn history_panel_mutation_row(mutation: &semio_framework::kernel::HistoryMutationEntry, controller_id: &str, locale: Locale, read_only: bool, begin_refusal: Option<semio_framework_time_travel::TimeTravelRefusal>) -> UiAssemblyResult<BuiltNode> {""",
        1,
    ),
    (
        """            let edit = row_action(IconName::Edit.as_str(), time_travel::HistoryPanelText::Edit.text(locale), semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, RowActionPlacement::Row)?;
            builder = builder.try_row_action(edit).map_err(|_| ui_assembly_error("history-panel.mutation-actions"))?.target(row_target(controller_id, Some(UiValue::Map(args.finish())), Some(semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID))?);""",
        """            let edit_text = time_travel::HistoryPanelText::Edit.text(locale);
            let (edit, activation) = match begin_refusal {
                None => (row_action(IconName::Edit.as_str(), edit_text, semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, RowActionPlacement::Row)?, Some(semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID)),
                Some(refusal) => {
                    let reason = refusal.label().localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
                    (row_action(IconName::Edit.as_str(), &format!("{edit_text}: {reason}"), semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, RowActionPlacement::Row)?.disabled(true), None)
                }
            };
            builder = builder.try_row_action(edit).map_err(|_| ui_assembly_error("history-panel.mutation-actions"))?.target(row_target(controller_id, Some(UiValue::Map(args.finish())), activation)?);""",
        1,
    ),
    (
        """                let remote_replay = self.remote_replay_status();
                let root = ui_history_panel(history, time_travel.as_ref(), remote_replay.as_ref(), &self.registry.controller_id,""",
        """                let remote_replay = self.remote_replay_status();
                let pages = self.history_mutation_pages(history, view_state);
                let root = ui_history_panel(history, time_travel.as_ref(), remote_replay.as_ref(), &pages, &self.registry.controller_id,""",
        1,
    ),
    (EXPORTS, EXPORTS_NEW, 2),
]

TEST_CALL = re.compile(r'ui_history_panel\((&[A-Za-z_]+), None, None, "ctrl"')


def main():
    main_text = MAIN.read_text(encoding="utf-8")
    for old, new, count in MAIN_EDITS:
        if main_text.count(old) != count:
            sys.exit(f"anchor count {main_text.count(old)} != {count}: {old[:100]!r}")
        main_text = main_text.replace(old, new)
    if "HISTORY_PANEL_MUTATION_ROWS" in main_text:
        sys.exit("HISTORY_PANEL_MUTATION_ROWS still referenced")
    tests = []
    for path in TESTS:
        text = path.read_text(encoding="utf-8")
        text, count = TEST_CALL.subn(r'ui_history_panel(\1, None, None, &Default::default(), "ctrl"', text)
        if count == 0 or text.count("ui_history_panel(") != count:
            sys.exit(f"{path.name}: {count} test calls rewritten, {text.count('ui_history_panel(')} present")
        tests.append((path, text, count))
    MAIN.write_text(main_text, encoding="utf-8")
    for path, text, _ in tests:
        path.write_text(text, encoding="utf-8")
    print(f"main: {len(MAIN_EDITS)} edits; tests: {[count for _, _, count in tests]} calls")


if __name__ == "__main__":
    main()
