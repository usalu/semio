# Norm `setActiveExample` refusal (static analysis)

Status: source fix applied, NOT compiled, NOT run. Confidence about 80 %.

## Most likely refusal

Candidate (e), not (a)-(d): `handle_action_invocation` returns `window kind norm-din4108-outliner does not own action setActiveExample`.
`advance_manifest_step` (`🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs:225-238`) takes the `is_action()` branch: the checks (b) plugin owner (line 237) and (c) mode (line 240, command branch only) are not the cause; the window-view projection only compares ids. It then calls
`handle_action_invocation` (`🧵️instance/🔌️plugin-app-impl/📨️action-dispatch/🦀️.rs:6-30`).

Chain:
1. Norm no longer declares `setActiveExample`. The din4108 editor has only five command modules (`✏️editor/🎮️commands`: selected-check, set-field, insert-item, remove-item, apply-remedy), `NORM_EDITOR_TOOL_IDS` (`app-surface/⌨️command-args-and-dispatch/🦀️.rs:341`) and `norm_command_from_action!` (`app-surface/🏭️command-and-tool-job-macros/🦀️.rs:7-8`) state that the framework catalogue route owns it.
   The `setActiveExample` entries in `🌎️hub/🧩️compositions/📕️norm/🧩️component/🔣️.json` (app `actions`, kind mutation, `expectedRevision: true`) are stale: that JSON also lists only the inputs/results window kinds and `setSnapshot`/`evaluate`, which no longer exist. The tests in `🌎️hub/🧩️compositions/📕️norm/🧪️tests/🧰️laws/🔬️surface/🦀️.rs:16-30` (`set_active_example_case!`, asserts `definition.actions` contains it and uses `commands::set_active_example`) are stale for the same reason.
2. The shell sends it as a window action addressed to the focused window (here `norm-din4108-outliner`; `window_definition` declares no actions, `🪟️manifest-and-media/🦀️.rs:15-34`). TS side: `🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts:32` says "An editor may dispatch `setActiveExample` without declaring it. The guest loads the registered catalogue."
3. `handle_action_invocation` (`📨️action-dispatch/🦀️.rs:14-26`): `window_action("norm-din4108-outliner","setActiveExample")` is `None` (registry fallback to app `actions`, `🪟️window-kits/📇️registry/🌱️original/🦀️.rs:58`, finds nothing) and `framework_reserved_action_kind` (`🧷️command-routes/📥️staging/🧵️instance-root/🦀️.rs:37`) is `None` (catalogue verb is not in `is_framework_reserved_action_id`, `👷️job/🏛️reserved/🦀️.rs:958`), so the `ok_or_else(...)` refuses before the catalogue route at `🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:196` can run.
   The catalogue route is only reachable through `dispatch_action` directly (the framework test `time-travel-part-2` calls it that way), never through an `ActionAddress`.
4. The refusal is a plain `Err`, returned in the same turn that logs `decoded`, then retained as `FailedDecoded` (`🔀️plugin-exchange/🦀️.rs:157`), matching the log (`seq 7 command decoded` then `failed-decoded`).

Ruled out: (a) `receiving.is_artifact()` is false for an action address; (b) `plugin_id` is `session.plugin_id` / the loaded plugin id `norm`, equal to `manifest.plugin_id` (`pluginId: "norm"`); (d) other windows' commands pass the same limits; din4108 lives in plugin `norm`, so the split is not involved. Mode check was the second candidate (`mode_active` false when the view state has no `activeModeId`, `🛂️manifest/window-view.rs:21`), which the fix also bypasses for this verb.

## Fix (applied)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/📨️action-dispatch/🦀️.rs:14`: an undeclared catalogue verb counts as a runtime mutation, like a reserved verb:

```
let reserved_kind = framework_reserved_action_kind(&address.action_id).or_else(|| (address.action_id == semio_plugin_command::CATALOGUE_EXAMPLE_ACTION_ID && self.tools.registry.get(&address.action_id).is_none()).then_some(ActionKind::Mutation));
```

An app that declares `setActiveExample` still resolves through `window_action` first. The catalogue verb also skips the mode-ownership check (mode independent load).

## Follow-ups not done

- Verify by rebuilding the guest; if the refusal persists, the `[DEBUG] exchange_generic: seq N FAILED with ...` probe (follow-up 28 of `norm-boot-context-refusal.md`) will print the exact text.
- Stale `set_active_example_case!` tests and the stale component JSON of the norm compositions should be regenerated/rewritten against the catalogue route.
- After the refusal is gone the catalogue route needs `Din4108PlayApp::examples()` to contain the requested `exampleId` (`demo`, `failing-thin-insulation`), otherwise `app.example.unknown`.
