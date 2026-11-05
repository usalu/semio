# Independent Review

## Presence, Browser Export Lifecycle and Native Transforms

Source review on 2026-11-04; execution remains with the native and React owners.

Presence constructor initialization is complete: ShootingCommandWork owns only its static tool id and consumed flag; it declares its exact frame release and uses existing bounded extent and original producer admission. The three old Config-only producer contracts and addShot now authorize Presence. The retained limits fixture matches all 35 production publication contracts exactly. No new persisted document operations or CRDT mechanism are introduced by presence publication.

Browser export cancellation rejects the foreground batch promptly, unsubscribes its listener and suppresses download delivery of late renderer results. The underlying renderer promise can finish later without re-publishing ledger progress or downloads. Each job uses a monotonic independent id; cancelling one row leaves concurrent rows intact. Notice callbacks and observers are isolated from the export outcome.

One actionable race was sent to the React owner: cancellation can occur after render subscription admission but before the queued microtask invokes host.render. The foreground promise is rejected, but the queued microtask still starts expensive rendering. Check cancellation again inside that microtask before invoking the renderer. This is unnecessary cancelled work, not a late-download defect.

Native transform review found the expected shared TRS construction, all-eight-corners transformed bounds before camera fitting, transformed preview/PNG instances and transformed SVG triangles plus edges. SVG lighting computes the face normal from transformed triangle positions, so nonuniform scaling follows the actual world-space surface. The same request transform is applied once to geometry and once to the original mesh bounds; no double application was found on the inspected paths. The scene close pending branch retains scene custody until close completes. No confirmed native transform defect found in source review.

No compile, test or GPU execution success is claimed by this review.

Further publication-path review found and fixed one required authority: ShootingPlayApp previously had no presence one-item preparation factory, and the trait defaults to None. The new Presence contracts would therefore have refused at mount/dispatch. The editor now installs bounded_transient_preparation_factory for its existing typed presence mutation, matching BIM. Native owner was notified before execution.
