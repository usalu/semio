# w3-command-palette — final report

**Status: Done.** Native tests pass clean (147/147, 0 failed — includes 13 new tests plus other concurrent sessions' additions). Wasm32 `cargo check` has 2 pre-existing errors, confirmed unrelated to this code.

File touched: only `framework/renderer/wgpu/rs/lib.rs`.

## OS-commands added (and what they mutate)

In `shell::ActionPanelAndUtilities`, `ShellState::build_os_commands()` builds six `semio_framework_core::CommandDefinition`s, each exposing an EXISTING mutation path rather than inventing new state:

| command id | category | mutates |
|---|---|---|
| `os.setAppearance` | appearance | `self.appearance_id` via `dispatch_action(ActionDescriptor{controller_id:"framework", action:"setAppearance"})` (arm already existed) |
| `os.setExpertise` | general | `self.expertise`, same `"framework"` dispatch pattern |
| `os.setLocale` | language | `self.locale_id`, same pattern |
| `os.setTerminology` | language | `self.terminology_id`, options sourced from the existing `active_terminologies()` helper |
| `os.toggleCompact` | layout | `self.compact_mode`, computes `!self.compact_mode` then dispatches `"setCompact"` |
| `os.resetDock` | layout | `self.layout_override = None; self.sync_dock();` — local, no plugin round-trip, mirrors React's `RESET_DOCK` |

Execution centralized in `ShellState::apply_os_command(command_id, option_value)`, reached from the search palette via a new `"os-command:{id}[:{value}]"` redirect string handled in `activate_search_item`.

**Deliberately omitted** (no persisted state to wire, and inventing it means touching `shell::ShellTypes`, off-limits this wave): `os.introduceApp` (no introduction-playback step field exists anywhere in wgpu), `os.setThemeId` (wgpu has no named `UiTheme` list — only light/dark/system `appearance_id`), `os.setLayout` (no desktop/tablet flag distinct from `compact_mode`). **Wiring request**: whoever owns `ShellTypes` next would need to add these three fields before these commands can exist.

## Category aggregation

Mirrors `os-shell.tsx`'s `resolveCommands`/`buildOsCommands`/`commandCategories` almost 1:1:
- `CommandSource` enum (`Os`/`Plugin`/`App`/`Mode(String)`) + `ResolvedCommand{ definition, source }`.
- `resolve_commands(os_commands, plugin_manifest, app, active_mode_id)` merges all four sources (Plugin from `PluginBridgeEntry.manifest.commands`, App/Mode from `AppDefinition.commands` gated by `ModeDefinition.commands` refs).
- `command_categories()`/`command_category_label()` give ordered, deduped, title-cased category groups.
- `SearchPaletteItem` gained a `category: Option<CommandScope>` field (struct lives just outside the `ShellTypes` region, so editable) so search results carry the os/plugin/app/mode tag.
- Note: currently zero real apps populate `AppDefinition.commands`/`PluginManifest.commands` in this codebase, so Plugin/App/Mode aggregation is correct but presently inert. Also: there is no `handle_command` RPC on the plugin bridge yet (only `handle_action`), so arg-carrying Plugin/App/Mode commands are aggregated into `resolved_commands()`/`build_command_panel_ui()` but intentionally excluded from the quick-search dispatch list (no way to execute them safely yet) — flagged as a follow-up.

## Command panel design (scope tradeoff, stated honestly)

wgpu has NO bottom-middle dock anchor — confirmed from the code itself: `group_side`'s doc comment says "this renderer only has a 2-panel (left/right) layout", and core's `PanelGroup::anchor()` doc says the two middle anchors "start empty... never via a `PanelGroup`". Panel registration (`panel_ui` HashMap, `ensure_framework_panel_ui`) and all drawing live in `ShellLifecycle`/`ShellChrome`, both off-limits this wave.

Given that, built `ShellState::build_command_panel_ui()` as a COMPLETE, READY-TO-WIRE `UiNode` tree (category-headed sections, one row per command) using the exact same pattern as the existing, working `build_settings_general_ui`. Four of six os commands (appearance/expertise/locale/terminology) render as fully-interactive `UiSelectNode`s wired to the already-existing `"framework"` dispatch arms — they'll work the instant a future wave inserts this into `panel_ui` under a tab id. `toggleCompact` is a working `UiButtonNode`. `resetDock` has no `dispatch_action` arm to attach a plain button to (only the `"os-command:"` string redirect reaches `apply_os_command` for it), so it renders as a label pointing at ⌘K search instead of a non-functional button. This is the honestly-scoped fallback: full data/logic layer, zero rendering wiring (requires editing `ShellLifecycle`/`ShellChrome`, outside ownership this wave).

## Fuzzy match

No fuzzy-search crate exists in `Cargo.lock`, so hand-rolled `fuzzy_match_score(query, target) -> Option<i64>`: case-insensitive subsequence match (all query chars must appear in order in target), scoring contiguous runs and word-start matches higher, lightly penalizing long targets. Wired into `filtered_search_items` (was pure `.contains()` substring) — matches against both `label` and `group`, sorted descending, top 20 kept.

## Build/test output

Native (clean):
```
test result: ok. 147 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s
```
All 13 `shell::command_registry_tests::*` pass.

Wasm32 (`cargo check --target wasm32-unknown-unknown`): 2 errors, both `error[E0425]: cannot find function 'action_result_from_patch_ops' in crate 'semio_framework_plugin'`, inside the wasm-only JS plugin-bridge glue (`handle_action_js`) — a completely different module, on the do-not-touch list regardless. Grepped the full wasm32 output for every symbol this agent added — zero matches. **This is a genuine pre-existing gap (or another concurrent session's in-progress work) in `semio_framework_plugin`'s public API, unrelated to this ticket, blocking a clean wasm32 build for everyone.**

## Wiring requests for other agents
1. `ShellTypes` owner: add `introduction_step`/theme-id/layout fields to unlock `os.introduceApp`/`os.setThemeId`/`os.setLayout`.
2. `ShellLifecycle`/`ShellChrome` owner: insert `build_command_panel_ui()`'s output into `panel_ui` under a new tab id + register a way to reach it (one `panel_ui.insert(...)` call plus a chrome affordance).
3. Plugin-bridge owner: a `handle_command` RPC (parallel to `handle_action`) would let Plugin/App/Mode-scope commands actually execute once any app populates `AppDefinition.commands`.
4. `semio_framework_plugin` crate: `action_result_from_patch_ops` is missing on the wasm32 target — unrelated to this ticket but blocks a clean wasm32 build for the whole workspace.
