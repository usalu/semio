# Note Standalone Archive Focus

The captured attempt 53 Math archive reached the React desktop shell after native loading completed. The old shell then queued the Studio panel updater and failed with `configured host app has no panel leaf`. Standalone Note has no configured Studio host panel. The error occurred inside the deferred React updater, after the archive import callback had already reported success.

Standalone archive import now submits the loaded instance to the existing shell session switch and retirement gate. The gate treats it as a replacement of the predecessor instance even when the app identity is unchanged. Adoption rechecks the exact loaded plugin handle and predecessor, uses the normal retirement admission and layout, and marks the replacement document initialized so the example loader does not overwrite it. The importer retains responsibility for an unpublished candidate. The transfer's cancellation signal participates in the retirement admission, including a pending predecessor save or discard prompt. An archive transfer that already owns the transfer slot lends its own progress and signal to the existing predecessor persistence body. This prevents the nested retirement save from trying to claim the same slot.

Studio continues to publish a live child through its panel. The standalone path creates its own primary session without building a Studio panel.

The neutral fixture checks successful same-app replacement, ordinary same-app requests, blocked predecessors, and cancellation before publication. SQLite independently computes the status and final visible instance. A separate test cancels while predecessor persistence holds a seal and verifies that the original predecessor is released. Both focused tests failed first against the old code and passed after the change. Combined with adjacent surface switch, surface document, and Pencil scope laws, the suite passed 26 tests and 123 assertions using the project's existing JSDOM package to provide the adjacent keyboard test's browser constructor.

The browser recheck used the same 805-byte Math archive and isolated, repository-transpiled attempt 53 guest as the original React crash. Native loading completed. Retirement attempted to keep the synthetic predecessor and received `natural-file.no-data-folder` from the detached playground. After the test confirmed closure of its empty Demo, the shell issued the switch, published the initialized successor, and showed the archived Math block at 51.6 seconds. A subsequent edit published a new text block at 57.0 seconds while retaining the Math block. The test exited successfully. See `🗑️generated/note-capture53-math-adoption-approved-fixture.log` and its report.

The prior actor accepted the close request but its background retirement later reported `original-turn.native-return-refused: semantic` under the captured guest. The matching check against the repository-published guest remains pending. The shared test harness handles the normal confirmation in focused import and full reload-then-import only with `SEMIO_PROBE_IMPORT_REPLACE_EMPTY_FIXTURE=1`, after comparing the current shell with its own fresh Note Demo and verifying zero blocks and ink. The production shell still waits for the user's decision when saving the predecessor fails.

## Changed Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛫️spawned-session/🟠️switch-plugin-app/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛫️spawned-session/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🚪️program-retirement/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🟠️close-document/🟦️.ts`
- `note-standalone-archive-focus.json`
- `note-standalone-archive-focus.test.ts`
- `🐍️note-acceptance.mjs`

Run `repo tools exec --light bun -- test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️02/NOTE-INFINITE-CANVAS-APP/note-standalone-archive-focus.test.ts'`.
