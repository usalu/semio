# Puzzle Editor Acceptance

The objective covers all three artifact editors and their actual application journeys. A compiled crate, source assertion, mock, isolated codec round trip, or emulator story alone does not establish end-to-end completion.

| Required Behavior | 2D | 3D | 5D | Evidence Needed |
| --- | --- | --- | --- | --- |
| Real application activation and correct editor route | Unverified | Unverified | Activation running | Successful normal activation, live page and editor identity |
| Create a valid artifact and open existing content | Unverified | Unverified | Unverified | Real application creation and load with nonempty rendered content |
| Meaningful edits through actual controls | Browser journey prepared | Browser journey prepared; unexecuted | Browser journey prepared; unexecuted | Executed UI interaction and changed document/render |
| Accepted/rejected input with useful feedback | Native regression prepared | Native regression prepared | Native regression prepared | Executed native and UI refusals preserve document/history |
| Save through intended persistence and reopen | Browser journey prepared | Native law prepared | Native law prepared | Actual durable backend/folder readback after fresh reopen |
| Undo and redo remain usable after reopen | Native/browser laws prepared | Native law prepared | Native law prepared | Executed fresh-instance history replay |
| Import and export preserve complete content | Regression prepared | Twin-field repair and regression prepared | Regression prepared | Executed exchange and independent JSON/codec oracle |
| Duplication, deletion, keyboard and asset handling | Unverified | Unverified | Unverified | Actual supported advertised operations exercised |
| Brush/fill, progress and cancellation | Unverified | Unverified | Unverified | Actual supported tool runs, observable progress and cancellation |
| Empty/malformed/load failure/save failure/navigation | Unverified | Unverified | Unverified | Executed failure journeys without silent edit loss |
| Explicit English and German UI | Browser locales selected | Unverified | Unverified | Both locale journeys rendered and exercised |
| Native/WASM build and relevant suites | Infrastructure pending | Infrastructure pending | Infrastructure pending | Correct owning tests run with nonzero case counts and successful builds |

Natural-file Save/Open codecs are absent for Puzzle. Existing snapshot exchange uses the editor's JSON import/export actions. Their absence is not independently a broken advertised Save control; persistence verification must follow the actual document store, folder archive, and local development backend paths. Storybook 3D/5D emulator state is excluded from runtime evidence.

Normal source admission initially failed at the unchanged 16 MiB ceiling. Narrowing the genuine MCP binary dependency boundary restored canonical admission. Normal activation now passes that gate and waits on the live shared dependency producer. Canvas runtime assets also require the genuine assets generator; its native-consumer contract is being repaired. Neither prerequisite is evidence that an editor journey has passed.

Current per-owner details are in `2d-runtime.md`, `terra-3d.md`, `terra-5d.md`, `launcher-repair.md`, and `progress.md`. All rows remain incomplete until authoritative execution proves the stated behavior.
