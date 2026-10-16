# w4fw ownership notice (executor prefix `w4fw`, task FRAMEWORK-FACADE)

State at 16:02 on 2026-10-15: the `semio-framework` facade is carved. Anyone editing the manifest, kernel, abi, action-bus, platform, interaction, source-projection, onboarding, history-edit or tool-run sources must now use the new layout:

- The old `🛂️manifest/🦀️.rs` parent, `🌲️canonical/🦀️.rs` and `📇️space-index/🦀️.rs` are gone. Their items live in the crate roots listed in `RESULT-W4-FRAMEWORK-FACADE.md` section 2.
- `🎠️kernel/🦀️.rs` is the root of `semio-framework-kernel` only; ids/capabilities, history wire and notices, UI delta and media/import jobs moved to sibling folders under `🎠️kernel/`.
- The facade `🧰️framework/📦️packages/🦀️rust/🦀️.rs` is only explicit re-exports plus `FRAMEWORK_RESERVED_CONTRIBUTIONS`.
- `relo-manifest` (slice A): your `onboarding`, `history_edit` and `tool_run` modules are now the crates `semio-framework-onboarding`, `semio-framework-time-travel-actions` and `semio-framework-tool-run-actions`, mounted nowhere else. `AppRole` moved down to `semio-framework-manifest-arg-schema`, the media enums and `PackageRole` stay where you put them (media / plugin crates). Re-read before editing, names are unchanged.
