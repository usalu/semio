# Wave AA — example-switch history hygiene (2026-09-10)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Repo MCP unavailable (noted like other waves).
Serve `:6014` wasm #31 left running; no rebuild/deploy this wave.
Coordinator took defect 1 (shell Undo mount). This wave owns **defect 2 only** (guest).

## Root causes

### (a) Wrong document-row label (`delete-object id=seed-left-001`)

`Puzzle3dSetActiveExampleWork`'s Complete emit set `coalesce_key` but left `description: None` (`..Default::default()`).

Tool-job publication (`VcsArtifactApp` `begin_apply_batch`) writes that `Emit.description` onto the VCS edit. The History COMMANDS list is then **backfilled** (`backfill_command_log`):

- if `edit.description` is `Some`, that string is the row label
- else the first forward op's `print_op()` — here `delete-object id=seed-left-001`

`record_command` (registry label "Set Active Example") is not on this publication path. The registry `LocalizedLabel` was already correct; the guest emit did not carry it.

### (b) Extra `Resize Window` row

Not a guest mutation label. wgpu Shell records `note_shell_command(..., "shell.windowResize", "Resize Window", None)` when the host resizes chrome after a `Chrome`/`Full` refresh. The Complete emit already has **empty** `window_config_mutations` and empty `config_mutations`. Guest cannot fold a host `noteShellCommand` into the coalesced document edit without editing host/renderer (out of scope).

## Fix (guest)

- `PUZZLE3D_SET_ACTIVE_EXAMPLE_DESCRIPTION = "Set Active Example"` (same English as the registry action).
- Complete emit sets `description: Some(...)` next to the existing `coalesce_key`.
- No `window_config_mutations` (resize stays off the document history stack).

## Laws

Existing W-X laws kept.

- `set_active_example_chunks_by_kind_and_emits_one_coalesced_gesture` — also asserts `emit.description` and empty `window_config_mutations`.
- **`set_active_example_history_is_one_set_active_example_row`** — Complete emit is labeled `"Set Active Example"`, no window-config resize, same coalesce key. One-undo restore remains `set_active_example_lands_as_one_edit_and_republishes_the_world_scene`.

`PluginApp::history_snapshot` / completion `HistoryPatch` rebuilds `print_op` for every forward op of the whole-fixture swap and overflows the default test stack. Native proof of the row text is therefore the backfill input (`Emit.description`), which is what the browser History panel reads after refresh.

## Verification

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
# Finished `dev` profile [unoptimized] target(s) in 2m 35s

RUST_MIN_STACK=16777216 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib set_active_example -- --test-threads=1
# test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 633 filtered out; finished in 1.07s
```

Landing/census async laws are stack-sensitive on this machine even without the description field (reproduced with description off). Family is green at 16 MiB `RUST_MIN_STACK`.

## Flags

- **needs wasm rebuild** — guest `✏️editor/🦀️.rs` changed; serve `:6014` still wasm #31.
- **no serve restart** — host/TS/wgpu untouched. `:6013`/`:6014` not killed.
