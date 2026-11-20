# Hub Viewer Tests: "Original Retained Turn Completed With Retained Owners"

## Symptom

`semio-hub-puzzle` `puzzle{2d,3d,5d}_viewer_never_mutates` failed in `close_registered_fixture_app` with `PluginLifecycleFailure`, fault "original retained turn completed with retained owners".

## Diagnosis

A temporary `[DEBUG]` backtrace in `admit_retained_clone_close` (run `hubown-1`) named the scope `original transient store retirement`. The call path was:

`close_registered_fixture_app` → `VcsArtifactApp::close_step` → `close_owned_stores_step` → `TransientStoreDisposer::close_step` → `TransientStoreRetirement::close_step`.

## Root Cause

`TransientStoreRetirement::close_inner` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/🌫️transient-and-interaction-stores/🦀️.rs`) handed the raw poll of `ReturnedSnapshotReadRetirement::admit_original` up from its `root` branch.

- `Complete` from `admit_original` only means the root moved into `active_root`, which is a child retirement that is still alive.
- The outer retirement therefore reported `Complete` while it still owned `active_root`, the read registry alias and the factory.
- Every other `admit_original` caller in the store maps the child step to `Progress`.

The same unmapped pattern existed twice in `🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs`: the prepared next root and the observed transient base.

## Fix

- Map the child step to `RetainedCloneStep::Progress(step.progress())` in all three places.
- `TransientStoreRetirement` now checks the registry alias through `registry_terminal_is_empty()` in `close_inner` and `awaits_live_read`. This is the guard the presence retirement already got, and it avoids dereferencing a funded-closed alias.
- Removed the `[DEBUG] hubown` probe.

## Verification

- `hubown-2`: the build failed on another agent's half-finished `reserve_rows` edit.
- `hubown-3`: 6 passed, 3 failed. The "retained owners" fault is gone. All three viewer tests now fail at the next close step: an `UnsupportedOwner` fault whose message is `semio_plugin_interaction::retirement::InteractionRetirement`.

## Next Failure: `InteractionRetirement` Reached Through `close_into`

- `ErasedSnapshotRetirement::close_into` defaults to `UnsupportedOwner` with the type name. That default is in `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs` and has not changed since HEAD.
- About 150 implementors, including `InteractionRetirement`, implement only `close_step`, so some wrapper calls the reply-shaped `close_into` on an owner that only supports `close_step`.
- `InteractionRetirementFactory` is issued through `DocumentStoreOwners` and `ArtifactStoreCursorDisposer`, which is the interaction config store.
- The `[DEBUG] coordci` backtrace (run `hubown-6`) showed the chain: `ArtifactStoreVcsRetirement` → `ArtifactGenesisRetirement` → `ArcRootRetirement::close_reply` → `FactoryTicketClose::close_into` → `FactorySharedRetirement` → `FactoryOwnedRetirement::native_close_into` → `FactoryTicketClose::drive` → `InteractionRetirement::close_into`, which fell through to the default.
- `FactoryTicketClose` (unchanged since 11-08) requires every ticket child to answer the reply-shaped `close_into`. Once the child is terminal-empty it also requires `native_header_release`. `InteractionRetirement` only had `close_step`.

### Fix

- `InteractionRetirement` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/♻️retirement/🦀️.rs`) now implements `close_into` by completing the reply from its synchronous `close_step`. This is the same shape as the ledger's decoded-edit retirement.
- It also implements `native_header_release` once it is terminal-empty.
- Removed the `[DEBUG] coordci` probe. The contract file matches HEAD again.
- Verification: run `hubown-9` gives 9 passed, 0 failed for `semio-hub-puzzle --lib surface`.
