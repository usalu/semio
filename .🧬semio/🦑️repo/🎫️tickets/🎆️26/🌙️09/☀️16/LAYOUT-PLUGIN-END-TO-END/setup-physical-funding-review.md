# Physical Funding Transition Review

A bounded read of the existing actual dependency image.json during the native wait observed size8588886016, filesystemBytes8588886016 and maximumPhysicalBytes8589934592. Existing mounted dependencies.json observed the same allocation bytes8588886016. These are read-only observations, not captured receipt authority or a custody proof. No receipt was edited and no live maintenance/publication was invoked.

The current desired allocation38653657088 exceeds that old observed backing maximum. A replacement issuer must retain the genuine current backing bound and may not treat configured ceiling or JSON claims alone as usable capacity. Root asked setup owner to prove the exact physical-capacity/publication/funding order: whether canonical publication genuinely raises the backing maximum under its actual observation protocol, or whether the mounted filesystem requires another canonical physical transition. Reservation and settlement must remain bounded by actual admitted capacity, with old charge retained on failure. This question is an integration proof obligation, not evidence that a valid transition is impossible.

No live transition or full Layout/browser success is claimed.

## Owner Contract Resolution

Setup owner traced actual semantics: publisher retains request.backing.size, admits mounted statfs no larger than envelope, and republishes the configured enforcement ceilings. It does not resize or establish36GiB of usable filesystem. NativeStorageBacking.maximumPhysicalBytes is an owner-supplied enforcement ceiling, not a measurement of usable/free capacity.

Root selected this sequence: genuine old8GiB-funded staging and conservative inactive retirement; complete canonical publication with its full unchanged physical/payload witness; fresh observation authenticating the new published receipt and actual physical tuple; actual replacement issuer reservation/settlement bounded by the fresh owner-backed ceiling and policy capacity. Configured36GiB before successful publication is insufficient. No resize is needed for this enforcement-envelope transition and no36GiB usable-capacity claim is permitted. Actual insufficient-space effects must fail honestly.

Required crash recovery cases include successful publication followed by interruption before funding settlement, current envelope without intent but old committed charge, and interruption after delta reservation. Setup must reconcile genuine funding before its current-envelope/no-intent early return. A new genuine invocation may use durable tuples only as preconditions and must obtain fresh actual issuer/control/custody proof, never old live branding or copied DATA authority. These cases are assigned to the setup owner; they have not yet passed.
