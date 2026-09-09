# Session Authority Progress and Cancellation

The Shell now presents the private session-authority bootstrap as an exact bilingual, accessible operation. English and German are explicit supported locales; an unknown locale is refused. Pending work is a polite busy status with one cancellation control and no invented progress count. Unavailability is an assertive terminal notice without a cancellation control.

Cancellation aborts the current identity bootstrap and refresher, closes the private broker port, retires authenticated inference presentation, resets the plugin actor to the unauthenticated client, and closes the identity document. The worker-side close invalidates broker proof and session authority even when cancellation races an authority read.

Evidence:

- `session-authority-cancellation-focused-current.log`: 2 passed, 331 skipped.
- `session-authority-notice-focused-green.log`: 2 passed, terminal success.
- Root full OS baseline `48804`: 333 passed before the catalog-generation contract was advanced.

The earlier notice RED was test-only: the repository test facade does not expose Jest DOM `toHaveAttribute` or `queryByRole`. Native DOM attribute and selector checks now express the same accessibility contract.
