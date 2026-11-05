# Selection Focus Repair

The event-source follow-up in viewer-selection-events.md supersedes the intermediate lane/pick priority limitations discussed below. Its native execution is still pending.

## Concrete Failure

A stored Results-window configuration check selected member c2. The user then picked member b1 in the viewport. The shared selection resolver kept check c2, added c2 to the selected entities, chose c2 as the primary entity for diagrams/properties, but narrowed the checks list to b1. The visible check selection could therefore lie outside the displayed list, and the diagrams remained on the previous entity.

## Change

The shared resolver now accepts a stored check while entity picks exist only when the check belongs to a picked document owner. Otherwise it resolves the governing check of the first picked entity. Explicit check-row picks retain priority. A picked entity without checks keeps its own primary focus and does not bring back the previous checked entity. No timestamps, persisted schema changes or compatibility layers were added. Check-row focus already replaces interaction selection with the check owner through the existing shell effect.

## Regression Fixture

A new language-neutral selection fixture supplies checks, scoped pick segments and expected entities, owners, check, primary focus and field paths for seven cases: stale check/new entity, matching check, subpart owner, explicit check row, owner without checks stored selection without picks and an explicit view-lane check with stale interaction picks. The existing native unit suite runs these cases through the production resolver; serde_json reads the independent expected result and compares its canonical JSON value with the native resolver output.

## Validation Status

Targeted native test command: repo --service=off cargo --store=shared --configuration=prototype -- test -p semio-s-plugin-norm-app-surface-panels-inspection --lib interaction_picks_and_stored_checks_share_one_consistent_focus. The default capacity queue first failed read-only outside the workspace. With the shared ticket-owned capacity directory, admission waits for 3584 MB while the host has less available memory or exceeds the swap ceiling. Canceled after 88 seconds waiting for capacity, before compilation. No native compilation or passing runtime claim yet.

The independent visual import/semantic compilation suite passes: 3 tests, 26 assertions, no TypeScript compiler diagnostics. This does not execute the Rust selection resolver.

## Changed Files

- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪄️selection-mirror/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪄️selection-mirror/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪄️selection-mirror/🧪️tests/🧫️cases.json
- ✏️s/🔌️plugins/📕️norm/README.md

## Independent Final Review

Static source review confirms the regression is registered under the real panels-inspection crate: its library module includes selection-mirror, selection-mirror registers its native unit-test module, the relative include_str fixture path exists, and the owning crate declares serde_json as a dev dependency. The test calls the actual production resolver and checks canonical JSON values; no replacement selection implementation is involved. These are registration/type plausibility checks, not successful Rust compilation.

Actual runtime scope is the ten Eurocode editors (EN1990–1999), which all paint through the shared LiveView::with_lanes. The five DIN/catalogue editors use different surfaces. The ten readonly Eurocode viewers use their view lane directly and ignore interaction picks in rendering, so they do not receive this stored-check/pick repair.

Review found and corrected a regression before handoff: the explicit transient view-lane check used by keyboard stepping must retain priority over stale entity picks. The new owner filter applies to stored Results-window configuration selection only. with_lanes now re-resolves an explicit lane check with priority, preserving its exact check, field and primary focus; the seventh fixture covers this distinction. Explicit lane priority can still preserve an older lane check after a later entity pick, because there is no ordering metadata. This existing broader latest-interaction limitation remains; no timestamp/schema change is included.

The readonly viewer still does not mirror picks. Clearing picks can still reveal stored selection. Review also found and corrected the selected-row visibility issue: the checks window clears its owner filter when the explicit selected check belongs outside the picked owners. The viewport and outliner keep the actual picked entities, while the selected check stays present in the checks list. Every fixture now specifies expected checks filters, and the native regression runs the production plan_checks to assert its selected row remains visible.

The tuple suite uses the real compiler on assignments to the production LtbMember type and the independent Ajv validator against nine wire vectors. The all-artifact compiler suite checks the production 60 entry roots directly; neither substitutes a fake parser or alternate implementation. Both suites were executed successfully earlier.

## Final Narrow Corrections

with_lanes preserves an explicit transient check through the same production resolver with explicit priority. The owner-filter repair applies only to stored configuration selection. A private checks_owners helper clears only an incompatible checks-list filter; it does not mutate selection.picked. Native fixture coverage now comprises seven cases, including explicit keyboard-step priority and selected-row visibility through the real plan_checks implementation. No additional heavy build was started during review; compilation is still unverified pending the parent native run.
