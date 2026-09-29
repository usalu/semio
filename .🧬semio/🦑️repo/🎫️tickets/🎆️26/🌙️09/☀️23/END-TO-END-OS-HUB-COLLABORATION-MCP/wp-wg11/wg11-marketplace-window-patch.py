#!/usr/bin/env python3
"""🛍️ WG11 session 14d — T6 row 8: the wgpu Marketplace as ONE windowed Tree section on U6's row target (coordinator 07:4x/08:1x:
"windowing (bounded pages) built on U6's (a') row-target model … no inline-toolbar interim, no raised ceiling").

ORDER: after U6's (a') row-target set (T6 row 6: `RowTarget`, `RowAction.disabled`, the post-U6 `PanelProjection::tree_item` it
edits) AND after `wg11-tree-window-shell-patch.py` (the Shell's `tree_windows` scheduler this reads). `--base <tree>` dry-runs the
anchors against another tree (e.g. U6's overlay) before U6 lands.

Measured: `panel 'framework.marketplace' exceeds 128 document nodes` (WG11 overlay build 4, the Display fixture's shell) — every
plugin was a row PLUS one nested TreeItem per verb (folded into an inline Toolbar of Buttons by the projection), so the roster
alone outgrew the retained ceiling and the panel faulted to nothing.

1. `build_marketplace_ui`: every plugin/extension row carries its verbs as ROW ACTIONS on ONE target (`framework` + `{pluginId}` /
   `{extensionId, enabled: !enabled}`); a verb the row cannot run is DISABLED, not absent (session program's Uninstall, a plugin
   mid-install, a failed extension's Enable — React's `disabled` buttons). The roster (`marketplace_roster`) is ONE windowed
   section `framework.marketplace.source.local`: it materialises the window the tree window observer reported
   (`TreeWindowScheduler::window_of`, one viewport before the first report) inside `MARKETPLACE_WINDOW_NODE_BUDGET`
   (the body budget less the section), and the retained layout pitches the rest as spacers.
2. `PanelProjection::tree_item` projects a row's `actions` onto the contract's `row_actions` + ONE `RowTarget` (the row's activation,
   or its first action's controller and argument map); an action outside that target is a projection error, never a second map.
3. `publish_shell_panel_document` publishes a windowed body as a scroll root (`panel_ui_scroll_records`) — the window streams
   against it.
Laws (`🖥️wgpu-display-conflicts-marketplace`): the action lane as row actions + one target; the session program's Uninstall is the
projected accessibility button `<row>::row-action::1` with `disabled == true`, never actionable (U6's law); the windowed roster
projects inside the ceiling, first paint within budget, and the observer's window (the LAST row) is the slice republished; Store
records keep React's ids with their verbs and next state in the target.

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/marketplace-window/` and applies;
`--revert` restores.
"""

import difflib
import shutil
import sys
from pathlib import Path

LIVE = Path("/Users/ueli/Documents/semio")
ROOT = Path(sys.argv[sys.argv.index("--base") + 1]) if "--base" in sys.argv else LIVE
WORK = Path("/Users/ueli" + "/Documents/semio/.tmp-ticket/wp-wg11/marketplace")
ELEMENTS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
SHELL = ELEMENTS / "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
LAWS = ELEMENTS / "🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs"
BACKUP = LIVE / ".🧬semio/🌐hub/s14-wg11-backup/marketplace-window"

BUILDER_START = "    /// 🛍️ The `framework.marketplace` leaf's body — the wgpu twin of React's `buildMarketplaceTree`:\n"
BUILDER_END = "    /// 💬️ Sends the draft to the connected agent"
LAWS_START = "/// 🛍️ **The Marketplace action lane.**"
LAWS_END = "#[test]\nfn conflict_resolution_buttons_are_inline_controls_before_row_selection() {"

