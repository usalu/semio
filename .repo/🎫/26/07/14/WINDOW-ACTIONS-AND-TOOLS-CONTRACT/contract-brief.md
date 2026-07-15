# Window Actions & Tools Contract — shared migration brief (WS-4 Wave B)

You are migrating ONE plugin family to the already-landed Window Actions & Tools contract.
The contract types are DONE in `framework/core/rs/lib.rs` + `framework/plugin/rs/lib.rs`.
Two proving-pair plugins are already migrated — READ THEM as your primary reference:
`shooting/plugin/rs/lib.rs` and `draw/plugin/rs/lib.rs`.

## Repo rules (CLAUDE.md) — obey strictly

- Greenfield: NO back-compat, NO legacy, NO deprecation, NO migration shims. Delete superseded code outright.
- Edit existing files in place using the existing `//#region`/`//#endregion` conventions. NO new files
  (a fresh `#[cfg(test)] mod tests` is OK only if a plugin genuinely has none).
- Never run destructive git (no commit/stash/checkout/reset). Others work concurrently.
- Use `cargo` via Bash. Check REAL exit codes, not piped grep. Use an ISOLATED target dir to avoid lock
  contention with concurrent waves: `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/916c6055-ff8e-45eb-af59-07ce3ec2368a/scratchpad/tgt-<family>`
- NEVER call any `mcp__repo__ticket_*` tool. Report results only in your final text message.
- Docstrings start with a unique emoji. No comments inside definitions. Concise code.

## The contract (USE these types — do not re-derive; verify exact shapes in framework/core/rs/lib.rs)

### Tool registry (replaces the deleted `fn tools()` impls)

- `ToolDefinition { id, label, icon_id, group: Option<String>, keys, cursor, category: Option<ui_wgpu::ToolCategory>, allows_actions_while_active: bool }`.
  Construct: `ToolDefinition::new(id, label, icon_id)` (defaults: no group/keys/cursor/category, allows_actions_while_active=false),
  then struct-update for extras, e.g.
  `ToolDefinition { group: Some("transform".into()), ..ToolDefinition::new("move", "Move", "move") }`.
- Register on the AppDefinition builder: `.tool(ToolDefinition{..})` and scope per window kind:
  `.window_kind_tools(WINDOW_KIND_ID, vec!["move".into(), "rotate".into(), ...])` (Vec<ToolRef>, &str→ToolRef via .into()).
- `ModeDefinition.tools` is now `Vec<ToolRef>`; if any `.mode_tools(id, vec![...])` call site uses the old
  `Vec<ToolNode>` shape, re-type it to `vec!["toolid".into(), ...]`.
- **Exclusivity is per window kind** (one active tool per window). `group` is only a visual toolbar collection.
- `ui_wgpu::ToolCategory` variants: `Selection | Tools` (the `Actions` variant is DELETED). Draw uses
  `ToolCategory::Selection` for select-family tools and `ToolCategory::Tools` for the rest. `category` is optional.

### Active tool = host-owned session view state

- Read the active tool from `view_state.active_tool_id.as_deref().unwrap_or(DEFAULT_TOOL)`.
  DELETE any `runtime.active_tool` / `runtime.transform_tool` / document `active_tool` field and all writes to it.
- The framework auto-injects a `setActiveTool` View action when an app declares tools. Add a handler arm:
  ```
  SET_ACTIVE_TOOL_ACTION_ID => { /* clear in-progress scratch (hover/gesture drafts) */ ActionEmit::default() }
  ```
  It must emit NO ops (View-kind actions returning non-empty ops now hard-error in VcsDocumentApp).
