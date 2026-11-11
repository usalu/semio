# semio-plugin-command migration

Crate: `🔌️plugin/🎮️command` (package `semio-plugin-command`).

## Result
- `check -p semio-plugin-command --lib` (en1990 workspace): 0 errors (log command-check-3.log).
- `test -p semio-plugin-command --lib` (root workspace owns the crate): 4 pass, 6 fail (see Open).

## Changes
- `📥️admission`: shared helpers `stepped` (state change through the owner's resident admission), `forwarded` (effect-shaped funded step), `paid` (one work unit), re-exports of `original_steps`.
  `ToolLatestWinsScope` derives `RetireOwned`; the registry retires scopes through one `RetainedCloneClose` (`scope_close`), `begin(.., effect)` admits the scope via `SharedOwner::admit_from`, `advance(effect)` pays ordered-cursor work on the effect.
  `PendingLatestWinsCommand`: resident `construction`, decoder `(NativeAllocationIdentity,u64)`, `close_step(effect)`, context view as `ViewLease`, `retired_key` removed, key retired generically.
- `🎟️typed/{🦀️,🎟️proof,📥️admission,📨️wire,🏃️runtime}`: each owner holds `construction: OriginalConstructionAdmission` (adopted at birth, `unbuilt` in close, part of `terminal_is_empty`); closes use `retired`/`retired_poll`/`begun` on the effect; wire advance runs through `forwarded` (cursor `advance/close(effect)`).
- `🗝️latest`: `take_key(effect)` via `admit_from`.
- Tests: latest-key test drives effects through `OriginalJobFixtureCaller`; runtime test grant derived from `original_construction_settled_quote` of the producer birth demand, decoder admitted explicitly; local `#[global_allocator]` removed (conflicts with testing allocator). Unmounted wire/proof/typed tests migrated mechanically, not compiled (mounted nowhere).

## Caller contract changes (not edited)
- `ToolLatestWinsRegistry::begin/advance` take an effect; `ToolLatestWinsKeyCopy::take_key(effect)`.
- `PendingLatestWinsCommand` literal: add `construction`, drop `retired_key`; `original_decoder` type; `close_step(effect)`, `close_key_step(effect)`.
- `OriginalToolQualification::admit`, `OriginalCommandAdmission::admit`, `ArtifactOriginalCommandWire::admit` take `&mut OriginalConstructionAdmission` before the output; `OriginalTypedCommandSource::original_decoder` type.

## Open
- Runtime producer tests (6) panic in drop of a `ConstructionProducer` authority: decl-app `op-binary` `built_with` passes a throwaway `&mut Default::default()` admission (`🧬️op-binary/🦀️.rs:8-9`), dropped while construction is still pending. Needs a resident admission in decl-app's producer (its owner).
- `Cargo.toml` unchanged.
