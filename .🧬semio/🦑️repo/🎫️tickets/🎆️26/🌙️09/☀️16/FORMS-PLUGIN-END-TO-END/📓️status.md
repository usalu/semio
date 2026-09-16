# 📋️ Forms plugin end to end — status

## 2026-09-16

- Native `cargo check -p semio-s-plugin-forms --lib`: passes (15 warnings in `semio-s-artifact-forms-forms`, all `unused_qualifications`/dead helpers in the Try transient owner).
- Restage: `screen -dmS forms-activate <ticket>/📜️activate-forms-react.sh` → `@semio-tech/framework-os-dev:activate-forms-react-dev` (log `🗑️generated/activate-forms-react.txt`). Started 16:10Z under load avg ~140 (peer fleets: process3d serve, demonstrator component-dev, plugin tests, energy check).
- Serve: `screen -dmS forms-serve <ticket>/📜️serve-forms-react.sh` (vite 6058, `?plugin=forms`); launch entry `forms-react-attach` added to `.claude/launch.json`.
- Probes: `🐍️forms-console-dump-probe.mjs` (boot), `🐍️forms-interact-probe.mjs` (Actions pane `add-step` → Artifact tree +1 step → `mod+z` undo → Try window `nextStep`).
- 16:25Z first restage died in the fine-grain-locking deadlock: `cargo rustc -p semio-s-plugin-{process,forms,cad-aec-building} --target wasm32-wasip2 --profile wasm-dev` all idle 9–15 min in `prebuild_lock_exclusive` (sample) with no rustc child; killed the set per pid (peers' builds included — they cannot progress either), relaunched `forms-activate` 16:26Z.

### Faults found and fixed (in order hit)
1. **Boot refused: `primary plugin forms does not declare pinned app ""`.** The serve passes `VITE_SEMIO_APP_ID: resolved.appId ?? ""` when the playground row has no `app`, and `🧑‍💻dev/🟦️.ts` read it with `??`, so `appId === ""` reached ShellHost's `appId !== undefined` pin guard. Fixed both ends: `app = "s.forms.forms@1/*#editor"` in the forms `Cargo.toml` playground block (as energy/cad/fem declare), and `VITE_SEMIO_APP_ID || boot.defaultAppId` in the dev boot (matches the `VITE_SEMIO_BRAND || undefined` consumer). `note` has the same missing `app` (not touched).
2. **Try window never rendered: `ui.surface-render: 1:forms-try [producer]: DuplicateSiblingKey parent=#0 key=#0`.** `Buildable::try_build` stamps an unkeyed node `"#0"`, `HasChildren::try_child` re-keys only EMPTY keys, so already-built text rows pushed as siblings collide. Keyed every `display`/`text` row explicitly in the editor Try window, the viewer Try window and `render_extension_question` (`🦀️.rs` of each).
3. **Every shell action refused: `action 'setActiveExample' is not a framework-reserved action … dispatched exclusively through the typed command channel`.** `FormsPlayApp` never overrode `ArtifactEditor::command_from_action` (energy/process/cad all do). Added `args_bridge::command_from_action`: emits both camelCase and snake_case spellings of every arg key (Try-window payloads decode camelCase via `rename_all`, document payloads snake_case), aliases the block-list host's `addBlock`/`removeBlock`/`moveBlock` verbs (`blockId`→`question_id`, `position` default `"after"`), prints the host-merged control `value` into the `*_json` string fields. Two unit tests (`command_from_action_round_trips_every_command_id`, `…_bridges_block_list_verbs_and_control_values`) pass.

Boot proven after (1)+(2) (`forms-boot-3`): `data-semio-os-ready=forms`, Blueprint block list (2 steps / 15 questions / 13-kind palette) + Try wizard "Step 1 / 2" with Back/Next, panels Artifact/Catalogue/Inspection.

Native tests `cargo test -p semio-s-artifact-forms-forms --lib`: 153 pass / 41 fail; every failure is pre-existing harness debt (`BuiltChildren requires retained page transport` from the test helper's serde walk, `interactive-job.live-instance` from the registry-less/unbound testkit, store drop witness) — see memory `project-registryless-testkit-new-app-unusable`.

### End-to-end proof (restage #5, 17:21Z; `🗑️generated/forms-interact-3/`)
`🐍️forms-interact-probe.mjs` — six steps, zero fault lines each:
1. boot: `data-semio-os-ready=forms`, Blueprint block list + Try wizard "Step 1 / 2"; history `patchCursor 0, upserts 0` (boot `setActiveExample demo` is now a no-op when the document already equals the example — `set-active-example` accepts `demo` and short-circuits on equality).
2. Blueprint "Add Step" button → `history patch applied … create-step "Step 3"`, canUndo.
3. Actions pane `action.addStep` row → `create-step "Step 4"`.
4. `mod+z` → history "Undo" patch, body text 493→486 (Step 4 gone).
5. Typing into the Try "Component Name" input → `setTryValue` settles, scheduled `setTryValueStep` settles (the `generation: Float(1.0)` refusal is gone after the bridge's integral-float restore).
6. Try "Next" (window button, not the pane row) → "Step 2 / 2".
`🐍️forms-args-probe.mjs` additionally proved Catalogue "Number" → `create-block step-id="identity" … kind="number"`.

Additional fixes this round: bridge restores whole finite floats to `UInt`/`Int` (host JSON → exact-u64 codecs); `demo` example id alias; equality short-circuit in `set-active-example`.

### Left as is
- 41 native test failures in `semio-s-artifact-forms-forms` are pre-existing harness debt (framework `BuiltChildren` serde refusal, unbound live-instance testkit, store drop witness), not runtime faults — every one reproduces the same fault family as before this ticket.
- `✏️s/🔌️plugins/🗒️note` has the same missing playground `app` (the dev-boot `||` fix alone now lets it boot; not verified here).
- Serve left running on 6058 (`screen forms-serve`) for the user; launch entry `forms-react-attach`.