- Programmatic tool switches use `HostEffect::SetActiveTool { window_kind_id, tool_id }` pushed onto an emit's
  `.effects` (see draw's `commit_with_tool_reset`). Import `SET_ACTIVE_TOOL_ACTION_ID` from `semio_framework_plugin`.

### Action registry — EVERY keybound/dispatched action id must be declared or `build_definition` panics

Builder methods (chain on the AppDefinition builder), each takes (id, label):

- `.operation(id, label)` — ActionKind::Operation (mutates the document → emits VCS ops)
- `.view_action(id, label)` — ActionKind::View (ephemeral: selection/hover/camera/tool-scratch; MUST NOT emit ops)
- `.shell_action(id, label)`— ActionKind::Shell (host effects: file open/download/export; MUST NOT emit ops)
- `.action_with(ActionDefinition{ in_palette:false, ..ActionDefinition::new(id,label,kind) })` for non-palette actions.
  KIND DISCIPLINE IS ENFORCED: View/Shell actions returning non-empty `ops` now hard-`Err`. So an id that emits
  ops on some paths MUST be `.operation(...)`. An id that only mutates runtime/scratch and returns
  `ActionEmit::default()`/effects MUST be `.view_action`/`.shell_action`.

### Staged argument forms (P1 actions)

Attach typed args post-hoc: `.action_args(action_id, vec![ ActionArgDef::... ])`.
`ActionArgDef` constructors: `text(id,label)` / `number(id,label)` / `slider(id,label,min,max)` /
`toggle(id,label)` / `select(id,label,vec![ActionArgOption::new(value,label), ...])` / `vec3(id,label)`.
Chainable: `.required()`, `.default_value(v)` (v: impl Into<serde_json::Value>), `.describe(s)`.
The action's `handle_action` must READ the staged args out of `args` (the descriptor args the panel provides),
materializing defaults host-side. Required args are guaranteed present post-materialization.

### Preview emission (coalescing) — ActionEmit constructors

- `ActionEmit::ops(ops)` — plain op emission.
- `ActionEmit::amend(ops, coalesce_key)` — PATTERN (a): per-tick coalesced. Every tick of one gesture with the
  same coalesce_key folds into ONE undoable edit. Use for CHEAP per-tick ops (camera, small transforms, a
  slider scrub, exaggeration). e.g. `ActionEmit::amend(vec![Op::Translate{..}], "gumball-translate")`.
- `ActionEmit::commit(ops, description)` — PATTERN (b): gesture-END commit of an app-runtime scratch draft as ONE
  described edit (coalesce_key None). Use for MEGABYTE-scale content where per-tick amend would be O(N²)
  (full-mesh diffs, paint strokes). Accumulate in runtime scratch across pointer-move ticks emitting ZERO ops,
  then one `commit` on pointer-up.
- `ActionEmit::effect(HostEffect)` — one host effect, no ops.

## THE MANDATORY COALESCING FIX (lowpoly + procedural-3d specifically)

`translateSelection/rotateSelection/scaleSelection` (gumball) currently emit full before/after object diffs
per tick with NO coalesce key → megabyte-scale op spam + O(N²) replay. This MUST be fixed:

- If the per-tick diff is CHEAP/small → `ActionEmit::amend(ops, "gumball-{translate|rotate|scale}")`.
- PREFERRED for anything touching a full mesh buffer → scratch-then-single-commit (pattern b): accumulate the
  transform delta in runtime scratch on each pointer-move (emit nothing), commit ONE final-diff op on pointer-up.
  Use judgement based on actual op/mesh sizes in the plugin. Shooting's gumball uses `amend` because its ops are
  tiny per-asset transform deltas (`TranslateAssets{asset_ids,dx,dy,dz}`) — NOT full-mesh diffs. If your plugin's
  transform op is a compact delta like that, `amend` is correct and sufficient. If it re-serializes the whole mesh,
  prefer the scratch-commit route.

## Recipe per plugin

1. Locate the crate/file layout and READ the current lib.rs (plugin + core crate).
2. Author/extend the action registry EXHAUSTIVELY — declare every action id handled in `handle_action`
   (and every keybound id) with the correct kind. Missing declarations panic `build_definition`.
3. Attach `.action_args(...)` for the P1 actions per your family classification.
4. Declare `ToolDefinition`s + `.window_kind_tools(...)`; DELETE the old `fn tools(...)` impl entirely and the
   `ToolCategory::Actions` footer buttons it built.
5. Replace active-tool storage with `view_state.active_tool_id`; add the `SET_ACTIVE_TOOL_ACTION_ID` handler arm.
6. Fix the uncoalesced-transform pattern where flagged.
7. Hand-fix fixtures for any deleted document fields (none of these 6 are flagged as doc-active-tool offenders,
   but verify: grep the plugin's example/fixture JSON for `activeTool`).
8. Extend the existing test module: (a) an arg-form action → ops, (b) tool switch produces NO ops/history,
   (c) for lowpoly/procedural-3d: a multi-tick gumball drag regression proving ONE coalesced edit (amend) OR
   zero mid-drag ops + one commit (scratch-commit).

## Verification (MUST do before reporting)

- `CARGO_TARGET_DIR=<isolated> cargo build -p <each package name>` → iterate until it exits 0.
- `CARGO_TARGET_DIR=<isolated> cargo test -p <each package name>` → iterate until green.
- `grep -n "fn tools(" <plugin lib.rs>` → zero hits. `grep -n "ToolCategory::Actions" <plugin>` → zero hits.
- Report: what you declared, the coalescing fix you chose and why, test results (call out the coalescing
  regression test explicitly), and any judgement calls.
