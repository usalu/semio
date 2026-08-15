# Realtime Interaction History Implementation

## Scope

- Added schema-first `HistoryEntry` and `HistoryPatch` invocation data.
- Added `uiScope` and `historyPatch` to binary invocation frames and a cursor-resynchronizing `ReadHistory`/`HistorySnapshot` pair.
- Changed the plugin command log to allocate a distinct sequence for every accepted action, including semantic interactions.
- Moved live history delivery out of UI dirty-scope widening and into the command response.
- Added host projection state, snapshot seeding, and in-order patch application before host effects.
- Made app-channel refresh tolerate individual section render faults by retaining the host's last-known-good section while continuing unrelated sections.
- Updated the first-class interaction design contract to require live semantic-interaction history delivery.

## Verification

- An initial `bun nx run @semio-tech/framework-os-kernel:check` completed successfully. The final repeat was queued behind another shared Cargo build-directory lock.
- Renderer and demonstrator test targets were started. The demonstrator target reached compilation of the plugin stack, but its workspace-wide test build ultimately failed in unrelated `semio-s-plugin-stdio` code with existing `E0282`, `E0308`, and `E0432` errors. No failure in the changed history/channel files was reported.
