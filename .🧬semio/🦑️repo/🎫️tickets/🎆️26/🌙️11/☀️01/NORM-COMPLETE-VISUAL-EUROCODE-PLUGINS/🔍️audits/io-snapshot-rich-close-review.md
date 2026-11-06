# IO snapshot rich close review

At review time NativeSnapshotReceiving advertises requires_original_control true but implements only close_original_step, not close_step_controlled. The trait default calls ordinary close_step without the supplied control, so a controlled factory-ticket caller can execute local or pending child physical effects without the original work preflight. Its original-loan branch separately forwards pending ticket work and prepares and commits local controlled-retirement work. The controlled branch needs the same selected-action priority while leaving delegated child admission to that child.

The original close maps a prepare_work error through From<ValueError>, classifying it as a primary rather than a work refusal. The conversion From<OriginalRetirementFailure> for NativeSnapshotFailure otherwise moves both primary and work owners and progress.

The semio-framework-io-control Cargo lib mounts control/🦀️.rs, which mounts snapshot and its cfg(test) tests. No ignore attribute was found in that scoped subtree. This is source-only review and does not establish runtime selection or pass.

## Follow-up

The revised source adds a controlled mirror and classifies prepare errors as work refusals in both variants, addressing the two findings. The pending child is delegated through its matching original or controlled ticket; local retirement retains its own preflight and settlement. Actual-caller tests should cover zero and funded work for local and nested children in both variants and preserve a work-preflight error. No runtime verification was performed in this review.
