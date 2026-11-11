# tool-machine

## semio-framework-tool-machine
No migration needed. `check -p semio-framework-tool-machine --all-targets` passes with 0 errors when run from the `🧰️framework` workspace (log `🗑️generated/tool-machine-8.log`). Run from en1990 or the repo root it reports spurious missing dev-dependency errors, because the crate is a member of the framework workspace only.
Fixed 2 warnings in its lib test (`🔗️read/🧪️tests/🧵️scrub/🦀️.rs`): unused `mem::size_of` import and unused `let d`. Those were unused-item warnings, not deprecations; the only deprecation warnings (`fetch_update`) are in other crates.

## semio-plugin-tool-machine (the 38 en1990 errors)
`check -p semio-plugin-tool-machine --lib` from the en1990 workspace: 0 errors (log `🗑️generated/tool-machine-plugin-4.log`).
- New dependency `semio-plugin-job-receiving`. The local `built`, `built_with` and `retired` helpers in 🔁️follow, 🧵️settlement and 🏷️tag/🧵️original are replaced by its `original_steps` helpers.
- Decoder identity type is now `(NativeAllocationIdentity,u64)`.
- Closes use `retired_poll` with `step(body,effect)`, `begun`, and `selected.step(effect)`.
- 🧵️settlement: `SharedUtf8::admit` takes the actual-capacity argument.
- 🔁️follow:
  - Projection retention now goes through `RetainedOwnedProjectionAdmission`, in two funded turns (admit, then take) via `replied` and `paid_effect`.
  - Three admission fields were added and are closed before their inputs.
  - Window epoch publication runs through `receive`.
  - The window preview leaf step uses the 3-argument `receive` closure.
- 🖐️gesture/🎟️issuer: effect-shaped API replacing the grant API.
  - `reserve_live(available,&mut Option<birth>,effect)`
  - `publish_live(birth,&mut slot,&mut published,effect)`
  - `issue_live(&mut published,operation,&mut Option<(op,capture)>,effect)`

## Open for callers and tests
- `🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs` (lines ~80-90) still calls the old `reserve_live(available,birth,handoff)` and `issue_live(birth,slot,op,handoff)`, so it must move to the new API.
- `🖐️gesture/🎟️issuer/🧪️tests/🦀️.rs` and the plugin's `--tests` targets are not migrated (only `--lib` was verified).
