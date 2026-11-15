# 1112 puzzlefinal (11/12)
## Step 1
- puzzle-2d-v1 session: RetainedWorkOutcome E annotated (2 sites); Puzzle2dClipboardJob owns one `semio_plugin::MediaConstruction` (fault born with `built`, closed first in close_step/close_demands, part of terminal_is_empty).
- records `native_preparation_child`: `replied`/`replied_value` no longer error on a pending reply (zero progress, retry); callers get a `recover` flag (effect invocation mode == Recover) and use recover_advance_into / recover_take / recover_compare_into / recover_begin_close_into.
- W-puzzlefinal-1 hub wasm `-p semio-s-artifact-puzzle-2d-v1` rc 0.
## Step 2
- hub compositions/puzzle Cargo.toml: semio-framework-value dev-dep -> dependency (dyn_enum_close! needs it). W-puzzlefinal-3 `-p semio-hub-puzzle -p semio-hub-puzzle-component` rc 0.
- sourcing curation editor artifact-store-preparation + config-store-preparation: close_step(effect)->OriginalRetirementPoll via owners.close_step(effect) (red in sourcing-curation-tests).
- hub compositions/demonstrator Cargo.toml: semio-framework-value added to dependencies. W-puzzlefinal-6 `-p semio-hub-demonstrator -p semio-hub-demonstrator-component` rc 0.
## Debts
- recover path: adapters pick recover_* only when the effect's invocation mode is Recover; a callee that left its reply pending inside a Receive-mode native turn is not re-entered through an owner-kept flag (flow's `self.recover` recipe); not exercised, `--tests` not run.
- LOCK NEEDED: hub puzzle + demonstrator compositions (semio-framework-value).
Rerun: "$C/1105b-tools/chk.sh" W-puzzlefinal-<n> "🌎️hub/Cargo.toml" wasm -p semio-hub-puzzle -p semio-hub-puzzle-component -p semio-hub-demonstrator -p semio-hub-demonstrator-component
STATUS: FIXED-verified W-puzzlefinal-6
