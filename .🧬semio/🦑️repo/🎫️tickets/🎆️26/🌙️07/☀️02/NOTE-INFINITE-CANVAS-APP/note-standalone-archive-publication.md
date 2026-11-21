# Note Standalone Archive Focus

The captured attempt 53 Math archive reached the React desktop shell after native loading completed. The old shell then queued the Studio panel updater and failed with `configured host app has no panel leaf`. Standalone Note has no configured Studio host panel. The error occurred inside the deferred React updater, after the archive import callback had already reported success.

Standalone archive import now submits the loaded instance to the existing shell session switch and retirement gate. The gate treats it as a replacement of the predecessor instance even when the app identity is unchanged. Adoption rechecks the exact loaded plugin handle and predecessor, uses the normal retirement admission and layout, and marks the replacement document initialized so the example loader does not overwrite it. The importer retains responsibility for an unpublished candidate. The transfer's cancellation signal participates in the retirement admission, including a pending predecessor save or discard prompt.

Studio continues to publish a live child through its panel. The standalone path creates its own primary session without building a Studio panel.

The neutral fixture checks successful same-app replacement, ordinary same-app requests, blocked predecessors, and cancellation before publication. SQLite independently computes the status and final visible instance. A separate test cancels while predecessor persistence holds a seal and verifies that the original predecessor is released. Both focused tests failed first against the old code and passed after the change. Combined with adjacent surface switch, surface document, and Pencil scope laws, the suite passed 26 tests and 121 assertions using the project's existing JSDOM package to provide the adjacent keyboard test's browser constructor.

Browser rechecks reused the same 805-byte archive and captured guest as the original React crash. Both actors mounted and acknowledged lifecycle. The shell did not report successful focus after 211.6 seconds with normal polling or 120.8 seconds with a browser-only zero-delay retry. The original document remained ready and the page recorded no fault or React error. Those runs logged the start of native loading and the eventual shell focus, which left the intervening phase unobserved. The follow-up trace marks post-load adoption, retirement admission, switch entry and publication individually. See `🗑️generated/note-capture53-math-adoption-recheck.log`, `note-capture53-math-adoption-unthrottled.log` and `note-capture53-math-adoption-frontier.log`.

## Changed Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛫️spawned-session/🟠️switch-plugin-app/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛫️spawned-session/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🚪️program-retirement/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🟠️close-document/🟦️.ts`
- `note-standalone-archive-focus.json`
- `note-standalone-archive-focus.test.ts`

Run `repo tools exec --light bun -- test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️02/NOTE-INFINITE-CANVAS-APP/note-standalone-archive-focus.test.ts'`.