SHELL_EDITS = [
    (
        '''const FRAMEWORK_MARKETPLACE_TAB_ID: &str = "framework.marketplace";
''',
        '''const FRAMEWORK_MARKETPLACE_TAB_ID: &str = "framework.marketplace";
/// 🛍️ The Marketplace's windowed roster section — React's `framework.marketplace.source.<sourceId>` for this renderer's one source.
const MARKETPLACE_LOCAL_SECTION_ID: &str = "framework.marketplace.source.local";
/// 🛍️ The node ledger one Marketplace window may materialise — the body-wide tree window budget less the windowed section itself.
const MARKETPLACE_WINDOW_NODE_BUDGET: usize = ui_contract::TREE_WINDOW_BODY_NODE_BUDGET - 1;
''',
    ),
    (
        '''        let props = ui_contract::TreeItemProps {
            label: measure_label(item.label.as_str()),''',
        '''        let (row_actions, target) = self.tree_item_row_target(item)?;
        let props = ui_contract::TreeItemProps {
            label: measure_label(item.label.as_str()),''',
    ),
    (
        '''            row_actions: Default::default(),
            target: item.action.as_ref().map(measure_row_target).transpose()?,
        };''',
        '''            row_actions,
            target,
        };''',
    ),
    (
        '''    /// 🎛️ Accept/Discard (and Marketplace Install/Reload/Uninstall) leaves authored as nested''',
        '''    /// 🎯️ A row's trailing actions as the contract's ONE target plus verbs (U6's row model): the target is the row's own click — its
    /// activation — or, for an action-only row, its first action's controller and argument map; every action must fire on that
    /// same controller and map and names only its verb, so N actions cost N verbs, never N argument maps.
    #[allow(clippy::type_complexity, reason = "the two halves of one row's projected action model")]
    fn tree_item_row_target(&self, item: &UiTreeItemNode) -> Result<(ui_contract::UiFixedList<ui_contract::RowAction>, Option<ui_contract::RowTarget>), String> {
        let actions = item.actions.as_deref().unwrap_or_default();
        let Some(anchor) = item.action.as_ref().or_else(|| actions.first().map(|action| &action.action)) else { return Ok((ui_contract::UiFixedList::default(), None)) };
        let mut target = measure_row_target(anchor)?;
        if item.action.is_none() {
            target.activation = None;
        }
        let mut row_actions = ui_contract::UiFixedList::default();
        for action in actions {
            if action.action.controller_id != anchor.controller_id || action.action.args != anchor.args {
                return Err(format!("panel '{}' tree row '{}' action '{}' fires outside the row's one target", self.surface_id, item.id, action.action.action));
            }
            let verb = UiText::try_from_str(&action.action.action).ok_or_else(|| format!("panel '{}' tree row '{}' verb exceeds the retained contract", self.surface_id, item.id))?;
            let placement = match action.placement() {
                ui_wgpu::wgpu::component::ui::UiTreeActionPlacement::Row => ui_contract::RowActionPlacement::Row,
                ui_wgpu::wgpu::component::ui::UiTreeActionPlacement::Menu => ui_contract::RowActionPlacement::Menu,
            };
            let row_action = ui_contract::RowAction { icon: UiText::clipped(action.icon_id.as_str()), label: action.label.as_ref().map(|label| measure_label(label.as_str())), verb, placement, disabled: action.disabled };
            row_actions.try_push(row_action).map_err(|_| format!("panel '{}' tree row '{}' packs more row actions than one record admits", self.surface_id, item.id))?;
        }
        Ok((row_actions, Some(target)))
    }

    /// 🎛️ Accept/Discard leaves authored as nested''',
    ),
    (
        '''        let records = panel_ui_records(tab_id, &node)?;
        self.publish_surface_records(tab_id, records).map(Some)
    }

    /// ♻️ Replaces one mounted host-owned retained panel''',
        '''        let records = if ui_node_is_windowed(&node) { panel_ui_scroll_records(tab_id, &node)? } else { panel_ui_records(tab_id, &node)? };
        self.publish_surface_records(tab_id, records).map(Some)
    }

    /// ♻️ Replaces one mounted host-owned retained panel''',
    ),
    (
        '''fn display_panel_body(panel_id: &str, sections: Vec<UiTreeSectionNode>) -> UiNode {''',
        '''/// 🪟️ Whether a shell-owned body streams a windowed Tree section — such a body IS its scroll viewport (the window streams against
/// it, its spacers give it the whole extent), so it publishes as a scroll root (`panel_ui_scroll_records`).
fn ui_node_is_windowed(node: &UiNode) -> bool {
    match node {
        UiNode::Tree(tree) => tree.sections.iter().any(|section| section.window.is_some()),
        UiNode::Stack(stack) => stack.children.iter().any(ui_node_is_windowed),
        _ => false,
    }
}

fn display_panel_body(panel_id: &str, sections: Vec<UiTreeSectionNode>) -> UiNode {''',
    ),
]


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def region(path: Path, source: str, start: str, end: str, replacement: str) -> str:
    begin = source.find(start)
    finish = source.find(end, begin)
    if source.count(start) != 1 or begin < 0 or finish < 0:
        sys.exit(f"{path.parent.name}/{path.name}: region markers moved")
    return source[:begin] + replacement + source[finish:]


def plans():
    shell = SHELL.read_text(encoding="utf-8")
    shell_after = region(SHELL, shell, BUILDER_START, BUILDER_END, (WORK / "build-marketplace-new.rs").read_text(encoding="utf-8"))
    shell_after = replaced(SHELL, shell_after, SHELL_EDITS)
    laws = LAWS.read_text(encoding="utf-8")
    laws_after = region(LAWS, laws, LAWS_START, LAWS_END, (WORK / "marketplace-laws-new.rs").read_text(encoding="utf-8") + "\n")
    return [(SHELL, shell, shell_after), (LAWS, laws, laws_after)]


def main():
    if "--revert" in sys.argv:
        for path in (SHELL, LAWS):
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print("REVERTED: 2 files restored from backups")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: 2 files against {ROOT} (crate: semio-framework-os-renderer-wgpu; after U6 row target + tree-window-shell)")


if __name__ == "__main__":
    main()
