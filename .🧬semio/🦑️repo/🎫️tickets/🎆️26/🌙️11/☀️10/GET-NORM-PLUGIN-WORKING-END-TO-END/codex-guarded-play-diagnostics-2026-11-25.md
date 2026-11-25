# Guarded Norm browser diagnostics

Use `repo "$T/verify-din4108-scoped-play.ts" <build-log> [previous-wrapper-sha]` for the scoped DIN 4108 workflow and `repo "$T/verify-standards-publication.ts"` for the complete Standards publication workflow, where `T` is this ticket directory. The earlier Python entrypoints were replaced.

Both workflows and the canonical companion sweep share one launcher, one end-to-end grant, the same live reservation check for each child, and the same service and process-tree cleanup. The passive publication waits run before admission. DIN 4108 still requires both preview files when overriding the published guest, verifies signed delivery for the ordinary run, and performs the archive and picker probes in order. Standards still waits for five signed bundles, activates five representative lanes and runs its delivery and four browser checks. The Standards route sweep runs one browser at a time. The launcher stops after a failed phase and closes its listener before releasing the grant.

The fixture and mock suite passed all five tests and 22 assertions, covering four probe plans, failure and cancellation, and direct re-entry without a grant for all three commands. SQLite independently verified the fixture probe counts, first phase, signed-delivery checks and picker checks. No browser, Vite or compiler was started.
